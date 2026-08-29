# Seedance 2.5 prompt templates v3

Status: drafting complete; dynamic references and current billing fields must be
materialised before submission. No paid task is authorised.

## Shared visual/audio lock

Append this block to every leg:

> High-end photorealistic off-the-plan property sales film, restrained natural
> colour, clear neutral Brisbane late-morning daylight, no dusk and no night.
> One continuous forward-moving camera, natural operator inertia, real parallax,
> no still-image zoom, no freeze, no cut, no dissolve, no black/white flash, no
> wipe, no teleport, no architecture morph and no alternate camera angle. Only
> one athletic white Australian woman, identical dark-brown ponytail, sage-grey
> fitted long-sleeve sports top, matte black full-length leggings, white-grey
> running shoes, black watch on left wrist and one stainless bottle in left hand.
> Right hand opens doors and presses buttons. No extra people, duplicate limbs,
> text, subtitles, watermark or spoken dialogue. Continue the same restrained
> ambient score and spatially correct environment sound without an audible reset.

## Leg A — 5s original reference generation

References:

- `@图片1`: concept Brisbane/Toowong world start with completed Koya already present at the target parcel;
- `@图片2`: official Koya/Toowong aerial, geography and daylight only;
- `@图片3`: official Koya facade, sole readable building-geometry authority;
- `@图片4`: locked rear-action character, used only near the end.

Prompt:

> Generate the first 5 seconds of one uninterrupted forward camera journey.
> 0.0–2.8s: begin exactly from the world in @图片1. The completed Koya building is
> already physically present at the target parcel in frame 1 and remains the same
> tracked object for the entire descent. From the high oblique Brisbane/Toowong
> aerial, accelerate down one fixed target axis toward 6 Josling Street; preserve
> a level horizon, no orbit, no map card, and real changing parallax rather than
> scaling a still image. The building may become more detailed as distance closes,
> but it may never disappear, be replaced, change site or gain/lose levels. 2.8–4.3s:
> decelerate continuously to approximately 1.6m eye height and resolve the exact
> official Koya facade from @图片3, keeping its five-level massing, curved balcony
> edges, timber battens, roof planting, central entry and right driveway unchanged.
> 4.3–5.0s: keep creeping forward as the single woman from @图片4 naturally enters
> the lower third from behind and begins her first step toward the central entry.
> End while both camera and woman still have forward momentum; do not freeze on
> the building. Preserve neutral late-morning daylight and camera-right sun.

Payload draft:

```json
{
  "model": "<verified Seedance 2.5 endpoint>",
  "omni_reference_task_type": "reference",
  "duration": 5,
  "ratio": "16:9",
  "resolution": "480p",
  "generate_audio": true,
  "return_last_frame": true,
  "output_format": "mov",
  "watermark": false
}
```

## Leg B — 7s extension, facade to lift

References:

- `@视频1`: actual approved Leg A MOV;
- `@图片1`: official Koya facade;
- `@图片2`: locked character rear view;
- `@图片3`: daylight entrance-threshold concept;
- `@图片4`: daylight lift-call concept.

Prompt:

> 向后延长 @视频1，续写7秒，画面与音频无缝衔接。The first frames must
> continue @视频1's exact final camera velocity, facade scale, entrance axis,
> resident gait phase, daylight and audio bed. Do not restart or re-establish the
> shot. The resident walks briskly toward the same central entrance while the
> camera follows from rear three-quarter view. She opens the same glass door with
> her right hand; the camera physically crosses behind her while exterior paving,
> metal sill and foyer stone floor remain visible together. Continue through the
> compact same foyer in @图片3–4: one elevator centred at the end, left planter,
> right bench, identical floor joints. End approximately one metre from the still-
> closed lift doors as her right hand nears the call button and the camera is still
> drifting forward. Never hide a spatial replacement behind the doorframe.

Payload draft: `duration=7`, `ratio=adaptive`, `resolution=480p`,
`generate_audio=true`, `output_format=mov`, `watermark=false`.

