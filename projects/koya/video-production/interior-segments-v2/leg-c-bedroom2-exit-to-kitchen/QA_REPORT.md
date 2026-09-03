# QA report — Bedroom 2 same-door exit to kitchen axis

Status: `PASS_INTERNAL_LITERAL_CONTINUITY_AWAITING_USER_REVIEW`

## Reviewed

- Full 5.056-second output at normal speed with audio.
- Entire output sampled at 0.125-second intervals.
- Entire output densely sampled at 0.0625-second intervals.
- Generated first and final frames.
- Local no-visual-transition join sampled at 0.0625-second intervals around the 36-second boundary.
- Scene-change detection above 0.18 and audio silence detection below -45 dB for 0.4 seconds.

## Findings

- The output begins on the exact accepted 36-second master endpoint inside Bedroom 2.
- The resident turns visibly inside the same room; she does not disappear or reappear outside.
- The camera backs continuously through the same timber door while the resident walks toward it and crosses the same threshold.
- Bed, robe, window, door leaf and door frame remain spatially stable during the return movement.
- The bottle remains in the resident's anatomical left hand throughout the turn, frontal approach, side profile and final rear view.
- After crossing the threshold, the camera settles behind the resident toward the east kitchen axis.
- The just-exited Bedroom 2 remains visible behind-left through the same open door in the final frame.
- Only the start of the compact linear kitchen and one modest island appear; the MPR is not entered or substituted into this segment.
- No cut, dissolve, crossfade, opacity blend, ghosting, black frame, body wipe, door wipe, wall passage or architecture replacement was found.
- No scene score above 0.18 and no qualifying audio silence interval were logged.

## Join

- Review master: `outputs/koya-continuous-review-bedroom2-exit-kitchen-41s.mp4`
- Duration: 41.000 seconds
- Video: H.264, 1280x720, 24fps, 983 frames
- Audio: AAC stereo
- Visual join: duplicate first frame removed; no visual crossfade.
- Audio join: 0.08-second audio-only crossfade.
- SHA-256: `c2e2d139198b7210c579efa1a4ec8136aaabd98e41e1438b6181cf485c7d4459`

## Evidence

- `qa/contact-0.125s.jpg`
- `qa/dense-0.0625s.jpg`
- `qa/generated-first.png`
- `qa/generated-last.png`
- `qa/join-35.35-37.25-0.0625s.jpg`
- `qa/scene-detect.log`
- `qa/silence.log`
- `qa/ffprobe.json`

## Gate

Internal literal-continuity QA passes. The next paid segment is the kitchen approach to the compact MPR threshold. Do not submit it until the user reviews and accepts the 41-second master.
