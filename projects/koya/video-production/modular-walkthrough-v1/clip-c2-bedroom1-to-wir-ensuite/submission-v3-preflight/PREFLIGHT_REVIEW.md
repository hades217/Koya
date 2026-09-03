# Seedance preflight review

Decision: **PASS**
Reviewed at: 2026-08-27T23:38:00+10:00
No task submitted: **yes**
Package fingerprint: `sha256:efae57867cb3579fd13326dc285e744318d4f1ba85a8aab64d6932689dd0ee4a`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint state: Running; `video_editing` is supported.
- Mode: video editing using one explicit `reference_video`.
- Task count / output count: 1 / 1.
- Parameters: inherit the 5.000-second reference duration (`-1`), adaptive ratio, 1080p, generated audio on, watermark off, seed unavailable.
- Output directory: `outputs/`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-reference-motion-v6.mp4` | `reference_video` | One continuous render from `koya-106-bedroom1-wir-ensuite-fixed-shell-v6` | PASS; Bedroom 1, both openings, single WIR wardrobe bank, left recess, Ensuite and fixtures exist in one fixed shell before motion | `9271a811c3cf593524e8ea0c2518e41cdaaffad408cb633493a305ea37dfa71e` |

Provider URL: `https://relatives-simple-incentive-lewis.trycloudflare.com/00-reference-motion-v6.mp4`

- GET status: HTTP 200.
- Content-Type: `video/mp4`.
- Content-Length: 3,711,326 bytes.
- Remote SHA-256 equals the locked local SHA-256.
- The local server and HTTPS tunnel must remain alive until the provider has fetched the input and the task reaches a safe terminal state.

## Video review evidence

- Normal-speed playback: the user reviewed the V6 five-second local render and instructed submission to continue.
- Frame-level review: all 150 V6 frames were inspected through six consecutive 30 fps contact sheets.
- Whole-video checkpoints: `qa/STRUCTURAL_CHECKPOINT_SHEET_01.jpg` and `qa/STRUCTURAL_CHECKPOINT_SHEET_02.jpg`.
- Dense turn/threshold checkpoints: 12 fps overview plus 30 fps inspection around both doorway crossings and the Ensuite turn.
- Route overlay: `qa/CLIP_C2_ROUTE_OVERLAY.png`; rechecked against the one-page official `Koya marketing plan Apartment 106.pdf`.
- Timed motion ledger: `qa/TIMED_MOTION_LEDGER.md`; all intervals total 5.00 seconds.
- Persistent-landmark audit: both doorway pairs persist and pass with physical parallax; the right wardrobe bank remains fixed; the left recess remains secondary; double vanity, toilet and shower remain in one Ensuite.
- Technical probe: 5.000 seconds, 1920x1080, 30 fps, H.264, no source audio.
- Automated scene detection at threshold 0.12 returned no cut candidates; this was treated as supplementary only.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Temporal geometry | One fixed V6 shell and one continuous camera path | None | PASS; no competing temporal input. |
| Bedroom 1 to WIR | Existing opening persists ahead-left during approach and is crossed physically | None | PASS. |
| WIR | One north/camera-right wardrobe bank; south/camera-left shallow fixed recess | None | PASS; no whole-room still is uploaded. |
| Ensuite | Double vanity, toilet and shower exist before motion in east-to-west order | None | PASS; no later image introduces fixtures. |
| Appearance intent | Simplified but complete fixed shell | `qa/OFFICIAL_VISUAL_ANCHOR_SHEET.jpg`, human review only | PASS; not a provider input and cannot override topology. |
| Time of day | Stable neutral daytime | Official Koya interiors use bright neutral daytime | PASS. |

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` role.
- Route physically present: PASS; Bedroom 1, both thresholds, WIR, recess and Ensuite fixtures already exist in V6.
- Timing valid: PASS; intervals total 5.00 seconds.
- Real translation and rotation: PASS; not a crop, slideshow or static push-in.
- Buyer-viewing objective: PASS; forward travel is prioritised and the recess is not featured.
- No instruction/input contradiction: PASS.
- Forbidden cuts, morphs, wall crossing, room swaps, extra doors, a second wardrobe bank, widening and game-like floating motion are explicit.

## Provider and price evidence

- Live endpoint resolution on 2026-08-27: Running; resolved model `doubao-seedance-2-5-260628`; `video_editing` supported.
- Non-billable dry-run: PASS for request construction; exact endpoint, one explicit reference-video HTTPS URL, inherited duration, adaptive ratio, 1080p and generated audio.
- Live account `V2V1080Completion` unit price: CNY 0.03312 per thousand completion tokens, discounted from CNY 0.046 until the provider-listed end time of 2026-09-17T14:00:00+08:00.
- Immediately preceding same-endpoint five-second 1080p generated-audio task usage: 488,025 completion tokens; actual cost CNY 16.163388.
- Working estimate for this one task: **CNY 16.163388**.
- Exact total before generation: `unavailable`; actual cost depends on returned completion-token usage.
- Failure/cancellation billing status: `unavailable`.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. One approval authorises one task and one output only; it does not authorise retries or variants.

Exact approval phrase: **确认提交主卧 C2 V3，V6 单任务单输出，指纹 efae57867cb3，参考估算 CNY 16.163388，实际按完成 tokens 结算**
