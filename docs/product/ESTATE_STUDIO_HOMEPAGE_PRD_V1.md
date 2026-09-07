# Estate Studio Web 产品介绍页 PRD

> 当前开发要求已更新为 [V2：独立 URL 的 SEO 产品介绍页（静态 MPA）](ESTATE_STUDIO_PRODUCT_SEO_PRD_V2.md)。英文路径为 `/products/estate-studio/`，中文路径为 `/zh/products/estate-studio/`。不要 SPA；首页保留，通过普通链接进入产品页。本文件保留为 V1 首页文案与实施历史，后续产品页开发以 V2 为准。

- 版本：V1.0
- 日期：2026-09-06
- 状态：内容与页面需求草案，可用于设计和开发评审；不代表官网或产品已经正式上线。
- 开发形态：Web 页面，通过浏览器 URL 访问，适配电脑、平板和手机，无需安装客户端。
- 页面范围：面向潜在客户的产品介绍页，不是登录后的项目管理首页。
- 语言方案：提供完整中文内容及英文主文案；面向澳大利亚市场时默认英文，中文作为独立语言版本，避免同屏双语堆叠。
- 本次交付：产品定位、首页文案、页面结构、交互与转化要求、验收标准。实际网页开发另行实施。

## 1. 首页要传达的核心观点

**让地产项目的价值，更容易被看见。**

Estate Studio 是为地产团队打造的 AI 内容工作室。把项目资料、户型图和品牌规范组织在一起，用于制作互动导览、销售资料和营销素材，让每一份内容围绕同一个项目展开。

首页必须让第一次访问的客户迅速理解三件事：

1. 这是为开发商、项目营销团队和地产创意团队准备的产品。
2. 它能帮助团队把现有资料转化为客户可以观看、探索和分享的内容。
3. 项目资料可以重复使用，重要事实与最终内容由团队确认。

**传播主线：看见成果 → 认同问题 → 理解价值 → 查看工作方式 → 预约演示。**

## 2. 产品定位与目标用户

### 2.1 一句话定位

中文：**从项目资料到销售展示，地产团队的 AI 内容工作室。**

英文：**The AI content studio for property teams.**

定位解释：围绕一个地产项目，整理资料、沿用品牌、制作内容、审阅成果并交付。互动导览是吸引用户的展示入口，销售资料与创意内容体现持续使用价值。

### 2.2 目标用户与优先级

| 优先级 | 用户 | 典型需求 | 首页应给出的答案 |
| --- | --- | --- | --- |
| P0 | 项目营销负责人 | 为项目准备统一、可审阅的推广内容 | 一份项目资料，支持多种销售内容 |
| P0 | 开发商及内部营销团队 | 展示尚未建成的住宅，管理对外表达 | 让买家理解空间，并保留事实确认和审阅环节 |
| P1 | 地产代理团队 | 用好现有户型图、图片和销售资料 | 更直观地介绍项目，更方便地分享内容 |
| P1 | 地产创意工作室 | 服务多个客户，减少反复整理与交接 | 项目独立管理，资料与成果可以交接 |

这些需求来自现有产品方向，属于待用户访谈验证的需求假设，不作为行业调研结论或销售效果证明。

### 2.3 首页业务目标

- 第一目标：获得有真实项目需求的演示预约。
- 第二目标：让访问者体验一个经过审阅的互动示例。
- 第三目标：建立对项目资料管理、品牌一致性和内容审阅的理解。
- 首版主要 CTA：`预约演示 / Book a demo`。
- 次要 CTA：`体验互动导览 / Explore a tour`，仅在示例链接可用且展示权利已确认时启用。
- 当前未核实正式发行和自助付费链路，首版不使用“立即下载”“免费试用”“立即购买”。

## 3. 为什么客户需要 Estate Studio

### 3.1 用户问题与价值表达

