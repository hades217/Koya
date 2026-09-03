# Apartment 106 Modular Hub V1 - Active Reference Manifest

Status: production asset build; no video generation authorised by this file.

## Truth hierarchy

1. `FLP-106-001`: `assets/official/floorplans/apartment-106.png` and its source PDF. Geometry, adjacency, doors and fixed-room placement.
2. `OFF-INT-001`: `assets/official/interiors/koya-2br-living-kitchen.jpg`. Public-space material language only; no claim that the render is Apartment 106.
3. `OFF-INT-002`: `assets/official/interiors/koya-2br-kitchen.jpg`. Kitchen joinery and material language only.
4. `OFF-INT-003`: `assets/official/interiors/koya-2br-master-bedroom.jpg`. Bedroom material language only.
5. `OFF-INT-004`: `assets/official/interiors/koya-2br-ensuite.jpg`. Bathroom material language only.
6. Approved V1 concept views listed below. These may support continuity but never override the plan or official render. For Clip EF Living panorama work, all public-zone generated views remain suspended pending the V2 glazing/topology re-audit.

No previous `continuity-endpoints` image is an active upstream reference for this pack. They remain audit history only.

## Camera coordinate convention

For this production pack, plan top is north and plan right is east.

- Camera height: 1.55-1.65 m.
- Lens: 32-35 mm full-frame equivalent for public rooms; 28-32 mm only for compact bath/WIR spaces.
- Vertical lines remain level.
- Daylight: neutral Brisbane daytime, no sunset or night drift.
- No people, hands, bodies or reflected operators.

## HUB_00 orientation triad

All three images represent one locked camera pivot position near the Living/Dining boundary.

| ID | Orientation | Required spatial proof | Status |
| --- | --- | --- | --- |
| HUB-00-N | north / northwest | Dining foreground, single island and kitchen beyond, route back to hall readable | accepted: `hub/HUB-00-N-v1.png` |
| HUB-00-W | west / southwest | Dining centre and solid MPR wall readable without inventing doors | accepted: `hub/HUB-00-W-v2.png` |
| HUB-00-E | east / southeast | Living at truthful scale and terrace threshold on exterior edge | accepted: `hub/HUB-00-E-v1.png` |

## Branch resource matrix

Each branch uses three kinds of frame: hub orientation, threshold, destination. The master suite adds WIR because it is a real mandatory connector.

| Branch | Start | Threshold | Destination / detail | Status |
| --- | --- | --- | --- | --- |
| Entry -> Hub | `entry-hub/EH-ENTRY-START-01-v2.png` | `entry-hub/EH-KITCHEN-TURN-01-v1.png` | HUB-00-E | accepted reference sequence |
| Bath/Laundry | HUB-00-N | `bath-laundry/BL-THRESHOLD-01-v1.png` | `bath-laundry/BL-END-01-v1.png` | accepted reference sequence |
| Bedroom 2 | HUB-00-W | `bedroom2/B2-THRESHOLD-01-v2.png` | `bedroom2/B2-END-01-v1.png` | accepted reference sequence |
| MPR | HUB-00-W | `mpr/MPR-THRESHOLD-01-v1.png` | `mpr/MPR-END-01-v1.png` | accepted reference sequence |
| Bedroom 1 suite | HUB-00-N | `master-suite/MS-B1-THRESHOLD-01-v1.png` | `master-suite/MS-B1-END-01-v1.png` -> `master-suite/MS-WIR-CONNECTOR-01-v1.png` -> `master-suite/MS-ENS-END-01-v1.png` | accepted reference sequence |
| Terrace | HUB-00-E | `terrace/TER-THRESHOLD-01-v2.png` | `terrace/TER-END-01-v1.png` | accepted concept sequence; Level 1 leafy outlook remains illustrative |

## Review contact sheets

- `contact-sheets/01-entry-hub-triad-v1.jpg`
- `contact-sheets/02-west-branches-v1.jpg`
- `contact-sheets/03-master-terrace-v1.jpg`

## Rejected in this build

The complete Clip EF render-level V1 contact sheet and its nine crops are rejected and forbidden as generation inputs. See `video-production/modular-walkthrough-v1/clip-ef-living-loop-to-terrace/living-panorama-storyboard-v1/GLAZING_AND_RENDER_AUDIT_V2.md`. The set failed to preserve the official south/east wraparound glazing, north internal TV wall, Living orientation and one stable apartment shell.

The following images were removed from this active pack on 2026-08-26. They are retained under `production-assets/references/rejected/modular-hub-v1/` only as audit evidence and are forbidden as generation inputs:

- `references/rejected/modular-hub-v1/HUB-00-W-v1-invented-openings.png` — invented public-room openings.
- `references/rejected/modular-hub-v1/B2-THRESHOLD-01-v1-direct-dining-shortcut.png` — bypassed the solid MPR volume.
- `references/rejected/modular-hub-v1/TER-THRESHOLD-01-v1-wrong-elevation.png` — implied a high-rise outlook inconsistent with Level 1.
- `references/rejected/modular-hub-v1/EH-ENTRY-START-01-v1-overlong-extra-doors.png` — overlong corridor and doors on the solid west wall.

## Forbidden input paths

- `production-assets/references/rejected/**`
- `video-production/**/rejected/**`
- `video-production/interior-segments-v2/leg-e-final-pov-storyboard/rejected/**`
- all earlier generated frames containing a person
- `production-assets/references/rejected/modular-hub-v1/**`
- any image that stretches Living into a long showroom, adds a study zone, duplicates the island, mirrors Bedroom 2, or provides direct Ensuite access outside Bedroom 1/WIR

## Labelling

Every output in this directory is `Concept Design / Artist Impression`. Furniture, views, fittings and styling are illustrative.
