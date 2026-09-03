# Clip C visual anchor manifest — v1

Status: **Concept Design / Artist Impression — pending user visual approval**

## Purpose

These four images define the intended materials, room identity, lighting and sales-readable compositions for the five-second Apartment 106 route from the living hub into Bedroom 1.

They complement, but do not replace, the eight-frame structural storyboard and continuous deterministic reference video. The structural storyboard/reference video remains the source of truth for camera motion, timing, geometry and physical continuity.

## Evidence hierarchy

1. `apartment-106.png` — official topology and adjacency truth.
2. `storyboard-v1/frames/C0...C7` — approved blockout camera poses and route.
3. Official Koya interior renders — materials and design language only; they do not override Apartment 106 geometry.
4. The images in this directory — generated visual-development anchors, labelled Concept Design / Artist Impression.

## Visual anchors

| Frame | Time | Purpose | Non-negotiable spatial read |
|---|---:|---|---|
| C0 | 0.00s | Living hub origin | One correct Bedroom 1 door at left; clear route; console wall at right. |
| C3 | 2.00s | Physical doorway crossing | Visible door-jamb parallax; camera crosses the opening; same bedroom beyond. |
| C5 | 3.40s | Full bedroom reveal | Full bed from foot; WIR at west/left; terrace glazing east/right. |
| C7 | 4.95s | Right-side endpoint | Bed retained at left; right wall, glazing and terrace become the sales-view focus. |

## Continuity lock

- 16:9 landscape.
- First-person camera; no person on screen.
- Eye height approximately 1.58 m.
- Rectilinear 30–32 mm bedroom view; no ultra-wide distortion.
- Neutral Brisbane daytime; no sunset, night or orange grade.
- Same bed, cream upholstery, dusty-mauve/brown throw, artwork, bedside tables, lamps, flooring, rug, curtains and dark glazing frames across C3, C5 and C7.
- No new doors, rooms, corridors, windows, duplicated WIR or relocated glazing.
- The Level 1 leafy outlook is illustrative and must not be represented as a verified view.

## Seedance safety note

Do **not** upload these four independently generated whole-room images together as an ordered Seedance keyframe sequence. They are review anchors only. A paid topology-sensitive generation must use the single continuous deterministic reference video from the approved fixed scene. Any future Seedance task still requires a fresh preflight PASS, exact-input fingerprint, current price check and explicit user approval.

## Files

- `CLIP_C_VISUAL_ANCHORS_V1.jpg` — four-frame review sheet.
- `frames/C0-00.00s-living-origin-concept-v1.png`
- `frames/C3-02.00s-threshold-entry-concept-v1.png`
- `frames/C5-03.40s-full-bed-reveal-concept-v1.png`
- `frames/C7-04.95s-right-side-endpoint-concept-v1.png`
