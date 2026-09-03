# Estate Studio agent instructions

These rules apply to every Codex or Claude task in this repository.

## Product stage and source of truth

- The current delivery target is the complete local-first MVP. OpenAI API integration, paid generation, subscriptions, and provider submission are Phase 2 unless the user explicitly changes the phase.
- Before product work, read `../../docs/product/REAL_ESTATE_AI_STUDIO_PRODUCT_VISION_V1.md` and `../../docs/product/OFFPLAN_TOUR_BUILDER_PRD_V1.md`. Treat the current repository and verified native app as implementation evidence; a PRD checkbox alone is not proof of completion.
- Distinguish prepared, generated, approved, exported, uploaded, deployed, and publicly verified states. Never collapse these gates into “done”.

## Evidence and safety

- Never invent property facts, dimensions, rooms, doors, views, finishes, prices, permissions, provider status, or metrics. Use `unavailable` when evidence is missing.
- Floor plans and accepted project files are the topology source of truth. Concept imagery is style evidence only unless explicitly classified otherwise.
- Do not send project assets to an external model or create a paid task without the user's explicit approval of the exact asset set, prompt, parameters, task count, and current price or an explicit `unavailable` price status.
- Keep secrets out of source, logs, prompts, command arguments, fixtures, and screenshots. Do not add hidden network calls or telemetry.

## Implementation discipline

- Preserve unrelated user changes. Do not reset, stash, broad-stage, overwrite, force-push, or delete work that is outside the requested scope.
- Keep browser and Tauri behavior aligned. Schema changes require migration-safe defaults, updated fixtures, and tests.
- Tauri commands are the desktop trust boundary. Validate identifiers and paths, isolate each project, and keep filesystem writes inside the intended project or app-data roots.
- Do not commit, push, deploy, publish, or submit to a provider unless the user explicitly asks for that separate action.

## UI quality

- For UI work, apply the bundled `design-taste-frontend` skill selectively together with the product's established design language. Its landing-page advice is not permission to weaken operational clarity.
- Optimize for a normal 15-inch desktop at 100% scale. Operational body text must be at least 14px, supporting text at least 12px, and form controls at least 14px with a 40px minimum target height.
- Do not ship 7–10px operational labels, clipped dialogs, inaccessible scrolling, low-contrast controls, or dense generic dashboard layouts.
- Settings and other long dialogs must retain visible scrolling, responsive single-column behavior, and reachable actions.

## Required verification

- Run `npm run verify` after implementation changes.
- Run `git diff --check` before handoff.
- For desktop UI changes, build with `npm run tauri:qa:app`, open the exact QA app bundle, and verify the changed flow visually in the native app. A successful build alone is not acceptance.
- Report what was verified and any remaining external or Phase 2 gate. Do not describe unverified behavior as complete.
