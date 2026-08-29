# QA report

## Result

**PASS.** The MPR stage now has a literal entry, compact room reveal and same-door exit.

## Findings

- The resident turns with visible foot/body motion, then walks toward the camera.
- Camera retreats continuously; room depth changes by parallax rather than zoom.
- Camera crosses the original doorway first and remains facing the MPR while the resident follows.
- Doorway, jamb, door leaf, desk, daybed and window stay continuous.
- Bottle remains in anatomical left hand; wardrobe and daylight remain stable.
- Kitchen-side view retains one island and one cabinetry line.
- No dissolve, cut, teleport, wall penetration, furniture swap or duplicated architecture found in 8 fps and 16 fps review sheets.

## Joined master

- `outputs/koya-continuous-review-mpr-complete-57s.mp4`
- Duration: 56.666667 seconds.
- 1280x720 H.264, 24 fps; 32 kHz stereo AAC.
- Duplicate first frame removed and audio joined with a 0.08-second crossfade.
- Junction inspected in `qa/join-dense-01.jpg` and `qa/join-dense-02.jpg`.

The master ends with the camera outside on the kitchen side, still facing the resident/MPR. The next stage must visibly reorient in the kitchen before moving east toward Bedroom 1.
