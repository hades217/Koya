---
format: 1920x1080
duration: 5s
message: "A single physical camera move from the Apartment 106 living hub into the main Bath"
arc: east hub -> west kitchen aisle -> single right turn -> bath threshold -> compact bath
audience: internal-production
mode: autonomous
---

## Beat 1 - Establish the correct aisle

- time: 0.0-0.4s
- camera: B0 to B1, first-person height 1.58m
- proof: island is left/south; fixed kitchen is right/north
- forbidden: opening inside MPR wall, second island, cut

## Beat 2 - Travel west

- time: 0.4-1.5s
- camera: fast continuous westward translation along the north side of the island, with slight early steering toward the visible Bath door
- proof: opposing parallax between island and kitchen; MPR becomes the solid left wall
- forbidden: yaw toward a wall, corridor invention, room substitution

## Beat 3 - One right turn

- time: 1.5-2.6s
- camera: one smooth clockwise curved approach from west to north while advancing toward the Bath door
- proof: the same single doorway enters frame and moves to centre
- forbidden: duplicated door, dissolve, teleport, wall crossing

## Beat 4 - Cross the threshold

- time: 2.6-3.0s
- camera: physically passes between the same two fixed jambs
- proof: jambs move behind the camera through real occlusion
- forbidden: door identity change, jump inside, camera clipping

## Beat 5 - Finish inside compact Bath

- time: 3.0-5.0s
- camera: moves forward and eases slightly without stopping dead
- proof: compact vanity, basin and toilet remain fixed; Laundry is not merged into Bath
- forbidden: oversized showroom, new corridor, duplicated fixtures
