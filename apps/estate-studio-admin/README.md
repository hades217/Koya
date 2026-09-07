# Estate Studio CRM / Super Admin

A runnable Web CRM with a Python standard-library HTTP service, transactional SQLite, and a dependency-free browser UI. It is a new sibling application; it does not alter desktop project files or invoke paid providers.

## Run locally

```sh
npm --prefix apps/estate-studio-admin run dev
```

Open **http://127.0.0.1:18767/admin** and select **以本地负责人进入** if no administrator has been configured. After administrator setup and a server restart, use the configured email, password and authenticator code; the shortcut and old preview-owner sessions are disabled. The server binds only to loopback. Local preview has an explicit owner shortcut, local verification outbox, and its own database. Do not expose preview through a tunnel or reverse proxy.

- `/admin`: owner CRM and platform controls.
- `/inquiry`: server-backed consultation form with a receipt ID and email fallback.
- `/register`: email-link registration/login. In local preview, find the verification link under 系统设置 → 本地验证邮箱; no email is sent.
- `/notify`: one-time notification-address verification, never a login or role grant.
- Initial data is empty. No fake customers, revenue or subscriptions are seeded.
- Default preview data: `.data/preview.sqlite3`; production: `.data/crm.sqlite3`.
- `CRM_DATA_FILE` overrides the database path. An environment marker prevents a preview process opening a production database and vice versa.
- The preview and QA databases are ignored by Git; do not commit them.

## Available workflows

- Customer and company records; labels, priority, notes, search, paging and registration filters.
- A customer page combines consultations, communication records, tasks, opportunities and a verified account association.
- Consultation inbox: immutable originals, status/reason validation, spam exclusion, appointment times and contact/task links.
- Follow-up tasks: deadlines, overdue/today views, completion results, rescheduling history and reopening reasons.
- Opportunity board and list, stage transitions, versioned quote records, accepted-deal evidence, independently recorded manual receipts. Manual receipts stay marked unverified.
- Email-link identities with one-time 15-minute verification tokens; no account is counted before verification. Same-email registration links the existing contact.
- Account disable, restore and session revocation. Each protected request checks account status; restore does not resurrect old sessions.
- CSV column mapping, validation preview, confirmed import, row error reports and idempotent import commits.
- Filtered CSV exports with optional contact data, formula protection, creator-only download and 24-hour expiry; capped at 10,000 rows per export.
- Customer merge with version checks, retained original records and a safe undo when affected records have not subsequently changed. Different registered identities cannot be merged.
- Configurable display timezone, period counts, audit trail and verified notification recipient.
- Notification queue persists independently of delivery. Unknown delivery is not auto-retried; confirmed successful messages cannot be retried. Preview produces a distinctly labelled local notification, never a sent-email claim.
- Optional read-only subscription ledger adapter, plus explicit user/ subscription mapping after administrator verification. No payment, generation or subscription mutations.

All administrator writes require a valid session, same-origin JSON request and CSRF token. User roles are checked server-side. SQLite transactions include business writes and audit events; optimistic versions prevent silent overwrites. Passwords, activation material and provider outputs are not exposed by the admin API. Lists mask email/phone; detail access is audited.

## Verification

```sh
npm --prefix apps/estate-studio-admin run verify
python3 apps/estate-studio-admin/tests/benchmark.py
```

The unit/integration suite starts real HTTP servers with isolated temporary databases. It checks authorization, CSRF/Host controls, MFA/replay, email verification, consultation idempotency, account revocation, state transitions, concurrent changes, exports, imports, merges, notification failure and database restoration. The benchmark creates and deletes an independent 100,000-contact / 100,000-inquiry fixture.

Browser checks and remaining release gates are in [ACCEPTANCE.md](ACCEPTANCE.md). Passing API tests is not a claim that every PRD acceptance criterion or external integration is complete.

## Production preparation

