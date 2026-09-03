# Exact controlled command

This document records the reviewed command shape. It is not authorisation to execute it.

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
  --duration 5 \
  --ratio 16:9 \
  --resolution 1080p \
  --generate-audio=true \
  --watermark=false \
  --return-last-frame \
  --save-to outputs \
  --format json \
  "$prompt_text"
```

One approval authorises one execution only. Do not add `--wait`, `--draft`, a second input, a second task or an automatic retry without a new review.
