# Clip B V4 — Submission Manifest

Status: rejected. The micro-piece strategy failed, and the single complete five-second replacement also failed spatial QA. No further task is authorized or queued.

## Piece 1

- Task ID: `cgt-20260825161206-b7x9x`
- Initial status: `queued`
- Final status: `succeeded`, user rejected
- Actual cost: CNY 4.5738
- Rejected output: `outputs/rejected/clip-b-v4-piece1-rejected-micro-move.mp4`
- Failure: five seconds were wasted on only 0.7 metres and 35 degrees; the complete visible action was not performed.
- First frame: `storyboards-v4/accepted/B4-00-PASSAGE-APPROACH-W.png`
- Last frame: `storyboards-v4/accepted/B4-01-BATH-APPROACH-PIVOT-v1.png`
- Physical move: about 0.7 metres west plus about 35-degree right yaw
- Provider duration: 5 seconds
- Intended final duration: 1.3–1.6 seconds
- Endpoint: `ep-20260812221158-hb576`
- Resolution: 720p, ratio inherited, native audio enabled
- Estimated cost: CNY 4.5738
- Retry policy: no automatic retry

## Cancelled pieces

- Piece 2: not submitted; no task ID; no cost.
- Piece 3: not submitted; no task ID; no cost.
- Three-piece micro-generation is forbidden going forward.

## Replacement

- One five-second task covering the entire physical route from Node 1 to Node 4.
- Exact timing and prompt: `PROMPT_V4_SINGLE_5S.md`.
- Task ID: `cgt-20260825162151-twrrd`
- Provider status: succeeded
- QA status: rejected
- Submitted: 2026-08-25
- Endpoint: `ep-20260812221158-hb576`
- Resolution: 720p, ratio inherited from the 16:9 endpoints, native audio enabled
- Duration: 5 seconds
- Input: `storyboards-v4/accepted/B4-00-PASSAGE-APPROACH-W.png`
- Output anchor: `storyboards-v4/accepted/B4-03-BATH-INSIDE-N.png`
- Retry policy: no automatic retry
- Actual cost: CNY 4.5738 (108,900 completion tokens)
- Rejected output: `outputs/rejected/clip-b-v4-single-5s-rejected-wrong-route.mp4`
- Failure: the generated starting architecture does not preserve the approved kitchen-passage anchor; the camera approaches a blank wall/incorrect door geometry, then the corridor and Bath are synthesized as changing spaces rather than one fixed floor-plan route. Although it ends in a Bath-like room, it is not a physical traversal of the approved apartment.
