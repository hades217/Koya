# Clip B V2 — Internal QA Report

Status: `USER_REJECTED_ENTIRE_CLIP_SPATIAL_AND_CINEMATOGRAPHY_FAILURE`.

## Technical

- H.264 video, 1280x720, 24 fps, 5.056 seconds.
- AAC audio, 32 kHz, stereo.
- Mean audio level: -37.7 dB; peak: -16.8 dB.
- No black frames detected.
- No scene-change score above 0.25 detected.

## Spatial continuity — corrected finding

- No section is approved for reuse. Although some individual frames are topologically recognizable, they belong to a visually and spatially failed clip.
- Approximately 1.9–2.3s is invalid: the Living/kitchen boundary is progressively replaced by a fully formed corridor. The opening width, camera-right structure and depth relationship reconfigure instead of the lens physically clearing one fixed threshold.
- This is a spatial morph rather than a single-frame hard cut. The automated scene-change score missed it because the replacement is distributed across successive frames.
- The turn from hall to main Bath is continuous in the 0.125-second threshold inspection.
- Laundry remains a separate adjacent space; the camera physically crosses the main-Bath doorway.
- Despite the later doorway movement being usable, one spatial replacement anywhere in the branch fails the entire continuity gate.

## Cinematography and realism failure — corrected finding

- Lighting is broadly filled from front to back, so walls, kitchen, corridor and bathroom carry nearly the same exposure instead of a motivated key-light direction.
- The corridor has insufficient shadow separation and no convincing transition from bright public space into a more enclosed wet-area zone.
- Highlights, midtones and practical lights do not create cinematic depth; surfaces remain evenly illuminated and visually flat.
- Materials remain smooth and synthetic, especially walls, tile, stone and cabinetry; the result reads as a moving architectural render.
- Camera response lacks a believable exposure adjustment, depth cue and optical character while crossing between spaces.
- The rebuilt bathroom endpoint did not overcome the render-like first frame and model prior. The whole clip fails the requested real, cinematic look.

## Evidence

- `qa-v2/contact-sheet-0.25s.jpg`
- `qa-v2/threshold-2.75-4.5s-0.125s.jpg`
- `qa-v2/frames/q-001.jpg` through `qa-v2/frames/q-021.jpg`

## Gate

Clip B V2 is rejected in full and retained only as a failure sample under `outputs/rejected/`. Do not salvage its opening, corridor, bathroom or colour treatment. Do not proceed to Clip C.
