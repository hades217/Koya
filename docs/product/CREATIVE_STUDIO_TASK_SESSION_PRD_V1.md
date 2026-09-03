# Creative Studio 任务工作台与 AI Session PRD

| 字段 | 内容 |
| --- | --- |
| 版本 | V1.1 |
| 状态 | 产品设计草案，待确认后实施 |
| 日期 | 2026-09-02 |
| 适用产品 | Estate Studio Desktop · Creative Studio |
| 首个参考项目 | Koya |
| 核心改版 | 顶部任务切换 + 中间 Preview + 右侧 AI Chat |
| AI 执行架构 | Codex App Server 原生 thread / turn / event 流程 |

## 1. 背景与问题

当前 Creative Studio 同时展示 Creative 类型、销售任务、Ask Codex、项目输出和 Preview，用户需要先理解页面结构，才能开始设计。AI Poster、A4 Document、Video 既像页面入口，又像生成方式；D01 与 S01–S07 又是另一套任务入口，导致“正在做哪个设计”“Chat 正在使用哪个上下文”“当前预览属于哪个任务”不够明确。

用户真正需要的是一个以设计任务为单位的 AI 工作台：

- 每个项目保持独立的项目设计规范和证据；
- 每个设计任务拥有独立工作上下文；
- 打开 S02 时进入 S02 自己的 session；
- 切换到 D01 时恢复 D01 自己的 session；
- Preview 始终对应当前任务与当前 session；
- AI Chat 是所有设计操作的核心入口；
- AI Poster、A4、Video 是 Chat 可调用的能力，不再占据一级页面导航。

## 2. 产品目标

将 Creative Studio 改造成一个项目内、任务级、session 驱动的设计工作台。

用户进入任何任务后，应在一个屏幕内完成：

`选择任务 -> 恢复任务 session -> 与 AI 讨论或修改 -> 查看 Preview -> 审阅 -> 保存输出`

### 2.1 成功标准

- 用户在 3 秒内知道当前项目、当前任务、当前 session 和当前输出状态。
- 切换 D01、S01–S07 时，不会串用其他任务的聊天记录、草稿或附件。
- Preview 不因切换任务而显示上一个任务的内容。
- 新增、重命名、切换、压缩、归档或删除 session 均可理解且可追踪。
- 用户无需离开 Chat，即可调用文案、版式、图片、A4、视频、证据检查和导出能力。
- Chat 不能把未经确认的对话内容自动写入项目 SoT。

### 2.2 AI Chat 产品定位

右侧 AI Chat 必须是一个真正可持续工作的 Codex Agent 客户端，而不是“一次输入 -> 生成一个 brief”的包装层。

它需要提供与 Codex 工作流一致的体验：

- 多轮对话，持续保留当前设计任务的工作上下文；
- Agent 可以读取项目资料、检查文件、修改允许范围内的设计源文件、运行渲染和验证命令；
- 流式显示 AI 消息、计划、工具调用、命令、文件变更、进度与最终结果；
- 用户可以在 Agent 工作过程中继续补充方向；
- 用户可以停止当前精确 turn；
- 恢复 task 时恢复原 Codex thread，而不是把最近几条消息重新拼进新 prompt；
- 文件修改、工具调用和审批结果与中间 Preview 联动；
- 不把 Codex 能执行文件操作误解为可以绕过项目事实、付费生成、审批或发布 gate。

目标体验示例：

```text
用户：把 S02 的户型目录改成按卧室数量分组。
Codex：读取 S02 SOT 和当前 document.md
Codex：提出结构调整并修改任务工作文件
系统：显示 file change / render / validation 状态
Preview：刷新为新 proposal revision
用户：第二页太挤，把图注再收短一点。
Codex：继续在同一个 thread 和 task 上工作
```

这与当前一次性的 `generateCreativeBrief` 不同。正式实现必须使用 Codex 自己的持久 thread、turn 和事件协议。

## 3. 非目标

本次改版不负责：

- 改变 D01 的项目设计规范审批规则；
- 改变销售资料的 `SOT.md -> document.md -> document.html -> PDF` 血缘关系；
- 将聊天记录视为官方项目证据；
- 绕过图片或视频生成前的人工审批与费用确认；
- 在未获得授权数据时推测价格、房源、面积、材料、日期或销售条款；
- 用一个跨项目 session 混合多个房地产项目。

