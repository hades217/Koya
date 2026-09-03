# Clip B V3 — Storyboard Prompt Set

Tool: built-in `image_gen`, project-bound raster generation/edit workflow.

## Shared contract

- Apartment 106 floor-plan crop is the mandatory geometry reference.
- Natural 32–35mm lens, 1.58m eye height, 16:9 composition.
- Exactly one island; one linear kitchen; no people or hands.
- Physical continuity takes priority over cinematic lighting.
- No long corridor, invented door, merged wet room, wall pass-through or topology morph.

## Node-specific prompts

- Node 1: position the camera at the Hub facing west through the passage; island left/south, kitchen right/north, wet doors obliquely right near the far end, solid wall ahead.
- Node 2: move the same camera forward about two metres without changing view direction or architecture.
- Node 3: move to the west end and rotate exactly 90 degrees right/north; Laundry left, main Bath right.
- Node 4: align with the right-hand Bath doorway and physically cross between the fixed jambs; timber outside, tile inside, Laundry remains outside left.
- Node 5: move one metre inside and stop; preserve rear bathtub/shower, centre-right WC and single right-wall vanity.

Two targeted correction passes were required for Node 1: first remove an invented centred end door, then move the wet doors from the wrong left/south wall to the correct right/north wall.