| 用户遇到的问题 | 产品价值 | 可直接展示的短文案 |
| --- | --- | --- |
| 户型图、品牌文件、图片和销售文案散落在多个位置 | 按项目组织资料，供后续内容任务重复使用 | 项目资料准备一次，后续创作接着用。 |
| 平面图和零散效果图需要销售人员反复解释 | 用互动导览帮助买家理解房间与布局 | 让客户探索空间，让沟通更具体。 |
| 每做一份海报或销售页，都要重新说明项目背景 | 内容任务共享已确认资料与品牌规范 | 从导览到销售资料，延续同一个项目表达。 |
| 好看的 AI 内容可能与户型、规格或项目信息不一致 | 将官方资料、概念图和审阅状态区分开 | 创意可以探索，项目信息需要有依据。 |
| 项目交接时缺少来源、版本或工作文件 | 保留项目结构和可交接成果 | 下一次修改，从已有项目继续。 |

### 3.2 首页不应使用的承诺

不使用未经验证的效率比例、成本节省、成交率或客户数量。不宣称自动保证建筑准确、完全替代专业制作、全自动发布、全离线 AI 或从任意平面图一键生成真实数字孪生。

这不需要变成首页的大段警告；通过准确文案、恰当标签和真实演示建立可信度。

## 4. 页面结构

```text
顶部导航
  ↓
首屏：产品承诺 + 真实成果 + 双 CTA
  ↓
成果展示：互动导览 / 销售资料 / 营销创意
  ↓
为什么需要：资料分散、重复沟通与内容一致性
  ↓
核心亮点：共享资料、空间探索、品牌延续、审阅与交接
  ↓
工作方式：整理 → 制作 → 审阅交付
  ↓
演示项目：从输入资料到可见成果
  ↓
常见问题
  ↓
预约演示 + 页脚
```

首屏直接展示最强成果。案例证据紧随价值解释，不在页面开头堆放功能图标或技术架构。

## 5. 首页逐屏文案与设计需求

### 5.1 顶部导航

- Logo：Estate Studio。
- 导航：`成果展示 / What you can create`、`工作方式 / How it works`、`常见问题 / FAQ`。
- 右侧主按钮：`预约演示 / Book a demo`。
- 桌面端使用锚点跳转；移动端保持 Logo 和主 CTA 可见，其余放入简洁菜单。
- 不展示尚无真实目的地的登录、价格、下载入口。

### 5.2 首屏 Hero

**眉题**

为地产团队打造的 AI 内容工作室

**主标题**

让地产项目的价值，
更容易被看见。

**副标题**

把户型图、项目资料和品牌素材，转化为互动导览、销售资料与营销内容。在同一个工作空间里准备、制作和审阅，让每一份内容都围绕你的项目展开。

**按钮**

- 主按钮：预约演示
- 次按钮：体验互动导览

**辅助短句**

资料重复使用 · 品牌贯穿内容 · 发布前由你确认

**英文版本**

- Eyebrow: The AI content studio for property teams
- Headline: **Bring your property's potential into view.**
- Body: Turn floor plans, project information and brand assets into interactive tours, sales materials and marketing content. Prepare, create and review in one workspace built around your development.
- Primary CTA: Book a demo
- Secondary CTA: Explore a tour
- Supporting line: Reuse project information. Carry your brand through. Review before release.

**视觉要求**

使用一张经审阅的导览画面作为主视觉，配合一个真实销售资料缩略图，表达“同一个项目，多种成果”。不要让多个窗口争抢视觉焦点。优先采用现有成果，不为首页另行生成房产影像。

主视觉应保留适当产品线索，例如房间名称、户型导航或缩略图，让用户知道这是可以探索的体验。具体展示素材需要在实施阶段完成权利与状态确认。AI 概念图在素材附近标注，不能只在页脚说明。

### 5.3 成果展示：先让客户看到能得到什么

**标题：一个项目，多种打动客户的表达。**

**引导文案：从买家探索空间，到销售团队介绍项目，让资料真正进入每一次沟通。**

