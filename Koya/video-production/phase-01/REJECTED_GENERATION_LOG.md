# Rejected generation log

## `cgt-20260815194403-tkx4f`

- Model: `doubao-seedance-2-5-260628`
- Generated: 2026-08-15
- Output: 10 seconds, 1010×912, 24fps, native AAC stereo audio
- Rejection reasons:
  - output is not 16:9;
  - the source first frame was near-square and forced the wrong aspect ratio;
  - the film jumps from the exterior to the elevator area instead of visibly crossing the entrance and foyer;
  - the ten-second prompt removed required spatial evidence;
  - it must not be described or reused as a literal one-take.
- Disposition: failed MP4, contact sheets, obsolete prompts, compressed test inputs and obsolete shot recipe moved to macOS Trash on user authorization.

The remote task history may remain in the provider account, but its output is forbidden as a future generation input.

## Rejected exterior anchor — wrong building massing

- Asset: `production-assets/references/rejected/wrong-exterior-storey-count-v1/`
- Reason: the AI-created wide exterior redrew Koya as a visually lower building and did not preserve the official five-level massing.
- Action: removed from every active input path before a Seedance task was created. The official `OFF-EXT-001` render is now the exterior architecture anchor.
- Billing/task state: the interrupted submission created no new task; the account task total remained 27.

## `cgt-20260815215827-bkkrq`

- Model: `doubao-seedance-2-5-260628`
- Requested and delivered technical format: 16:9, 1280×720, 10 seconds, native AAC audio
- Official exterior safeguard: the active exterior references were exact crops of `OFF-EXT-001`; the rejected AI low-rise anchor was not submitted
- Rejection reasons: no wide five-level establishing view; woman begins facing the camera, then changes through a ghosted dissolve into a rear-facing figure entering the building; physical threshold continuity fails
- Disposition: retained in `video-production/phase-01/rejected/` for user review and audit only; forbidden as an extension input
- Full QA: `qa/QA_CGT_20260815215827_BKKRQ.md`
