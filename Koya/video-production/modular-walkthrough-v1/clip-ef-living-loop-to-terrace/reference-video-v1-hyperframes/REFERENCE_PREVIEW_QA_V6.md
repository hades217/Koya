# Apartment 106 EF deterministic reference preview V6

Status: `reference_mp4_ready / frame-level QA passed / user review required / no Seedance task submitted`

## Locked scope

- Duration: 8.00 seconds.
- Format: 1920 x 1080, 16:9, first-person camera.
- Route: MPR glance -> Dining and one island -> kitchen -> TV wall -> sofa and wrap glazing -> east terrace opening -> physical track crossing -> terrace look-back.
- One WebGL scene, one camera and one seekable GSAP timeline. No image sequence, cut, dissolve or crossfade is used.

## Automated gate

- HyperFrames CLI: 0.8.17, confirmed current.
- `check`: 0 lint errors, 0 runtime errors, 0 layout issues and 0 motion findings across 18 explicit samples.
- Dense checkpoint set: 19 frames from 0.00 to 7.95 seconds under `qa-v6/dense-checkpoints/`.
- Endpoint turn set: seven frames from 7.15 to 7.95 seconds under `qa-v6/endpoint-turn-v4/`.

## Rendered reference MP4

- File: `renders/apartment-106-living-panorama-to-terrace-reference-v6.mp4`
- Codec: H.264.
- Dimensions: 1920 x 1080.
- Frame rate: 30 fps.
- Duration: 8.000 seconds.
- Size: 6,390,687 bytes.
- SHA-256: `ae03d71427b6ac15a174034d404f6b677abf5e13ea34d5cea7c92e209a495781`.
- Audio: none; this is a topology and camera-motion reference.

## MP4 visual QA

- All 240 encoded frames were extracted and reviewed in ten consecutive 24-frame sheets under `qa-v6/rendered-all-contacts/`.
- The full clip was also sampled at 8 fps; the threshold and endpoint interval from 5.75 to 8.00 seconds was sampled at 12 fps.
- No cut, dissolve, static-image zoom, room substitution, wall penetration, glass penetration or dropped black frame was found.
- The same east opening expands with forward parallax, the camera crosses the black track, arrives on terrace tile, turns across the open terrace side and looks back through the same opening.

## Visual correction made during QA

The first endpoint interpolation passed the look target too close to the camera and produced a sudden downward floor view around 7.55 seconds. That path was rejected. The accepted V4 endpoint turn keeps the target at a stable horizontal distance, first reveals the open wraparound terrace side, then turns back through the same doorway to Living/Dining/Kitchen.

## Current gate

The rendered local MP4 is now the user-review surface. Acceptance of this MP4 still does not authorise any paid Seedance submission; that requires a separate preflight package, current pricing evidence and explicit one-task approval.
