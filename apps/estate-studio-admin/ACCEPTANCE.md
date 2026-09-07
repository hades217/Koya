# CRM implementation and acceptance record

Date: 2026-09-06. Status: **working local CRM; not deployed and not full production PRD acceptance**.

## Implemented and checked

- 32 HTTP/service integration tests pass. Coverage includes ordinary-user denial, CSRF and hostile Host rejection, password/TOTP and TOTP replay, one-use email verification, stable inquiry/contact/user linking, immediate session revocation, consultation replay conflicts, state transition validation, optimistic concurrency, CSV field mapping/import/export, safe merges and undo, notification-recipient verification and uncertain-delivery retry, read-only gateway field allowlisting, persistence and backup restoration.
- JavaScript syntax check passes.
- Browser on the actual running service completed: owner entry; customer creation and list read-back; contact communication record; follow-up creation; opportunity creation; quotation creation; closed-won validation; public inquiry receipt; local email-link registration; return to the same customer with prior inquiry, task, communication and opportunity history retained; notification-address verification.
- Browser QA uses `.data/qa.sqlite3` on port 18768. The user workspace uses `.data/preview.sqlite3` on port 18767 and starts with no sample customers. No external email or paid provider task was sent.
- Browser layouts checked at 390px (customer detail and scrollable task form), 360px (workbench) and 1280px (full desktop workbench). Measured document width equals viewport width in these checks; no page-level horizontal overflow. Browser error log was empty after the final workbench verification. Header button wrapping and panel spacing were adjusted after screenshot review.
- A browser-discovered async form issue was fixed: the submit handler now retains its form reference before awaiting the request. Success hides the form and shows the received ID without a false error.
- Same-document email verification fragments are handled on hash change; used verification links are omitted from the local outbox.
- Runtime compatibility: the host Python lacks `hashlib.scrypt`; admin password hashing uses the available PBKDF2-SHA256 implementation with 600,000 iterations. The MFA tests verify that path.

## Local performance fixture

Disposable SQLite dataset: 100,000 customers and 100,000 consultations; seven queries for each case. This is a local service-query measurement, not a production load or availability guarantee.

| Query | Median | Maximum |
| --- | --- | --- |
| Customer list | 126.2 ms | 126.7 ms |
| Consultation list | 163.7 ms | 170.5 ms |
| Customer text search | 178.5 ms | 180.1 ms |

## PRD boundary

The CRM workflows are usable locally, but the following are **not** completed release claims:

- **A02 / production security:** administrator bootstrap and MFA work; no real owner credentials are enrolled, and self-service password/MFA recovery is not implemented. Production HTTPS, recovery procedure and owner enrollment remain deployment gates.
- **A03 / website:** the new standalone `/inquiry` endpoint/page is verified. The separately edited static marketing website's existing mailto CTA has not been changed or publicly routed to this intake.
- **A05 / email:** queue, failure independence, verification and retry semantics are tested. Real SMTP mailbox delivery is not verified; preview notifications are explicitly marked unsent.
- **A10 / whole App:** CRM account denial and session revocation work. Existing desktop/gateway authentication, offline licensing and billing cancellation remain independent. No remote desktop lockout claim.
- **A11 / analytics:** today, 7-day and 30-day new inquiry/user counts and status totals are implemented. Arbitrary custom ranges are supported by list APIs; a custom-range dashboard picker and fuller conversion analysis are not implemented.
- **A13 / exports:** creator-only, expiring, filtered CSV downloads work with a 10,000-row limit. File preparation is bounded and synchronous; very large asynchronous exports remain future work.
- **A15 / acceptance:** local browser core workflows are verified. Formal production end-to-end acceptance has not occurred.
- **A16 / operations:** SQLite backup/restore is tested. A production backup schedule, encrypted persistent volume and recovery SLA are not configured.
- CRM company/contact linking exists. Additional opportunity participants and application tenant/member management are not implemented.
- Manual receipt records stay unverified. Provider-verified payment matching, amount reconciliation and receipt correction workflows are not implemented.
- Notification retries and task due views exist; persistent per-admin read/unread task reminders and calendar integrations are not implemented.
- Customer merge retains target fields and preserves original source data; a per-field conflict-choice editor and post-change automated unmerge are not implemented.
- Personal-data retention/anonymization policy and user-facing approved terms need owner decisions before production use.

## Repository scope

Only the new `apps/estate-studio-admin/`, root admin launch/verify scripts, the apps index and implementation references are changed for this work. No desktop files, property media, provider ledgers, unrelated changes, credentials or customer records are committed, pushed or deployed.

`git diff --check` passes. Repository layout validation currently fails because a pre-existing root `CHECK_REPORT.json` is outside the allowed top-level entries. That unrelated file has been preserved; it was present before this task's edits.

### Sales funnel extension — 2026-09-06

- Named pipeline filtering, recorded-stage conversion, closed win rate, stage-duration and stalled-opportunity review implemented locally.
- Transition reasons and immutable history covered by HTTP tests, including skips, reopen, missing history, cohort filtering, unauthorized access and attempted history forgery.
- Browser read-back verified the analysis page, navigation, cohort filters, empty-state metrics and explicit unknown values.
- No live sales records seeded for demonstration. Per-pipeline custom stages and anonymous acquisition analytics remain outside this implementation.

### Lead acquisition / Sales commissions — 2026-09-06

- Staff/partner records, channel-specific rate rules, two attribution roles, independent-rate and total-pool modes, historical snapshots and estimates implemented locally.
- 39-test suite passed; after refining cent rounding and adding the same-person attribution case, both affected calculation and HTTP tests passed again.
- Browser verified the new workspace entry and rule form with acquisition channel, computation mode, acquisition percentage and Sales percentage.
- No staff identities, real commission rates or payouts created. Settlement/refund/payroll integration is not implemented.
