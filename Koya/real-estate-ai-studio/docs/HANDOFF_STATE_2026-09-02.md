# Handoff state — 2026-09-02 AEST

## Source and Git

- Repository root: `/Users/lightman/Documents/sites/Koya`.
- Branch: `main`; `HEAD` and `origin/main` are both `5cf2081`; ahead/behind is `0/0`.
- The complete `Koya/real-estate-ai-studio/` application and `.autonomous/complete-estate-studio-mvp/` tracker are untracked. They have not been staged, committed, or pushed.
- Numerous unrelated Koya panorama/image/PDF/temp assets are also untracked. They were preserved and must not be swept into an Estate Studio commit.
- A future commit requires explicit authorisation and scoped staging of reviewed Estate Studio/tracker files only. Generated `node_modules`, `dist`, `target`, QA app data, and ignored release overlays remain outside Git.

## Build and tests

- Current manifest schema: v30. Browser-preview creation, reads, and writes share the same v30 constant; legacy localStorage records are migration-normalised instead of being rewritten as v26.
- `npm run verify`: frontend build plus 3/3 browser/schema-migration, 6/6 runtime, 6/6 gateway, and 48/48 Rust tests pass.
- `git diff --check` has no tracked diff because the application is currently untracked; file-level whitespace/syntax checks pass.
- Minified frontend JavaScript is 527,116 bytes and retains a Vite chunk-size advisory.

## Package and installed-app states

- Exact local QA bundle: `src-tauri/target/debug/bundle/macos/Estate Studio QA.app`.
- QA bundle ID/data: `com.landiq.estatestudio.qa`; ad-hoc local signature; launched and read back through native accessibility; Harbourlight persistence verified.
- Fresh QA DMG: `src-tauri/target/debug/bundle/dmg/Estate Studio QA_0.1.0_aarch64.dmg`, 16,724,049 bytes, SHA-256 `2baf2460a47d39c04a904e4594a5d7755bf7a53e430ce3913b5b3a23ec11a371`. Its isolated mount/copy/launch/relaunch/uninstall loop passed and preserved all four QA projects. The repaired DMG app now has a complete bundle-level ad-hoc signature and passes strict `codesign` validation.
- Harbourlight now has six accepted test inputs, two locked unit graphs, three approved local tour stops, two verified mobile/4K/8K derivative sets, an assigned representative MP4, and native desktop/phone, panorama-rotation, floor-plan-navigation, video-playback, responsive background-processing, and browser URL-deep-link evidence. The second derivative run completed through one persisted idempotent task at attempt 1. Its validated static preview switched `a1/qa-zone-1` → `a1/qa-zone-2` → `a2/qa-zone-a` with matching visible state and no browser console errors. All generated fixtures remain explicitly test-only and `Concept Style Only` where applicable.
- A separate writable `Koya Official Intake QA` project now contains two locally accepted `SRC-007` A-level official renders with byte-for-byte source/copy hash equality and internal-only usage permission. The same native form rejected a corrupt PNG before project copy, displayed plain-language recovery guidance, and accepted the valid replacement without failed-input residue. The bundled Koya example remains read-only; Harbourlight and Cedar remain separate company/project roots.
- Native Codex criterion 17 passed through one operator-approved, project-scoped read-only Harbourlight audit. Settings now binds through the official app-server account interface and reported `codex-cli 0.151.0 · ChatGPT team · app-server · CONNECTED` without exposing email or token material; the audit returned accepted evidence, both unit states, and publication blockers, proposed no draft, changed no project file/state, submitted no provider/generation task, and left no credential pattern in the project. App-server account binding does not by itself claim an image model, exact panorama dimensions, price, or image quota.
- Existing production-name release-path `Estate Studio.app`/DMG remain stale development artifacts. The fresh QA DMG is also non-distributable: its app signature is valid only as ad-hoc, has no TeamIdentifier, is not notarised, and is Gatekeeper-rejected.
- Production macOS signed/notarised package: unavailable; the current preflight fails closed because no release identity was explicitly selected and protected notarisation/updater inputs remain unavailable.
- Windows native signed MSI/NSIS: unavailable; the current preflight rejects the macOS host and requires native Windows execution.
- QA-only installation/relaunch/uninstall evidence: passed in an isolated temporary directory. Customer signed/notarised installation, signed update, and clean-machine uninstall evidence remain unavailable.

## Service, deployment, and public states

- Gateway implementation/tests: local pass; production gateway deployment and authenticated subscription account unavailable.
- Provider capability/current price/quota and paid output: unavailable; no provider task submitted.
- Customer publishing target/hosting account/domain/TLS/deployed URL: unavailable.
- Static build/export/read-back workflows: implemented and automated locally. The newly separated licence-free Harbourlight URL preview build passed its native static validation and browser deep-link run without creating a release. The licensed immutable-release build and portable export remain correctly rejected because no signed licence is installed; no customer deployment or bundle was produced.
- Public/unlisted/private verified release, share URL, and QR for Harbourlight: unavailable because there is no authorised deployment/read-back.
- Koya example remains read-only and `internal_only`; no commercial redistribution claim.

## Completion boundary

Internal implementation and local acceptance are substantially complete. The current PRD matrix is 14 Pass, 4 Partial pass, 1 Implemented/not executed, and 9 Blocked. Tasks 42, 43, and 46 remain evidence-gated by real macOS notarisation, a Windows host/signing environment, provider/rights/account data, customer hosting/domain, public read-back, and a second signed licensed installation. Task 48 records this boundary; it does not convert those external gates into completed release acceptance.

The authoritative presence/absence checks, current preflight failures, remaining-criterion table, and exact resume inputs are recorded in `docs/FINAL_EXTERNAL_BLOCKER_AUDIT_2026-09-02.md`.
