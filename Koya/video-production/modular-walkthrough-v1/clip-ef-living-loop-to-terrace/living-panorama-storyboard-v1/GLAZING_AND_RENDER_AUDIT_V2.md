# Apartment 106 Living Panorama - Glazing and Render Audit V2

Status: `V1_RENDER_SET_REJECTED / REBUILD_FROM_PLAN_REQUIRED`

This audit is a planning gate only. It does not authorise image or paid video generation.

## Geometry source of truth

- Official source: `Koya marketing plan Apartment 106.pdf`
- Working crop: `floorplan-audit-v5/public-zone-crop.png`
- Plan north is up and east is right.
- Living occupies the southeast public zone.
- Terrace wraps the Living room along the south and east exterior edges.
- The south and east exterior edges are full-height glazed assemblies. The plan indicates a mix of sliding/openable leaves and fixed glazed leaves, interrupted only by the drawn columns, jambs and frames.
- The solid TV/display wall is the internal wall on the north side of Living, shared with Bedroom 1. It must never be moved onto the south or east terrace edges.
- The sofa is placed in the southeast Living zone and faces north toward that internal TV wall.
- Kitchen is north of Dining/Living and has one island only.
- MPR is west of Dining. Its public-facing volume remains solid except for the plan-indicated northeast entry.

## V1 render-set verdict

The entire nine-frame V1 render contact sheet is forbidden as a visual or video-generation input. Individual frames are not to be salvaged because the set does not preserve one stable apartment shell.

| Frame | Verdict | Principal failure |
| --- | --- | --- |
| 00 | reject | Entry/MPR relationship is ambiguous and does not establish the plan-locked public-space orientation. |
| 01 | reject | Living is reduced to a narrow corridor; south/east wrap glazing and the north TV wall are not represented correctly. |
| 02 | reject | Bedroom-like doorway and solid decorated wall are invented beside Living; terrace glazing topology drifts. |
| 03 | reject | Public space is stretched and the Living wall/glazing orientation is inconsistent with the plan. |
| 04 | reject | Frontal kitchen view is not a continuous continuation of the same camera path and hides the required plan relationships. |
| 05 | reject | Long decorated wall and console are placed on an exterior/glazed side instead of the internal north TV wall. |
| 06 | reject | Terrace threshold is attached to the wrong solid-wall arrangement; interior orientation no longer matches Apartment 106. |
| 07 | reject | Interior/terrace relationship inherits the same wrong wall orientation and cannot serve as a continuity bridge. |
| 08 | reject | Terrace-only endpoint may illustrate mood, but it does not prove the correct Apartment 106 threshold and therefore cannot be used for topology-sensitive generation. |

## V2 rebuild gate

Before generating photorealistic frames, a deterministic shell must visibly prove all of the following in one fixed coordinate system:

- [ ] South Living edge is glazed, not a decorated wall.
- [ ] East Living edge is glazed, not a decorated wall.
- [ ] The southeast terrace corner reads as one continuous wraparound exterior zone.
- [ ] The north internal Living wall holds the TV/display position.
- [ ] Sofa orientation faces north.
- [ ] Dining remains west/central, between MPR and Living.
- [ ] MPR remains compact and west of Dining.
- [ ] Kitchen remains north with exactly one island.
- [ ] Camera path crosses a real east or south sliding-door opening without clipping a frame, wall or furniture.
- [ ] Every storyboard frame comes from the same shell and the same continuous camera path.

No V2 render frame is accepted until all boxes pass against the official plan.
