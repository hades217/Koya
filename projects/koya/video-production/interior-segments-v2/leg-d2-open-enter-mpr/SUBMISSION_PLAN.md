# Submission plan

- Purpose: D2 only, open the same closed MPR door and physically enter.
- Accepted source master: `../leg-d1-kitchen-to-mpr-closed-door/outputs/koya-continuous-review-through-mpr-closed-door-46s.mp4`.
- Exact start: extracted actual frame at 45.936 seconds.
- Exact end: actual-endpoint-derived MPR threshold frame, camera just inside.
- Endpoint/model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Parameters: 5 seconds, 720p, inherited 16:9, native audio, return last frame.
- Paid tasks in this pass: one maximum.
- Working cost estimate: CNY 4.5738 if provider usage is again 108,900 completion tokens at the project ledger rate of CNY 0.042 per 1,000.
- Stop gate: do not submit D3 or D4 until D2 passes dense threshold QA.

Reject if the door dissolves, the wall changes, the resident appears inside without crossing, the camera remains outside, the room becomes oversized, or the door/threshold is not retained at the end.
