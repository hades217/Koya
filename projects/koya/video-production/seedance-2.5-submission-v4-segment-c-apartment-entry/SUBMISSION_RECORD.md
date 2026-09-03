# Segment C submission record

- Status: `SUCCEEDED_QA_REJECTED`
- Task ID: `cgt-20260817220121-98zlr`
- Submitted: `2026-08-18T00:01:21+10:00`
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
- Last-frame SHA-256: `7e39bc410f8c0f1e3f3701942dc456959724972795b4f0211624f67c2641fbbb`
- User authorization: `确认`, received immediately after the exact Segment C cost gate
- Authorized scope: this Segment C task only; no automatic retry or Segment D submission

The provider accepted the request and returned a queued task. Further checks must poll this exact task ID; do not create another task.

## Result

- Provider status: `succeeded`
- Actual duration: 5.056 seconds
- Output: `outputs/cgt-20260817220121-98zlr.mp4`
- Output SHA-256: `21a81bd717b883c660a72b680ac4b7ef35ad580645aa10edb3d861c0e4df50df`
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo; max volume -4.9 dB
- Completion tokens: 108,900
- Actual cost: CNY 4.5738 at CNY 0.042 per 1,000 completion tokens
- QA: `REJECT_DUPLICATED_ENTRY_DOOR_GEOMETRY`
- This output must not be included in the final one-take assembly.
