# Kitchen & Dining Panorama V1

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Unit: Apartment 106
- Room: Kitchen & Dining / Living hub
- Projection: 2:1 equirectangular
- Camera: one fixed conceptual camera centre between the single island and round dining table

## Geometry source

- `public/tour/apartment-106-plan.png` — Apartment 106 floor-plan topology
- Drawing audit status: `PASS_FOR_FIXED_SHELL`
- Required invariants: one linear kitchen run, exactly one island, Dining between MPR and Living, grouped full-height glazing on the Dining/Living exterior

## Visual references

- `public/tour/kitchen.png` — canonical kitchen/dining identity anchor
- `public/tour/hub-west.png` — west-facing continuity and dining relationship
- `public/tour/living.png` — Living and terrace-glazing continuity

## Image-model operations

1. Built-in image model generated the initial 1774 x 887 equirectangular panorama from the audited plan and three visual anchors.
2. A targeted edit was performed after rotating the wrap seam to the centre. The result was rotated back after repair.
3. An outer-edge edit candidate was rejected because it increased the wrap-edge mismatch.

## Delivery derivatives

- `public/tour/panoramas/kitchen-dining-panorama-4k.webp` — 4096 x 2048 mobile/fallback
- `public/tour/panoramas/kitchen-dining-panorama-8k.webp` — 8192 x 4096 capable desktop
- Enhancement: Real-ESRGAN x4 plus final Lanczos delivery resize

## Automated QA

- 2:1 projection: PASS
- 8K width: PASS
- Wrap-edge RMSE after centre-seam repair: `0.0335966`
- Browser/WebGL interaction QA: pending local viewer test
- Spatial/marketing acceptance: pending user review

This asset is illustrative concept imagery and must not be presented as an official render, measured 360° survey, or constructed-condition record.
