# Seedance preflight review — I2V V4 no-reference-video experiment

Decision: `BLOCKED`

Reviewed at: 2026-08-29 Australia/Brisbane

No task submitted: yes

Package fingerprint: unavailable because the spatial evidence gate did not pass.

## Blockers

1. The task asks the camera to translate across an architectural threshold, but supplies only one first-frame image and no fixed-scene continuous path evidence.
2. The first frame shows the doorway and part of the Terrace, but it cannot prove the complete camera route, occlusion sequence, jamb parallax or post-threshold camera pose.
3. Koya architecture rules require a deterministic fixed-scene camera path before a paid topology-sensitive walkthrough submission.
4. The required normal-speed, reverse, 4 fps and dense 12 fps threshold review cannot be performed without a reference video.
5. The current account pricing read-back returned `IsOverdue: true`; the response did not expose a clearly named I2V 1080p price item, so exact cost is `unavailable`.

## Exact task reviewed

- Endpoint / resolved model: `ep-20260812221158-hb576` / `doubao-seedance-2-5-260628`
- Mode: first-frame image-to-video
- Task count / output count: `1 / 1`
- Parameters: `8 seconds`, `1080p`, aspect ratio inherited, generated audio enabled, watermark disabled, return last frame enabled
- Prompt: `PROMPT_ZH.txt`
- Output directory: `outputs/`

## Input and role

| Input | Role | Provenance | SHA-256 |
|---|---|---|---|
| `../living-panorama-storyboard-v2/rendered-storyboard-v7-plan-rebuild/00-master-public-zone-lock.png` | `first_frame` | Approved-looking independent render; not fixed-scene temporal evidence | `7c7e32ea154100a1dc8ba891b6bbe15b3a3d98b6175409e70f636bbfa09f4215` |

## Provider and price evidence

- Live endpoint: running; resolved to `doubao-seedance-2-5-260628`.
- Non-billable dry-run: request construction succeeded with one explicit `first_frame` input.
- Exact I2V 1080p price: `unavailable`.
- Failure/rejection billing: `unavailable`.

## Gate

No paid Seedance task may be created from this package. A technically constructible request is not a preflight PASS. The missing evidence is a single fixed-scene continuous route video that proves the approach, doorway crossing and Terrace endpoint.
