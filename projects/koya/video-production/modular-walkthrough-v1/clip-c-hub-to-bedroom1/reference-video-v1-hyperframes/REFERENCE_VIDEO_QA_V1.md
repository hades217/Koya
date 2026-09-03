# Clip C1 continuous reference video — QA v1

Date: 2026-08-26 (Australia/Brisbane)

Decision: **PASS as structural motion proof; NOT approved as a paid Seedance submission package**

Completion state: `preview_ready`

## Output

- MP4: `renders/koya-106-clip-c1-reference-v1-5s.mp4`
- Duration: 5.000 seconds
- Resolution: 1920x1080
- Frame rate: 24 fps
- Codec: H.264
- Audio: none
- Size: 1,976,410 bytes

## Automated checks

- HyperFrames lint: 0 errors, 0 warnings.
- Runtime: 0 errors, 0 warnings.
- Layout: 0 issues across 20 samples.
- Motion assertions: 0 errors, 0 warnings.
- Full render completed successfully.

## Structural review

- PASS: camera begins in the Living hub with the same Bedroom 1 doorway visible.
- PASS: camera advances toward that same opening; the door does not appear from nowhere.
- PASS: camera physically crosses between the doorway jambs with visible parallax.
- PASS: camera reaches the bedroom before performing the main right turn.
- PASS: full bed becomes readable around the planned reveal interval.
- PASS: the rightward movement continues beyond the bed toward the east/right glazing.
- PASS: no cut, dissolve, room swap, wall penetration or static-image zoom was observed in the dense checkpoint sheets.

## Remaining limitations

- The reference remains a simplified fixed-scene structural render, not the approved photoreal material target.
- The detailed C0/C3/C5/C7 visual anchors are still pending user approval.
- The detailed anchors must not be uploaded together as ordered Seedance reference images.
- Before any paid task, the approved visual intent must be reconciled with the fixed scene or prompt without introducing geometry conflicts.
- A fresh `seedance-preflight-review`, live endpoint/model resolution, current price evidence and explicit approval are still mandatory.

## Review evidence

- Full route sheet: `qa-reference-v1/full-5fps-contact.jpg`
- Doorway/turn dense sheet: `qa-reference-v1/threshold-turn-12fps-contact.jpg`
- Technical metadata: `qa-reference-v1/ffprobe.json`

