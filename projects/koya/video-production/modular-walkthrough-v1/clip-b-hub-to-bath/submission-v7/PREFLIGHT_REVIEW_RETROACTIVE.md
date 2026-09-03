# Seedance preflight review — retrospective V7 test

Decision: **BLOCKED**
Reviewed at: 2026-08-26T11:20:00+10:00
No new task submitted by this review: yes
Package fingerprint: `sha256:72d6085a7dbd5dbecd610f44c3b5f7e3796092fdaac1397f09be6de503410156`

## Blockers

- The reference video is a simplified white-box shell and does not contain the final Laundry/Bath composition.
- All three `reference_image` files are independently generated whole-room images with `same_scene_id=unavailable`.
- The kitchen, threshold and Bath images introduce composition and architecture not present in the reference video.
- The prompt claims the generic `reference_image` role will affect materials only; the provider exposes no such materials-only role.
- The package expects negative prompt wording to prevent structural influence from contradictory visual inputs.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Mode: video editing
- Task count / output count: 1 / 1
- Parameters: `duration=-1`, `ratio=adaptive`, `resolution=1080p`, `generate_audio=true`
- Output directory: `outputs`

## Inputs and roles

| Input | Role | Provenance | Same-scene result |
|---|---|---|---|
| `00-reference-motion.mp4` | `reference_video` | simplified WebGL shell | baseline only |
| `01-style-kitchen-start.jpg` | `reference_image` | independent whole-room generation | BLOCKED |
| `02-style-threshold.jpg` | `reference_image` | independent whole-room generation | BLOCKED |
| `03-style-main-bath.jpg` | `reference_image` | independent whole-room generation | BLOCKED |

## Video review evidence

- Normal-speed playback: simplified path is visible, but its wet-zone geometry is incomplete.
- Whole-video checkpoints: 26 white-box checkpoints exist.
- Dense turn/threshold checkpoints: the reference shows a simplified approach that cannot support the later two-door still.
- Route overlay: Apartment 106 route exists, but the rendered shell does not fully implement it.
- Persistent-landmark audit: fails because Laundry/Bath landmarks are absent or disagree across inputs.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Kitchen start | white-box aisle | independent photoreal dining/kitchen pose | BLOCKED |
| Wet-zone approach | simplified door geometry | frontal Laundry-left/Bath-right scene | BLOCKED |
| Bath endpoint | simplified compact room | independent photoreal Bath pose | BLOCKED |

## Prompt review

- Supported role semantics only: **fail**; it invents materials-only semantics for `reference_image`.
- Route physically present: **partial/fail**; final wet-zone topology is missing from the reference shell.
- Timing valid: provider parameters are valid for video editing.
- No instruction/input contradiction: **fail**.

## Provider and price evidence

- Live endpoint resolution: previously confirmed for the historical package.
- Non-billable dry-run: request construction only; it did not prove topology.
- Price: historical account read-back recorded in the package.

## Approval gate

No retry is permitted from this package. There is no valid approval phrase. A new package requires a complete same-scene reference video and a fresh review/fingerprint.
