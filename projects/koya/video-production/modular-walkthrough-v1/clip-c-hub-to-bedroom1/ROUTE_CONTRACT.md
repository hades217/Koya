# Apartment 106 Clip C - Living Hub to Bedroom 1

Status: route_locked_2d; no Seedance task submitted.

## Source of truth

- Official floor plan: `../../../production-assets/references/floorplans/Koya marketing plan Apartment 106.pdf`
- Untouched plan crop: `../route-audit-v2/apartment-106-plan-untouched-crop.png`
- Existing accepted branch origin: `../clip-a-entry-to-hub/outputs/accepted/clip-a-entry-to-hub-v1.mp4`
- Official master-bedroom render: `../../../production-assets/assets/official/interiors/koya-2br-master-bedroom.jpg` (appearance language only; not confirmed Apartment 106 geometry and not eligible as an independent whole-room Seedance input)

Plan orientation is north-up and east-right.

## Literal route

`C0 Living/Dining hub -> C1 outside Bedroom 1 doorway -> C2 cross the fixed doorway -> C3 fully inside Bedroom 1 at the bed foot`

Only one room threshold is crossed. The camera never enters the kitchen island, TV console, WIR, Ensuite or Terrace.

## Camera poses

| Pose | Plan pixel position | Physical position | Facing | Required evidence |
| --- | ---: | --- | --- | --- |
| C0 | 1850,1050 | clear Living-side circulation floor, west of the TV console | north-northeast | Bedroom 1 doorway remains left of the TV console; Terrace glazing remains east/right |
| C1 | 1885,975 | immediately south of the fixed Bedroom 1 doorway | north | both doorway jambs are visible and stable |
| C2 | 1890,885 | inside the doorway | north-northeast | the same jambs pass behind the camera by physical occlusion |
| C3 | 1890,815 | fully inside Bedroom 1, in the clear floor area south-west of the bed foot | north-northeast, then directly right toward the bed and Terrace glazing | fixed bed remains ahead without collision; Terrace glazing remains east/right; no leftward WIR inspection |

## Five-second motion contract

| Time | Action | Physical proof |
| --- | --- | --- |
| 0.00-0.70s | one smooth leftward reorientation from the accepted Living hub toward the already-visible Bedroom 1 door | no new doorway appears; TV console slides right by parallax |
| 0.70-2.00s | advance from C0 to C1 | the same two jambs enlarge continuously |
| 2.00-2.75s | cross C1-C2 almost straight through the same doorway | camera passes between the jambs; the reveal is not wasted while still in the doorway |
| 2.75-3.40s | once fully inside, perform one fast, large right turn while moving into the clear south-west floor area | the complete bed opens into view without collision, wall penetration or a leftward WIR detour |
| 3.40-4.30s | continue the same rightward pan beyond the bed reveal | the east/right wall and Terrace glazing open into view; the camera does not stop prematurely on the bed |
| 4.30-5.00s | hold the farther-right inspection composition | the client has time to inspect the right-side wall/glazing; motion does not snap back, drift sideways or invent another opening |

The route contains a short physical translation plus an in-room orientation sweep. The walking portion must remain at a normal residential pace rather than being stretched across the full five seconds.

## Immutable topology

- one Bedroom 1 doorway in the north wall of the Living/Kitchen axis;
- door swings inward into Bedroom 1 as drawn;
- bed head remains on the north wall and foot faces south;
- WIR remains west/left of Bedroom 1;
- Terrace glazing remains east/right of Bedroom 1;
- TV console remains south of the Bedroom 1 wall on the Living side;
- no extra door, corridor, island, window, room or fixture.

## Input policy for the later paid task

The first topology-sensitive Seedance attempt must use one locally rendered reference video from one complete scene shell. Independently generated Living, doorway or Bedroom whole-room images are forbidden as `reference_image` inputs. The official master-bedroom render may guide local material decisions only.
