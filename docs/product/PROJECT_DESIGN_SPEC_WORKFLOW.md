# Project Design Specification workflow

Status: implemented in Creative Studio

## Product decision

Every property project owns an independent visual system. Estate Studio's UI
remains neutral; project documents inherit only the active project's approved
design specification.

## Required order

1. Audit approved project website, brand assets and imagery.
2. Create `creative/design-system/PROJECT_DESIGN_SPEC.md`.
3. Derive `creative/design-system/design-spec.json` for rendering.
4. Approve colour, typography, grid, image treatment and reusable components.
5. Create each sales-document `SOT.md`.
6. Derive `document.md`, then `document.html`, then PDF.

The design specification controls presentation only. Project facts remain
governed by each document's fingerprinted SOT. Missing facts stay
`unavailable`.

## Koya reference direction

- Warm ivory editorial field: `#F3EFE5`
- Near-black architectural ink: `#241F1A`
- Restrained terracotta accent: `#A96D45`
- Muted stone: `#817D76`
- Botanical secondary accent: `#5F7567`
- Display typography: Didot/Bodoni/Georgia serif fallback stack
- Body typography: Avenir/Helvetica/Arial sans-serif fallback stack
- Layout: asymmetric editorial grid, large display titles, thin rules,
  compact uppercase metadata and generous negative space
- Imagery: approved project imagery at natural colour; neutral scrim only for
  text contrast

## Gate

Sales materials may be drafted while the design specification is incomplete,
but client-ready PDF export must show the design state. An approved project
design specification is the target production gate.
