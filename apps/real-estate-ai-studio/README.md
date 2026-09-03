# Estate Studio

Estate Studio is a local-first Tauri 2 desktop workspace for evidence-controlled off-the-plan property marketing. It keeps developments, company profiles, source evidence, AI approvals, QA, builds, releases, and portable handover data in isolated local workspaces.

## Current MVP implementation

- Multi-company project library, archive/restore, clean structure duplication, multi-unit creation, and a versioned read-only Koya example.
- Portable `.estateproject` export/import with checksums, schema migration, collision protection, cache exclusion, symlink/traversal blocking, and secret scanning.
- Source intake with owner, permission, MIME, SHA-256, evidence class, duplicate linkage, immutable review events, and structured rejection.
- Floor-plan viewport/editor, assisted draft, manual hotspots/openings/entrance, connectivity checks, version history, and room-graph locking.
- Standard property workflow and Advanced technical mode, room asset matrix, identity/panorama slots, exact generation packages, current capability/price/quota gates, one-task/one-output approval, durable output provenance, rejection, correction, and QA.
- Managed OpenAI gateway contract with short-lived memory-only desktop sessions, subscription allowances, idempotent credits/top-ups, provider failure release, and usage/cost reconciliation that preserves `unavailable` values.
- Deterministic panorama seam repair and mobile/4K/8K derivatives with decoded-size and memory limits, continuity-controlled fallback plans, and resumable background jobs.
- Manifest-driven zero-dependency static tour with desktop/mobile/low-memory tiers, unit/room deep links, panorama/floor-plan/video controls, still fallback, disclosures, keyboard/focus/reduced-motion support, and privacy-safe optional same-origin analytics.
- Immutable static builds, plain export, customer-owned directory publishing adapter, cookie-free HTTPS deployment read-back, verified releases, unit links/QRs, supersession, and rollback candidates.
- Seven governed A4 sales-document workflows, AI poster packages, and HyperFrames storyboard packages without claiming unrendered media exists.
- Ed25519 offline licence verification, edition/role gates, protected settings and secret-file references, stable/beta signed updater workflow, and fail-closed macOS/Windows release preflights.

## Local development and exact QA

```bash
npm install
npm run tauri dev
```

For native acceptance without colliding with an older installed `com.landiq.estatestudio` bundle, package the isolated QA app:

```bash
npm run tauri:qa:app
open -n "src-tauri/target/debug/bundle/macos/Estate Studio QA.app"
```

To exercise the local disk-image mount/copy/launch lifecycle, build the QA DMG separately:

```bash
npm run tauri:qa
```

The combined command builds the DMG and app, verifies the app's complete ad-hoc bundle signature and identifier, verifies the DMG checksum, and requires Gatekeeper to reject the non-notarised QA app. The QA bundle uses `com.landiq.estatestudio.qa` and separate application data. It is ad-hoc signed for local testing only and is not a customer installer. A QA DMG mount or launch never satisfies Developer ID signing, Gatekeeper, notarisation, updater, or clean-machine release acceptance.

## Verification

```bash
npm run verify
```

This runs the frontend production build, five static-runtime/accessibility tests, six managed-gateway tests, and 46 Rust unit/integration/migration/property tests. The current JavaScript chunk is 527,116 bytes minified and triggers Vite's 500 kB advisory warning.

## Release gates

Development and QA packages are not distributable releases.

- `npm run macos:release:preflight` and `npm run macos:release` require an explicitly selected Developer ID identity, complete Apple notarisation credentials, signed-updater keys, and HTTPS endpoint metadata. Post-build checks require strict signing, Gatekeeper acceptance, and a stapled notarisation ticket.
- `npm run windows:release:preflight` and `npm run windows:release` must run on Windows with an exact installed code-signing certificate, HTTPS timestamp service, signed-updater inputs, and native MSI/NSIS prerequisites. Post-build checks require valid Authenticode signatures.
- Publishing requires a licensed publisher role, a complete approved project, a customer-authorised target, HTTPS deployment, and a separate logged-out read-back. Build, upload, deployment, and verified public release are distinct states.

No production signing/notarisation credentials, updater keys/endpoints, Windows host/certificate, managed provider account/current price/quota, customer hosting/domain, deployment URL, or production signed licence are included in this workspace.

## Evidence and handoff

- `docs/AUTOMATED_TEST_MATRIX.md`
- `docs/LOCAL_ACCEPTANCE_EVIDENCE_2026-09-02.md`
- `docs/MVP_28_CRITERIA_STATUS_2026-09-02.md`
- `docs/OPEN_DECISIONS_AND_EXTERNAL_GATES.md`
- `docs/FINAL_EXTERNAL_BLOCKER_AUDIT_2026-09-02.md`
- `docs/UPDATES_AND_RECOVERY.md`
- `docs/MACOS_RELEASE_ACCEPTANCE.md`
- `docs/WINDOWS_RELEASE_ACCEPTANCE.md`

The 28-criterion status document is authoritative for MVP acceptance. The MVP is not release-accepted while any criterion is not `Pass`; build logs and provider task status cannot substitute for required native, visual, paid, licensed-handover, or public read-back evidence.
