# Segment C Retry 1 literal-continuity QA

Disposition: `PASS_LITERAL_APARTMENT_ENTRY_THRESHOLD`

## Evidence reviewed

- Full 5.056-second playback with native audio
- 0.125-second contact sheet: `qa/segment-c-retry1-contact-sheet-0.125s.jpg`
- 0.5-second door-geometry sheet: `qa/segment-c-retry1-door-geometry-0.5s.jpg`
- B-to-C join sheet: `qa/segment-b-c-retry1-join-contact-sheet-0.125s.jpg`
- First and last frame comparisons against the locked references

## Continuity findings

- The shot begins on the actual passed Segment B final frame at the one closed Apartment 106 door.
- The single timber door rotates inward toward the left around the same left-side hinges.
- The black lever and lock move with the same door panel; they do not remain on or reappear on the right wall.
- No second entrance door or duplicated lock appears.
- The woman crosses the carpet-metal-timber threshold first and the camera follows across the same threshold.
- By the final section, corridor carpet and metal threshold are behind the camera and no longer visible.
- The right wall remains plain and continuous.
- The same resident, wardrobe, ponytail and left-hand stainless bottle remain stable.
- The camera continues into a compact entry hall; no oversized foyer or entrance MPR is introduced.

## Technical findings

- Video: H.264, 1280x720, 24 fps
- Audio: AAC stereo, 32 kHz; mean -28.4 dB, peak -3.7 dB
- Duration: 5.056 seconds
- Required first-frame normalized RMSE: 0.0099609
- Required last-frame normalized RMSE: 0.0434149
- A+B+C combined preview: 21.047 seconds, H.264 + AAC, 1280x720, 24 fps
- The B-to-C visual join removes one duplicate frame and uses no visual crossfade or hidden wipe.

## Remaining scope

The passed continuous route now reaches the compact Apartment 106 entry hall. The next paid segment must start from this actual generated last frame and continue into the kitchen/dining zone without changing the apartment geometry or character.
