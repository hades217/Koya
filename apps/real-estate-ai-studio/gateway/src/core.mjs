import { createHmac, createHash, timingSafeEqual } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, renameSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';

const json = (value) => Buffer.from(JSON.stringify(value)).toString('base64url');
const fingerprint = (value) => createHash('sha256').update(JSON.stringify(value)).digest('hex');
const ledgerId = (kind, idempotencyKey, createdAt) => `${kind}-${createHash('sha256').update(`${kind}:${idempotencyKey}:${createdAt}`).digest('hex').slice(0, 20)}`;

export class GatewayError extends Error {
  constructor(status, code, message) { super(message); this.status = status; this.code = code; }
}

export function issueSessionToken({ subscriptionId, signingSecret, now, lifetimeSeconds = 900 }) {
  if (!subscriptionId || signingSecret.length < 32 || lifetimeSeconds < 60 || lifetimeSeconds > 3600) throw new GatewayError(500, 'invalid_session_configuration', 'Session configuration is invalid.');
  const payload = json({ sub: subscriptionId, iat: now, exp: now + lifetimeSeconds });
  const signature = createHmac('sha256', signingSecret).update(payload).digest('base64url');
  return { accessToken: `${payload}.${signature}`, expiresAt: now + lifetimeSeconds };
}

export function verifySessionToken(token, signingSecret, now) {
  const [payload, signature, extra] = String(token ?? '').split('.');
  if (!payload || !signature || extra) throw new GatewayError(401, 'invalid_session', 'Managed subscription session is invalid.');
  const expected = createHmac('sha256', signingSecret).update(payload).digest();
  let supplied;
  try { supplied = Buffer.from(signature, 'base64url'); } catch { throw new GatewayError(401, 'invalid_session', 'Managed subscription session is invalid.'); }
  if (supplied.length !== expected.length || !timingSafeEqual(supplied, expected)) throw new GatewayError(401, 'invalid_session', 'Managed subscription session is invalid.');
  let claims;
  try { claims = JSON.parse(Buffer.from(payload, 'base64url').toString('utf8')); } catch { throw new GatewayError(401, 'invalid_session', 'Managed subscription session is invalid.'); }
  if (!claims.sub || !Number.isInteger(claims.exp) || claims.exp <= now || claims.exp > now + 3600) throw new GatewayError(401, 'expired_session', 'Managed subscription session expired.');
  return claims;
}

export class MemoryStore {
  constructor(seed = {}) { this.subscriptions = new Map(Object.entries(seed.subscriptions ?? {})); this.requests = new Map(); this.rateEvents = new Map(); this.events = []; }
  subscription(id) { return this.subscriptions.get(id); }
  saveSubscription(value) { this.subscriptions.set(value.id, structuredClone(value)); }
  request(key) { return this.requests.get(key); }
  saveRequest(key, value) { this.requests.set(key, structuredClone(value)); }
  record(event) { this.events.push(structuredClone(event)); }
  rate(subscriptionId, now, limit, windowSeconds) {
    const events = (this.rateEvents.get(subscriptionId) ?? []).filter((item) => item > now - windowSeconds);
    if (events.length >= limit) throw new GatewayError(429, 'rate_limited', 'Generation rate limit reached.');
    events.push(now); this.rateEvents.set(subscriptionId, events);
  }
}

export class JsonFileStore extends MemoryStore {
  constructor(path, seed = {}) {
    const loaded = existsSync(path) ? JSON.parse(readFileSync(path, 'utf8')) : seed;
    super({ subscriptions: loaded.subscriptions ?? {} });
    this.path = path;
    this.requests = new Map(Object.entries(loaded.requests ?? {}));
    this.events = loaded.events ?? [];
    this.persist();
  }
  persist() {
    mkdirSync(dirname(this.path), { recursive: true });
    const temporary = `${this.path}.tmp`;
    writeFileSync(temporary, JSON.stringify({ subscriptions: Object.fromEntries(this.subscriptions), requests: Object.fromEntries(this.requests), events: this.events }, null, 2), { mode: 0o600 });
    renameSync(temporary, this.path);
  }
  saveSubscription(value) { super.saveSubscription(value); this.persist(); }
  saveRequest(key, value) { super.saveRequest(key, value); this.persist(); }
  record(event) { super.record(event); this.persist(); }
}

function activeSubscription(store, id) {
  const subscription = store.subscription(id);
  if (!subscription) throw new GatewayError(403, 'subscription_unavailable', 'Managed subscription is unavailable.');
  if (subscription.status !== 'active') throw new GatewayError(403, 'subscription_inactive', 'Managed subscription is not active.');
  return subscription;
}

