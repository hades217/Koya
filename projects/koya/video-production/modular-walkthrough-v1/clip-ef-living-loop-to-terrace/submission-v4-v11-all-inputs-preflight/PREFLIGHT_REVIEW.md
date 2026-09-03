# Seedance preflight review — Clip EF V11 plus ten accepted renders

Decision: **PASS**

Reviewed at: `2026-08-29T21:28:09+10:00`

No task submitted: **yes**

Package fingerprint: `sha256:e50b3093d6be85c1645c8459bc300292bf37ed10c51141892279bed318b305c1`

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Live endpoint status: `Running`
- Supported input modalities: text, image, video, audio
- Provider task type: `multimodal_to_video`
- Task count / output count: `1 / 1`
- Parameters: `omni_reference_task_type=reference`, `duration=8`, `ratio=16:9`, `resolution=1080p`, `generate_audio=true`, `return_last_frame=true`, `watermark=false`, `output_format=mp4`
- Output directory: `outputs/clip-ef-v11-all-inputs-v1`

## Inputs

- One locked V11 continuous reference video, SHA-256 `0693dcc7cf4040465b6ab5ea4fce68d73af713f9f6f42b46b14a353fb9b2a4d1`.
- Ten accepted rendered storyboard images, all listed and hashed in `PREFLIGHT_FINGERPRINT.json`.
- All eleven provider URLs were fetched with HTTP GET and byte-verified against the locked local SHA-256 values.

## V11 video review

- H.264 High, 1920 × 1080, 30 fps, 8.000 seconds, 240 frames.
- One fixed Three.js scene and one continuous first-person camera.
- Whole-clip review at 8 fps; threshold and terrace turn/look-back review at 12 fps.
- No black frame or detected freeze.
- No cut, dissolve, crossfade, teleport, repeated room, wall penetration or furniture penetration found.
- The same east opening remains visible through approach, physical threshold crossing and final look-back.
- Evidence: `../reference-video-v11-render-locked/REFERENCE_PREVIEW_QA_V11.md` and its `qa-final/` contact sheets.

## Cross-input consistency

- The previous rejected V10 video is not present.
- V11 was rebuilt against the accepted V7 storyboard: compact MPR, four-seat Dining, one island, north internal TV wall, broad Living, L-shaped sofa, south/east glazing, one east opening and the wrap Terrace remain aligned.
- The Chinese prompt assigns the V11 video to continuous camera motion and the ten images to buyer-facing appearance, while explicitly avoiding any false claim that ordinary reference images are guaranteed ordered keyframes.
- The route timing in the prompt matches the V11 camera ledger and the accepted rendered storyboard sequence.

## Prompt review

- Complete Chinese prompt included and hashed.
- All eleven inputs explicitly required.
- Eight-second physical route, final frame, daylight, spatial locks, sound and negative constraints are present.
- Previous outdoor-furniture conflict was removed; the Terrace now follows the accepted open walking strip and planted-edge renders.

## Provider and price evidence

- Non-billable dry-run contains one `reference_video`, ten `reference_image` items and the complete prompt; client fidelity is `partial`, so this proves request construction but not provider compliance.
- Current discounted `V2V1080Completion` unit price: CNY `0.03312 / 1,000 completion tokens`, discounted from CNY `0.046` through `2026-09-17T14:00:00+08:00`.
- Exact total before generation: `unavailable`; the provider returns completion-token usage only after completion.
- Account overdue: no.

## Accepted residual risk

Seedance does not guarantee that ten ordinary reference images act as an ordered timeline or that each one influences the result equally. This package is therefore a single-task owner-directed multimodal continuity attempt. PASS means the exact inputs are internally reviewed and technically ready; it does not guarantee visual success. No automatic retry is authorised.

## Approval gate

This PASS does not submit the task. Any change to the prompt, input bytes or URLs, roles, model, parameters, task count, output count or price evidence invalidates the fingerprint. The exact fingerprint and current price status require explicit user approval before one paid task may be created.
