# Apartment 106 — Modular Living-Hub Video Plan

> Historical planning and attempt log. The current operational source of truth is `modular-walkthrough-v1/CURRENT_PRODUCTION_SOT.md`. Its branch naming and active-input rules override this file.

Status: Clip A is user-approved. Clip B attempts remain unaccepted. Current work is Clip C1, Living hub to Bedroom 1: route and structural storyboard are locked, detailed visual anchors are pending user approval, and no Clip C1 Seedance task has been submitted. See the current SOT above for all operational decisions.

Active reference pack: `../production-assets/assets/generated/unit-concepts/106/modular-hub-v1/`

- 17 accepted 16:9 source frames.
- Three locked Living/Dining hub orientations.
- Forward-only threshold and destination frames for every branch.
- Four rejected hallucinated frames moved out of the active pack to `../production-assets/references/rejected/modular-hub-v1/`.
- The first-person modular plan does not require a character turnaround sheet; no person will appear in these branch clips.

## Production decision

The interior sales film will no longer pretend to be one uninterrupted take. It will be produced as a set of independently verifiable, forward-only walkthrough clips connected by one consistent Living-room hub.

This changes the risk model:

- A failed room path affects only one short branch.
- The camera never has to reverse out of a room, which was the main source of topology drift, dissolves, duplicated doors, and wall pass-throughs.
- The viewer always knows where they are because every private-room visit starts from the same Living-room orientation.
- Existing accepted room footage can be reused after a fresh source audit; it will not be regenerated merely to force a one-take claim.

## Master structure

### Clip A — Entry to Living hub

- Purpose: establish the apartment once.
- Route: apartment entrance -> hall -> open kitchen edge -> Dining -> Living.
- Target duration: 8-10 seconds.
- End frame: `HUB_00`, a locked 16:9 Living-room view using a natural 32-35 mm perspective.
- This is the only clip that must connect directly from the apartment entrance.
- Accepted output: `modular-walkthrough-v1/clip-a-entry-to-hub/outputs/accepted/clip-a-entry-to-hub-v1.mp4`.
- Provider task: `cgt-20260825124458-kzwf7`; actual cost CNY 9.1098.

### HUB_00 — reusable Living anchor

- Same camera position, height, focal length, furniture, daylight, exposure and exterior view every time.
- Dining remains visible enough to orient the buyer; kitchen edge remains readable; terrace glazing is visible on the exterior side.
- No people.
- This exact frame is the first-frame reference for every branch below.
- A 0.8-1.2 second motion plate can be used between branches: subtle camera breath only, no spatial movement.

### Clip B — Living hub to Bath / Laundry

- Route: Living -> Dining edge -> west side of kitchen -> hall wet-area door -> Bath/Laundry.
- Target duration: 5-6 seconds.
- End inside the wet area after one readable reveal.
- Do not generate the return trip.
- Rejected candidate: `modular-walkthrough-v1/clip-b-hub-to-bath/outputs/rejected/clip-b-hub-to-bath-rejected-v1.mp4`.
- Provider task: `cgt-20260825132439-gptgg`; actual cost CNY 4.5738.
- V2 rejected output: `modular-walkthrough-v1/clip-b-hub-to-bath/outputs/rejected/clip-b-hub-to-bath-rejected-v2-spatial-morph.mp4`.
- V2 provider task: `cgt-20260825140013-75kgl`; actual cost CNY 4.5738.
- V3 physical storyboard: `modular-walkthrough-v1/clip-b-hub-to-bath/storyboards-v3/CLIP-B-PHYSICAL-STORYBOARD-V3.png`; user approved.
- V3 Piece B1 candidate: `modular-walkthrough-v1/clip-b-hub-to-bath/outputs/candidates-v3-b1/cgt-20260825152023-v6tlx.mp4`.
- V3 Piece B1 provider task: `cgt-20260825152023-v6tlx`; actual cost CNY 4.5738; physical continuity passed but original pace rejected.
- V3 Piece B1 free pace correction: `modular-walkthrough-v1/clip-b-hub-to-bath/outputs/local-speed-tests/clip-b-v3-b1-speed-3x.mp4`; 1.750 seconds; awaiting user review.
- V3 Piece B2 rejected output: `modular-walkthrough-v1/clip-b-hub-to-bath/outputs/rejected/clip-b-v3-b2-rejected-wall-turn.mp4`; provider task `cgt-20260825153750-cjdhl`; actual cost CNY 4.5738; do not salvage.
- V4 corrected storyboard: `modular-walkthrough-v1/clip-b-hub-to-bath/storyboards-v4/CLIP-B-CORRECTED-PHYSICAL-STORYBOARD-V4.png`; no V4 Seedance task submitted.

