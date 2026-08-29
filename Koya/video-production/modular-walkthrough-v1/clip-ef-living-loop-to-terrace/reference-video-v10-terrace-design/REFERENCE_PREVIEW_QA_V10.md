# Apartment 106 EF deterministic reference V10

Status: `rendered_local_qa_passed`

## Fixed-scene contract

- One new Three.js scene built from the official Apartment 106 marketing plan and current route audit.
- One first-person camera, one seekable eight-second timeline, 1920 x 1080.
- No V8/V9 source, camera path, screenshot or derived video was reused.
- No still-image zoom, cut, crossfade, dissolve, teleport or room replacement.
- The original Terrace concept image `terrace-design-v1/apartment-106-terrace-lookback-concept-v1.png` is superseded and must not be supplied to Seedance: it incorrectly presents a broad solid TV wall with a centred wall-mounted television.
- The V2 concept `terrace-design-v1/apartment-106-terrace-lookback-concept-v2-tv-edge-glazing.png` is rejected because it invented a third glazed elevation behind the television.
- The V3 concept `terrace-design-v1/apartment-106-terrace-lookback-concept-v3-two-glazed-faces.png` is rejected because it left a broad TV backing wall and omitted the required MPR/Bedroom 1 doorway read.
- The V5 concept `terrace-design-v1/apartment-106-terrace-lookback-concept-v5-mpr-bedroom-door-tv-on-right-glazing.png` is rejected because the Bedroom 1 doorway, MPR corner and Living scale are wrong.
- `terrace-design-v1/apartment-106-terrace-lookback-concept-v9-wrap-terrace-candidate.png` is the current project-owner `TERRACE ACCEPTED_LOCAL` visual direction. It originated from the corrected deterministic shell and is human-review evidence only; it is not a provider input.
- `terrace-design-v1/apartment-106-terrace-lookback-concept-v10-wrap-terrace-alternate-candidate.png` is a same-topology alternate review candidate. It is not a provider input and has not replaced V9 as the accepted local endpoint.
- The drawing establishes grouped glazing but does not establish the operability of every individual panel; prompts must not invent panel operation.

## Route proof

- 0.00-0.75: compact MPR and four-seat Dining glance.
- 0.75-1.65: Kitchen with exactly one island.
- 1.65-2.55: internal TV wall and broad Living.
- 2.55-3.35: continuous sheet-down/sheet-right glazing and the single sheet-right open bay.
- 3.35-4.45: forward approach to the same bay.
- 4.45-5.35: physical crossing over the dark track between the same jambs.
- 5.35-6.25: Terrace arrival with compact dining, two lounge chairs, side table and grouped planting.
- 6.25-7.30: continuous Terrace turn.
- 7.30-8.00: hold the look-back through the same opening into Living, Dining and Kitchen.

## Verification

- `hyperframes check` passed across 20 explicit samples: runtime 0 errors, layout 0 issues, motion 0 errors.
- Dense snapshots cover 0.00, 0.25, 0.50, 0.75, 1.20, 1.65, 2.10, 2.55, 2.95, 3.35, 3.90, 4.45, 4.85, 5.35, 5.80, 6.25, 6.80, 7.30, 7.65 and 7.95 seconds.
- The remaining non-gating lint warning is file length. The scene is intentionally monolithic because splitting the fixed WebGL world into sub-compositions would violate the single-scene reference contract.
- `hyperframes keyframes --ghost` reported no DOM keyframes because the actual subject is the WebGL camera; rendered pixel checkpoints and the seekable timeline are the relevant proof.

## Rendered reference MP4

- File: `renders/apartment-106-ef-v10-reference.mp4`
- SHA-256: `0ffc88e8ae20fe5ea7782f00faf4415aca20132ef80bbaf21cc0367fc5b6f868`
- Video: H.264, 1920 x 1080, yuv420p, 30 fps, 8.000 seconds, 240 frames.
- Audio: none; this deterministic reference controls only topology and camera motion.
- Final-file QA: whole clip inspected at 8 fps; opening approach, threshold crossing and Terrace turn/look-back inspected at 12 fps.
- No black frame, cut, dissolve, teleport, duplicated opening, wall penetration, furniture collision or downward-floor ending was found.
- QA evidence: `qa-final/contact-whole-8fps.jpg`, `qa-final/contact-threshold-12fps.jpg`, `qa-final/contact-turn-lookback-12fps.jpg`, and `qa-final/ffprobe.json`.

## Gate

This preview is not a Seedance submission package and has no provider cost. Render the local MP4 only after the user approves this exact Studio preview. A later paid Seedance V2V package still requires a fresh preflight PASS, provider-readable URL verification, current price evidence and explicit one-task approval.
