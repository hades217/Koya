# Koya Property Video Production Asset Bible

更新日期：2026-08-16  
用途：Koya 沉浸式项目视频与模块化销售看房视频的统一视觉真相源

## 资产等级

| 等级 | 定义 | 使用规则 |
| --- | --- | --- |
| A - Official | 开发商提供的正式项目效果图与户型资料 | 建筑、配套和房间内容的最高真相源；不得由AI版本反向覆盖 |
| B - Locked Generated | 已锁定的人物身份、服装和道具母版 | 后续人物镜头必须引用；不得重新随机生成另一张脸或改变服装 |
| C - Concept Generated | 官方资料未提供的大堂、电梯、走廊等概念连接空间 | 仅作故事连接及艺术表现；必须与官方空间分开标识 |
| D - Derived Review | 分镜、联系表和其他审核材料 | 仅用于创意与连续性审核，不作为销售交付内容 |

## A级：官方项目效果图

所有文件均从原目录复制，未压缩、未改色、未覆盖；SHA-256已核对，与原文件完全一致。

| ID | 分类 | 文件 | 尺寸 | 主要用途 |
| --- | --- | --- | --- | --- |
| OFF-EXT-001 | 建筑外立面 | `assets/official/exterior/koya-building-hero.jpg` | 6000×3600 | 建筑比例、立面、入口及景观锚点 |
| OFF-LOC-001 | Brisbane/Toowong官方航拍 | `assets/official/location/koya-toowong-brisbane-aerial.webp` | 5669×3543 | Scene 00城市、河流、CBD与Toowong关系；SHA-256 `45a3af2f1de8c71c205043e2cf1d9ad4c173858b9c6a1c9b3b19f736c1b1444e` |
| OFF-LOC-002 | Koya官方区位图 | `assets/official/location/koya-toowong-location-map.webp` | 5669×3543 | Scene 00 Toowong至项目定位；SHA-256 `104c9af401e3d9a27e518a164e5467d08869666962a95fd5931486cc8f48e281` |
| OFF-AMN-001 | 屋顶配套 | `assets/official/amenities/koya-rooftop-pool.jpg` | 6000×4000 | 场景五泳池及城市方向视觉锚点 |
| OFF-AMN-002 | 屋顶配套 | `assets/official/amenities/koya-rooftop-bbq-dining.jpg` | 6000×4000 | 场景五屋顶用餐及社交空间锚点 |
| OFF-AMN-003 | 屋顶配套 | `assets/official/amenities/koya-rooftop-plan.jpg` | 6000×4000 | 屋顶休闲区、户外厨房、用餐、入口及通往泳池台阶的锚点 |
| OFF-INT-001 | 两房客厅/厨房 | `assets/official/interiors/koya-2br-living-kitchen.jpg` | 6000×4000 | 场景一进门揭示及场景三客餐厅锚点 |
| OFF-INT-002 | 两房厨房 | `assets/official/interiors/koya-2br-kitchen.jpg` | 6000×4000 | 场景三早餐和厨房动作锚点 |
| OFF-INT-003 | 两房主卧 | `assets/official/interiors/koya-2br-master-bedroom.jpg` | 6000×4000 | 场景二晨起及窗帘转场锚点 |
| OFF-INT-004 | 两房主卫 | `assets/official/interiors/koya-2br-ensuite.jpg` | 6000×4000 | 场景二主卫材质及洗漱动作锚点 |
| OFF-INT-005 | 三房客厅 | `assets/official/interiors/koya-3br-living.jpg` | 6000×4000 | 客厅、露台与城市方向的备选锚点 |

官方房间总览：`contact-sheets/official-interiors.jpg`  
官方外立面与配套总览：`contact-sheets/official-exterior-amenities.jpg`

OFF-AMN-003来源：Koya官网Wix静态资产 `2a230a_619e9e87e40744c6a4a201ff4456b49d~mv2.jpg`。该文件是屋顶休闲区透视效果图，不是俯视式平面图。画面可确认户外厨房/吧台、用餐桌、休闲沙发、绿化、玻璃栏杆和通往更高平台的台阶；具体门后功能及动线仍以正式建筑文件为准。

Scene 00白天clean plate：`assets/generated/concept-spaces/location/BRISBANE-TOOWONG-CLEAN-DAY-V1.png`，1672×941，SHA-256 `da25f99fb417e92b6fde4f395fab46c154e734d3d87bc1cfdd783b8eb1c05723`。该图由`OFF-LOC-001`移除营销文字、引线及Logo并改为中性白天色温，用于运镜/色彩参考；它是AI清理后的C级衍生概念底图，不升级为地理或测绘证据。项目目标轴依据原图Koya引线落点，约位于原画面宽度27.4%、高度84.6%的位置；最终公开使用前仍需销售方确认授权和位置。

