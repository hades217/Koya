# MVP 28-criteria status — 2026-09-02 AEST

`Pass` requires the exact user-visible or external evidence named by the PRD. `Implemented / not executed` means the controlled workflow exists and automated coverage passes, but this acceptance run did not produce the required visual, provider, installation, licensed-second-installation, or public read-back evidence. `Blocked` names an external gate and never substitutes zero or an estimate.

| # | Status | Current evidence or exact missing gate |
| --- | --- | --- |
| 1 | Partial pass | A QA DMG was checksum-verified, mounted read-only, copied to an isolated temporary install directory, launched from that installed path, quit/relaunched, and read back all four persisted QA projects. The QA overlay now produces a complete bundle-level ad-hoc signature that passes strict `codesign` validation; the repaired DMG was installed and launched again successfully. Gatekeeper correctly rejects this non-notarised QA package, so a Developer ID signed/notarised customer DMG or native Windows installer is still required for Pass. |
| 2 | Pass | Native QA UI created Harbourlight Residences with A1 and A2. |
| 3 | Pass | Native Harbourlight intake imported two floor plans, supplied panoramas, and an H.264 video. A separate writable Koya QA project imported the registered `SRC-007` A-level official 6000×4000 living/kitchen render, preserved its SHA-256, recorded internal-only permission, and moved it from `IMPORTED` to `OFFICIAL / ACCEPTED` through a separate operator action. |
| 4 | Pass | Native evidence was classified and accepted; A1 and A2 assisted drafts remained unapproved until separate operator review and room-graph lock. |
| 5 | Pass | After graph lock, native room cards identified missing identity and panorama coverage before those roles were assigned. |
| 6 | Blocked | Exact paid image package approval requires current provider capability, price, quota, rights-approved inputs, and explicit approval. These are unavailable. |
| 7 | Blocked | No authorised provider outputs exist to accept/reject. Rejection non-reuse and immutable review history are automated. |
| 8 | Pass | The native QA UI processed two accepted 4096×2048 sources into verified seam-repaired, mobile, 4K, and 8K files. The second run used one persisted idempotent background task, kept the native UI responsive, and completed at attempt 1 with dimensions, SHA-256 values, derivative paths, and the no-new-detail 8K process recorded. |
| 9 | Pass | Harbourlight opened in the native Desktop 1× viewer, then switched to Phone portrait with the same accepted tour state. |
| 10 | Pass | The exact QA app built Harbourlight's validated local static URL preview without creating a release. In a real browser, direct `?unit=a1&room=qa-zone-1` loading succeeded; adjacent-room navigation changed the URL to `a1/qa-zone-2`, and the unit selector changed it to `a2/qa-zone-a`, with matching visible room/status state and no console errors. Public sharing remains a separate criterion 11/12 gate. |
| 11 | Blocked | Public/unlisted immutable publishing exists, but no licensed publisher entitlement, complete release, or customer target was supplied. |
| 12 | Blocked | No deployed URL exists for logged-out/outside-authoring read-back. |
| 13 | Pass | The native local tour rotated from 180° to 198°, jumped Zone 1 → Zone 2 → Zone 1 through floor-plan controls, and played the assigned H.264 video with `Pause`, elapsed-time, and two-second duration controls visible. Public URL read-back remains separately blocked under criterion 12. |
| 14 | Pass | Harbourlight's native tour retained the visible `CONCEPT STYLE ONLY` disclosure while navigating between accepted stops. |
| 15 | Blocked | Verified release/share/QR creation requires a successful deployment read-back and publisher entitlement. |
| 16 | Partial pass | Koya, Sunward, and Riverside QA are separate profiles/projects with distinct project roots and brand records. Export separation cannot be completed without the signed licence entitlement. |
| 17 | Pass | Native Settings now reads the authenticated account through the official Codex app-server `account/read` interface and visibly reports `codex-cli 0.151.0 · ChatGPT team · app-server · CONNECTED` without exposing the account email or tokens. The operator also approved one project-scoped read-only Harbourlight audit. Codex returned accepted evidence, both units' local-tour status, and current publication blockers, preserved missing facts as `unavailable`, proposed no project draft, and explicitly reported no file, field, provider-task, or generation-job change. Project mtime/state remained unchanged and a credential-pattern scan found no credential material in the project. |
| 18 | Blocked | Managed provider switch requires a configured gateway session/capability and approved job. Gateway URL/session are unavailable. |
| 19 | Blocked | Portable round trip is automated, but a second signed, licensed installation and an accepted unrelated tour are unavailable. |
| 20 | Partial pass | Expired/exhausted allowance and idempotent top-up are gateway-tested; no authorised live subscription/top-up account was supplied. |
| 21 | Blocked | Reconciliation logic preserves `unavailable` provider usage/cost, but no completed live generation ledger exists. |
| 22 | Implemented / not executed | Standard workflow hides keys, model names, tokens, prompts, JSON, terminal, and deployment commands; managed generation cannot run without a gateway/session. |
| 23 | Pass | The visible native flow created, persisted, imported, classified, locked, completed, and previewed a two-unit project with property-language actions. A deliberately corrupt PNG then failed before project copy with explicit guidance to choose an intact PNG/JPEG/WebP; the operator stayed in the same form, selected a valid 6000×4000 source, imported and accepted it, while the invalid asset count and filesystem copy remained zero. |
| 24 | Pass | Advanced mode exposed Harbourlight's project ID and exact project-relative source paths; switching modes did not change the accepted local tour or its disclosure. |
| 25 | Pass | Native QA opened Koya/library and created unrelated Harbourlight for Sunward with separate identity and data root. |
| 26 | Partial pass | Harbourlight reached accepted local tours for both units using supplied rights-safe inputs while Koya remained read-only and untouched. Paid generation, licensed build, and export remain blocked. |
| 27 | Pass | Native clean duplication created `Cedar QA Clean Template` for `Riverside QA` with two generic units, zero evidence files, no customer facts, no room graph, and no release records. |
| 28 | Blocked | Automated bundle round trip retains identity, structure, evidence, QA, and releases; a second licensed installation is unavailable. |

The MVP is not release-accepted while any criterion above is not `Pass`. Build logs and automated tests are supporting evidence only and are not being used as substitutes for paid, visual, installation, licensed-handover, or public read-back gates.