## Leg C — 4s extension, same lift cabin

References:

- `@视频1`: actual approved Leg B MOV;
- `@图片1`: lift threshold/crossing concept;
- `@图片2`: same cabin open-door state;
- `@图片3`: same cabin closed-door state;
- `@图片4`: same cabin destination-opening state.

Prompt:

> 向后延长 @视频1，续写4秒，画面与音频无缝衔接。Continue the exact
> inherited camera axis and resident stance; no restart. 0.0–1.1s: the same single
> lift opens and the resident immediately enters; camera follows across the metal
> sill, showing foyer floor, doorframe, sill and cabin floor in one continuous
> view. 1.1–2.0s: camera is inside the same cabin facing the same doors as they
> close. 2.0–2.8s: compress travel using only subtle upward body-weight motion,
> mechanical lift sound and one arrival tone; camera remains in the cabin.
> 2.8–4.0s: the same doors reopen and the resident begins crossing into a short
> daylight residential corridor while both cabin side walls remain visible. End
> mid-threshold with forward momentum. No black-frame swap and no corridor camera.

Payload draft: `duration=4`, `ratio=adaptive`, `resolution=480p`,
`generate_audio=true`, `output_format=mov`, `watermark=false`.

## Leg D — 6s extension, lift exit to compact apartment hall

References:

- `@视频1`: actual approved Leg C MOV;
- `@图片1`: lift-exit/corridor continuity concept;
- `@图片2`: Apartment 106 door/threshold concept;
- `@图片3`: official Apartment 106 floor plan;
- `@图片4`: corrected compact Apartment 106 hall concept.

Prompt:

> 向后延长 @视频1，续写6秒，画面与音频无缝衔接。Open by completing
> the threshold crossing already started in @视频1; the cabin edges move naturally
> behind camera. Follow the resident quickly through a short, normal-width corridor
> to the apartment door. She opens it with her right hand while the bottle remains
> in her left. Camera physically crosses behind her; corridor carpet, metal sill
> and interior timber floor are visible together. Enter the compact Apartment 106
> hall governed by @图片3. Keep the hall narrow and ordinary residential scale;
> do not create a lobby, giant MPR or direct living-room jump. End inside the hall
> as the kitchen island edge first becomes visible, with continued forward motion.

Payload draft: `duration=6`, `ratio=adaptive`, `resolution=480p`,
`generate_audio=true`, `output_format=mov`, `watermark=false`.

## Leg E — 8s extension, living-zone reveal

References:

- `@视频1`: actual approved Leg D MOV;
- `@图片1`: official Apartment 106 floor plan;
- `@图片2`: official Koya 2BR living/kitchen material reference only;
- `@图片3`: corrected compact Unit 106 hall-to-living concept.

Prompt:

> 向后延长 @视频1，续写8秒，画面与音频无缝衔接。Continue the exact hall
> width, floor joints, island edge, resident scale, camera height and daylight of
> @视频1. Move naturally past the kitchen edge into an apartment-scale linear
> kitchen and island, six-seat dining area and living zone, following adjacency in
> @图片1 and only the material language of @图片2. The woman walks toward the terrace
> and becomes secondary while the camera gradually slows. Finish on a credible
> kitchen–dining–living–terrace relationship with subtle forward drift still alive.
> Do not create a penthouse, banquet table, oversized island, extra room, invented
> MPR tour, dusk view or exaggerated skyline.

Payload draft: `duration=8`, `ratio=adaptive`, `resolution=480p`,
`generate_audio=true`, `output_format=mov`, `watermark=false`.

## Submission discipline

- Materialise and show the exact Leg A payload before billing.
- Submit one task only and save its task ID.
- Inspect the entire result and dense final frames before preparing Leg B.
- Dynamic `@视频1` references may never point to a storyboard animatic or rejected
  video; they must point to the immediately preceding approved provider output.
