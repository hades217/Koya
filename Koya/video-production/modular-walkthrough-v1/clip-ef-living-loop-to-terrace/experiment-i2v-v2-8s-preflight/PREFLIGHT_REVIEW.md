# Seedance preflight review

Decision: `INVALIDATED AFTER PROVIDER PARAMETER REJECTION / DO NOT RETRY`

Reviewed at: 2026-08-29 Australia/Brisbane.

No task created: yes.

Package fingerprint: `sha256:1a3ac54da0067a921bdb2f4c9a463c0c5478611865c8893ec143f671db196f16`

## Blockers

- None for the restricted eight-second Living-to-Terrace segment.

## Scope boundary

- This is not the complete MPR-to-Terrace panorama.
- It covers only one continuously visible scene: Living position -> existing open east sliding bay -> same visible Terrace.
- The MPR, Kitchen, unseen rooms and a large look-back turn are explicitly excluded to avoid unsupported topology invention from a single first frame.

## Exact task

- Endpoint: `ep-20260812221158-hb576`.
- Live-resolved model: `doubao-seedance-2-5-260628`.
- Live endpoint state: `Running`, region `cn-beijing`.
- Mode: first-frame image-to-video.
- Task count / output count: `1 / 1`.
- Parameters: `8 seconds`, `16:9`, `1080p`, generated audio on, watermark off, return last frame on, seed unavailable.
- Output directory: `outputs`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-first-frame.png` | `first_frame` | Byte-identical copy of V7 plan-rebuild frame 05 | PASS for the visible Living/glazing/threshold/Terrace scene | `ece6e45c5045f8906abe5bc3881512e258e3f8d27e4c305e4cac6a11904eac55` |

No other visual input is attached. Rejected V6, V8 and V9 assets are excluded.

## Structural and visual review evidence

- Official source: `Koya/Koya marketing plan Apartment 106.pdf` and `floorplan-audit-v5/public-zone-crop.png`.
- Exact opening preview: `inputs/00-first-frame.png`, 1672 x 941, effectively 16:9.
- Timed movement: recorded in `MOTION_LEDGER.md`; totals exactly 8.0 seconds.
- Threshold proof: pre-approach, approach, crossing and post-threshold checkpoints are specified in `STRUCTURAL_CHECKPOINT_REVIEW.md`.
- Visual-anchor role separation: recorded in `VISUAL_ANCHOR_REVIEW.md`; V7 frames 06-08 are QA targets only and are not provider inputs.
- Persistent landmarks: sofa, continuous glazing, black mullions, one open east bay, track, Terrace tile and planting remain fixed throughout the requested route.

## Cross-input consistency

| Feature | Provider input | Other provider input | Result |
|---|---|---|---|
| Living scale | First frame only | None | No contradiction |
| East sliding bay | First frame only | None | No contradiction |
| Threshold and Terrace | Already visible in first frame | None | No contradiction |
| Lighting and palette | First frame only | None | No contradiction |

## Prompt review

- Supported role semantics only: PASS; one explicit `first_frame` role.
- Route physically present: PASS for a direct forward path through the already visible open bay.
- Timing valid: PASS; six intervals total exactly 8.0 seconds.
- Real motion requested: PASS; forward translation, threshold parallax and jamb occlusion are explicit; zoom-only motion is forbidden.
- No instruction/input contradiction: PASS within the restricted scope.
- Unsupported MPR/Kitchen generation: excluded.
- Prompt SHA-256: `080b3098396fad87a2f31394fec32971e40a277ee4003df5838abaf8d79cdeb2`.

## Provider and price evidence

- Authentication: logged in through the active Platform profile at review time.
- Endpoint resolution: live read-back confirmed the endpoint is Running and resolves to `doubao-seedance-2-5-260628`, with image/video/audio input capability and video output.
- Non-billable dry-run: PASS for request construction; payload contains `duration: 8`, `ratio: 16:9`, `resolution: 1080p`, audio true, watermark false and return-last-frame true.
- Exact I2V price: `unavailable`. Live account pricing returned V2V/NV2V items but no I2V charge item; no substitute estimate is used.
- Failure, cancellation or provider-rejection billing: `unavailable`.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. Any prompt, input, role, endpoint, resolved model, parameter, task/output count or price-basis change invalidates this PASS.

Exact approval phrase:

`确认提交 Clip EF I2V V2，单任务单输出，指纹 1a3ac54da006，8秒、16:9、1080p、生成音频，费用 unavailable，实际按提供方完成用量结算`
