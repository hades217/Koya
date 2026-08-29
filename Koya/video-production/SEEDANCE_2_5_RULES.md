# Koya Seedance 2.5 production rules

Last verified: 2026-08-26, Australia/Brisbane.

This is the canonical Seedance rulebook for Koya. It applies to Codex, Claude Code, and any agent working inside this project.

## 1. Evidence and source of truth

1. Open the relevant official floor-plan PDF and route overlay before making or changing a shot.
2. Record the exact apartment, start point, endpoint, travel direction, camera turn, doorway crossing, visible rooms and forbidden geometry.
3. Official project renders control the approved architectural language and finishes. AI concepts must be labelled concepts and cannot override the floor plan.
4. If a fact is missing, write `unavailable` or `未公开`; never infer it from an AI image.
5. Never invent extra rooms, openings, doors, islands, corridors, floors, views, fixtures, ceiling height, room area or building massing.

For Apartment 106, use the local official floor plan and the current approved route audit. Re-open them each run; filenames alone are not evidence that an old interpretation remains correct.

## 2. Separate the production stages

The required order is:

1. **Research** — floor plan, official renders and existing accepted video endpoints.
2. **Topology map** — route drawn on the untouched floor plan.
3. **Shot contract** — exact start, end, direction, duration, lens, camera height and forbidden events.
4. **Continuity control** — normally a continuous local blockout/reference video from one fixed scene. When the project owner explicitly chooses image-only experimental continuity, use the approved floor-plan-audited image set, preserve its manifest order, and record the waiver and non-guarantee.
5. **Checkpoint QA** — dense contact sheet from that same preview.
6. **Seedance manifest** — endpoint/model, mode, every input and role, prompt, duration, resolution, ratio, audio, output count and price evidence.
7. **User approval** — approval is for the displayed manifest only.
8. **One paid submission** — never fan out variants automatically.
9. **Poll the same task** — queued/running is not failure.
10. **Download and preserve** — save the provider result, task record and returned metadata locally.
11. **Frame-level QA** — inspect topology, continuity, realism and exact endpoint.
12. **Accept, reject or plan a scoped repair** — never call an output accepted merely because the provider returned `succeeded`.

Do not skip directly from unaudited still images to paid video generation. A project-owner-authorized image-only experiment is allowed only after the complete still set, route, prompt, roles, price status and package fingerprint have been reviewed.

## 3. Seedance input modes and when to use them

### Text-to-video

Use only where exact architectural topology and identity are not important. It is not suitable for a floor-plan-faithful apartment route.

### First-frame image-to-video

Use for a simple move where the visible scene remains substantially the same. A first frame cannot define unseen rooms around a corner.

### First-frame plus last-frame

Use only when both images are from the same spatial model and the required change is small and unambiguous. It is unsafe for a large turn, multiple rooms or independently generated endpoints.

Current server validation observed in this project:

- `first_frame` and `last_frame` form a two-anchor mode;
- first/last-frame roles cannot be assumed combinable with arbitrary reference-image roles;
- explicit output ratio may be inherited/restricted in first/last-frame mode.

Re-verify current server behaviour before every new workflow; do not generalise an old response forever.

### Multi-reference images

Use `reference_image` for appearance, subject, material, style and identity guidance.

Hard fact: a list of reference images is **not** a provider-guaranteed ordered compulsory timeline. Naming files `B0`, `B1`, `B2` or mentioning “image 1 then image 2” in a prompt does not guarantee that the camera physically visits each pose. More independently generated images can add contradictory doors, walls and perspective.

The project owner may nevertheless authorize one experimental image-only continuity task. In that mode:

- do not attach a `reference_video`;
- use only explicit provider-compatible image roles. If live server validation rejects mixing `first_frame` or `last_frame` with reference media, submit the complete reviewed set as explicit `reference_image` inputs; describe opening and endpoint compositions in the prompt without claiming those images are enforced keyframes;
- preserve the reviewed image order in the manifest and request payload;
- audit every image against the same official floor plan and persistent-landmark ledger;
- state clearly that continuity is required by the prompt but not guaranteed by the provider;
- generate one output only, with no automatic retry;
- reject the result if frame-level QA finds a cut, morph, teleport or topology drift.

For topology-sensitive architectural **video editing**, the default is **zero whole-room `reference_image` inputs**. A prompt cannot make the API's generic `reference_image` role ignore composition, perspective or architecture. The owner-authorized image-only experimental mode is `multimodal_to_video`, not video editing, and is reviewed under the exception above.