| 卡片 | 中文文案 | 英文文案 | 展示方式 |
| --- | --- | --- | --- |
| 互动导览 | 让买家自己探索房间、查看户型，建立对空间更直观的理解。 | **Interactive tours.** Help buyers explore rooms and understand the layout. | 可操作的经审阅示例，或有明确播放入口的预览 |
| 销售资料 | 将已确认的项目信息整理成项目画册、户型图册和单户型销售页。 | **Sales materials.** Shape approved project information into brochures, floor plan books and unit sales sheets. | 实际文档页面，可点击放大 |
| 营销创意 | 围绕项目品牌准备海报与推广素材，沿用已有内容和视觉方向。 | **Marketing creative.** Build posters and campaign assets around your project's information and visual identity. | 已完成样张；只有草案时明确标注概念样例 |

英文标题：**One development. More ways to tell its story.**

交互要求：桌面三列，移动端纵向排列；每张卡片只提供一个明确动作。任何尚无完成样例的类别不使用模拟成品冒充正式成果。

### 5.4 为什么需要：让用户认出自己的工作问题

**标题：项目资料已经很多，把它们用好仍然很费力。**

**正文：**

一张户型图、一组效果图、一份品牌文件，往往只是起点。准备导览、制作销售资料、调整推广内容时，团队还需要反复寻找文件、说明背景、核对信息。

Estate Studio 把这些工作放回同一个项目中。整理好的资料可以继续使用，确认过的内容可以作为下一次创作的起点。

**三条短句：**

- 少一些反复交代项目背景。
- 更容易沿用已确认的资料和品牌方向。
- 每次修改，都能从已有成果继续。

**英文标题：Your project has the information. Put it to work.**

**英文正文：**

Floor plans, renders and brand files are only the starting point. Creating a tour, preparing sales materials and updating a campaign can mean finding files, repeating briefs and checking details all over again. Estate Studio keeps that work organised around one project, so each new task can build on what your team has already prepared and approved.

### 5.5 核心亮点：四个值得选择的理由

#### 亮点一：项目资料准备一次，持续用于创作

把户型、图片、品牌和已确认的信息保存在项目中。制作下一份内容时，从已有资料继续，减少重复整理。

**EN — Prepare once. Build on it.**
Keep plans, imagery, branding and approved information together, ready for the next piece of content.

证据画面：实际项目资料界面与不同内容任务；避免用无产品对应关系的装饰性流程图。

#### 亮点二：把空间说明，变成可探索的体验

把户型与房间画面连接起来，让买家按自己的节奏查看空间，也让销售人员有更直观的沟通工具。

**EN — Give buyers a space to explore.**
Connect floor plans and room views in an experience buyers can explore at their own pace.

证据画面：已审阅的户型热点与房间切换。不得表述为任意自由行走或测量级三维模型。

#### 亮点三：让不同内容延续同一个品牌

从销售页到项目海报，沿用项目的视觉规范与已确认信息，让每次对外展示更连贯。

**EN — Carry your brand through every piece.**
Use shared project information and visual guidelines across sales materials and marketing creative.

证据画面：同一项目两至三件实际成果，展示字体、色彩和信息的延续性。

#### 亮点四：制作有记录，交付有依据

保留资料来源、版本和审阅记录。团队可以检查内容从哪里来、哪些已经确认，并将项目文件与成果一起交接。

**EN — Review with context. Hand over with clarity.**
Keep source information, versions and review records with the project, alongside the work you deliver.

证据画面：简洁的版本或审阅界面。首页只展示用户能理解的状态，不展示内部校验字段。

### 5.6 工作方式

**标题：从已有资料开始，让下一份内容更有基础。**

**EN: Start with your project. Build from there.**

| 步骤 | 中文标题与文案 | 英文标题与文案 |
| --- | --- | --- |
| 01 | **整理项目。** 导入户型图、项目图片和品牌资料，确认关键信息。 | **Prepare your project.** Bring in plans, imagery and brand assets, then confirm the key details. |
| 02 | **制作内容。** 选择导览、销售资料或营销素材，围绕已有项目内容展开制作。 | **Create your content.** Build a tour, sales document or creative asset using your project information. |
| 03 | **审阅并交付。** 检查事实、画面和品牌表达，确认后导出；在线发布按项目交付方式安排。 | **Review and deliver.** Check the details, visuals and branding before export. Arrange online publishing around your project needs. |

