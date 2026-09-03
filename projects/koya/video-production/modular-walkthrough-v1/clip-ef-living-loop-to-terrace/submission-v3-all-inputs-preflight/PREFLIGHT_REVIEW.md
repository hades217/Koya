# Seedance preflight review — Clip EF all inputs

Decision: **BLOCKED**

Reviewed at: 2026-08-29T20:58:00+10:00

No task submitted: **yes**

Package fingerprint: `sha256:092b613d6b8c48a6f5a3160d58bfe0ba545de679837bb01fa13ec3861452078d`

## Blockers

1. The sole `reference_video` is the V10 3D source whose final look and spatial representation the project owner has explicitly rejected. The ten `reference_image` files are independently generated rendered storyboard views, not frames rendered from that same V10 scene. The package therefore contains conflicting whole-room inputs without same-scene provenance.
2. The prompt asks Seedance to retain the reference video's camera path while replacing its unacceptable 3D/spatial appearance with the ten images. The provider exposes all items as ordinary references; it exposes no separate control guaranteeing that the video governs only motion while the images govern only final architecture/materials.
3. Client dry-run proves that all 11 references are present in the request construction, but it is explicitly `fidelity: partial`; it does not prove server acceptance or that all ten images will affect the result.

The package cannot receive `PASS` until a continuous reference video is rendered from the same approved scene and visual design as the final storyboard, or the owner selects a separately reviewed image-only experiment. The already rejected V10 3D video cannot be reused as the topology controller for another paid task.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Live endpoint status: `Running`
- Supported input modalities: text, image, video, audio
- Supported task types: `multimodal_to_video`, `video_editing`, `video_extension`
- Mode: `multimodal_to_video`
- Task count / output count: `1 / 1`
- Parameters: `omni_reference_task_type=reference`, `duration=8`, `ratio=16:9`, `resolution=1080p`, `generate_audio=true`, `return_last_frame=true`, `watermark=false`, `output_format=mp4`
- Output directory: `outputs/clip-ef-all-inputs-v1`

## Inputs and roles

| Input | Role | Result |
|---|---|---|
| `00-reference-video.mp4` | `reference_video` | Included; byte-verified remote URL; rejected 3D/spatial source blocks the task |
| `00-master-public-zone-lock.png` | `reference_image` | Included; byte-verified remote URL |
| `01-mpr-dining-opening.png` | `reference_image` | Included; byte-verified remote URL |
| `02-dining-single-island-kitchen.png` | `reference_image` | Included; byte-verified remote URL |
| `03-tv-wall-living-width.png` | `reference_image` | Included; byte-verified remote URL |
| `04-wrap-glazing-sliding-bay.png` | `reference_image` | Included; byte-verified remote URL |
| `05-align-east-sliding-bay.png` | `reference_image` | Included; byte-verified remote URL |
| `06-threshold-approach.png` | `reference_image` | Included; byte-verified remote URL |
| `07-cross-east-threshold.png` | `reference_image` | Included; byte-verified remote URL |
| `08-terrace-arrival.png` | `reference_image` | Included; byte-verified remote URL |
| `09-terrace-lookback-endpoint.png` | `reference_image` | Included; byte-verified remote URL |

All eleven provider URLs were fetched with HTTP GET and produced the same SHA-256 as the locked local files.

## Video review evidence

- Normal-speed and frame-level review: already completed in the V10 V2V package.
- Motion controller: one continuous 8-second path.
- Blocking result: the owner has now rejected the controller's 3D/spatial representation as an acceptable basis for the final EF generation.
- Route ledger: `../full-panorama-to-terrace-v1-preflight/MOTION_LEDGER.md`
- Rendered visual review: `../full-panorama-to-terrace-v1-preflight/IMAGE_REVIEW_MANIFEST.md`
- Official topology source: Apartment 106 floor plan referenced by the current production SOT.

## Cross-input consistency

| Feature | Reference video | Rendered images | Result |
|---|---|---|---|
| Camera movement | Continuous 8-second 3D path | Independent still viewpoints | Motion exists only in video |
| Final realism/materials | Simplified 3D/architectural visualisation | Buyer-facing rendered target | Conflict |
| Whole-room provenance | V10 Three.js scene | Independently generated storyboard images | Not the same scene |
| Provider role separation | Ordinary `reference_video` | Ordinary `reference_image` | No documented motion-only/style-only separation |
| Guaranteed image order | N/A | None | Images are not guaranteed keyframes |

## Prompt review

- Complete Chinese prompt: included and fingerprinted.
- All eleven inputs explicitly named: yes.
- Eight-second route and final frame contract: yes.
- Unsupported guarantee avoided: yes; prompt states images are not guaranteed ordered keyframes.
- Instruction/input contradiction: **unresolved and blocking** because the prompt asks the model to repair rejected video geometry/appearance using independent whole-room images.

## Provider and price evidence

- Live endpoint resolution: PASS.
- Non-billable dry-run: request contains one `reference_video`, ten `reference_image` inputs and the full prompt; fidelity remains partial.
- Current discounted `V2V1080Completion` unit price: CNY `0.03312 / 1,000 completion tokens`.
- Exact total before generation: `unavailable` because completion-token usage is returned only after the provider task completes.
- Account overdue: no.

## Approval gate

No Seedance task may be created from this fingerprint because the decision is `BLOCKED`. The full request has been assembled and verified, but technical inclusion of every file does not resolve the contradictory architecture controller.
