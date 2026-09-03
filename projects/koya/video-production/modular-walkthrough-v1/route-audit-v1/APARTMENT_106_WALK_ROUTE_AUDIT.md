# Apartment 106 Walk Route Audit V1

Status: spatial reset; no video or storyboard generation authorised. The first manually approximated 3D blockout and camera map were rejected because they were not dimensionally traced from the official plan. They are retained only under `rejected/blockout-v1/`.

## Evidence hierarchy

1. Official Apartment 106 floor plan: `production-assets/assets/official/floorplans/apartment-106.png` and its source PDF.
2. Official Koya interior renders: material and styling references only; they are not confirmed Apartment 106 geometry.
3. Generated images and videos: review evidence only; they cannot override the floor plan.

Plan orientation: north is up, east is right.

## Literal walk from the apartment entry

### P0 - Entry threshold

- Position: just inside the single north entry door.
- Facing: south.
- Must see: compact straight Hall; storage mass on the plan-east side.
- Must not see: Living, Dining, island, MPR interior, Bath interior or a long hotel corridor.

### P1 - South through the Hall

- Continue south without inventing a side corridor.
- Laundry and Bath occupy the plan-east/north wet-area block, but their actual door thresholds must remain fixed to the plan-supported lower wet-area edge.
- A room opening must not migrate along the Hall wall as the camera advances.

### P2 - Hall meets the central kitchen axis

- This is the first major turn/reveal point.
- Turn east/left into the long central axis.
- North/camera-left: linear kitchen wall and fixed wet/private-room volumes.
- South/camera-right near the western end: the solid Bedroom 2 and MPR volumes must exist before the open Dining/Living zone.
- The island cannot begin directly at the Hall corner and cannot replace the MPR wall.

### P3 - Advance east beside the kitchen

- Camera travels between the north linear kitchen and the south-side MPR/open-zone edge.
- The single island sits farther east, south of the kitchen run.
- The island remains one fixed object with physical parallax; it may not enlarge into a showroom-scale slab.
- Bedroom 1/WIR/Ensuite remain fixed north of the axis and cannot dissolve into cabinetry.

### P4 - Dining orientation hub

- Dining is south of the island and west of Living.
- Living is east of Dining, not a replacement for the Dining zone.
- MPR remains west of Dining as a real enclosed volume.
- From this point, each room branch must start from a verified camera pose, not an independently generated 'hub' image.

### P5 - Living and Terrace

- Living remains immediately west of the private Terrace glazing.
- Terrace is the east/south exterior edge and must not become an arbitrary high-rise balcony.
- Bedroom 1 remains north of Living and has its own fixed access from the central axis.

## Current Clip A failure points

| Approx. time | What the generated video shows | Floor-plan problem |
| --- | --- | --- |
| 0.0-2.5s | Long Hall with Laundry/open utility room exposed on camera-left | Wet-area threshold/door relationship is not proven and shifts into a corridor-like presentation |
| 3.0-5.5s | Camera rounds the corner and immediately sees a large open kitchen and island | The near solid MPR/Bedroom 2 mass and correct western approach geometry are not preserved |
| 5.5-8.0s | Island dominates the route and public space widens rapidly | Island position and scale behave like an independent marketing render rather than the Type 106 plan |
| 8.0-10.0s | Dining/Living/Terrace appear attractively | Broad ordering is plausible, but it is downstream of incorrect geometry and cannot validate the route |

## Production gate before any further paid task

- Do not manually estimate wall lengths or camera coordinates from the plan image.
- Obtain a dimensioned architectural drawing, CAD/BIM export, or developer-confirmed room dimensions before claiming a measured spatial blockout.
- If only the marketing plan remains available, keep the next deliverable strictly as a 2D route annotation on the untouched plan; do not convert it into a claimed-accurate perspective interior.
- Place fixed walls, every door opening, kitchen run, island, MPR volume, Dining, Living and Terrace threshold.
- Walk the virtual camera through P0-P5 and export review frames at every threshold and 45-degree turn.
- Compare each frame with the plan before applying Koya materials.
- Only after the blockout passes user review may a five-second room branch be submitted to Seedance.
- Seedance must not be asked to infer apartment topology from two generated endpoint images.
