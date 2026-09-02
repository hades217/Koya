# Final external blocker audit — 2026-09-02 AEST

This audit identifies the exact external state required to finish Tasks 42, 43, and 46 and every remaining non-Pass PRD criterion. It records presence/absence only; no credential value was read, printed, copied, or stored.

## Platform release gates

### macOS

`npm run macos:release:preflight` failed closed with:

> `APPLE_SIGNING_IDENTITY is required for a signed and notarised macOS release.`

The process environment contains none of the required release inputs: `APPLE_SIGNING_IDENTITY`, either complete Apple notarisation credential set, `ESTATE_STUDIO_UPDATER_PUBKEY`, `ESTATE_STUDIO_UPDATE_STABLE_URL`, or `TAURI_SIGNING_PRIVATE_KEY`. No generated production release overlay exists. A complete, strict-valid ad-hoc QA package exists, but it has no TeamIdentifier, is not notarised, and is intentionally Gatekeeper-rejected.

Task 42 therefore requires release-owner authorisation of the exact Developer ID identity, protected notarisation/updater inputs, a production build, Gatekeeper/stapler verification, and clean-machine install/update/uninstall evidence. A QA signature cannot substitute for any of these.

### Windows

`npm run windows:release:preflight` failed closed with:

> `Windows installer production and acceptance must run on a Windows host.`

The current host is `Darwin 25.4.0 arm64`. `ESTATE_STUDIO_WINDOWS_CERTIFICATE_THUMBPRINT` and `ESTATE_STUDIO_WINDOWS_TIMESTAMP_URL` are absent, and no Windows release overlay exists. Task 43 requires a native supported Windows host, exact installed certificate, timestamp/updater inputs, MSI and NSIS production, Authenticode verification, and native install/update/uninstall evidence.

## Licence, provider, and subscription gates

- `/Users/lightman/Library/Application Support/com.landiq.estatestudio.qa/licence/license.json` is absent. Native Settings reports `No signed licence is installed`; commercial export, release, publishing, and portable-export actions remain blocked.
- `desktop-settings.json` is absent, so conservative defaults apply: managed gateway URL `None`, publishing targets empty, analytics endpoint `None`, and secret references empty.
- `generation-capabilities.json` is absent. The process environment has no `OPENAI_API_KEY`; no direct provider key is expected in the desktop product.
- All four QA projects have zero generation jobs. Harbourlight has one completed deterministic local background job, not a Provider generation task.
- The authenticated Codex app-server binding passes criterion 17. Official GPT Image 2 documentation supports a provider-native 3840 × 1920 2:1 request, now reflected by the approval package, but the bound account still does not report exact image-tool access, current price, image quota, or a provider output.
- No authenticated managed-subscription session, live entitlement, allowance/top-up account, provider request, usage record, or internal-cost record exists.

Criteria 6, 7, 18, 20, 21, and 22 therefore require a rights-approved exact test package plus either verified Codex image capability or a real customer managed-gateway session with current capability/price/quota. Provider submission, top-up, output review, provider switching, and reconciliation each require their own authorised live evidence; the passing local gateway tests cannot replace it.

## Deployment, export, and handover gates

- Harbourlight has six accepted local QA assets, two locked/tour-ready units, a validated `file://` static preview, zero releases, and no public URL.
- Cedar QA and Koya Official Intake QA also have zero releases. The bundled Koya example's historical public URL belongs to a read-only example and cannot serve as Harbourlight customer deployment evidence.
- No `.estateproject` or ZIP export exists inside the QA project library.
- Publishing targets are empty; no customer-authorised host, domain, TLS state, access mode, account identity, timezone, deployment URL, logged-out read-back, share link, or QR evidence exists.
- The app list contains the active QA bundle and a stale inactive production-ID app. Neither data domain contains a valid production licence, so this is not a second signed licensed installation.

Criteria 11, 12, 15, 16, 19, 26, and 28 therefore require a signed licence with the necessary export/publisher entitlements, a customer-authorised destination/domain, successful deployment and logged-out read-back, plus a genuinely separate signed licensed installation for portable handover. Deployment, domain mutation, upload, and public publication also require explicit user authority.

## Git and delivery boundary

- Repository branch: `main`.
- `HEAD` and `origin/main`: `5cf20818ce3e4a5bfdf0ae9ed4feca41ebf20cb6`.
- Ahead/behind: `0/0`.
- `Koya/real-estate-ai-studio/` and `.autonomous/complete-estate-studio-mvp/` are entirely untracked; Git tracks zero files in those paths.
- Numerous unrelated Koya assets remain present and untouched.

No commit, scoped staging, push, deployment, domain change, or public mutation is authorised by the current goal. Any future commit must be explicitly authorised and must stage only reviewed Estate Studio/tracker paths.

## Remaining acceptance state

Current matrix: 14 Pass, 4 Partial pass, 1 Implemented/not executed, and 9 Blocked.

| Criterion | Current state | Exact evidence still required |
| --- | --- | --- |
| 1 | Partial | Developer ID signed/notarised macOS customer install or native signed Windows install. |
| 6 | Blocked | Current exact image capability, price, quota, rights-safe inputs, and explicit package approval. |
| 7 | Blocked | At least two authorised Provider outputs and native accept/reject/non-reuse evidence. |
| 11 | Blocked | Licensed public/unlisted immutable release to an authorised target. |
| 12 | Blocked | Resulting customer URL opened outside the authoring session. |
| 15 | Blocked | Successful deployment read-back followed by verified release/share/QR creation. |
| 16 | Partial | Licensed exports from two differently branded company profiles, inspected for separation. |
| 18 | Blocked | Approved job switched to a real managed subscription without evidence/provider reuse. |
| 19 | Blocked | Portable export opened on another signed licensed installation without vendor cloud. |
| 20 | Partial | Live expired/exhausted rejection and authorised top-up on a real subscription account. |
| 21 | Blocked | One completed live request reconciled across desktop credit, gateway fingerprint, usage, and exact internal cost. |
| 22 | Implemented / not executed | Complete standard managed-subscription journey through a real configured gateway with no technical fields exposed. |
| 26 | Partial | Authorised generation, licensed build, and export from Harbourlight while Koya remains byte-unchanged. |
| 28 | Blocked | Import the exported project into another signed licensed installation and verify all retained records/releases. |

Task 46 remains open until all 28 criteria are Pass. Automated tests, QA ad-hoc installation, local gateway fixtures, and the read-only Koya example are supporting evidence only and do not convert the external rows above into acceptance.

## Resume inputs

Work can resume without redesign when the responsible owners provide or authorise:

1. exact macOS release identity plus protected notarisation and updater inputs;
2. native Windows build/test host plus certificate and timestamp/updater inputs;
3. a signed production licence covering required export, publisher, and portable-handover entitlements;
4. a rights-approved exact image test package and live Codex-image or managed-gateway capability/price/quota evidence;
5. an authorised managed-subscription account/top-up and provider cost read-back;
6. a customer publishing target/domain/TLS/access-mode decision and deployment authority;
7. a second signed licensed installation for portable handover; and
8. separate explicit authority for commit, push, deployment, or domain mutation when each action is intended.

Until those inputs change, there is no remaining local action that can truthfully satisfy the outstanding PRD acceptance evidence.