不承诺固定分钟数或全自动完成；有额外生成费用的操作，应在产品内展示费用并确认。

### 5.7 项目演示与可信证据

**标题：看看项目资料，如何成为一次完整的展示。**

**EN: See the journey from project files to presentation.**

建议结构：

- 输入：允许公开展示的户型图或项目资料局部。
- 输出：同一项目、同一户型的互动导览与销售资料。
- 说明：这些成果使用了哪些资料，哪些画面属于概念可视化。
- 按钮：`查看演示 / View the demo`。

Koya 仅作为候选演示项目。未经本次公开展示权利与最终素材状态核实，不使用“客户成功案例”“客户评价”或项目品牌背书。需要匿名时使用中性的演示项目名，不虚构客户。

若公开演示尚未准备好，删除此区域的操作入口，并将首屏次 CTA 改为 `查看制作流程 / See how it works`，跳转工作方式区。不能留下失效按钮，也不能将本地路径当作公开链接。

### 5.8 常见问题：完整候选文案

**Q1：Estate Studio 适合谁？ / Who is Estate Studio for?**

适合需要制作和管理项目展示内容的开发商、项目营销团队、地产代理及创意工作室，尤其适合围绕同一项目持续制作多种资料的团队。

For developers, project marketers, real-estate agencies and creative studios creating multiple kinds of content around the same property project.

**Q2：开始需要准备什么？ / What do I need to get started?**

可以从户型图、项目图片、品牌文件和已确认的项目信息开始。演示时会结合你希望制作的内容，梳理需要补充的资料。

Start with floor plans, project imagery, brand files and confirmed project information. During the demo, we can review what else your intended outputs may need.

**Q3：团队需要懂 AI 或编程吗？ / Does my team need AI or coding experience?**

产品按项目整理、内容制作和审阅的工作方式设计。演示会带你了解实际操作流程、可用能力和所需支持。

The workflow is organised around project preparation, content creation and review. The demo will walk you through the available capabilities and any support your team may need.

**Q4：AI 画面能代表最终建成效果吗？ / Do AI visuals represent the finished property?**

AI 生成画面属于概念可视化，需要对照官方资料审阅，并保留相应说明。实际规格和交付内容应以项目正式文件为准。

AI-generated visuals are conceptual and need review against official project information, with appropriate disclosures. Refer to formal project documents for specifications and delivery details.

**Q5：查看产品介绍需要安装软件吗？ / Do I need to install software to explore the product?**

不需要。你可以通过电脑或手机浏览器查看产品介绍、浏览可用的演示内容，并提交演示申请。

No installation is needed. Use your desktop or mobile browser to explore the product page, view available demos and request a demo.

**Q6：如何了解价格与使用方式？ / How can I learn about pricing and access?**

预约演示并说明项目类型与希望制作的内容，我们会结合当前可提供的软件、制作支持与服务范围沟通方案。

Book a demo and tell us about your project and the content you want to create. We will discuss the software access, production support and services currently available.

FAQ 默认折叠，首项可展开；每项内容可被搜索引擎和辅助技术读取。未经确认的价格、可用平台与正式发行状态不写成既定事实。

### 5.9 底部转化区

**标题：让你的下一个项目，有更好的展示起点。**

**正文：带上你现有的户型图、图片或项目想法。我们一起看看，哪些内容值得先做，以及 Estate Studio 如何融入你的工作。**

**主按钮：预约项目演示**

**EN headline: Give your next project a stronger start.**

**EN body: Bring your plans, imagery or project idea. Let's explore what to create first and how Estate Studio could fit your workflow.**

**EN CTA: Book a project demo**

表单辅助说明：首次联系无需上传项目文件。

表单成功文案：`已收到你的演示申请。我们会通过你提供的邮箱联系你。`

EN: `Your demo request has been received. We will contact you using the email address provided.`

仅在后端确认接收成功后展示。若采用外部预约系统，应明确跳转并沿用真实预约确认状态。

