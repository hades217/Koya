# Provider Validation Log

## 2026-08-19 — rejected before task creation

- Attempted mode: `reference`
- Inputs: `first_frame` plus two `reference_image` items
- Provider response: `first/last frame content cannot be mixed with reference media content`
- Task ID: none
- Paid generation created: no
- Charge expected: none
- Request ID: `02178709572567869bee7302ebcfceb03bd4615aad0dc321bba51`

Corrective action: use the already-tested extension contract: the accepted 21.047-second film as `reference_video`, followed by the two spatial stills as `reference_image`; `omni_reference_task_type=extend`, `duration=10`, `ratio=adaptive`.
