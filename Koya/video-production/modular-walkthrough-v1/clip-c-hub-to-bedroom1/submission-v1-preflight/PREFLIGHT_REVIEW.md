# Seedance preflight review

Decision: **PASS**
Reviewed at: 2026-08-26T22:12:00+10:00
No task submitted: **yes**
Package fingerprint: `sha256:41ebb0a24f75d74ac815324717c8797ca76c959064b64d9e0e7eb13f4f12dd20`

## Blockers

- None.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Mode: `video_editing`
- Task count / output count: `1 / 1`
- Parameters: inherit reference duration (`duration=-1`, reference is 5.000 seconds); inherit reference aspect ratio (`ratio=adaptive`, reference is 16:9); `resolution=1080p`; `generate_audio=true`; `watermark=false`; seed `unavailable`
- Output directory: `outputs/`

## Inputs and roles

| Input | Role | Provenance | Same-scene result | SHA-256 |
|---|---|---|---|---|
| `inputs/00-reference-motion.mp4`, delivered as `https://intervals-assembled-herald-unavailable.trycloudflare.com/00-reference-motion.mp4` | `reference_video` | Continuous 5.000-second render from the approved Clip C1 fixed WebGL scene and camera path | PASS: HTTPS response is `video/mp4`, 1,976,410 bytes and byte-identical to the locked local file; scene ID `koya-106-clip-c1-fixed-shell-v1` | `eb1f4ab1e4c23dcb75bf747b62cf36e1b6ea081f6632615a811ebd7de22ca8cc` |

No `reference_image` input is present. The independently generated whole-room visual anchors are human-review material only and are excluded from this provider task.

## Video review evidence

- Normal-speed playback: PASS; the full five-second route was reviewed as one continuous move.
- Reverse playback: PASS; `REVERSE_REVIEW_ONLY.mp4` preserves the same room, doorway and jambs in reverse and exposes no hidden scene replacement.
- Whole-video checkpoints: PASS; `../reference-video-v1-hyperframes/qa-reference-v1/full-5fps-contact.jpg` samples the entire route at 5 fps.
- Dense turn/threshold checkpoints: PASS; `../reference-video-v1-hyperframes/qa-reference-v1/threshold-turn-12fps-contact.jpg` samples the doorway crossing and in-room turn at 12 fps.
- Route overlay: PASS; `../route-audit-v1/CLIP_C_ROUTE_OVERLAY.png` agrees with the Apartment 106 Bedroom 1 path.
- Persistent-landmark audit: PASS; one Bedroom 1 doorway persists from Living hub approach through the same jamb crossing; the north-head bed, west/left WIR relation and east/right glazing remain in the same fixed scene.

## Cross-input consistency

| Feature | Reference video | Other input | Result |
|---|---|---|---|
| Temporal geometry | Sole continuous fixed-scene controller | None | PASS |
| Bedroom 1 doorway | Same visible door and jambs throughout approach/crossing | Human-review route overlay agrees | PASS |
| Bed | Head north, foot south | Human-review detailed anchors agree | PASS |
| WIR | West/left of Bedroom 1 | Human-review detailed anchors agree | PASS |
| Terrace glazing | East/right; final inspection target | Human-review detailed anchors agree | PASS |
| Materials and styling | Simplified structural surfaces | Human-review anchors express appearance intent only and are not uploaded | PASS |

## Prompt review

- Supported role semantics only: PASS; the exact task uses one explicit `reference_video` role in video-editing mode.
- Route physically present: PASS; every requested movement already exists in the submitted fixed-scene video.
- Timing valid: PASS; all timed instructions fit within the inherited 5.000-second duration and match the motion ledger.
- No instruction/input contradiction: PASS; the prompt preserves geometry and changes only surface realism, daylight, furnishings and audio.

## Provider and price evidence

- Live endpoint resolution: PASS on 2026-08-26; endpoint is `Running`, resolves to `doubao-seedance-2-5-260628`, accepts video input and supports `video_editing` through `/v3/contents/generations`.
- Non-billable dry-run: PASS; `DRY_RUN_URL.json` records the exact endpoint, HTTPS reference-video URL, prompt and parameters. It did not create a network generation task and incurred no generation charge.
- Price or unavailable status: live account unit price is **CNY 0.03312 per 1,000 V2V1080 completion tokens**. Exact total before generation is `unavailable` because completion-token usage is not known in advance. The working estimate is **CNY 16.163388**, based on the prior same-endpoint five-second 1080p V2V usage of 488,025 completion tokens. Actual cost must be read from the completed provider task.
- Failure billing status or unavailable: `unavailable`; no verified provider evidence in this review establishes whether a failed task is billed.

## Approval gate

No Seedance task may be created until the user approves this exact fingerprint. Approval authorises exactly one task and one output; any prompt, input, role, parameter, model, price or task-count change invalidates this PASS.

The earlier local-path create attempt was rejected before task creation because the provider requires a Web URL for `reference_video`. It returned no task ID and no usage. The corrected HTTPS transport changes the reviewed package, so the earlier approval cannot authorise this fingerprint.

Exact approval phrase: **确认提交 Clip C1 单个 Seedance 2.5 任务，指纹 41ebb0a24f75d74ac815324717c8797ca76c959064b64d9e0e7eb13f4f12dd20，1080p、5秒、生成音频，预计 CNY 16.163388（实际费用以返回 tokens 为准）**