## 6. 功能需求与转化流程

| 编号 | 优先级 | 要求 | 验收条件 |
| --- | --- | --- | --- |
| H01 | P0 | 响应式首页与锚点导航 | 360px 手机、平板和桌面均可访问，无横向溢出 |
| H02 | P0 | 全页统一主 CTA | 点击后进入同一个有效表单或真实预约页 |
| H03 | P0 | 演示素材状态控制 | 只有验证可访问且允许公开展示的素材才显示入口；否则使用流程锚点 |
| H04 | P0 | 预约表单 | 必填姓名、邮箱；选填公司、项目需求；联系方式和提示经过确认 |
| H05 | P0 | 表单异常处理 | 校验邮箱、避免重复提交；失败保留输入并支持重试；不虚报成功 |
| H06 | P0 | 隐私说明 | 提交前提供真实隐私说明链接；营销订阅若存在则独立、默认不勾选 |
| H07 | P0 | 媒体加载 | 主视觉优先展示静态封面；导览由用户主动加载；视频不自动出声 |
| H08 | P0 | 无障碍 | 语义标题、按钮键盘操作、可见焦点、图片替代文字、FAQ 状态可读 |
| H09 | P0 | 内容可维护 | 标题、正文、CTA、FAQ、素材与演示地址可集中修改 |
| H10 | P0 | 基础搜索信息 | 可抓取正文、唯一 H1、title、description、分享图；不写无依据的评分或客户评价数据 |
| H11 | P1 | 中英语言版本 | 保持同一信息结构与转化路径；完成校对后上线，不依赖同屏机器翻译 |
| H12 | P1 | 成果放大查看 | 支持关闭、键盘 Esc、焦点返回与手机手势兼容 |

表单处理方式在实施时二选一：已有预约服务，或接入真实后端的站内表单。首页上线前必须确认接收负责人、收件目标和隐私页，不要求访问者先注册账号。

页脚最少包含品牌、真实联系入口、隐私说明及适用的条款。不要添加占位社交链接或未确认的公司资质。

## 7. 视觉与内容规范

- 方向：现代、克制、有建筑感，突出项目成果的品质与产品的可操作性。
- 布局：大幅成果画面、清晰标题、充分留白；辅助界面只承担解释功能。
- 色彩：实施时沿用 Estate Studio 已确认的品牌规范；没有定稿前使用中性色原型，不把单个楼盘的品牌当作软件品牌。
- 排版：首屏正文控制在短段落；移动端主标题不超过三行，关键动作不被装饰遮挡。
- 动效：轻微进入与状态过渡；尊重减少动态效果设置，不做强制长距离滚动动画。
- 素材：使用真实产品截图与经审阅成果；不虚构客户 Logo、评价、数据或奖项。
- 品牌称谓：统一使用 Estate Studio。Koya 是示例项目，不是产品名称。
- 文案优先表达用户得到什么。模型名称、技术栈、内部审批编号等放在必要的说明或支持材料中。

建议搜索文案：

- 中文 Title：`Estate Studio｜地产 AI 内容工作室`
- 英文 Title：`Estate Studio | AI Content Studio for Property Teams`
- 中文 Description：`用项目资料、户型图与品牌素材，制作互动导览、销售资料和营销内容。了解 Estate Studio，为你的地产项目预约演示。`
- 英文 Description：`Turn property plans, project information and brand assets into interactive tours, sales materials and marketing content. Book an Estate Studio demo.`

## 8. 产品表述与证据边界（内部）

本 PRD 依据当前本地产品文档，不是正式发行审计。仓库中的实现说明可以帮助定义演示主题，不能单独证明客户可购买、下载或稳定使用。