Scene 00世界起始锚点：`assets/generated/concept-spaces/location/BRISBANE-TOOWONG-KOYA-WORLD-START-V1.png`，1672×941，SHA-256 `d9e6322af2eda2a5ed9a66b7745995eafd294b4207e8b9a70ceb332d5c821841`。该图在上述clean plate的目标地块加入一栋已建成Koya体量，使项目从第一帧存在并可被连续追踪；它是C级概念世界锚点，不是开发商批准的真实航拍合成，也不得覆盖`OFF-EXT-001`的近景立面几何。高空尺度只用于锁定目标，不用于销售建筑细节。

## A级：住宅户型平面图

4份PDF均为Level 1住宅户型图，与屋顶配套效果图分开管理：

| ID | 公寓 | 格局 | 车位 | 室内 | 室外 | 总面积 | 文件 |
| --- | ---: | --- | ---: | ---: | ---: | ---: | --- |
| FLP-102 | 102 | 2房2卫 | 1 | 79㎡ | 25㎡ | 104㎡ | `references/floorplans/Koya marketing plan Apartment 102.pdf` |
| FLP-103 | 103 | 1房＋书房角、1卫 | 1 | 71㎡ | 15㎡ | 86㎡ | `references/floorplans/Koya marketing plan Apartment 103.pdf` |
| FLP-104 | 104 | 2房2卫 | 1 | 89㎡ | 17㎡ | 106㎡ | `references/floorplans/Koya marketing plan Apartment 104.pdf` |
| FLP-106 | 106 | 2房＋多功能房、2卫 | 2 | 99㎡ | 53㎡ | 152㎡ | `references/floorplans/Koya marketing plan Apartment 106.pdf` |

这些户型图用于核对房间关系、入口位置、露台尺度、储物和功能空间。当前正式室内效果图文件名指向U504和U501，不能在没有对应资料时假设它们就是102、103、104或106号公寓的精确室内表现。

## B级：锁定人物资产

### 锁定角色阵容

| ID | 角色 | 正式母版 | 设定 | 状态 |
| --- | --- | --- | --- | --- |
| `CHAR-RESIDENT-001` | 东亚裔澳大利亚男性 | `assets/generated/character/resident-master-reference-v1.png` | 38岁，短黑发，沉静专业 | 可用于全部已定义场景 |
| `CHAR-RESIDENT-002` | 白人澳大利亚女性 | `assets/generated/character/white-female/resident-white-female-master-reference-v2.png` | 32岁，蜜糖棕金色锁骨发，温暖从容 | 身份已锁定；当前只批准回家场景服装 |
| `CHAR-RESIDENT-003` | 华人澳大利亚女性 | `assets/generated/character/chinese-female/resident-chinese-female-master-reference-v1.png` | 34岁，黑色锁骨发，沉静亲和 | 身份已锁定；当前只批准回家场景服装 |
| `CHAR-RESIDENT-004` | 运动型白人澳大利亚女性 | `assets/generated/character/sports-female/resident-sports-female-master-reference-v2.png` | 28岁，深棕马尾，匀称且有训练感的跑者体型 | 用于跑步、健身、运动后归家及休闲运动场景 |
| `CHAR-RESIDENT-005` | 运动型白人澳大利亚男性 | `assets/generated/character/sports-male/resident-sports-male-master-reference-v1.png` | 29岁，深棕短发，精瘦匀称的跑者/游泳者体型 | 只用于跑步、健身及休闲运动场景 |

五位角色总览：`contact-sheets/locked-character-cast-v4.jpg`

每份人物母版均包含：正面全身、背面全身、左侧面、行走背面、正面半身、后脑及衣领、钥匙、皮包与手表。男性拆分单张位于`assets/generated/character/master-panels/`；两位女性拆分单张分别位于各自目录下的`master-panels/`。

白人女性第一版因选角气质不符合要求已弃用，保留于`references/rejected-character/resident-white-female-master-reference-v1.png`，不得作为生成输入。

`CHAR-RESIDENT-004`运动服装已与身份一起锁定：鼠尾草绿色长袖运动上衣、哑光黑色全长legging、白灰跑鞋、黑色运动手表、灰褐色跑步腰包和不锈钢水瓶。正式独立面板位于`assets/generated/character/sports-female/master-panels-v2/`。v2加强了肩背姿态、腰线及臀腿训练感，同时保持真实跑者比例。该角色可用于河岸跑步、绿荫街道、屋顶轻运动，以及有明确“运动结束后回家”语境的建筑入口、电梯、走廊和入户镜头；不得无解释地穿运动装进入正式晚宴或商务场景。v1仅作版本记录，不再用于新镜头。

`CHAR-RESIDENT-004`全屋看房表演候选板：`assets/generated/character/sports-female/resident-sports-female-performance-board-v1.png`（1672x941；SHA-256 `525eab0a555d132a68c492c290f1b6e6218cc7cdc4bc61637a7e2bf9a1b1d92e`）。该板只控制真人微表情、回头确认、开门让路、侧身邀请及露台放松等表演，不可用它覆盖户型或室内空间真相；当前状态为待用户批准。

