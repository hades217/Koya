#!/bin/zsh
set -euo pipefail

submission_dir="${0:A:h}"
project_dir="${submission_dir:h:h}"
prompt_text="$(<"${submission_dir}/KOYA_SEEDANCE_2_5_MASTER_PROMPT_V2.txt")"

ARKCLI_CALLER_TYPE=ai_agent \
ARKCLI_CALLER_NAME=codex \
ARKCLI_SKILL_NAME=arkcli-gen \
arkcli +gen \
  --model ep-20260812221158-hb576 \
  --modality video \
  --input reference_video:@"${submission_dir}/stage-01-camera-blockout-30s-v2.mp4" \
  --input reference_image:@"${project_dir}/production-assets/assets/official/exterior/koya-building-hero.jpg" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-01-HIGH-AERIAL-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-02-MID-AERIAL-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-03-LOW-AERIAL-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-04-STREET-FRONT-DAY.png" \
  --input reference_image:@"${project_dir}/production-assets/assets/generated/character/sports-female/master-panels-v2/back-full.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-05-RESIDENT-APPROACH-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-06-ENTRY-THRESHOLD-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-07-FOYER-LIFT-CALL-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-08-LIFT-ENTRY-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-09-LIFT-ASCENT-COVER-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-10-LIFT-EXIT-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-11-APARTMENT-DOOR-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-12-APARTMENT-THRESHOLD-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-13-APARTMENT-HALL-DAY.png" \
  --input reference_image:@"${project_dir}/storyboards/stage-01-opening-redesign-v2/frames/SB-V2-14-LIVING-REVEAL-DAY.png" \
  --input reference_image:@"${project_dir}/production-assets/assets/official/floorplans/apartment-106.png" \
  --duration 30 \
  --ratio 16:9 \
  --resolution 720p \
  --generate-audio \
  --return-last-frame \
  --output-format mp4 \
  --priority 0 \
  --extra-body '{"omni_reference_task_type":"reference"}' \
  --save-to "${submission_dir}/outputs" \
  --no-open \
  --dry-run \
  --format json \
  "${prompt_text}"
