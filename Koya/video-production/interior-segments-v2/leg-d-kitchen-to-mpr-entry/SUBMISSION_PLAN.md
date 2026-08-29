# Submission plan — Kitchen axis to MPR entry

Status: `SUBMITTED_AND_REJECTED_REPLACEMENT_NOT_AUTHORIZED`

## Route

Accepted 41-second endpoint at the kitchen axis -> short eastward movement between the linear kitchen and the single island -> anatomical-right turn to the MPR northeast door -> same hinged door opens inward -> resident and camera physically cross the same threshold -> stop fully inside the compact MPR.

## References

| Role | File | Constraint |
|---|---|---|
| first_frame | `references/first-actual-41s-end.png` | Exact accepted 41-second generated endpoint; non-negotiable kitchen, character and lighting start. |
| last_frame | `references/last-mpr-inside-v1.png` | Actual-endpoint-derived, floor-plan-audited endpoint fully inside the compact MPR with the same open door retained for return. |

## Generation

- Seedance 2.5 through endpoint `ep-20260812221158-hb576`.
- One paid task only.
- Duration: 5 seconds.
- Ratio inherited from the 1280x720 first frame: 16:9.
- Resolution: 720p.
- Native synchronized audio: enabled.

## QA gate

Reject if the MPR appears by dissolve, the camera remains outside, the door changes type, the kitchen island duplicates, the MPR becomes oversized, the bottle changes hand, or the room is substituted for Bedroom 1.
