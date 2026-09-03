import assert from 'node:assert/strict';
import test from 'node:test';
import { activationCode, createGatewayService, GatewayError, MemoryStore } from '../src/core.mjs';

const secret = 'test-signing-secret-with-at-least-32-characters';
const adminSecret = 'test-admin-secret';
const baseSubscription = { id: 'sub-1', status: 'active', plan: 'studio-monthly', includedCredits: 8, topUpCredits: 0, usedCredits: 0, reservedCredits: 0, cancelAtPeriodEnd: false, periodEndsAt: 2_000 };
const config = { signingSecret: secret, adminSecret, rateLimit: 3, rateWindowSeconds: 60, capability: { modelId: 'verified-test-model', width: 3840, height: 1920, panoramaMode: 'equirectangular_2_1', creditCost: 4, priceAmountMinor: 400, priceCurrency: 'AUD', internalCostAmountMinor: 180, internalCostCurrency: 'USD' } };

function setup(provider = { async generate() { return { providerRequestId: 'provider-1', outputs: [{ b64Json: 'image-data' }], usage: { total_tokens: 12 } }; } }) {
  let now = 1_000;
  const store = new MemoryStore({ subscriptions: { 'sub-1': structuredClone(baseSubscription) } });
  const service = createGatewayService({ config, store, provider, clock: () => now });
  const session = service.exchange({ subscriptionId: 'sub-1', activationCode: activationCode('sub-1', secret) });
  const headers = { authorization: `Bearer ${session.accessToken}`, 'x-estate-subscription': 'sub-1' };
  return { service, store, headers, setNow: (value) => { now = value; } };
}

const capabilityRequest = { projectId: 'project-1', jobId: 'job-1', approvalFingerprint: 'approval-1', idempotencyKey: 'idem-1', requestedWidth: 3840, requestedHeight: 1920, panoramaMode: 'equirectangular_2_1', outputCount: 1 };
const imageRequest = { ...capabilityRequest, inputs: [{ role: 'topology_source', checksumSha256: 'abc', dataUrl: 'data:image/png;base64,AA==' }], prompt: 'Create the exact reviewed panorama.', parameters: { size: 'auto' } };

test('short-lived session exposes subscription state but not signing material', () => {
  const { service, headers } = setup();
  const status = service.subscriptionStatus(headers);
  assert.equal(status.remainingCredits, 8);
  assert.equal(JSON.stringify(status).includes(secret), false);
  assert.throws(() => service.subscriptionStatus({ ...headers, 'x-estate-subscription': 'sub-2' }), (error) => error instanceof GatewayError && error.code === 'subscription_mismatch');
});

test('capability is exact and allowance is top-up and cancellation aware', () => {
  const { service, headers } = setup();
  assert.equal(service.capability(headers, capabilityRequest).capabilityStatus, 'available');
  assert.equal(service.capability(headers, { ...capabilityRequest, requestedWidth: 4096 }).capabilityStatus, 'unavailable');
  assert.equal(service.topUp(adminSecret, 'sub-1', 4, 'payment-1').remainingCredits, 12);
  assert.equal(service.topUp(adminSecret, 'sub-1', 4, 'payment-1').remainingCredits, 12);
  assert.equal(service.cancel(headers).cancelAtPeriodEnd, true);
});

test('same idempotent image request charges once and conflict fails closed', async () => {
  let providerCalls = 0;
  const { service, headers, store } = setup({ async generate() { providerCalls += 1; return { providerRequestId: 'provider-1', outputs: [{ b64Json: 'image-data' }], usage: { total_tokens: 12 } }; } });
  const first = await service.submitImage(headers, imageRequest);
  const replay = await service.submitImage(headers, imageRequest);
  assert.deepEqual(replay, first);
  assert.equal(providerCalls, 1);
  assert.equal(store.subscription('sub-1').usedCredits, 4);
  assert.deepEqual(service.customerLedger(headers).map((event) => event.kind), ['credit_reserved', 'credit_consumed']);
  const reconciliation = service.reconcile(adminSecret, 'idem-1');
  assert.equal(reconciliation.status, 'reconciled');
  assert.equal(reconciliation.customerCredits, 4);
  assert.equal(reconciliation.internalCostAmountMinor, 180);
  await assert.rejects(() => service.submitImage(headers, { ...imageRequest, prompt: 'Changed after approval.' }), (error) => error instanceof GatewayError && error.code === 'idempotency_conflict');
});

test('provider failure releases reservation and does not consume credits', async () => {
  const { service, headers, store } = setup({ async generate() { throw new Error('provider unavailable'); } });
  await assert.rejects(() => service.submitImage(headers, imageRequest), (error) => error instanceof GatewayError && error.code === 'provider_failed');
  assert.equal(store.subscription('sub-1').usedCredits, 0);
  assert.equal(store.subscription('sub-1').reservedCredits, 0);
  assert.equal(service.customerLedger(headers).at(-1).reason, 'provider_failed_no_customer_charge');
});

test('reconciliation blocks rather than treating missing usage and cost as zero', async () => {
  const incompleteConfig = { ...config, capability: { ...config.capability, internalCostAmountMinor: undefined, internalCostCurrency: undefined } };
  const store = new MemoryStore({ subscriptions: { 'sub-1': structuredClone(baseSubscription) } });
  const service = createGatewayService({ config: incompleteConfig, store, provider: { async generate() { return { outputs: [{ b64Json: 'image-data' }], usage: null }; } }, clock: () => 1_000 });
  const session = service.exchange({ subscriptionId: 'sub-1', activationCode: activationCode('sub-1', secret) });
  const headers = { authorization: `Bearer ${session.accessToken}`, 'x-estate-subscription': 'sub-1' };
  await service.submitImage(headers, imageRequest);
  const result = service.reconcile(adminSecret, 'idem-1');
  assert.equal(result.status, 'blocked');
  assert.deepEqual(result.gaps, ['provider_usage_unavailable', 'internal_cost_unavailable']);
  assert.equal(result.internalCostAmountMinor, null);
});

test('exhausted allowance blocks before provider submission', async () => {
  let calls = 0;
  const { service, headers, store } = setup({ async generate() { calls += 1; return { outputs: [{ b64Json: 'x' }] }; } });
  const subscription = store.subscription('sub-1'); subscription.usedCredits = 8; store.saveSubscription(subscription);
  await assert.rejects(() => service.submitImage(headers, imageRequest), (error) => error instanceof GatewayError && error.code === 'allowance_exhausted');
  assert.equal(calls, 0);
});
