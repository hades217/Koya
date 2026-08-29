# Clip B V4 — Storyboard QA

Status: REJECTED after paid-generation QA. Do not reuse any V4 storyboard node as a video-generation input.

- Composite: `storyboards-v4/CLIP-B-CORRECTED-PHYSICAL-STORYBOARD-V4.png`
- The V4 board incorrectly represents the public-space approach as a narrow dead-end kitchen passage instead of preserving Apartment 106's open kitchen/hall geometry.
- Its generated concept nodes were treated as geometry truth even though they are not official Apartment 106 renders.
- The paid task received only the first and last concept frames, not the official floor plan as a geometry reference.
- Seedance consequently rebuilt doors, corridor and room proportions between endpoints; the output is not Apartment 106.
- V3 Node `B3-02-WET-DOORS-N` was moved to `storyboards-v3/rejected/`.
- Provider task `cgt-20260825153750-cjdhl` was moved to `outputs/rejected/clip-b-v3-b2-rejected-wall-turn.mp4`.

Rejected task: `cgt-20260825162151-twrrd`.
Rejected output: `outputs/rejected/clip-b-v4-single-5s-rejected-wrong-route.mp4`.
