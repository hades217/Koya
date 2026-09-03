# Segment B submission record

- Status: `SUCCEEDED_QA_PASS`
- Task ID: `cgt-20260817214856-d4dx5`
- Submitted: `2026-08-17T23:48:56+10:00`
- Endpoint: `ep-20260812221158-hb576`
- Model: `doubao-seedance-2-5-260628`
- Duration request: 5 seconds
- Resolution: 720p
- Ratio: inherited 16:9 from the 1280x720 first frame; no explicit ratio sent
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Prompt and reference manifest: `APPROVAL_GATE.md`
- First-frame SHA-256: `f385c6065da8e3d8c5b075710d0982157ccd3459b44e255d91001312dcc6c6d2`
- Last-frame SHA-256: `5aa309c741fda46b92f8b29087c7cf6e55d6557d9481bf0069f148bb20e5f26a`
- User authorization: `后面内容继续完成`, received immediately after the exact Segment B cost gate
- Authorized scope: this Segment B task only; no automatic retry or Segment C submission

The provider accepted the request and returned a queued task. Further checks must poll this exact task ID; do not create another task.

## Result

- Provider status: `succeeded`
- Actual duration: 5.056 seconds
- Output: `outputs/cgt-20260817214856-d4dx5.mp4`
- Output SHA-256: `c6374395383c95152bb524e1ad26bb778bed3bf2958a5aad2a51beeafe0a278a`
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo; max volume -8.3 dB
- Completion tokens: 108,900
- Actual cost: CNY 4.5738 at CNY 0.042 per 1,000 completion tokens
- QA: `PASS_LITERAL_LIFT_EXIT_TO_APARTMENT_DOOR`
- Continuous A+B preview: `outputs/koya-continuous-preview-through-apartment-106-door-16s.mp4`
