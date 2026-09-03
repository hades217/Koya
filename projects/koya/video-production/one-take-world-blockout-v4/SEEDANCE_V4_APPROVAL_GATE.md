# Koya complete one-take — Seedance 2.5 V4 approval gate

Status: `READY_FOR_CONTROL_PREVIEW_APPROVAL`

The final deliverable is one continuous 30-second film. Production is sequential:

1. Segment A, 0–10 s: Brisbane/Toowong dive → persistent Koya facade → same resident → glass entrance → foyer → same lift; finish inside with doors closed.
2. Segment B, 10–20 s: inherit A's final frame and tail motion → same closed lift → Level 1 → Apartment 106 door visibly opens → camera crosses threshold; finish inside the apartment.
3. Segment C, 20–30 s: inherit B's final frame and tail motion → compact kitchen → dining → living → balcony outlook.

The 10 s and 20 s boundaries are production handoffs only. Each successor must receive the predecessor's returned last frame plus its final 1–2 seconds as motion reference. The assembly uses matched overlap frames and continuous audio ambience; it must not contain a visible cut, crossfade, wipe, occlusion, teleport or static-image zoom.

## Paid task A — exact inputs

| Order | Provider role | Local source | Purpose |
| ---: | --- | --- | --- |
| Video 1 | `reference_video` | `segment-a-camera-control-0-10s-16x9.mp4` | Sole route, speed, threshold and camera-motion authority. Rendered from the approved HyperFrames control. |
| Image 1 | `reference_image` | `../seedance-2.5-submission-v3/references/01-world-start-16x9.png` | Brisbane/Toowong world relationship; Koya exists on the target site from frame one. |
| Image 2 | `reference_image` | `../seedance-2.5-submission-v3/references/02-official-facade-16x9.jpg` | Sole exterior architecture authority. |
| Image 3 | `reference_image` | `../seedance-2.5-submission-v3/references/03-character-back-16x9.png` | Locked resident identity, wardrobe and left-hand bottle. |
| Image 4 | `reference_image` | `../seedance-2.5-submission-v3/references/04-entry-foyer-16x9.png` | Compact entrance and single-lift foyer concept. |
| Image 5 | `reference_image` | `../seedance-2.5-submission-v3/references/05-lift-entry-16x9.png` | Same-lift threshold and cabin appearance. |

All images are reference images, not first-frame inputs. The camera-control video controls motion only and must not contribute low-poly appearance.

## Paid task A — exact parameters

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Task type: full-modal reference video generation
- Duration: 10 seconds
- Ratio: 16:9
- Resolution: 720p
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Paid scope: exactly one Segment A task; no automatic retry and no Segment B/C submission

## Paid task A — exact prompt

```text
生成一条10秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。它是完整30秒影片的第一段，但本段内部必须是single uninterrupted take、zero edits。

唯一摄影机从Brisbane/Toowong高空沿固定目标轴快速向前下方俯冲，同一栋Koya从第一帧起已经真实存在于目标地块并持续变大。0–3秒完成高空到街面的真实空间飞行，有河流、街区、道路和建筑的真实三维视差；不绕飞、不穿树叶、不用云雾或前景遮挡。3秒时清楚看见完整Koya官方立面、正确总层数、圆角阳台、木格栅、栏杆、中央入口、右侧车道和屋顶绿化。

运动型白人女性在街面构图形成时已经自然存在于前方，绝不能突然出现、淡入、从空气中生成或被另一人物替换。她保持深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋和黑色运动手表；左手始终只有一只不锈钢水瓶，右手开门。摄影机保持自然35–40mm视角，可稍微偏离人物以同时看清建筑和入口，不需要贴着后背。

摄影机连续跟随她快步走到中央玻璃入口。她右手开门，摄影机亲自跨过门外地面、金属门槛和门内石材地面，门框自然经过画面两侧，不能遮满画面偷换空间。随后在同一个紧凑foyer快速到达同一台单电梯。人物先跨过电梯金属门槛，摄影机紧随进入同一轿厢；进入过程中同时看见foyer、门框、门槛和轿厢地面，证明摄影机实际进入。最后约0.7秒，同一扇电梯门在人物和摄影机面前连续关闭。本段最后一帧必须稳定停在同一轿厢内部、门已完全关闭、人物仍在原位置，为下一段从完全相同画面继续上行。

@视频1只控制唯一摄影机的路线、速度、方向、跟拍距离、所有门槛时序以及最后关闭电梯门的连续动作；绝不复制低多边形、白模、人物模型或材质。@图片1控制Brisbane/Toowong和目标地块的白天世界关系。@图片2是Koya外观和几何的最高权威。@图片3锁定唯一人物。@图片4控制玻璃入口和紧凑单电梯foyer。@图片5控制同一台电梯的门槛和轿厢。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、architecture morph、人物突然出现、静态图片Ken Burns放大、树叶擦镜、柱子擦镜、门板遮满画面、贴脸遮挡、人物后背遮满画面和过度运动模糊。参考图片必须融合成一个连续三维世界，不能逐张播放。全程晴朗Brisbane晚上午、5200–5600K中性日光、同一太阳方向和白平衡；禁止黄昏、夜景、橙青电影色、塑料CG感、字幕、Logo、水印、旁白、对白和歌词。

原生音频连续：高空风声自然收窄为街道车辆与城市底噪，然后是跑鞋脚步、玻璃门五金、石材脚步、紧凑foyer混响、电梯提示音和滑轨关门声；极轻现代无歌词氛围音乐保持在环境声下方，不得用音频断点掩盖画面切换。
```

## Current provider-backed price evidence

Checked 2026-08-17 Australia/Brisbane:

- `doubao-seedance-2-5` `V2VCompletion`: CNY 0.042 per 1,000 completion tokens.
- The rejected prior 30-second task used 1,296,900 completion tokens and cost CNY 54.4698.
- Linear 10-second planning estimate: `1,296,900 ÷ 3 ÷ 1,000 × 0.042 = CNY 18.1566`.

This is an estimate, not a fixed quote. The provider charges the new task's actual completion tokens. One-pass planning estimate for A+B+C is approximately CNY 54.4698. A failed segment retry is planned at approximately CNY 18.1566 rather than regenerating all 30 seconds.

## Approval required

Approval authorizes only one paid Segment A generation at the exact inputs, prompt and parameters above. It does not authorize Segment B, Segment C or any retry.