| 主题 | 当前依据 | 首页处理 |
| --- | --- | --- |
| 项目资料、本地工作空间、导览、审阅与交付 | 应用 README 描述了本地 MVP 实现 | 可以作为产品方向和演示主题；上线前核对演示路径 |
| 七类销售资料 | 销售资料工作流规定了文档与证据关系 | 首屏突出画册、户型图册、单户型销售页；以真实样张证明 |
| 海报、完整营销素材包和销售站点 | 产品愿景包含相关规划；README 对部分能力仅描述制作包 | 不把制作包说成已渲染成品；本期只展示实际完成并审阅的样张 |
| 长视频自动生成、自动社媒发布、实时多人协作 | 无本次可用性证据，部分超出现有 MVP 范围 | 不放入首版卖点 |
| macOS / Windows 下载与自助订阅 | README 列有发行和外部服务条件 | 只提供预约演示，实际销售方案待核实 |
| 客户效果、节省时间、转化提升 | 无本次量化证据 | 不写具体数字或保证 |
| Koya 公开案例 | 示例展示和分发有权利要求 | 公开展示前确认素材、权利与状态；未确认使用匿名或删除 |

## 9. 数据指标与验证计划

以下是首页测量方案，不是现有经营数据。首版先建立基线，再决定优化目标。

| 事件 | 触发条件 | 用途 |
| --- | --- | --- |
| homepage_view | 首页有效访问，按会话去重 | 漏斗入口 |
| demo_cta_click | 点击预约按钮；记录 hero/nav/footer 来源 | 判断哪个位置产生意向 |
| tour_open | 用户成功打开演示 | 判断成果吸引力 |
| tour_interaction | 示例中第一次有效切换房间等操作；能合法接入时才记录 | 区分打开与实际体验 |
| lead_submit_success | 后端确认有效申请接收成功 | 计算提交转化率 |
| lead_submit_error | 提交失败；只记录错误类型 | 发现阻碍 |

- 预约提交率 = 成功提交的去重会话数 / 首页有效会话数。
- 合格线索率由业务负责人依据真实项目需求判定，标准在开始评估前统一。
- 不向分析事件发送邮箱、姓名或项目资料正文；分析工具选择与同意机制按实际上线配置确认。
- 小范围理解测试：邀请 5 名目标用户看首屏 10 秒，再复述“产品给谁用、能做什么、下一步是什么”。目标至少 4 人答对核心意思；这是验收目标，不是已测结果。

## 10. 首版验收与上线依赖

### 内容验收

- [ ] 首屏能明确识别“地产团队 + 内容制作 + 项目资料复用”。
- [ ] 三类成果各有真实样例或准确的状态说明。
- [ ] 卖点能够映射到演示或产品依据，没有无来源的量化承诺。
- [ ] 中英文分别可直接阅读，没有占位文字或混用产品名称。
- [ ] 项目素材、概念说明和公开展示权利已经确认。

### 交互验收

- [ ] 导航、主 CTA、FAQ、成果入口均有效。
- [ ] 演示不可用时按既定方案回退，不跳到空页。
- [ ] 表单成功、失败、重复点击、必填校验均验证通过。
- [ ] 手机首屏可读、媒体不会阻塞主要动作、键盘可完成核心操作。
- [ ] 埋点不包含表单隐私数据。

### 上线依赖

- 官网域名、发布位置与品牌视觉定稿。
- 真实演示地址、展示素材和对应权利确认。
- 预约接收渠道、接收负责人、真实联系信息和隐私文本。
- 当前对外提供的软件与服务范围，供 FAQ 和演示沟通使用。

这些依赖不阻碍文案和页面设计；缺少接收渠道或展示授权时，不能把完整转化链路视为已上线。

## 11. 后续迭代

- 首版：完成清晰定位、真实成果展示、预约演示与基础测量。
- 获得客户授权与结果证据后：补充项目案例、使用过程与可验证的客户反馈。
- 正式发行和商业方案核实后：增加定价、下载、试用和相应支持说明。
- 有线索数据后：按访问来源优化不同用户的入口，不预先扩展为复杂的行业站点。

## 12. 本地依据

