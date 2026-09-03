# Apartment 106 master-suite plan scale audit

Status: local geometry correction source; no paid generation authorised

## Source and method

- Source: `Koya marketing plan Apartment 106.pdf`, one-page official marketing plan.
- The PDF's drawn 0–5 m graphic scale measures 285.03 PDF points, giving **57.006 PDF points per metre**.
- The official 3508 × 2481 PNG is a 2.0833 px/point render, giving **118.76 image pixels per metre**.
- Room faces were measured between the internal wall faces in the official drawing. The PDF itself states that plans and dimensions are indicative and measurements are approximate, so these are **plan-derived modelling dimensions**, not contractual construction dimensions.

## Plan-derived dimensions used by Clip C2

| Element | Drawing measurement | Model dimension | Confidence |
|---|---:|---:|---|
| Bedroom 1, east-west | about 363 px | 3.06 m | Medium-high |
| Bedroom 1, north-south | about 346 px | 2.91 m | Medium-high |
| WIR, east-west | about 241 px | 2.03 m | High |
| WIR, north-south overall | about 199 px | 1.68 m | High |
| WIR fitted wardrobe depth | about 74 px | 0.62 m | Medium |
| WIR clear aisle | about 114 px | 0.96 m | Medium |
| Bedroom 1/WIR clear opening | about 112 px | 0.94 m | Medium |
| Ensuite, east-west | about 387 px | 3.26 m | High |
| Ensuite, north-south | about 199 px | 1.68 m | High |
| WIR/Ensuite doorway | about 95 px | 0.80 m | Medium-high |

## Topology lock

- Bedroom 1 is east of WIR.
- WIR is a compact connector, not a separate room-scale dressing suite.
- WIR has one fitted wardrobe bank on its north wall and a plain south wall.
- Ensuite is west of WIR and shares the same approximate north-south depth as WIR.
- Ensuite north fixture order is shower, toilet, double vanity from west to east.
- The camera must remain in the clear aisle west of the Bedroom 1 bed, then cross the two real openings. It must not pass through the bed, wardrobe, walls or fixtures.

## Corrections from the rejected blockout

- Bedroom 1 east-west span reduced from 7.84 model units to 3.06 m.
- Bedroom 1 north-south span reduced from 4.92 model units to 2.91 m.
- WIR reduced from 2.20 m to 2.03 m east-west; its 1.68 m overall depth is retained.
- Ensuite north-south depth reduced from 3.44 model units to 1.68 m.
- Ensuite east-west width increased slightly from 2.92 to 3.26 m.
- Camera start moved from the bed footprint to the approximately 0.75 m-wide circulation aisle west of the bed.

Exact surveyed and construction-set dimensions remain unavailable in the supplied marketing floor plan.

## User-directed WIR recess concept

The user has confirmed that the WIR entrance-side south/camera-left wall must contain a real shallow recess rather than one uninterrupted plane. The supplied marketing plan does not publish a dimension for this feature. For local visual review only, the deterministic shell uses a provisional approximately 0.82 m-wide × 0.28 m-deep recess with a set-back rear plane and two return walls. This is not an official or contractual dimension and remains blocked from paid generation until visually approved.
