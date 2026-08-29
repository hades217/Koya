# Clip C2 deterministic reference geometry review

Status: **V6 LOCAL RENDER READY FOR USER REVIEW; not approved for paid Seedance submission**

Current live source: the fixed shell is retained, but the camera path is being rebuilt against the approved static design sequence. It now starts inside Bedroom 1 with the bed readable on camera-right, approaches the WIR from the foot-side circulation aisle, and then follows the same forward route through WIR and Ensuite.

Superseded artifact: `renders/koya-106-clip-c2-reference-v2-fast-walk-5s.mp4` — rejected locally because the vanity basins, mirrors and drawer fronts were occluded/under-resolved.

Rejected by user: `renders/koya-106-clip-c2-reference-v3-fast-walk-5s.mp4` — the WIR's left/plain-wall clearance existed geometrically but was not visually legible enough.

Superseded review artifact: `renders/koya-106-clip-c2-reference-v4-forward-with-left-recess-5s.mp4` — blocked because its opening composition did not agree with the approved Bedroom 1 design frame.

Rejected local candidate: `renders/koya-106-clip-c2-reference-v5-bedroom-wir-ensuite-5s.mp4` — the early view was too wall-dominant and made the approach to the real WIR opening harder to read.

Current review artifact: `renders/koya-106-clip-c2-reference-v6-bedroom-wir-ensuite-5s.mp4`.

## V6 reference changes

- Starts in Bedroom 1's foot-side circulation aisle, outside the bed footprint.
- Keeps the bed readable on camera-right while the only WIR opening remains ahead-left.
- Bends physically around the bed corner before crossing the Bedroom/WIR jambs.
- Preserves the one right-side wardrobe bank and the shallow left-side recess.
- Crosses the one Ensuite doorway before beginning the fixture inspection.
- Shows the double vanity, separate toilet and shower as one fixed north-wall fixture run.
- Ends on a diagonal shower view with explicit rain head, hand-shower/control cue and glass enclosure boundaries.
- Keeps the real WIR opening visible on camera-left during the early approach instead of aiming at the bedroom partition.
- Local HyperFrames check: 0 lint errors, 0 runtime errors, 0 layout issues and 0 motion errors.

Preview URL: `http://localhost:3041/#project/reference-video-v1-hyperframes`

## Technical result

- Timeline duration: 5.000 seconds.
- Resolution: 1920 x 1080.
- Frame rate: 30 fps.
- Codec: H.264.
- Audio: none; this structural reference controls geometry and camera motion only.
- Frames: 150.
- File size: 3,711,326 bytes.
- SHA-256: `9271a811c3cf593524e8ea0c2518e41cdaaffad408cb633493a305ea37dfa71e`.
- HyperFrames check: 0 lint errors, 0 runtime errors, 0 layout issues and 0 motion errors.

## Frame-level inspection

- All 150 V6 rendered frames inspected through six consecutive 25-frame contact sheets in `qa-reference-v6/contact-sheets-30fps/`.
- Automated scene detection at threshold 0.12 returned no cut candidates.
- The camera starts in the clear west-side aisle and does not cross the bed footprint.
- The 0.25–4.20 second travel uses a brisk first-person walking signature rather than a floating dolly: approximately 2.05 footsteps/second, 1.6 cm vertical head movement and 1.2 cm lateral weight transfer.
- The Bedroom 1/WIR opening exists from frame one and is approached rather than generated mid-shot.
- The camera crosses the Bedroom 1/WIR jambs physically.
- The WIR remains a 2.03 × 1.68 m connector with one north-wall wardrobe bank and a plain south wall.
- The user's required left-side recess is fixed geometry with a set-back rear plane and two return walls. Its provisional visual-review dimensions are approximately 0.82 m × 0.28 m because exact construction dimensions remain unavailable.
- The camera does not turn toward the recess or pause on it; the recess is visible only through natural peripheral edge depth and passing parallax while the forward sales route stays dominant.
- The camera crosses the only WIR/Ensuite doorway physically.
- The Ensuite remains one fixed room; no dissolve, morph, duplicate door or wall crossing occurs.
- The Ensuite is held to approximately 3.26 × 1.68 m. Because its real plan depth is compact, the camera inspects the north fixture run sequentially from double vanity through toilet to shower instead of falsely widening the room.
- The corrected current render exposes both vanity mirrors, two basins, taps and drawer fronts; the toilet remains separate and the shower glass/tray remain visible at the west end.
- Daylight, camera height, lens and exposure remain stable.

## Known limitation

This is a deliberately simplified topology reference, not a photorealistic finish render. The opening approach and final shower view are intentionally structural and sparse. Dimensions are scaled from the official marketing plan's graphic bar but remain indicative. Fixture details and finishes remain illustrative where the developer has not published exact specifications. The paid model must not be asked to alter the route or topology.

## Gate

The V6 local MP4 may be shown to the user. A paid Seedance task remains blocked until the user approves this exact rendered reference and a fresh `seedance-preflight-review` PASS locks the provider URL, prompt, endpoint/model, parameters, one-output task count and current price evidence.
