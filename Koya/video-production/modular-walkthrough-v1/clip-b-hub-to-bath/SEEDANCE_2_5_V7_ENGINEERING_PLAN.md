# Clip B V7 — Seedance 2.5 engineering plan

Status: planning only. No paid generation is authorised by this file.

## What was wrong with V5 and V6

- V5 supplied seven independently generated stills as `reference_image`. Seedance treated them as visual references, not as mandatory ordered timeline keyframes. This allowed it to blend/rebuild doors, walls and room identity.
- V6 supplied only a first frame and a last frame. The endpoints were too far apart spatially, so the model invented the missing right turn and changed doorway geometry during interpolation.
- Therefore adding more independently generated still images is not a reliable fix. More inconsistent images give the model more conflicting geometry.

## Verified Seedance 2.5 capability

Current endpoint: `ep-20260812221158-hb576`

Resolved model: `doubao-seedance-2-5-260628`

The live endpoint accepts text, image, video and audio and exposes:

- multimodal-to-video
- video editing
- video extension

The official Ark video-generation API accepts combinations of text, images, video and audio. Images with role `reference_image` and video with role `reference_video` are reference materials. The API does not state that a list of reference images becomes an ordered, compulsory keyframe timeline.

## Correct V7 method

Use a deterministic continuous reference video to lock physical motion, then use Seedance 2.5 video editing/reference-video generation to add realism.

The deterministic reference video must be approved before any paid Seedance submission.

### Reference-video requirements

- Duration: exactly 5.0 seconds.
- Aspect ratio: 16:9.
- Camera height: 1.58 m, first-person property-viewing height.
- Lens: rectilinear 24–28 mm full-frame equivalent; no fisheye.
- One camera object and one uninterrupted camera spline.
- No cuts, dissolves, hidden wipes, teleporting or image crossfades.
- Apartment shell, island, MPR wall, fixed kitchen, Bath doorway and Bath fixtures remain one static 3D scene.
- The camera may translate and yaw only; no geometry is allowed to move or regenerate.

## 26-checkpoint camera timeline

These are QA checkpoints rendered from the same scene, not 25 independently generated images.

| Frame | Time | Position and view constraint |
|---:|---:|---|
| 00 | 0.0 s | Start at B0 in the hub; camera already facing west into the north-side aisle. Island edge is left/foreground; fixed kitchen is right. |
| 01 | 0.2 s | Fast westward move begins; island remains left and fixed kitchen remains right. |
| 02 | 0.4 s | Reach B1; opposing parallax proves translation in one physical aisle. |
| 03 | 0.6 s | Continue west and begin a very slight clockwise steering motion. |
| 04 | 0.8 s | Continue west; MPR remains a solid wall on the left/south. |
| 05 | 1.0 s | Approach the island's west end; no invented corridor or MPR opening. |
| 06 | 1.2 s | The single Bath doorway first becomes visible ahead-right. |
| 07 | 1.4 s | Same doorway stays visible while the camera continues translating. |
| 08 | 1.6 s | Begin the main continuous clockwise curve toward that doorway. |
| 09 | 1.8 s | Both jambs retain the same identity; no second door appears. |
| 10 | 2.0 s | Doorway moves toward frame centre through camera yaw and translation. |
| 11 | 2.2 s | Bath vanity becomes readable through the same opening. |
| 12 | 2.4 s | Approach the threshold; jambs remain fixed and opaque. |
| 13 | 2.6 s | Complete the curve at B3 and begin the straight threshold crossing. |
| 14 | 2.8 s | Camera physically passes between the two fixed jambs. |
| 15 | 3.0 s | Fully inside the compact Bath; doorway recedes behind camera. |
| 16 | 3.2 s | Continue forward; vanity, mirror and toilet remain fixed. |
| 17 | 3.4 s | Slight rightward look develops naturally toward the vanity. |
| 18 | 3.6 s | Maintain forward momentum; Bath scale stays compact. |
| 19 | 3.8 s | Continue inside the same room; no duplicated fixture or new corridor. |
| 20 | 4.0 s | Ease forward while keeping the vanity and mirror readable. |
| 21 | 4.2 s | Same fixed Bath geometry; no wall or fixture movement. |
| 22 | 4.4 s | Gentle final approach, still one continuous camera take. |
| 23 | 4.6 s | Ease without stopping dead or changing room identity. |
| 24 | 4.8 s | Settle near B5 with residual forward motion. |
| 25 | 4.99 s | End inside compact main Bath, suitable as the next clip's exact first frame. |

## Seedance instruction structure

The submission prompt must refer to the reference video as the source of truth for camera motion and topology:

1. Preserve the reference video's exact camera trajectory, timing, room dimensions, wall positions, doorway identity and physical occlusion.
2. Restyle only materials, daylight, photographic texture and micro-details to match Koya's approved interior references.
3. Do not add, remove, widen, relocate or duplicate any wall, door, room, island, cabinet or plumbing fixture.
4. Do not cut, dissolve, morph, teleport, pass through solids, or transition between alternative room layouts.
5. Keep continuous daytime exposure and realistic first-person stabilised walking motion.

## Approval gate before spending

Do not submit V7 to Seedance until all of the following pass on the free local reference-video preview:

- exact floor-plan route confirmed against `CLIP_B_EXACT_PLAN_ROUTE_V2.png`;
- all 26 checkpoints visible in one contact sheet;
- one doorway identity throughout frames 06–14;
- no wall crossing or crossfade;
- user approves speed, turn timing and compact Bath scale;
- one task only, one output only.

## Sources

- Official Ark CreateContentsGenerationsTasks API: https://api.volcengine.com/api-docs/view?action=CreateContentsGenerationsTasks&serviceCode=ark&version=2024-01-01
- Official Ark video-generation documentation: https://www.volcengine.com/docs/82379/1520758?lang=zh
- Official Jimeng first/last-frame mode documentation: https://www.volcengine.com/docs/85621/1791184?lang=zh