`CHAR-RESIDENT-005`锁定运动服为桉树灰长袖技术上衣、炭灰短跑裤叠穿哑光黑色全长压缩legging、白灰跑鞋、黑色运动手表、炭灰跑步腰包和不锈钢水瓶。独立面板位于`assets/generated/character/sports-male/master-panels/`。他可与`CHAR-RESIDENT-004`分别或共同出现在河岸晨跑、屋顶轻运动和社区生活镜头中，但共同出现时必须保持各自身份、体型和服装，不得融合面孔或互换道具。

### 场景服装

文件：`assets/generated/character/resident-wardrobe-continuity-v1.png`

以下四场服装板目前只对应`CHAR-RESIDENT-001`。`CHAR-RESIDENT-002`与`CHAR-RESIDENT-003`的身份母版已经包含场景一回家服装；如果选择她们继续出演晨起、早餐、社区或屋顶场景，必须先生成各自独立的服装连续性板。

| 场景 | 锁定服装 |
| --- | --- |
| 场景一：回家 | 暖米色轻薄外套、象牙白针织衫、炭灰长裤、深棕皮鞋、棕色皮包、银色手表、钥匙 |
| 场景二：晨起 | 燕麦色棉质睡衣套装、赤脚、无配件 |
| 场景三：早餐 | 象牙白长袖针织上衣、浅灰褐色休闲裤、赤脚、银色手表 |
| 场景四/五：社区与屋顶 | 灰绿色薄外套、象牙白T恤、深海军蓝长裤、米白皮质运动鞋、银色手表 |

每组正面与背面单张位于：`assets/generated/character/wardrobe-panels/`。

## C级：概念连接空间

旧连接空间母版已移至 `references/rejected/concept-spaces-v1/connective-spaces-master-v1.png`，不得作为新视频输入。当前有效锚点为下表中的 `CON-FOY-001` 与 `CON-FOY-002`；其余连续空间仍需逐镜补齐并锁定。

| ID | 空间 | 文件 | 状态 |
| --- | --- | --- | --- |
| CON-ENT-001 | 入口内侧通廊旧版 | `references/rejected/concept-spaces-v1/panels/entrance-vestibule.png` | deprecated；空间关系未证明，不得作为新视频输入 |
| CON-LBY-001 | 双电梯大堂旧版 | `references/rejected/concept-spaces-v1/panels/lift-lobby.png` | rejected；与当前单电梯foyer冲突 |
| CON-LFT-001 | 电梯内部旧版 | `references/rejected/concept-spaces-v1/panels/lift-interior.png` | deprecated；未与当前foyer建立同一轿厢关系 |
| CON-COR-001 | 住宅走廊旧版 | `references/rejected/concept-spaces-v1/panels/residential-corridor.png` | deprecated；只能作为材质参考，不能作为连续路线锚点 |
| CON-FOY-001 | 摄影机跨过入口门槛，电梯在同一轴线可见 | `assets/generated/concept-spaces/foyer/CON-FOY-001-threshold-v1.png` | v1概念艺术表现；原生16:9分镜锚点 |
| CON-FOY-002 | 同一foyer内连续前进至电梯呼梯键 | `assets/generated/concept-spaces/foyer/CON-FOY-002-lift-call-v1.png` | v1概念艺术表现；原生16:9分镜锚点 |
| CON-FOY-001-DAY | 白天版入口门槛至单电梯轴线 | `assets/generated/concept-spaces/foyer/CON-FOY-001-threshold-day-v2.png` | v2活动概念锚点；1672×941；中性Brisbane晚上午日光；SHA-256 `9fed7d88b75594a408cebd323e66e0c498436791fafc006e621884e008f60f74` |
| CON-FOY-002-DAY | 同一白天foyer前进至呼梯键 | `assets/generated/concept-spaces/foyer/CON-FOY-002-lift-call-day-v2.png` | v2活动概念锚点；1672×941；与入口版共享单电梯、地面、植物、座椅和人物；SHA-256 `6c3725800d5ed66fe0b74cde74b112fa5b56a1439faad8b42a02bdebf1e7a0c4` |
| SB-S01-LIFT-OPEN-001 | 同一单电梯开门 | `assets/generated/concept-spaces/elevator/SB-S01-LIFT-OPEN-001.png` | 新增概念分镜锚点；16:9；公共区域未获官方确认 |
| SB-S01-LIFT-CROSS-001 | 人物与摄影机跨过同一电梯门槛 | `assets/generated/concept-spaces/elevator/SB-S01-LIFT-CROSS-001.png` | 新增关键连续性锚点；16:9 |
| SB-S01-CABIN-OPEN-001 | 摄影机进入同一轿厢后、门仍开启 | `assets/generated/concept-spaces/elevator/SB-S01-CABIN-OPEN-001.png` | 新增概念分镜锚点；16:9 |
| SB-S01-DOORS-CLOSED-001 | 同一轿厢同机位关门 | `assets/generated/concept-spaces/elevator/SB-S01-DOORS-CLOSED-001.png` | 新增时间压缩锚点；16:9 |
| SB-S01-ARRIVE-001 | 同一扇门在目标层重新打开 | `assets/generated/concept-spaces/elevator/SB-S01-ARRIVE-001.png` | 新增概念分镜锚点；16:9 |
| SB-S01-EXIT-001 | 人物与摄影机连续走出电梯 | `assets/generated/concept-spaces/corridor/SB-S01-EXIT-001.png` | 新增关键连续性锚点；16:9 |
| SB-S01-CORRIDOR-001 | 目标层走廊快速接近公寓门 | `assets/generated/concept-spaces/corridor/SB-S01-CORRIDOR-001.png` | 新增概念分镜锚点；16:9 |
| SB-S01-DOOR-001 | 右手触碰公寓门把、镜头停门外 | `assets/generated/concept-spaces/corridor/SB-S01-DOOR-001.png` | 新增Stage 1终点锚点；16:9 |

