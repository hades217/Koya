# Seedance I2V V4 实验提交清单

状态：`PREFLIGHT PENDING / NOT SUBMITTED`

## 实验边界

这是一次无 `reference_video` 的单首帧 I2V 实验，只验证同一可见公共空间中的真实环视、向右侧露台门步行和跨越门槛。它不包含 MPR，也不声称完成完整 Clip EF 路线。

## 精确任务

- Task count: `1`
- Output count: `1`
- Endpoint: `ep-20260812221158-hb576`
- Live resolved model: `doubao-seedance-2-5-260628`
- Mode: `first_frame` image-to-video
- Duration: `8 seconds`
- Resolution: `1080p`
- Aspect ratio: inherited from 1672x941 first frame; do not send `--ratio`
- Generate audio: `true`
- Watermark: `false`
- Prompt: `PROMPT_ZH.txt`
- Output directory: `outputs/`

## 唯一生成输入

- Provider role: `first_frame`
- File: `../living-panorama-storyboard-v2/rendered-storyboard-v7-plan-rebuild/00-master-public-zone-lock.png`
- Dimensions: `1672x941`
- SHA-256: `7c7e32ea154100a1dc8ba891b6bbe15b3a3d98b6175409e70f636bbfa09f4215`

其他 01–09 分镜不上传给 provider，避免普通 `reference_image` 角色引入互相冲突的空间透视。

## 价格证据

- Pricing model: `doubao-seedance-2-5`
- Live pricing lookup time: 2026-08-29 Australia/Brisbane
- Account returned `IsOverdue: true`.
- Pricing response did not expose a clearly named I2V 1080p charge item; exact task estimate: `unavailable`.
- Actual charge, failed-task billing and rejection billing: `unavailable` until provider records usage.
