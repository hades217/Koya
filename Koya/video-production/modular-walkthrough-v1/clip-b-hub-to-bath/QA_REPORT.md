# Clip B — Internal QA Report

Status: `USER_REJECTED_VISUAL_REALISM`.

## Technical

- H.264 video, 1280x720, 24 fps, 5.056 seconds.
- AAC audio, 32 kHz, stereo.
- Mean audio level: -35.2 dB; peak: -8.8 dB.
- No black frames detected.
- No scene-change score above 0.25 detected.

## Continuity inspection

- 0.0–1.25s: begins on the locked north-facing Living/Dining hub and immediately moves northwest; the Dining table and single island retain solid parallax.
- 1.25–2.75s: the camera clears the island and physically enters a compact hall without crossing the table, island, cabinetry or wall.
- 2.75–3.50s: the hall remains continuous while the camera turns toward the two adjacent wet-area openings; no cut, dissolve or hidden wipe appears in the 0.125-second review.
- 3.50–4.25s: the separate Laundry remains briefly visible beside the main Bath as the camera physically crosses the Bath doorway.
- 4.25–5.056s: the camera settles inside the compact main Bath with vanity, WC and bathing area readable; scale stays residential rather than showroom-sized.
- Bright daytime, warm neutral materials and first-person eye height remain stable.
- No person, hand, operator reflection, duplicate island, extra room, merged wet room or wall pass-through appears.

## Review evidence

- `qa/contact-sheet-0.25s.jpg`
- `qa/threshold-2.75-4.5s-0.125s.jpg`
- `qa/frames/q-001.jpg` through `qa/frames/q-021.jpg`

## Gate

The user rejected the clip because the latter interior frames read as architectural design renders rather than a real photographed apartment. Spatial continuity passed, but visual realism is a separate mandatory gate and failed. The clip is retained under `outputs/rejected/` as a diagnostic sample and must not be used in the final edit or as a later-branch style reference.

## Visual-realism failure

- Surfaces are uniformly clean and procedurally smooth, without credible material variation.
- Illumination is too even and showroom-controlled, with weak natural exposure falloff.
- Bathroom fittings and styling read as a developer CGI still rather than a camera observing a built home.
- The supplied CGI-like last frame constrained Seedance to preserve that synthetic finish; prompt wording alone could not override the endpoint image.
