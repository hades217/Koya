# Segment D — compact entry hall to kitchen and dining reveal

Status: `READY_FOR_USER_APPROVAL`

Continuity contract: `literal_walkthrough`. This segment starts on the actual final generated frame of passed Segment C Retry 1 and follows the resident along the right side of the same kitchen island into the open-plan dining zone.

## Exact input manifest

| Order | Provider role | File | Authority | SHA-256 |
| ---: | --- | --- | --- | --- |
| Image 1 | `first_frame` | `references/01-inside-entry-hall-start-16x9.png` | Actual final generated frame of task `cgt-20260817221551-zv9sf`; same camera and resident fully inside Apartment 106. | `f47cf778e80f2fd1fab9fb5f295827490e0f5140a3e821c53db46d07f3edd4b5` |
| Image 2 | `last_frame` | `references/02-open-plan-kitchen-dining-end-16x9.png` | Active Type 106 open-plan concept endpoint; floor plan controls room relationship and Koya renders control material language. | `daf71f36c7e04d9291139c80a1abba2d2da142ad15e95bebd83c26903e088e2a` |

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
- Authorized scope if approved: exactly one Segment D task; no automatic retry and no Segment E task

## Exact prompt

```text
生成一条5秒、16:9、720p、带原生环境声音的高端写实房地产销售影片。single uninterrupted take; zero edits。它紧接Koya Segment C Retry 1的真实末帧：同一位运动型白人女性和同一摄影机都已经完全进入Apartment 106短玄关，前方右侧是同一座浅色石材厨房岛台。

@图片1是强制首帧。保持同一35–40mm自然镜头、约1.6米高度和持续前进动量。人物快速沿同一座岛台的右侧通道向前走，摄影机保持在她后方并沿同一条通道跟进。摄影机必须从岛台右侧绕过岛台靠近玄关的一端，不能穿过石材、柜体、墙面或人物身体。

空间变化必须由真实前进产生：开始时岛台位于画面右前方；随着摄影机沿其右侧经过，岛台端部产生真实近景视差并逐渐移到画面左侧；同一排暖木厨房柜体连续留在岛台后方；餐桌和客厅从前方逐步展开，而不是突然替换成另一个房间。

严格遵守Type 106开放式公共区关系：厨房、岛台、餐厅、客厅和露台在同一连续开放空间内。只展示厨房和餐厅的空间建立，客厅保持在更远处，暂时不到露台门。不得增加第二座岛台、第二个厨房、第二层空间、楼梯、超大挑高、入口MPR或酒店大堂。

人物保持深棕马尾、鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表；不锈钢水瓶始终只在左手。她正常快速行走，不突然停住，不交换手，不把水瓶放到岛台。

@图片2是强制末帧。摄影机已经沿同一岛台右侧进入开放式餐厅区域；同一座石材岛台现在自然位于画面左侧，人物仍在摄影机前方朝餐桌和客厅方向前进；餐桌位于中右侧，客厅更远，前方保持中性Brisbane晚上午日光。该帧作为最后一段继续靠近客厅和露台的首帧。

禁止cut、edit、jump cut、alternate angle、teleport、dissolve、crossfade、flash、black frame、body wipe、pillar wipe、architecture morph、穿过岛台、岛台瞬移、复制岛台、复制厨房、空间突然放大、人物突然出现、换脸换衣、左右手交换、水瓶消失、MPR插入、静态图片Ken Burns放大、字幕、Logo和水印。

原生声音连续：木地板上的跑鞋脚步、轻微衣料声、厨房与开放空间混响逐渐变宽、远处自然城市底噪；不得用音频断点掩盖画面跳转。
```

## Cost estimate

- Verified rate: CNY 0.042 per 1,000 completion tokens.
- The three recent like-for-like 5-second first-last-frame tasks each used 108,900 completion tokens.
- Like-for-like estimate: `108,900 / 1,000 x 0.042 = CNY 4.5738`.

Final billing follows actual completion tokens.
