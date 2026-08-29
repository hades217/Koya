# Reference policy v3

The old all-at-once set of seventeen independent images is retired. Each task
receives only the references needed for its physical space.

## Leg A candidate references

1. Concept world start with completed Koya already present at the target parcel:
   `production-assets/assets/generated/concept-spaces/location/BRISBANE-TOOWONG-KOYA-WORLD-START-V1.png`.
2. Official Koya/Toowong aerial: `production-assets/assets/official/location/koya-toowong-brisbane-aerial.webp`, geography authority only.
3. Official Koya facade: `production-assets/assets/official/exterior/koya-building-hero.jpg`, readable building-geometry authority.
4. Character rear action panel used only for the final resident acquisition:
   `production-assets/assets/generated/character/sports-female/master-panels-v2/jogging-rear.png`.

## Leg B candidate references

1. Actual approved Leg A MOV as `@视频1` and extension source.
2. Official facade.
3. Locked character `back-full.png` or `jogging-rear.png` — one view, not a grid.
4. `CON-FOY-001-threshold-day-v2.png`.
5. `CON-FOY-002-lift-call-day-v2.png`.

## Leg C candidate references

1. Actual approved Leg B MOV as `@视频1`.
2. `SB-S01-LIFT-CROSS-001.png`.
3. `SB-S01-CABIN-OPEN-001.png`.
4. `SB-S01-DOORS-CLOSED-001.png`.
5. `SB-S01-ARRIVE-001.png`.

## Leg D candidate references

1. Actual approved Leg C MOV as `@视频1`.
2. `SB-S01-EXIT-001.png`.
3. `SB-S01-CORRIDOR-001.png` and `SB-S01-DOOR-001.png`.
4. Official Apartment 106 floor plan.
5. Corrected compact apartment-threshold/hall concept frame after continuity QA.

## Leg E candidate references

1. Actual approved Leg D MOV as `@视频1`.
2. Official Apartment 106 floor plan.
3. Official two-bedroom living/kitchen render for material language only.
4. Corrected compact Unit 106 hall/living concept frame.

## Reference rules

- A prior leg is motion and handoff truth, not architecture truth.
- Official project/floor-plan sources remain higher authority than generated video.
- Use one character view appropriate to the shot, never a multi-panel character grid.
- Do not provide references for rooms that are not reached in the current leg.
- Do not generate a later leg until the actual prior video and seam report exist.
- Public/common-area concept art must remain labelled artist impression.

## Hard missing inputs before Leg A

- confirmation that the candidate world-start plate is authorised and its target
  axis matches the project;
- a true moving-camera 3D/whitebox route reference, not still-image scaling;
- user approval of the Leg A exact prompt, payload and price.
