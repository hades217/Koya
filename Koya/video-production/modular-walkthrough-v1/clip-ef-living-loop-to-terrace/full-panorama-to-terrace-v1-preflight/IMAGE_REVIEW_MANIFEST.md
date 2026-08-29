# Clip EF 图片审查清单

## 结论

以下十张 16:9 渲染图全部进入提交前人工审查包，用来约束完整路线、Prompt 覆盖和成片逐帧验收。它们不能被描述成 Seedance 会按顺序执行的关键帧，因为当前 `reference_image` 角色没有这种已验证语义。

真正的拓扑控制输入仍必须是一条由同一固定场景渲染的连续 8 秒 `reference_video`。在该视频完成并通过审查前，付费提交保持阻塞。

## 图片与画面职责

| 编号 | 文件 | 尺寸 | SHA-256 | 审查职责 |
|---|---|---:|---|---|
| 00 | `00-master-public-zone-lock.png` | 1672x941 | `7c7e32ea154100a1dc8ba891b6bbe15b3a3d98b6175409e70f636bbfa09f4215` | 公共区整体空间与材质身份，不作为时间帧 |
| 01 | `01-mpr-dining-opening.png` | 1672x941 | `bbcc062237718b052ae746d6a828a08f5f9bf688cbe1939c28dbd018f343e31e` | 0.00–0.55 秒，MPR 宽开口与四人餐桌 |
| 02 | `02-dining-single-island-kitchen.png` | 1672x941 | `8e23b68b9c782a827581bb1b4647854d4f7d9b03e520bcf2fb5db5723c72c7e4` | 0.55–2.55 秒，餐厅、唯一中岛、厨房连续关系 |
| 03 | `03-tv-wall-living-width.png` | 1672x941 | `d7cac55298cb7464b2a3d2aa975523466af99d70526efa0357255995df57b1b3` | 2.55–3.35 秒，室内电视墙与客厅宽度 |
| 04 | `04-wrap-glazing-sliding-bay.png` | 1672x941 | `818e85e32ccd7a00546ae0de2a2d763a5e63fafd55a432e41ffca81cf740df5d` | 3.35–4.30 秒，沙发和南/东侧连续玻璃 |
| 05 | `05-align-east-sliding-bay.png` | 1672x941 | `ece6e45c5045f8906abe5bc3881512e258e3f8d27e4c305e4cac6a11904eac55` | 4.30–5.35 秒，对准同一东侧推拉门 |
| 06 | `06-threshold-approach.png` | 1672x941 | `5153f7526be5ed71bf2bbedc3c823c996b89be2273895b23a0cb2dbf57d7a681` | 5.35–6.35 秒，接近同一门槛并建立视差 |
| 07 | `07-cross-east-threshold.png` | 1672x941 | `f2057bff4f0268357b15794b0f02db08ee24df2e9f311ba86478646a08797e77` | 6.35–7.15 秒，橡木地板到轨道再到露台砖 |
| 08 | `08-terrace-arrival.png` | 1672x941 | `99411967ada9add57a81d8df13dd11ee138f1fc60e301fb5d51944e5f4b7f55c` | 7.15 秒后，露台抵达与户外纵深 |
| 09 | `09-terrace-lookback-endpoint.png` | 1672x941 | `5ff979687d6379a467bcd2db36eda657a27b41f43ce74f8446b0b160c96ab202` | 8.00 秒，从露台看回同一客餐厨终点 |

## Provider 输入角色

| 内容 | Provider role | 是否随当前任务上传 |
|---|---|---|
| 新建并获批的连续 8 秒固定场景视频 | `reference_video` | 必须；当前缺失 |
| 00–09 渲染图 | 无；人工审查与成片 QA | 不作为普通 `reference_image` 上传 |
| Apartment 106 官方户型图与路线图 | source-of-truth evidence | 不作为生成输入；用于预审与成片验收 |

## 为什么不是把十张图全部直接上传

十张图的内容必须全部覆盖，但不能把“全部覆盖”错误理解为“全部作为普通参考图上传”。这些图片为独立生成视角，不具备同一三维场景的时间连续性。Seedance 当前普通 `reference_image` 输入没有被验证为按 01→09 的顺序执行；直接上传会重新引入门窗、透视、空间宽度和家具位置冲突。

因此正确流程是：十张图约束人工审查 → 在一个固定场景中重建并渲染连续路线 → 逐帧与十张图核对 → 只把通过审查的连续视频作为空间与运镜输入 → 再运行 Seedance 提交前审查。
