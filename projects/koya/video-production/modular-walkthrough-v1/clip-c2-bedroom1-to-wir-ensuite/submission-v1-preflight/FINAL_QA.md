# Clip C2 Seedance output — final QA

Date: 2026-08-26 (Australia/Brisbane)

Decision: **REJECTED — do not merge**

The provider task succeeded technically, but the generated clip fails the mandatory Clip C1-to-C2 continuity and appearance gate. No replacement task is authorised.

## Provider result

- Task ID: `cgt-20260826212744-nn77h`
- Provider status: `succeeded`
- Model: `doubao-seedance-2-5-260628`
- Output: `outputs/cgt-20260826212744-nn77h.mp4`
- Output SHA-256: `167f9223969546f46d9286d01b99abe88b154a2f251fdda3c8c0b5c39d99eaa0`
- Completion tokens: 471,825
- Actual cost: CNY 15.626844

## Technical inspection

- Container duration: 4.736 seconds
- Resolution: 1920x1080
- Aspect ratio: 16:9
- Frame rate: 24 fps
- Video codec: HEVC
- Audio: AAC stereo
- Audio level: mean -39.0 dB, maximum -17.2 dB
- File size: 5,722,277 bytes

## Mandatory failures

- **0.000 seconds — hard join discontinuity.** Clip C1 ends in a highly detailed bedroom with a low upholstered bed, sheer curtains, leafy terrace outlook and warm photographic materials. Clip C2 starts in a different synthetic bedroom with a raised platform bed, no matching curtains, a different glazing composition, a distant urban outlook and different furniture. It is not the accepted Bedroom 1 endpoint.
- **0.000–1.250 seconds — room identity and finish drift.** Bed construction, artwork, bedside furniture, glazing, exterior view, lighting and material detail do not remain consistent with Clip C1 or the approved master-bedroom visual anchor.
- **Approximately 1.250–2.500 seconds — WIR appearance is under-resolved.** The narrow connector survives topologically, but the wardrobe banks read primarily as blank timber wall panels and do not deliver the approved fitted-WIR sales view with shelves, hanging rails, clothing and drawers.
- **Entire clip — realism gate fails.** Surfaces, lighting and motion retain a simplified game-render/blockout character rather than the requested real residential cinematography and the approved Koya visual-anchor quality.

## What passed

- One continuous forward camera move; no obvious cut, dissolve, teleport or wall penetration was found across the 113 decoded frames.
- The route proceeds from a bedroom through a narrow WIR-like connector and through a second physical doorway into the Ensuite.
- Doorframe edges show forward parallax during both threshold crossings.
- The final Ensuite broadly preserves the required plan-relative fixture order: shower left/west, toilet central and double vanity right/east, with a doorway edge at frame right.
- No visible person, hands, operator reflection, text, logo or watermark appears.
- Daylight and exposure remain stable within this clip.

The passing internal route does not compensate for the failed start identity and visual-quality contract. This output must remain audit evidence only and must not be stitched after Clip C1.

## Evidence

- `qa/final-v1/FFPROBE.json`
- `qa/final-v1/sheet-001-020.jpg`
- `qa/final-v1/sheet-021-040.jpg`
- `qa/final-v1/sheet-041-060.jpg`
- `qa/final-v1/sheet-061-080.jpg`
- `qa/final-v1/sheet-081-100.jpg`
- `qa/final-v1/sheet-101-113.jpg`
- Accepted Clip C1 endpoint: `../../clip-c-hub-to-bedroom1/submission-v1-preflight/qa-final-v1/end.png`
- Visual target sheet: `qa/MASTER_SUITE_VISUAL_ANCHORS.jpg`

## Required repair before any new paid submission

Rebuild the deterministic C2 source so its first rendered frame is an exact image-and-camera match to the accepted Clip C1 terminal frame, then carry that same detailed bedroom identity into a fully dressed WIR and the approved Ensuite. The corrected local reference must be reviewed at normal speed and frame-by-frame, and a new exact package must pass preflight and receive separate user approval. Do not retry the current package or rely on prompt wording to repair the mismatched source scene.
