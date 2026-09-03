# Missing storyboard shots audit

Status: `14 STRUCTURAL CHECKPOINTS + 14 RENDER-LEVEL REVIEW FRAMES COMPLETE / PHOTOREALISTIC REFERENCE VIDEO STILL BLOCKED`

Reviewed against:

- official Apartment 106 topology;
- `ROUTE_CONTRACT_V4_LIVING_PANORAMA.md`;
- the seven accepted visual anchors in `index.html`;
- the one-take threshold and turn coverage rules.

## Finding

The principal views and all required bridge positions are now covered by the V6 fixed-shell captures and the 14-frame rendered review board. See `structural-storyboard-v6/` and `rendered-storyboard-v6/FRAME_MANIFEST.md`.

| Missing ID | Target time | Required frame | Why it is mandatory |
|---|---:|---|---|
| EF-S00 | 0.00 s | Exact start pose, already facing east in the kitchen-side passage, with the real MPR doorway only at the far-left peripheral edge. | Prevents an unattractive static MPR opening and fixes the initial walking direction. |
| EF-S02 | 0.45 s | Gaze returning east after the brief left glance; the same door recedes at the left edge and the island route opens ahead. | Proves that the camera turned back instead of replacing the room. |
| EF-S03 | 0.80 s | Camera clearing the island's west end, with measurable side parallax and Dining/Living beginning to open. | Prevents collision with the island and a sudden jump into the hub. |
| EF-S04 | 1.35 s | Mid-walk entry into the Living/Dining hub; island behind/right, Dining ahead/left, Living ahead/right. | Bridges the kitchen approach to the hub panorama and fixes room scale. |
| EF-S07 | 3.15 s | Intermediate rotation between the west Dining/MPR-wall view and the north kitchen view; the same Dining table and island remain visible together. | Prevents a blank-wall frame, furniture teleport or different-room substitution during the turn. |
| EF-S09 | 4.20 s | Intermediate rotation from kitchen toward Living; island exits one edge as the same sofa group enters the other. | Proves a single fixed public room rather than a cut between two renders. |
| EF-S12 | 5.85 s | Close approach to the east terrace opening, before the threshold; both jamb and floor track are clearly visible. | Establishes the exact opening used and makes the crossing readable. |
| EF-S13 | 6.70 s | Camera straddles the track with strong jamb parallax. | Proves the camera is physically crossing rather than passing through glass or a wall. |
| EF-S14 | 7.05 s | Camera is fully outside while retaining part of Living behind. | Proves arrival from the interior rather than a terrace teleport. |

## Existing seven anchors and their role

| Existing anchor | Coverage | Limitation |
|---|---|---|
| `MPR-THRESHOLD-01-v1` | MPR door identity and compact room scale | Not the exact V4 start camera pose. |
| `EH-KITCHEN-TURN-01-v1` | Kitchen-side approach design | Does not prove the brief glance-return motion or island clearance. |
| `HUB-00-W-v2` | Dining plus solid MPR public wall | Principal view only; lacks entry and exit rotation bridges. |
| `HUB-00-N-v1` | Kitchen and single island | Principal view only; cannot prove continuity with HUB-W or HUB-E. |
| `HUB-00-E-v1` | Living volume and terrace glazing | Principal view only; does not show the later close door approach. |
| `TER-THRESHOLD-01-v2` | Terrace opening and track design | Does not by itself prove the camera crosses the track. |
| `TER-END-01-v1` | Final terrace composition | Does not prove arrival from the interior. |

## Required completed structural sequence

The completed fixed scene must provide at least these 17 checkpoints:

`0.00, 0.20, 0.45, 0.80, 1.35, 1.90, 2.35, 2.75, 3.15, 3.55, 4.20, 4.70, 5.40, 5.85, 6.70, 7.05, 8.00 seconds`.

During the MPR glance, the two panorama turns and the terrace crossing, final QA must sample more densely than this approval board.

## Gate decision

- Main sales-view composition: `COVERED`.
- Continuous-motion structural storyboard: `COVERED`.
- Fixed-scene deterministic photorealistic reference video: `NOT READY`.
- Paid Seedance submission: `BLOCKED`.

Do not solve the missing frames by independently generating nine more whole-room images. They must be captured from the same completed fixed scene and exact camera path.
