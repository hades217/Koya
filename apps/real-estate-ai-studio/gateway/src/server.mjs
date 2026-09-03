import { createServer } from 'node:http';
import { createGatewayService, GatewayError, JsonFileStore } from './core.mjs';
import { createOpenAiImageProvider } from './provider.mjs';

const env = process.env;
const number = (name) => env[name] === undefined ? undefined : Number(env[name]);
const subscriptions = env.GATEWAY_SUBSCRIPTIONS_JSON ? JSON.parse(env.GATEWAY_SUBSCRIPTIONS_JSON) : {};
const service = createGatewayService({
  config: {
    signingSecret: env.GATEWAY_SIGNING_SECRET ?? '', adminSecret: env.GATEWAY_ADMIN_SECRET,
    rateLimit: number('GATEWAY_RATE_LIMIT') ?? 3, rateWindowSeconds: number('GATEWAY_RATE_WINDOW_SECONDS') ?? 60,
    capability: { modelId: env.OPENAI_IMAGE_MODEL, width: number('OPENAI_OUTPUT_WIDTH'), height: number('OPENAI_OUTPUT_HEIGHT'), panoramaMode: env.OPENAI_PANORAMA_MODE, creditCost: number('GATEWAY_CREDIT_COST'), priceAmountMinor: number('GATEWAY_PRICE_AMOUNT_MINOR'), priceCurrency: env.GATEWAY_PRICE_CURRENCY, internalCostAmountMinor: number('OPENAI_INTERNAL_COST_AMOUNT_MINOR'), internalCostCurrency: env.OPENAI_INTERNAL_COST_CURRENCY }
  },
  store: new JsonFileStore(env.GATEWAY_DATA_FILE ?? new URL('../.data/gateway.json', import.meta.url).pathname, { subscriptions }),
  provider: createOpenAiImageProvider({ apiKey: env.OPENAI_API_KEY })
});

async function body(request) {
  const chunks = []; let size = 0;
  for await (const chunk of request) { size += chunk.length; if (size > 25 * 1024 * 1024) throw new GatewayError(413, 'payload_too_large', 'Approved generation payload is too large.'); chunks.push(chunk); }
  if (!chunks.length) return {};
  try { return JSON.parse(Buffer.concat(chunks).toString('utf8')); } catch { throw new GatewayError(400, 'invalid_json', 'Request body must be valid JSON.'); }
}

const server = createServer(async (request, response) => {
  try {
    const url = new URL(request.url, 'http://gateway.local'); const data = await body(request);
    let result;
    if (request.method === 'GET' && url.pathname === '/healthz') result = { status: 'ok' };
    else if (request.method === 'POST' && url.pathname === '/v1/auth/exchange') result = service.exchange(data);
    else if (request.method === 'GET' && url.pathname === '/v1/subscription') result = service.subscriptionStatus(request.headers);
    else if (request.method === 'GET' && url.pathname === '/v1/subscription/ledger') result = service.customerLedger(request.headers);
    else if (request.method === 'POST' && url.pathname === '/v1/subscription/cancel') result = service.cancel(request.headers);
    else if (request.method === 'POST' && url.pathname === '/v1/desktop/capabilities') result = service.capability(request.headers, data);
    else if (request.method === 'POST' && url.pathname === '/v1/desktop/images') result = await service.submitImage(request.headers, data);
    else if (request.method === 'POST' && /^\/v1\/admin\/subscriptions\/[^/]+\/top-ups$/.test(url.pathname)) result = service.topUp(request.headers['x-gateway-admin'], decodeURIComponent(url.pathname.split('/')[4]), data.credits, data.reference);
    else if (request.method === 'POST' && /^\/v1\/admin\/requests\/[^/]+\/reconcile$/.test(url.pathname)) result = service.reconcile(request.headers['x-gateway-admin'], decodeURIComponent(url.pathname.split('/')[4]));
    else throw new GatewayError(404, 'not_found', 'Gateway route is unavailable.');
    response.writeHead(200, { 'content-type': 'application/json', 'cache-control': 'no-store' }); response.end(JSON.stringify(result));
  } catch (error) {
    const status = error instanceof GatewayError ? error.status : 500; const code = error instanceof GatewayError ? error.code : 'internal_error';
    response.writeHead(status, { 'content-type': 'application/json', 'cache-control': 'no-store' }); response.end(JSON.stringify({ error: { code, message: error instanceof GatewayError ? error.message : 'Gateway request failed.' } }));
  }
});

server.listen(Number(env.PORT ?? 8787), env.HOST ?? '127.0.0.1', () => console.log(`Estate Studio subscription gateway listening on ${env.HOST ?? '127.0.0.1'}:${env.PORT ?? 8787}`));
