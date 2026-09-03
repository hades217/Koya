# Seedance preflight review

Decision: **WITHDRAWN AFTER POST-SUBMISSION AUDIT — DO NOT REUSE**
Reviewed at: 2026-08-28T18:03:42+10:00
Historical note: this report preceded task `cgt-20260828173707-5hxv6`. Its original PASS was later found semantically invalid. See `SUBMISSION_AUDIT_V2.md`.
Package fingerprint: `sha256:068752e310f71025f99dba3e975663bc1201ea13b13f7726b4cc4b6497d87be9`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint state: Running; video input and `video_editing` are supported.
- Mode: video editing using one explicit `reference_video` only.
- Task count / output count: 1 / 1.
- Parameters: inherit the 8.000-second reference duration (`duration=-1`); adaptive ratio; 1080p; generated audio on; watermark off; seed unavailable.
- Output directory: `outputs/`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-reference-motion-v6.mp4` | `reference_video` | One continuous eight-second V6 render from `koya-106-living-dining-kitchen-terrace-fixed-shell-v6`; all 240 encoded frames inspected | PASS; MPR opening, Dining, one island, Kitchen, TV wall, Living, wrap glazing, one east threshold and Terrace exist in one fixed shell before motion | `ae03d71427b6ac15a174034d404f6b677abf5e13ea34d5cea7c92e209a495781` |

Provider URL: `https://fly-profit-respect-solve.trycloudflare.com/00-reference-motion-v6.mp4`

- GET status: HTTP 200.
- Content-Type: `video/mp4`.
- Content-Length: 6,390,687 bytes.
- Remote SHA-256 equals the locked local SHA-256.
- The local server and HTTPS tunnel must remain alive until the provider has fetched the input and the task reaches a safe terminal state.
- No `reference_image` input is present. The rendered storyboard images are human-review evidence only and are excluded from the provider request.

## Video review evidence

- Normal-speed playback: the user reviewed the local V6 route and instructed work to continue into the Seedance stage.
- Reverse playback: PASS; `qa/REVERSE_REVIEW_ONLY.mp4` and `qa/REVERSE_4FPS_CONTACT.jpg` expose no hidden scene replacement, spatial dissolve or impossible threshold.
- Whole-video checkpoints: PASS; `qa/FORWARD_8FPS_CONTACT.jpg` samples the complete eight seconds.
- Dense turn/threshold checkpoints: PASS; `qa/THRESHOLD_12FPS_CONTACT.jpg` checks the east opening, track crossing, Terrace sweep and look-back.
- Frame-level review: PASS; all 240 encoded frames were inspected through ten consecutive contact sheets in the source QA folder.
- Route overlay: PASS; `qa/CLIP_EF_ROUTE_OVERLAY.png` agrees with the official Apartment 106 plan and shows the MPR glance, public-zone loop and east Terrace crossing.
- Persistent-landmark audit: PASS; one island, the north Kitchen line, internal TV wall, sofa, south/east glazing, the same open east bay and the same track remain fixed and pass with physical parallax.
- Technical probe: 8.000 seconds, 1920x1080, 30 fps, H.264, no source audio.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Temporal geometry | One fixed V6 shell and one continuous camera path | None | PASS; no competing temporal input. |
| MPR | Brief glance through its one existing public opening; camera does not enter | None | PASS. |
| Dining and Kitchen | Compact four-seat Dining and exactly one island remain in one connected space | None | PASS. |
| Living | Internal north TV wall, sofa and buyer-readable width stay fixed | None | PASS. |
| Glazing | Full-height south and east facade glazing remains exterior glazing | None | PASS. |
| Terrace threshold | One east open bay and the same track are approached and crossed physically | None | PASS. |
| Appearance intent | Complete simplified fixed shell | Rendered storyboard V6, human review only | PASS; storyboard images are not provider inputs and cannot override topology. |
| Time of day | Stable daytime in the reference | Prompt requires stable neutral Brisbane daytime | PASS. |

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` role in video-editing mode.
- Route physically present: PASS; every requested camera movement, landmark and threshold already exists in the submitted fixed-scene video.
- Timing valid: PASS; the eight intervals total exactly 8.00 seconds and match the locked route contract.
- Real translation and rotation: PASS; the reference is not a slideshow, crop animation or static-image push-in.
- Buyer-viewing objective: PASS; the route gives a brief MPR read, a full public-zone panorama, the glazing, then a physical Terrace arrival and look-back.
- No instruction/input contradiction: PASS.
- Explicit rejection controls: cuts, dissolves, morphs, room swaps, moving walls/glazing/furniture, duplicated island, invented doors/columns, narrow Living, solid exterior walls, wall/glass penetration, game-like floating motion and day/night changes are forbidden.

## Provider and price evidence

- Live endpoint resolution on 2026-08-28: Running; resolved model `doubao-seedance-2-5-260628`; video input and `video_editing` supported through the provider generation API.
- Non-billable dry-run: PASS; `DRY_RUN.json` locks the exact endpoint, prompt, one explicit HTTPS reference-video URL, inherited duration, adaptive ratio, 1080p and generated audio. The dry-run created no generation task and incurred no generation charge.
- Live account `V2V1080Completion` unit price: CNY 0.03312 per 1,000 completion tokens, provider-discounted from CNY 0.046 through the provider-listed `2026-09-17T14:00:00+08:00`.
- Working token estimate: 780,840 completion tokens, calculated proportionally from repeated same-endpoint five-second 1080p generated-audio V2V usage of 488,025 tokens.
- Working estimate for this one eight-second task: **CNY 25.8614208**.
- Exact total before generation: `unavailable`; actual cost depends on returned completion-token usage.
- Failure/cancellation billing status: `unavailable`.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. One approval authorises exactly one task and one output; it does not authorise retries, variants or a second candidate. Any prompt, input, URL, role, parameter, endpoint, model, price or task-count change invalidates this PASS.

Exact approval phrase: **确认提交 Clip EF V6 单个 Seedance 2.5 任务，指纹 068752e310f7，1080p、8秒、生成音频，参考估算 CNY 25.8614208，实际按完成 tokens 结算**
