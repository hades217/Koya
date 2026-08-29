# Koya Seedance 2.5 reference manifest v3

This package is intentionally reduced to one motion authority and eight visual authorities. The previous 17-image storyboard stack is rejected because it encouraged static frame-to-frame imitation.

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Video 1 | `reference_video` | `stage-01-camera-control-30s-v4-1080p.mp4` | Sole camera route, timing, physical thresholds and one-take continuity. Never appearance. | `cf0383667cefd64989386772fe14d51fafae79b85f71bf38d0d80d423b72d84f` |
| Image 1 | `reference_image` | `references/01-world-start-16x9.png` | Brisbane/Toowong world and target geography; concept composite, not survey evidence. | `1dc848e7d1a753c31d926c0546137cfbe5515a2ecdb74c9e2ceaff81a41c1a4e` |
| Image 2 | `reference_image` | `references/02-official-facade-16x9.jpg` | Official Koya facade geometry and exterior appearance. Highest exterior authority. | `f8b807bbe68078b95d5203bb05510e8a06d44c2139237ffd85b19a481ea51aad` |
| Image 3 | `reference_image` | `references/03-character-back-16x9.png` | Locked `CHAR-RESIDENT-004 v2`, rear identity, wardrobe and left-hand bottle. | `76ae51087203e6cf0d31d16b13655b9aaab21df7a3610111ad32e67ad39d26bc` |
| Image 4 | `reference_image` | `references/04-entry-foyer-16x9.png` | Concept entrance threshold and compact single-lift foyer. | `6f523952fab1775b9075774bf67f0639b057ac1b75c6faf9720777be1d5c8439` |
| Image 5 | `reference_image` | `references/05-lift-entry-16x9.png` | Same-lift entry threshold and cabin appearance. | `17df24029d9d5a47fd6018667a0b7e5b14a2a64acf7957596dca2ef7458befd6` |
| Image 6 | `reference_image` | `references/06-lift-exit-16x9.png` | Same doors reopen and connect to the Level 1 corridor. | `5f850be998bd4a24efe7e4b9c17c4804472ef2973282953682877c8f23aff23c` |
| Image 7 | `reference_image` | `references/07-official-living-16x9.jpg` | Official Koya interior materials and apartment scale only; not exact Apartment 106 geometry. | `6534655edfe19b911071017f23a0a7ce9516b9395a8f78566d698cc6fd4d5df9` |
| Image 8 | `reference_image` | `references/08-apartment-106-plan-16x9.png` | Official Level 1 Apartment 106 adjacency and scale only; must never appear visually in output. | `94928a6704cf35685f0e36cd0f664ef486cd400b1322d986317e53423bdeccb9` |

All eight active images are normalized to exactly 1280×720. The camera-control video is 1920×1080, H.264, 24fps, exactly 30.000 seconds, with no audio and no review overlays.

## Truth hierarchy

1. Official facade and official floor plan override generated concept space details.
2. Official interior controls material direction only; it is not represented as an exact Apartment 106 rendering.
3. Generated foyer and lift images are labelled concept designs and may not be represented as developer-approved architecture.
4. The low-poly camera-control video controls motion and adjacency only, never final appearance.
