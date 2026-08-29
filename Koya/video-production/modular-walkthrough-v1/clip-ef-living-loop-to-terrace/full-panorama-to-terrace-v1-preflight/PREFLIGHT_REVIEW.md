# Seedance preflight review — complete Clip EF

Decision: `BLOCKED`

Reviewed at: 2026-08-29 Australia/Brisbane.

No task submitted: yes.

Package fingerprint: unavailable because the mandatory continuous reference video is missing.

## Blockers

1. No approved continuous eight-second reference video exists from one fixed Apartment 106 scene.
2. The only locally available fixed-scene V9 reference was explicitly rejected by the user and is marked `DO NOT USE`.
3. The nine photorealistic V7 storyboard images are independently generated review anchors, not same-scene temporal frames. Seedance does not expose an ordered-keyframe role that guarantees a 1→9 path.
4. Uploading all nine images as ordinary `reference_image` inputs would contradict the architectural continuity rules and cannot receive PASS.
5. Without the continuous reference video there is no provider-readable URL or SHA-256 transport verification to lock.

## Prompt review

下一次提交的候选提示词为中文 `PROMPT_ZH_DRAFT.txt`。英文 `PROMPT_DRAFT.txt` 仅保留作历史对照，不作为实际提交文本。

- Complete MPR -> Dining -> Kitchen -> TV -> sofa/glazing -> threshold -> Terrace route: included.
- Timing totals exactly 8 seconds: PASS.
- Correct MPR scope and no entry: PASS.
- Exactly one island and internal TV wall: PASS.
- Continuous south/east glazing and same east opening: PASS.
- Physical threshold crossing and Terrace endpoint: PASS.
- No cut, morph, teleport, game camera or static push-in: included.
- Prompt alone cannot supply the missing continuous topology controller: BLOCKED.

## Inputs and roles

- Official floor plan and all nine rendered storyboard frames: present in the human review pack.
- Actual Seedance `reference_video`: missing.
- No unsupported provider role will be invented.

## Task and parameters

- Planned task/output count: 1 / 1.
- Planned duration: 8 seconds.
- Planned resolution: 1080p.
- Planned aspect ratio: inherit exact validated reference video.
- Planned generated audio: true.
- Planned watermark: false.
- Exact provider price: must be refreshed after the missing reference video is completed; currently unavailable for this unfinalized package.

## Gate

No paid task may be created. Complete and approve the continuous fixed-scene reference video first, then rerun the full Seedance preflight, generate a fingerprint and request explicit user approval for the exact package.
