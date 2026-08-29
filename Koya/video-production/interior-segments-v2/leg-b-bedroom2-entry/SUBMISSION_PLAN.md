# Leg B submission plan — Bedroom 2 entry

Status: `READY_FOR_SINGLE_PAID_SUBMISSION`

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Duration: 5 seconds
- Resolution: 720p
- Ratio: adaptive from 16:9 first and last frames
- Audio: synchronized native audio enabled
- Return last frame: enabled
- Paid task count: one

## Ordered inputs

1. `first:references/first-actual-31s-end.png` — exact generated endpoint from the accepted preceding segment.
2. `last:references/last-bedroom2-inside-v2.png` — corrected compact Bedroom 2 endpoint with the same open door retained on the right.

## Pricing gate

- Current Seedance 2.5 V2V 720p rate: CNY 0.042 per 1,000 completion tokens.
- Exact cost is unknown until the provider returns completion-token usage.
- The previous same-spec five-second task used 108,900 completion tokens, equal to CNY 4.5738; this is the working estimate only.