这些空间沿用Koya的暖木、浅色石材、米白墙面、深色金属及克制Japandi语言，但没有正式公共区域设计资料支持。开发商一旦提供真实资料，应优先替换相应概念资产。活动视频输入改用`CON-FOY-001-DAY`与`CON-FOY-002-DAY`，共同锁定一条约6–8米的直线入口动线：中央玻璃入口、左侧植物、右侧内置座椅、尽端单部电梯；二者只能作为同一空间的连续机位使用，不得被拆成两个不同大堂。偏夜景的v1只作版本记录，不再用于白天视频生成。

## C级：户型驱动概念室内

这组资产用于补充开发商尚未提供的具体户型透视图。空间关系以对应Level 1平面图为几何真相源，材质、色温与软装语言参考A级官方室内效果图。它们是视频前期视觉开发，不是开发商正式效果图，也不构成交付承诺。

### Apartment 106 模块化 Living Hub V1（当前活动室内输入）

目录：`assets/generated/unit-concepts/106/modular-hub-v1/`

- 活动清单：`ACTIVE_REFERENCE_MANIFEST.md`
- 户型硬约束：`TOPOLOGY_LOCK.md`
- 17张16:9有效资源图：入口2张、Living/Dining HUB三视图3张、Bath/Laundry 2张、Bedroom 2 2张、MPR 2张、主卧/WIR/Ensuite 4张、Terrace 2张。
- 审核联系表：`contact-sheets/01-entry-hub-triad-v1.jpg`、`02-west-branches-v1.jpg`、`03-master-terrace-v1.jpg`。
- 本轮发现的4张错误构图已于2026-08-26移出活动目录，统一隔离至`references/rejected/modular-hub-v1/`，不得作为任何ImageGen或Seedance输入。
- 该包采用第一视角、无人物、只向前进入房间的模块化销售看房方式；各房间结束后通过确定性剪辑返回同一个HUB画面，不再生成倒退出房间的镜头。
- 此包在新的模块化室内制作中取代旧`continuity-endpoints/`作为活动上游。旧资料保留作审计历史，不得自动混用。

