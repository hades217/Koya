# Submission plan — Bedroom 2 same-door exit to kitchen axis

Status: `AUTHORIZED_BY_USER_READY_TO_SUBMIT_ONE_TASK`

## Route

Accepted 36-second master endpoint inside Bedroom 2 -> resident turns naturally inside the same room -> camera backs through the same northwest door -> resident crosses that threshold -> camera settles behind her in the compact hall -> eastward kitchen axis begins.

The MPR is deliberately excluded from this five-second segment.

## References

| Role | File | Constraint |
|---|---|---|
| first_frame | `references/first-actual-36s-end.png` | Exact accepted 36-second generated endpoint; non-negotiable geometry and character start. |
| last_frame | `references/last-hall-kitchen-axis-v1.png` | Actual-endpoint-derived, floor-plan-audited endpoint outside the same Bedroom 2 door, facing east toward the kitchen. |

## Generation

- Provider/model: Seedance 2.5 through the existing resolved Ark endpoint.
- One paid task only.
- Duration: 5 seconds.
- Ratio: 16:9.
- Resolution: 720p.
- Native synchronized audio: enabled.
- No accumulated 36-second video upload; continuity is constrained by the exact actual final frame and the newly corrected final frame.

## QA gate

Reject if the resident or camera teleports out of Bedroom 2, the door/bed/robe moves, the camera crosses her body, a dissolve replaces the architecture, the bottle changes hand, or MPR appears before the next segment.
