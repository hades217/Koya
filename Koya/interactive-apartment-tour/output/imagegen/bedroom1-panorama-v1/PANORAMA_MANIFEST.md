# Bedroom 1 panorama manifest

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Camera: Bedroom 1, normal eye height, equirectangular 360 panorama
- Topology authority: `public/tour/apartment-106-plan.png`
- Identity anchors: `bedroom1.png`, `bedroom1-threshold.png`, `wir.png`, `ensuite.png`
- Required circulation: public/kitchen side entry -> Bedroom 1 -> WIR -> ensuite
- Model output master: `bedroom1-panorama-master-v3.png`
- Delivery assets: `bedroom1-panorama-4k.webp`, `bedroom1-panorama-8k.webp`
- Model operations: panorama generation; topology correction; centre-seam repair
- Super-resolution: Real-ESRGAN x4plus, then exact 2:1 delivery resize
- Rejected candidates: initial direct ensuite doorway; second correction retaining that doorway
- Seam QA: 16-pixel wrap RMSE improved from `0.129650` to `0.038146`
- Disclosure: AI-generated concept panorama; not a measured survey or representation of a completed apartment