function remaining(subscription) {
  return Math.max(0, Number(subscription.includedCredits ?? 0) + Number(subscription.topUpCredits ?? 0) - Number(subscription.usedCredits ?? 0) - Number(subscription.reservedCredits ?? 0));
}

function authenticate(headers, config, store, now) {
  const authorization = headers.authorization ?? headers.Authorization ?? '';
  if (!authorization.startsWith('Bearer ')) throw new GatewayError(401, 'authentication_required', 'Managed subscription sign-in is required.');
  const claims = verifySessionToken(authorization.slice(7), config.signingSecret, now);
  const headerSubscription = headers['x-estate-subscription'] ?? headers['X-Estate-Subscription'];
  if (headerSubscription !== claims.sub) throw new GatewayError(403, 'subscription_mismatch', 'Subscription identity does not match the session.');
  return activeSubscription(store, claims.sub);
}

function validateCapabilityRequest(body) {
  for (const key of ['projectId', 'jobId', 'approvalFingerprint', 'idempotencyKey', 'panoramaMode']) if (typeof body?.[key] !== 'string' || !body[key]) throw new GatewayError(400, 'invalid_capability_request', `Missing ${key}.`);
  if (!Number.isInteger(body.requestedWidth) || !Number.isInteger(body.requestedHeight) || body.outputCount !== 1) throw new GatewayError(400, 'invalid_capability_request', 'Exact dimensions and one output are required.');
}