| ID | 公寓 | 概念板 | 重点空间 | Hero用途判断 |
| --- | ---: | --- | --- | --- |
| CON-U102-001 | 102 | `assets/generated/unit-concepts/102/apartment-102-concept-board-v1.png` | 玄关、岛台、客厅、露台、主卧 | 标准两房归家路线，适合温暖家庭叙事 |
| CON-U103-001 | 103 | `assets/generated/unit-concepts/103/apartment-103-concept-board-v1.png` | 线性厨房、客厅、书房角、卧室 | 紧凑高效，适合单身或年轻伴侣叙事 |
| CON-U104-001 | 104 | `assets/generated/unit-concepts/104/apartment-104-concept-board-v1.png` | L形入口、岛台、客厅、露台、主卧 | 空间均衡，适合标准两房销售叙事 |
| CON-U106-001 | 106 | `assets/generated/unit-concepts/106/apartment-106-concept-board-v1.png` | 入口、厨房、客餐厅、MPR、大转角露台 | 连续镜头路径最丰富，建议作为Hero主住宅候选 |
| CON-U106-ENTRY-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-entry-inside-door-left-v2.png` | 入户门后约0.8米的紧凑玄关；唯一门板留在左后方，右墙无复制门锁 | Segment C Retry 1活动末帧；由Type 106平面图约束，官方两房室内图仅作材质语言参考 |
| CON-U106-GALLEY-CINE-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-galley-mid-cinematic-v1.png` | 入户后沿厨房直通通道前进；连续柜体在左、唯一岛台在右、人物位于真实净宽内 | 新入口重做候选中间锚点；1672x941；SHA-256 `389d147ace3305fa54d72d220e9a18c0502b9c52ed2b2dac0007ae839b31b5df`；待用户批准 |
| CON-U106-DINING-TURN-CINE-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-dining-turn-cinematic-v1.png` | 第一版右转末帧；岛台在右但客厅错误落在右侧 | rejected；不得作为视频输入；SHA-256 `d004c99a4d068d7afacf3359515b35091d5b3543953e0ea26865ed4611c02bb9` |
| CON-U106-DINING-TURN-CINE-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-dining-turn-cinematic-v2.png` | 人物清空岛台尽端后右转；原岛台保持在右侧尾部，餐厅正前、客厅左前、露台更远 | 新入口重做候选末端锚点；1672x941；SHA-256 `890788b3f5d082324fa1aaad56befee4eba44b1204e1d12d04df69855f3549c3`；待用户批准 |
| CON-U106-WELCOME-CINE-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-welcome-living-cinematic-v1.png` | 第一版公共区邀请帧，空间拓扑可读但水瓶与邀请手左右颠倒 | rejected；不得作为视频输入；SHA-256 `b998e302c75c67c80c007fccadfe9b0f5b7264e3c7646920116d690d6a32863b` |
| CON-U106-WELCOME-CINE-003 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-welcome-living-cinematic-v3.png` | 人物左手水瓶、右手邀请；餐厅画面右、客厅画面左、露台正前 | Floor-plan audited Concept Design候选；1672x941；SHA-256 `b5da9513e4cbf67a5b665417df582d9497f83ec9303274a409e9f42c26e2dfe1`；待用户批准 |
| CON-U106-LIVING-FINAL-CINE-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-living-terrace-final-cinematic-v1.png` | 摄影机接受邀请并越过人物后的真正房间终点；客厅左/中、餐厅右、露台为远端光线目标 | Concept Design候选最终帧；1672x941；SHA-256 `a678526c683ad7d0478e6ec65909ae3492b8f49bc7ce5d494a94481c0f74a22f`；Type 106控制空间，官方室内图仅控制材质与气氛；待用户批准 |
| CON-U106-BED2-DOOR-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-bedroom2-door-cinematic-v1.png` | 从真实门口进入紧凑次卧；衣柜在左、床在右、外墙采光可读，人物在门口引导 | Concept Design候选；1672x941；SHA-256 `41941420135b4ed3be80d79a83c8eccbe5ecab3f2a5e437f0d6c3015fde1f41b`；待用户批准 |
| CON-U106-MPR-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-mpr-cinematic-v1.png` | 紧凑MPR门槛视角；书桌与日床表达多功能用途，不扩大成卧室或客厅 | Concept Design候选；1672x941；SHA-256 `36eb4f5c35538fad066baaa369a1b42f73bb47158e11e93eb92b4a43a98b8710`；待用户批准 |
| CON-U106-BATH-LDRY-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-main-bath-laundry-cinematic-v1.png` | 同一走廊内相邻但分离的洗衣与主卫门口关系，避免合并成虚构大房间 | Concept Design候选；1672x941；SHA-256 `48acb12134e59f6e406394dc307453a2b442f3b268a9f7d082aff243fa1c4748`；待用户批准 |
| CON-U106-BED1-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-bedroom1-arrival-cinematic-v1.png` | 主卧到达轴线；床、外部采光与左侧WIR入口同时可读 | Concept Design候选；1672x941；SHA-256 `a73858790b60f1572bbafc51f04eed8e353384e1403e836cc824e64dce74b698`；待用户批准 |
| CON-U106-WIR-ENS-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-wir-ensuite-cinematic-v1.png` | 穿过双侧WIR进入尽端套卫的单一路径，保持主卧套间的真实连接 | Concept Design候选；1672x941；SHA-256 `e215cc5ab4e1f970b15100f302bf88d97756b1168be262b02fe269d5e940cd1c`；待用户批准 |
| CON-U106-TERRACE-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-private-terrace-cinematic-v1.png` | 从客厅跨门槛到Type 106私属露台；户外家具与景观仅作说明，不虚构泳池或屋顶配套 | Concept Design候选；1672x941；SHA-256 `e0eb5a8740939dd5d2e13f8a346a1954581e6fb72dd7b2a4d5540e08b98a8d6f`；待用户批准 |
| CON-U106-K01-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k01-entry-wet-area-connector-v1.png` | 旧版入户湿区连接 | REJECTED：与保留视频21.047秒真实末帧机位不一致，摄影机回跳至入户门附近，不得作为视频输入 |
| CON-U106-K01-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k01-actual21s-turn-to-wet-area-v2.png` | 从真实21.047秒朝东末帧在原地连续回身向西；湿区位于画面右侧；人物左手水瓶、右手引导 | Actual-endpoint-derived / Floor-plan audited候选；1672x941；SHA-256 `f5e32ab00aa143cc575287e337ef89253172bdfb962c18c463d3e0e40888dfca`；待用户批准 |
| CON-U106-K03-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k03-wet-area-to-bedroom2-connector-v2.png` | 旧版湿区至次卧连接 | REJECTED：人物运动重新指向入户门，未证明转向次卧，不得作为视频输入 |
| CON-U106-K03-003 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k03-wet-area-to-bedroom2-connector-v3.png` | 离开北侧湿区后向南转入次卧唯一西北门；入户门保持在镜头后；人物左手水瓶、右手开门 | Floor-plan / hand-continuity audited候选；1672x941；SHA-256 `e2463420762effdebcef514d78e95dca73b380ca191f45232bc134b059650476`；待用户批准 |
| CON-U106-K05-003 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k05-bedroom2-same-door-exit-v3.png` | 次卧西墙衣柜在画面左、床在右；同一西北门退出；人物左手水瓶 | Floor-plan audited连续连接候选；1672x941；SHA-256 `11634f45a2e071e3e29ba202d2bf669ede87aa545bea75bb22a4174487ba5843`；待用户批准；v1/v2 rejected |
| CON-U106-BED2-EXIT-ACTUAL-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-bedroom2-exit-to-kitchen-actual36s-v1.png` | 从已通过36秒视频真实末帧派生：同一西北门退出后，次卧留在左后方并朝东进入厨房轴线；不提前进入MPR | Actual-endpoint-derived / Floor-plan audited；ImageGen内置模式；1536x864；SHA-256 `4bf7bba51503292138441c576ee3d7078955c429fdba706c46c9bc85ef3b96e0`；已用于Seedance任务 `cgt-20260819191900-nzq2t` |
| CON-U106-K06-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k06-hall-to-kitchen-axis-v1.png` | 次卧留在左后方，镜头转入厨房线性动线 | 连续连接候选；1672x941；SHA-256 `cb2709b0b743c617a5e0cc7d2949f0ef1e614829f0b9309f1d45eb566b8030d4`；待用户批准 |
| CON-U106-K07-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k07-kitchen-to-mpr-threshold-v2.png` | MPR从厨房/餐区一侧出现，使用向MPR内开的铰链门，唯一岛台保持可读 | Floor-plan audited连续连接候选；1672x941；SHA-256 `892aff4cb58a9de7641d25e2051229661f2b43fdf4c2a065a1c0040beb1ec997`；待用户批准；v1 rejected |
| CON-U106-MPR-CLOSED-ACTUAL-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-mpr-closed-door-actual41s-v1.png` | 从已接受41秒末帧派生；保留线性厨房与唯一岛台，人物左手水瓶、右手停在完全关闭的MPR门把手，MPR内部不可见 | Actual-endpoint-derived / Floor-plan audited；ImageGen内置模式；1672x941；SHA-256 `76ba8baf8953011db011df21ee5f741c058bd3aa2f88a3e1a86f0a6349ee0847`；已用于Seedance任务 `cgt-20260819220733-jd7c5`并通过逐帧QA |
| CON-U106-MPR-ENTRY-ACTUAL-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-mpr-entry-actual46s-v1.png` | 从45.936秒真实关门末帧派生；同一右铰链门向MPR内开启，摄影机刚跨东北门槛；书桌左、紧凑日床右、南窗正前 | Actual-endpoint-derived / Floor-plan audited；ImageGen内置模式；1672x941；SHA-256 `83ad51c6210ab14d70c6b5395d127ab15d946848dd6926e87a6159c572563ab5`；已用于Seedance任务 `cgt-20260819225144-55gsl`并通过逐帧QA |
| CON-U106-MPR-BACKOUT-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-mpr-backout-profile-v2.png` | 摄影机退至厨房侧仍面向MPR；人物从同一门槛走出并侧看厨房方向，左手水瓶；房内书桌、日床和南窗仍可读 | Actual-endpoint-derived / Floor-plan audited；ImageGen内置模式；1672x941；SHA-256 `c3cde54ef079940fe7541850a0eb2e943d984de1aad83f1d76d2cd59f2b50368`；已用于Seedance任务 `cgt-20260819234720-lzx48`并通过逐帧QA |
| CON-U106-K09-003 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k09-mpr-same-door-exit-v3.png` | MPR书桌、日床、同一铰链门与门外厨房同框；人物左手水瓶 | Floor-plan audited连续连接候选；1672x941；SHA-256 `22379bd2eea74107a09b9dad0de1c994d2fc422154c3686f03d952625ee1e4c8`；待用户批准；v1/v2 rejected |
| CON-U106-K11-001 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k11-kitchen-to-bedroom1-threshold-v1.png` | 厨房保持在左后方，主卧真实门口、床与采光同时出现 | 连续连接候选；1672x941；SHA-256 `fb66ffb60fccd47546aaa79a17ce6cdf02d018c3460bc192664058478f2b5019`；待用户批准 |
| CON-U106-K14-002 | 106 | `assets/generated/unit-concepts/106/continuity-endpoints/apartment-106-k14-suite-return-to-kitchen-v2.png` | WIR位于返程右后方、床在左、原西南门与门外厨房在前，证明套间原路返回 | Floor-plan audited连续连接候选；1672x941；SHA-256 `9cc07097b50e223c3a75b5f37f06882c3d9f6b79e8dde16636984f881d8462d2`；待用户批准；v1 rejected |

