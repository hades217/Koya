# Clip EF deterministic preview status

Status: `V5_STRUCTURAL_STORYBOARD_READY / USER_REVIEW_REQUIRED / LOCAL_VIDEO_PENDING`

## Current preview

- Duration: 8 seconds.
- Ratio: 16:9.
- Camera: one first-person camera, 1.59 m eye height, approximately 32-35 mm full-frame equivalent.
- Scene: one deterministic WebGL Apartment 106 public-space shell.
- Route: Dining/Living hub -> true MPR lower-right opening glance -> one clockwise public-zone panorama -> east terrace threshold -> Terrace.
- Lighting mode: continuous daytime.
- Audio: none at structural-reference stage.

## Verification performed

- HyperFrames 0.8.17 check at 14 explicit route checkpoints: passed with 0 lint errors, 0 warnings, 0 runtime or layout findings.
- Runtime: 0 errors.
- Layout: 0 issues.
- Motion: 0 errors or warnings.
- Fourteen structural checkpoints are collected in `../living-panorama-storyboard-v1/structural-storyboard-v5/`.
- The MPR opening now has explicit white-box jamb/head/threshold cues so the true plan opening cannot read as a solid wall.

## Remaining gate

Both rendered storyboard V1 and V2 are rejected. V2 also skipped the true MPR opening and did not preserve a single physical camera route. Structural storyboard V5 is the only current route candidate and awaits user review before any render-level anchors are rebuilt. No local V5 MP4 and no paid Seedance submission are authorised.