- `apps/real-estate-ai-studio/README.md`：产品描述、本地 MVP 能力和发行边界。
- `docs/product/REAL_ESTATE_AI_STUDIO_PRODUCT_VISION_V1.md`：项目中心定位、目标客户及平台方向。
- `docs/product/OFFPLAN_TOUR_BUILDER_PRD_V1.md`：导览价值、用户旅程与能力边界。
- `docs/product/CREATIVE_STUDIO_TASK_SESSION_PRD_V1.md`：内容任务与持续制作工作空间。
- `docs/product/SALES_MATERIALS_SOT_MD_HTML_WORKFLOW.md`：销售资料类型和资料依据要求。

以上文件用于内部对齐，不应作为首页面向客户的内容模块。

## 13. 设计与开发交接规格

### 13.1 首版范围与信息层级

本期开发交付一个公开的 Web 产品介绍页，包含第 5 节的页面文案、成果展示、FAQ 和演示申请流程。页面以 URL 访问，按网站方式部署与验收。后台项目库、导览编辑器、支付、软件安装与客户账户不属于本页面的开发范围。

首屏必须同时回答：给谁用、能得到什么、下一步做什么。完整介绍段落可供 About、销售提案或页面摘要复用：

> Estate Studio 是面向开发商、地产营销团队和创意工作室的 AI 内容工作空间。它围绕每个地产项目组织户型图、项目资料和品牌素材，帮助团队准备互动导览、销售资料与营销内容。团队可以沿用已有资料开展新的制作任务，结合来源与版本审阅成果，并在确认后安排交付，让项目展示拥有连贯的信息与视觉表达。

产品名称统一为 Estate Studio；“地产 AI 内容工作室”是品类说明。本次页面需求以 Web 为准，不要求宣传桌面工作空间或安装客户端。现有桌面应用文档仅用于理解业务能力，不决定本次页面的开发形态，也不意味着完整编辑能力已经迁移到 Web。下文“桌面布局”均指电脑浏览器下的网页布局。

### 13.2 页面线框与移动端顺序

| 区域 | 桌面布局 | 移动端布局 | 内容约束 |
| --- | --- | --- | --- |
| 导航 | Logo、三个锚点、预约按钮 | Logo、预约按钮、菜单 | 当前语言只有一套导航，不混排中英文 |
| 首屏 | 左侧标题与动作，右侧主成果画面 | 标题、正文、主按钮、次按钮、画面 | 主 CTA 在 390 × 844 的初始视口内可见 |
| 成果 | 三张等权卡片 | 导览、销售资料、营销创意依次纵排 | 每张卡只有一个结果入口 |
| 问题与价值 | 短段落、四个亮点及对应证据 | 文案紧邻对应图片 | 同一卖点不在连续两屏重复解释 |
| 工作方式 | 三步横排 | 三步纵排 | 每步标题加一段说明 |
| 项目演示 | 同一项目的输入与输出并列 | 输入、输出、说明、体验入口 | 无合格素材时整段隐藏，页面间距同步收起 |
| FAQ | 单列折叠列表 | 同桌面 | 支持同时展开多项，长答案不裁切 |
| 预约区 | 介绍和表单并列 | 介绍后直接显示表单 | 所有主 CTA 均定位此处；外部预约模式则统一跳转 |

视觉尺寸由设计稿确定；首版检查宽度为 360、390、768、1280 和 1440px。文字放大到 200% 后，正文和表单仍可阅读操作。

### 13.3 预约表单的完整状态

采用站内表单时，至少实现以下状态；外部预约模式不另外展示一个不可提交的站内表单。

| 状态 | 用户看到的内容 | 行为要求 |
| --- | --- | --- |
| 初始 | 姓名、邮箱、公司（选填）、项目需求（选填）、提交按钮 | 不要求上传资料或注册；字段具有持续可见的标签 |
| 校验失败 | 对应字段下方的具体提示 | 姓名为空提示“请输入姓名”；邮箱无效提示“请输入有效的邮箱地址”；聚焦首个错误字段 |
| 提交中 | “正在提交…” / “Sending…” | 禁止重复提交，保留全部内容，向辅助技术通报状态 |
| 接收成功 | 第 5.9 节的成功文案 | 仅在接收端明确确认后显示；替换表单，避免再次误提交 |
| 接收失败 | “暂时无法提交，请稍后重试。” / “We couldn't send your request. Please try again.” | 保留输入并恢复提交按钮；不显示原始服务错误或内部信息 |
| 结果不确定 | “暂时无法确认提交结果，请重试。” / “We couldn't confirm your request. Please retry.” | 同一申请重试应复用提交标识，接收端避免重复创建线索 |
| 接收渠道未配置 | 本地预览显示清晰的演示状态 | 禁止伪造成功；生产上线验收不通过 |

