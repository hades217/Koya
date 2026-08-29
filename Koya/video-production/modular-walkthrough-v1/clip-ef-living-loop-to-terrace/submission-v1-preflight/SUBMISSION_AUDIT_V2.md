# Clip EF submitted-task audit V2

Decision: `REJECTED / HISTORICAL PREFLIGHT PASS WITHDRAWN / DO NOT REUSE`

Reviewed task: `cgt-20260828173707-5hxv6`  
No new task submitted during this audit: yes

## Exact submitted task

- Endpoint/model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Mode: video editing with one explicit `reference_video`.
- Task/output count: one / one.
- Submitted input: `inputs/00-reference-motion-v6.mp4`.
- Submitted input SHA-256: `ae03d71427b6ac15a174034d404f6b677abf5e13ea34d5cea7c92e209a495781`.
- Submitted prompt SHA-256: `57ae12958fcabe16e67fa98d4ff5b47391a1b04fdeedf39d7cd5e160274c3fc2`.
- Original package fingerprint: `068752e310f71025f99dba3e975663bc1201ea13b13f7726b4cc4b6497d87be9`.
- Provider status: succeeded.
- Output: `outputs/cgt-20260828173707-5hxv6.mp4`.
- Output SHA-256: `a0c8001d1d899cc25a78500df2c547804f001fcd6a8918929d7c0bc6df99237c`.
- Actual completion tokens: 779,625.
- Actual recorded cost: CNY 25.821180.

## What was not submitted

- `reference-video-v8-threejs` was not an input.
- No V8/V15 QA frame, preview capture, render or hosted derivative appears in the package, dry-run or create response.
- The only provider video input was the byte-locked V6 MP4 above.
- No active `arkcli +gen` create process was found during this audit.

## Withdrawal of the historical PASS

The `PASS` in `PREFLIGHT_REVIEW.md` is withdrawn for reuse. The package was technically fingerprinted correctly, but the semantic review was wrong.

1. The submitted V6 first checkpoint did not reproduce the approved MPR/Dining opening composition. It began on an island/TV-dominated view.
2. The V6 shell was structurally and visually under-detailed. The accepted four-seat Dining, complete Kitchen, correct TV composition, sofa relationship and buyer-readable Living width were not rebuilt into the submitted fixed scene.
3. The prompt required Seedance to preserve immutable geometry while also transforming an incomplete primitive blockout into the approved design. Negative wording could not supply the missing spatial/design evidence.
4. The prompt explicitly prohibited enlarging Living even though the submitted shell already encoded the narrow Living scale that later failed.
5. The final 0.85 seconds asked for Terrace travel, a turn back through the opening and a stable multi-room endpoint. That sequence was too compressed and was not delivered by the output.
6. The original review fingerprinted the exact bytes but failed the required checkpoint-to-storyboard comparison. Cryptographic integrity did not prove creative or spatial correctness.

## Output findings

- Technical output is valid: 8.064 seconds, 1920x1080, HEVC, 24 fps, AAC stereo.
- There is no obvious hard cut; basic interior-to-Terrace continuity survives.
- The output is nevertheless rejected because the opening MPR/Dining read is missing, the public-zone panorama is incomplete, Living is narrow, the TV treatment is wrong, and the final camera remains on the planter/fence instead of looking back through the same opening.
- Do not stitch this MP4 into the master and do not use its final frame as a continuation anchor.

## Permanent exclusion set

- `inputs/00-reference-motion-v6.mp4` as a future provider input.
- `outputs/cgt-20260828173707-5hxv6.mp4` as an accepted clip or extension source.
- Original `PREFLIGHT_REVIEW.md` as evidence that this package remains approved.
- `reference-video-v8-threejs/**` under its separate `DO_NOT_SUBMIT.md` decision.

## Current gate

No replacement Seedance submission package exists. A new task is `BLOCKED` until a new, user-approved deterministic reference video exists from an acceptable same-scene source. The independently generated V7 storyboard remains human visual review material only and is not an ordered provider timeline.
