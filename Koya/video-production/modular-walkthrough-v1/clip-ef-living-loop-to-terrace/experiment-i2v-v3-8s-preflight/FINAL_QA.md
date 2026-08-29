# Clip EF I2V V3 final QA

Status: `REJECTED / DO NOT STITCH`

Task: `cgt-20260829114235-c9k4n`

Output: `outputs/cgt-20260829114235-c9k4n-1.mp4`

## Technical verification

- Duration: 8.064 seconds.
- Video: HEVC, 1918 x 1080, 24 fps.
- Ratio: 959:540, inherited from the 1672 x 941 first frame; effectively 16:9.
- Audio: AAC stereo, 32 kHz.
- File size: 27,850,347 bytes.
- SHA-256: `e487d0d851d66892d9b21d20c8afe2278369107771955176eb50291f896b9eaa`.
- Provider completion tokens: 390,417.

## Visual inspection

- Full clip inspected at 8 fps: 64 frames.
- Threshold interval 3.4–6.4 seconds inspected at 12 fps: 36 frames.
- Original-resolution checkpoints inspected at 0, 2, 4, 5, 6, 7, 7.5 and 7.95 seconds.
- No cut, dissolve, crossfade, black frame or frozen interval detected.
- The same sofa, glazing rhythm, folded/sliding frame, Terrace tiles, planting and daylight persist.
- Forward translation is visible through sofa exit, mullion growth and near-jamb occlusion; it is not a static crop zoom.
- At 5 seconds the camera remains inside; during 6–7 seconds the same near frame passes the camera; by 7.95 seconds the camera is on the Terrace tile.
- Final frame retains the same opening jamb at frame-left as the continuity cue.
- No person, invented room, MPR, Kitchen, logo or text appears.
- No obvious glass, wall or furniture penetration was found in the dense threshold sequence.

## Audio inspection

- Continuous audio stream present.
- Mean volume: -41.8 dB; maximum: -22.3 dB.
- No abrupt mute, black-frame-associated audio break or edit was detected.

## Billing evidence

- The successful task reports 390,417 completion tokens.
- Exact I2V CNY price remains `unavailable` from the account pricing response.
- Split-bill capability is not enabled for this account, so current CNY deduction cannot be read through `arkcli billing`.
- Provider billing is T+1; no precise instant CNY deduction is claimed.

## Decision

The output passes only the restricted Living-to-Terrace mechanics, but it fails the user's actual Clip EF brief because it omits the required Living panorama and MPR glance. It must not be described as the complete Clip EF route and must not be stitched into the master walkthrough.

User rejection reason: no panoramic survey of the Living room before moving to the Terrace.
