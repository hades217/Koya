# Route and handoff ledger v3

Status: active continuity source of truth  
Continuity architecture: sequential forward extension  
Total runtime: 30.00 seconds

## Global camera state

| Property | Locked value |
| --- | --- |
| Screen direction | forward toward destination; no orbit or reversal |
| Horizon | level after aerial pitch-down; roll under 2 degrees |
| Human-height camera | approximately 1.6m |
| Lens feel | 28–32mm exterior/common areas; no abrupt lens swap; interior may ease toward 32–35mm without a cut |
| Motion | controlled forward glide with natural operator inertia; no still-image zoom |
| Light | neutral late-morning daylight; sun direction remains camera-right outside |
| Lead | `CHAR-RESIDENT-004 v2`, viewed mainly from rear/three-quarter rear |

## Leg A — geographic descent to resident acquisition

- Start: high Brisbane/Toowong view with the target axis already established.
- End: eye-level official Koya facade, entrance centred, forward creep still active;
  resident has just entered the lower third and begun the first step toward the
  entrance.
- Last 12 frames must preserve visible forward parallax. They may not settle into
  a frozen facade.
- Rejection: unreadable or substituted building, wrong storey count, tree wipe,
  map-card cut, static push-in, dusk colour, resident appearing inside the building.

## A→B handoff

Leg B must inherit the actual Leg A MOV as `@视频1`. The first second of Leg B
continues the same forward velocity and the resident's same gait phase. The
facade scale at Leg B frame 1 must match Leg A's final frame.

Reject on: frozen restart, size pop, changed entrance, changed ponytail/wardrobe,
camera height jump, new sunlight direction.

## Leg B — facade to lift axis

- Start: inherited A→B state.
- Route: follow resident along the same entrance axis; right hand opens the same
  glass door; camera physically crosses after her; move through a compact,
  daylight foyer to the single lift.
- End: camera approximately 1m from the closed lift doors; resident is right of
  centre with right hand near the call button; gentle forward drift continues.
- Rejection: logo insert, exterior-to-foyer replacement, doorframe covering a
  spatial swap, dark/night lobby, double-lift hall, bottle changing hand.

## B→C handoff

Leg C inherits the actual Leg B MOV. The lift doors, call button, floor joints,
resident stance and camera axis must match. The doors begin opening only after
the inherited forward movement is visibly continued.

## Leg C — one lift cabin, compressed travel

- Start: inherited lift-call state.
- 0.0–1.1s: same lift opens; resident crosses the metal threshold; camera follows
  and shows foyer floor, threshold and cabin floor together.
- 1.1–2.0s: camera is inside the same cabin facing the same doors; doors close.
- 2.0–2.8s: brief upward body-weight cue and lift sound; no location change.
- 2.8–4.0s: the same doors reopen; resident begins crossing out while both cabin
  edges and destination corridor remain visible.
- End: camera is still partly inside the cabin and moving forward across the
  destination threshold.
- Rejection: camera teleports to corridor, new cabin, new door colour, reverse
  direction, black-frame substitution, more than one resident.

## C→D handoff

Leg D inherits the actual Leg C MOV. Its opening completes the already-started
threshold crossing. Cabin side walls remain in the first frames and move behind
camera naturally.

## Leg D — corridor to compact Apartment 106 hall

- Start: inherited partial lift exit.
- Route: finish lift exit; traverse a short residential corridor; resident opens
  the apartment door with right hand; camera sees corridor carpet, metal sill and
  interior timber floor together; enter the compact hall.
- End: inside the Apartment 106 hall, kitchen island edge just becoming visible;
  forward movement continues.
- Rejection: hotel-scale corridor, apartment-door cut, giant foyer/MPR, direct
  jump to living room, wrong hand/bottle, invented room adjacency.

## D→E handoff

Leg E inherits the actual Leg D MOV. Hall width, door positions, timber floor,
island edge, resident scale and daylight vector must be identical for the first
frames.

## Leg E — living-zone reveal

- Start: inherited compact hall state.
- Route: continue past the kitchen edge into the apartment-scale kitchen,
  six-seat dining area and living zone; resident becomes secondary; camera slows
  toward terrace glazing.
- End: stable but still alive forward drift showing a credible kitchen–dining–
  living–terrace relationship.
- Rejection: penthouse-scale hall, giant island/banquet table, direct room cut,
  night view, exaggerated skyline, invented MPR tour.

## Seam QA for every join

Create two review strips:

1. last 0.5s of prior leg + first 0.5s of next leg at 0.125s intervals;
2. the same one-second interval played in reverse.

Pass only if geometry, optical flow, subject gait, exposure, colour and sound
remain continuous. A crossfade cannot convert a failed join into a pass.