## 4. 核心产品模型

### 4.1 层级

```text
Project
└── Creative Task
    ├── Task definition
    ├── Task evidence scope
    ├── Design outputs
    └── Sessions
        ├── Session A
        │   ├── Messages
        │   ├── Attached assets
        │   ├── Tool runs
        │   ├── Draft revisions
        │   └── Active preview revision
        └── Session B
```

### 4.2 Task 与 Session 的区别

**Task** 是一个明确的设计交付物或长期设计职责，例如 D01 项目设计规范、S02 户型图册、P01 项目海报。

**Session** 是完成该 Task 的一次连续 AI 工作过程。一个 Task 可以有多个 Session，例如：

- S02 / `初版结构探索`
- S02 / `销售团队修改`
- S02 / `最终校对`

切换 Task 时，系统恢复该 Task 最近使用的 Session；用户也可以在该 Task 内切换其他 Session。

### 4.3 固定与动态任务

固定任务：

| ID | 名称 | 默认 Preview | Chat 主要能力 |
| --- | --- | --- | --- |
| D01 | Project Design Specification | 设计系统画板/规范页 | 品牌分析、颜色、字体、版式、组件、审批 |
| S01 | Project sales brochure | 多页画册 | 文案、页面结构、图片编排、PDF |
| S02 | Floorplan book | 户型图册 | 户型筛选、图纸检查、图注、分页、PDF |
| S03 | Unit sales sheet | 单户型销售页 | 单元事实、户型图、卖点、CTA、A4/PDF |
| S04 | Price & availability | 价格与房源表 | 授权数据导入、表格、状态、版本 |
| S05 | Finishes & specifications | 材料与精装规格册 | 规格组织、缺失项、免责声明、PDF |
| S06 | Agent kit | Agent Kit / 演示稿 | 话术、FAQ、演示结构、导出 |
| S07 | Showroom & EOI pack | 展厅与 EOI 资料包 | 流程、表单说明、合规文案、PDF |

动态任务由 `+ New design task` 创建：

- `Pxx`：AI Poster / Campaign Creative；
- `Vxx`：Video / Storyboard；
- `Cxx`：其他自定义设计任务。

AI Poster、A4 Document 和 Video 不再作为 Creative Studio 顶部的全局模式按钮。它们改为：

1. 新建设计任务时可选择的任务模板；
2. 右侧 Chat 中可调用的工具；
3. 当前 Task 决定默认启用的工具组合。

## 5. 新版信息架构

