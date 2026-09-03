# Apartment 106 modular walkthrough — production checklist

Updated: 2026-08-26 (Australia/Brisbane)

Source of truth: `CURRENT_PRODUCTION_SOT.md`

Legend:

- `[x]` completed and locally verified
- `[ ]` not completed
- `REJECTED` after a checked item means the failure has been documented and excluded

## 0. Global controls

- [x] Apartment 106 selected as the working unit.
- [x] Official Apartment 106 floor plan saved locally.
- [x] Plan orientation locked: north up, east right.
- [x] Official renders classified as material/style references unless exact unit mapping is confirmed.
- [x] Modular hub-and-spoke production method selected.
- [x] Full-apartment one-take claim abandoned for this workflow.
- [x] First-person, no-person rule locked for modular room branches.
- [x] Neutral Brisbane daytime and 16:9 landscape locked.
- [x] Structural storyboard, detailed visual anchors and continuous reference video assigned separate roles.
- [x] Independently generated whole-room images forbidden as an ordered Seedance timeline.
- [x] Rejected-image folders forbidden as generation inputs.
- [x] Current production SOT saved.
- [x] Asset and cleanup index saved.

## 1. Asset cleanup

- [x] Four known-bad modular-hub images identified.
- [x] Bad images removed from the active modular-hub directory.
- [x] Bad images moved to the recoverable audit quarantine at `production-assets/references/rejected/modular-hub-v1/`.
- [x] Active reference manifest updated with the new rejected location.
- [x] Modular-hub QA report updated.
- [x] Asset Bible updated.
- [x] Old `continuity-endpoints/` marked historical and forbidden.
- [ ] Review other legacy folders before any future permanent deletion.
- [ ] Do not delete provider task records, cost records, prompts, manifests, QA reports or rejected MP4 evidence without a separate explicit decision.

## 2. Master branch status

### Clip A — Entry to Living hub

- [x] Video generated.
- [x] Earlier visual approval identified.
- [x] REJECTED BY LATER FLOOR-PLAN AUDIT: the current file is excluded from the final master.
- [ ] Rebuild from a new official-plan fixed scene only if a complete entry-to-hub opening is required.

### Clip B — Living hub to Bath/Laundry

- [x] REJECTED: previous generated versions exist but are not accepted.
- [x] REJECTED: wall-turn, spatial-morph, wrong-route and micro-movement failures documented.
- [ ] Build a new same-scene structural route only when Clip B becomes the active priority.
- [ ] Produce new continuous reference MP4.
- [ ] Run Seedance preflight on exactly one task/output.
- [ ] Obtain explicit user approval before submission.
- [ ] Submit, poll the same task ID and perform frame-by-frame QA.

### Clip C1 — Living hub to Bedroom 1

#### Route and topology

- [x] Route contract written.
- [x] Official floor-plan route overlay created.
- [x] Bedroom 1 bed direction locked: head north, foot south.
- [x] WIR locked west/left.
- [x] Terrace glazing locked east/right.
- [x] No WIR detour included in Clip C1.

#### Structural storyboard

- [x] Fixed-scene HyperFrames project exists.
- [x] Eight structural frames created.
- [x] Dense threshold/turn checkpoint frames created.
- [x] Five-second motion ledger created.
- [x] Full bed reveal target locked at approximately 3.40 seconds.
- [x] Right-wall/Terrace endpoint locked at approximately 4.95 seconds.

#### Detailed visual anchors

- [x] C0 Living hub origin created.
- [x] C3 physical doorway-crossing anchor created.
- [x] C5 full-bedroom reveal anchor created.
- [x] C7 right-wall/Terrace endpoint created.
- [x] Four-frame visual review sheet created.
- [x] Detailed-anchor manifest created.
- [ ] User approves or rejects C0.
- [ ] User approves or rejects C3.
- [ ] User approves or rejects C5.
- [ ] User approves or rejects C7.
- [ ] Any rejected anchor is corrected before motion work.

#### Continuous reference video

- [ ] Reconcile approved visual intent into the single fixed scene.
- [ ] Confirm no wall, door, WIR, bed or glazing displacement.
- [x] Render one continuous five-second 16:9 structural reference MP4.
- [x] Confirm real translation rather than static-image zoom.
- [ ] Review full-speed playback.
- [x] Review 2.00–3.40 seconds densely for doorway crossing and turn.
- [x] Verify the final right pan has sufficient amplitude.
- [x] Verify technical metadata: duration, resolution, frame rate and codec.
- [ ] User reviews the continuous reference MP4 at normal playback speed.
- [ ] Upgrade structural `preview_ready` status only after visual-anchor approval and final input review.

#### Seedance gate