export function createGatewayService({ config, store = new MemoryStore(), provider, clock = () => Math.floor(Date.now() / 1000) }) {
  if (!config?.signingSecret || config.signingSecret.length < 32) throw new Error('GATEWAY_SIGNING_SECRET must contain at least 32 characters.');
  const capability = config.capability ?? {};

  return {
    store,
    exchange({ activationCode, subscriptionId }) {
      const now = clock();
      const subscription = activeSubscription(store, subscriptionId);
      const expected = createHmac('sha256', config.signingSecret).update(`activation:${subscriptionId}`).digest('hex');
      if (!activationCode || activationCode.length !== expected.length || !timingSafeEqual(Buffer.from(activationCode), Buffer.from(expected))) throw new GatewayError(401, 'invalid_activation', 'Subscription activation is invalid.');
      const session = issueSessionToken({ subscriptionId, signingSecret: config.signingSecret, now });
      return { ...session, subscriptionId, plan: subscription.plan, allowanceRemaining: remaining(subscription) };
    },
    subscriptionStatus(headers) {
      const now = clock(); const subscription = authenticate(headers, config, store, now);
      return { subscriptionId: subscription.id, status: subscription.status, plan: subscription.plan, includedCredits: subscription.includedCredits, topUpCredits: subscription.topUpCredits, usedCredits: subscription.usedCredits, reservedCredits: subscription.reservedCredits, remainingCredits: remaining(subscription), cancelAtPeriodEnd: Boolean(subscription.cancelAtPeriodEnd), periodEndsAt: subscription.periodEndsAt ?? null };
    },
    cancel(headers) {
      const now = clock(); const subscription = authenticate(headers, config, store, now);
      subscription.cancelAtPeriodEnd = true; subscription.updatedAt = now; store.saveSubscription(subscription);
      store.record({ kind: 'cancellation_requested', subscriptionId: subscription.id, createdAt: now });
      return { status: subscription.status, cancelAtPeriodEnd: true, periodEndsAt: subscription.periodEndsAt ?? null };
    },
    customerLedger(headers) {
      const now = clock(); const subscription = authenticate(headers, config, store, now);
      return store.events.filter((event) => event.subscriptionId === subscription.id && ['credit_reserved', 'credit_consumed', 'credit_released', 'top_up', 'cancellation_requested'].includes(event.kind)).map(({ internalCostAmountMinor, internalCostCurrency, providerUsage, ...event }) => event);
    },
    topUp(adminSecret, subscriptionId, credits, reference) {
      const now = clock();
      if (!config.adminSecret || adminSecret !== config.adminSecret) throw new GatewayError(401, 'admin_authentication_required', 'Admin authentication is required.');
      if (!Number.isInteger(credits) || credits <= 0 || !reference) throw new GatewayError(400, 'invalid_top_up', 'A positive credit amount and payment reference are required.');
      const subscription = activeSubscription(store, subscriptionId);
      if (store.events.some((event) => event.kind === 'top_up' && event.reference === reference)) return this.subscriptionStatusForAdmin(subscription);
      subscription.topUpCredits = Number(subscription.topUpCredits ?? 0) + credits; subscription.updatedAt = now; store.saveSubscription(subscription);
      store.record({ kind: 'top_up', subscriptionId, credits, reference, createdAt: now });
      return this.subscriptionStatusForAdmin(subscription);
    },
    subscriptionStatusForAdmin(subscription) { return { subscriptionId: subscription.id, remainingCredits: remaining(subscription), topUpCredits: subscription.topUpCredits, usedCredits: subscription.usedCredits, reservedCredits: subscription.reservedCredits }; },
    capability(headers, body) {
      const now = clock(); const subscription = authenticate(headers, config, store, now); validateCapabilityRequest(body);
      const exact = capability.modelId && capability.width === body.requestedWidth && capability.height === body.requestedHeight && capability.panoramaMode === body.panoramaMode;
      const priceAvailable = Number.isInteger(capability.creditCost) && capability.creditCost > 0 && Number.isInteger(capability.priceAmountMinor) && capability.priceAmountMinor > 0 && typeof capability.priceCurrency === 'string';
      return { requestFingerprint: body.approvalFingerprint, entitlementStatus: 'active', capabilityStatus: exact && priceAvailable ? 'available' : 'unavailable', modelId: exact ? capability.modelId : null, supportedWidth: exact ? capability.width : null, supportedHeight: exact ? capability.height : null, panoramaMode: exact ? capability.panoramaMode : null, priceStatus: priceAvailable ? 'available' : 'unavailable', priceAmountMinor: priceAvailable ? capability.priceAmountMinor : null, priceCurrency: priceAvailable ? capability.priceCurrency : null, creditCost: priceAvailable ? capability.creditCost : null, quotaStatus: remaining(subscription) >= Number(capability.creditCost ?? Infinity) ? 'available' : 'exhausted', quotaRemaining: remaining(subscription), checkedAt: now, expiresAt: now + 300 };
    },
    async submitImage(headers, body) {
      const now = clock(); const subscription = authenticate(headers, config, store, now); validateCapabilityRequest(body);
      if (!Array.isArray(body.inputs) || body.inputs.length === 0 || typeof body.prompt !== 'string' || !body.prompt.trim()) throw new GatewayError(400, 'invalid_generation_request', 'Exact approved inputs and prompt are required.');
      const requestFingerprint = fingerprint({ projectId: body.projectId, jobId: body.jobId, approvalFingerprint: body.approvalFingerprint, requestedWidth: body.requestedWidth, requestedHeight: body.requestedHeight, panoramaMode: body.panoramaMode, outputCount: body.outputCount, inputs: body.inputs, prompt: body.prompt, parameters: body.parameters ?? {} });
      const existing = store.request(body.idempotencyKey);
      if (existing) {
        if (existing.requestFingerprint !== requestFingerprint) throw new GatewayError(409, 'idempotency_conflict', 'Idempotency key was already used for different approved inputs.');
        return existing.publicResponse;
      }
      const check = this.capability(headers, body);
      if (check.capabilityStatus !== 'available' || check.priceStatus !== 'available') throw new GatewayError(409, 'capability_unavailable', 'Exact generation capability or current price is unavailable.');
      if (check.quotaStatus !== 'available') throw new GatewayError(402, 'allowance_exhausted', 'Generation allowance is exhausted; an authorised top-up is required.');
      store.rate(subscription.id, now, config.rateLimit ?? 3, config.rateWindowSeconds ?? 60);
      const credits = capability.creditCost;
      subscription.reservedCredits = Number(subscription.reservedCredits ?? 0) + credits; store.saveSubscription(subscription);
      const pending = { subscriptionId: subscription.id, requestFingerprint, approvalFingerprint: body.approvalFingerprint, credits, status: 'submitted', createdAt: now };
      store.saveRequest(body.idempotencyKey, pending);
      store.record({ id: ledgerId('credit_reserved', body.idempotencyKey, now), kind: 'credit_reserved', subscriptionId: subscription.id, idempotencyKey: body.idempotencyKey, requestFingerprint, credits, createdAt: now });
      try {
        const result = await provider.generate({ model: capability.modelId, prompt: body.prompt, inputs: body.inputs, parameters: body.parameters ?? {}, outputCount: 1 });
        subscription.reservedCredits -= credits; subscription.usedCredits = Number(subscription.usedCredits ?? 0) + credits; store.saveSubscription(subscription);
        const publicResponse = { requestFingerprint, approvalFingerprint: body.approvalFingerprint, idempotencyKey: body.idempotencyKey, providerRequestId: result.providerRequestId ?? null, status: 'completed', outputCount: 1, outputs: result.outputs, usage: result.usage ?? null, completedAt: clock() };
        store.saveRequest(body.idempotencyKey, { ...pending, status: 'completed', providerRequestId: result.providerRequestId ?? null, publicResponse });
        const completedAt = clock();
        store.record({ id: ledgerId('credit_consumed', body.idempotencyKey, completedAt), kind: 'credit_consumed', subscriptionId: subscription.id, idempotencyKey: body.idempotencyKey, requestFingerprint, credits, createdAt: completedAt });
        store.record({ id: ledgerId('provider_usage', body.idempotencyKey, completedAt), kind: 'provider_usage', subscriptionId: subscription.id, idempotencyKey: body.idempotencyKey, requestFingerprint, providerRequestId: result.providerRequestId ?? null, providerUsageStatus: result.usage ? 'available' : 'unavailable', providerUsage: result.usage ?? null, createdAt: completedAt });
        const internalCostAvailable = Number.isInteger(capability.internalCostAmountMinor) && capability.internalCostAmountMinor > 0 && typeof capability.internalCostCurrency === 'string' && capability.internalCostCurrency.length > 0;
        store.record({ id: ledgerId('internal_cost', body.idempotencyKey, completedAt), kind: 'internal_cost', subscriptionId: subscription.id, idempotencyKey: body.idempotencyKey, requestFingerprint, internalCostStatus: internalCostAvailable ? 'available' : 'unavailable', internalCostAmountMinor: internalCostAvailable ? capability.internalCostAmountMinor : null, internalCostCurrency: internalCostAvailable ? capability.internalCostCurrency : null, source: internalCostAvailable ? 'deployment_verified_fixed_request_cost' : 'unavailable', createdAt: completedAt });
        return publicResponse;
      } catch (error) {
        subscription.reservedCredits -= credits; store.saveSubscription(subscription);
        store.saveRequest(body.idempotencyKey, { ...pending, status: 'failed', publicResponse: { requestFingerprint, approvalFingerprint: body.approvalFingerprint, idempotencyKey: body.idempotencyKey, status: 'failed', failureCode: 'provider_failed', completedAt: clock() } });
        const failedAt = clock();
        store.record({ id: ledgerId('credit_released', body.idempotencyKey, failedAt), kind: 'credit_released', subscriptionId: subscription.id, idempotencyKey: body.idempotencyKey, requestFingerprint, credits, reason: 'provider_failed_no_customer_charge', failurePolicyVersion: 'v1', createdAt: failedAt });
        throw new GatewayError(502, 'provider_failed', 'Provider generation failed without consuming customer credits.');
      }
    },
    reconcile(adminSecret, idempotencyKey) {
      const now = clock();
      if (!config.adminSecret || adminSecret !== config.adminSecret) throw new GatewayError(401, 'admin_authentication_required', 'Admin authentication is required.');
      const request = store.request(idempotencyKey);
      if (!request) throw new GatewayError(404, 'request_unavailable', 'Generation request is unavailable.');
      const events = store.events.filter((event) => event.idempotencyKey === idempotencyKey);
      const credit = events.find((event) => event.kind === 'credit_consumed');
      const usage = events.find((event) => event.kind === 'provider_usage');
      const cost = events.find((event) => event.kind === 'internal_cost');
      const gaps = [];
      if (request.status !== 'completed') gaps.push('generation_not_completed');
      if (!credit || credit.credits !== request.credits) gaps.push('customer_credit_unreconciled');
      if (!usage || usage.providerUsageStatus !== 'available') gaps.push('provider_usage_unavailable');
      if (!cost || cost.internalCostStatus !== 'available') gaps.push('internal_cost_unavailable');
      const result = { idempotencyKey, requestFingerprint: request.requestFingerprint, status: gaps.length ? 'blocked' : 'reconciled', customerCredits: credit?.credits ?? null, providerRequestId: usage?.providerRequestId ?? null, providerUsage: usage?.providerUsage ?? null, internalCostAmountMinor: cost?.internalCostAmountMinor ?? null, internalCostCurrency: cost?.internalCostCurrency ?? null, gaps, reconciledAt: now };
      if (!events.some((event) => event.kind === 'reconciliation' && event.status === result.status)) store.record({ id: ledgerId('reconciliation', idempotencyKey, now), kind: 'reconciliation', subscriptionId: request.subscriptionId, ...result, createdAt: now });
      return result;
    }
  };
}

export function activationCode(subscriptionId, signingSecret) { return createHmac('sha256', signingSecret).update(`activation:${subscriptionId}`).digest('hex'); }
