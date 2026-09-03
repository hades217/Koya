# Koya Example Project Rights Audit

Status: `internal_only`

Reviewed: 2026-09-02 AEST

## Included

- A read-only manifest containing unit labels and facts already recorded from the local official floor-plan evidence register.
- A link to the separately published Koya interactive-tour reference.
- The existing QA-passed living-to-terrace structural-reference MP4, labelled `concept_style_only` and restricted to this internal read-only example.
- Generic Estate Studio UI metadata and one non-generated authoring example.

## Not bundled

- Customer PDFs, drawings, renders, photographs, panoramas, official-marketing videos, logos, fonts, contracts, or rejected generations.
- API keys, deployment credentials, provider tokens, cookies, or other secrets.
- Any claim that generated concepts are official, surveyed, as-built, or contract documents.

## Distribution decision

Commercial redistribution approval for a named Koya sample is `unavailable`. The current manifest is therefore an internal read-only development example. It is blocked from portable export. A commercial installer must either obtain written approval for this exact sample or replace it with a separately licensed/anonymised example before release.

## Runtime invariant

Rust/Tauri and browser fallback must load `project.json` from this directory. They may not maintain independent Koya facts in source code.