成功接收不等于已预约具体时段。如果系统没有日历确认，不使用“预约已确认”或“会议已安排”。不承诺未经团队确认的回复时限。

### 13.4 演示与媒体交互

- 页面先加载封面及正文，用户点击后再加载互动导览；导览加载失败时保留封面和预约入口，并提示“演示暂时无法加载”。
- 公开演示的链接可用性与素材状态应由发布配置控制。不得仅凭图片文件存在就启用“体验互动导览”。
- 图片放大使用对话框：打开后焦点进入，Tab 留在对话框内；关闭或按 Esc 后焦点回到原卡片。
- 导览需保持自身的返回、退出或关闭操作；移动端退出后回到原页面位置。
- 媒体容器预留宽高比，避免加载时推动正文与按钮；手机首屏不下载全景原图或整段视频。
- 图片替代文字描述实际内容，例如“Estate Studio 中的户型与房间导航”，避免写成营销标语。

### 13.5 实施任务与完成证据

| 工作包 | 优先级 | 交付内容 | 完成证据 |
| --- | --- | --- | --- |
| 内容与设计 | P0 | 默认英文页面稿、对应中文稿、桌面及手机设计稿 | 文案逐项对应本 PRD，素材来源和展示状态有记录 |
| 页面开发 | P0 | 响应式页面、导航、FAQ、媒体预览及降级状态 | 五个目标宽度截图，真实按钮操作记录 |
| 线索接收 | P0 | 一个有效的预约渠道、成功与失败状态 | 一次测试申请在接收端可查；重复重试不生成重复申请 |
| 演示接入 | P0 | 合格的公开示例，或流程锚点回退 | 未登录访问结果；断开演示后仍能继续阅读和预约 |
| 内容质量 | P0 | 标题、分享信息、替代文字、键盘与放大检查 | 无占位链接；键盘完成预约流程；概念素材说明就近可见 |
| 基础测量 | P0 | 第 9 节中的可用事件 | CTA 点击与接收成功分别触发，事件不携带姓名和邮箱 |
| 独立中文版本 | P1 | 经校对的中文页面及语言切换 | 两个语言版本都能进入同一个有效转化流程 |

这些是待执行的验收要求，并非本次已完成的网页功能。文档完成、页面本地可用、预约渠道接通、公开上线分别记录；上线以真实访问和接收端验证为准。

## 14. Web 首版实施记录（2026-09-06）

- 页面代码：`apps/estate-studio-website/`，构建输出为独立静态网页；英文 `/`，中文 `/zh/`。
- 已完成本地页面：响应式布局、成果展示、三房间图片切换、销售版式放大、FAQ、预约邮件表单和隐私说明。
- 用户确认沿用 `hello@estatestudio.io`。本版预约采用 `mailto` 邮件草稿，由访客在邮件应用中确认发送；这是首版明确采用的联系方案，不等于第 6 节中具有接收端确认的站内表单。页面不显示申请已接收或会议已安排。
- 本地浏览器已检查中英文页面的 360、390、768、1280、1440px 宽度，无页面横向溢出；房间切换、资料弹窗、Esc 关闭、FAQ 展开、必填与邮箱格式校验可用，检查时无控制台错误。
- 图片为带明确标识的本地概念预览，版式为示意；完整互动导览、预约接收端确认、分析埋点、公开部署及公开素材权利核验尚未完成，不计入本次已实现范围。
- `npm --prefix apps/estate-studio-website run verify` 检查静态构建、浏览器脚本语法、两种语言页面的本地资源、链接、锚点及基础文档结构。
