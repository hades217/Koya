# Segment A lift-threshold bridge — paid approval gate

Status: `READY_FOR_USER_APPROVAL`

Seedance 2.5 rejects any request that combines a forced `last_frame` with a reference video or reference image. This bridge therefore uses only the provider-supported first-frame plus last-frame interpolation. The first frame is extracted from the final frame of the approved 0–8 second motion, so the join occurs on the same image.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Image 1 | `first_frame` | `references/01-foyer-lift-approach-start-frame-16x9.png` | Exact final frame of the approved 0–8 second Koya motion; camera is following the locked resident toward the lift. | `4fec06a4fc6560349a4c9d6906f6a131c4e5fe144c0761ef73aceead14553433` |
| Image 2 | `last_frame` | `references/02-inside-lift-end-frame-16x9.png` | Exact endpoint: camera and resident fully inside the same cabin, doors closed. | `afb6c0ce213b47765a405e348da551acdf2930ce043fc106959085837a4aeef1` |

## Exact parameters

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Provider duration: 5 seconds
- Ratio: 16:9
- Resolution: 720p
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Input mode: first frame + last frame only
- Planned final edit: join on the identical first frame; if full-speed review is too slow, time-compress this bridge to approximately 2.5–3 seconds without removing any threshold evidence
- Authorized scope if approved: exactly one bridge task; no automatic further retry and no Segment B/C task

## Exact prompt

```text
生成一条5秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。这不是独立蒙太奇，而是Koya一镜到底影片中从foyer走进同一台电梯的连续桥接镜头。

@图片1是强制首帧，必须精确从这一个画面、同一摄影机位置、同一35–40mm自然视角和同一向前动量开始。画面中只有同一名运动型白人女性：深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表；左手始终只有一只不锈钢水瓶。保持人物身份、身体比例、服装、左手水瓶、foyer材质、光线和屏幕方向不变。

人物继续快速而自然地向正前方同一台电梯走去。电梯门在她接近时完全打开，必须看到门洞、金属门槛以及foyer石材地面与轿厢地面连续相接。人物先跨过金属门槛。摄影机紧跟她右后方继续向前，亲自跨过同一条金属门槛并完全进入同一轿厢；左右门框和门槛向摄影机后方移出画面，证明摄影机不再位于foyer。

摄影机进入后继续向前约一步，在人物后方稳定。左右出现同一轿厢的浅木墙和黑色扶手。只有人物和摄影机都完全进入后，正前方同一扇金属门才关闭。门关闭只表示电梯即将上行，绝不能用门板遮挡偷换摄影机位置。

@图片2是强制末帧，必须精确到达：摄影机和人物都完全位于同一电梯内部；人物面向已经关闭的金属门；左右浅木墙和黑色扶手包围摄影机；foyer、走廊和门槛全部不可见。保持人物左手水瓶、衣服和身份完全一致。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、architecture morph、人物突然出现、人物换脸换衣、左右手交换、静态图片Ken Burns放大、门板遮满画面后偷换位置、摄影机留在电梯外、从foyer直接跳进轿厢。必须在连续运动中清楚看到人物和摄影机依次跨过同一金属门槛。

全程保持晴朗Brisbane晚上午、5200–5600K中性日光和一致白平衡；禁止黄昏、夜景、橙青电影色、塑料CG感、字幕、Logo、水印、旁白、对白和歌词。原生声音连续：石材上的快速跑鞋脚步、紧凑foyer轻微混响、电梯到达提示音、鞋底跨过金属门槛、轿厢内声场和最后的滑轨关门声；不得用音频断点掩盖画面切换。
```

## Current cost evidence

- Current `V2VCompletion` rate: CNY 0.042 per 1,000 completion tokens.
- A comparable 5-second Seedance task uses approximately 216,900 completion tokens.
- Estimated charge: `216,900 / 1,000 × 0.042 = CNY 9.1098`.

The estimate is not a fixed quote; final billing follows actual completion tokens.
