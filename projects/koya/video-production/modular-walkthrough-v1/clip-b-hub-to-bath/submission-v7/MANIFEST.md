# Apartment 106 Clip B V7 submission manifest

Status: **REJECTED** after user review. Do not use, splice, publish or extend this output.

## Inputs

- `inputs/00-reference-motion.mp4` — 5.000 s, 1920x1080, 24 fps deterministic topology and camera source of truth.
- `inputs/01-style-kitchen-start.jpg` — 1280x720 material/style reference only.
- `inputs/02-style-threshold.jpg` — 1280x720 Laundry/Bath threshold reference only.
- `inputs/03-style-main-bath.jpg` — 1280x720 floor-plan-corrected compact main-Bath reference only.

## Submission contract

- endpoint: `ep-20260812221158-hb576`
- resolved model: `doubao-seedance-2-5-260628`
- modality: video editing / reference-video generation
- duration request: `-1` (video-editing mode inherits the 5.000-second reference-video duration)
- ratio request: `adaptive` (video-editing mode inherits the 16:9 reference-video ratio)
- resolution: 1080p
- native audio: enabled
- outputs requested: one
- reference input roles: motion uses explicit `reference_video`; all three stills use explicit `reference_image`; no independent still is assigned `first_frame` or `last_frame`
- prompt: `PROMPT.md`

## Price evidence

Account price query on 2026-08-26 returned:

- `V2V1080Completion`: CNY 0.03312 per thousand output tokens (account price; original CNY 0.046).
- Exact task cost is unavailable before completion because the generation token count is not known in advance.

## QA gate after generation

- poll the same task ID; never resubmit because a queued/running task has not finished;
- inspect the full clip at normal speed;
- extract 4 fps checkpoints and at least 12 fps from 1.2–3.2 seconds;
- reject any change to island count, MPR wall, Bath-door identity, Laundry separation or Bath fixture layout;
- approve only if the camera physically crosses the same doorway with no cut, morph or wall clipping.

## Rejected pre-generation task

- `cgt-20260826090552-cj4vw`: failed server parameter validation before generation; `usage={}`. The server required video-editing parameters `ratio=adaptive` and `duration=-1`. Do not poll or reuse this task.

## Successful task

- task ID: `cgt-20260826090750-2fszv`
- status: `succeeded`
- local output: `outputs/cgt-20260826090750-2fszv.mp4`
- actual media: HEVC 1920x1080 at 24 fps, AAC stereo 32 kHz, 5.056 s
- usage: 488,025 completion tokens
- calculated account charge: 488.025 x CNY 0.03312 = **CNY 16.163388**
- temporary reference-video tunnel: closed after success

## Corrected QA finding

- `qa/full-contact-4fps.jpg`: full-clip 4 fps contact sheet
- `qa/turn-contact-12fps.jpg`: 1.2–3.2 s dense turn audit
- `qa/threshold-contact-12fps.jpg`: 2.2–4.2 s dense threshold audit
- The earlier automated scene-score check was insufficient and led to a false PASS.
- The deterministic reference video contained a simplified single-door wet-zone approach, while the threshold still introduced a different two-door Laundry/Bath composition. These inputs conflicted structurally even though the still was described as style-only.
- Seedance rebuilt the wet-zone approach during the move instead of preserving one fixed apartment shell. The visible continuity is therefore invalid for a physical property walkthrough.
- Output `outputs/cgt-20260826090750-2fszv.mp4` is rejected in full.
