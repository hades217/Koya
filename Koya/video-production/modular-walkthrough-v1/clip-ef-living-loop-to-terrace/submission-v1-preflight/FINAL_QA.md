# Clip EF Seedance output QA

Decision: **REJECTED / DO NOT STITCH**

Task: `cgt-20260828173707-5hxv6`

## Technical result

- Provider status: succeeded.
- File: `outputs/cgt-20260828173707-5hxv6.mp4`.
- 8.064 seconds, 1920x1080, 16:9, HEVC, 24 fps.
- AAC stereo audio at 32 kHz.
- 193 decoded video frames inspected.
- Automated scene detection at threshold 0.12 found no cut candidate. This is supplementary only.

## What passed

- One apparent continuous camera take; no obvious hard cut or dissolve.
- Stable daytime exposure.
- The camera approaches and crosses the east glazing threshold without an obvious wall penetration.
- Basic interior-to-Terrace continuity survives.

## Rejection findings

1. **The opening public-zone sales read is missing.** From roughly 0.00-1.20 seconds the image is dominated by the island and TV wall. The required brief MPR read and compact Dining read are not delivered clearly.
2. **The panorama does not communicate the whole public space.** The Kitchen occupies too much of the early duration, then the move advances to sofa/glazing. A buyer cannot reliably understand MPR -> Dining -> Kitchen -> TV wall -> Living as one connected sequence.
3. **The endpoint contract fails.** After crossing to the Terrace, the camera remains aimed at the planter/fence. By 7.95 seconds the doorway is only peripheral on camera-right; the camera never completes the required look-back through the same opening to Living, Dining and Kitchen.
4. **The Terrace conclusion is weak for sales use.** Most of the final interval presents a close planter and dark fence rather than Terrace depth, the wraparound quality, outlook, or retained interior connection.
5. **The output is not suitable for stitching.** Its final visual direction and spatial information do not provide the approved handoff endpoint.
6. **The Living-room geometry and scale are wrong.** The generated room reads as a narrow strip between an oversized timber island/partition and the glazing. It does not preserve the plan's broad open Living/Dining zone or the approved storyboard's depth.
7. **The TV treatment is wrong.** Seedance invented a small floating black screen and undersized console on a generic blank end wall. It does not reproduce the approved internal north TV/display composition below Bedroom 1, facing the sofa.
8. **The approved decoration design was not carried into the provider input.** Rendered storyboard V6 already showed the intended compact four-seat Dining, one detailed island, full Kitchen joinery, north TV wall, sofa, continuous glazing and Terrace styling. Those independent images were correctly excluded as topology-sensitive provider inputs, but their design was never rebuilt inside the deterministic reference scene. Seedance therefore inferred and redesigned the decoration from a primitive blockout.
9. **The submitted MP4 does not reproduce the current route scene at its first checkpoint.** The current scene/snapshot starts by looking toward the MPR opening, while the submitted reference MP4 starts on the island/TV composition. The preflight locked and reviewed the MP4 bytes but failed to compare its checkpoint images against the current scene and approved rendered storyboard. This invalidates the earlier reference-video acceptance.

## Evidence

- `qa/output-native-sheets/sheet-01.jpg` through `sheet-09.jpg`: every native output frame in consecutive contact sheets.
- `qa/REFERENCE_CHECKPOINTS.jpg`: locked structural reference at ten route checkpoints.
- `qa/OUTPUT_CHECKPOINTS.jpg`: generated output at the same ten checkpoints.
- `qa/OUTPUT_FFPROBE.json`: technical probe.
- `qa/SCENE_DETECTION.log`: supplementary automated scene-cut test.

## Cost and retry gate

- Completion tokens: 779,625.
- Actual calculated cost: **CNY 25.821180**.
- No automatic retry is authorised.
- Any retry requires a new corrected reference, new preflight package and new explicit single-task approval. Reusing this output or only changing prompt text would not repair the missing endpoint motion.
- The corrected reference must be rendered from the exact current project, include the approved decoration and fixed Apartment 106 geometry in that same scene, and pass a checkpoint-by-checkpoint comparison against the official floor plan and rendered storyboard before another provider task is considered.
