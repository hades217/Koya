# Clip B V3 — Physical Storyboard QA

Status: `PASS_INTERNAL_TOPOLOGY_AWAITING_USER_APPROVAL`.

## Source of truth

- Geometry: `Koya marketing plan Apartment 106.pdf`, Level 1.
- Materials are illustrative concept design derived from the existing Koya asset language.
- The plan and generated concepts remain indicative, not construction or contractual documentation.

## Corrected route

The camera does not enter the north Entry Hall to reach the main Bath. It travels west through the passage between the linear kitchen and the single island. At the west end it turns right/north to the adjacent wet doors: Laundry left/west, main Bath right/east.

## Node checks

1. `B3-00-HUB-AXIS-W-v1.png`: one island left, one linear kitchen right, clear westbound passage, wet doors only on the right/north side near the far end, no centered end door.
2. `B3-01-PASSAGE-MID-W-v1.png`: same west-facing axis after a forward camera move; island and kitchen retain their sides and rigid geometry.
3. `B3-02-WET-DOORS-N-v1.png`: camera has reached the west end and turned north; Laundry is left, main Bath is right, both open from one compact junction.
4. `B3-03-BATH-THRESHOLD-N-v1.png`: camera is visibly crossing the right-hand Bath doorway; timber remains outside and tile begins at the threshold; Laundry stays outside at far left.
5. `B3-04-BATH-INSIDE-N-v1.png`: camera is fully inside the compact Bath; bathtub/shower remains at the rear, WC centre-right and one vanity on the right wall.

## Rejected frames

- `rejected/B3-00-v0-centered-end-door.png`: invented a door centred on the westbound optical axis.
- `rejected/B3-00-v0-wet-doors-wrong-left-wall.png`: placed wet-area doors on the south/left wall instead of north/right.

## Gate

- No V3 Seedance request has been submitted.
- Do not generate video until the user approves the five-node physical route.
- Paid generation must be threshold-bounded; never ask Seedance to invent the entire Hub-to-Bath path from only the first and last frames.
