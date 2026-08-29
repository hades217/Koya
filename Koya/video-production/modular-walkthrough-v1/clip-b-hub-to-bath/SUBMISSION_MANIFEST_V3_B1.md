# Clip B V3 / Piece B1 — Submission Manifest

Status: provider succeeded; internal physical-continuity QA passed; awaiting user approval.

- Task ID: `cgt-20260825152023-v6tlx`
- Submission time: 2026-08-25 15:20 AEST
- Initial provider status: `queued`
- Final provider status: `succeeded`
- Local output: `outputs/candidates-v3-b1/cgt-20260825152023-v6tlx.mp4`
- Completion tokens: 108,900
- Actual cost: CNY 4.5738

- Provider/model: Seedance 2.5 via `ep-20260812221158-hb576`
- Model resolved live: `doubao-seedance-2-5-260628`
- Workflow: first frame + last frame to video
- First frame: `storyboards-v3/accepted/B3-00-HUB-AXIS-W-v1.png`
- Last frame: `storyboards-v3/accepted/B3-01-PASSAGE-MID-W-v1.png`
- Duration: 5 seconds
- Resolution: 720p
- Ratio: inherited from first frame; no explicit ratio submitted
- Audio: Seedance native audio enabled
- Prompt: `PROMPT_V3_B1.md`
- Retry policy: no automatic retry; task ID must be recorded before any further action

## Cost basis

- Live price checked 2026-08-25: CNY 0.042 per 1,000 video completion tokens (`V2VCompletion`).
- Same-endpoint prior 5-second first/last-frame usage: 108,900 completion tokens.
- Estimated cost for this one submission: CNY 4.5738.
- Actual charge depends on the task's returned completion-token usage.

## Acceptance gate

The output is rejected if the camera changes direction, architecture morphs, a room is replaced, any object is crossed, or the endpoint composition is reached by a dissolve rather than physical forward travel.
