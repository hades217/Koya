# QA report — Bedroom 2 entry

Status: `PASS_INTERNAL_LITERAL_CONTINUITY_AWAITING_USER_REVIEW`

## Reviewed

- Full 5.056-second output at normal speed.
- Entire output sampled at 0.125-second intervals.
- Door-threshold movement sampled at 0.0625-second intervals from 0.5 to 4.0 seconds.
- Generated first and final frames.
- Local no-visual-transition join sampled at 0.0625-second intervals around the 31-second boundary.
- Scene-change detection above 0.18 and audio silence detection below -45 dB for 0.4 seconds.

## Findings

- The shot begins on the actual preceding generated endpoint with the resident's right hand on the same timber door.
- The same door continues opening; no second panel, duplicate handle or alternate doorway appears.
- Resident and camera advance across the threshold; by the final frame corridor flooring and the threshold are no longer visible and the camera is inside Bedroom 2.
- The same open door remains on the near right edge, preserving the physical same-door return route.
- Fixed robe remains west/left and the bed east/centre-right at compact residential scale.
- No cut, dissolve, crossfade, semi-transparent overlap, black frame, wall wipe, body wipe or architecture replacement was found.
- Same resident, clothing, ponytail and left-hand bottle remain stable.
- No scene score above 0.18 and no qualifying audio silence interval were logged.

## Join

- Review master: `outputs/koya-continuous-review-inside-bedroom2-36s.mp4`
- Duration: 36.000 seconds
- Video: H.264, 1280x720, 24fps
- Audio: AAC stereo
- Visual join: duplicate first frame removed; no visual crossfade.
- Audio join: 0.08-second audio-only crossfade.
- SHA-256: `e3c5be2e2c948f5f06aa966bbd56b4f88cc99400e8b64c528545bc2c743d1b59`

## Evidence

- `qa/contact-0.125s.jpg`
- `qa/threshold-0.5-4.0-0.0625s.jpg`
- `qa/generated-first.png`
- `qa/generated-last.png`
- `qa/join-30.35-32.10-0.0625s.jpg`
- `qa/scene-detect.log`
- `qa/silence.log`
- `qa/ffprobe.json`

## Gate

Internal literal-continuity QA passes. Do not submit the next paid same-door exit segment until the user reviews and accepts the 36-second master.
