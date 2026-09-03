# Leg A approval package — not submitted

Prepared: 2026-08-17  
Task: first 5 seconds only, high aerial → exact Koya facade → resident acquired  
Authorisation: none

## Exact reference manifest

| Order | Role | File | SHA-256 |
| --- | --- | --- | --- |
| `@图片1` | first world state; completed Koya already present at target parcel | `production-assets/assets/generated/concept-spaces/location/BRISBANE-TOOWONG-KOYA-WORLD-START-V1.png` | `d9e6322af2eda2a5ed9a66b7745995eafd294b4207e8b9a70ceb332d5c821841` |
| `@图片2` | official Brisbane/Toowong geographic relationship only | `production-assets/assets/official/location/koya-toowong-brisbane-aerial.webp` | `45a3af2f1de8c71c205043e2cf1d9ad4c173858b9c6a1c9b3b19f736c1b1444e` |
| `@图片3` | sole readable Koya facade geometry authority | `production-assets/assets/official/exterior/koya-building-hero.jpg` | `6bd7b2965987212178109fb3374462e4564fd73a4e0e76d89c16581fc32eecfb` |
| `@图片4` | character rear action, used only at end | `production-assets/assets/generated/character/sports-female/master-panels-v2/jogging-rear.png` | `1d1bc93dc004d17d4f3432adc5ddf6f1af7ec8cd2db72a8ee9875d9f84f93fec` |

`@图片1` is C-grade concept art derived from the official aerial and facade. It
is not survey evidence or an official completed-project aerial.

## Complete prompt

> Generate the first 5 seconds of one uninterrupted forward camera journey.
> Begin exactly from the Brisbane/Toowong world in @图片1. The completed Koya
> building is already physically present at the target parcel in frame 1 and
> remains the same tracked object for the entire descent. @图片2 controls only the
> true city/river relationship. 0.0–2.8s: accelerate down one fixed target axis
> toward 6 Josling Street with a level horizon, no orbit and real changing
> parallax; this is not a crop, push-in or zoom on a still image. The building may
> gain visible detail as distance closes but may never disappear, be replaced,
> change site or gain/lose levels. 2.8–4.3s: decelerate continuously to about 1.6m
> eye height and resolve the exact official Koya facade from @图片3, keeping the
> five-level massing, paired curved balcony fronts, warm timber battens, thin dark
> railings, off-white concrete, roof planting, central entry and right driveway
> unchanged. 4.3–5.0s: keep creeping forward as the single athletic white
> Australian woman from @图片4 naturally enters the lower third from behind and
> begins her first step toward the central entry. End while both camera and woman
> still have forward momentum; do not freeze or pause on the building.
>
> High-end photorealistic off-the-plan property sales film, restrained natural
> colour, clear neutral Brisbane late-morning daylight, same camera-right sun
> throughout, no dusk and no night. One continuous physical moving camera, natural
> operator inertia, real parallax, no still-image zoom, no freeze, no cut, no
> dissolve, no black/white flash, no tree/roof/body wipe, no teleport, no
> architecture morph and no alternate camera angle. The woman has an identical
> dark-brown ponytail, sage-grey fitted long-sleeve sports top, matte black full-
> length leggings, white-grey running shoes, black watch on left wrist and one
> stainless bottle in left hand. No extra people, text, labels, pins, subtitles,
> watermarks or spoken dialogue. Natural high-air wind narrows continuously into
> subtle street ambience with one restrained ambient music bed and no audible
> reset.

## Exact payload fields

```json
{
  "model": "ep-20260812221158-hb576",
  "content": [
    {"type": "text", "text": "<complete prompt above>"},
    {"type": "image_url", "role": "reference_image", "image_url": {"url": "<uploaded @图片1>"}},
    {"type": "image_url", "role": "reference_image", "image_url": {"url": "<uploaded @图片2>"}},
    {"type": "image_url", "role": "reference_image", "image_url": {"url": "<uploaded @图片3>"}},
    {"type": "image_url", "role": "reference_image", "image_url": {"url": "<uploaded @图片4>"}}
  ],
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

## Live cost estimate

- Endpoint state: Running; bound to `doubao-seedance-2-5-260628`.
- No-video-input rate: CNY 0.07 / 1,000 tokens.
- 480p 16:9: 854×480 at 24fps.
- Estimated tokens: `5 × 854 × 480 × 24 ÷ 1024 = 48,037.5`.
- Estimated Leg A charge: **CNY 3.362625**.
- Maximum estimated one-pass 480p preview for A–E, only if every preceding leg
  passes and each later leg is separately authorised: **CNY 22.327830**.

Submitting Leg A requires an explicit user approval after reviewing this package.
No retry or Leg B submission is implied.

