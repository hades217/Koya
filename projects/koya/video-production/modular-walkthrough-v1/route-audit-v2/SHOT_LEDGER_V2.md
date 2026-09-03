# Apartment 106 Clip B - Exact 2D Shot Ledger V2

Status: corrected 2D route draft on an untouched official PDF render, awaiting visual approval. No perspective storyboard or video generation authorised yet.

## Coordinate contract

- Source image: `apartment-106-plan-untouched-crop.png`, rendered directly from the official PDF at 200 dpi.
- Canvas: 2934 x 1800 pixels.
- No wall, room, door, fixture, furniture or label in the official plan has been redrawn or moved.
- Blue line is physical camera movement. Camera look direction is recorded explicitly in the pose table rather than rendered as arrows, avoiding overlay ambiguity.
- The review overlay is the raster file `CLIP_B_EXACT_PLAN_ROUTE_V2.png`. The rejected SVG renderer output must not be used downstream.

## Camera poses

| Pose | Pixel position | Physical position | Facing | Required plan evidence |
| --- | --- | --- | --- | --- |
| B0 | 1850,1050 | clear circulation floor at the Dining/Living orientation hub, west of the Bedroom 1 door swing | northwest | Living remains southeast; Bedroom 1/private block remains north |
| B1 | 1710,950 | east of and north of the single island | west | clear aisle lies between northern kitchen/private wall and island |
| B2 | 1160,950 | west of the island, still on the central axis | west | island is now behind/east; MPR remains a solid room south |
| B3 | 625,950 | directly south of the main Bath doorway | north | Laundry remains separate to the west; Bath doorway is the fixed northern target |
| B4 | 625,800 | within the main Bath doorway/threshold | north/northeast | camera crosses the existing jambs; circulation timber ends at threshold |
| B5 | 700,680 | fully inside the main Bath | north/northeast | Bath fixtures remain within the plan's fixed room boundary |

## Collision check

- Route does not cross Living furniture.
- Route does not cross the Dining table.
- Route remains north of the single island and does not cross its cabinetry or stools.
- Route does not enter MPR, Bedroom 2, Bedroom 1, Laundry, WIR or Ensuite.
- The only crossed room threshold is the official main Bath doorway.

## Next gate

Perspective frames must be generated one at a time from B0-B5. Every frame must include the corresponding plan crop and camera marker as a geometry reference. Any generated frame that invents a front-facing end door, moves the Bath threshold, exposes Laundry as a corridor, removes the MPR volume or duplicates the island is rejected before video generation.
