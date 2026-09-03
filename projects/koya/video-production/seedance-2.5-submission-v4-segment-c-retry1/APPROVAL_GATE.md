# Segment C Retry 1 — corrected single-door apartment entry

Status: `READY_FOR_USER_APPROVAL`

Continuity contract: `literal_walkthrough`. The first paid Segment C was rejected because its incompatible endpoint caused a duplicated entrance door. This retry replaces that endpoint with a floor-plan-constrained frame whose single open door remains on the correct left side and whose camera is fully inside the apartment.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Image 1 | `first_frame` | `references/01-apartment-106-closed-door-start-16x9.png` | Actual final generated frame of passed Segment B; one door, left hinges, black lever/lock on the right edge. | `8ac6040b4b5ddfabd8f6031360ca0a2ebe63c1aba57e08abbdb7ed305483aa68` |
| Image 2 | `last_frame` | `references/02-inside-entry-single-door-left-end-16x9.png` | Corrected concept endpoint built from the official Type 106 floor plan, official Koya material language and locked resident. Camera is fully inside; the only entrance door is behind at far left; right wall is clean. | `50419736db35fe40865770c135e4e2efa2a4f930e06c918c14d0526d0f747ce2` |

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
- Authorized scope if approved: exactly one Segment C Retry 1 task; no automatic second retry and no Segment D task

## Exact prompt

```text
生成一条5秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。紧接Koya Segment B真实末帧，同一位运动型白人女性站在Apartment 106唯一一扇关闭的浅木入户门外，摄影机在她正后方。

@图片1是强制首帧和门体几何真相：这扇门只有一个门板，合页固定在画面左侧，黑色门锁与把手固定在门板右侧自由边。人物右手下压同一个把手并将门向Apartment 106室内打开。门板围绕左侧合页连续向室内左侧旋转；黑色锁和把手必须始终牢固附着在同一门板右侧自由边，并与门板一起向左移动，绝不能留在右墙、复制到右墙或生成第二个门板。

开门过程中始终能追踪同一个门板、同一组合页和同一门把。人物先跨过门槛，摄影机紧跟并亲自跨过同一门槛。走廊地毯、金属门槛和室内浅木地板必须连续相接；摄影机越过后，金属门槛和走廊地毯自然退到镜头后方，不再出现在最终画面。

人物和摄影机进入Type 106短而紧凑的entry hall。她放开右手门把并快速向前走；不锈钢水瓶始终只在左手。同一扇打开的入户门留在摄影机左后方，最终只允许在画面最左边看到一小段与左合页相连的门板。画面右侧从始至终只能是连续的米白墙面和木踢脚线，绝对不能出现木门框、黑色门锁、门把、锁舌、第二扇门或第二个入口。

严格遵守Apartment 106平面图的紧凑尺度：短窄玄关连接开放式厨房；左侧可有纤细收纳，右前方只开始看到暖木厨房柜体和浅色石材岛台。不是大型豪宅门厅，不是酒店长廊，不在入口旁增加MPR，不增加房间、楼梯、双层挑高或额外玻璃。

@图片2是强制末帧。摄影机已经完全进入Apartment 106，前景只有浅木地板，不再看见走廊地毯或金属门槛；人物在约1.5米前继续向厨房方向走；唯一打开的入户门只在最左边保留窄边和左合页；右墙完整、纯净、没有任何入户门五金。该帧作为下一段进入厨房和客餐厅的首帧。

唯一人物保持深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋和黑色运动手表；水瓶始终左手。摄影机保持同一35–40mm自然视角、约1.6米高度、同一前进方向。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、door wipe、body wipe、architecture morph、门把脱离门板、门锁留在右墙、复制门锁、复制门板、第二入口、穿墙、换脸换衣、左右手交换、水瓶消失、超大玄关、入口MPR、静态图片Ken Burns放大、字幕、Logo和水印。门板经过画面时不得成为隐藏剪辑点。

全程保持同一中性Brisbane晚上午白平衡。原生声音连续：门把下压、门锁释放、同一门板围绕合页开启、鞋底从地毯跨过金属门槛再踏上木地板、紧凑室内混响；不得用音频断点掩盖画面跳转。
```

## Cost and prior failed charge

- Rejected Segment C task `cgt-20260817220121-98zlr` completed and cost CNY 4.5738. It will not be used.
- Verified rate: CNY 0.042 per 1,000 completion tokens.
- Like-for-like retry estimate: `108,900 / 1,000 x 0.042 = CNY 4.5738`.
- No retry task has been created yet.

Final billing follows actual completion tokens.
