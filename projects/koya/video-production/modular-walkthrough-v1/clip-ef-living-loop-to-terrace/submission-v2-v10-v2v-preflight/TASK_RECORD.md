# Clip EF V10 V2V task record

Status: `REJECTED_AFTER_USER_REVIEW`

- Approved package fingerprint: `378668769e2a88b219222131f36df2f3164017615cf084d800174154bc666fda`.
- One create call only.
- Task ID: `cgt-20260829175719-2hwhz`.
- Endpoint/model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`.
- Mode: video editing with one `reference_video` and zero `reference_image` inputs.
- Provider terminal status: `succeeded`.
- Local output: `outputs/cgt-20260829175719-2hwhz.mp4`.
- Output SHA-256: `dfecc7954781d23ee42bb0870d49d3c57efc00a9254d6796a0ef15a583d8a6ff`.
- Returned usage: 763,425 completion tokens.
- Cost calculated from the locked live unit price CNY 0.03312 per 1,000 V2V1080 completion tokens: **CNY 25.284636**.
- Provider billing read-back may lag; the calculation is based on returned usage and the preflight unit price.
- No retry, variant or second candidate was submitted.

The earlier `USER_ACCEPTED` interpretation was withdrawn after the project owner reviewed the exact EF segment in the assembled timeline. The clip is rejected because the provider package used the V10 3D reference video as its only visual input and supplied zero rendered storyboard images; the output therefore retained the unacceptable 3D/reference-design character and did not follow the approved rendered visual direction.

The task remains valid billing/history evidence but is forbidden as an accepted edit source or future reference input. The preserved rejected copy is `../outputs/rejected/clip-ef-v10-v2v-rejected-after-user-review.mp4`. No retry is authorised by this status correction.

The temporary HTTPS reference-video tunnel and local server were shut down only after the provider reached `succeeded` and the output was downloaded and hashed locally.
