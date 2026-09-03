#!/bin/zsh
set -euo pipefail

submission_dir="${0:A:h}"
prompt_text="$(<"${submission_dir}/KOYA_SEEDANCE_2_5_MASTER_PROMPT_V3.txt")"
public_base_url="${KOYA_SEEDANCE_PUBLIC_BASE_URL:?Set KOYA_SEEDANCE_PUBLIC_BASE_URL to the verified temporary HTTPS asset origin}"
mkdir -p "${submission_dir}/outputs"

ARKCLI_CALLER_TYPE=ai_agent \
ARKCLI_CALLER_NAME=codex \
ARKCLI_SKILL_NAME=arkcli-gen \
arkcli +gen \
  --model ep-20260812221158-hb576 \
  --modality video \
  --input reference_video:"${public_base_url}/stage-01-camera-control-30s-v4-1080p.mp4" \
  --input reference_image:"${public_base_url}/references/01-world-start-16x9.png" \
  --input reference_image:"${public_base_url}/references/02-official-facade-16x9.jpg" \
  --input reference_image:"${public_base_url}/references/03-character-back-16x9.png" \
  --input reference_image:"${public_base_url}/references/04-entry-foyer-16x9.png" \
  --input reference_image:"${public_base_url}/references/05-lift-entry-16x9.png" \
  --input reference_image:"${public_base_url}/references/06-lift-exit-16x9.png" \
  --input reference_image:"${public_base_url}/references/07-official-living-16x9.jpg" \
  --input reference_image:"${public_base_url}/references/08-apartment-106-plan-16x9.png" \
  --duration 30 \
  --ratio 16:9 \
  --resolution 720p \
  --generate-audio \
  --return-last-frame \
  --priority 0 \
  --extra-body '{"omni_reference_task_type":"reference"}' \
  --save-to "${submission_dir}/outputs" \
  --open \
  --format json \
  "${prompt_text}"
