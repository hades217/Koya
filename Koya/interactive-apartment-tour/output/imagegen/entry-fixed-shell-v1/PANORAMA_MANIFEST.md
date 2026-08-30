# Apartment 106 Entry panorama manifest

- Status: `rejected_not_for_tour`
- Evidence class: `concept_floorplan_grounded`
- Space: Apartment 106 Entry Hall
- Camera: fixed point immediately inside the main entry door, approximately 1.60 m eye height
- Initial viewer direction: inward along the hall
- Initial Three.js yaw: `0deg` (browser-verified)
- Survey claim: none; this is not a measured 360 capture

## Topology lock

The official Apartment 106 plan and drawing audit are authoritative. Facing inward:

- viewer-left contains full-height storage, then separate Laundry and Main Bath openings;
- viewer-right is an uninterrupted solid wall;
- the single main entry door is behind the camera;
- facing back toward the entry door, the handle is viewer-left and the hinges are viewer-right;
- no island or invented opening is present at the entry.

Source evidence:

- `../../../../production-assets/drawing-audit/apartment-106/pdf-recheck-20260829/apartment-106-page1.png`
- `../../../../production-assets/drawing-audit/apartment-106/TOPOLOGY_LOCK.md`
- `../../../../production-assets/drawing-audit/apartment-106/OPENING_LEDGER.md`

## Construction

The panorama is rendered from one deterministic ray-cast shell by
`scripts/render_entry_fixed_shell.py`. Directional image-model outputs provide
surface detail but do not define walls or openings. Adjacent directions use a
110-degree overlap and broad feather on the same shell. The equirectangular
seam is rotated onto the viewer-right solid wall.

Locked directional sources:

- `source-front-v1.png`
- `source-left-v2.png` — regenerated against the fixed shell to remove the duplicated Laundry/Bath geometry from the rejected panorama
- `source-right-v1.png`
- `source-back-v1.png`

Rejected predecessor:

- `../entry-panorama-v2/PANORAMA_MANIFEST.md`

## Published local assets

- `public/tour/panoramas/entry-fixed-shell-v3-4k.webp` — 4096x2048 bootstrap texture
- `public/tour/panoramas/entry-fixed-shell-v3-8k.webp` — 8192x4096 desktop texture

Automated panorama inspection on the 8K asset:

- dimensions: PASS, 8192x4096
- wrap-edge RMSE: `381.618 (0.00582312)`
- visual cardinal-direction QA: PASS for inward hall, wet-area side, solid-wall side, and main entry door
- interactive browser QA: PASS at 1440x900; initial inward yaw and actual 8192x4096 texture load verified
- user acceptance: rejected; left/right spatial continuity and translation behaviour were not acceptable
- integration status: removed from the Entry tour on 2026-08-31; the default Entry experience now uses the QA-passed vertical-scroll video
