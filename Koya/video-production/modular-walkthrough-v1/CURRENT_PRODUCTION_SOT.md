# Apartment 106 modular walkthrough — current production source of truth

Updated: 2026-08-29 (Australia/Brisbane)

Status: **Only Clip C1 and Clip C2 V3 are user accepted. Clip A, Clip B and Clip EF are excluded; Clip D remains unproduced.**

This file is the operational source of truth for the current Apartment 106 interior sales walkthrough. Older one-take plans, continuity-endpoint experiments and rejected Seedance generations are historical evidence only.

Execution checklist: `PRODUCTION_CHECKLIST.md`

## Sales objective

Help a buyer understand the Apartment 106 layout through short, physically believable first-person room visits. The priority is spatial clarity and useful room coverage, not the claim that the whole edited film is one uninterrupted take.

## Production logic

Use a modular hub-and-spoke structure:

1. Establish the apartment once from the entrance to the Living hub.
2. Lock one Living hub identity and camera family.
3. Start each private-room branch from that hub.
4. Move forward through the real doorway and inspect the destination.
5. End inside the destination; do not generate a difficult reverse exit.
6. Return to the hub by an intentional editorial cut, not an AI-generated dissolve.

Every short branch must be internally continuous and obey physical-world movement. The assembled film is a modular walkthrough, not a literal single-take tour of the entire apartment.

## Authoritative route order and naming

| ID | Route | Current status | Next action |
|---|---|---|---|
| Clip A | Apartment entry -> kitchen edge -> Dining/Living hub | Earlier visual acceptance withdrawn after later floor-plan audit | Exclude `clip-a-entry-to-hub-v1.mp4`; rebuild from the official plan before any complete entry-to-hub master. |
| Clip B | Living hub -> Bath/Laundry | Previous attempts rejected or partial; not accepted | Rebuild only after a new fixed-scene route and preflight. |
| Clip C1 | Living hub -> Bedroom 1 | Accepted after task `cgt-20260826201457-2wj85` succeeded and passed frame-level QA | Preserve; use its final pose as Clip C2 continuity origin. |
| Clip C2 | Bedroom 1 -> WIR -> Ensuite | V3 task `cgt-20260827214141-8xgfg` USER_ACCEPTED after full-frame QA | Preserve V3; V1 and earlier rejected outputs remain excluded. |
| Clip D | Living hub -> Bedroom 2 | Planned | Still required only if Bedroom 2 is included in the final buyer tour. |
| Clip E | Living hub -> compact MPR | Standalone route superseded | Combined Clip EF gives the required brief MPR read without presenting it as a large bedroom. |
| Clip F | Living hub -> private Terrace | Not accepted | Replan together with the public-zone panorama only after its rendered storyboard and visual design are approved. |
| Clip EF | MPR/Dining glance -> Kitchen/TV/Living panorama -> same east threshold -> Terrace look-back | V10 V2V task `cgt-20260829175719-2hwhz` and V11 all-inputs task `cgt-20260829193219-mgnj9` both REJECTED | Exclude both from every accepted edit. V11 used the continuous reference plus all ten approved renders and fixed the interior appearance/crossing, but failed the required final Terrace-to-interior look-back. |

This table replaces the older naming in `MODULAR_LIVING_HUB_PRODUCTION_PLAN.md`, where Clip C was previously assigned to Bedroom 2. The existing directory `clip-c-hub-to-bedroom1/` is now authoritative.

## Evidence hierarchy

1. Official Apartment 106 floor plan controls topology, adjacency, door locations and room identity.
2. Fixed-scene structural frames control camera pose, movement, threshold crossing, turn direction and timing.
3. Official Koya renders control material and design language only unless the developer confirms the exact unit mapping.
4. Detailed generated visual anchors control the intended look and buyer-facing composition, but are Concept Design / Artist Impression.
5. Generated videos never override any upstream evidence.

## Three-layer image/video method

### Layer 1 — structural proof

Render dense checkpoint frames from one fixed shell. These frames may look plain; their job is to prove walls, doors, parallax, camera movement and the final viewing direction.

### Layer 2 — detailed visual anchors

Create only the informative review frames: origin, threshold crossing, principal room reveal and final inspection endpoint. Generate the destination reveal first, approve its identity, then use it to keep the other detailed anchors consistent.

Independently generated detailed anchors are not ordered keyframes and must not be uploaded together as a temporal Seedance sequence.

### Layer 3 — continuous reference video

Render one deterministic reference MP4 from the same complete fixed scene and camera path. This single video controls temporal geometry for topology-sensitive Seedance work. Detailed anchors are used for human review and for improving the local scene/prompt, not as conflicting whole-room inputs.

## Clip C1 current five-second design

Route: Living hub -> same visible Bedroom 1 doorway -> physically cross the doorway -> full bed reveal -> decisive right pan to right wall and Terrace glazing.

| Time | Required action |
|---:|---|
| 0.00–0.70s | Reorient from Living hub toward the already-visible Bedroom 1 door. |
| 0.70–2.00s | Approach the same door; jambs enlarge continuously. |
| 2.00–2.75s | Cross between the jambs; no wall penetration or room replacement. |
| 2.75–3.40s | Once inside, turn quickly right and reveal the complete bed. |
| 3.40–4.30s | Continue farther right; do not stop on the bed. |
| 4.30–5.00s | Hold the useful right-wall/Terrace inspection composition. |

Spatial lock:

