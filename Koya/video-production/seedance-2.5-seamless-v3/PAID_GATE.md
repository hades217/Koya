# Paid generation gate

Current authorisation: **none**.

The previous approval applied only to task `cgt-20260817115116-j8jxl` and is
consumed. It does not authorise v3, retries or later extension legs.

Before Leg A submission, record:

- provider account/profile and endpoint;
- model identity and running state;
- exact references with checksums;
- full prompt;
- complete request payload;
- current unit price;
- estimated Leg A charge;
- estimated maximum 480p preview-pass charge;
- user's explicit approval message;
- submitted task ID or proof that no task was created.

No automatic retry is permitted. If a call times out, query the provider task
list before doing anything billable.

## Live read-only verification — 2026-08-17

- Active profile: `platform_cn-beijing_accountwide`.
- Endpoint: `ep-20260812221158-hb576`.
- Endpoint state: `Running`.
- Bound model: `doubao-seedance-2-5-260628`.
- Supported task types include multimodal-to-video and video extension.
- Current account rate: `NV2VCompletion` CNY 0.07 / 1,000 tokens;
  `V2VCompletion` CNY 0.042 / 1,000 tokens.
- 480p 16:9 output: 854×480.
- Official estimate formula: `(input video seconds + output video seconds) ×
  width × height × fps ÷ 1024`.

At 854×480 and 24fps, the estimate is 9,607.5 tokens per billed video second.

| Leg | Billed seconds under v3 | Estimated tokens | Estimated CNY |
| --- | ---: | ---: | ---: |
| A, no video input | 5 | 48,037.5 | 3.362625 |
| B, 5s prior video + 7s output | 12 | 115,290 | 4.842180 |
| C, 7s prior video + 4s output | 11 | 105,682.5 | 4.438665 |
| D, 4s prior video + 6s output | 10 | 96,075 | 4.035150 |
| E, 6s prior video + 8s output | 14 | 134,505 | 5.649210 |
| **Maximum one-pass preview if all five first attempts are approved** | **52** | **499,590** | **22.327830** |

These are estimates before submission. Actual charge is determined by the
successful task's returned usage. Approval must begin with Leg A only; the full
preview estimate is not permission to submit all five legs automatically.
