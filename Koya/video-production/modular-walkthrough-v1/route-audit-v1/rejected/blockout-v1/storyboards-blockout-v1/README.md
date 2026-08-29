# Clip B Deterministic Blockout Storyboard V1

Status: local spatial-previsualization draft; no Seedance task authorised.

Source of truth: official Apartment 106 floor plan. The white-box scene uses fixed walls and fixed camera poses rather than AI-inferred transitions.

## Frames

- `B0.png`: Living/Dining-side circulation pose facing west.
- `B1.png`: east of the single island, moving west through the clear gap.
- `B2.png`: west kitchen axis; MPR remains a solid volume on camera-left/south.
- `B3.png`: wet-area junction with two separate fixed openings visible.
- `B4.png`: camera aligned with the main Bath doorway between the fixed jambs.
- `B5.png`: camera fully inside the compact Bath.
- `CLIP_B_BLOCKOUT_STORYBOARD_V1.png`: six-frame review sheet.

## Gate

This blockout validates route and collision logic only. It is intentionally not a marketing render. Do not submit it to Seedance until the user approves the route. After approval, generate materialized storyboard frames one at a time from these fixed camera poses; reject any frame that adds a front-facing end door, removes the MPR mass, duplicates the island, merges Laundry and Bath, or moves a threshold.

The earlier ImageGen eight-panel sheet and both B0 attempts are rejected and were not copied into this project folder.
