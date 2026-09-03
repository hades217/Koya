# Seedance preflight review

Decision: **PASS**
Reviewed at: 2026-08-26T23:23:00+10:00
No task submitted: **yes**
Package fingerprint: `sha256:1c1b07e95fd00727742741800e1f8ac78c9dcfb413965e2cb93c4c7b1140cac9`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint state: Running.
- Mode: video editing using one `reference_video`.
- Task count / output count: 1 / 1.
- Parameters: inherit the reference duration (`-1`, reference is 5.000s), adaptive ratio, 1080p, generated audio on, watermark off, seed unavailable.
- Output directory: `outputs/`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-reference-motion.mp4` | `reference_video` | One continuous render from `koya-106-master-suite-fixed-shell-v1` | PASS; Bedroom 1, both openings, WIR, Ensuite and fixtures exist in one shell before motion | `274b1bad6cee5f4205041942e840cd97a4c118d306ec978466743894f3f4c248` |

Provider URL: `https://dealers-catherine-hull-police.trycloudflare.com/00-reference-motion.mp4`

- GET status: HTTP 200.
- Content-Type: `video/mp4`.
- Content-Length: 2,810,490 bytes.
- Remote SHA-256 equals the locked local SHA-256.
- The local server and HTTPS tunnel must remain alive until provider input fetch and task completion.

## Video review evidence

- Normal-speed motion: five-second route fits the timed ledger and holds the final sales view.
- Reverse playback: `qa/REVERSE_REVIEW_ONLY.mp4`; no hidden replacement exposed.
- Whole-video checkpoints: 12 fps sheets plus all 120 frames in six 24 fps sheets under the fixed-scene project's `qa-reference-v1/`.
- Dense turn/threshold checkpoints: 24 fps inspection around 1.50–3.75 seconds.
- Route overlay: `../route-audit-v1/CLIP_C2_ROUTE_OVERLAY.png`.
- Persistent-landmark audit: bed remains during the initial turn; Bedroom/WIR jambs, opposing wardrobes, WIR/Ensuite jambs, shower, toilet and vanity remain fixed and pass with physical parallax.
- Technical probe: 5.000s, 1920x1080, 24 fps, H.264.

## Cross-input consistency

| Feature | Reference video | Human-review anchor | Result |
|---|---|---|---|
| Master-suite sequence | Bedroom 1 -> WIR -> Ensuite | Official plan and `qa/MASTER_SUITE_VISUAL_ANCHORS.jpg` | PASS |
| Bedroom identity | bed, east glazing and west WIR opening persist | official master-bedroom render | PASS for material intent; no provider image input |
| WIR | narrow east-west connector with wardrobes on both sides | concept WIR connector | PASS for appearance intent; no provider image input |
| Ensuite fixture order from south-side final pose | shower left/west, toilet centre, double vanity right/east | official Koya Ensuite render | PASS |
| Time of day | neutral daytime throughout | all active appearance references use daytime | PASS |

No whole-room `reference_image` is attached to the provider request. The only temporal and architectural input is the continuous fixed-scene video.

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` role.
- Route physically present: PASS; both thresholds and all destination fixtures exist before motion.
- Timing valid: PASS; intervals total 5.00 seconds.
- Real translation and rotation: PASS; not a crop or static push-in.
- No instruction/input contradiction: PASS.
- Forbidden cuts, morphs, wall crossing, extra doors, shortcut, corridor widening and game-render appearance are explicit.

## Provider and price evidence

- Live endpoint resolution: Running; `video_editing` supported; resolved model `doubao-seedance-2-5-260628`.
- Non-billable dry-run: `DRY_RUN.json`; correct endpoint, one explicit reference-video URL, inherited duration, adaptive ratio, 1080p and generated audio.
- Live account V2V1080 price: CNY 0.03312 per thousand completion tokens.
- Prior same-endpoint 5-second 1080p generated-audio task usage: 488,025 completion tokens.
- Estimated total: CNY 16.163388.
- Exact total before generation: unavailable.
- Failure/cancellation billing status: unavailable.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint.

Exact approval phrase: **确认提交主卧 C2，预计 CNY 16.163388**
