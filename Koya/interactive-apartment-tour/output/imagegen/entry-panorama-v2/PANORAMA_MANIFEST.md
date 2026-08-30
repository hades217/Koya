# Entry Hall panorama V2 manifest

- Status: `rejected_not_for_tour`
- Evidence class: `concept_floorplan_grounded`
- Camera: immediately inside the Apartment 106 entry threshold, 1.6 m concept eye height, facing inward
- Topology authority: `Koya/production-assets/assets/official/floorplans/apartment-106.png`
- Drawing audit: `PASS_FOR_FIXED_SHELL`, FLP-106 SHA-256 `00b17651a4e1c757bc7cef4bc4426d9dd7b78ca7a781881589ca1a51b1aadecd`
- Geometry anchor: `Koya/production-assets/assets/generated/unit-concepts/106/modular-hub-v1/entry-hub/EH-ENTRY-START-01-v2.png`
- Material reference only: `Koya/production-assets/assets/official/interiors/koya-2br-living-kitchen.jpg`
- Projection: 2:1 equirectangular assembled from six square rectilinear cubefaces
- Required forward topology: storage, Laundry and Bath on viewer left; uninterrupted wall on viewer right; kitchen only at the distant end of the hall
- Required reverse topology: one Apartment 106 entrance door; inside-facing view shows hinges on viewer right and lever on viewer left
- Model operations: six continuity-controlled ImageGen cubefaces; one rejected left-face attempt was excluded because it merged Laundry and Bath
- Assembly: FFmpeg `v360`, `c6x1` cubemap to equirectangular, Lanczos interpolation
- Master: `entry-panorama-master-v2.png`, 4096 x 2048
- Delivery: `entry-panorama-v2-4k.webp`, 4096 x 2048; `entry-panorama-v2-8k.webp`, 8192 x 4096
- Known limitation: independently generated cubefaces are concept reconstruction, not a measured scan; seams and exact dimensions require interactive review
- Rejection reason: independent cubefaces did not preserve one shared shell across their edges; left/right continuity and doorway placement became inconsistent after spherical assembly
- Tour decision: removed from the Entry stop; do not publish or restore without a deterministic single-shell rebuild
- Disclosure: Concept Design / Artist Impression; furniture, finishes and fittings are illustrative
