# Koya Stage 1 storyboard gap audit v1

> 2026-08-17：已由`STAGE_01_REFERENCE_AND_GAP_MANIFEST_V1.md`取代。旧版没有把30秒运镜预演和官方入口门槛桥列为P0硬闸门。

Date: 2026-08-16  
Mode: geographic prelude `continuous_impression`, then `literal_walkthrough` from official facade  
Route: Brisbane/river → Toowong → 6 Josling Street → official Koya facade → sign → entrance → foyer → elevator → destination corridor → apartment door  
Video work: paused for storyboard review and timed animatic

## Active approved-direction anchors

| Order | ID | Purpose | Native frame | Exact video anchor |
| --- | --- | --- | --- | --- |
| 00A | `SB-S00-DIVE-A-BRISBANE-2500M` | Start a high westward Brisbane aerial descent | Official aerial guidance plus generated reconstruction | `scene-00-geographic-dive-v2/SB-S00-DIVE-A-BRISBANE-2500M.png` |
| 00B | `SB-S00-DIVE-B-TOOWONG-900M` | Same camera descends toward Toowong | Official aerial/map guidance plus generated reconstruction | `scene-00-geographic-dive-v2/SB-S00-DIVE-B-TOOWONG-900M.png` |
| 00C | `SB-S00-DIVE-C-BLOCK-250M` | Near-vertical plunge keeps target block centred | Concept geographic linking frame | `scene-00-geographic-dive-v2/SB-S00-DIVE-C-BLOCK-250M.png` |
| 00D | `SB-S00-DIVE-D-MOTION-COVER-60M` | Brake through unreadable treetop/roof-edge motion | Concept physical movement cover | `scene-00-geographic-dive-v2/SB-S00-DIVE-D-MOTION-COVER-60M.png` |
| 00E | `SB-S00-DIVE-E-WHIP-PITCH` | Pitch continuously from steep-down to eye level | Concept physical movement cover | `scene-00-geographic-dive-v2/SB-S00-DIVE-E-WHIP-PITCH.png` |
| 00F | `SB-S00-DIVE-F-OFFICIAL-FACADE-GOLDEN` | Settle on exact official facade geometry | `../production-assets/assets/official/exterior/koya-building-hero.jpg` | `scene-00-geographic-dive-v2/SB-S00-DIVE-F-OFFICIAL-FACADE-GOLDEN.png` |
| 00G | `SB-S00-DIVE-G-RESIDENT-ACQUIRED` | Acquire the locked resident mid-step | Official facade plus isolated character layer | `scene-00-geographic-dive-v2/SB-S00-DIVE-G-RESIDENT-ACQUIRED.png` |
| 00 | `OFF-EXT-001` | Exact official five-level building establishment before the resident becomes the focus | `../production-assets/assets/official/exterior/koya-building-hero.jpg` | `../video-production/phase-01/anchors-16x9/OFF-EXT-001-official-1280x720.jpg` |
| 01B | `SB-S01-BRAND-001` | Official architecture detail for a natural pass beside the readable Koya sign | `../production-assets/assets/official/exterior/koya-building-hero.jpg` | `../video-production/phase-01/anchors-16x9/OFF-EXT-001-official-entry-sign-1280x720.jpg` |
| 02 | `CON-FOY-001` | Camera and resident physically cross the glazed entrance threshold | `../production-assets/assets/generated/concept-spaces/foyer/CON-FOY-001-threshold-v1.png` | `../video-production/phase-01/anchors-16x9/CON-FOY-001-threshold-1280x720.png` |
| 03 | `CON-FOY-002` | Same foyer axis; resident reaches the lift call button | `../production-assets/assets/generated/concept-spaces/foyer/CON-FOY-002-lift-call-v1.png` | `../video-production/phase-01/anchors-16x9/CON-FOY-002-lift-call-1280x720.png` |
| 04A | `SB-S01-LIFT-OPEN-001` | Same single lift opens in same foyer | Concept storyboard | `../production-assets/assets/generated/concept-spaces/elevator/SB-S01-LIFT-OPEN-001.png` |
| 04B | `SB-S01-LIFT-CROSS-001` | Camera physically crosses lift sill behind resident | Concept storyboard | `../production-assets/assets/generated/concept-spaces/elevator/SB-S01-LIFT-CROSS-001.png` |
| 04C | `SB-S01-CABIN-OPEN-001` | Camera settled in same open cabin | Concept storyboard | `../production-assets/assets/generated/concept-spaces/elevator/SB-S01-CABIN-OPEN-001.png` |
| 05 | `SB-S01-DOORS-CLOSED-001` | Same cabin position with doors closed | Concept storyboard | `../production-assets/assets/generated/concept-spaces/elevator/SB-S01-DOORS-CLOSED-001.png` |
| 06A | `SB-S01-ARRIVE-001` | Same doors reopen at destination | Concept storyboard | `../production-assets/assets/generated/concept-spaces/elevator/SB-S01-ARRIVE-001.png` |
| 06B | `SB-S01-EXIT-001` | Camera crosses out behind resident | Concept storyboard | `../production-assets/assets/generated/concept-spaces/corridor/SB-S01-EXIT-001.png` |
| 07A | `SB-S01-CORRIDOR-001` | Fast corridor approach looking past resident | Concept storyboard | `../production-assets/assets/generated/concept-spaces/corridor/SB-S01-CORRIDOR-001.png` |
| 07B | `SB-S01-DOOR-001` | Right hand reaches handle; camera stops outside | Concept storyboard | `../production-assets/assets/generated/concept-spaces/corridor/SB-S01-DOOR-001.png` |

## Remaining required refinements

| Priority | Refinement | Required detail | Why it is necessary |
| --- | --- | --- | --- |
| 1 | Licensed geographic replacement | Replace generated A–C reconstruction with licensed aerial/map motion while preserving the approved camera path | Prevents concept geography from being mistaken for exact location evidence |
| 2 | `SB-S01-CALL-001` optional closer call action | If the existing button action is too small at playback speed, move the same camera physically closer; never cut to an inserted hand shot | Preserves speed while keeping the lift-call action readable |
| 3 | Timed animatic | Assign exact frame durations and test the 22–25s route before Seedance | Prevents another prompt from forcing incompatible locations into one short generation |

## Detail policy

- Use natural proximity close-ups, not inserted detail shots.
- Koya signage, lift-call action, lift threshold and apartment handle must be readable because the camera physically approaches them.
- Do not cut to a hand, button, logo or door handle from a new angle.
- Keep every active anchor at exact 16:9; standardize video inputs to 1280×720.
- Common areas remain `Concept Design / Artist Impression` until official material exists.

## Timing implication

Use a fast but legible target of roughly 22–25 seconds from Brisbane context to the apartment door: geographic fly-in 5.5–6.0s, building/entrance 6.5–7.5s, foyer and lift call 2.0–2.3s, lift open and physical entry 2.0–2.5s, doors close/travel/reopen 2.0–2.5s, corridor approach and door endpoint 3.5–4.0s. The camera may move faster than the resident and may look past her toward the next spatial target. Shorten pauses and elevator dwell first; never remove the entrance, lift threshold or corridor to hit the duration.

## Gate

Do not submit another Seedance task until the user reviews the corrected 18-frame route and a timed animatic proves the 22–25s pacing. Public sales use also requires licensed geographic imagery for A–C.
