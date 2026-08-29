# Koya project agent rules

These rules apply to every task under `/Users/lightman/Documents/sites/Koya`.

## Seedance and property-video work

Before planning, generating, extending, editing, polling, downloading, stitching, or approving any Seedance asset, read and follow:

- `Koya/video-production/SEEDANCE_2_5_RULES.md`

The Seedance rules are mandatory and override generic video-production defaults whenever the work concerns Koya property imagery, Apartment 106, spatial walkthroughs, storyboards, reference images, reference videos, or paid generation.

Before every Seedance create, edit, extend or retry operation, invoke `/Users/lightman/.codex/skills/seedance-preflight-review/SKILL.md`. A current `PASS` report and package fingerprint are mandatory, but do not replace the user's explicit approval. Any prompt, asset, role, parameter, model, price or task-count change invalidates the PASS. The review skill must never submit the task itself.

Non-negotiable summary:

- Floor plans and official project assets are the topology source of truth. Do not invent rooms, doors, dimensions, floors, views, finishes, or building facts.
- Never claim that multiple `reference_image` inputs are a provider-guaranteed ordered keyframe timeline. The project owner may explicitly choose a single-task experimental multi-image continuity mode without `reference_video`; in that mode, preserve the requested image order in the manifest and prompt, disclose that continuity is best-effort, and apply stricter post-generation frame QA.
- For a physically continuous camera move through architecture, the default and recommended control is a deterministic single-scene reference video and Seedance reference-video/video-editing mode. A `reference_video` is not mandatory when the project owner explicitly waives it for the exact reviewed task and accepts the continuity risk.
- For topology-sensitive V2V, do not attach independently generated whole-room `reference_image` files, even when the prompt calls them style-only. This restriction does not prohibit an explicitly owner-authorized image-only experimental task; that task must be labelled `multimodal_to_video`, not V2V, and every image must be audited against the floor plan before submission.
- A simplified white-box reference is not sufficient for paid photorealistic restyling when doors, rooms or fixtures are absent. Complete the fixed shell locally first.
- Never submit a paid task before the user has approved the exact preview, manifest, prompt, inputs, parameters, task count, and current provider price or an explicit `unavailable` price status.
- A queued video task is not a failure. Poll the same task ID; never resubmit merely because the result is not ready.
- One approval authorises one task and one output only. Multiple candidates require separate explicit approval.
- Inspect the final MP4 frame-by-frame. A successful provider status is not visual acceptance.