### 5.1 桌面布局

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ Koya · Creative Studio       D01  S01  S02  S03  S04  S05  S06  S07  + Task │
│                              当前状态/未读/生成中提示                         │
├────────────────────────────────────────────────┬─────────────────────────────┤
│                                                │ S02 · Floorplan book        │
│                                                │ Session: 初版结构探索   ▾  + │
│                 LIVE PREVIEW                   ├─────────────────────────────┤
│                                                │ AI CHAT                     │
│     当前任务、当前 session、当前 revision       │                             │
│     的画布 / A4 / 多页 / 图片 / 视频预览         │ 对话记录                    │
│                                                │ 工具结果与待审阅变更          │
│                                                │                             │
│                                                ├─────────────────────────────┤
│                                                │ ＋附件  工具  上下文  Send  │
├────────────────────────────────────────────────┴─────────────────────────────┤
│ Revision / evidence / warnings / export status                                │
└──────────────────────────────────────────────────────────────────────────────┘
```

推荐桌面宽度分配：

- Preview：`minmax(620px, 1fr)`，占可用内容宽度约 62–68%；
- AI Chat：`360–440px`，占约 32–38%；
- 顶部 Task Bar：单行固定，任务过多时横向滚动；
- Chat 可折叠，但默认展开；
- 不再保留独立的中间 Ask Codex 栏。

### 5.2 顶部 Task Bar

每个任务按钮至少显示：

- Task ID；
- 简称；
- 状态点：Not started / Draft / Preview ready / Approved / Blocked；
- 当前活动任务高亮；
- 生成中、需要审阅或存在 warning 时显示非干扰提示。

Task Bar 行为：

- 点击 S02：保存当前 session UI 状态，打开 S02，并恢复 S02 最近活动 session；
- 点击 D01：恢复 D01 session 与对应 Preview；
- 切换不能复制上一任务的输入框内容、附件、draft 或 pending action；
- 有未保存编辑时，先自动保存为 session draft；若无法安全自动保存，显示离开确认；
- `+ Task` 打开任务模板菜单，而不是传统大表单。

### 5.3 中间 Preview

Preview 是画布核心，必须始终标明：

- 当前 Task；
- 当前 Session；
- 当前 Revision；
- Preview 状态；
- 数据/证据 warning；
- 输出格式与尺寸。

Preview 支持按任务切换渲染器：

- D01：设计规范画板与组件预览；
- A4/销售资料：HTML print preview、多页导航、缩放、PDF；
- Poster：图片画布、尺寸变体、可靠文字覆盖层；
- Video：Storyboard、时间线和视频预览；
- 空任务：显示任务目标、所需证据和建议的第一条 AI 操作。

Preview 只显示已生成的 revision 或明确的草稿状态。Chat 中尚未确认的建议不得静默替换当前已保存 Preview。

### 5.4 右侧 AI Chat

Chat 是 Creative Studio 的主要控制面板，不是通用问答窗口。

Chat 同时也是 Codex App Server 的原生事件客户端。用户看到的消息、工具卡片、命令状态、文件变更、审批请求和完成状态，均来自当前 task/session 所映射的 Codex thread，不由前端伪造工作过程。

Chat Header 显示：

- 当前任务 ID 与名称；
- 当前 session 名称；
- session 下拉菜单；
- `+ New session`；
- `...` session 管理菜单；
- 当前上下文摘要，例如 `Koya / S02 / 8 accepted sources / Design Spec approved`。

Chat 消息区支持：

- 普通用户与 AI 消息；
- 工具调用卡片；
- 附件与 evidence 引用；
- 变更提案 diff；
- Preview revision 卡片；
- warning、blocked 和 approval gate；
- `Apply to preview`、`Discard`、`Save revision`、`Approve` 等明确动作。

Codex 运行事件需要映射为可读 UI：

| Codex 事件/请求 | Creative Studio 呈现 |
| --- | --- |
| Agent message delta | 流式 AI 回复 |
| Item started / completed | 工作步骤与完成状态 |
| Command execution | 命令、工作目录、输出摘要 |
| File change | 修改文件、diff 和 Preview 影响 |
| Tool call | 设计工具卡片与结果 |
| Request user input | Chat 内澄清问题 |
| Approval request | Accept / Decline / Cancel 审批卡片 |
| Turn completed | 完成、失败或 interrupted 状态 |

Composer 支持：

- 自然语言输入；
- 添加项目内素材；
- 选择任务允许的工具；
- 查看/调整本次上下文；
- 停止当前请求；
- 发送快捷指令。

## 6. Chat 多功能能力

### 6.1 通用能力

- 读取当前项目 manifest 和当前 Task 的 evidence scope；
- 检查 unsupported claims；
- 起草和修改文案；
- 提出版式与视觉方向；
- 比较当前 revision 与提案；
- 创建新 revision；
- 解释 warning 与缺失数据；
- 从已接受项目素材中选择附件；
- 生成导出前检查清单。

### 6.2 按任务启用的工具

| 工具 | 适用任务 | 关键限制 |
| --- | --- | --- |
| Design system editor | D01 | 修改后需重新审批；不改项目事实 |
| SOT builder | S01–S07 | 只读已接受证据；缺失数据为 unavailable |
| Markdown editor | S01–S07 | 必须记录 SOT hash |
| HTML renderer | S01–S07 | 只从 Markdown 再生成 |
| PDF exporter | S01–S07 | 不能绕过设计规范与事实 warning |
| Floorplan inspector | S02、S03 | 官方图纸优先；不得替换相似户型 |
| Image composition | D01、S01–S07、Pxx | 区分官方、render 与 AI imagery |
| Image generation | Pxx 等 | 每次付费生成需单独审批与价格状态 |
| Storyboard/video authoring | Vxx | 生成视频仍受独立审批与 QA gate 控制 |
| Evidence checker | 所有任务 | Chat 内容本身不是 evidence |

工具可以由 AI 建议，但任何会写入 Task SoT、创建付费任务、覆盖已批准 revision 或发布输出的操作，都必须显示明确的人类确认按钮。

## 7. Session 管理

### 7.1 Session 生命周期

```text
active -> inactive -> compacted -> archived
                    \-> deleted
