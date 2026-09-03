#!/bin/zsh
set -euo pipefail

submission_dir="${0:A:h}"
project_dir="${submission_dir:h:h}"
prompt_text="$(<"${submission_dir}/KOYA_SEEDANCE_2_5_MASTER_PROMPT_V2.txt")"
public_base_url="${KOYA_SEEDANCE_PUBLIC_BASE_URL:?Set KOYA_SEEDANCE_PUBLIC_BASE_URL to the temporary HTTPS asset origin}"

mkdir -p "${submission_dir}/outputs"

ARKCLI_CALLER_TYPE=ai_agent \
ARKCLI_CALLER_NAME=codex \
ARKCLI_SKILL_NAME=arkcli-gen \
arkcli +gen \
  --model ep-20260812221158-hb576 \
  --modality video \
  --input reference_video:"${public_base_url}/reference-video.mp4" \
  --input reference_image:"${public_base_url}/koya-building-hero.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-01-HIGH-AERIAL-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-02-MID-AERIAL-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-03-LOW-AERIAL-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-04-STREET-FRONT-DAY.jpg" \
  --input reference_image:"${public_base_url}/character-back-full.png" \
  --input reference_image:"${public_base_url}/SB-V2-05-RESIDENT-APPROACH-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-06-ENTRY-THRESHOLD-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-07-FOYER-LIFT-CALL-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-08-LIFT-ENTRY-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-09-LIFT-ASCENT-COVER-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-10-LIFT-EXIT-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-11-APARTMENT-DOOR-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-12-APARTMENT-THRESHOLD-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-13-APARTMENT-HALL-DAY.jpg" \
  --input reference_image:"${public_base_url}/SB-V2-14-LIVING-REVEAL-DAY.jpg" \
  --input reference_image:"${public_base_url}/apartment-106.png" \
  --duration 30 \
  --ratio 16:9 \
  --resolution 720p \
  --generate-audio \
  --return-last-frame \
  --priority 0 \
  --extra-body '{"omni_reference_task_type":"reference"}' \
  --save-to "${submission_dir}/outputs" \
  --no-open \
  --format json \
  "${prompt_text}"
