# Reusable Off-plan Property Hero Skill Suite

Created: 2026-08-15

The Koya workflow has been converted into eight personal Codex skills under `/Users/lightman/.codex/skills/`.

| Skill | Purpose |
| --- | --- |
| `offplan-property-hero-pipeline` | Master orchestration, phase status, templates, initialization and validation |
| `offplan-property-research` | Official-source research, PDF review and factual project brief |
| `offplan-property-neighbourhood-research` | Daily-life amenities, transport, education, health, parks, planning change and risk checks |
| `offplan-property-asset-bible` | Asset classification, provenance, versions, contact sheets and rules |
| `offplan-property-floorplan-concepts` | Plan geometry extraction and concept interior boards |
| `offplan-property-character-casting` | Locked identities, wardrobes, props and rejected-version handling |
| `offplan-property-one-take-storyboard` | Story, camera route, hidden transitions and keyframe approval |
| `offplan-property-scroll-video` | Clip production, web encoding, scrub implementation and final QA |

## Starting a future project

Ask Codex to use `offplan-property-hero-pipeline`, or initialize the standard structure directly:

```bash
bash "$CODEX_HOME/skills/offplan-property-hero-pipeline/scripts/init_project.sh" /absolute/path/to/new-project
```

Then progress P0 through P6 in `PIPELINE_STATUS.md`. Do not begin video merely because concept images exist.

Validate artifacts through a chosen phase:

```bash
bash "$CODEX_HOME/skills/offplan-property-hero-pipeline/scripts/validate_project.sh" /absolute/path/to/project P4
```

## Koya current state

Koya has completed project research, first-party neighbourhood desktop research, source classification and four plan-driven concept boards. Five character identities exist. Address-level FloodWise, official school catchment and on-site route/noise checks remain due-diligence items. The full cast/wardrobe decision and complete storyboard remain unfinished. No video has been generated, assembled, encoded, integrated or scroll-tested.
