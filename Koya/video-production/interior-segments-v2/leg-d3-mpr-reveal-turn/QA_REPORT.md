# QA report

## Result

**PARTIAL PASS.** The complete five-second task is rejected. The clean opening 0.85 seconds is accepted as a short MPR room-reveal continuation.

## Failure

- During the attempted 150-180-degree turn, the woman, window, daybed, desk and doorway become semi-transparent and dissolve into the reverse view.
- Evidence: `qa/dissolve-boundary-01.jpg`, `qa/dissolve-boundary-02.jpg`, and full frames `qa/check-3.25.png` through `qa/check-3.625.png`.
- This is not physical camera motion and fails `literal_walkthrough`.

## Retained material

- The initial 0.85 seconds remain fully continuous from the actual 50.917-second endpoint.
- It advances inside the same compact room and keeps desk left, daybed right and window ahead.
- Accepted partial master: `outputs/koya-continuous-review-mpr-reveal-safe-52s.mp4`, duration 51.682 seconds.
- The rejected full clip was never joined.

The exit strategy was changed to a straight backward dolly while facing the resident, avoiding any architecture-replacing camera turn.
