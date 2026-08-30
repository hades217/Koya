# Multipurpose Room panorama manifest

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Topology authority: `public/tour/apartment-106-plan.png`
- Identity anchors: `mpr-threshold.png`, `mpr.png`, `hub-west.png`
- Required geometry: compact bounded MPR, curved enclosure, grouped exterior glazing and one large retracted Dining-side opening
- Model output master: `mpr-panorama-master-v3.png`
- Delivery assets: `mpr-panorama-4k.webp`, `mpr-panorama-8k.webp`
- Model operations: panorama generation and centre-seam repair
- Super-resolution: Real-ESRGAN x4plus, then exact 2:1 delivery resize
- Seam QA: 16-pixel wrap RMSE `0.077977`; visual wrap inspection passed at the glazing/curtain seam
- Disclosure: AI-generated concept panorama; not a measured survey or representation of a completed apartment
