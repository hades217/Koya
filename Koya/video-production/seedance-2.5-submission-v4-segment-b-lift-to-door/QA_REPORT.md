# Segment B literal-continuity QA

Disposition: `PASS_LITERAL_LIFT_EXIT_TO_APARTMENT_DOOR`

## Evidence reviewed

- Full 5.056-second playback with native audio
- 0.25-second contact sheet: `qa/segment-b-contact-sheet-0.25s.jpg`
- 0.125-second threshold sheet: `qa/segment-b-threshold-contact-sheet-0.125s.jpg`
- A+B join sheet: `qa/segment-a-b-join-contact-sheet-0.125s.jpg`
- First and last frame comparisons against the locked references

## Continuity findings

- The shot begins in the same wood-lined elevator cabin with the doors closed.
- The same doors open directly onto the compact Level 1 corridor.
- The woman crosses the metal lift threshold first; the camera follows across the same threshold.
- The lift jambs and cabin walls move continuously behind the camera, providing visible spatial evidence that the camera left the cabin.
- There is no second elevator entry, cut, dissolve, teleport, wall crossing or architecture substitution.
- The same resident, wardrobe and dark ponytail remain stable.
- The stainless bottle remains in her left hand; her right hand reaches the apartment-door handle.
- The shot stops at the closed Apartment 106 door, ready for the next literal threshold crossing.

## Technical findings

- Video: H.264, 1280x720, 24 fps
- Audio: AAC stereo, 32 kHz; mean -30.9 dB, peak -8.3 dB
- Duration: 5.056 seconds
- Required first-frame normalized RMSE: 0.00934574
- Required last-frame normalized RMSE: 0.0557809
- A+B combined preview: 16.023 seconds, H.264 + AAC, 1280x720, 24 fps
- The A+B visual join removes one duplicate frame and uses no visual crossfade or hidden wipe.

## Remaining scope

This pass reaches Apartment 106 but does not open or enter it. The next paid segment must begin from this actual generated final frame and visibly open the door, cross the entry threshold and enter the apartment before any room movement.
