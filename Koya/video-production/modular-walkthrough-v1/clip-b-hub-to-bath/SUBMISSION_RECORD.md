# Clip B — Seedance 2.5 Submission Record

Status: `PROVIDER_SUCCEEDED_USER_REJECTED_VISUAL_REALISM`

- User authorization: `不错，继续下一个`
- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Paid task ID: `cgt-20260825132439-gptgg`
- Requested duration: 5 seconds
- Delivered duration: 5.056 seconds
- Ratio: inherited from the 1672x941 first frame (16:9 display geometry); no explicit ratio field is sent because Seedance first/last-frame generation rejects it.
- Resolution: 720p
- Generate native audio: true
- Versions requested: one
- Inputs uploaded to provider: one first frame and one last frame only
- Local-only route QA reference: `references/02-qa-wet-threshold.png`
- Current known V2V rate: CNY 0.042 per 1,000 completion tokens
- Working estimate from the same endpoint's prior 5-second usage pattern: 108,900 completion tokens, approximately CNY 4.5738. Final charge must use provider-returned actual tokens.
- Completion tokens: 108,900
- Actual cost: `108,900 / 1,000 x 0.042 = CNY 4.5738`
- Delivered file: `outputs/review/clip-b-hub-to-bath-review-v1.mp4`
- SHA-256: `62e3defe15990000af11f5c357e6771ba5c685097e699950e7553142146f3faa`
- Rejected output: `outputs/rejected/clip-b-hub-to-bath-rejected-v1.mp4`
- User rejection reason: later interior frames look like architectural design renders rather than a real photographed apartment.

No automatic retry or second candidate is authorized. Confirm a task ID before any retry decision.

## Non-billable validation event

The first execution attempt was rejected before task creation because an explicit `ratio=16:9` cannot be combined with first/last-frame generation; the service requires the output ratio to follow the first frame. It returned no task ID and created no generation charge. The corrected request removes only that redundant field.
