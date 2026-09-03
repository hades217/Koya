# Clip B V3 / Piece B1 — Internal QA Report

Status: physical continuity passed, but the 5.056-second source is rejected for pace. A free 3x local retime has been produced for user review. This is not yet an accepted production clip.

## Provider and file

- Task: `cgt-20260825152023-v6tlx`
- Output: `outputs/candidates-v3-b1/cgt-20260825152023-v6tlx.mp4`
- SHA-256: `27c0c516933ec5208917a42536cfac55e67ddeaf8ec266528581c3f00c90b2ec`
- Duration: 5.056 seconds
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, stereo, 32 kHz
- Completion tokens: 108,900
- Actual cost: CNY 4.5738

## Dense visual inspection

- A 4 fps contact sheet covers the complete shot at 0.25-second intervals: `qa-v3-b1/contact-4fps.png`.
- The camera advances on one fixed west-facing axis. It does not pan, turn, orbit or change rooms.
- The same single island remains left; the same linear kitchen remains right; the far solid wall and two right-side doors remain fixed.
- Foreground island and kitchen edges change against the background with continuous parallax, so the move is not a static image zoom.
- No cut, dissolve, room replacement, wall pass-through, duplicate island, person or extra centred door was found.
- Automated scene-change scores remain far below the configured cut threshold; no freeze longer than 0.35 seconds was detected.
- Lighting remains consistent daytime. The final frame settles without a topology change.

## Audio inspection

- A real stereo audio stream is present.
- Integrated sample statistics are non-silent: peak approximately -22.43 dB, RMS approximately -42.00 dB.

## Next gate

The original 5.056-second file must not enter the final edit at normal speed. Review `outputs/local-speed-tests/clip-b-v3-b1-speed-3x.mp4`, duration 1.750 seconds. Do not submit Piece B2 until the user approves the corrected pace. If approved, use `qa-v3-b1/B1-actual-final-frame.png` as the first-frame source for B2.
