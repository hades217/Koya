---
workflow: general-video
flow: automation
storyboard: yes
message: "Apartment 106 Clip B deterministic five-second camera reference"
destination: seedance-reference-video
aspect: 1920x1080
language: none
audience: internal-production
length: 5s
angle: floorplan-locked-first-person-route
---

## Intent

Create a free local deterministic reference video for Seedance 2.5. This is not the sales master. Its only purpose is to lock Apartment 106 topology and camera movement before any paid generation.

## Source of truth

- `../../../route-audit-v2/CLIP_B_EXACT_PLAN_ROUTE_V2.png`
- `../../../../production-assets/references/floorplans/Koya marketing plan Apartment 106.pdf`
- `../SEEDANCE_2_5_V7_ENGINEERING_PLAN.md`

## Literal route

B0 living-side hub -> B1 island north aisle -> B2/B3 west along the fixed kitchen -> one clockwise/right turn north -> B4 cross the single main Bath doorway -> B5 stop fully inside the compact Bath.

## Non-negotiable geometry

- fixed kitchen on camera-right/north during westward travel;
- single island on camera-left/south during the first half;
- solid MPR wall on camera-left/south after the island;
- one main Bath doorway, with stable jamb identity;
- compact Bath, Laundry separate to the west;
- one camera, no edits or image transitions.