四户型总览：`contact-sheets/floorplan-concepts-v1.jpg`

Apartment 106全屋缺失空间审核总览：`contact-sheets/apartment-106-full-tour-missing-spaces-v1.jpg`（1860x788；SHA-256 `4dab430eb5dacf75ee08e6f5a5505e6a9ffebb2d92233a0b6b7930c0b7707141`）

Apartment 106连续连接分镜审核总览v1：`contact-sheets/apartment-106-continuity-connectors-v1.jpg`（2512x898；SHA-256 `9e7c2ad4918da516ee8e976e624855a58eab91dfc5a7dc9eee158d778f73a105`；superseded，含已拒绝版本，不得作为视频输入）

Apartment 106平面图审核后连接分镜：`contact-sheets/apartment-106-continuity-connectors-v2-floorplan-audited.jpg`（SHA-256 `2e348a385ece09e92da0cf75cf89d7d0fedfe86978976bce140d7f39dc6ca8f0`）

Apartment 106完整18节点平面图审核序列v2：`contact-sheets/apartment-106-full-sequence-v2-floorplan-audited.jpg`（2520x933；SHA-256 `90ea33afac1624b241d69de2c61376137b517c2d4bd45671a5898c19f6923394`；SUPERSEDED，含已拒绝K01 v1及K03 v2，不得作为新视频输入）

