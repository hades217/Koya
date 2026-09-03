# Koya Estate Studio workspace

Private monorepo for the Estate Studio desktop product and Koya Residences production project.

## Repository map

| Directory | Owns | Must not own |
| --- | --- | --- |
| `apps/` | Runnable product code and app-local tests | Customer source files or provider task archives |
| `docs/` | Cross-product requirements, architecture and operating policy | Generated output |
| `projects/koya/` | Koya evidence, production state, assets and delivery records | Reusable application code |
| `scripts/` | Repository-wide validation and maintenance commands | Product business logic |

The two applications are deliberately siblings:

- `apps/real-estate-ai-studio/` — Tauri desktop application.
- `apps/interactive-apartment-tour/` — browser/static tour embedded by the desktop app and deployed separately.

The Koya project workspace is divided into:

- `source/` — original renders and marketing plans;
- `research/` — verified project and neighbourhood research;
- `production-assets/` — classified official, generated and rejected assets;
- `storyboards/` and `video-production/` — governed production records and QA;
- `operations/` — pipeline state, briefs and handoff notes;
- `inbox/` — unclassified provider downloads awaiting reconciliation;
- `workbench/` — legacy or temporary derived material, never a source of truth.

Read [the repository architecture](docs/REPOSITORY_ARCHITECTURE.md) before adding a new top-level directory. Run `npm run check:repo` after structural changes.

For property-video or Seedance work, also read `AGENTS.md`, `projects/koya/AGENTS.md` and `projects/koya/video-production/SEEDANCE_2_5_RULES.md`.

Large binary assets use Git LFS. Dependencies, framework caches and rebuildable experiments are excluded from version control.
