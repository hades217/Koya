# Seedance preflight review — Clip EF ten-image V3

Decision: `PASS`

Reviewed at: 2026-08-29 16:51:54 AEST

No task submitted: yes

Package fingerprint: `sha256:59f650fa4abc9c98acec91394e415f67177ff664e85a380883fb556e3b935b1b`

## Blockers

- None. The project owner explicitly selected one image-only experimental continuity task with no `reference_video`.
- Continuity is required by the prompt but is not provider-guaranteed. The result must be rejected if frame-level QA finds a cut, dissolve, morph, teleport, topology drift or wall/glass/furniture penetration.

## Exact task

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Live endpoint state: `Running`; `multimodal_to_video` supported
- Mode: `multimodal_to_video`
- Task count / output count: `1 / 1`
- Parameters: 8 seconds; 16:9 requested in the Chinese prompt and reinforced by ten 1672×941 inputs; no explicit `--ratio` flag; 1080p; generated audio; return last frame; no watermark
- Output directory: `outputs/clip-ef-all-images-v3`

## Inputs and roles

All inputs are sent in the listed order with the explicit provider role `reference_image`:

| Order | Input | Role | SHA-256 |
|---:|---|---|---|
| 00 | `00-master-public-zone-lock.png` | `reference_image` | `7c7e32ea154100a1dc8ba891b6bbe15b3a3d98b6175409e70f636bbfa09f4215` |
| 01 | `01-mpr-dining-opening.png` | `reference_image` | `bbcc062237718b052ae746d6a828a08f5f9bf688cbe1939c28dbd018f343e31e` |
| 02 | `02-dining-single-island-kitchen.png` | `reference_image` | `8e23b68b9c782a827581bb1b4647854d4f7d9b03e520bcf2fb5db5723c72c7e4` |
| 03 | `03-tv-wall-living-width.png` | `reference_image` | `d7cac55298cb7464b2a3d2aa975523466af99d70526efa0357255995df57b1b3` |
| 04 | `04-wrap-glazing-sliding-bay.png` | `reference_image` | `818e85e32ccd7a00546ae0de2a2d763a5e63fafd55a432e41ffca81cf740df5d` |
| 05 | `05-align-east-sliding-bay.png` | `reference_image` | `ece6e45c5045f8906abe5bc3881512e258e3f8d27e4c305e4cac6a11904eac55` |
| 06 | `06-threshold-approach.png` | `reference_image` | `5153f7526be5ed71bf2bbedc3c823c996b89be2273895b23a0cb2dbf57d7a681` |
| 07 | `07-cross-east-threshold.png` | `reference_image` | `f2057bff4f0268357b15794b0f02db08ee24df2e9f311ba86478646a08797e77` |
| 08 | `08-terrace-arrival.png` | `reference_image` | `99411967ada9add57a81d8df13dd11ee138f1fc60e301fb5d51944e5f4b7f55c` |
| 09 | `09-terrace-lookback-endpoint.png` | `reference_image` | `5ff979687d6379a467bcd2db36eda657a27b41f43ce74f8446b0b160c96ab202` |

- Every provider URL returned a byte-identical PNG matching the locked local SHA-256.
- No `first_frame`, `last_frame` or `reference_video` input is included. The prompt requests image 00 as the opening composition and image 09 as the endpoint; it does not claim they are enforced keyframes.

## Prompt and route review

- Exact prompt: `PROMPT_ZH_ALL_IMAGES.txt`; SHA-256 `b3145ff0655b62d37439b665fb1af453f762ab55ea74d054d963f428a10a862b`
- Timed route totals exactly 8.00 seconds: public-zone opening → MPR/Dining glance → Kitchen/single island → TV wall/Living width → wrap glazing → east sliding bay → approach → threshold crossing → Terrace arrival → Terrace look-back.
- Persistent topology constraints cover the MPR opening, one island, TV wall, sofa orientation, continuous south/east glazing, one east sliding opening, one threshold and the same Terrace.
- Requested camera motion is physical translation and rotation, not a slideshow or static push-in.
- Non-billable dry-run: `PASS`; exact payload contains one prompt and ten ordered `reference_image` inputs, duration 8, resolution 1080p, generated audio and return-last-frame.

## Provider validation history

- Earlier role mix `reference_image` + `last_frame` was rejected before task creation: last-frame content cannot be mixed with reference media.
- Earlier role mix `first_frame` + `reference_image` was rejected before task creation: first/last-frame content cannot be mixed with reference media.
- Those parameter rejections created no task and showed no new usage at their verification time.
- Current pre-submit baseline: 22 existing tasks; latest existing task `cgt-20260829114235-c9k4n` is already succeeded. No residual `arkcli +gen` process is running.

## Provider and price evidence

- Authentication: active.
- Account overdue state: false from the refreshed provider check performed for this package.
- Exact ten-image multimodal-to-video price: `unavailable` before completion usage.
- Whether provider rejection/failure is billable: `unavailable`.

## Approval gate

This PASS does not submit a task. One approval authorizes exactly one paid task and one output. It does not authorize an automatic retry.

Exact approval phrase: **确认提交 Clip EF 十图 V3，指纹 59f650fa4abc，10 张图全部为 reference_image，不使用 reference video，8 秒、1080p、16:9 提示词要求、生成音频、单任务单输出，费用 unavailable，接受连续性由逐帧 QA 判断且不自动重试。**
