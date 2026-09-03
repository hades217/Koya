# Controlled submission command

Status: `NOT AUTHORISED / DO NOT RUN BEFORE CURRENT PASS AND EXPLICIT USER APPROVAL`

The provider requires first-frame I2V to inherit aspect ratio from the first frame. Therefore this command intentionally contains no `--ratio` flag. The 1672 x 941 first frame is effectively 16:9.

```bash
prompt_text=$(<PROMPT.txt)
ARKCLI_NO_UPDATE_NOTIFIER=1 \
ARKCLI_CALLER_TYPE=ai_agent \
ARKCLI_CALLER_NAME=codex \
ARKCLI_SKILL_NAME=arkcli-gen \
arkcli +gen \
  --model ep-20260812221158-hb576 \
  --modality video \
  --input first_frame:@inputs/00-first-frame.png \
  --duration 8 \
  --resolution 1080p \
  --generate-audio=true \
  --watermark=false \
  --return-last-frame \
  --save-to outputs \
  --format json \
  "$prompt_text"
```

One approval authorises one task and one output. Never auto-retry.
