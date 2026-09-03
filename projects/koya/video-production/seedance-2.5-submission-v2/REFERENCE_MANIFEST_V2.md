# Koya Seedance 2.5 single-generation reference manifest v2

Status: `@视频1`, prompt, reference order, endpoint, parameters and live cost estimate verified on 2026-08-17. Final cost-specific user approval pending; do not submit until received.

Mode: full-modal reference generation (`omni_reference_task_type=reference`), one provider task, 30 seconds, 16:9, 720p, native audio.

## Upload order

1. `@视频1` — `stage-01-camera-blockout-30s-v2.mp4` — ready: H.264, 1280×720, 24fps, 30.000s, 9,222,931 bytes, silent. Controls only camera route, speed and threshold timing.
2. `@图片1` — `production-assets/assets/official/exterior/koya-building-hero.jpg` — official facade geometry truth.
3. `@图片2` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-01-HIGH-AERIAL-DAY.png` — 0-second high aerial.
4. `@图片3` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-02-MID-AERIAL-DAY.png` — mid aerial.
5. `@图片4` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-03-LOW-AERIAL-DAY.png` — low aerial.
6. `@图片5` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-04-STREET-FRONT-DAY.png` — eye-level facade.
7. `@图片6` — `production-assets/assets/generated/character/sports-female/master-panels-v2/back-full.png` — character identity and wardrobe.
8. `@图片7` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-05-RESIDENT-APPROACH-DAY.png` — approach scale and direction.
9. `@图片8` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-06-ENTRY-THRESHOLD-DAY.png` — exterior/foyer threshold.
10. `@图片9` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-07-FOYER-LIFT-CALL-DAY.png` — foyer/lift axis.
11. `@图片10` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-08-LIFT-ENTRY-DAY.png` — lift entry threshold.
12. `@图片11` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-09-LIFT-ASCENT-COVER-DAY.png` — same cabin and closed doors.
13. `@图片12` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-10-LIFT-EXIT-DAY.png` — same cabin reopening and exit.
14. `@图片13` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-11-APARTMENT-DOOR-DAY.png` — short corridor and apartment door.
15. `@图片14` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-12-APARTMENT-THRESHOLD-DAY.png` — apartment threshold.
16. `@图片15` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-13-APARTMENT-HALL-DAY.png` — corrected compact hall scale.
17. `@图片16` — `storyboards/stage-01-opening-redesign-v2/frames/SB-V2-14-LIVING-REVEAL-DAY.png` — corrected apartment-scale living reveal.
18. `@图片17` — `production-assets/assets/official/floorplans/apartment-106.png` — official interior adjacency and scale authority.

## Submission parameters verified live on 2026-08-17

- Endpoint: `ep-20260812221158-hb576` (`Running`).
- Model: `doubao-seedance-2-5-260628`.
- `omni_reference_task_type=reference`.
- `duration=30`.
- `ratio=16:9`.
- `resolution=720p`.
- `generate_audio=true`.
- `return_last_frame=true`.
- `output_format=mp4`.
- `watermark=false`.
- Unsupported and therefore omitted: `seed`, `frames`, `camera_fixed`, `draft`.

## Remaining paid-generation gates

1. Obtain explicit approval for the current estimated charge of approximately CNY 54.432.
2. Submit exactly once; save the returned `task_id` and poll that task instead of retrying.

## Live price estimate — 2026-08-17

- Current account price for Seedance 2.5 with video input at 480p/720p: CNY 0.042 per 1,000 tokens.
- Official estimate formula: `(input video seconds + output video seconds) × output width × output height × fps ÷ 1024`.
- This request: `(30 + 30) × 1280 × 720 × 24 ÷ 1024 = 1,296,000 estimated tokens`.
- Estimated charge: `1,296,000 ÷ 1,000 × 0.042 = CNY 54.432`.
- Actual charge is determined by the successful task's returned `usage.completion_tokens`; failed generation is not charged according to the official pricing document.
