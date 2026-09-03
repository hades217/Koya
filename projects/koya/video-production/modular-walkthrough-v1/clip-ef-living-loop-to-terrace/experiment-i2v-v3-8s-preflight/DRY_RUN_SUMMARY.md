# Non-billable ArkCLI dry-run

Result: `PASS FOR REQUEST CONSTRUCTION / PARTIAL CLIENT PREVIEW`

- Exact duration: 8 seconds.
- Explicit `ratio` parameter: omitted.
- Output ratio behaviour: inherited from the 1672 x 941 first frame, effectively 16:9, as required by the provider's first-frame I2V validation.
- Resolution: 1080p.
- Generated audio: true.
- Watermark: false.
- Return last frame: true.
- Task created: no.

The corrected dry-run payload contains no `ratio` field.
