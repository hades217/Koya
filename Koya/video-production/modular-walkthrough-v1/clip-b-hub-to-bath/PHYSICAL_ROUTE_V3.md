# Clip B V3 — Floor-plan-locked Physical Route

Status: five storyboard nodes generated and internally checked; awaiting user approval. No Seedance task submitted.

Geometry source: `Koya marketing plan Apartment 106.pdf`, Level 1. The plan is indicative; it is used only as the spatial source of truth for this concept walkthrough.

## Corrected topology

- North is the top of the plan; west is left.
- Living and Dining occupy the southeast/south public area.
- One island sits south of the linear kitchen.
- The usable kitchen passage runs east-west between the linear kitchen on the north and the single island on the south.
- At the west end of that passage, Laundry and main Bath are two adjacent rooms on the north side.
- When the camera stands at the west end of the kitchen passage and faces north, Laundry is the left/west doorway and main Bath is the right/east doorway.
- The north-south Entry Hall joins at the extreme west end, but the camera does not travel up that Hall to reach the main Bath.
- The main-Bath door is approached directly from the kitchen passage and crossed northward.

## Storyboard nodes

1. `B3-00-HUB-AXIS-W`: camera at the Dining/Living hub, reoriented west into the kitchen passage. Linear kitchen is camera-right/north; the single island is camera-left/south.
2. `B3-01-PASSAGE-MID-W`: camera physically inside the passage, moving west. The same kitchen remains right and the same island remains left; no room replacement.
3. `B3-02-WET-DOORS-N`: camera at the west end of the passage, now turned north. Laundry doorway is left; main-Bath doorway is right. No long corridor.
4. `B3-03-BATH-THRESHOLD-N`: camera aligned with the right-hand main-Bath doorway and beginning to cross it. Laundry remains outside to the left.
5. `B3-04-BATH-INSIDE-N`: camera fully inside the compact main Bath. Bathtub/shower is at the back, WC centre-right and one vanity on the right wall.

## Physical constraints

- Every consecutive node shares one visible fixed structure with the previous node.
- No wall, doorway, island, kitchen run or wet-area room may appear or disappear between nodes.
- No long hotel corridor, extra door, merged Bath/Laundry, duplicate island or invented room.
- Natural 32–35mm perspective at 1.58m eye height; no person or hands.
- Lighting may be plain and functional. Physical continuity takes priority over cinematic styling.

## Planned generation split

If all stills pass, video generation will use threshold-bounded pieces rather than another Hub-to-Bath interpolation:

- Piece B1: Node 1 -> Node 2.
- Piece B2: actual final frame of B1 -> Node 3.
- Piece B3: actual final frame of B2 -> Node 5, with Node 4 used as the local doorway QA reference.

## Mandatory pace correction

- Piece B1 covers about two metres and must occupy only 1.5–1.9 seconds in the final edit.
- The 5.056-second provider output is a source-generation duration, not an acceptable viewing duration.
- Piece B2 and B3 must calculate physical distance and final screen duration using `../WALKTHROUGH_SPEED_STANDARD.md` before any paid submission.
- Do not use a fixed five-second final duration per storyboard interval.

No paid video task may be submitted until these storyboard nodes are approved.
