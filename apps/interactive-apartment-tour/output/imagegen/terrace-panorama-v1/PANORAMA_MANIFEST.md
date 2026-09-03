# Private Terrace panorama manifest

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Topology authority: `public/tour/apartment-106-plan.png`
- Identity anchors: `terrace-threshold.png`, `terrace.png`, `living.png`
- Required geometry: wraparound terrace, grouped glazing, real Living opening, balustrade and planter edge
- Model output master: `terrace-panorama-master-v3.png`
- Delivery assets: `terrace-panorama-4k.webp`, `terrace-panorama-8k.webp`
- Model operations: panorama generation and centre-seam repair
- Super-resolution: Real-ESRGAN x4plus, then exact 2:1 delivery resize
- Seam QA: 16-pixel wrap RMSE `0.067647`; visual wrap inspection passed at the planter/balustrade seam
- Disclosure: AI-generated concept panorama and illustrative outlook; not a measured survey or verified view
