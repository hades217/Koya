# Clip A — Internal QA Report

Status: `REJECTED_BY_LATER_FLOORPLAN_AUDIT`. The earlier visual approval is retained as history, but this clip must not be used as Apartment 106 spatial truth or as a new generation endpoint.

## Technical

- H.264 video, 1280x720, 24 fps, 10.048 seconds.
- AAC audio, 32 kHz, stereo.
- Mean audio level: -40.6 dB; peak: -23.1 dB.
- No black frames detected.
- No scene-change score above 0.25 detected.

## Continuity inspection

- 0.0–3.0s: compact entry hall remains readable; camera-right west wall stays solid; wet-area functions remain on camera-left.
- 3.0–5.5s: camera physically rounds the corner into the kitchen; dense 0.25-second review shows no cut, dissolve or wall pass-through.
- 5.5–8.0s: exactly one island remains present; camera advances beside it without clipping through the stone.
- 8.0–10.0s: Dining appears before Living; camera reaches and settles on the locked east-facing Living hub.
- Daylight and warm material palette remain stable.
- No person, hand, operator reflection or accidental private-room visit appears.

## Review evidence

- `qa/contact-sheet-0.5s.jpg`
- `qa/turn-2.5-5.5s-0.25s.jpg`
- `qa/hub-6.5-10s-0.25s.jpg`

## Gate

The user initially approved the clip visually with `不错，继续下一个`. A later detailed walk-through against the official Apartment 106 plan invalidated the spatial gate:

- the Laundry opening becomes visible from the long entry hall before the plan-supported wet-area threshold is reached;
- after the hall-to-kitchen turn, the solid MPR volume that must occupy the near south/right side is not preserved;
- the island and open kitchen are revealed too early and at an enlarged promotional-render scale;
- therefore the final Living hub cannot serve as a geometrically reliable start point for Bath, Bedroom or MPR branches.

No new paid generation may use this clip or its last frame until a floor-plan-derived spatial blockout replaces it.
