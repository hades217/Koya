# Apartment 106 modular walkthrough — asset and cleanup index

Updated: 2026-08-26

## Current active material

### Official truth

- Floor plan PNG: `../../production-assets/assets/official/floorplans/apartment-106.png`
- Floor plan PDF: `../../production-assets/references/floorplans/Koya marketing plan Apartment 106.pdf`
- Official Living/Kitchen render: `../../production-assets/assets/official/interiors/koya-2br-living-kitchen.jpg`
- Official Bedroom render: `../../production-assets/assets/official/interiors/koya-2br-master-bedroom.jpg`
- Official Ensuite render: `../../production-assets/assets/official/interiors/koya-2br-ensuite.jpg`

Official renders supply appearance language only where their exact unit mapping is unconfirmed.

### Active Apartment 106 concept pack

- Root: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/`
- Active manifest: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/ACTIVE_REFERENCE_MANIFEST.md`
- Topology lock: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/TOPOLOGY_LOCK.md`
- QA report: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/QA_REPORT.md`

Only files explicitly listed in the active manifest may be selected for preparation.

### Clip C1 structural material

- Route contract: `clip-c-hub-to-bedroom1/ROUTE_CONTRACT.md`
- Route overlay: `clip-c-hub-to-bedroom1/route-audit-v1/CLIP_C_ROUTE_OVERLAY.png`
- Structural storyboard: `clip-c-hub-to-bedroom1/storyboard-v1/CLIP_C_STORYBOARD_V1.jpg`
- Eight full-resolution structural frames: `clip-c-hub-to-bedroom1/storyboard-v1/frames/`
- Dense checkpoint QA: `clip-c-hub-to-bedroom1/storyboard-v1/checkpoint-qa/`
- Fixed-scene project: `clip-c-hub-to-bedroom1/reference-video-v1-hyperframes/`

### Clip C1 detailed visual material

- Detailed contact sheet: `clip-c-hub-to-bedroom1/storyboard-v2-detail/CLIP_C_VISUAL_ANCHORS_V1.jpg`
- Detailed manifest: `clip-c-hub-to-bedroom1/storyboard-v2-detail/VISUAL_ANCHOR_MANIFEST.md`
- Full-resolution C0/C3/C5/C7 frames: `clip-c-hub-to-bedroom1/storyboard-v2-detail/frames/`

Status: pending user approval. These four images are Concept Design review anchors, not ordered Seedance keyframes.

## Accepted video

- Clip C1: `clip-c-hub-to-bedroom1/outputs/accepted/clip-c1-living-to-bedroom1-user-accepted.mp4`
- Clip C2 V3: `clip-c2-bedroom1-to-wir-ensuite/outputs/accepted/clip-c2-bedroom1-wir-ensuite-user-accepted.mp4`

Clip EF V10 V2V is **not accepted**. The preserved rejected copy is:

- `clip-ef-living-loop-to-terrace/outputs/rejected/clip-ef-v10-v2v-rejected-after-user-review.mp4`

Task `cgt-20260829175719-2hwhz` used one 3D `reference_video` and zero `reference_image` inputs. It must not be selected for a master edit or future reference package.

Clip EF V11 all-inputs task `cgt-20260829193219-mgnj9` is also **not accepted**. It used the rebuilt V11 continuous reference plus all ten accepted rendered images and improved the interior/threshold sequence, but never completed the final Terrace look-back. Its output and QA evidence remain under `clip-ef-living-loop-to-terrace/submission-v4-v11-all-inputs-preflight/` for audit only.

The historical Clip A file under `clip-a-entry-to-hub/outputs/accepted/` is no longer accepted for the final Apartment 106 edit. Its later floor-plan audit overrides the earlier visual approval; retain it only as historical evidence.

## Rejected assets moved out of the active pack

Location: `../../production-assets/references/rejected/modular-hub-v1/`

- `EH-ENTRY-START-01-v1-overlong-extra-doors.png` — overlong corridor and extra doors.
- `HUB-00-W-v1-invented-openings.png` — invented openings.
- `B2-THRESHOLD-01-v1-direct-dining-shortcut.png` — impossible shortcut through the MPR mass.
- `TER-THRESHOLD-01-v1-wrong-elevation.png` — incorrect high-elevation outlook.

These files were moved, not permanently deleted, so their failure evidence remains recoverable. The active modular-hub directory now contains no `rejected/` subdirectory.

## Historical paths forbidden for new inputs

- `../../production-assets/references/rejected/**`
- `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/**`
- `clip-b-hub-to-bath/outputs/rejected/**`
- `clip-b-hub-to-bath/rejected-spatial-storyboards/**`
- legacy `interior-*`, `seedance-2.5-submission-*` and old one-take folders unless the current SOT names an exact accepted file

Historical files are retained for task/cost/QA traceability. Do not delete provider task records, rejected MP4s with documented task IDs, manifests or QA reports without a separate explicit cleanup decision.

## Safe selection rule

For future generation work, choose inputs from the official folders and the active manifest only. Never search the whole Koya tree and pick files by recency, similar filename, visual attractiveness or directory order.