Do not run the preview command on a public host. Start without `--local-preview`; startup requires an administrator configuration and an HTTPS origin. The application still binds to loopback and is intended to sit behind an HTTPS reverse proxy that preserves the configured Host.

```sh
npm --prefix apps/estate-studio-admin run setup
# Set CRM_ORIGIN to the approved HTTPS origin in the service environment.
npm --prefix apps/estate-studio-admin start
```

Setup prompts interactively for the owner email and password, hashes the password with PBKDF2-SHA256 (600,000 iterations), and generates a TOTP enrollment secret. The secret is displayed only by this user-operated setup command; enroll it in an authenticator and store it securely. The configuration file is owner-readable only and is outside source control. No production administrator credentials have been created by this task.

Production identity requirements:

- Admin password plus TOTP, with accepted TOTP-step replay prevention.
- `HttpOnly`, `SameSite=Strict`, `Secure`, host-only cookie; 30-minute idle timeout and 12-hour maximum session duration.
- Shared-IP login throttling; operator-controlled setup. Admin invitation and self-service MFA recovery are not yet implemented.
- Deploy the approved privacy/terms copy and retention policy before enabling real signup.

For real verification/notification email, configure these server environment variables; never put values in source or command arguments:

- `CRM_SMTP_HOST`, `CRM_SMTP_PORT` (default 465, implicit TLS).
- `CRM_SMTP_FROM`.
- Optional authenticated SMTP: `CRM_SMTP_USER`, `CRM_SMTP_PASSWORD`.

The notification destination is set in the UI and activated only through a verification link delivered to that address. A verified recipient change affects future notifications; failed/unconfigured earlier notifications require explicit retry. A crash during delivery marks the message outcome uncertain on restart.

For an existing subscription gateway, configure `CRM_GATEWAY_DATA_FILE` to an explicitly selected readable ledger file. The adapter consumes the existing `subscriptions` and `requests` JSON format, returns an allowlist of safe fields, and never writes the file. Missing/invalid data is unavailable, and missing credit fields are not treated as zero. This is a file read-back, not evidence that the gateway is reachable or deployed.

The existing desktop client continues to use its own licensing and subscription authentication. The CRM account API (`GET /api/account`) is ready for a same-origin Web client; it is not wired into desktop authentication. Disabling a CRM account does not cancel billing or immediately revoke offline permissions.

## Website handoff

The new `/inquiry` page is a functional intake channel. Route the website's online inquiry CTA to it when deploying both services behind the approved origin; keep the existing mailto contact option as fallback. The older static website is under concurrent development and has not been overwritten or publicly redeployed by this task. Its existing mailto CTA is not counted as a persisted inquiry.

If embedding an inquiry form on the same origin, POST JSON to `/api/public/inquiries` with `name`, `email`, optional `company`/`message`, `terms: true`, and a unique `key`. Reuse the same key only for an unchanged retry. Only `{received: true, id: ...}` means the consultation was persisted. No cross-origin wildcard access is enabled.

## Backup and restore

```sh
# Back up whichever database you explicitly select in the environment.
npm --prefix apps/estate-studio-admin run backup
# Or back up the default local preview explicitly:
python3 apps/estate-studio-admin/server.py --backup --local-preview
```

Uses SQLite's online backup API, not a raw copy of a running WAL database. Backups are mode 0600 and stored next to the selected database. To restore, stop the owned service, preserve the current database and its WAL/SHM files as a recovery set, select the backup as a **new database path** through `CRM_DATA_FILE`, and restart in the matching environment mode. Verify record counts, associations and audit history before switching the service to the restored database. Do not overwrite an active SQLite file.

The app does not schedule backups or automatically purge records. Hosting, encrypted-volume configuration, backup schedule, retention/anonymization workflow, production email delivery and desktop integration require their explicit deployment work.

## Basic membership billing

