# Scene 00 — Brisbane to Koya continuous geographic dive v2

Date: 2026-08-16  
Format: 16:9, exact working anchors 1280×720  
Continuity: `continuous_impression` until the official facade stabilises; `literal_walkthrough` begins when the resident is acquired  
Target duration: 5.8 seconds

## Director's correction

The old sequence used an aerial crop followed by two map crops. It communicated location but did not describe one moving camera. The active sequence now uses one forward axis, one fixed destination and progressively steeper pitch: Brisbane high aerial → Toowong descent → target block → in-camera motion cover → rapid pitch-up → exact official Koya facade → locked resident walks toward the entrance.

## Camera ledger

| Time | ID | Approx. height / pitch | Camera movement | Continuity lock | Evidence boundary |
| --- | --- | --- | --- | --- | --- |
| 0.00–1.00s | `SB-S00-DIVE-A-BRISBANE-2500M` | 2,500m / 55° down | Accelerate west with Brisbane River as the geographic spine | Toowong target sits lower-centre; sun remains camera-right | Generated geographic reconstruction guided by official Koya aerial; not survey or live drone evidence |
| 1.00–2.20s | `SB-S00-DIVE-B-TOOWONG-900M` | 900m / 70° down | Descend and push forward without orbiting | Same river orientation, heading and warm light; target does not jump sideways | Generated geographic reconstruction guided by official aerial and location map |
| 2.20–3.50s | `SB-S00-DIVE-C-BLOCK-250M` | 250m / 82° down | Fast near-vertical plunge; peripheral radial blur increases | Target block stays at frame centre; river remains at upper-right edge | Concept geographic linking frame; exact parcel geometry must be replaced by licensed map/aerial data for final sales use |
| 3.50–3.80s | `SB-S00-DIVE-D-MOTION-COVER-60M` | 60m / pitch begins to release | Hard brake while passing treetops and indistinct roof edges | Architecture is deliberately unreadable; motion remains forward | Approved physical movement cover; it may hide generated-to-official asset handoff, not a new story location |
| 3.80–4.10s | `SB-S00-DIVE-E-WHIP-PITCH` | treetop height / 80°→5° | One rapid continuous pitch-up and forward sweep | Same centre axis and warm camera-right light; no readable facade | Approved in-camera blur cover; no dissolve, black frame or teleport |
| 4.10–5.00s | `SB-S00-DIVE-F-OFFICIAL-FACADE-GOLDEN` | eye level / 0–3° | Motion settles while still creeping toward the central entrance | Building geometry switches to and remains the exact official facade | Direct official facade derivative with colour grade only; no AI geometry |
| 5.00–5.80s | `SB-S00-DIVE-G-RESIDENT-ACQUIRED` | eye level / 0–3° | Camera continues forward and acquires the resident from behind | Same official facade pixels; locked resident is mid-step on the entrance axis | Official facade plus isolated `CHAR-RESIDENT-004 v2` walking layer |

## Non-negotiable motion constraints

- No map card, map cut, orbit, cross-dissolve or zoom reset between A and G.
- The target may drift vertically as pitch changes, but lateral drift must stay within 4% of frame width.
- Do not rotate the horizon more than 2°. The sensation comes from descent, pitch and speed ramp, not a spiral.
- A→C accelerates; D is the brake; E is the fastest angular pitch; F visibly settles; G resumes a controlled forward walk.
- D and E must remain shorter than 0.35s each. Holding either frame reveals non-authoritative spatial detail.
- F and G preserve the official building exactly. Do not interpolate an AI rooftop or extra facade between E and F.
- Character identity, ponytail, wardrobe, watch side and bottle hand remain locked from G through the apartment-door endpoint.

## Active review files

- Geographic dive contact sheet: `scene-00-geographic-dive-v2/scene-00-geographic-dive-contact-sheet-v4.jpg`
- Full 18-frame route: `scene-01-full-route-contact-sheet-v6.jpg`
- The readable AI roof/facade attempts were removed from the active route and retained only under `../references/rejected/geographic-dive-readable-ai-building-v1/`.
