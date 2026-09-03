# Estate Studio Design System

Status: review draft v0.1

This system separates the product interface from each property brand. The app
shell must remain calm, neutral and operational. Koya's identity belongs in
project titles, approved imagery and campaign outputs, not across every UI
surface.

## 1. Design principles

1. Quiet confidence: hierarchy comes from scale, spacing and typography, not decoration.
2. Project first: property imagery is the visual focus; product chrome steps back.
3. Two-layer identity: the neutral product shell and the active project brand never share token roles.
4. Evidence over atmosphere: imagery appears only where it communicates a project, unit or asset.
5. Controlled colour: one interface accent, four semantic states, no ornamental gradients.
6. Editorial, not theatrical: restrained typography and long rules replace oversized cards and luxury clichés.

## 2. Brand architecture

### Product shell

Used for navigation, workspace surfaces, forms, tables, AI chat, jobs and deployment.

- Neutral mineral palette.
- Sweet Sans-style UI typography.
- No project photography in navigation or chat.
- No gold, bronze, brown wash or project accent as the primary button colour.

### Project identity

Used for project hero, project name, unit identity and approved campaign assets.

- Project-owned imagery may appear in hero and asset previews.
- Project display typography may be used for the project name only.
- Koya colours never recolour global navigation, forms or semantic status states.

## 3. Colour system

### Product neutrals

| Token | Value | Role |
| --- | --- | --- |
| `--ui-canvas` | `#F4F4F1` | App background |
| `--ui-surface` | `#FCFCFA` | Primary cards and panels |
| `--ui-surface-muted` | `#EEF1F0` | Secondary groups and selected rows |
| `--ui-ink` | `#20282C` | Primary text |
| `--ui-ink-muted` | `#6C767A` | Supporting text |
| `--ui-line` | `#D9DEDC` | Borders and rules |
| `--ui-sidebar` | `#24343B` | Navigation shell |
| `--ui-sidebar-active` | `#354D56` | Active navigation item |
| `--ui-accent` | `#607D88` | Focus, progress and restrained emphasis |

### Koya project profile

These tokens belong to project identity and campaign output only.

| Token | Value | Role |
| --- | --- | --- |
| `--brand-koya-ink` | `#38261D` | Official warm brand ink |
| `--brand-koya-black` | `#1F1A16` | Official near-black |
| `--brand-koya-ivory` | `#F3F0E8` | Editorial project field |
| `--brand-koya-stone` | `#CBC4B9` | Plans and secondary rules |
| `--brand-koya-botanical` | `#66746B` | Optional image-adjacent accent |

### Semantic states

Semantic colours must never inherit a project's brand colour.

| State | Ink | Background |
| --- | --- | --- |
| Success | `#356248` | `#E2ECE5` |
| Warning | `#8A5B22` | `#F3EBDD` |
| Danger | `#93483F` | `#F3E4E1` |
| Information | `#486B7B` | `#E3ECEF` |

## 4. Typography

### Families

- UI sans: `Sweet Sans Pro`, falling back to `Avenir Next`, `Avenir`, `Helvetica Neue`.
- Project display: `Pilot`, falling back to `Didot Italic` or `Bodoni 72 Italic`.
- Monospace: `SFMono-Regular`, for paths, identifiers and technical evidence only.

The commercial Koya fonts are not extracted from embedded PDFs. Production use
requires licensed font files. Fallbacks preserve the hierarchy during development.

### Roles

| Role | Size / line | Weight | Treatment |
| --- | --- | --- | --- |
| Display | 48 / 52 | 300 | UI sans; page title only |
| Project title | 38 / 42 | 400 | Display italic; project identity only |
| Section title | 28 / 34 | 300 | UI sans |
| Card title | 18 / 24 | 400 | UI sans |
| Body | 14 / 21 | 400 | UI sans |
| Small | 12 / 18 | 400 | UI sans |
| Eyebrow | 10 / 14 | 500 | Uppercase, `0.18em` tracking |
| Metric | 30 / 32 | 300 | Tabular numerals where available |

Rules:

- Do not apply the display italic to navigation, buttons, forms, chat or metrics.
- Do not use serif typography to make ordinary UI look luxurious.
- Use uppercase tracking only for metadata and section labels, never paragraphs.
- Avoid weights above 600 except destructive or irreversible warnings.

## 5. Layout and spacing

- Base unit: 4px.
- Spacing scale: 4, 8, 12, 16, 24, 32, 48, 64.
- Main content maximum: 1440px.
- Reading measure: 680px.
- Sidebar: 232px.
- AI panel: 400px desktop; overlay below 1280px.
- Page padding: 48px desktop, 32px compact desktop.
- Section separation: 48px; related component gap: 12px or 16px.

## 6. Shape and elevation

- Radius: 2px for tags, 4px for controls, 8px for cards and dialogs.
- Pills are reserved for filters, states and compact metadata.
- Standard border: 1px `--ui-line`.
- Cards are flat by default.
- Hover shadow: `0 12px 28px rgba(27, 39, 44, 0.08)`.
- Modal shadow: `0 28px 72px rgba(20, 28, 32, 0.20)`.
- No gradients on UI controls. Image overlays may use a single neutral scrim.

## 7. Component rules

### Navigation

- Dark neutral shell; no project brand colour.
- Active state uses a quiet tonal fill and a 2px accent rule.
- One icon style and one optical size across all items.

### Buttons

- Primary: dark mineral fill, white label.
- Secondary: white surface, neutral border.
- Tertiary: text only.
- Danger is used only for destructive actions.
- Maximum radius 4px; no glossy gradients or luxury-gold treatments.

### Cards

- Cards group actions or records; they are not page decoration.
- One border, no nested shadow stacks.
- Project cards may contain one approved image with a fixed aspect ratio.
- Status belongs in metadata, not floating decorative badges.

### AI workspace

- Plain `--ui-surface` or `--ui-canvas` background.
- No property image, texture, ambient illustration or campaign art.
- Project context is a compact bordered block.
- Assistant and user messages use shape and tone, not large colour fields.

### Project hero

- One approved project image.
- Neutral dark scrim only when needed for text contrast.
- Project title may use the project display font.
- No decorative stock image or generated background.

### Data and status

- Missing facts read `Unavailable`, never `0` unless zero is verified.
- Status colours are semantic and consistent across projects.
- Progress uses a single accent; circular charts are reserved for one primary metric.

## 8. Imagery

- Use official project assets first.
- Preserve natural material colour; do not tint images to match the UI.
- Hero ratio: 16:7 to 16:9; card ratio: 4:3 or 3:2.
- No image behind chat, forms, tables or long-form text.
- No decorative architecture render when the screen has no project context.
- Generated images must be labelled and must not imply verified project evidence.

## 9. Motion

- Standard duration: 160ms; panel transition: 220ms.
- Ease: `cubic-bezier(.2,.8,.2,1)`.
- Motion communicates state change only.
- No parallax, floating ornaments or continual ambient motion in the product shell.

## 10. Approval gate

Before this system is connected to production UI, approve:

- Product neutral palette.
- Koya project profile palette.
- Typography roles and fallback policy.
- Shape system: 2 / 4 / 8px.
- Image boundaries, especially the plain AI workspace.

After approval, implementation must replace legacy CSS values with tokens instead
of appending another theme override.
