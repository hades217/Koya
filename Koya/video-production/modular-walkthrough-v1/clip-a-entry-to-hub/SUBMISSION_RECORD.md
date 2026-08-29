# Clip A — Seedance 2.5 Submission Record

Status: `USER_APPROVED_ACCEPTED`

- User authorization: `搞清楚了么，先完成第一段视频`
- Paid task ID: `cgt-20260825124458-kzwf7`
- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Paid tasks created: one
- Requested duration: 10 seconds
- Delivered duration: 10.048 seconds
- Ratio: 16:9
- Resolution: 1280x720, 24 fps
- Native synchronized audio: AAC, 32 kHz stereo
- Completion tokens: 216,900
- Current V2V rate: CNY 0.042 per 1,000 completion tokens
- Actual cost: `216,900 / 1,000 x 0.042 = CNY 9.1098`
- Accepted output: `outputs/accepted/clip-a-entry-to-hub-v1.mp4`
- SHA-256: `a54e2aaf99a0ac8aad04daefbd8c5e851d579386e182293868ab936b56633228`

## Non-billable validation failures before submission

Two provider validation requests returned errors before any task ID was created:

1. Image-role shorthand was not materialized by the installed CLI.
2. Seedance 2.5 rejected mixing a `reference_image` with a `last_frame`.

The successful request therefore used exactly one `first_frame` and one `last_frame`. The kitchen-turn image remained a local prompt and QA reference. Neither validation failure created a task or incurred generation completion tokens.

One byte-identical duplicate local download of the same task was removed. This was not another provider task; the retained review MP4 has the same SHA-256.

## User gate

- User decision: approved with `不错，继续下一个`.
- Clip A was promoted from `outputs/review/` to `outputs/accepted/` before Clip B submission.
