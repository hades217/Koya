# Seedance 2.5 capability audit

Date: 2026-08-16  
Status: verified against the current Volcengine Ark documentation, live endpoint metadata and downloaded official packages  
Paid generation performed during this audit: none

## Local official material

- Official ByteDance AgentKit repository: `video-production/seedance-2.5-capability-pack/bytedance-agentkit-samples/`
- Relevant official skills:
  - `skills/byted-ark-seedance-guide/`
  - `skills/byted-sol-seedance-prompt/`
  - `skills/v1/video-generate/`
- Official Ark Seedance 2.5 SDK quickstart archive: `video-production/seedance-2.5-capability-pack/ark_seedance2.5_quickstart_package.zip`
- Extracted quickstart: `video-production/seedance-2.5-capability-pack/official-quickstart/ark_seedance2.5_quickstart_package/`

The SkillHub market item `volcengine/seedance/byted-seedance-video-generate` was found as version 1.0.0. `arkcli agent skill download` cannot download public SkillHub items because that command only retrieves custom managed-agent skills. The same official ByteDance AgentKit source repository was therefore cloned locally. Its generic generation skill predates Seedance 2.5, while the repository's current Ark guide and the official 2.5 quickstart supply the current 2.5 workflow.

## Verified Seedance 2.5 capabilities

- Model ID: `doubao-seedance-2-5-260628`.
- The configured endpoint `ep-20260812221158-hb576` is running and resolves to this model.
- Inputs: text, image, video and audio. Output: video.
- Supported task families: multimodal-to-video, video editing and video extension.
- A single generation supports 4–30 seconds; 30 seconds is a native single output, not two clips joined together.
- Output resolutions currently listed for Seedance 2.5: 480p and 720p.
- Output ratios include 16:9 and `adaptive`.
- Reference capacity: up to 30 images, 10 videos and 10 audio files, with up to 50 reference items in total; reference video/audio total duration is up to 30 seconds.
- Native audio generation is supported.
- Returning the last frame is supported.
- The official guide explicitly demonstrates multiple keyframe images controlling a single one-take sequence.

## Task-mode constraints that matter for Koya

### First-frame / first-and-last-frame mode

- Image roles are `first_frame` and optionally `last_frame`.
- `ratio` must be `adaptive`; output ratio follows the input frame.
- This mode does not provide the multi-reference route control required for Koya.
- The live service rejected mixing a `last_frame` item with `reference_image` or `draft_task` content. Therefore this is not the correct mode for the 18-anchor walkthrough.

### Full-modal reference generation

- Use `reference_image`, `reference_video` and/or `reference_audio` roles.
- Each reference must have one explicit job in the prompt: geography, exact facade, character identity, foyer layout, lift continuity, corridor or soundtrack.
- For an original one-take generation, use the reference-generation intent, not edit or extend wording.
- A 30-second 16:9 output can be requested directly.

### Video editing

- Requires at least one reference video and an explicit edit intent.
- `ratio` must be `adaptive`, `duration` must be `-1`, and the reference video must be 4–30 seconds.
- This is suitable for repairing an already approved 30-second take, not for the first Koya generation.

### Video extension

- Requires a reference video and explicit extension wording.
- `ratio` must be `adaptive`.
- This is a fallback for a later continuation, not the requested first 30-second one-take.

## Why the rejected attempts failed

The 5-second pair violated the single-generation requirement. The 10-second attempt used only first/last-frame control, which gave the model too much freedom over the intermediate geography, building and movement. It also spent only one third of Seedance 2.5's available native duration. A technically continuous file is not the same as a storyboard-faithful one-take.

## Mandatory gate before the next paid task

1. One provider task only: 30 seconds, 16:9, 720p, native audio.
2. Full-modal reference generation, not first/last-frame mode.
3. The approved 18-frame route and locked character are passed as explicitly assigned references.
4. The official Koya facade is the only readable building geometry.
5. The request payload and estimated cost are shown to the user before submission.
6. No task is submitted until the user approves the reference manifest and 30-second timeline.

## Official source

- Seedance 2.5 tutorial: https://docs.volcengine.com/docs/82379/2607688