```

状态定义：

- `active`：当前任务正在使用；
- `inactive`：保留完整记录，可继续；
- `compacted`：完整 transcript 已被受控摘要替代，后续使用新 provider session；
- `archived`：默认不显示，但可恢复；
- `deleted`：本地删除；执行前必须二次确认。

### 7.2 必需操作

- New session；
- Rename；
- Switch；
- Duplicate context as new session；
- Compact；
- Archive / Restore；
- Delete；
- Export transcript and tool log；
- Stop 当前运行中的精确 request。

### 7.3 默认规则

- 首次打开一个 Task 时，自动创建 `Session 1`；
- 再次打开 Task 时，恢复 `lastActiveSessionId`；
- session 名称可先自动取自首条请求，再由用户修改；
- 每个 session 保存独立 composer draft、滚动位置、附件、上下文选择与 Preview revision；
- session 可以引用同一 Task 的已保存输出，但不得共享未保存草稿；
- 每个 session 保存对应的 Codex `threadId` 和 `sessionId`；
- 每次用户发送创建一个 Codex turn，并保存精确 `turnId`；Stop 通过该 `threadId + turnId` 只中断当前 turn；
- 运行中的补充要求优先使用 Codex turn steering，不重新创建独立请求；
- Compact 直接调用 Codex thread compaction，并在本地保存可读摘要、关键决策、未完成事项、asset ID 和 revision ID；
- Delete 不删除已正式保存的 Task output；如 output 只存在于该 session，删除前必须明确提示。

### 7.4 Session 与证据边界

- Session transcript 是本地辅助记忆，不是项目 evidence；
- AI 从聊天中发现的新事实必须进入 `proposed evidence` 流程，不能直接写入 SOT；
- 只有项目 manifest 与 accepted evidence 可以支撑正式文档事实；
- Session summary 不能成为审批、发布或法律条款的唯一来源；
- Session 引用的素材需记录 asset ID、checksum 与当时 evidence status。

## 8. 建议数据模型

```ts
type CreativeTask = {
  id: string;                    // D01, S02, P01, V01
  projectId: string;
  type: 'design_spec' | 'sales_document' | 'poster' | 'video' | 'custom';
  templateId: string;
  title: string;
  status: 'not_started' | 'draft' | 'preview_ready' | 'approved' | 'blocked';
  evidenceAssetIds: string[];
  outputIds: string[];
  sessionIds: string[];
  lastActiveSessionId?: string;
  createdAt: number;
  updatedAt: number;
};

type CreativeSession = {
  id: string;
  projectId: string;
  taskId: string;
  name: string;
  status: 'active' | 'inactive' | 'compacted' | 'archived';
  codexThreadId: string;
  codexSessionId: string;
  activeTurnId?: string;
  lastTurnStatus?: 'inProgress' | 'completed' | 'failed' | 'interrupted';
  cachedItemIds: string[];
  attachedAssetIds: string[];
  selectedContext: CreativeContextSelection;
  activeRevisionId?: string;
  composerDraft?: string;
  compactedSummary?: string;
  createdAt: number;
  updatedAt: number;
  lastOpenedAt: number;
};

type CreativeMessage = {
  id: string;
  sessionId: string;
  codexItemId: string;
  turnId?: string;
  role: 'user' | 'assistant' | 'tool' | 'system_boundary';
  content: string;
  assetRefs: string[];
  revisionRefs: string[];
  createdAt: number;
};

type CreativeRevision = {
  id: string;
  taskId: string;
  sessionId: string;
  parentRevisionId?: string;
  status: 'proposal' | 'preview' | 'saved' | 'approved' | 'rejected';
  renderer: 'design_board' | 'html_print' | 'image' | 'storyboard' | 'video';
  sourceHashes: Record<string, string>;
  outputRelativePaths: string[];
  createdAt: number;
};
```

建议在项目目录中保存：

```text
creative/
├── tasks.json
├── sessions/
│   └── <task-id>/
│       └── <session-id>.json
├── revisions/
│   └── <task-id>/
└── sales-sheets/
    └── <task-id>/
        ├── SOT.md
        ├── document.md
        ├── document.html
        └── brief.json
