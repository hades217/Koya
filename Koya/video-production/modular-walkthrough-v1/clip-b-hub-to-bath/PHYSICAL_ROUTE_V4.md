# Clip B V4 — Corrected Bath-Side Turn

Status: REJECTED. The four generated nodes do not provide reliable Apartment 106 geometry and must not be used again.

## What V3 got wrong

V3 incorrectly sent the camera to the blank west end of the kitchen passage and then asked it to turn right toward a front-facing pair of wet-area doors. The official Apartment 106 plan does not support that move. The resulting B2 video turned toward a wall and then generated the doors behind it. Both the V3 door node and the B2 video are rejected.

## Intended floor-plan route (not successfully represented by V4)

- The camera travels west through the kitchen passage.
- Bath and Laundry are on the camera-right/north side before the blank west end.
- Travelling west, the main Bath is the nearer/eastern doorway; Laundry is the immediately farther-west doorway.
- The camera arcs right into the main Bath before reaching the blank end wall.
- It never turns at the blank wall and never faces a symmetric pair of doors straight ahead.

## Corrected storyboard nodes

1. `B4-00-PASSAGE-APPROACH-W`: camera faces west; the two right-wall door positions are already physically present.
2. `B4-01-BATH-APPROACH-PIVOT`: camera moves about 0.7 metres west and yaws right about 35 degrees; the nearer Bath doorway is open and visible on the right; the farther Laundry doorway remains beyond it.
3. `B4-02-BATH-THRESHOLD`: camera moves about 0.5 metres forward-right and continues the same right yaw; Bath is ahead through fixed jambs; Laundry remains outside left/back; kitchen cabinetry remains outside right/back.
4. `B4-03-BATH-INSIDE-N`: camera crosses northward onto tile and stops inside the compact Bath.

## Superseded micro-piece proposal

- The three-piece strategy is rejected. Its first five-second task produced only a small approach move and wasted the available duration.
- Piece 2 and Piece 3 were never submitted.

## Cancelled generation plan

- Do not use one five-second Seedance task from Node 1 to Node 4; both endpoint concepts are rejected as spatial inputs.
- Nodes 2 and 3 are mandatory route checkpoints described in the timed prompt, not separate paid videos.
- No replacement task is authorised.
- Exact timing: `PROMPT_V4_SINGLE_5S.md`.
