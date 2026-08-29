# Apartment 106 Full Interior Completion V1

Status: `READY_FOR_BUDGET_APPROVAL_NOT_SUBMITTED`

## Completion target

Preserve the accepted 21.047-second exterior-to-apartment-entry film, then add six sequential ten-second interior legs. Expected final duration is approximately 81 seconds after exact boundary assembly.

The physical route follows the official Type 106 plan:

`entry -> separate laundry and main bathroom -> Bedroom 2 -> same-door exit -> kitchen axis -> MPR -> same-door exit -> Bedroom 1 -> WIR -> ensuite -> reverse through the same suite -> kitchen -> dining -> living -> terrace`

The MPR is reached from the kitchen/dining side, not directly from the entry hall.

## Why this replaces the failed method

The rejected R1 extension used two intermediate `reference_image` items. Seedance visibly dissolved from the bathroom reference into the corridor reference at about 6.4 seconds. The replacement tasks use `first_frame + last_frame` only. No intermediate still is supplied for the model to treat as an edit node.

## Six paid tasks

| Leg | Duration | Literal route | Mandatory endpoint |
| --- | ---: | --- | --- |
| A | 10s | actual 21.047s endpoint -> turn -> distinct laundry and main bathroom -> Bedroom 2 threshold | `apartment-106-k03-wet-area-to-bedroom2-connector-v3.png` |
| B | 10s | enter Bedroom 2 -> compact reveal -> visibly leave through same door -> regain kitchen axis -> MPR threshold | `apartment-106-k07-kitchen-to-mpr-threshold-v2.png` |
| C | 10s | enter compact MPR -> short reveal -> same-door exit -> continue through galley -> Bedroom 1 threshold | `apartment-106-k11-kitchen-to-bedroom1-threshold-v1.png` |
| D | 10s | enter Bedroom 1 -> establish bed/glazing -> continue physically through WIR -> ensuite | `apartment-106-wir-ensuite-cinematic-v1.png` |
| E | 10s | leave ensuite through WIR -> cross Bedroom 1 -> same-door exit -> regain kitchen -> clear island end -> dining | `apartment-106-dining-turn-cinematic-v2.png` |
| F | 10s | invitation -> camera passes resident -> living -> physically cross glazing threshold -> private terrace | `apartment-106-private-terrace-cinematic-v1.png` |

## Exact parameters for every task

- Endpoint: `ep-20260812221158-hb576`
- Model: `doubao-seedance-2-5-260628`
- Inputs: one `first_frame` plus one `last_frame`; no reference images or reference video
- Duration: `10`
- Ratio: `adaptive` inherited from 1280x720 frames
- Resolution: `720p`
- Generate audio: `true`
- Return last frame: `true`
- Watermark: `false`
- Priority: `0`
- Output count: exactly one
- Submit one task at a time; never retry automatically

Leg A starts with the actual preserved endpoint. Legs B-F start with the actual returned last frame of the preceding QA-passed leg, not a pre-generated approximation.

## Prompt source

The complete exact global prompt and all six leg instructions are in `PROMPTS.md`. The provider prompt for a leg is the unmodified Global Lock followed by that leg's unmodified section.

## Cost gate

Current verified rate: CNY 0.042 per 1,000 completion tokens.

- Closest previous ten-second first/last-frame task: 432,900 tokens = CNY 18.1818.
- Evidence-based six-task one-pass estimate: `6 x CNY 18.1818 = CNY 109.0908`.
- Conservative planning ceiling using the latest unusually expensive ten-second extension task: `6 x CNY 28.1232 = CNY 168.7392`.

The ceiling is not a fixed quote. Provider billing follows actual tokens. No retry is included. If any leg fails visual QA, the chain stops and unsubmitted downstream tasks incur no charge.

## Authorization phrase

`确认完成 Apartment 106 后续六段，一次通过预计 CNY 109.0908，单次任务逐段质检，总预算上限按 CNY 168.7392 控制，不自动重试。`

