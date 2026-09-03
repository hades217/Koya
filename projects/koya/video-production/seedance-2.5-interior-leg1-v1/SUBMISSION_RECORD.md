# Interior Leg 1 — Submission Record

Date: 2026-08-18 (Australia/Brisbane)

User approval: `确认提交 Interior Leg 1，预计 CNY 54.4698`

## Approved payload

- Provider: Volcengine Ark / Seedance
- Endpoint: `ep-20260812221158-hb576`
- Model: `doubao-seedance-2-5-260628`
- Operation: extend the preserved 21.047-second video
- Generated continuation: 30 seconds
- Ratio: adaptive, returned 16:9
- Resolution: 720p
- Audio: native synchronized audio enabled
- Return last frame: enabled
- Watermark: disabled
- Prompt: `PROMPT.txt`
- Submitted reference manifest: `REFERENCE_MANIFEST.md`

## Task ledger

### Primary

- Task ID: `cgt-20260818145357-8t8g4`
- Final provider status: `succeeded`
- Completion tokens: `1,101,600`
- Rate: CNY `0.042` / 1,000 completion tokens
- Actual charge calculation: `1,101,600 / 1,000 x 0.042 = CNY 46.2672`
- Video: `outputs/task-cgt-20260818145357-8t8g4/cgt-20260818145357-8t8g4.mp4`
- Last frame: `outputs/task-cgt-20260818145357-8t8g4/cgt-20260818145357-8t8g4-last-frame.jpeg`
- QA disposition: rejected

### Delayed duplicate

- Task ID: `cgt-20260818145533-jlz2g`
- Final provider status: `succeeded`
- Completion tokens: `1,101,600`
- Rate: CNY `0.042` / 1,000 completion tokens
- Actual charge calculation: `1,101,600 / 1,000 x 0.042 = CNY 46.2672`
- Video: `outputs/task-cgt-20260818145533-jlz2g/cgt-20260818145533-jlz2g.mp4`
- Last frame: `outputs/task-cgt-20260818145533-jlz2g/cgt-20260818145533-jlz2g-last-frame.jpeg`
- QA disposition: rejected

## Duplicate incident

The high-level ArkCLI submission performed model lookup and then exited without stdout or a task ID. A service-side task appeared only after the fallback request had already been issued. The fallback therefore created a second identical-payload task. Deletion of `cgt-20260818145533-jlz2g` was attempted immediately while it was running, but the provider returned: `Cannot delete task because it is currently running`.

No third task or further paid retry was submitted.

Combined completion tokens: `2,203,200`

Combined actual charge calculation: `2,203,200 / 1,000 x 0.042 = CNY 92.5344`
