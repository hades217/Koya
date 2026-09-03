# Release notes

## 0.1.0 — local MVP candidate, not released

Status date: 2026-09-02 AEST.

### Added

- Local-first multi-company/multi-project desktop authoring and versioned read-only Koya example.
- Evidence intake, room-graph authoring, asset matrix, governed generation packages, panorama processing, static tour build/preview, publishing/read-back, release history, QR/share links, creative packages, offline licensing, secure settings, and signed-update foundations.
- Separately deployable managed subscription gateway and local Codex controlled-edit adapter.
- Portable project handover and deterministic security/recovery controls.
- macOS and Windows protected release preflights and acceptance procedures.
- Unified automated suite and native local acceptance evidence.

### Fixed during acceptance

- Added a valid empty updater configuration so development/QA builds launch while still reporting signed updates unavailable.
- Isolated the exact QA bundle from stale installed production-ID packages.
- Added randomized path-component isolation and blocked malicious unit-directory escape.
- Tightened Unix project-library/project-root permissions to `0700` and manifests/company registries to `0600`.
- Added visible keyboard focus and improved primary/muted text contrast.
- Added queued supplied-panorama mobile/4K/8K delivery processing, accepted panorama selection, room-video binding, and native panorama/floor-plan/video interaction controls.
- Added atomic source preflight with category/extension, decoded-image, and container-signature validation plus visible failed-input recovery guidance.
- Separated the licence-free validated static URL preview from licensed immutable release creation, and made accepted panoramas the deterministic still fallback when no separate fallback was curated.
- Added explicit bundle-level ad-hoc signing and a reproducible QA package verifier after the install run exposed a linker-only signature with no sealed resources.

### Verification

- Frontend production build: pass, with a 527,116-byte minified JavaScript advisory.
- Static runtime/accessibility: 5/5 pass.
- Managed gateway: 6/6 pass.
- Rust: 46/46 pass.
- Exact ad-hoc QA bundle: native launch, settings/update failure gate, two-unit unrelated-project creation, responsive background 8K processing, panorama/floor-plan/video interaction, relaunch persistence, validated static build, and browser unit/room URL switching pass.
- QA DMG: checksum/mount/copy/installed-path launch/relaunch/temp-uninstall lifecycle passed while preserving app data; strict bundle signature verification now passes, while ad-hoc identity and Gatekeeper rejection correctly remain non-release blockers.
- Codex: authenticated native capability read-back and one approved read-only Harbourlight audit passed without project mutation, provider submission, or credential persistence.

### Not released

No commit, push, signed/notarised customer package, Windows native package, update publication, production licence, gateway deployment, paid generation, customer deployment, domain change, or public read-back is claimed. See `docs/OPEN_DECISIONS_AND_EXTERNAL_GATES.md` and `docs/MVP_28_CRITERIA_STATUS_2026-09-02.md`.
