# Apartment 106 — Living Panorama to Terrace

Status: `SALES VIEWS LOCKED / 14-FRAME V6 REVIEW BOARD COMPLETE / NOT A SEEDANCE INPUT PACKAGE`

This board describes one continuous first-person sales-viewing move. It is a visual and topology review document. The images are design anchors only; they must not be supplied to Seedance as an ordered keyframe timeline.

Continuity audit: see `MISSING_SHOTS_AUDIT.md`. Fourteen render-level review frames now cover the full route in `rendered-storyboard-v6/`. They validate intended views and threshold logic, but the independently generated renders are not proof of motion continuity; the eventual video still requires one deterministic fixed-scene reference render.

## Purpose

Let a buyer understand the public zone before entering the terrace:

1. briefly acknowledge the real MPR door from the kitchen-side circulation;
2. arrive at the Living/Dining hub;
3. look around the open-plan room while walking naturally;
4. show Dining, the solid public wall of the compact MPR, kitchen and one island, Living, glazing and terrace;
5. cross the real terrace threshold without cutting or passing through geometry.

## Eight-second camera plan

| Time | Camera and walking action | Required visible evidence | Forbidden result |
|---|---|---|---|
| 0.00–0.35 s | Start already walking in the kitchen-side passage, eye height about 1.65 m. Look slightly left for a fast peripheral acknowledgement. | The MPR's real northeast doorway appears only briefly. | No ugly static opening; no stop; no camera entering MPR. |
| 0.35–0.70 s | Return gaze forward/east and clear the island's west end. | Kitchen circulation remains readable. | No instant room swap; no wall crossing. |
| 0.70–1.90 s | Walk into the Living/Dining hub at a brisk human pace. | One island, dining area and open Living volume establish their real relationship. | No second island; no oversized MPR; no floating motion. |
| 1.90–2.75 s | Moving look west/northwest. | Dining in the foreground; the MPR's solid public wall beyond it; kitchen/island to the right. | Do not show an invented MPR door from this angle. |
| 2.75–3.70 s | Continue a smooth clockwise head turn north. | Full kitchen line and the single island. | No full-light showroom look; no geometry morph. |
| 3.70–5.00 s | Continue turning east/southeast while making a shallow arc through Living. | Sofa group, media/console zone, glazing and believable depth of the Living room. | No stationary 360 spin; no narrow corridor-like Living room. |
| 5.00–6.35 s | Align body and camera with the east terrace opening and walk toward it. | Door frame and track approach with clear parallax. | No direct fly-through glass; no dissolve. |
| 6.35–7.25 s | Physically step over the visible threshold track. | Interior remains behind the camera edge; terrace becomes foreground. | No clipping through frame, furniture or wall. |
| 7.25–8.00 s | Settle on the terrace with a small natural deceleration. | Terrace depth and connection back to Living remain readable. | No abrupt freeze or unrelated view. |

## Visual anchor order

These anchors define appearance and required visual facts, not temporal interpolation:

1. `MPR-THRESHOLD-01-v1` — source for the real MPR door and its compact scale only.
2. `EH-KITCHEN-TURN-01-v1` — source for the kitchen-side approach.
3. `HUB-00-W-v2` — source for Dining plus the solid MPR public wall.
4. `HUB-00-N-v1` — source for the kitchen line and one island.
5. `HUB-00-E-v1` — source for the Living volume, furniture and east glazing.
6. `TER-THRESHOLD-01-v2` — source for the physical door track.
7. `TER-END-01-v1` — source for the final terrace composition.

## Continuity locks

- Continuous daytime exposure and neutral-warm colour temperature.
- First-person camera; no visible guide character.
- Normal walking bob is subtle; no drone glide and no game-like sprint.
- Approximate 32–35 mm full-frame lens; no ultra-wide stretching.
- The MPR is only glanced at. It is not treated as a third bedroom or a destination.
- Apartment shell, openings, cabinetry, island, furniture and terrace geometry must stay fixed across every frame.
- The eventual Seedance topology reference must be a deterministic video rendered from one completed fixed scene.
