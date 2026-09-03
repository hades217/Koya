# Seedance preflight review — Clip EF V5

Decision: `BLOCKED`

Reviewed at: 2026-08-29 Australia/Brisbane

No task submitted: yes

Package fingerprint: `sha256:e315aeb3441ea9d7090f88504a7faeb1b27ca7877901f5a7698a74f9868b8dce`

## Blockers

- The live provider pricing response reports `IsOverdue: true` for `doubao-seedance-2-5`. Paid submission is blocked until the account is no longer overdue and the same exact package is rechecked.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Mode: `video_editing`
- Task count / output count: `1 / 1`
- Parameters: 8 seconds, 1920x1080 inherited 16:9, 1080p, generated audio, return last frame, no watermark, no explicit ratio override
- Output directory: `outputs/clip-ef-i2v-v5`

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `koya-106-full-route-fixedscene-v5-8s.mp4` | `reference_video` | Plan-locked Apartment 106 fixed public-zone shell | PASS: one fixed scene and one continuous camera | `45a65bef376fabefed57ec64e37e677a6b3f99c1e81a8631b00502120eabfdae` |

Provider URL: `https://downloading-surface-march-just.trycloudflare.com/koya-106-full-route-fixedscene-v5-8s.mp4`

Transport verification: HTTP 200, `Content-Type: video/mp4`, 7,274,432 bytes, remote SHA-256 byte-identical to local.

## Video review evidence

- Normal-speed playback: PASS for one continuous fixed-scene motion path.
- Reverse playback: PASS; no cut, dissolve, teleport, duplicate room or reset.
- Whole-video checkpoints: 4 fps contact sheet at `../reference-video-v1-hyperframes/qa-full-route-fixedscene-v5/contact-sheet-4fps.jpg`.
- Dense turn/threshold checkpoints: 12 fps from 5.6 to 7.95 seconds at `../reference-video-v1-hyperframes/qa-full-route-fixedscene-v5/contact-sheet-threshold-12fps.jpg`.
- Route overlay: `../floorplan-audit-v5/public-zone-route-overlay-v5.png`.
- Persistent-landmark audit: one kitchen line, one island, one TV wall, one sofa, continuous south/east glazing, one east sliding threshold and one terrace remain fixed.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Architecture topology | Fixed-scene V5 | No other provider input | PASS |
| Camera timing | Fixed-scene V5 | Chinese prompt uses the same 0.00-8.00 second schedule | PASS |
| Terrace endpoint | Southward terrace settle with door frame at right edge | Chinese prompt requests the same endpoint | PASS |

The independently generated V7 whole-room renders remain human review anchors only. They are not attached to the provider request. Rejected V6, V8 and V9 assets are excluded.

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` role.
- Route physically present: PASS.
- Timing valid: PASS; all intervals total exactly 8.0 seconds.
- No instruction/input contradiction: PASS.

## Provider and price evidence

- Live endpoint resolution: PASS; endpoint `Running`, model `doubao-seedance-2-5-260628`, `video_editing` supported.
- Non-billable dry-run: PASS; payload contains one text prompt and one explicit reference-video URL, 8 seconds, 1080p, audio enabled.
- Price: `V2V1080Completion` CNY `0.03312` per 1,000 completion tokens. Exact task total is unavailable before completion-token usage is known.
- Account readiness: BLOCKED; live response reports `IsOverdue: true`.
- Failure billing status: unavailable; no paid task was created.

## Approval gate

No Seedance task may be created while the account is overdue. After the provider reports `IsOverdue: false`, rerun transport, pricing and fingerprint checks. The user must then approve the exact refreshed fingerprint before one paid task and one output may be submitted.

Exact approval phrase: unavailable until the provider account blocker is cleared and the package is re-fingerprinted.