### Clip C1 — Living hub to Bedroom 1

- Route: Living -> the fixed Bedroom 1 doorway -> Bedroom 1.
- Target duration: 5 seconds.
- End after the full bed reveal and decisive right pan to the right wall and Terrace glazing.
- Do not generate the return trip.

### Clip C2 — Bedroom 1 to WIR / Ensuite

- Route: Bedroom 1 -> WIR -> Ensuite.
- Build only after Clip C1 is accepted.
- This is the only valid route to the Ensuite under the Apartment 106 plan.
- End in the Ensuite; do not generate the return trip.

### Clip D — Living hub to Bedroom 2

- Route: Living -> Dining -> west around the MPR edge -> Bedroom 2 northwest doorway -> Bedroom 2.
- Target duration: 5-6 seconds.
- End with the bed, robe and exterior light readable in one stable composition.
- Do not generate the return trip.

### Clip E — Living hub to MPR

- Route: Living -> Dining -> MPR doorway -> MPR.
- Target duration: 4-5 seconds.
- End after establishing that it is a compact multipurpose room, not a third large bedroom.
- Do not generate the return trip.

### Clip F — Living hub to Terrace

- Route: Living -> open sliding-glass threshold -> private Terrace.
- Target duration: 5-6 seconds.
- This is the final branch and the final film payoff.
- End outdoors with Living still visible through the opening so the interior/exterior relationship remains clear.

## Final edit logic

Recommended sequence:

1. Clip A: Entry -> Living hub.
2. Clip B: Hub -> Bath/Laundry.
3. Deterministic editorial cut back to `HUB_00`.
4. Clip C1: Hub -> Bedroom 1.
5. Clip C2: Bedroom 1 -> WIR -> Ensuite.
6. Cut back to `HUB_00`.
7. Clip D: Hub -> Bedroom 2.
8. Cut back to `HUB_00`.
9. Clip E: Hub -> MPR.
10. Cut back to `HUB_00`.
11. Clip F: Hub -> Terrace.

The cuts are intentional editing, not AI-generated dissolves. Use a 6-8-frame clean cut or very short dip-to-neutral only after the camera has stopped at each destination. Keep one continuous music and room-tone bed underneath so the finished film feels cohesive without falsely appearing to be one take.

## Duration target

- Generated motion: approximately 36-44 seconds before trimming.
- Final sales edit: approximately 32-38 seconds by shortening destination holds and using the shared hub plate only where orientation is needed.
- Do not force the route into 30 seconds if that makes door thresholds unreadable.

## Engineering assets

Each branch receives its own folder containing:

- `START_HUB_00.png` — identical across all branches.
- `END_<ROOM>.png` — floor-plan-audited destination frame.
- `PROMPT.txt` — only the forward route for that branch.
- `DRY_RUN.json` — exact Seedance parameters before paid submission.
- `SUBMISSION_RECORD.md` — task ID, actual price and provider status.
- `QA_REPORT.md` — topology, continuity, visual and technical inspection.
- `outputs/accepted/` and `outputs/rejected/` — never mix the two.

Before paid video work begins, the accepted source frames in `modular-hub-v1/` are copied into the relevant branch folder. Files under its `rejected/` directory and all previous `continuity-endpoints` remain forbidden upstream inputs.

## Acceptance gates per branch

1. Start frame matches `HUB_00` without furniture or lighting drift.
2. Camera crosses every required doorway physically.
3. Destination matches Apartment 106 topology and scale.
4. No people, extra rooms, duplicate islands, wall pass-throughs, dissolves or morphing.
5. Natural 28-35 mm property-film perspective; no ultra-wide showroom stretching.
6. Only an accepted branch can enter the final edit.

## Reuse rule

Existing videos are candidates, not automatically accepted. Audit and salvage only sections that already satisfy topology and visual-quality gates. A source clip that contains one bad transition may still contribute a clean room reveal after a deterministic edit, but it must never be represented as continuous motion through the faulty transition.
