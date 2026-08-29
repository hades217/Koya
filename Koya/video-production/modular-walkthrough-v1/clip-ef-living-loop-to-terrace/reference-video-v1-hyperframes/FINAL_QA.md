# Clip EF local reference QA

- Status: `V5 STRUCTURAL BOARD PASSED AUTOMATED CHECK / VISUAL USER REVIEW PENDING`
- Scope: deterministic spatial and camera-path reference only
- Paid Seedance task: not submitted
- Duration: 8.000 seconds
- Frame size: 1920 x 1080
- Frame rate: 30 fps
- Codec: H.264
- Previous local reference: `renders/koya-106-mpr-glance-living-loop-terrace-reference-v2-8s.mp4` (retained only as audit history; forbidden as an upstream generation input)
- SHA-256: `276ac28d257c7f1dc66be625c90e6663bce774c44507a22527260ed8e39c6378`

## V5 structural route checks

- MPR is only glanced through its official lower-right public opening; no kitchen-side or curved-wall opening is used.
- The camera starts at the Dining/Living hub and turns clockwise across Dining, one island, kitchen, the internal TV wall, sofa and wrap glazing.
- The public-room move is continuous and does not cut, dissolve, teleport, or pass through furniture.
- The camera crosses the same east-side Living glazing and visible floor track onto the Terrace.
- The endpoint remains on the Terrace; no reverse cut or indoor room substitution occurs.

## Evidence

- Current 14-frame board: `../living-panorama-storyboard-v1/structural-storyboard-v5/index.html`
- Frame manifest: `../living-panorama-storyboard-v1/structural-storyboard-v5/FRAME_MANIFEST.md`
- Structural contact sheet: `../living-panorama-storyboard-v1/structural-storyboard-v5/structural-contact-v5.jpg`
- HyperFrames check: passed at 14 named timestamps with 0 findings.

## Boundary

Earlier local MP4s and rendered storyboards V1/V2 remain audit history only and are forbidden as generation inputs. The next gate is user approval of the V5 structural storyboard, followed by render-level anchors from the same fixed scene. A new frame-reviewed V5 reference video is required before any Seedance preflight.