- [x] Freeze exact prompt.
- [x] Freeze exact reference-video input.
- [x] Use zero independently generated whole-room image inputs for topology-sensitive V2V.
- [x] Resolve live endpoint/model and current provider parameters.
- [x] Obtain current provider unit price; exact pre-generation total recorded as `unavailable` and the estimate labelled separately.
- [x] Set task count to one and output count to one.
- [x] Run `seedance-preflight-review`.
- [x] Receive `PASS` and package fingerprint.
- [x] Show exact package, inputs, parameters and price to the user.
- [x] Receive explicit approval for that exact task.
- [x] Submit exactly once: `cgt-20260826201457-2wj85`.
- [x] Confirm task creation before polling or retrying.
- [x] Poll the same task ID until terminal state: `succeeded`.
- [x] Download and preserve the output and submission record.
- [x] Inspect final MP4 frame-by-frame; local continuity/topology QA PASS.
- [x] User approves Clip C1 before Clip C2 or another branch starts.

### Clip C2 — Bedroom 1 to WIR to Ensuite

- [x] Begin only after Clip C1 acceptance.
- [x] Route through Bedroom 1 -> WIR -> Ensuite; no alternate Ensuite access.
- [x] Build structural fixed-scene route.
- [x] Create separate detailed review anchors.
- [x] Render continuous reference MP4.
- [x] Run exact-package preflight and receive PASS: fingerprint `1c1b07e95fd00727742741800e1f8ac78c9dcfb413965e2cb93c4c7b1140cac9`.
- [x] Receive explicit approval for one task and one output.
- [x] Submit exactly once: `cgt-20260826212744-nn77h`.
- [x] Poll the same task to terminal state and download the one output.
- [x] Inspect all 113 decoded frames and technical/audio metadata.
- [x] REJECTED: frame zero does not match the accepted Clip C1 endpoint; Bedroom identity changes.
- [x] REJECTED: WIR appearance is under-resolved and the overall image remains game-render-like.
- [x] Preserve rejected output and task/cost evidence; exclude it from the master.
- [ ] Rebuild the deterministic C2 source from the exact accepted Clip C1 terminal image.
- [ ] Fully dress the WIR in that same scene and re-run local QA.
- [ ] Run a fresh preflight and obtain new explicit approval before any replacement task.

Current override after the documented V1/V2 history:

- [x] V3 fixed-scene route and exact package completed.
- [x] V3 task `cgt-20260827214141-8xgfg` passed complete frame QA.
- [x] User accepted the exact V3 output.
- [x] Accepted V3 isolated at `clip-c2-bedroom1-to-wir-ensuite/outputs/accepted/clip-c2-bedroom1-wir-ensuite-user-accepted.mp4`.

### Clip D — Living hub to Bedroom 2

- [x] Confirm the Bedroom 2 northwest doorway and route around the solid MPR volume.
- [ ] Build structural route and detailed anchors.
- [ ] Render continuous reference MP4.
- [ ] Complete preflight, explicit approval, one submission and final QA.

### Clip E — Living hub to MPR

- [x] Standalone Clip E was previously superseded by combined Clip EF.
- [x] REJECTED: combined Clip EF did not meet the rendered-storyboard/visual-reference requirement.
- [ ] Replan the public-zone glance from the approved rendered storyboard before another provider task.

### Clip F — Living hub to private Terrace

- [x] V10 V2V task `cgt-20260829175719-2hwhz` completed once with no retry.
- [x] Submission audit confirms one 3D `reference_video` and zero `reference_image` inputs.
- [x] REJECTED AFTER USER REVIEW: the result retains the reference 3D look and does not follow the approved rendered storyboard imagery.
- [x] Rejected output moved to `clip-ef-living-loop-to-terrace/outputs/rejected/clip-ef-v10-v2v-rejected-after-user-review.mp4`.
- [x] Rebuilt and submitted the exact EF V11 package from one approved continuous control video plus all ten approved rendered visual targets: task `cgt-20260829193219-mgnj9`, one output, no retry.
- [x] REJECTED AFTER FRAME QA: V11 improved the interior and crossed the correct threshold, but did not execute the required final Terrace-to-interior look-back.
- [ ] Any further EF correction requires a new reviewed package, new preflight PASS and new explicit paid-task approval.
- [ ] Run a new preflight and obtain new explicit approval before any paid replacement task.

## 3. Final edit

- [x] Currently accepted C1 and C2 V3 branches are isolated in their own `outputs/accepted/` directories.
- [x] Rejected Clip EF is excluded from the accepted-clips preview and active edit.
- [ ] Intentional editorial cuts return to the identical Living hub between branches.
- [ ] No AI dissolve is presented as physical continuity.
- [ ] One continuous music and room-tone bed joins the modular clips.
- [ ] Colour and exposure are matched across all accepted clips.
- [ ] Final film is labelled and described as a modular sales walkthrough, not a full-apartment one-take.
- [ ] Final 16:9 master receives full-playback and frame-by-frame QA.
- [ ] User approves the final assembled film.
