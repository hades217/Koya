# Clip C2 V2 final visual QA

## Decision

`PASS`

Provider success was not treated as acceptance. The downloaded MP4 was decoded into all 121 frames, reviewed in four dense contact sheets, checked in reverse playback, and scanned for discontinuities.

## Technical verification

- Task: `cgt-20260827112126-cvjvw`
- Output: `../outputs/cgt-20260827112126-cvjvw.mp4`
- SHA-256: `0e93618d4cf07991df22b69a7d846e5b7465bd3e51fda8cae271c2845e8dca88`
- Frame size: 1920×1080 (16:9)
- Frame rate: 24 fps
- Video frames: 121
- Duration: 5.056 seconds
- Video: HEVC Main 10, BT.709
- Audio: AAC stereo, 32 kHz
- Automated scene scan: no frame exceeded the 0.18 hard-scene-change threshold.

## Spatial and continuity review

- The initial pose remains at the Bedroom 1/WIR threshold and begins moving immediately.
- The camera physically crosses the Bedroom 1-to-WIR opening; the jamb and surrounding walls translate with parallax.
- The WIR stays compact. One fitted open wardrobe bank remains on camera-right.
- The camera does not turn to feature the shallow camera-left recess.
- The WIR-to-Ensuite doorway remains a real second threshold and is crossed without a wall wipe, dissolve, teleport or scene replacement.
- In the Ensuite, the camera turns only after clearing the doorway.
- The bathroom remains one continuous space and presents the double vanity, toilet and shower along the fixed run.
- The end settles on the shower end without black, snap-back or a new viewpoint.

## Image-quality review

- Neutral daytime and warm Koya-compatible finish language remain stable.
- No person, hands, operator reflection, text, logo or watermark appears.
- No duplicated room, second wardrobe bank, moving wall, moving fixture, visible morph, penetration or hard cut was found.
- The motion reads as one continuous first-person walk rather than a static-image push-in.

## Cost record

- Completion tokens: `488025`
- Locked V2V 1080p rate: `CNY 0.03312 / 1,000 completion tokens`
- Calculated task cost: `CNY 16.163388`

## Integration gate

The clip is visually accepted as a standalone C2 candidate. It has not been stitched into the master walkthrough. Master integration remains a separate action so the boundary with the preceding clip can be checked before any merge.
