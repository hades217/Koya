# Apartment 106 public-zone fixed-scene topology check V5

Status: `LOCAL_PREVIEW_ONLY`

Authority order: official Apartment 106 plan -> drawing-audit topology/opening ledgers -> Route Contract V5 -> accepted V7 storyboard -> current Three.js scene.

| Stable element | Official/audit requirement | V7 review target | Current fixed-scene implementation | Gate |
| --- | --- | --- | --- | --- |
| MPR | Compact, west of Dining | Broad retracted O08 opening; no small door or freestanding column | Compact fixed room; broad east opening with short south jamb and rounded upper return | PASS for local preview |
| Dining | Between MPR and Living | Compact square table, four seats | One table, four chairs | PASS |
| Kitchen | North of public room | One linear kitchen and exactly one island | One north cabinet line and one island | PASS |
| TV wall | Solid internal north wall below Bedroom 1 | One TV facing sofa; never on glazing | Solid wall, one TV and console | PASS |
| Living | Broad eastern public zone | Sofa faces TV; clear path north of sofa | Fixed sofa group and north-side camera path | PASS |
| South envelope | Grouped full-height glazing O04/O05 | Continuous framed glazing | Fixed grouped glazed system with structural frames | PASS |
| East envelope | Grouped full-height glazing O06 | One broad open sliding bay | Fixed north glazed leaf, broad empty bay, stacked leaf and track | PASS |
| Threshold | Interior oak -> track -> terrace tile | All three remain visible | Fixed materials and physical crossing | PASS |
| Terrace | Wraps south and east | Broad wrap terrace, not high-rise balcony | Fixed east/south tile zones and glass balustrade | PASS for topology; outlook illustrative |

## Camera collision gate

- Camera remains east of the Dining table during the opening glance.
- Camera travels through the clear band north of the sofa.
- Camera crosses the east opening between the same fixed jambs.
- No camera key crosses the island, Dining table, sofa, TV wall, glass panel or structural frame.
- Endpoint at 7.95 seconds looks back through the same east opening.

## Submission exclusion

The accepted V7 storyboard images are human visual targets only. They are not ordered keyframes and must not be attached as independent whole-room `reference_image` inputs to a topology-sensitive Seedance V2V task.
