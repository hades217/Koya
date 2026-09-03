# 完整输入内容审计

中文生成提示词见 `PROMPT_ZH_DRAFT.txt`；图片逐项尺寸、哈希和画面职责见 `IMAGE_REVIEW_MANIFEST.md`。

## 必须进入人工审核包的全部内容

| 内容 | 用途 | 当前状态 |
|---|---|---|
| Apartment 106 official floor plan | 拓扑真相 | 存在 |
| Public-zone floor-plan crop / route overlay | 起点、方向、门槛和终点 | 存在 |
| `00-master-public-zone-lock.png` | 公共区整体视觉身份 | 存在；非时间线帧 |
| `01-mpr-dining-opening.png` | 0.0 秒 MPR/Dining | 存在 |
| `02-dining-single-island-kitchen.png` | 0.8 秒 Dining/Kitchen | 存在 |
| `03-tv-wall-living-width.png` | 1.6 秒 TV/Living width | 存在 |
| `04-wrap-glazing-sliding-bay.png` | 2.4 秒 sofa/glazing | 存在 |
| `05-align-east-sliding-bay.png` | 3.2 秒 align opening | 存在 |
| `06-threshold-approach.png` | 4.0 秒 threshold | 存在 |
| `07-cross-east-threshold.png` | 4.8 秒 crossing | 存在 |
| `08-terrace-arrival.png` | 6.0 秒 arrival | 存在 |
| `09-terrace-lookback-endpoint.png` | 8.0 秒 endpoint | 存在 |
| 8 秒固定场景连续参考视频 | Seedance 时间/空间控制 | **缺失** |

## Seedance 实际 provider inputs

当前允许的完整路线输入应当是：

| 输入 | Provider role | 状态 |
|---|---|---|
| 完整 8 秒固定场景连续参考视频 | `reference_video` | **缺失，阻塞提交** |

九张 V7 图片是独立生成的渲染级分镜。它们必须全部用于人工审核、Prompt 覆盖和生成后逐项验收，但不能冒充 Seedance 的有序关键帧输入；当前接口没有证明会按 1→9 执行 `reference_image`。把它们全部作为普通 whole-room `reference_image` 上传会制造互相矛盾的房间透视和门窗关系，预审必须阻止。

## 禁止输入

- V6 Seedance rejected output。
- `reference-video-v8-threejs`。
- `reference-video-v9-planlocked`。
- 已拒绝的 I2V V3 `cgt-20260829114235-c9k4n`。
- 任何由独立图片放大、交叉溶解或幻灯片组成的伪参考视频。
