# Apartment 106 public-zone fixed scene V9 QA

Status: `REJECTED_BY_USER / STOPPED / DO NOT USE`

User decision: this V9 Three.js approach must not be continued or used. See `DO_NOT_USE.md`.

## Source separation

- V9 is a new plan-parameterized scene. The rejected V8/V15 source, camera keys and renders were not copied into it.
- V7 render-level storyboard remains human sales-coverage guidance only.
- No generated whole-room storyboard image is composited into this video or attached as a temporal input.

## Geometry lock

- Public-zone normalized envelope: 15.0 x 6.8 units, matching the traced drawing ratio of about 2.21:1.
- Living normalized envelope: 7.8 x 5.3 units, with furniture deliberately subordinate to the room.
- Compact MPR west of Dining with one broad east-facing public opening and no freestanding column.
- Four-seat Dining only.
- One north Kitchen line and exactly one island.
- One TV on the solid north internal wall.
- Continuous grouped south/east glazing.
- One physically empty east sliding bay between z 3.34 and z 5.76.
- Camera crosses x 15 at approximately z 4.11, inside that empty bay.

## Camera and sales coverage

| Time | Required view | Current V9 result |
|---:|---|---|
| 0.00 | MPR broad opening plus Dining | PASS for local preview |
| 0.80 | Dining to Kitchen and one island | PASS for local preview |
| 1.60 | broad Living and north TV wall | PASS for local preview |
| 2.40 | sofa and wrap glazing | PASS for local preview |
| 3.20–4.00 | align with same east opening | PASS for local preview |
| 4.80 | physical east-threshold crossing | PASS for local preview |
| 5.25 | brief Terrace depth confirmation | PASS for local preview; no prolonged railing hold |
| 5.75–6.25 | continuous return turn toward Living | PASS for local preview |
| 6.25–7.95 | stable look back into the same public zone | PASS for local preview; user composition review required |

## Automated checks

- HyperFrames 0.8.17 upgrade check: current.
- HyperFrames check: 0 lint errors, 0 warnings.
- Runtime: 0 errors, 0 warnings.
- Layout: 0 issues across nine requested samples.
- Whole-route dense review: 33 frames at 0.25-second spacing found no cut, dissolve, room replacement or wall crossing.
- Revised threshold/return review: `qa-dense-threshold-v2/`, 17 frames from 4.00 to 8.00 seconds at 0.25-second spacing plus two contact sheets.
- The crossing occurs through the same east opening. The return begins immediately after a short Terrace-depth read, and a buyer-readable public-zone endpoint is established by approximately 6.25 seconds.

## Gate

This is a local deterministic preview, not a photoreal final and not a Seedance package. It may proceed to a separate Seedance preflight only after the user approves the exact V9 preview. Any later geometry, camera or timing change requires this QA to be rerun.