- Never submit a whole-room style image unless it is a render or frame from the exact same approved 3D shell, camera path and fixture layout as the reference video.
- Never combine a simplified reference video with independently generated kitchen, threshold or bathroom views. Calling those images “material-only” in the prompt does not remove their structural influence.
- If material guidance is needed, prefer geometry-free close-up swatches or apply the materials inside the deterministic scene before rendering the reference video.
- Before every task, create a cross-input topology table for every visible wall, opening, door, island and fixture. Any disagreement blocks submission.
- The first V2V attempt for a literal property route must use the approved reference video alone unless every additional visual input passes the same-scene test.

### Reference video / video editing

Use this by default for architecture, camera choreography and literal short one-takes. It is recommended rather than mandatory when the project owner explicitly selects the image-only experimental mode.

The reference video must already contain:

- one camera;
- one continuous camera path;
- one fixed apartment shell;
- correct walls, doors, island, cabinetry and room scale;
- physical parallax and occlusion;
- the exact turn and threshold crossing;
- no cuts, dissolves, teleports or image crossfades.

Seedance may then improve materials, daylight, texture, human micro-motion and photographic realism while being instructed to preserve the reference video's geometry and motion. This instruction is guidance, not a technical geometry lock; the local reference must already be sufficiently close to the desired final architecture and appearance.

### Video extension

Use only when the accepted previous clip's final frame, direction and momentum are suitable. Extension is not permission to change topology. Validate the join densely before merging.

## 4. Storyboards for continuous architecture

More storyboard detail is required, but it must come from one spatial source.

- Render checkpoints from the same deterministic scene at 0.2–0.25 second intervals.
- Do not independently AI-generate every checkpoint and ask Seedance to repair their disagreements.
- Each checkpoint records camera position, heading, visible fixed landmarks, occlusion and forbidden objects.
- During a turn or doorway crossing, increase density to 8–12 checkpoints per second if needed.
- Keep a stable ID for every wall, doorway and major fixture across all checkpoints.
- A contact sheet is normally QA evidence, not a guaranteed input timeline. In the explicitly owner-authorized image-only experiment, its individual frames may be submitted in manifest order as best-effort references without claiming ordered-keyframe semantics.

For Clip B, follow `clip-b-hub-to-bath/SEEDANCE_2_5_V7_ENGINEERING_PLAN.md` and its 26-checkpoint route.

## 5. Prompt contract

Write prompts in this order:

1. **Source-of-truth instruction** — identify the reference video or the explicitly approved floor-plan-audited image set and route that must be preserved.
2. **Single camera path** — start, travel direction, turn direction, physical threshold and end.
3. **Timing map** — explicit time intervals whose total equals the requested duration.
4. **Immutable geometry** — walls, door identity, island, cabinets, fixtures and room scale.
5. **Allowed changes** — materials, natural light, photographic texture, subtle stabilised walking motion and sound.
6. **Forbidden events** — cuts, dissolves, morphs, teleporting, wall crossing, extra doors, duplicated rooms, widening, fisheye, day/night change and game-like CGI.
7. **Endpoint contract** — describe the exact final pose needed for the next clip.

Negative wording alone cannot repair inconsistent inputs. Fix source geometry before rewriting prompts.

Prompt text must never claim an unsupported control distinction such as “read this whole-room image for materials only.” If the API exposes only `reference_image`, assume the model may use every visible property of that image.

## 6. ArkCLI execution rules

Use the installed `arkcli-gen` workflow. Every ArkCLI command run by an agent must use:

```bash
ARKCLI_NO_UPDATE_NOTIFIER=1 \
ARKCLI_CALLER_TYPE=ai_agent \
ARKCLI_CALLER_NAME=<agent-id> \
ARKCLI_SKILL_NAME=arkcli-gen \
arkcli <command>
```

Required steps:

1. `arkcli auth status`
2. For an explicit endpoint: `arkcli resources resolve <endpoint> --format json`
3. Confirm live model, region, input/output modalities, task types and endpoint status.
4. If using a model ID rather than an endpoint, query its current `supported_params` before setting parameters.
5. Build and inspect `arkcli +gen ... --dry-run --format json`; a partial client preview is not server validation.
6. For `reference_video`, verify whether the current endpoint requires a Web URL. The Koya Seedance 2.5 endpoint rejected a local path on 2026-08-26 with `reference_video must be provided as a web url`; do not pass a local video path to a paid create call unless live tooling proves automatic upload support.
7. When using an HTTPS delivery URL, verify a complete remote GET has the same byte length and SHA-256 as the locked local input. Record and fingerprint the exact URL.
8. Present the complete submission manifest and obtain explicit user approval.
9. Submit exactly once in a controlled terminal/session.
10. Record the returned task ID immediately.
11. Poll that task with `arkcli gen get <task-id>` until a terminal status. Never call `+gen` again just because the task is queued or slow.
12. Download to the intended versioned output directory and preserve the task response.

