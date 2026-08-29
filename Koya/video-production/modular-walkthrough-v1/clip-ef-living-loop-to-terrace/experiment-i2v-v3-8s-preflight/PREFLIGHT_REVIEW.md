# Seedance preflight review

Decision: `PACKAGE UNCHANGED / TRANSPORT RETRY REQUIRES NEW USER APPROVAL`

Reviewed at: 2026-08-29 Australia/Brisbane.

No generation task created from the attempted request: yes.

Package fingerprint: `sha256:c6f6062b91f5f778633675add9a96a551c06cf84a5cc205b8ea5ec167c7e9a10`

## Blockers

- None for the restricted eight-second Living-to-Terrace segment.

## Correction from V2

- The V2 provider request was rejected before task creation because first-frame I2V does not accept an explicit `ratio` parameter.
- Task-list read-back showed no new task and no new usage entry.
- V3 removes only the CLI/API `ratio` field. Duration, prompt, input bytes, resolution, audio, watermark and output count remain unchanged.
- The output inherits 1672 x 941 from the exact first frame, which is effectively 16:9.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Endpoint status at review: Running.
- Mode: first-frame image-to-video.
- Task count / output count: 1 / 1.
- Parameters: 8 seconds; ratio inherited from first frame; 1080p; generated audio on; watermark off; return last frame on.
- Output directory: `outputs`.

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-first-frame.png` | `first_frame` | Byte-identical V7 plan-rebuild frame 05 | PASS for visible Living/glazing/threshold/Terrace | `ece6e45c5045f8906abe5bc3881512e258e3f8d27e4c305e4cac6a11904eac55` |

Rejected V6, V8 and V9 assets remain excluded.

## Structural review

- Exact scope: Living position -> existing open east sliding bay -> same visible Terrace.
- Timed motion ledger totals exactly 8.0 seconds.
- Threshold approach, jamb parallax, physical track crossing and Terrace arrival are explicit.
- MPR, Kitchen, unseen rooms and a large return turn are excluded.
- V7 frames 06-08 remain human QA targets only, not provider inputs.

## Prompt review

- Prompt SHA-256: `080b3098396fad87a2f31394fec32971e40a277ee4003df5838abaf8d79cdeb2`.
- One supported provider role only: PASS.
- Route is visible and physically reachable: PASS.
- Translation rather than static zoom is explicit: PASS.
- No unsupported ordered-keyframe semantics: PASS.
- No input contradiction: PASS within the restricted route.

## Provider and price evidence

- Live endpoint resolution: Running, `doubao-seedance-2-5-260628`, multimodal input and video output.
- Corrected non-billable dry-run: PASS; payload has duration 8 and no ratio field.
- Exact I2V price: `unavailable`; live pricing exposes V2V/NV2V items but no I2V item.
- Failure, cancellation or provider-rejection billing: `unavailable`.

## Approval gate

The prior V3 authorization was consumed by a create attempt that failed before task creation with a non-JSON HTTP 400 response. Task-list read-back confirmed no new task or task usage entry. The package itself is byte-for-byte unchanged, but no retry may occur until the user explicitly approves one retry of this same fingerprint.

Exact approval phrase:

`确认重试 Clip EF I2V V3，单任务单输出，指纹 c6f6062b91f5，8秒、首帧继承16:9、1080p、生成音频，费用 unavailable，实际按提供方完成用量结算`
