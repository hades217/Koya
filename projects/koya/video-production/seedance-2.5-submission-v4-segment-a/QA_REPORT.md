# Segment A creative QA

## Result

`FAILED_CAMERA_THRESHOLD_GATE`

The provider task completed technically, but it cannot be used as the first leg of the literal one-camera walkthrough.

## Passed

- The aerial movement reads as spatial flight rather than a static Ken Burns zoom.
- Brisbane river, suburb and CBD provide real parallax.
- The same Koya facade emerges on the target axis and grows continuously into the street view.
- Exterior storey count, balconies, battens, entry and right-side driveway remain broadly consistent with the official facade reference.
- The resident is already visible in the resolved street composition; there is no obvious mid-shot pop-in.
- Resident identity, sage top, black leggings, white shoes and left-hand metal bottle remain consistent.
- The glass entrance visibly opens and the route continues into the foyer.
- Lighting and white balance remain neutral Brisbane daytime.

## Failed

- At approximately 9.0 seconds the resident crosses into the lift while the camera remains outside the metal threshold.
- From approximately 9.4 seconds onward, the lift doors close in front of the camera from the foyer side.
- The 10-second endpoint is therefore outside the lift, not inside the same cabin with the resident.
- This breaks the required physical camera route and makes a seamless Level 1 continuation impossible without a visible cheat.

## Decision

- Do not submit Segment B from this endpoint.
- Do not hide the failure with a cut, crossfade, door wipe or wall transition.
- Any retry requires fresh user approval and must make the last 2.0 seconds an explicit camera-threshold action: camera crosses the metal sill behind the resident, rear door jambs pass behind the lens, then the same doors close while both are visibly inside the cabin.

## Evidence

- `qa/segment-a-contact-sheet-0.5s.jpg`
- `qa/frame-9.0s.jpg`
- `qa/frame-9.8s.jpg`

