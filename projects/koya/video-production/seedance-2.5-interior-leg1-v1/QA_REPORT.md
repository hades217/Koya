# Interior Leg 1 — QA Report

Overall result: `REJECTED_NOT_VISUALLY_SEAMLESS`

Neither candidate is approved as the next master segment. Both are retained only as review evidence.

## Candidate A — cgt-20260818145357-8t8g4

Technical checks:

- Duration: 30.000 seconds
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo
- File size: 34,620,602 bytes
- Start continuity: passed visually; generated first frame matches the preserved video's final view, resident position, kitchen axis, material palette and daylight.
- Character continuity: broadly stable; same woman, outfit and bottle presentation.
- Route coverage: wet rooms, Bedroom 2, compact MPR and kitchen are present.

Reject reason:

- At approximately 23.375 seconds, the picture jumps directly from the MPR to the kitchen. Dense 0.125-second inspection confirms a hard spatial cut rather than a physical same-door exit. This violates the literal one-camera requirement.

Evidence:

- `outputs/task-cgt-20260818145357-8t8g4/qa/start-continuity-side-by-side.jpg`
- `outputs/task-cgt-20260818145357-8t8g4/qa/contact-sheet-1s.jpg`
- `outputs/task-cgt-20260818145357-8t8g4/qa/transition-22.5-24.5-0.125s.jpg`
- `outputs/task-cgt-20260818145357-8t8g4/qa/scene-metadata.txt`

## Candidate B — cgt-20260818145533-jlz2g

Technical checks:

- Duration: 30.000 seconds
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo
- File size: 34,379,056 bytes
- Start continuity: visually consistent with the preserved endpoint.
- Character/daylight/material continuity: broadly stable.
- Candidate B handles the MPR-to-kitchen transition more smoothly than Candidate A.

Reject reason:

- At approximately 28.667 seconds, the picture hard-cuts from the kitchen/dining axis into a different hallway view. Automated scene score is 0.405565 and dense 0.125-second frames confirm the discontinuity. It does not physically continue toward the Bedroom 1 southwest threshold.

Evidence:

- `outputs/task-cgt-20260818145533-jlz2g/qa/contact-sheet-1s.jpg`
- `outputs/task-cgt-20260818145533-jlz2g/qa/transition-21-25-0.125s.jpg`
- `outputs/task-cgt-20260818145533-jlz2g/qa/transition-27.5-30-0.125s.jpg`
- `outputs/task-cgt-20260818145533-jlz2g/qa/scene-metadata.txt`

## Required strategy before any future paid retry

Do not ask one 30-second generation to perform both wet-area visits, two same-door room loops and the final kitchen route. Seedance complied with appearance and room content but inserted a late spatial edit to satisfy the overloaded route.

The next paid attempt should be a shorter literal continuation with one topology problem only: preserve the exact start, visit bath/laundry and Bedroom 2, visibly leave through the same Bedroom 2 door, then end on a clean kitchen-axis frame. MPR and Bedroom 1 must move to the following continuation using that returned last frame. This reduces topology compression without reverting to arbitrary five-second montage clips.
