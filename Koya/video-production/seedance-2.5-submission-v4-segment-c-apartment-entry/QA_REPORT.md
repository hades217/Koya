# Segment C literal-continuity QA

Disposition: `REJECT_DUPLICATED_ENTRY_DOOR_GEOMETRY`

## Evidence reviewed

- Full 5.056-second playback with native audio
- 0.125-second contact sheet: `qa/segment-c-threshold-contact-sheet-0.125s.jpg`
- 0.5-second door-geometry sheet: `qa/segment-c-door-geometry-0.5s.jpg`
- First and last frame comparisons against locked references

## Failure

- The start frame has one closed door with the black lever and lock on its right edge and hinges on its left.
- During opening, the timber door panel rotates toward the left, but the black lock/lever geometry remains or reappears on a new timber surface at the right side.
- Later frames visibly contain the opened original door on the left and a second lock-bearing timber door surface on the right.
- This is an architecture morph and duplicated apartment entrance, not one physical door followed by one camera.
- The defect begins during the threshold action, so no sufficiently complete camera-crossing portion is safe to salvage.

## Items that remained stable

- Same resident identity, wardrobe and rear-follow camera direction.
- Bottle remained in the left hand.
- 16:9, 720p and native audio were correct.

## Root cause and correction

The rejected endpoint reference placed the open entrance door and lock hardware on the right, conflicting with the real start frame's left-hinge/right-handle door geometry. A corrected endpoint has been produced from the official Type 106 floor plan and Koya material references:

`../seedance-2.5-submission-v4-segment-c-retry1/references/02-inside-entry-single-door-left-end-16x9.png`

The corrected endpoint is fully inside the apartment, keeps only a narrow portion of the one open door at the far left, removes the threshold from the foreground and keeps the entire right wall free of entrance-door hardware.
