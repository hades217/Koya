# Clip EF V11 all-inputs final QA

Decision: **REJECTED**

## Passed observations

- One output only; no retry.
- Buyer-facing render style replaced the white-box look.
- Interior opening sweep is visually continuous.
- Compact dining, one kitchen island, north TV wall, broad Living and L-shaped sofa remain readable.
- South/east glazing is retained.
- Camera approaches and physically crosses the open east threshold without an obvious cut, dissolve or wall penetration.
- Daylight and exposure remain stable.
- Output contains a continuous audio stream.

## Blocking failure

From approximately 6.0 seconds onward, the camera continues along the Terrace. It does not complete the required turn back through the same opening. The final frame therefore shows the Terrace path and planting rather than the contracted look-back containing Living, Dining, the one island, Kitchen and north TV wall.

This violates `PROMPT_ZH.txt` sections `7.15–8.00 秒` and `最终画面契约`. Provider `succeeded` is not visual acceptance.

## Evidence

- `qa-final/contact-whole-8fps.jpg`
- `qa-final/contact-interior-pan-12fps.jpg`
- `qa-final/contact-threshold-12fps.jpg`
- `qa-final/contact-terrace-lookback-12fps.jpg`
- `qa-final/ffprobe.json`
- `qa-final/automated-av-check.txt`

No retry is authorised or submitted.
