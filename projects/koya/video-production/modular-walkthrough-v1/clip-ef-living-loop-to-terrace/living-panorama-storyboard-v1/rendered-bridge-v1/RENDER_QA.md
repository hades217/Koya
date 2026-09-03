# Render-level bridge storyboard QA

Status: `REJECTED - WRONG LIVING SOUTH/EAST GLAZING / FORBIDDEN AS VIDEO INPUT`

## Rejection finding

The Apartment 106 plan shows the Living room opening to the wraparound Terrace along both the south and east exterior edges through grouped full-height glazing/sliding doors, interrupted only by structural columns and frames. V1 incorrectly introduced long solid decorated walls in several east-facing public-space panels. This changes the buyer's understanding of the Living room and blocks the intended wraparound-window panorama.

V1 is audit history only. Do not use any V1 panel as a generation input.

## Inputs and roles

- `structural-contact-3x3.png`: camera pose and panel-order guide.
- `apartment-106.png`: topology and adjacency source of truth.
- `koya-2br-living-kitchen.jpg`: official material, furniture and lighting language only.
- `koya-2br-kitchen.jpg`: official joinery and one-island language only.
- `03-master-terrace-v1.jpg`: accepted terrace threshold and Level 1 visual language.

## Historical visual review

- [x] Nine panels, fixed 3x3 order, no people or text burned into frames.
- [x] Neutral Brisbane daytime remains consistent.
- [x] Warm oak, cream cabinetry, muted stone and slim dark glazing are consistent.
- [x] Exactly one kitchen island is present.
- [x] Public room reads as residential rather than a narrow corridor or oversized showroom.
- [x] Door jamb and track are visible before and during the terrace crossing.
- [x] Terrace endpoint retains the approved planter, railing and leafy Level 1 impression.
- [x] No night/day change, fisheye or game-render styling.

## Fatal limitation

- [ ] South and east Living boundaries match the official wraparound glazing. `FAILED`.

## Other limitations

- The generated render board is a visual concept sheet, not a metric 3D model.
- It cannot prove exact continuous geometry between panels.
- It must not be used as an ordered multi-reference image timeline for Seedance.
- Final video topology must come from one completed deterministic scene/reference video.

## Generation record

- Mode: built-in image generation, multi-reference sketch-to-render.
- Output: `rendered-contact-3x3-v1.png` plus nine mechanically cropped panels.
- No Seedance task was submitted.