- bed head north, foot south;
- WIR west/left;
- Terrace glazing east/right;
- no leftward WIR detour in Clip C1;
- no people;
- neutral Brisbane daytime;
- first-person eye height approximately 1.58 m;
- 30–32 mm rectilinear bedroom view;
- no extra door, corridor, glazing or oversized hotel-like room.

## Clip C1 files

### Structural/topology assets

- Route contract: `clip-c-hub-to-bedroom1/ROUTE_CONTRACT.md`
- Route overlay: `clip-c-hub-to-bedroom1/route-audit-v1/CLIP_C_ROUTE_OVERLAY.png`
- Eight-frame structural storyboard: `clip-c-hub-to-bedroom1/storyboard-v1/CLIP_C_STORYBOARD_V1.jpg`
- Structural manifest: `clip-c-hub-to-bedroom1/storyboard-v1/STORYBOARD_MANIFEST.md`
- Fixed-scene HyperFrames project: `clip-c-hub-to-bedroom1/reference-video-v1-hyperframes/`

### Detailed review assets

- Four-frame sheet: `clip-c-hub-to-bedroom1/storyboard-v2-detail/CLIP_C_VISUAL_ANCHORS_V1.jpg`
- Detailed manifest: `clip-c-hub-to-bedroom1/storyboard-v2-detail/VISUAL_ANCHOR_MANIFEST.md`
- Full-resolution frames: `clip-c-hub-to-bedroom1/storyboard-v2-detail/frames/`

The four detailed frames remain human-review appearance references and are not provider inputs. A five-second continuous structural reference MP4 exists at `clip-c-hub-to-bedroom1/reference-video-v1-hyperframes/renders/koya-106-clip-c1-reference-v1-5s.mp4` and passed local structural-motion QA. The corrected exact one-task package at `clip-c-hub-to-bedroom1/submission-v1-preflight/` used a verified byte-identical HTTPS delivery URL and passed preflight with fingerprint `41ebb0a24f75d74ac815324717c8797ca76c959064b64d9e0e7eb13f4f12dd20`. Task `cgt-20260826201457-2wj85` succeeded, was downloaded, passed local frame-level continuity/topology QA and was accepted by the user. Actual cost was CNY 16.163388.

## Active source assets

- Official floor plan: `../../production-assets/assets/official/floorplans/apartment-106.png`
- Official Living/Kitchen appearance reference: `../../production-assets/assets/official/interiors/koya-2br-living-kitchen.jpg`
- Official Bedroom appearance reference: `../../production-assets/assets/official/interiors/koya-2br-master-bedroom.jpg`
- Active Apartment 106 concepts: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/`
- Active reference manifest: `../../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/ACTIVE_REFERENCE_MANIFEST.md`

Only files listed by the active manifest may be considered for new preparation. Never discover inputs by selecting the newest filename automatically.

## Rejected and historical material

Forbidden upstream paths:

- `../../production-assets/references/rejected/**`
- `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/**`
- `../**/outputs/rejected/**`
- `../**/rejected-spatial-storyboards/**`
- old Seedance submission packages unless an exact accepted segment is named in this SOT

Four known-bad modular-hub images were removed from the active pack on 2026-08-26 and quarantined at `../../production-assets/references/rejected/modular-hub-v1/`. They were not permanently erased because their checksums and failure reasons are useful audit evidence.

## Clip C2 result and active next action

The exact one-output package fingerprinted as `1c1b07e95fd00727742741800e1f8ac78c9dcfb413965e2cb93c4c7b1140cac9` was submitted once as task `cgt-20260826212744-nn77h`. It returned one 4.736-second 1920x1080 output and consumed 471,825 completion tokens, for an actual cost of CNY 15.626844. Frame-level QA rejected the result because frame zero is not the accepted Clip C1 bedroom endpoint, the Bedroom 1 identity changes, the WIR is visually under-resolved and the clip retains a game-render appearance. The internal Bedroom -> WIR -> Ensuite movement is continuous and the final fixture order broadly survives, but the clip is forbidden from the accepted master.

The local deterministic Clip C2 shell has now been rescaled from the official PDF's 5 m graphic scale. Plan-derived modelling sizes are approximately Bedroom 1 3.06 × 2.91 m, WIR 2.03 × 1.68 m and Ensuite 3.26 × 1.68 m. The camera starts in the west-side bed aisle, crosses both real openings with a brisk first-person walking gait, briefly glances left to establish the WIR's plain-wall clearance, then inspects the compact Ensuite sequentially from vanity to toilet to shower. These measurements remain indicative, not contractual; see `clip-c2-bedroom1-to-wir-ensuite/route-audit-v1/PLAN_SCALE_DIMENSION_AUDIT.md`.

Before another paid task, visually approve this revised local route, rebuild its first frame to exactly match `clip-c-hub-to-bedroom1/submission-v1-preflight/qa-final-v1/end.png`, then re-run the complete local review, preflight, price read-back and explicit approval gate. No retry is currently authorised.

Floor-plan correction recorded on 2026-08-27: Apartment 106's WIR has one fixed wardrobe bank along its north/top wall. The south/bottom side is a plain wall. Any prior C2 reference, prompt or output describing opposing wardrobe banks is incorrect and forbidden as a new generation input.

## Acceptance gate for Clip C1

- Same doorway from Living through threshold.
- Visible jamb parallax and real forward translation.
- No cut, dissolve, teleport, wall crossing or static-image zoom.
- Bedroom scale agrees with Apartment 106.
- Full bed is readable by approximately 3.40 seconds.
- Right-side wall and Terrace glazing are clearly inspected by the endpoint.
- Detailed appearance remains consistent across the route.
- Final MP4 passes frame-by-frame QA and user approval.
