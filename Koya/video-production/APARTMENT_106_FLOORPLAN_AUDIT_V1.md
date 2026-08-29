# Apartment 106 Floor-plan Audit V1

Status: `ACTUAL_21S_START_AUDITED_CANDIDATES_READY_FOR_REVIEW`  
Primary spatial source: `../Koya marketing plan Apartment 106.pdf`  
Rendered source: `../production-assets/assets/official/floorplans/apartment-106.png`  
Audited sequence: `../production-assets/contact-sheets/apartment-106-full-sequence-v3-actual-endpoint-audited.jpg`

## Spatial truth hierarchy

1. The Type 106 floor plan controls adjacency, route, doors, fixed robe location, kitchen line, island, WIR, ensuite and terrace relationship.
2. Official Koya interior renders control material and visual language only.
3. Generated room images are Concept Design / Artist Impression candidates. Furniture, styling, view and un-dimensioned details remain illustrative.
4. If a generated image conflicts with the Type 106 plan, the generated image is rejected even when it looks more attractive.

## Audited route

```text
Entry
  -> hall-side laundry and main bathroom
  -> Bedroom 2 through its northwest door
  -> exit Bedroom 2 through the same door
  -> turn east into the kitchen circulation
  -> enter MPR through its northeast inward-hinged door
  -> exit MPR through the same door
  -> continue east through the kitchen
  -> enter Bedroom 1 through its southwest door
  -> move west through WIR into ensuite
  -> reverse through ensuite, WIR and Bedroom 1
  -> exit Bedroom 1 through the same southwest door
  -> clear the single island
  -> dining west / living east
  -> terrace to the south and east
```

## Floor-plan comparison matrix

| Sequence | Plan fact | Active visual proof | Audit |
| --- | --- | --- | --- |
| 21s continuity start | Preserved camera faces east at the hall/kitchen threshold; continuing to the wet rooms requires a physical yaw west, not a reset to the entry door | Actual 21.047s frame and K01 v2 | PASS continuity candidate; no camera teleport |
| Entry/hall | When looking west from the kitchen threshold, laundry/main bath resolve camera-right and Bedroom 2 is farther camera-left | K01 v2 | PASS topology; joinery design illustrative |
| Laundry/main bath | Laundry and bath are separate adjacent spaces | Bath/laundry v1 | PASS topology; exact fixtures and finishes illustrative |
| Wet area -> Bedroom 2 | Camera leaves the north-side wet rooms, yaws south and reaches Bedroom 2 through its single northwest door; entry door stays behind camera | K03 v3 | PASS topology and hand continuity |
| Bedroom 2 | Fixed robe occupies west wall; bed is east/centre; one northwest entry door | Bedroom 2 v1 and K05 v3 | PASS; robe camera-left, bed camera-right, same-door exit |
| Hall -> kitchen | Camera must regain circulation before reaching MPR | K06 v1 | PASS; Bedroom 2 remains visible behind-left |
| Kitchen | Linear kitchen is north; one island sits south of it | Galley v1 | PASS topology; cabinet details illustrative |
| MPR | Compact room south of kitchen; only entry is northeast and hinged inward | K07 v2, MPR v1, K09 v3 | PASS; same hinged door used for entry and exit |
| Bedroom 1 entry | Bedroom 1 lies east of kitchen and is entered at its southwest corner | K11 v1 and Bedroom 1 v1 | PASS |
| WIR/ensuite | WIR sits west of Bedroom 1; ensuite is west of WIR | WIR/ensuite v1 | PASS topology; joinery and fixtures illustrative |
| Suite return | Reverse ensuite -> WIR -> Bedroom 1 -> southwest door -> kitchen | K14 v2 | PASS; WIR origin, bed and kitchen exit readable |
| Dining/living | Dining is west of living; living occupies eastern public zone | Dining-turn v2, welcome v3, living-final v1 | PASS; camera composition shows dining right and living left |
| Invitation | Bottle left hand; right hand invites; apartment not mirrored | Welcome v3 | PASS |
| Terrace | Terrace wraps south and east of living/Bedroom 1 | Terrace v1 | PASS topology; landscaping, furniture and outlook illustrative |

## Rejected visual versions

- K01 v1: did not match the preserved 21.047-second endpoint; it reset the camera near the entry door and changed the corridor orientation.
- K01 turn draft 1 (`exec-eb8f8983-c33c-45db-9095-be96198fecce.png`): human performance was usable but the wet rooms were placed camera-left after the west-facing turn, opposite the Type 106 plan.
- K03 v2: pointed back toward the apartment entry and therefore did not prove the wet-area-to-Bedroom-2 movement.
- K03 v3 draft 1 (`exec-1cba170f-7747-4631-9a17-9bfbf9f4432c.png`): route was correct but the bottle moved to the anatomical right hand.
- K03 v1: bottle appeared in the anatomical right hand.
- K05 v1: Bedroom 2 robe and bed relationship was mirrored.
- K05 v2: room geometry corrected but bottle remained in the anatomical right hand.
- K07 v1: MPR entrance read as a bare opening/pocket-door condition rather than the plan's hinged door.
- K09 v1 and v2: doorway or hand continuity was not acceptable.
- K14 v1: did not reliably prove the WIR -> Bedroom 1 -> southwest door return direction.
- Welcome v1: bottle and invitation hands were reversed.

Rejected versions remain as audit history and must not be passed to image or video generation.

## Remaining hard gates

- K00 is now extracted from the preserved 21.047-second video. K01 v2 must be visually approved before it can become a Seedance reference.
- The current images prove topology, not exact built dimensions or developer-approved finishes.
- Before every Seedance leg, include only active audited frames and exclude all rejected versions.
- Each returned video leg must be checked for door multiplication, room mirroring, island duplication, hand swap, daylight drift and wall penetration.
- No paid Seedance submission until references, full prompt, parameters and price are shown for approval.