`/membership` is the customer billing page for Estate Studio Basic, AUD 90 per month. Email verification now links to it. Authenticated customer routes under `/api/account/billing/` use a server-only gateway bridge; browser-supplied account IDs and prices are ignored. Every billing write retains session, same-origin and CSRF checks.

Configure `CRM_BILLING_GATEWAY_URL`, `CRM_BILLING_ADMIN_SECRET`, and `CRM_BILLING_MODE` on the server. Local preview is restricted to test billing. Missing configuration displays unavailable and cannot charge a customer. Provider secrets and gateway activation material are never exposed to the browser. The bridge uses bounded network calls while holding the existing CRM transaction lock; higher-traffic production should move billing orchestration outside that shared lock.

See [Stripe membership setup](../../docs/product/ESTATE_STUDIO_STRIPE_BASIC_MEMBERSHIP.md) for the gateway configuration, Stripe events and remaining release gates. This is a web billing integration, not a completed native desktop licence migration or deployed payment service.

## Sales funnel analysis

Open `/admin#funnel`. Create named pipelines (for example 新客户, 续费, 企业合作) on opportunities and filter the board or analysis by pipeline. Pipelines currently share the six existing sales stages; per-pipeline custom stages are not implemented. A saved pipeline cannot be reassigned, preserving cohort comparisons.

Each stage change requires a fresh reason category and customer feedback. Server-generated stage history is append-only through the API, saved with the opportunity and audit in the same transaction. Reopening clears the previous close date. Existing opportunities retain their data; missing early history is explicitly counted, never reconstructed as fact.

Analytics select opportunities by creation period (all, 30, 90, or 365 days) and track their recorded changes through today. Stage conversion is unique opportunities observed leaving a stage forwards divided by unique opportunities observed entering it. Skipped stages are not filled in. Reopened opportunities can appear in both advanced and lost counts; those columns are not mutually exclusive. Exit duration averages recorded completed visits, while win rate uses currently closed won/lost opportunities. No denominator produces an unavailable result, not 0%. Reason distribution counts transitions and links to the original feedback. Stall review includes open opportunities with at least 14 days in the current recorded stage, an explicit blocker, or missing history.

This release analyses recorded sales opportunities. Anonymous website visits and acquisition-channel attribution are not tracked by this module. Reasons are operator-entered evidence, not automated causal conclusions.

## Lead acquisition and Sales commission

`/admin#commissions` manages staff/partners, channel rules and opportunity estimates. Staff records have acquisition, Sales, dual, or referral-partner duties; they do not grant login access. Supported acquisition channels: Marketing, street promotion, operations, referrals, Sales self-sourcing and other. Each opportunity can assign one acquisition contributor and one Sales contributor, including the same person. New Sales assignments require a Sales-capable, active staff record.

Rules support either independent acquisition/Sales percentages of the deal amount, or a total commission percentage with an acquisition share and the remainder assigned to Sales. Percentages have two-decimal precision, must be explicit, and are stored as integer basis points. No business rates are seeded. Rule selection requires a matching channel and both contributors. Each selection saves a versioned rate/name snapshot; subsequent rule edits do not reprice existing opportunities. Winning locks the attribution and snapshot even if the opportunity is later reopened. There is no override/settlement correction workflow yet.

Open opportunities use expected amounts; won opportunities use their current agreed amounts; lost opportunities do not accrue an estimate. Unknown amounts/rules remain unknown. Currency minor-unit rounding uses half-up on the total and acquisition portion, allocating the remainder to Sales so the parts equal the total. This is a forecast/calculation module, not payroll or verified payouts: actual collections, refunds, tax/net-base policy, settlement approval and payout execution are not connected. Editing the agreed deal amount recalculates its estimate; a locked rule does not make the deal amount an immutable settlement ledger.

Verification includes direct/pool math, rounding, invalid percentages, unavailable amounts, channel and staff validation, snapshots surviving rule changes, forged snapshots, won/reopened attribution locks, same-person contributions and admin-only access.
