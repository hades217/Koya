# Estate Studio subscription gateway

This is a separately deployable Node 20 service for managed subscription authentication, allowance checks, explicit top-ups, cancellation state, rate limits, idempotent image requests, and server-side provider authentication. It is not a project store or web authoring service.

The service accepts only the exact generation payload approved by the desktop client. Subscription, request, and audit records are written atomically to `GATEWAY_DATA_FILE` with mode `0600`; production deployment should mount that path on encrypted persistent storage and use a single process or replace the adapter with a transactional database.

Required deployment configuration:

- `GATEWAY_SIGNING_SECRET`: at least 32 characters, server only.
- `GATEWAY_ADMIN_SECRET`: billing webhook/admin secret, server only.
- `GATEWAY_SUBSCRIPTIONS_JSON`: initial subscription records keyed by subscription ID.
- `GATEWAY_DATA_FILE`: durable ledger path.
- `OPENAI_API_KEY`: provider key, server only.
- `OPENAI_IMAGE_MODEL`, `OPENAI_OUTPUT_WIDTH`, `OPENAI_OUTPUT_HEIGHT`, `OPENAI_PANORAMA_MODE`: capability evidence from a verified provider test.
- `GATEWAY_CREDIT_COST`, `GATEWAY_PRICE_AMOUNT_MINOR`, `GATEWAY_PRICE_CURRENCY`: current commercial evidence. Missing values keep capability unavailable.
- `OPENAI_INTERNAL_COST_AMOUNT_MINOR`, `OPENAI_INTERNAL_COST_CURRENCY`: verified internal per-request cost evidence used only by the internal reconciliation ledger. Missing cost is `unavailable`, never zero.

`POST /v1/auth/exchange` exchanges a vendor-issued activation code for a token valid for 15 minutes. Customer routes require that token plus the matching `X-Estate-Subscription` header. `POST /v1/desktop/images` reserves credits, records the idempotency key before the provider call, and consumes the credits only after success. A replay returns the existing result; a changed payload using the same key is rejected.

The default OpenAI `/v1/images/generations` adapter deliberately rejects reference inputs rather than silently dropping them. A paid topology-sensitive task remains blocked until an audited input-capable OpenAI adapter and exact supported output capability are configured.

Customer ledger responses expose credits, top-ups, cancellation and the documented no-charge failure policy. Provider usage and internal cost stay in the operator ledger. `POST /v1/admin/requests/:idempotencyKey/reconcile` requires the admin secret and returns `blocked` with explicit gaps whenever provider usage or internal cost evidence is unavailable.

Run contract tests with `npm test`. Starting the service or passing tests does not prove a deployed account, current provider price, production persistence, or successful paid generation.
