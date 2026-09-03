# Clip EF V10 视觉锚点审查

## 角色区分

- `inputs/00-apartment-106-ef-v10-reference.mp4`：唯一提交给 Seedance 的 `reference_video`，控制摄影机、时间和空间拓扑。
- `qa/VISUAL_ANCHOR_V9_ACCEPTED_LOCAL.png`：项目方已接受的阳台/室内视觉方向，仅用于人工审查，不上传 Seedance。
- `qa/VISUAL_ANCHOR_V10_ALTERNATE.png`：同拓扑的阳台精修候选，仅用于人工审查，不上传 Seedance。
- `qa/PUBLIC_ZONE_ROUTE_OVERLAY.png`：官方户型图上的路线证据，不上传 Seedance。

## 一致性检查

| 项目 | V10 固定场景 | V9/V10 视觉锚点 | 结果 |
|---|---|---|---|
| MPR | 左侧紧凑空间，宽开口 | 保留 MPR 与公共区关系 | PASS |
| Dining | 四人餐桌 | 四人室内餐桌 | PASS |
| Kitchen | 一字型柜体和一个 island | 相同厨房/单岛台关系 | PASS |
| TV | 短内墙位置 | 相同边缘内墙电视关系 | PASS |
| Living | 计划比例、非酒店尺度 | 紧凑但有景深 | PASS |
| 外立面 | 南/东两面玻璃 | 两面玻璃 | PASS |
| 门槛 | 东侧同一开放门洞 | 同一室内外连接 | PASS |
| Terrace | 环绕露台、两椅、边几、四人户外餐桌 | 相同功能分区与清晰步行动线 | PASS |
| 时间/色彩 | 稳定白天 | 稳定自然白天 | PASS |

视觉锚点不是时间线控制，也不能覆盖官方户型或固定场景；它们只证明提示词要求的材质、家具气质和阳台吸引力有明确人工目标。
