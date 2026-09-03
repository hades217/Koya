# Failure postmortem — Apartment 106 Clip B V7

## Verdict

The paid output `cgt-20260826090750-2fszv` is fully rejected. The failure was caused by an invalid input design and an internally contradictory prompt, not by an acceptable random variation.

## Exact input conflict

| Input | What it actually contained | Conflict |
|---|---|---|
| `00-reference-motion.mp4` | Simplified white-box aisle and a simplified wet-zone approach | It did not contain the final two-door Laundry/Bath composition or the approved photoreal interior shell. |
| `01-style-kitchen-start.jpg` | Independent photoreal kitchen view from a different camera composition | Island, dining area, opening and perspective did not match frame 0 of the reference video. |
| `02-style-threshold.jpg` | Independent frontal Laundry-left/Bath-right composition with island corner and plant | It introduced architecture absent from the reference video and acted like an unlabelled destination composition. |
| `03-style-main-bath.jpg` | Independent photoreal Bath endpoint | It was not rendered from the reference video's Bath geometry or endpoint camera. |

## Prompt errors

1. The prompt said the video was the topology source while also asking the model to match three structurally different whole-room images.
2. It asserted that the images were “material and photographic appearance references only”, but the API role was only generic `reference_image`; no materials-only control existed.
3. It described the images semantically as kitchen start, threshold and corrected Bath. This encouraged the model to use them as scene destinations even though they had no ordered timeline roles.
4. It tried to solve visual contradictions with negative text. Negative wording cannot remove doors, perspectives and room layouts already present in image conditioning.
5. The reference video was too abstract and incomplete for the requested photoreal architectural edit. Seedance had to invent the missing design rather than merely restyle fixed geometry.

## Visible consequence

The generated video begins by following the independent kitchen appearance, then reconstructs the wet-zone approach toward the threshold still, and finally converges toward the independent Bath still. The result looks smooth at individual frames but is not one persistent physical apartment.

## QA failure

The original QA relied on contact sheets and an FFmpeg scene-change threshold. Slow spatial morphing can remain below a scene-change score, so this method cannot prove architectural continuity. The PASS was false and has been withdrawn.

## Mandatory prevention

- No paid retry from this package.
- Build one complete Apartment 106 shell containing the kitchen, island, MPR wall, Laundry door, Bath door and Bath fixtures simultaneously.
- Derive every storyboard frame and the entire reference video from that one shell.
- Do not attach independently generated whole-room images to the first V2V submission.
- Validate persistent geometry by tracked landmark identity and optical/3D correspondence, not by scene-cut score alone.
