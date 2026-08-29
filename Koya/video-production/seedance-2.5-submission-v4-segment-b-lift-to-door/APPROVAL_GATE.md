# Segment B — same lift to Apartment 106 door

Status: `READY_FOR_USER_APPROVAL`

Continuity contract: `literal_walkthrough`. This segment starts on the actual final frame of the passed Segment A lift bridge and ends at the Apartment 106 door before it opens.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Image 1 | `first_frame` | `references/01-inside-lift-closed-start-16x9.png` | Actual final generated frame of task `cgt-20260817213439-7cgx2`; same camera, resident and cabin. | `f385c6065da8e3d8c5b075710d0982157ccd3459b44e255d91001312dcc6c6d2` |
| Image 2 | `last_frame` | `references/02-apartment-106-door-end-16x9.png` | Locked endpoint at the Apartment 106 door before entry. | `5aa309c741fda46b92f8b29087c7cf6e55d6557d9481bf0069f148bb20e5f26a` |

## Exact parameters

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Duration: 5 seconds
- Output ratio: inherited from 1280×720 first frame (16:9); no explicit `ratio` field
- Resolution: 720p
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Inputs: first frame + last frame only
- Authorized scope if approved: exactly one Segment B task; no automatic retry and no Segment C task

## Exact prompt

```text
生成一条5秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。它紧接Koya上一段摄影机和人物已经完全进入同一电梯后的真实末帧。

@图片1是强制首帧，必须精确从同一摄影机位置、同一35–40mm自然视角、同一浅木墙和黑色扶手的电梯轿厢开始。唯一人物保持深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表；不锈钢水瓶始终只在左手。摄影机始终位于人物后方并留在同一轿厢内。

开始时金属电梯门保持完全关闭约0.4秒，只压缩真实上行时间；关门期间摄影机、人物、镜头方向和轿厢身份绝不能变化。随后同一扇门从正前方自然打开，门外是与轿厢直接相邻的Level 1紧凑住宅走廊，白色墙面、浅木门套、暖中性顶灯和浅灰低绒地毯。

人物先向前跨过同一电梯门槛，摄影机紧跟并亲自跨过门槛。必须看到轿厢地面、金属门槛和走廊地毯连续相接；左右电梯门框和木质轿厢墙向摄影机后方退出画面，证明摄影机已经从同一轿厢走到Level 1，而不是直接跳到走廊。

人物和摄影机沿同一条短而紧凑的走廊快速前进，不停顿、不绕路、不经过大型大厅。保持人物在画面中部偏前，摄影机不贴住人物后背。她走到Apartment 106的单扇浅木入户门前，用右手握住黑色门把手，左手仍拿不锈钢水瓶；门尚未打开。

@图片2是强制末帧，必须精确到达：同一人物站在Apartment 106门前，右手握住黑色门把，左手水瓶可见，摄影机仍在她后方，走廊尺度紧凑。这个末帧将作为下一段实际开门并跨入Apartment 106的首帧。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、door wipe、architecture morph、人物突然出现、换脸换衣、左右手交换、摄影机瞬移到走廊、重复进入电梯、第二部电梯、酒店式超长走廊、静态图片Ken Burns放大、门牌数字乱码、字幕、Logo和水印。关闭的电梯门只能压缩上行时间，不能隐藏地点或摄影机变换。

全程保持与上一段相同的中性晚上午白平衡；禁止夜景、黄昏和橙青电影色。原生声音连续：轿厢低频运行声和轻微提示音、同一扇门开轨声、鞋底跨越金属门槛、地毯脚步、紧凑走廊混响，以及右手轻触门把的金属声；不得用音频断点掩盖画面跳转。
```

## Cost estimate

- Verified rate: CNY 0.042 per 1,000 completion tokens.
- The immediately preceding 5-second first-last-frame bridge used 108,900 completion tokens.
- Like-for-like estimate: `108,900 / 1,000 × 0.042 = CNY 4.5738`.

Final billing follows actual completion tokens.
