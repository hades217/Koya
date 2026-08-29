# Clip B V3 / Piece B2 — Submission Manifest

Status: provider succeeded, but user and internal QA rejected the entire output for impossible wall-turn topology. Do not salvage.

- Task ID: `cgt-20260825153750-cjdhl`
- Initial provider status: `queued`
- Final provider status: `succeeded`
- Completion tokens: 108,900
- Actual cost: CNY 4.5738
- Rejected output: `outputs/rejected/clip-b-v3-b2-rejected-wall-turn.mp4`
- Failure: the camera reached the blank wall, turned toward that wall, and only then generated the wet-area doors; this is not a physical route.

- Provider/model: Seedance 2.5 via `ep-20260812221158-hb576`
- Workflow: first frame + last frame to video
- First frame: `qa-v3-b1/B1-actual-final-frame.png`
- Last frame: `storyboards-v3/accepted/B3-02-WET-DOORS-N-v1.png`
- Physical action: about 1.2 metres west, then one 90-degree right/north turn
- Provider duration: 5 seconds
- Intended final screen duration: about 2.0 seconds
- Planned local retime after QA: 2.5x
- Resolution: 720p
- Ratio: inherited from first frame
- Audio: Seedance native audio enabled
- Retry policy: no automatic retry
- Estimated cost: CNY 4.5738 based on 108,900 completion tokens at CNY 0.042/1,000 tokens

## Acceptance gate

Reject if the camera enters a room, dissolves between axes, invents a corridor, merges the two wet rooms, swaps the door positions or changes the kitchen geometry.
