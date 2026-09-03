# Koya 30s visually seamless walkthrough — production plan v3

Date: 2026-08-17  
Status: pre-production reset; no paid generation authorised  
Final format: 30.00s, 16:9, daylight, one perceived forward camera journey

## Why v2 is rejected

Task `cgt-20260817115116-j8jxl` is a rejected draft. It interpreted a still-image
animatic and seventeen independent reference frames as a slideshow. Zooming a
still image, replacing a scene behind a doorway, or producing one 30-second file
does not prove spatial continuity.

The following v2 inputs must not be reused as a generation package:

- `stage-01-camera-blockout-30s-v2.mp4`;
- `KOYA_SEEDANCE_2_5_MASTER_PROMPT_V2.txt`;
- `REFERENCE_MANIFEST_V2.md`;
- the fourteen storyboard stills as one simultaneous route-control bundle.

They remain only as failure evidence and visual-reference history.

## New production architecture

The finished film remains exactly 30 seconds. Production is sequential because
visual continuity is the acceptance criterion:

1. Generate Leg A as the only original reference-generation task.
2. Inspect its actual final 0.5 seconds at frame level.
3. Generate Leg B by extending the actual Leg A video.
4. Repeat for Legs C, D and E. A later leg may not be generated while the prior
   handoff is unapproved.
5. Assemble only approved legs. A join may use a 2–4-frame audio/video blend as
   technical insurance, but it may not conceal a location, angle or identity
   change.

This follows Seedance 2.5's official video-extension mode. Extension inputs and
outputs use MOV, `ratio=adaptive`, and explicit `续写/向后延长` intent.

## Locked creative contract

- One athletic white Australian woman: `CHAR-RESIDENT-004 v2`.
- Sage-grey long-sleeve sports top, matte black leggings, white-grey shoes,
  ponytail, black watch on left wrist, stainless bottle in left hand.
- One forward-moving camera, about 1.6m high after the aerial descent.
- Daylight throughout: neutral late-morning Brisbane light, no dusk or night.
- Official Koya facade is the only exterior geometry authority.
- Foyer, lift and corridor are labelled concept/artist impression.
- Apartment 106 floor plan controls interior adjacency and scale; official
  interior renders control material language but are not claimed as Unit 106.

## Final 30-second route

| Leg | Final time | Duration | Physical route | Task type |
| --- | ---: | ---: | --- | --- |
| A | 0.0–5.0s | 5s | Brisbane/Toowong descent → exact Koya street facade; resident acquired only at the end | original reference generation |
| B | 5.0–12.0s | 7s | same facade → resident approaches → same entrance threshold → same foyer/lift axis | extend Leg A |
| C | 12.0–16.0s | 4s | lift opens → resident and camera enter same cabin → doors close/brief rise → same doors reopen → begin exit | extend Leg B |
| D | 16.0–22.0s | 6s | complete lift exit → short corridor → Apartment 106 door → camera crosses apartment threshold → compact hall | extend Leg C |
| E | 22.0–30.0s | 8s | compact hall → kitchen island → dining/living → slow toward terrace | extend Leg D |

The elevator is four seconds total, not ten. No room is toured merely to fill
time; the first film ends in the living zone.

## Generation ladder

### Preview pass

- 480p, native audio, MOV.
- One leg at a time.
- Stop at the first failed handoff.
- Do not upscale or produce 720p until all five preview legs pass.

### Final pass

- 720p, native audio, MOV generation masters.
- Re-run only from approved route inputs and approved actual handoff videos.
- Assemble 30.00s master; create web MP4 only after visual approval.

## Approval and billing gate

Before any task is submitted, show the user:

1. exact files for Leg A;
2. full Leg A prompt and API payload;
3. preview resolution, duration and audio settings;
4. current account price and estimated charge for Leg A;
5. maximum preview-pass exposure if every leg is approved;
6. confirmation that one task will be submitted and its returned task ID saved.

Approval for one leg does not authorise automatic retries or later legs.

## Completion definition

The job is complete only when:

- every join passes forward and reverse frame inspection;
- normal-speed playback reads as one continuous physical journey;
- facade, character, wardrobe, light, camera direction and thresholds do not jump;
- the full 30-second master passes user review;
- final delivery encodes and, if requested, scroll-scrub integration pass seek QA.

