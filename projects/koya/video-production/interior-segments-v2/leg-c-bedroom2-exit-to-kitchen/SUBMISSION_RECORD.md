# Seedance submission record — Bedroom 2 same-door exit to kitchen axis

Status: `PROVIDER_SUCCEEDED_SINGLE_TASK`

- User authorization: `继续完成`
- Task ID: `cgt-20260819191900-nzq2t`
- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Paid tasks created: one
- Requested duration: 5 seconds
- Delivered duration: 5.056 seconds
- Ratio: 16:9, inherited from the 1280x720 first frame
- Resolution: 720p
- Native synchronized audio: yes
- Completion tokens: 108,900
- Current V2V rate: CNY 0.042 per 1,000 completion tokens
- Actual cost: `108,900 / 1,000 x 0.042 = CNY 4.5738`
- Output: `outputs/cgt-20260819191900-nzq2t.mp4`
- SHA-256: `890de1e5235d14914ccba5d6a45731a255692a4232ac4cb7c5ab5627a4cdfb61`

The first submission attempt included an explicit `ratio` and was rejected before task creation because first/last-frame generation inherits the first frame ratio. It created no task and no charge. The corrected request removed only that invalid parameter.

Two sequential status checks downloaded identical copies of the same successful task. SHA-256 equality was verified and the redundant copy was removed. This was not a second generation and did not create a second charge.
