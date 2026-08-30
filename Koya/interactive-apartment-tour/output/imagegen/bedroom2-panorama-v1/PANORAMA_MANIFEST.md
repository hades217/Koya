# Bedroom 2 panorama manifest

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Topology authority: `public/tour/apartment-106-plan.png`
- Identity anchors: `bedroom2-threshold.png`, `bedroom2.png`, `entry.png`
- Required geometry: one public entry, one bed, robe and grouped exterior glazing
- Model output master: `bedroom2-panorama-master-v3.png`
- Delivery assets: `bedroom2-panorama-4k.webp`, `bedroom2-panorama-8k.webp`
- Model operations: panorama generation
- Super-resolution: Real-ESRGAN x4plus, then exact 2:1 delivery resize
- Seam QA: 16-pixel wrap RMSE `0.031495`
- Disclosure: AI-generated concept panorama; not a measured survey or representation of a completed apartment
