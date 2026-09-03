# Entry Hall panorama manifest

- Status: `local_qa_candidate`
- Evidence class: `concept_floorplan_grounded`
- Camera: immediately inside the apartment entry, normal eye height
- Topology authority: `public/tour/apartment-106-plan.png`
- Identity anchors: `entry.png`, `entry-turn.png`, `kitchen.png`
- Required circulation: apartment door -> compact western hall -> kitchen/public hub
- Model output master: `entry-panorama-master-v3.png`
- Delivery assets: `entry-panorama-4k.webp`, `entry-panorama-8k.webp`
- Model operations: panorama generation; centre-seam repair
- Super-resolution: Real-ESRGAN x4plus, then exact 2:1 delivery resize
- Seam QA: 16-pixel wrap RMSE `0.032255`
- Disclosure: AI-generated concept panorama; not a measured survey or representation of a completed apartment
