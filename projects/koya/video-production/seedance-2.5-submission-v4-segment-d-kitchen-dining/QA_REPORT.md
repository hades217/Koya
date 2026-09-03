# Segment D QA Report

- Task ID: `cgt-20260817223830-fnz5b`
- Status: `REJECTED_BY_USER_VISUAL_CONTINUITY`
- Output: `outputs/cgt-20260817223830-fnz5b.mp4`
- SHA-256: `85d7a738ef8d1154ee380b60b5a91df777a93c94650037f46b06f3a5ccc87667`
- Actual duration: 5.056 seconds
- Video: H.264, 1280x720, 24 fps, 16:9
- Audio: AAC, 32 kHz, stereo, continuous native ambience
- Usage: 108,900 completion tokens
- Actual cost: CNY 4.5738 at CNY 0.042 per 1,000 completion tokens

## Continuity inspection

- Reviewed full playback and a 4 fps contact sheet across the entire segment.
- Reviewed the island pass more densely at 8 fps from 0.75 to 3.50 seconds.
- No hard cut, dissolve, black frame, body wipe or static Ken Burns motion was found.
- The resident remains the same woman in the locked sage top, black leggings and white-grey runners.
- The stainless bottle remains in her left hand.
- The resident and camera move continuously around the same island; the island produces real foreground occlusion and parallax before moving naturally to the left of frame.
- The kitchen cabinetry remains spatially continuous while dining and living areas progressively emerge.
- No duplicate island, duplicate kitchen, inserted MPR, staircase or oversized double-height room was found.
- The first decoded frame matches the approved continuation frame closely: SSIM 0.972136.
- No audio silence interval of 0.4 seconds or longer below -45 dB was detected.

## Decision

`REJECTED`. The original automated contact-sheet review missed a material spatial failure. During the island occlusion, the resident and camera do not visibly clear the physical end of the island before the room orientation changes. The result reads as passing through the island / architecture morphing. The interior also has an overly clean game-render appearance rather than the requested cinematic live-action quality. This segment and its combined 26-second preview are forbidden as final delivery inputs.

## Combined preview

- File: `outputs/koya-continuous-preview-through-kitchen-dining-26s.mp4`
- Duration: 26.071 seconds
- Video: H.264, 1280x720, 24 fps
- Audio: AAC, 32 kHz, stereo
- SHA-256: `bd9957e429e347ef19cd176232e8974c71d8a590e5d101008a0404e89e341a28`
- Join inspection note: the file is retained only for audit. A technically smooth join does not cure the later island penetration and spatial morph.
