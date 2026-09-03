---
name: property-poster-studio
description: Create evidence-safe A4 property sales sheets or separately reviewed Image Model poster requests.
---

# Property poster studio

Route every request into one explicit mode. Do not blend the modes.

## Sales materials documents

Support exactly seven sales-material tasks:

1. Project sales brochure
2. Floorplan book
3. Unit sales sheet
4. Price and availability
5. Finishes and specifications
6. Agent kit
7. Showroom and EOI pack

Every task uses the same lineage: `SOT.md` → `document.md` → `document.html`. First write an evidence snapshot containing the project facts, selected unit facts, accepted evidence IDs and checksums, and an explicit gap register. Then draft the customer-facing Markdown using only that SOT. Finally render HTML from the saved Markdown; never maintain parallel HTML copy and never edit HTML as an authoring source.

Record the SOT SHA-256 in the Markdown front matter and both the SOT and Markdown SHA-256 values in the derived HTML metadata. Missing prices, availability, finishes, contacts, sales steps, EOI terms, privacy wording or legal statements remain `unavailable` until an authorised source is supplied. Marketing imagery and prior copy cannot silently fill those gaps.

Prioritise readable hierarchy, unit facts, body copy and call to action. These documents are deterministic outputs, not Image Model outputs.

Users may provide the brief through natural-language Codex chat or the traditional structured form. Both routes must produce the same editable review draft and use the same evidence gates and lineage.

## AI creative poster

Start by analysing the current project, selected unit, accepted evidence and brand. Extract three to five short, evidence-grounded selling points that can become poster highlights. If the project does not contain enough verified facts, return fewer highlights and record the limitation instead of inventing claims.

Then write the headline, supporting copy and call to action, and prepare an ads-marketing Image Model prompt for a high-impact raster main visual. Keep headline, logo, facts, highlights and call to action outside the generated image so Estate Studio can overlay reliable typography. Ask for intentional negative space and prohibit invented views, finishes, amenities, people or building facts.

Users may describe the desired poster entirely in natural language, including what to emphasise and the intended visual tone. The traditional form remains an equivalent manual input route. Both routes must return the same editable highlights, copy and Image Model prompt.

Saving this mode creates an auditable request package only. Keep provider, dimensions, evidence IDs, price status and approval state explicit. Never label a request as generated before a real image result is saved and reviewed.

- Keep the headline, supporting copy, call to action, format, audience, and evidence list explicit.
- Do not add claims about lifestyle, views, scarcity, pricing, completion dates, or amenities without evidence.
- Require human review before any Image Model submission; one approval covers one requested output only.
