# QA report — wet area to Bedroom 2 threshold

Status: `PASS_INTERNAL_LITERAL_CONTINUITY_AWAITING_USER_REVIEW`

## Reviewed

- Full five-second output at normal speed.
- Entire output sampled at 0.125-second intervals.
- Critical turn sampled at 0.0625-second intervals from 0.75 to 3.75 seconds.
- Exact first-frame and requested last-frame comparisons.
- Local no-visual-transition join sampled at 0.0625-second intervals around the 26-second boundary.
- Scene-change detection above 0.18 and audio silence detection below -45 dB for 0.4 seconds.

## Findings

- Generated first frame matches the actual preserved 26-second endpoint.
- The resident and camera make one continuous short physical movement from the wet-area position to Bedroom 2.
- Laundry remains readable at the near edge while the camera yaws; the Bedroom 2 door progressively appears through parallax.
- No hard cut, dissolve, crossfade, semi-transparent overlap, wall wipe, black frame or alternate camera angle was detected.
- Same resident, sage top, black leggings, ponytail and left-hand stainless bottle remain stable.
- Her empty right hand reaches the single Bedroom 2 handle; the clip stops at the threshold and does not enter.
- Neutral daylight and warm material palette remain consistent.
- No scene score above 0.18 and no audio silence interval meeting the test threshold were logged.

## Join

- Review master: `outputs/koya-continuous-review-through-bedroom2-door-31s.mp4`
- Duration: 31.000 seconds
- Video: H.264, 1280x720, 24fps
- Audio: AAC stereo
- Visual join: duplicate first video frame removed; no visual crossfade.
- Audio join: 0.08-second audio-only crossfade.
- SHA-256: `009c9f791d705cefb9da2f98474a2fffd4f832e54a7bb348c561307282fdef9e`

## Evidence

- `qa/contact-0.125s.jpg`
- `qa/transition-0.75-3.75-0.0625s.jpg`
- `qa/first-match.jpg`
- `qa/last-match.jpg`
- `qa/join-25.35-27.10-0.0625s.jpg`
- `qa/scene-detect.log`
- `qa/silence.log`
- `qa/ffprobe.json`

## Gate

Internal continuity QA passes, but this remains a user-review master. Do not submit the next paid Bedroom 2 entry segment until the user reviews and accepts this 31-second version.
