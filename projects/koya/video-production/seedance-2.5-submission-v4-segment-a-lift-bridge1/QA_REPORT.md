# QA report — Segment A lift-threshold bridge

Disposition: `PASS_LITERAL_LIFT_THRESHOLD`

## Technical result

- 1280×720, 16:9
- H.264, 24 fps
- AAC stereo, 32 kHz
- Duration: 5.056 seconds
- Native audio is present and non-silent (`max_volume -13.2 dB`)

## Literal continuity review

- Reviewed full clip and 20 frames sampled every 0.25 seconds.
- The resident remains the same athletic white woman with dark ponytail, sage long-sleeve top, black leggings, white/grey runners and stainless bottle in her left hand.
- The lift opens in the same foyer directly ahead of the resident.
- The resident visibly crosses the metal threshold first.
- The camera visibly advances across the same threshold; the door jambs move behind the camera and the foyer disappears.
- The camera and resident finish fully inside the same wood-lined cabin.
- The metal doors close only after both camera and resident are inside.
- No cut, dissolve, teleport, alternate angle or architecture replacement is visible in the 0.25-second contact sheet.

## Endpoint checks

- Generated first frame versus required first frame RMSE: `0.00820594` normalized.
- Generated final frame versus required final frame RMSE: `0.0123035` normalized.
- These differences are consistent with video compression; composition and camera positions match.

## Continuous preview

The approved 0–8 second Segment A motion was joined to a speed-adjusted 3-second version of this bridge. The duplicate shared frame was removed. Video is a same-frame join; only audio uses a short crossfade.

- Preview duration: 11.001 seconds
- Preview: `outputs/koya-segment-a-continuous-preview-11s.mp4`
- Join reviewed every 0.125 seconds from 7.25–9.75 seconds.
- No visible spatial jump or character discontinuity is present at the join.

This passes the entrance-to-inside-lift portion only. It does not yet prove lift travel, destination-door reopening, corridor traversal or Apartment 106 entry.
