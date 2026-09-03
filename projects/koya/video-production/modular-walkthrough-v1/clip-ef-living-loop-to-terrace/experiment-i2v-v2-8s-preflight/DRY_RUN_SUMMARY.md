# Non-billable ArkCLI dry-run

Run date: 2026-08-29 Australia/Brisbane.

Result: `PASS FOR REQUEST CONSTRUCTION / PARTIAL CLIENT PREVIEW`

- Network: blocked by dry-run.
- Filesystem: read-only by dry-run.
- Model argument: `ep-20260812221158-hb576`.
- Explicit modality: `video`.
- Input reference: `first_frame:@inputs/00-first-frame.png`.
- Duration: `8`.
- Ratio: `16:9`.
- Resolution: `1080p`.
- Generate audio: `true`.
- Watermark: `false`.
- Return last frame: `true`.
- Prompt: byte-for-byte content of `PROMPT.txt`.

The preview correctly states that execution will materialize the local image and resolve the data-plane profile. This dry-run created no task and incurred no generation fee.
