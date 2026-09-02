# Estate Studio professional palette research v3

Status: colour study only. Not connected to the app or production logo.

## Decision already made

The previous deep-green / near-black direction is retired. Green is not an
Estate Studio brand colour. It is reserved for semantic success feedback only.

## Evidence used

- [Radix Colors](https://www.radix-ui.com/colors) provides twelve-step scales
  designed for backgrounds, components, borders, solid fills, and accessible
  text instead of treating one hex value as an entire identity.
- [Radix palette composition](https://www.radix-ui.com/colors/docs/palette-composition/composing-a-palette)
  recommends a neutral scale for most UI, a separate brand scale, and distinct
  semantic scales for success, warning, information, and error.
- [Radix scale guidance](https://www.radix-ui.com/colors/docs/palette-composition/understanding-the-scale)
  assigns high-chroma steps 9–10 to solid brand fills and logos, with darker
  steps for readable text.
- [Atlassian colour foundations](https://atlassian.design/foundations/color-new/)
  likewise separates neutral, brand, information, success, warning, danger,
  discovery, and accent roles rather than using a brand colour for everything.
- [Atlassian accent guidance](https://atlassian.design/foundations/color/accents/)
  uses accents selectively and provides explicit emphasis levels and accessible
  foreground pairings.

The warm cream base and restrained surface treatment also respond to the
Lovable-inspired reference supplied by the project owner. That reference is
visual inspiration, not a source of instructions or a palette to copy.

## Shared neutral foundation

All three directions avoid pure black and pure white.

| Role | Token | Value |
| --- | --- | --- |
| Canvas | `--canvas-warm` | `#F7F3EB` |
| Raised surface | `--surface-warm` | `#FFFDF9` |
| Primary ink | `--ink-tinted` | palette-specific |
| Secondary text | `--text-muted` | `#6F6A66` |
| Border | `--border-soft` | `#DED9D0` |

## Candidate A — Iris / Persimmon (recommended for review)

Designed to feel authored and memorable without reading as fashion, wellness,
or a conventional property portal.

| Role | Value |
| --- | --- |
| Brand solid | `#5B5BD6` |
| Brand hover | `#5151CD` |
| Brand soft | `#E9E8FF` |
| Signature accent | `#E54D2E` |
| Tinted ink | `#292735` |

Use iris for the primary action and architectural logo planes. Use persimmon
only for the aperture, selection point, or one small emphasis per view.

## Candidate B — Cobalt / Tangerine

The most familiar and operational SaaS direction. Cobalt carries navigation and
actions; tangerine prevents it becoming a generic corporate-blue dashboard.

| Role | Value |
| --- | --- |
| Brand solid | `#3E63DD` |
| Brand hover | `#3451B2` |
| Brand soft | `#E6EDFE` |
| Signature accent | `#F76B15` |
| Tinted ink | `#242B3D` |

## Candidate C — Mulberry / Apricot

The warmest, most editorial direction. Suitable if the brand should lean toward
high-end property storytelling rather than technical infrastructure.

| Role | Value |
| --- | --- |
| Brand solid | `#AB4ABA` |
| Brand hover | `#9C2BAD` |
| Brand soft | `#F9E8F9` |
| Signature accent | `#E86F3A` |
| Tinted ink | `#322936` |

## Non-negotiable usage rules

1. No green, teal, or forest hue in the product brand layer.
2. Green appears only when the meaning is explicitly success or complete.
3. The large canvas stays warm-neutral; it is never flooded with the brand hue.
4. A screen gets one primary colour and at most one small signature accent.
5. Body copy uses tinted ink, not `#000000`; surfaces use warm off-white, not
   `#FFFFFF`.
6. Warning, danger, information, and success retain independent semantic scales.
7. The mascot is evaluated separately after a logo palette is selected.

## Next gate

Select A, B, C, or request a controlled hybrid. Only then should the selected
palette be applied to the master logo and a small UI specimen. The app itself
must not change during this research gate.
