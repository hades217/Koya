# Segment C — Apartment 106 door to compact entry hall

Status: `READY_FOR_USER_APPROVAL`

Continuity contract: `literal_walkthrough`. This segment starts on the actual final generated frame of passed Segment B and ends inside Apartment 106 after the camera has visibly crossed the real entry threshold.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Image 1 | `first_frame` | `references/01-apartment-106-closed-door-start-16x9.png` | Actual final generated frame of task `cgt-20260817214856-d4dx5`; same camera, resident, corridor and closed door. | `8ac6040b4b5ddfabd8f6031360ca0a2ebe63c1aba57e08abbdb7ed305483aa68` |
| Image 2 | `last_frame` | `references/02-inside-compact-entry-hall-end-16x9.png` | Active Apartment 106 compact-entry endpoint, constrained by the official Type 106 floor plan and Koya interior language. | `7e39bc410f8c0f1e3f3701942dc456959724972795b4f0211624f67c2641fbbb` |

## Exact parameters

- Endpoint: `ep-20260812221158-hb576`
- Resolved model: `doubao-seedance-2-5-260628`
- Duration: 5 seconds
- Output ratio: inherited from 1280x720 first frame (16:9); no explicit `ratio` field
- Resolution: 720p
- Native audio: enabled
- Return last frame: enabled
- Priority: 0
- Inputs: first frame + last frame only
- Authorized scope if approved: exactly one Segment C task; no automatic retry and no Segment D task

## Exact prompt

```text
生成一条5秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。它必须紧接Koya Segment B的真实末帧：同一位运动型白人女性站在Apartment 106关闭的浅木入户门外，摄影机在她正后方，右手已握住黑色门把，不锈钢水瓶始终在左手。

@图片1是强制首帧。人物用右手自然下压同一个黑色门把并把同一扇门向内打开。门打开过程中，门扇、合页侧、锁侧门框、走廊地毯、金属门槛和室内浅木地板必须保持连续可见，不能在门移动时替换空间。

人物先真实跨过Apartment 106的门槛，摄影机紧跟在后并亲自跨过同一门槛。必须清楚看到走廊地毯连接金属门槛再连接室内浅木地板；门框从画面两侧连续向摄影机后方退出，证明摄影机已经进入住宅，而不是穿墙或利用门扇遮挡切换。

进入后人物放开门把并快速向前走入短而窄的住宅entry hall。严格遵守Type 106户型的紧凑尺度：这只是从入户门连接开放式厨房和客餐厅的短入口，不是大型豪宅门厅，不是酒店长廊，不增加第二个foyer，也不在入口旁虚构大型MPR。浅木收纳、米白墙面和浅木地板沿同一路线连续延伸，开放式厨房只在前方开始被看见，暂时不完整展示整个客厅。

@图片2是强制末帧。到达时同一人物和摄影机都已经完全位于Apartment 106内部；打开的入户门仍留在摄影机右后侧边缘，人物在短入口内向前走，右前方开始出现暖木厨房柜体和浅色石材岛台，前方保持中性晚上午日光。这个末帧将作为下一段连续进入厨房、餐厅和客厅的真实首帧。

唯一人物必须保持深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表；不锈钢水瓶始终只在左手。摄影机保持同一35–40mm自然视角、约1.6米高度和同一前进动量，不贴住人物后背。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、door wipe、body wipe、architecture morph、穿墙、门后瞬移、人物突然出现、换脸换衣、左右手交换、水瓶消失、第二扇入户门、超大玄关、酒店长廊、在入口虚构大型MPR、静态图片Ken Burns放大、字幕、Logo和水印。门扇经过镜头时也绝不能成为隐藏剪辑点。

全程保持与上一段相同的中性Brisbane晚上午白平衡，禁止夜景、黄昏和橙青电影色。原生声音连续：右手下压金属门把、门锁释放、木门合页轻响、鞋底从地毯跨过金属门槛再踏上木地板、紧凑室内自然混响；不得用音频断点掩盖画面跳转。
```

## Cost estimate

- Verified rate: CNY 0.042 per 1,000 completion tokens.
- Both immediately preceding like-for-like 5-second first-last-frame tasks used 108,900 completion tokens.
- Like-for-like estimate: `108,900 / 1,000 x 0.042 = CNY 4.5738`.

Final billing follows actual completion tokens.
