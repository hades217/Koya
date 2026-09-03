# Segment C Retry 1 submission record

- Status: `SUCCEEDED_QA_PASS`
- Task ID: `cgt-20260817221551-zv9sf`
- Submitted: `2026-08-18T00:15:51+10:00`
- Endpoint: `ep-20260812221158-hb576`
- Model: `doubao-seedance-2-5-260628`
- Duration request: 5 seconds
- Resolution: 720p
- Ratio: inherited 16:9 from the 1280x720 first frame; no explicit ratio sent
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Prompt and reference manifest: `APPROVAL_GATE.md`
- First-frame SHA-256: `8ac6040b4b5ddfabd8f6031360ca0a2ebe63c1aba57e08abbdb7ed305483aa68`
- Last-frame SHA-256: `50419736db35fe40865770c135e4e2efa2a4f930e06c918c14d0526d0f747ce2`
- User authorization: `继续做做成功`, received immediately after the exact Retry 1 gate
- Authorized scope: this Retry 1 task only; no automatic Retry 2 or Segment D task

The provider accepted the request and returned a queued task. Further checks must poll this exact task ID; do not create another task.

## Result

- Provider status: `succeeded`
- Actual duration: 5.056 seconds
- Output: `outputs/cgt-20260817221551-zv9sf.mp4`
- Output SHA-256: `125a76ac62b45804cf13286c575e9c659c083793a8054f22764684f513cde593`
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo; max volume -3.7 dB
- Completion tokens: 108,900
- Actual cost: CNY 4.5738 at CNY 0.042 per 1,000 completion tokens
- QA: `PASS_LITERAL_APARTMENT_ENTRY_THRESHOLD`
- Continuous A+B+C preview: `outputs/koya-continuous-preview-through-apartment-entry-21s.mp4`
