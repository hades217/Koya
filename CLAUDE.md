# Koya project instructions

For all Koya Seedance, property-video, Apartment 106, storyboard, reference-media, and walkthrough work, you MUST read and obey:

- `Koya/video-production/SEEDANCE_2_5_RULES.md`

Do not rely only on general video-generation knowledge. The project rules define the required evidence, mode selection, approval gate, ArkCLI workflow, cost controls, continuity QA, and failure handling.

Critical constraints:

- Floor plan first; no invented topology.
- Multi-reference images are visual references, not guaranteed ordered keyframes.
- Architectural one-takes require a deterministic continuous reference video before paid Seedance work.
- Do not combine that video with independently generated whole-room style images. The generic `reference_image` role can influence geometry and composition; prompt wording cannot demote it to materials-only.
- Before every Seedance submission or retry, run `/Users/lightman/.codex/skills/seedance-preflight-review/SKILL.md`, obtain a current PASS plus package fingerprint, show the exact package to the user, and wait for explicit approval. Any package change invalidates the review.
- No paid generation without approval of one exact submission manifest.
- Never create two tasks or two variants from one approval.
- Poll an existing queued task instead of submitting another.
- Provider success is not acceptance; inspect dense frames and reject morphs, cuts, wall crossings, duplicated doors, scale drift, or lighting discontinuity.