Apartment 106活动18节点序列v3：`contact-sheets/apartment-106-full-sequence-v3-actual-endpoint-audited.jpg`（3384x1104；SHA-256 `aeb49f89d1c058fe792c1167f3e73f357bf7bd51f60d43a39694f8b18e0abd11`）。已替换K01为真实21.047秒末帧导出的原地回身版本，并替换K03为湿区向南进入次卧版本；这是当前活动总览。

Apartment 106保留视频真实起点：`../video-production/continuity-audit/apartment-106-actual-21s-end-frame.png`（从21.047秒、1280x720、24fps保留视频末端提取；SHA-256 `87c8357b703b3e341372d07a2b81516ffb8c2aec4b436e4c3f975a3ea3b2909b`）。这是21秒后任何延续镜头的K00硬参考，不得用概念图替代。

Apartment 106真实起点至次卧审核板：`contact-sheets/apartment-106-start-sequence-v3-actual-endpoint-audited.jpg`（2448x1032；SHA-256 `7701c8af1306bacd702503544a71ca4d9bfb9a840ff18ccbbe67dd6d1c928699`）。顺序为K00真实末帧、K01原地回身、K02独立洗衣/主卫、K03向南进入次卧、K04次卧展示及户型路线证据。

Apartment 106真实房间路径图：`contact-sheets/apartment-106-literal-route-v2.png`。蓝色1–9为入户至套卫的查看路径，橙色为原路返回并进入餐厅、客厅和私属露台；该图仅叠加路线，不改动官方Type 106平面图。

限制说明：概念透视图只能表达经平面图约束的空间意图，不能证明精确尺寸、窗外景观、层高细部、固定家具、设备型号或最终材料。图中家具、艺术品、植物、灯具和城市远景均为说明性视觉元素。发生冲突时，正式销售合同、建筑文件和A级官方资料优先。

## D级：审核材料

- Stage 1单次30秒新版导演分镜：`../storyboards/STAGE_01_OPENING_30S_STORYBOARD_V1.md`
- Stage 1参考资料与缺口清单：`../storyboards/STAGE_01_REFERENCE_AND_GAP_MANIFEST_V1.md`
- Stage 1新版18格视觉顺序：`../storyboards/stage-01-opening-redesign-v1/stage-01-opening-30s-contact-sheet-v1.jpg`
- 场景一运动女性v3联系表与10镜导演分镜：已移至`references/rejected/scene-01-v3/`，不得作为新视频输入
- 场景一真实连续路线v4：`../storyboards/SCENE_01_LITERAL_WALKTHROUGH_STORYBOARD_V4.md`
- Scene 00连续俯冲运镜：`../storyboards/SCENE_00_GEOGRAPHIC_DIVE_V2.md`、`../storyboards/scene-00-geographic-dive-v2/scene-00-geographic-dive-contact-sheet-v4.jpg`
- Scene 00–01补充分镜与修正后的18格完整路线联系表：`../storyboards/STORYBOARD_SUPPLEMENT_V1.md`、`../storyboards/scene-01-full-route-contact-sheet-v6.jpg`
- 近地可读AI建筑版本因与官方立面几何不完全一致，已移至`../references/rejected/geographic-dive-readable-ai-building-v1/`，不得作为视频输入
- 场景一Koya品牌细节锚点：`../video-production/phase-01/anchors-16x9/OFF-EXT-001-official-entry-sign-1280x720.jpg`（`SB-S01-BRAND-001`，由A级`OFF-EXT-001`直接裁切；旧AI重绘版已移至`references/rejected/ai-exterior-brand-approach-v1/`）
- Apartment 106旧版12格室内联系表、MPR收尾帧和导演分镜：已移至`references/rejected/apartment-106-walkthrough-v1/`；因空间尺度争议且不属于当前Stage 1，不得作为活动输入
- 原始导演说明：`../KOYA_HERO_ONE_TAKE_BRIEF.md`

