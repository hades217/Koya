# Seedance preflight review

Decision: **PASS**
Reviewed at: 2026-08-27T13:20:46+10:00
No task submitted: **yes**
Package fingerprint: `sha256:bbd377b8bcb7d3048953d577e7e8b5e20807cd36f1c1cc7a9a09d130e6da48bc`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint state: Running; video editing is listed as supported.
- Mode: video editing using one explicit `reference_video`.
- Task count / output count: 1 / 1.
- Parameters: inherit the 5.000-second reference duration (`-1`), adaptive ratio, 1080p, generated audio on, watermark off, seed unavailable.
- Output directory: `outputs/`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-reference-motion-v4.mp4` | `reference_video` | One continuous render from `koya-106-master-suite-fixed-shell-v4` | PASS; Bedroom 1, both openings, the single WIR wardrobe bank, the left recess, Ensuite and fixtures exist in one shell before motion | `63c058d7b90d1511dc2835bce7e903033a0eeada84f3e945b94c8c77c8ef6522` |

Provider URL: `https://package-particle-red-pressure.trycloudflare.com/00-reference-motion-v4.mp4`

- GET status: HTTP 200.
- Content-Type: `video/mp4`.
- Content-Length: 2,144,135 bytes.
- Remote SHA-256 equals the locked local SHA-256.
- The local server and HTTPS tunnel must remain alive until the provider has fetched the input and the task reaches a safe terminal state.

## Video review evidence

- Normal-speed playback: user reviewed the five-second local V4 and instructed the workflow to continue after the camera was restored to forward travel.
- Reverse playback: `qa/REVERSE_REVIEW_ONLY.mp4`; no hidden room replacement, crossing or snap-back was exposed.
- Whole-video checkpoints: all 150 frames under the V4 fixed-scene QA set; combined view at `qa/STRUCTURAL_CHECKPOINT_SHEET.jpg`.
- Dense turn/threshold checkpoints: 30 fps inspection across both doorway crossings and the Ensuite turn.
- Route overlay: `../route-audit-v1/CLIP_C2_ROUTE_OVERLAY.png`.
- Timed motion ledger: `qa/TIMED_MOTION_LEDGER.md`; all intervals total 5.00 seconds.
- Persistent-landmark audit: both doorway jamb pairs persist and pass with physical parallax; the right wardrobe bank remains fixed; the left recess has a fixed rear plane and two returns; vanity, toilet and shower remain in one Ensuite.
- Technical probe: 5.000 seconds, 1920x1080, 30 fps, H.264, no source audio.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Temporal geometry | One fixed V4 shell and one continuous camera path | None | PASS; no competing temporal input. |
| WIR | One north/camera-right wardrobe bank; south/camera-left fixed recess | None | PASS; no whole-room still is uploaded. |
| Ensuite | Double vanity, toilet and shower exist before motion | None | PASS; no later image introduces fixtures. |
| Appearance intent | Simplified but complete fixed shell | `qa/OFFICIAL_VISUAL_ANCHOR_SHEET.jpg`, human review only | PASS; the sheet is not provider input and its role limits are recorded in `qa/VISUAL_ANCHOR_REVIEW.md`. |
| Time of day | Stable neutral daytime | Official Koya interiors use bright neutral daytime | PASS. |

## Prompt review

- Supported role semantics only: PASS; one explicit `reference_video` role.
- Route physically present: PASS; both thresholds, WIR, recess and Ensuite fixtures already exist in the V4 input.
- Timing valid: PASS; intervals total 5.00 seconds.
- Real translation and rotation: PASS; not a crop, slideshow or static push-in.
- Recess treatment: PASS; prompt forbids a wall-focused turn and keeps the recess secondary.
- No instruction/input contradiction: PASS.
- Forbidden cuts, morphs, wall crossing, room swaps, extra doors, a second wardrobe bank, widening and game-like floating motion are explicit.

## Provider and price evidence

- Live endpoint resolution on 2026-08-27: Running; resolved model `doubao-seedance-2-5-260628`; `video_editing` supported.
- Non-billable dry-run: PASS for request construction; exact endpoint, one explicit reference-video HTTPS URL, inherited duration, adaptive ratio, 1080p and generated audio. Client preview correctly reports provider-side role materialisation and account routing as execution-time steps.
- Live account V2V1080 unit price: CNY 0.03312 per thousand completion tokens, discounted from CNY 0.046 during the provider's current promotional window.
- Prior same-endpoint five-second 1080p generated-audio C2 task usage: 471,825 completion tokens; actual cost CNY 15.626844.
- Working estimate for this one task: **CNY 15.626844**.
- Exact total before generation: `unavailable`; actual cost depends on returned completion-token usage.
- Failure/cancellation billing status: `unavailable`.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. One approval authorises one task and one output only; it does not authorise retries or variants.

Exact approval phrase: **确认提交主卧 C2 V2，单任务单输出，参考估算 CNY 15.626844，实际按完成 tokens 结算**
