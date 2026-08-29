# Leg A submission plan

Status: `READY_FOR_SINGLE_PAID_SUBMISSION`

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Duration: 5 seconds
- Resolution: 720p
- Ratio: adaptive from 16:9 first and last frames
- Audio: synchronized native audio enabled
- Output: MP4, no watermark
- Task type: multimodal reference
- Return last frame: enabled
- Number of paid tasks: one

## Ordered generation inputs

1. `first:references/first-actual-26s-end.png` — mandatory exact first frame.
2. `last:references/last-bedroom2-threshold-v4.png` — mandatory physical destination.

The locally retained motion tail is QA context only and is intentionally excluded from the paid request. This avoids charging a V2V extension against the accumulated master and matches the previously successful five-second first/last-frame request shape.

## Pricing

- Current Seedance 2.5 V2V 720p rate: CNY 0.042 per 1,000 completion tokens.
- Exact task cost is unknowable before the provider returns completion-token usage.
- Historical comparable 5.056-second segment used 108,900 completion tokens, equal to CNY 4.5738 at the current rate.
- This is a planning estimate only; final cost must be calculated from the returned task usage.
