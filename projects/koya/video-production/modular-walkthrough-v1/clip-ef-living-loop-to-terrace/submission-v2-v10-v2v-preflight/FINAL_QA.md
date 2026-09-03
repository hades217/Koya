# Clip EF V10 V2V final QA

Decision: **REJECTED AFTER USER REVIEW**

The earlier continuity-only pass did not constitute final sales-film acceptance. After reviewing the exact segment in context, the project owner rejected it. The provider package used one V10 3D `reference_video` and zero rendered storyboard `reference_image` inputs, so the generated result remained tied to the unacceptable 3D/reference design rather than the approved rendered visual direction.

## Technical result

- File: `outputs/cgt-20260829175719-2hwhz.mp4`.
- SHA-256: `dfecc7954781d23ee42bb0870d49d3c57efc00a9254d6796a0ef15a583d8a6ff`.
- Container: MP4.
- Video: HEVC, 1920x1080, 24 fps.
- Audio: AAC stereo, 32 kHz; mean volume -33.6 dB, max -11.8 dB.
- Actual probed duration: 7.712 seconds. This is 0.288 seconds shorter than the 8.000-second reference but retains a readable final hold.

## Continuity findings

- PASS: one continuous camera path; no hard cut, dissolve, crossfade or static-image push-in.
- PASS: opening MPR/Dining read, Kitchen/single-island sweep and TV/Living turn remain in one connected space.
- PASS: the same east glazed opening remains visible during approach.
- PASS: both jambs and the floor track expand through forward parallax before the camera crosses the threshold.
- PASS: camera physically moves from interior oak floor to Terrace tile without wall, glass or furniture penetration.
- PASS: Terrace furniture and planting retain stable relative positions during the turn.
- PASS: final frames look back through the same opening and retain the Living/Dining/Kitchen connection.
- PASS: no duplicated island, extra doorway, third glazed elevation, invented column, person or operator reflection detected.
- PASS: daylight and exposure remain stable.

Evidence:

- `qa/OUTPUT_WHOLE_8FPS.jpg`
- `qa/OUTPUT_THRESHOLD_12FPS.jpg`
- `qa/OUTPUT_TURN_LOOKBACK_12FPS.jpg`
- `qa/OUTPUT_ALL_FRAMES_01.jpg` through `qa/OUTPUT_ALL_FRAMES_06.jpg` (all encoded frames)
- `qa/SCENE_DETECTION.log`

## Aesthetic observation

The clip is materially more photographic than the deterministic reference and preserves the correct physical route. However, the result still reads as restrained architectural visualisation/CG rather than fully natural residential cinematography. Interior styling is sparse, the exterior outlook is generic/washed, and the material/detail richness does not fully reach the accepted V9/V10 photoreal still direction.

This aesthetic and reference-fidelity failure is blocking for the sales film. The clip is excluded even though its local continuity checks passed. Its retained file is failure evidence only, not an accepted source. No replacement task is authorised by this report.