旧版男性场景一联系表及说明已于2026-08-15移至macOS废纸篓，不得作为新镜头输入。v3联系表拆分出的单格仅作审核，不是最终16:9视频母帧。v3中“入口遮挡后直接换成大堂”的02→03连接不符合`literal_walkthrough`，不得继续作为视频路线依据；真实连续路线以v4及两张foyer原生16:9锚点为准。

## 文件夹约定

```text
production-assets/
├── assets/
│   ├── official/             # A级官方真相源
│   │   ├── exterior/
│   │   ├── amenities/
│   │   └── interiors/
│   ├── generated/
│   │   ├── character/        # B级锁定人物
│   │   ├── concept-spaces/   # C级概念连接空间
│   │   └── unit-concepts/    # C级户型驱动概念室内
│   └── storyboards/          # D级审核分镜
├── contact-sheets/           # 分类总览
├── references/floorplans/    # 户型PDF副本
└── .media/                   # Media OS机器清单
```

## 制作硬规则

1. 生成任何建筑或房间镜头时，必须至少引用一张对应A级官方图。
2. 生成任何人物镜头时，必须引用人物母版及该场景对应服装图。
3. 不允许从旧分镜截图反向推断建筑或人物；分镜不是上游真相源。
4. 外立面楼层、阳台曲线、木格栅、栏杆、入口和植物布置以OFF-EXT-001为准。
5. 厨房柜体、石材岛台、地板、画作、家具及房间比例以对应OFF-INT文件为准。
6. 不得把U504/U501室内效果图直接标注为102/103/104/106号公寓，除非销售方提供明确对应关系。
7. 大堂、电梯和走廊必须标记为concept / artist impression，不能作为实际交付承诺。
8. 项目名、价格、完工日期、户型编号和CTA不烧录进生成画面，由网页DOM层承载。
9. AI生成的人物、房间或道具出现明显漂移时，退回上游母版重新生成，不在错误帧上继续补丁式迭代。
10. 户型概念室内必须同时引用对应编号的平面图和A级官方室内图；不得跨户型借用空间关系。
11. 概念图中的窗外景观、家具、灯具、艺术品、植物和设备仅为视觉说明，不得写成销售承诺。
12. 每个镜头必须明确指定一个角色ID；同一角色在连续场景中不得被另一角色面孔替换。
13. `CHAR-RESIDENT-002`只能引用v2正式母版；位于`references/rejected-character/`的白人女性v1不得使用。
14. `CHAR-RESIDENT-004`的legging必须保持全长、哑光、不透明；禁止改变为短裤、亮面材质、裸露上腹或带品牌标识的服装。
15. `CHAR-RESIDENT-004`只允许引用v2母版和`master-panels-v2/`；v1及旧面板不得进入新一代生成链。
16. `CHAR-RESIDENT-005`的短跑裤必须始终叠穿在压缩legging外，所有运动服均保持无品牌、哑光和实用尺度。
17. `literal_walkthrough`必须让摄影机亲自跨过入口门槛并连续走过foyer；入口构件、黑帧或门框不得用于把外部直接替换成电梯大堂。
18. 电梯门关闭只能压缩同一电梯轿厢内的上楼时间；不得在关门期间更换摄影机、轿厢或空间位置。
19. `koya`品牌标识必须在入口连续跟拍中自然形成可读近景；不得另切Logo插片，也不得改变官方小写拼写、字形位置或标识墙关系。
20. Apartment 106入户门的活动连续性几何固定为左侧合页、右侧把手；向内开启时门板与把手必须一起移向室内左侧。右墙出现第二门板、第二门锁或残留把手即判定失败。
21. Apartment 106从玄关进入公共区时，人物先沿北侧连续厨房柜体与南侧唯一岛台之间的直线通道向东前进；必须完全越过岛台东端后才能右转进入餐厅。转弯后岛台保持在摄影机右后侧，餐厅在正前，客厅在左前。任何岛台移到左侧、人物被岛台完整遮挡后改变空间朝向、或未看见清空岛台尽端即进入餐厅，都判定为穿物/串模失败。
22. Apartment 106新的模块化室内片只允许使用`modular-hub-v1/ACTIVE_REFERENCE_MANIFEST.md`列出的活动图；该目录`rejected/`及旧`continuity-endpoints/`不得作为生成输入。
23. 模块化室内片的每条分支只从HUB向房间前进，不生成倒退、原路退出或房间之间的捷径；结束后由剪辑明确返回HUB。
24. 模块化室内片为第一视角无人物方案，因此人物规则12不适用于这些分支；若以后重新加入人物，必须建立独立版本并恢复人物母版约束。
