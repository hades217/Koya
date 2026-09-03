# Seedance preflight review — Clip EF V10 V2V

Decision: **PASS**
Reviewed at: 2026-08-29T19:45:45+10:00
No task submitted: **yes**
Package fingerprint: `sha256:378668769e2a88b219222131f36df2f3164017615cf084d800174154bc666fda`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint status: `Running`; current metadata lists video input, `video_editing`, `multimodal_to_video` and `video_extension`.
- Mode: `video_editing` using one explicit `reference_video` only.
- Task count / output count: **1 / 1**.
- Parameters: inherit reference duration (`duration=-1`, source is exactly 8.000 seconds); adaptive ratio; 1080p; generated audio on; watermark off; seed unavailable.
- Output directory: `outputs/`.
- Active provider tasks at review: zero.
- Active local `arkcli +gen` create processes at review: zero.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-apartment-106-ef-v10-reference.mp4` | `reference_video` | One continuous 8.000-second, 1920x1080, 30 fps render from the plan-locked V10 Apartment 106 public-zone/Terrace fixed Three.js shell; one physical camera; all 240 frames reviewed | PASS | `0ffc88e8ae20fe5ea7782f00faf4415aca20132ef80bbaf21cc0367fc5b6f868` |

Provider URL: `https://fine-increase-friendship-templates.trycloudflare.com/00-apartment-106-ef-v10-reference.mp4`

- Complete remote GET: HTTP 200.
- Content-Type: `video/mp4`.
- Content-Length: 4,093,739 bytes.
- Remote SHA-256 matches the locked local SHA-256.
- The exact URL is bound into the package fingerprint.
- The local server and HTTPS tunnel must remain alive until Seedance has fetched the input.
- No `reference_image` is included. V9/V10 photoreal images are human-review evidence only and cannot override fixed topology.

## Video review evidence

- Normal-speed playback: PASS; full 8.000-second route reviewed.
- Reverse playback: PASS; `qa/REFERENCE_REVERSE_REVIEW.mp4` and `qa/REFERENCE_REVERSE_4FPS.jpg` show no hidden replacement, spatial dissolve or impossible threshold.
- Whole-video checkpoints: PASS; `qa/REFERENCE_WHOLE_8FPS.jpg` covers the complete shot.
- Dense threshold checkpoints: PASS; `qa/REFERENCE_THRESHOLD_12FPS.jpg` shows approach, both jambs, track expansion, crossing and post-threshold Terrace arrival.
- Dense Terrace turn/look-back checkpoints: PASS; `qa/REFERENCE_TURN_LOOKBACK_12FPS.jpg` keeps Terrace furniture, railing, opening and interior relationship continuous.
- Route overlay: PASS; `qa/PUBLIC_ZONE_ROUTE_OVERLAY.png` uses the official Apartment 106 plan and places the physical route inside Living and through the east Terrace boundary.
- Persistent landmarks: PASS; MPR opening, four-seat Dining, one Kitchen island, Kitchen line, TV wall, sofa, south/east glazing, the same east opening, the same track and Terrace furniture remain stable with real parallax.
- Reference is not a slideshow, Ken Burns crop, still-image zoom, crossfade, dissolve or multi-camera cut.

## Cross-input consistency

| Feature | Reference video | Human visual evidence | Result |
|---|---|---|---|
| Topology controller | One V10 fixed scene and camera | Official floor plan and route overlay | PASS |
| MPR | Compact, broad public opening, brief glance only | V9/V10 review keeps MPR/public-zone relationship | PASS |
| Dining/Kitchen | Four-seat Dining, one island, one Kitchen line | Same functional relationship | PASS |
| TV/Living | TV on short internal wall; plan-proportional Living | Same edge-TV and compact but deep Living identity | PASS |
| Glazing | Exactly two public exterior glazed faces | Same two-face glazing direction | PASS |
| Terrace | Same east threshold and wrap Terrace | Two lounge chairs, one round side table, four-seat outdoor dining and clear route | PASS |
| Time/colour | Stable daytime | Stable natural Brisbane daytime | PASS |

`qa/VISUAL_ANCHOR_V9_ACCEPTED_LOCAL.png` and `qa/VISUAL_ANCHOR_V10_ALTERNATE.png` are not provider inputs. The prompt describes their accepted material and furnishing direction without assigning unsupported control semantics to them.

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` in video-editing mode.
- Route physically present: PASS; every rotation, forward translation, threshold and endpoint already exists in the submitted video.
- Timing valid: PASS; nine continuous intervals total exactly 8.00 seconds.
- Real camera motion: PASS; approach, jamb parallax, track crossing, Terrace turn and look-back are already encoded.
- Immutable geometry matches the input: PASS.
- Allowed transformation is limited to photographic materials, daylight, natural micro-motion and audio.
- No instruction/input contradiction: PASS.
- Prompt explicitly rejects cuts, dissolves, morphs, static push-ins, wall/glass penetration, room swaps, extra doors/glazing/columns, Living scale drift, TV relocation, furniture drift, game-like motion and day/night changes.

## Provider and price evidence

- Live endpoint resolution on 2026-08-29: Running; `doubao-seedance-2-5-260628`; video input and `video_editing` supported.
- Non-billable dry-run: PASS; `DRY_RUN.json` contains the exact endpoint, complete Chinese prompt, one HTTPS `reference_video`, inherited duration, adaptive ratio, 1080p and generated audio. No generation task was created.
- Live account `V2V1080Completion` unit price: **CNY 0.03312 per 1,000 completion tokens**, discounted from CNY 0.046 through the provider-listed 2026-09-17 14:00 +08:00.
- Same-endpoint historical reference for an 8-second, 1080p, generated-audio V2V task: 779,625 completion tokens / **CNY 25.821180**.
- This one-task reference estimate: **CNY 25.821180**.
- Exact total before generation: `unavailable`; actual total is determined by returned completion-token usage.
- Failure/cancellation billing status: `unavailable`.

## Approval gate

No Seedance task has been submitted. One approval authorises exactly one task and one output, with no automatic retry or second candidate. Any prompt, input bytes, input role, URL, endpoint, model, duration, resolution, audio setting, price basis, task count or output count change invalidates this PASS.

Exact approval phrase:

**确认提交 Clip EF V10 V2V，指纹 378668769e2a，单任务单输出，唯一输入为 V10 reference_video，不附加整屋图片，8 秒、1080p、16:9 自适应、生成音频，参考估算 CNY 25.821180，实际按完成 tokens 结算，不自动重试。**
