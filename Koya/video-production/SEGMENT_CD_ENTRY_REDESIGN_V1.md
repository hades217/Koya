# Apartment 106 entry redesign V1

Status: `STILL_FRAME_REVIEW_ONLY`  
Paid Seedance submission: `NOT_AUTHORIZED / NOT_SUBMITTED`

## Why the previous route failed

The rejected Segment D asked the island to move from the camera-right side of the galley to the camera-left side after an occlusion. Type 106 does not support that topology. The model hid the resident behind the island and morphed the room instead of showing a physical clearance and turn.

## Correct floor-plan route

1. Start at the actual Segment B final frame outside Apartment 106.
2. The single entrance door opens inward on its left hinge; resident and camera cross the same threshold.
3. Continue through the compact entry/hall without enlarging it.
4. Enter the galley axis: continuous kitchen cabinetry remains camera-left; the only stone island remains camera-right.
5. Resident and camera walk straight through the real clear passage between cabinetry and island.
6. Both visibly clear the far eastern end of the island.
7. Only then make a gentle 90-degree right turn toward dining.
8. After the turn: the original island trails on camera-right, dining is centered ahead, living is ahead-left and terrace glazing is farther ahead.

## Candidate cinematic anchors

- Mid-galley anchor: `production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-galley-mid-cinematic-v1.png`
- Post-turn anchor: `production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-dining-turn-cinematic-v2.png`
- The earlier `apartment-106-dining-turn-cinematic-v1.png` is rejected because it placed the living zone on the wrong side of the south-facing post-turn view.
- Emotional final endpoint: `production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-welcome-living-cinematic-v1.png`
- Both are Concept Design / Artist Impression candidates. Type 106 plan controls geometry; official Koya interior renders control materials only.

## Recommended replacement generation

Replace both the old Segment C and rejected Segment D with one uninterrupted approximately 10-second Seedance 2.5 take. A single take avoids another artificial seam inside the apartment.

Proposed timing:

- 0.0-2.0s: open the one entrance door and physically cross the threshold.
- 2.0-4.5s: continue through the compact hall and establish the galley axis.
- 4.5-7.5s: walk straight between left cabinetry and right island.
- 7.5-8.5s: visibly clear the island end and turn right into the dining route.
- 8.5-10.0s: resident steps slightly left, slows, turns back toward the camera and gives a restrained open-right-hand invitation; camera settles on the living room and terrace destination.

Required provider references:

1. `first_frame`: actual Segment B final frame at Apartment 106 door.
2. `reference_image`: Type 106 floor plan for topology only.
3. `reference_image`: official Koya two-bedroom living/kitchen image for material language only.
4. `reference_image`: locked `CHAR-RESIDENT-004` character sheet.
5. `reference_image`: cinematic mid-galley anchor.
6. `reference_image`: cinematic post-turn anchor, used only to enforce the physical turn.
7. `last_frame`: emotional welcome/living endpoint.

No paid task may be created until the user approves both candidate stills, the complete prompt, exact parameters and current cost estimate.

## Cinematic treatment

- Live-action camera language: 35-40mm lens, human 1.55-1.60m height, restrained Steadicam movement.
- Directional neutral late-morning window light, controlled highlight roll-off and warm interior bounce.
- Real timber/stone microtexture, subtle lens falloff and fine grain; no uniform showroom lighting.
- Final whole-film grade may add restrained contrast, highlight compression, subtle halation and fine grain, but post-processing must never hide a spatial discontinuity.
- The performance arc is arrival rather than NPC locomotion: brisk return-home walk, natural deceleration after clearing the island, a small breath, brief eye contact, subtle smile and understated open-palm invitation.
