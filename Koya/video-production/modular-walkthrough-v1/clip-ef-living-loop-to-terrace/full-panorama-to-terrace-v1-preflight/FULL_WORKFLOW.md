# Clip EF 完整提交工作流

状态：`PLANNED / PREFLIGHT BLOCKED / NO TASK SUBMITTED`

目标：一次 8 秒的第一人称连续看房镜头，完整执行：MPR -> Dining -> Kitchen -> TV wall -> sofa and wrap glazing -> east sliding bay -> Terrace。

## 阶段 1：官方空间核对

1. 打开 `Koya marketing plan Apartment 106.pdf`。
2. 使用 `floorplan-audit-v5/public-zone-crop.png` 核对公共区。
3. 锁定 MPR 正确大开口、四人 Dining、唯一 Kitchen island、内部 TV wall、南/东连续玻璃立面和东侧露台开口。
4. 任何与平面图冲突的渲染图移出有效输入集合。

## 阶段 2：完整分镜核对

按固定顺序检查九张 V7 渲染级分镜：

1. `01-mpr-dining-opening.png`
2. `02-dining-single-island-kitchen.png`
3. `03-tv-wall-living-width.png`
4. `04-wrap-glazing-sliding-bay.png`
5. `05-align-east-sliding-bay.png`
6. `06-threshold-approach.png`
7. `07-cross-east-threshold.png`
8. `08-terrace-arrival.png`
9. `09-terrace-lookback-endpoint.png`

这些图片必须全部进入人工审核材料，任何一个步骤都不能从最终 Prompt 和 QA 清单中消失。

## 阶段 3：构建真正的连续空间控制输入

1. 从同一个固定场景生成一条完整 8 秒参考视频。
2. 参考视频必须连续经过九个分镜检查点。
3. 参考视频不能是静态图放大、幻灯片、图片交叉溶解或独立渲染帧拼接。
4. MPR、Dining、Kitchen、TV wall、sofa、glazing、door track 和 Terrace 必须在同一个空间模型中保持固定。
5. 旧 V6、V8、V9 和当前被拒绝的 I2V V3 均不得作为该参考视频。

## 阶段 4：提交内容准备

Seedance 实际提交必须包含：

- 一条已通过的完整 8 秒连续参考视频，角色为 `reference_video`。
- 完整 `PROMPT_DRAFT.txt`，逐秒写明九个步骤。
- 8 秒、1080p、生成音频、关闭水印、单任务、单输出。
- 若当前接口要求 Web URL，必须上传参考视频并验证远端字节数和 SHA-256 与本地一致。

人工审核包必须同时包含：

- 官方 Apartment 106 floor plan。
- 公共区 route overlay。
- 九张 V7 渲染级分镜。
- 分镜 contact sheet。
- 连续参考视频的 8 fps 全片检查和门槛 12 fps 检查。

## 阶段 5：Skills 提交前审查

1. 使用 `offplan-property-one-take-storyboard` 核对路线、门槛、转向和分镜覆盖。
2. 使用 `offplan-property-sales-walkthrough-video` 核对买家可读性、真实移动和模块边界。
3. 使用 `seedance-preflight-review` 审查完整 Prompt、输入角色、参考视频、URL、参数、价格、任务数和输出数。
4. 只有 `PASS` 才生成 package fingerprint。
5. 把完整 manifest、Prompt、全部实际输入和指纹展示给用户。
6. 用户针对该精确指纹批准一次后，才能创建一次任务和一个输出。

## 阶段 6：生成后验收

1. 只轮询同一 task ID。
2. 下载原始 MP4。
3. 检查 duration、dimensions、codec、fps 和 audio。
4. 全片按 8 fps 检查；转向和玻璃门跨越按至少 12 fps 检查。
5. 对照九个必到检查点逐项验收。
6. 缺少任何一个环顾步骤、出现空间跳变或未跨门槛，立即标记 `REJECTED / DO NOT STITCH`。

## 当前停止点

完整连续参考视频不存在，因此现在不能通过 Seedance preflight，也不能付费提交。