If a create command returns blank output or the caller's initial wait expires, inspect the local process table before retrying. A still-running `arkcli +gen` process is an active create attempt even when no task ID is visible. Also query `arkcli gen list`; allow no second create process until the first process has ended and the provider shows no new task and no usage.

Use explicit input roles:

- `first_frame:@file` for a true first-frame anchor;
- `last_frame:@file` for a true last-frame anchor;
- `reference_image:@file` for appearance/reference only;
- `reference_video:@file` for motion/topology reference;
- `reference_audio:@file` for audio reference.

Do not rely on the first input's implicit role when a paid task is involved.

The currently known Koya endpoint is `ep-20260812221158-hb576`, previously resolved to `doubao-seedance-2-5-260628`, but both are drift-prone and MUST be resolved live before each submission.

## 7. Cost and approval gate

Any generation, editing or extension request can create cost.

Before submission, show:

- exact task count and output count;
- endpoint and resolved model;
- task mode;
- duration, resolution, aspect ratio and audio setting;
- every input file with its explicit role;
- final prompt;
- output directory;
- current provider price source and estimated total, or `unavailable` when it cannot be confirmed;
- whether failure, cancellation or provider rejection is billable, if officially documented; otherwise `unavailable`.

Rules:

- Never invent or reuse a stale price.
- Never interpret “confirm” as permission for multiple variants.
- One approval equals one submitted task and one generated output.
- Do not enable two candidates, rerolls, automatic retries or parallel submissions without separate explicit approval.
- A rejected visual result does not automatically authorise a replacement task.
- Server parameter rejection may be retried only after proving that no generation task was created and no charge occurred.

## 8. QA requirements

Inspect the downloaded final MP4, not only a browser preview or provider status.

Minimum checks:

- `ffprobe` duration, dimensions, frame rate, codec and audio;
- contact sheet across the entire clip;
- 8 fps inspection for normal travel;
- 12 fps or denser inspection around turns, doors, walls, people and clip joins;
- compare start/end frames with their approved contracts;
- compare route and visible topology with the floor plan;
- verify stable daylight, exposure, lens and scale;
- verify real parallax and physical occlusion;
- verify no cut, dissolve, morph, teleport, duplicated doorway, wall penetration, geometry drift or game-like camera motion;
- verify the clip communicates the intended property-viewing objective.

If any mandatory check fails, mark the output rejected and state exact timestamps. Do not quietly stitch it into the master.

## 9. Repair strategy

1. Diagnose exact bad intervals before proposing regeneration.
2. Preserve accepted intervals; do not regenerate the whole video by default.
3. Prefer deterministic local editing for timing, trim, audio, colour or a clean existing seam.
4. Do not use a crossfade to hide a topology error in a claimed one-take.
5. For geometry failure, repair the deterministic reference motion/source scene first, then request approval for one scoped Seedance edit.
6. Use the exact accepted boundary frame and matching camera momentum when a modular join is allowed.
7. Keep rejected outputs in a clearly labelled `rejected/` directory unless the user explicitly authorises deletion.

## 10. Completion states

Use precise status language:

- `planned` — documents only;
- `preview_ready` — deterministic local preview exists;
- `approved_for_one_submission` — exact manifest approved;
- `queued` / `running` — provider task exists, not finished;
- `succeeded_unreviewed` — provider returned a video but visual QA is incomplete;
- `rejected` — reviewed and failed specified checks;
- `accepted_local` — user/QA accepted local output;
- `merged_local` — integrated into a local master;
- `published` — separately verified public delivery.

Never collapse these states into “done”.

## 11. Official references

- Ark video generation API: https://api.volcengine.com/api-docs/view?action=CreateContentsGenerationsTasks&serviceCode=ark&version=2024-01-01
- Ark video-generation documentation: https://www.volcengine.com/docs/82379/1520758?lang=zh
- Jimeng first/last-frame API: https://www.volcengine.com/docs/85621/1791184?lang=zh

Re-check official documentation when the model, endpoint, ArkCLI version or provider behaviour changes. Third-party claims such as maximum reference count are not authoritative unless reproduced against the current endpoint without creating a paid task.
