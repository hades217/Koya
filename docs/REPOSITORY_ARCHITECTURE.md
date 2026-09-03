# Repository architecture

## Purpose

This repository contains a reusable product and one customer project. Those are different lifecycles and must remain different filesystem domains.

```text
apps/                         runnable products
  interactive-apartment-tour/
  real-estate-ai-studio/
docs/                         product-wide decisions and specifications
projects/
  koya/                       Koya-only evidence and production state
    source/                   immutable original inputs
    research/                 evidence-backed written research
    production-assets/        classified asset registry
    storyboards/              shot intent and review material
    video-production/         manifests, provider records and frame QA
    operations/               current briefs, status and handoffs
    inbox/                    unreconciled external downloads
    workbench/                derived legacy/scratch material
scripts/                      repository boundary checks
```

## Dependency direction

`docs -> no runtime dependency`

`projects/koya -> may reference app export formats, never app internals`

`apps/real-estate-ai-studio -> apps/interactive-apartment-tour public build`

`apps/* -> may read a selected project through explicit adapters; must not hard-code arbitrary project filesystem paths`

The desktop app currently embeds the tour build as an explicit transitional dependency. New Koya-specific assets should enter through a project bundle or fixture, not by adding more imports from `projects/koya/` to reusable product code.

## Artifact policy

- `source/` is immutable evidence. A transformed copy belongs in `production-assets/` with provenance.
- Provider task manifests, responses and QA stay together under `video-production/`.
- `inbox/` is not accepted evidence. Files leave it only after identity and provenance are recorded.
- `workbench/` is not authoritative and must not be referenced by a release manifest.
- Build caches and dependency directories are ignored. They are never committed as delivery evidence.
- A local build, exported bundle, deployment and public verification remain separate states.

## Adding code or data

1. Reusable runtime behavior goes in the owning app.
2. Cross-app product decisions go in `docs/product/`.
3. Koya-only facts and media go in `projects/koya/`.
4. A new top-level directory requires an architecture change and an update to the repository checker.
5. Run `npm run check:repo`. App changes must also run their app-specific verification.

## Migration note

The previous repository nested everything below `Koya/` and left downloads, PDF renders and a motion project at the repository root. The 2026-09-03 migration changed paths only: it did not approve, publish, regenerate or delete any property asset.