```

不建议把完整 Chat transcript 直接塞入 `manifest.json`，避免 manifest 变大、写入冲突和事实层被对话污染。Manifest 只保存 Task/Session 索引与最近活动指针。

Codex thread 是对话历史和 agent 事件的权威来源。Estate Studio 只保存 task/session 到 Codex thread 的映射、UI 缓存、revision 引用和产品审计信息；不得维护一套与 Codex thread 相互竞争的“伪聊天历史”。

### 8.1 Codex 原生集成决策

正式实现采用 Codex App Server，而不是每条消息启动一次 `codex exec`。

原因：Codex App Server 是 Codex 为富客户端提供的嵌入接口，覆盖认证、持久 conversation history、审批和流式 agent 事件；这与 Creative Studio 的任务级长期设计工作完全匹配。

当前 Estate Studio 状态：

- 已能启动 App Server 并通过 account read 检查用户的官方 Codex 登录状态；
- 实际 Chat 仍通过一次性 `codex exec --json` 执行；
- 每次请求只手工附带最近少量历史；
- 没有持久 Codex thread、turn steering、interrupt、原生 tool/file-change event 或 session resume。

目标状态：

```text
Estate Studio Desktop
└── Long-running Codex App Server connection
    ├── initialize / initialized
    ├── account state
    ├── D01 Session A -> Codex thread A
    ├── S02 Session A -> Codex thread B
    ├── S02 Session B -> Codex thread C
    └── streamed thread / turn / item / approval events
