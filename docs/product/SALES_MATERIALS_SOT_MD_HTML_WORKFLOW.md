# Sales Materials: SOT → Markdown → HTML

Status: implementation contract

## Outcome

Estate Studio treats sales materials as evidence-controlled documents, not isolated design files. Each saved task produces one auditable package:

```text
creative/sales-sheets/<task-id>/
├── SOT.md
├── document.md
├── document.html
└── brief.json
```

- `SOT.md` is the factual evidence snapshot for the exact task.
- `document.md` is the human-editable sales document and records the SOT SHA-256.
- `document.html` is regenerated from `document.md` and records both upstream hashes.
- `brief.json` records the AI request, task type, warnings and workflow state.

HTML is a preview and print surface. It is never an authoring source and must not be edited independently.

## The seven task types

| Task ID | Sales material | Required truth before approval | Missing-data rule |
| --- | --- | --- | --- |
| SALES-01 | Project sales brochure | Project identity, verified highlights, location, approved imagery and team facts | Unsupported claims remain unavailable |
| SALES-02 | Floorplan book | Approved plan revisions, unit labels, areas and plan notes | Missing plan or area blocks the affected entry |
| SALES-03 | Unit sales sheet | Selected unit, approved floorplan and verified unit facts | Do not substitute a similar unit |
| SALES-04 | Price and availability | Authorised price and live availability schedule | Never estimate price or infer availability |
| SALES-05 | Finishes and specifications | Approved finishes, appliances, fixtures, options and substitution wording | Renders do not prove inclusions |
| SALES-06 | Agent kit | Project SOT, approved contacts, sales process, FAQs and talking points | No invented returns, scarcity or guarantees |
| SALES-07 | Showroom and EOI pack | Approved appointment, EOI, deposit, privacy and legal wording | Legal/commercial terms require authorised input |

## AI workflow

1. Read the current project manifest and only accepted evidence selected for the task.
2. Resolve the task type and selected unit.
3. Write `SOT.md` with project facts, unit facts, evidence IDs, SHA-256 checksums and known gaps.
4. Fingerprint `SOT.md`.
5. Draft `document.md` from that exact SOT and record its fingerprint.
6. Render `document.html` from the saved Markdown.
7. Show the HTML preview for human review.
8. Regenerate downstream files whenever the SOT or Markdown changes.

## Stop rules

- Missing data is `unavailable`, never zero, an estimate or plausible marketing copy.
- Official plans and schedules outrank generated imagery and earlier copy.
- A render cannot prove dimensions, inclusions, views, availability or delivery terms.
- Project-level facts cannot automatically become unit-specific facts.
- A similar unit cannot stand in for the selected unit.
- HTML must never diverge from Markdown.
- Preview-ready is not client-approved, printed, distributed or published.

## Acceptance

A task is preview-ready only when:

- all three lineage files exist;
- the Markdown SOT hash matches `SOT.md`;
- the HTML SOT and Markdown hashes match both upstream files;
- unsupported facts are absent or explicitly marked unavailable;
- the document type and selected unit are correct;
- the HTML renders without broken structure at its intended print size.
