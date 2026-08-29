# Segment A Retry 1B — provider-compatible paid approval gate

Status: `READY_FOR_USER_APPROVAL`

The provider rejected Retry 1 before task creation because `last_frame` cannot be mixed with additional `reference_image` inputs. No task ID was created and no generation was queued. Retry 1B preserves the continuity-critical inputs and removes only the incompatible extra still references.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Video 1 | `reference_video` | `segment-a-passed-motion-0-8s.mp4` | Passed photorealistic aerial, Koya facade, resident, entrance and foyer motion. Ends before the failed lift threshold. | `b1067f6d64c81a172f89a2d866682944de1436631571f008744a47b7acb7c181` |
| Image 1 | `first_frame` | `references/01-world-start-16x9.png` | Exact first-frame world context. | `1dc848e7d1a753c31d926c0546137cfbe5515a2ecdb74c9e2ceaff81a41c1a4e` |
| Image 2 | `last_frame` | `references/06-locked-inside-lift-end-frame-16x9.png` | Exact endpoint: camera and resident fully inside the same cabin, doors closed. | `afb6c0ce213b47765a405e348da551acdf2930ce043fc106959085837a4aeef1` |

## Exact parameters

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Duration: 10 seconds
- Ratio: 16:9
- Resolution: 720p
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Full-modal task type: `reference`
- Authorized scope if approved: exactly one Retry 1B task; no automatic further retry and no Segment B/C task

## Exact prompt

```text
生成一条10秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。它是Koya完整30秒一镜到底看房影片的第一段。

@视频1是已经通过质检的0–8秒摄影机、Koya建筑、唯一运动女性、中央玻璃入口和紧凑foyer的最高连续性权威。尽量逐帧保持它的Brisbane/Toowong高空俯冲、Koya持续接近、人物身份服装、写实质感、中性白天色温、空间方向与摄影机动量。@视频1在人物到达电梯前结束；绝不能复制旧版本中“人物进入但摄影机留在电梯外”的错误。

@图片1是强制首帧。摄影机从这个Brisbane/Toowong高空世界沿固定目标轴快速向前下方飞行，河流、CBD、街区和道路产生真实三维视差；同一栋Koya已经存在于目标地块并在接近过程中持续变大，不得消失、替换或突然生成。约3秒到达街面，建筑外观、层数与@视频1保持一致。

运动型白人女性在街面构图形成时已经自然存在，与@视频1保持同一身份：深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表；左手始终只有一只不锈钢水瓶，右手开门。摄影机以35–40mm自然视角连续跟随，但不要贴着人物后背。

她右手打开中央玻璃门，摄影机亲自跨过门外地面、金属门槛和门内石材地面；门框连续经过画面两侧，不遮满画面。进入同一个紧凑foyer后快速走到同一台电梯。

最后2.0秒是本次最高优先级动作，必须严格完成：电梯门保持完全打开；人物先跨过电梯金属门槛。摄影机紧跟在她右后方，画面清楚看见foyer地面、金属门槛和轿厢地面连续相接。然后摄影机也实际向前跨过金属门槛并完全进入同一轿厢，左右门框和门槛必须向后移出镜头，证明摄影机不再位于foyer。摄影机进入后继续向前约一步并稳定在人物后方，左右两侧出现轿厢内的浅木墙和黑色扶手。只有当人物和摄影机都已完全在轿厢内后，同一扇金属门才在前方关闭。

@图片2是强制末帧，必须精确到达：摄影机已完全位于同一电梯内部，人物也在轿厢内面向已经完全关闭的同一扇门；左右浅木墙和黑色扶手包围摄影机，foyer、走廊和门槛均不可见。这个末帧将作为下一段电梯上行的第一帧。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、architecture morph、人物突然出现、静态图片Ken Burns放大、树叶擦镜、柱子擦镜、门板遮满画面偷换位置、人物后背遮满画面和过度运动模糊。尤其禁止人物进入电梯而摄影机留在外面；禁止在镜头跨过门槛之前关门；禁止从foyer视角直接跳到轿厢视角。

全程保持晴朗Brisbane晚上午、5200–5600K中性日光和一致白平衡；禁止黄昏、夜景、橙青电影色、塑料CG感、字幕、Logo、水印、旁白、对白和歌词。原生音频无缝连续：高空风声收窄为街道车辆与城市底噪，随后是跑鞋脚步、玻璃门五金、石材脚步、foyer混响、电梯提示音、脚步跨过金属门槛和最后的滑轨关门声；不得用音频断点掩盖画面切换。
```

## Current cost evidence

- Current `V2VCompletion` rate: CNY 0.042 per 1,000 completion tokens.
- The immediately preceding 10-second full-modal Segment A used 432,900 completion tokens.
- Like-for-like estimate: `432,900 / 1,000 × 0.042 = CNY 18.1818`.

The estimate is not a fixed quote; final billing follows actual completion tokens.