```

协议映射：

| Creative Studio 操作 | Codex App Server 流程 |
| --- | --- |
| New session | Start a new Codex thread |
| Open existing session | Resume the stored Codex thread ID |
| Duplicate context | Fork the Codex thread |
| Send message | Start a turn on the mapped thread |
| Add instruction while working | Steer the active turn |
| Stop | Interrupt the exact active turn |
| Read inactive session | Read stored thread without activating it |
| Compact | Start thread compaction |
| Archive / Restore | Archive / unarchive mapped thread plus local session index |
| Stream Chat UI | Subscribe to thread, turn and item events |

不得通过截取最近 8 条消息拼接 prompt 来模拟 session resume。恢复必须基于持久 `codexThreadId`。

### 8.2 App Server 连接与生命周期

- Estate Studio 启动后建立一个受管理的长驻 App Server 连接，而不是每条消息创建一个 CLI 子进程；
- 每次连接先执行初始化握手，再允许 thread 或 turn 请求；
- App 退出时安全终止本 App 启动的进程；不能结束用户其他 Codex 任务；
- App Server 异常退出时，UI 保留 session 映射并显示 disconnected；重连后通过 thread resume 恢复；
- 所有协议请求使用唯一 JSON-RPC request ID；
- 所有 event 先按 `threadId`、`turnId`、`itemId` 路由，再更新对应 task/session；
- 不同 Task 的事件不得进入当前 Chat，只在顶部任务按钮显示后台完成或需要处理状态。

### 8.3 Task 工作目录与权限

每个 Codex thread 必须在当前项目和当前 Task 的明确工作范围中启动：

```text
Project root                         read only where possible
├── manifest and accepted evidence  readable context
└── creative/<task-id>/              writable task workspace
```

- `cwd` 指向当前 task workspace；
- 项目 manifest、accepted evidence 和批准的 design spec 可读；
- 默认只允许写当前 task workspace 与其 revision 目录；
- 项目级字段更新通过 Estate Studio 的受控 action，不允许 Agent 任意改 manifest；
- command、file change、network、MCP 和其他工具审批使用 Codex 原生审批请求，并叠加 Estate Studio 的业务 gate；
- MVP 不依赖实验性的动态工具协议；设计专用能力优先通过版本化 Skill、受控本地命令或稳定 MCP 工具暴露；
- 当前 task 的 `AGENTS.md`/Skill 指令必须写明证据边界、输出目录、渲染命令和禁用操作。

### 8.4 Preview 同步机制

Codex 不直接向前端“画一张假 Preview”。它修改或生成任务目录中的真实源文件，Estate Studio 再从受控 artifact 构建 Preview：

```text
Codex file/tool result
-> task source changed
-> validate lineage and schema
-> render proposal revision
-> refresh centre Preview
-> user Apply / Save / Approve
```

- 文件变更事件可以触发预览准备状态，但最终 Preview 以磁盘文件和验证结果为准；
- 未通过验证的变更显示为 failed proposal，不覆盖已保存 Preview；
- S01–S07 必须重新校验 SOT/Markdown/HTML hash；
- Preview revision 记录来源 `threadId + turnId + itemId`；
- Undo/Restore 作用于 Estate Studio revision，不尝试篡改 Codex 历史。

## 9. 关键交互流程

### 9.1 打开 S02

1. 用户点击顶部 `S02 Floorplan book`。
2. 系统保存当前 Task 的 UI 草稿状态。
3. 读取 S02 的 `lastActiveSessionId` 和对应 `codexThreadId`。
4. 若不存在 session，创建 `S02 / Session 1` 并启动新的 Codex thread；若存在则 resume 原 thread。
5. 中间加载 S02 active revision；没有 revision 时加载空状态。
6. 右侧加载该 session 的 messages、附件、工具记录与 composer draft。
7. Chat 工具自动切换为 Floorplan / SOT / Markdown / HTML / PDF 能力。

### 9.2 从 S02 切换到 D01

1. S02 未发送的输入保存到 S02 session。
2. 清除 S02 的 pending proposal 在页面上的活动状态，但不删除记录。
3. 激活 D01 及其最近 session。
4. Preview 切换到设计规范画板。
5. Chat 上下文切换到 D01，不携带 S02 transcript。
6. 仅共享项目级 accepted evidence 与已批准 Design Spec 等明确的跨任务依赖。

### 9.3 在 Chat 中创建输出

1. 用户提出修改或生成要求。
2. AI 显示本次使用的 Task、evidence 和工具。
3. AI 返回 proposal 或 revision，不直接覆盖当前保存输出。
4. 用户在 Chat 卡片或 Preview 中选择 Apply / Discard。
5. Apply 后 Preview 显示新 revision。
6. Save 后写入 Task output；涉及 S01–S07 时，重新生成完整 SOT/Markdown/HTML 血缘。
7. Approve、付费生成、PDF export 或 publish 继续遵守各自 gate。

## 10. 错误与并发处理

- Task 或 session 加载失败时，Preview 与 Chat 都显示同一错误边界，不显示旧任务内容。
- provider 不可用时，session 仍可查看和管理，但发送按钮说明连接状态。
- 请求运行中切换 Task 时，请求继续绑定原 task/session；顶部显示生成中状态，完成后显示提示，不把结果注入当前 Task。
- 同一 Task 多 session 同时生成时，每个结果写回发起它的 session。
- 项目 `updatedAt` 冲突时，不静默覆盖；保留 proposal 并要求刷新或另存 revision。
- source hash 变化时，将相关 Preview 标记 stale，禁止误认为已同步。

## 11. 无障碍与桌面适配

- Task 按钮、session 菜单、Chat 工具和 Preview 操作均可通过键盘访问；
- 当前 task、session 和状态不能只依赖颜色表达；
- 1280px 及以上保持 Preview + Chat 双栏；
- 900–1279px 允许 Chat 宽度缩小到 340px；
- 小于 900px 时，Preview 与 Chat 使用可切换面板，不同时压缩显示；
- Chat 最小可用宽度 340px；Preview 不得因 Chat 展开而让 A4 文本不可读。

## 12. MVP 范围与实施顺序

Phase 1 与 Phase 2 是同一个 MVP 发布门槛。只有新版三栏布局、但仍使用一次性 `codex exec` 的界面，不得标记为 Creative Studio AI Chat 完成。

### Phase 1：任务切换与布局

- 移除独立 Ask Codex 中栏；
- 顶部加入 D01、S01–S07 Task Bar；
- 中间 Preview、右侧 Chat；
- AI Poster/A4/Video 入口迁入 Chat 与 New Task；
- 为每个固定任务创建 task record。

### Phase 2：本地 Session 基础能力

- 长驻 Codex App Server manager 与事件路由；
- 每 Task 独立 session，并持久化 Codex thread 映射；
- New / Rename / Switch / Archive / Restore；
- thread start / resume / read / archive / unarchive；
- 保存 composer draft、附件和 active revision，消息从 Codex thread 读取；
- 切换任务恢复最近 session；
- turn start / steer / interrupt；
- 流式 agent、tool、command、file-change 与 approval UI。

### Phase 3：Chat 工具与 Revision

- Task-aware tool registry；
- proposal -> preview -> save 状态；
- revision history；
- SOT/Markdown/HTML/PDF 工具卡片；
- Poster 与 Video 动态任务。

### Phase 4：高级 Session 管理

- Codex thread compaction；
- Codex thread fork；
- Delete 与输出依赖检查；
- transcript/tool log export；
- stale context 与跨 revision 对比。

## 13. MVP 验收标准

1. 打开 Koya Creative Studio 时，顶部可见 D01、S01–S07。
2. 点击 S02 后，Preview 与 Chat Header 均显示 S02。
3. 在 S02 输入未发送文本，切到 D01，再切回 S02，文本仍在 S02 session 中。
4. D01 不显示 S02 的 transcript、附件或 pending proposal。
5. 每个 Task 首次打开时创建自己的 Session 1。
6. 一个 Task 可创建并切换至少两个 session。
7. session 可以重命名、归档和恢复。
8. 删除 session 前显示其未保存输出影响；删除操作需要确认。
9. AI Poster、A4 Document、Video 不再作为页面一级模式切换器。
10. Chat 可以按当前 Task 显示适用工具，并隐藏或禁用不适用工具。
11. Chat proposal 不会自动覆盖当前已保存 Preview。
12. S01–S07 保存后仍生成可验证的 SOT、Markdown 和 HTML 血缘；PDF 为派生输出。
13. 切换 Task 时，运行中的请求结果只写回原 task/session。
14. Chat transcript 不会被记录为 accepted project evidence。
15. 原生 macOS App 在 1280px 宽度下完整显示 Preview 与 Chat，不出现横向页面溢出。
16. 每个 Creative Session 都持久映射到一个真实 Codex thread；重新启动 App 后可以 resume。
17. AI 回复、工具、命令和文件变更状态由 App Server 流式事件驱动，不使用伪造进度文案。
18. 用户在运行中追加要求时可以 steer 当前 turn；Stop 只 interrupt 当前 `threadId + turnId`。
19. Codex 修改任务文件后，必须通过磁盘 read-back 与验证才能生成新的 Preview revision。
20. App Server 断开重连后，不丢失 Task/Session 映射，也不会把事件写入错误任务。

## 14. 产品决策摘要

- Creative Studio 的基本单位从“输出类型”改为“设计任务”。
- AI Chat 是核心操作入口，但视觉主画布仍是 Preview。
- 顶部只负责 Task 切换；右侧 Chat 负责所有设计能力和 session 管理。
- 每个 Task 至少有一个独立 Session，一个 Task 可以有多个 Session。
- Task 之间共享受控的项目证据和已批准依赖，不共享未保存对话上下文。
- AI Poster、A4 和 Video 是 Task 模板与 Chat 工具，不是一级页面。
- Session 是本地工作记忆，不是 SoT、审批记录或官方证据。
- 每个 Session 由真实 Codex thread 驱动；Estate Studio 不自建第二套对话引擎。
- Chat 的工作过程来自 Codex 原生 turn/item/tool/file-change 事件，并由 Estate Studio 负责设计任务 UI、权限、Preview 与业务审批。

## 15. 官方实现依据

- OpenAI Codex App Server：<https://developers.openai.com/codex/app-server>
- 采用 App Server 的原因：它面向富客户端深度集成，并提供认证、conversation history、approvals 和 streamed agent events。
- PRD 使用的 thread start/resume/fork/read、turn start/steer/interrupt、thread compaction、item events 与 approval requests 均以该官方协议为实现基线。
- App Server 或 Codex CLI 版本升级时，应重新生成/核对协议 schema，并运行兼容性测试；不能假设实验字段长期稳定。
