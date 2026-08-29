# Seedance 2.5 Apartment 106 Interior Leg 1 — Approval Package

Status: `SUBMITTED_2026-08-18_QA_REJECTED_NO_FURTHER_PAID_RETRY`

## Objective

Extend the preserved 21.047-second one-camera video by one continuous 30-second interior leg. This leg ends at the Bedroom 1 southwest doorway so the next leg can continue from the returned real last frame.

## Provider resource

- Endpoint: `ep-20260812221158-hb576`
- Foundation model: `doubao-seedance-2-5-260628`
- Endpoint state verified: `Running`
- Supported modality: multimodal video generation, video editing and video extension

## Ordered inputs

1. Reference video / immutable preceding footage: `../seedance-2.5-submission-v4-segment-c-retry1/outputs/koya-continuous-preview-through-apartment-entry-21s.mp4`
2. K01 actual-endpoint turn: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k01-actual21s-turn-to-wet-area-v2.png`
3. K02 separate bath/laundry: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-main-bath-laundry-cinematic-v1.png`
4. K03 turn south/open Bedroom 2: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k03-wet-area-to-bedroom2-connector-v3.png`
5. K04 Bedroom 2 reveal: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-bedroom2-door-cinematic-v1.png`
6. K07 hinged MPR threshold: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k07-kitchen-to-mpr-threshold-v2.png`
7. K08 compact MPR reveal: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-mpr-cinematic-v1.png`
8. K09 same-door MPR exit: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k09-mpr-same-door-exit-v3.png`
9. K11 Bedroom 1 threshold/end target: `../../production-assets/assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k11-kitchen-to-bedroom1-threshold-v1.png`

The planned K05 Bedroom 2 exit still was excluded from the submitted payload after the provider preflight flagged that single generated person image as possible privacy information. The same-door exit action remained explicitly locked in the prompt and was bracketed by the adjacent Bedroom 2 and route references.

The official Apartment 106 floor plan is topology evidence but is intentionally not uploaded as a video-generation image, preventing plan graphics or labels from leaking into the film.

## Exact requested parameters

- Task type: `extend`
- Duration: `30` seconds
- Ratio: `adaptive` (mandatory/safest for video extension; source is 16:9)
- Resolution: `720p` (Seedance 2.5 currently supports 480p/720p, not 1080p)
- Native audio: `true`
- Return last frame: `true`
- Output: `mp4`
- Watermark: `false`
- Priority: `0`
- Draft: unsupported and omitted
- Seed: unsupported and omitted
- Camera-fixed: unsupported and omitted

## Current price evidence and estimate

- Live account rate verified on 2026-08-18: `V2VCompletion` CNY `0.042` per 1,000 completion tokens.
- Closest successful like-for-like evidence: previous Seedance 2.5, 30-second, 16:9, 720p, native-audio task used `1,296,900` completion tokens.
- Estimated charge: `1,296,900 / 1,000 x 0.042 = CNY 54.4698`.
- This is an evidence-based estimate, not a fixed quote; final billing follows the successful task's returned usage.

## Paid gate

The user explicitly approved the estimated CNY 54.4698 submission on 2026-08-18. No further paid retry is authorized.

## Dry-run verification

The corrected client preview passed with network blocked and no task creation. Verified outgoing payload fields:

```json
{
  "model": "ep-20260812221158-hb576",
  "duration": 30,
  "generate_audio": true,
  "omni_reference_task_type": "extend",
  "priority": 0,
  "ratio": "adaptive",
  "resolution": "720p",
  "return_last_frame": true,
  "watermark": false
}
```

The prompt contains no stray boolean tokens. The first attempted client preview was discarded before submission because separated boolean arguments would have prefixed `true false` to the prompt and enabled the watermark. No network call or billable task occurred. The corrected syntax uses explicit `=true` / `=false` values.

## Outcome

- Primary task: `cgt-20260818145357-8t8g4` — succeeded technically, rejected in visual QA for a hard cut at approximately 23.375 seconds.
- Delayed duplicate: `cgt-20260818145533-jlz2g` — succeeded technically, rejected in visual QA for a hard cut at approximately 28.667 seconds.
- Both returned 30-second 1280x720 H.264 video, 24 fps, AAC stereo audio and a last frame.
- Each used 1,101,600 completion tokens. At the verified CNY 0.042 per 1,000 tokens, each cost CNY 46.2672; combined cost was CNY 92.5344.
- The duplicate was caused by a delayed silent task creation followed by the documented raw-interface fallback. The provider refused deletion while the duplicate was running.
- Neither output is approved as a visually seamless one-take master. See `SUBMISSION_RECORD.md` and `QA_REPORT.md`.
