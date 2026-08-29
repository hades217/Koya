# Seedance preflight review

Decision: **PASS FOR ONE EXPERIMENTAL TASK / USER APPROVAL STILL REQUIRED**

Reviewed at: 2026-08-28T23:53:00+10:00  
No task submitted: yes  
Package fingerprint: `sha256:3807d50cc1aa8b0cb3c212b59625a1cb48d699e32030a1104167406e3400f29c`

## Blockers

- None for this deliberately restricted single-visible-scene I2V experiment.
- This PASS does not approve the complete Living-loop-to-Terrace route.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Live endpoint status: Running in `cn-beijing`; image input and video output supported; `multimodal_to_video` advertised.
- Mode: first-frame image-to-video.
- Task count / output count: 1 / 1.
- Parameters: 5 seconds, 16:9, 1080p, generated audio on, watermark off, return last frame on, seed unavailable.
- Output directory: `outputs/`.

## Inputs and roles

| Input | Role | Provenance | Scope result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-first-frame.png` | `first_frame` | V7 plan-rebuild storyboard frame 05; Concept Design / Artist Impression | PASS for a small move confined to the visible Living scene | `ece6e45c5045f8906abe5bc3881512e258e3f8d27e4c305e4cac6a11904eac55` |

No V6 video, rejected V8/V15 source, rejected V9 source, last frame, reference image, reference video or second candidate is included.

## Motion review evidence

- The supplied frame already shows the Living room, sofa, grouped glazing, open east bay, interior floor and Terrace.
- The requested motion is a straight approximately 1.2-metre forward walk with no meaningful turn and no threshold crossing.
- Timing ledger totals exactly 5.0 seconds.
- The endpoint remains inside the same visible room immediately before the same visible threshold.
- Because only a first frame is supplied, the model can still fail to produce real parallax; such an output will be rejected during frame-level QA rather than concealed or automatically retried.

## Cross-input consistency

| Feature | First frame | Other input | Result |
|---|---|---|---|
| Living scale | broad visible room | none | PASS |
| Sofa | one fixed sofa at left | none | PASS |
| Glazing | continuous black-framed glazing | none | PASS |
| East opening | one already visible open bay at right | none | PASS |
| Daylight | stable daytime | none | PASS |

## Prompt review

- Supported role semantics only: PASS; one explicit `first_frame` role.
- Route physically visible: PASS for this restricted experiment; no unseen room or turn is requested.
- Timing valid: PASS; intervals total 5.0 seconds.
- Immutable geometry: explicit.
- Forbidden cuts, morphs, zoom-only motion, new doors and threshold crossing: explicit.
- No instruction/input contradiction: PASS.

## Provider and price evidence

- Authentication: logged in with active Platform profile at review time.
- Live endpoint resolution: Running; resolved model `doubao-seedance-2-5-260628`.
- Non-billable dry-run: PASS after correcting boolean flag syntax; prompt, duration, ratio, resolution, audio, watermark and return-last-frame match the manifest. Client preview remains partial until execution materializes the local first-frame file.
- Current account pricing query returned `V2VCompletion`, `V2V1080Completion`, `NV2VCompletion` and `NV2V1080Completion` only. It returned no `I2VCompletion`, `FLF2VCompletion` or `ToVCompletion` item.
- Exact I2V price and total before generation: **unavailable**.
- Failure/cancellation/rejection billing status: **unavailable**.
- No V2V price is used as a substitute estimate.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. One approval authorises exactly one task and one output. It does not authorise a retry or second candidate.

Exact approval phrase: **确认提交 Clip EF I2V 实验 V1，单任务单输出，指纹 3807d50cc1aa，5秒、16:9、1080p、生成音频，费用 unavailable，实际按提供方完成用量结算**
