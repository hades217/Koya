# Koya Seedance 2.5 V3 QA report

## Disposition

`FAILED_CONTINUITY_GATE`

The Seedance provider task succeeded technically, but this output must not be described or shipped as a literal one-take property walkthrough.

## Provider and file verification

- Task ID: `cgt-20260817174649-k6jt7`
- Model: `doubao-seedance-2-5-260628`
- Provider status: `succeeded`
- Video: H.264, 1280×720, 24 fps
- Duration: 30.048 seconds
- Audio: AAC, stereo, 32 kHz
- Actual usage: 1,296,900 completion tokens
- Actual charge at CNY 0.042 per 1,000 tokens: CNY 54.4698
- Local file: `outputs/cgt-20260817174649-k6jt7.mp4`

## What passed

- The result contains real continuous camera motion rather than a sequence of static five-second image zooms.
- The official Koya facade is reproduced closely once the camera reaches it; the principal floor count, curved screening, balconies, central entrance, right driveway, and roof planting remain recognizable.
- One sportswoman remains visually consistent for most of the route: sage top, black leggings, ponytail, white shoes, and bottle in her left hand.
- Daylight and neutral warm interior color remain broadly consistent.
- A same-cabin elevator entry, closed-door interval, and same-door exit are visually represented.
- Native audio is present through the 30-second file.

## Hard failures

1. **A visible dissolve occurs around 27.6–28.0 seconds.** The woman and first living-room geometry become translucent and overlap a second living-room composition. Evidence: `qa/crossfade-27.75s.jpg` and `qa/thresholds/26s-30s-8fps.jpg`. This alone fails the zero-cuts / zero-crossfade requirement.
2. **Apartment 106 entry is skipped.** After leaving the lift, the woman turns through the corridor and the camera arrives in the kitchen without a visible apartment-door opening or a physically demonstrated carpet–metal–timber threshold. Evidence: `qa/thresholds/18s-23s-8fps.jpg`.
3. **The requested timing is not followed.** At approximately 20.5 seconds the camera is still crossing the elevator threshold, although the approved route required elevator exit by 17.5 seconds and apartment entry by 19.5 seconds. Evidence: `qa/no-apartment-threshold-20.5s.jpg`.
4. **Koya is not visibly persistent from the first frame.** The opening aerial does not keep the target building identifiable; a dark unrelated building crosses the flight path before Koya is acquired. Evidence: `qa/thresholds/2.25s-4.25s-8fps.jpg`.
5. **The final reveal changes spatial design.** The last seconds settle on a different room arrangement after the dissolve, so the kitchen–dining–living path is not one persistent 3D apartment.
6. **The ground-floor lift entry changes camera position before the crossing is complete.** Between approximately 11.5 and 12.0 seconds, the camera appears inside/alongside the cabin instead of remaining behind the resident and physically following her over the threshold. The following frames then replay the approach into the cabin. Evidence: `qa/10s-14s-labeled-montage.jpg`.
7. **The lift threshold movement loops between approximately 17 and 20 seconds.** The resident repeatedly approaches/crosses the same doorway over several seconds, creating the visual impression of entering or exiting the lift twice rather than completing one directional exit. Evidence: `qa/14s-22s-labeled-montage.jpg`.
8. **At 21–22 seconds the resident passes through architecture instead of opening Apartment 106.** There is no apartment door leaf, handle interaction, lock action, or corridor-to-apartment floor threshold; the corridor wall/opening becomes the kitchen. This is an architecture morph/teleport, not a door crossing. Evidence: `qa/14s-22s-labeled-montage.jpg`.
9. **The resident pops into existence between 4.75 and 5.00 seconds.** At 4.75 seconds the street is empty; at 5.00 seconds the full-size resident is suddenly standing in the road with no approach motion or occlusion that could physically reveal her. She should already be visible as a small, distant figure when the street-level facade is acquired, then grow naturally through camera approach and her own continuous walk. Evidence: `qa/3.5s-5.5s-labeled-montage.jpg`.

## User playback findings confirmed

The user's normal-speed playback identified the repeated lift action around 17–20 seconds and the wall entry around 21 seconds. Frame-by-frame inspection confirms both observations and also shows the earlier camera-position substitution during the ground-floor lift entry.

## Automated checks

- No frame exceeded the high scene-change threshold of 0.28, consistent with the model disguising the final edit as a dissolve rather than a hard cut.
- Lower-threshold scene changes cluster around 3.04, 4.75–5.00, 8.00, and 11.00 seconds; visual inspection was used instead of treating these scores alone as proof.
- Audio has three short near-silent intervals between 23.95 and 25.44 seconds, each under 0.44 seconds; audio presence passes, but it cannot repair visual discontinuity.

## Reference-size note

The eight reference images are already normalized to 1280×720. Several PNG concept frames can be JPEG-compressed for a future submission to reduce transfer size, but the successful task used temporary HTTPS URLs and submitted in 26 seconds. Compression would improve transport, not fix the continuity failures above.

## Spending gate

No second paid generation is authorized or submitted. Any revision requires a new reviewed payload, reference plan, and cost approval.
