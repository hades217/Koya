# Koya — Seedance 2.5 verified usage

Verified: 2026-08-15  
Purpose: literal off-plan property sales walkthrough, not a website Hero montage

## Verified account resource

- Endpoint: `ep-20260812221158-hb576`
- Bound model: `doubao-seedance-2-5-260628`
- Region: `cn-beijing`
- Endpoint state: `Running`
- Input modalities: text, image, video and audio
- Output modality: video
- Advertised task types: multimodal-to-video, video editing and video extension
- API family: `/v3/contents/generations`

These values come from the live account control plane. Do not replace the Endpoint with the model name when using the current platform profile.

## Verified service behaviour on this account

The following constraints were returned by the live Seedance service during the Koya test:

1. `1080p` was rejected for this account and model. Use `720p`; the exact horizontal deliverable is `1280×720`.
2. First-frame or first-and-last-frame generation follows the first image's aspect ratio. Supplying `--ratio` at the same time was rejected.
3. A `last_frame` image cannot be mixed with `reference_image` inputs in the tested workflow.
4. Image roles must be written explicitly on the wire: `first_frame`, `last_frame`, or `reference_image`. Do not depend on position-only shorthand.
5. A reference image with aspect ratio `2.77` was rejected; keep every active reference within the service's accepted range and use exact 16:9 masters for Koya.
6. Ten-second, 720p generation with native audio succeeded. Longer duration has not yet been service-validated for this Endpoint and must not be presented as confirmed.
7. `generate_audio=true` produced an AAC stereo stream in the downloaded MP4.

## Correct Koya input strategy

Use `reference_image` mode for the final 16:9 route draft because Koya needs multiple independent truth sources: official architecture, locked character identity and approved route anchors. In this mode, pass `--ratio 16:9 --resolution 720p` and use only exact 16:9 or service-safe reference images.

Use first/last-frame mode only for a short, tightly controlled motion between two exact 1280×720 anchors. In that mode:

- provide only `first_frame` and `last_frame` roles;
- do not mix in `reference_image` inputs;
- omit `--ratio`, because the service follows the first frame;
- verify both frames have exactly the same 16:9 dimensions.

For a literal walkthrough longer than one validated task duration, prefer the Endpoint's video-extension capability and verify the continuation frame-by-frame. Do not assemble independent clips and call the result a literal one-take.

## Safe command sequence

1. `arkcli auth status`
2. `arkcli resources resolve ep-20260812221158-hb576 --format json`
3. `arkcli +gen ... --dry-run --modality video` to inspect the client payload only
4. Submit exactly once with the Endpoint ID and explicit media roles
5. Record the returned `task_id`
6. Poll the same task with `arkcli gen get <task_id>` until terminal state
7. Download locally and verify dimensions, duration, video stream, native audio stream and dense continuity frames

`--dry-run` does not perform server-side validation. A successful dry run is not proof that the Endpoint accepts a parameter combination.

## Koya hard gates before the next paid task

- complete and approve every Stage 1 route anchor;
- all active video anchors are exactly 1280×720;
- official Koya signage is readable without an insert cut;
- the entrance, foyer, single elevator and destination corridor form one route ledger;
- the camera enters and remains inside the same elevator;
- the prompt says `single uninterrupted take; zero edits` and forbids spatial substitution;
- submit only one draft and stop for user review.

