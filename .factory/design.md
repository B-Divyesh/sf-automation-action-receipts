# Action Receipts visual thesis

## Direction: chain-of-custody neo-brutalism

Action Receipts should feel like an evidence envelope crossed with a dependable
terminal: direct, inspectable, and difficult to quietly alter. The system uses
hard black rules, offset shadows, square corners, visible sequence numbers, and
paper-like panels. The visual weight signals accountability without pretending
that a signature makes an automation morally correct.

## Palette

- `ink #171713` — primary text and borders, 14.5:1 on paper.
- `paper #F4F0E6` — warm document background; avoids generic SaaS white.
- `sheet #FFFDF7` — raised evidence sheets.
- `signal #FF5C35` — vermilion seal/action color; black text gives 5.1:1.
- `proof #B9F227` — chartreuse integrity/highlight color; black text gives 14:1.
- `blueprint #3B5BDB` — links and technical annotations; white gives 6.1:1.
- `quiet #5C5A51` — secondary text, 6.3:1 on paper.
- `danger #B42318`, `success #166534`, `warning #7A4D00` — always paired
  with an icon or word, never color alone.
- Dark treatment uses `#11110F` canvas, `#1C1C18` sheets, `#F7F3E8` text,
  `#A8D929` proof, and `#FF7657` signal. The UI follows the device preference
  and also exposes a labeled theme toggle.

## Typography

- Display/labels: **Arial Black**, then `Arial`, system sans. Its compressed,
  blunt forms make statuses read like stamped evidence without a font request.
- Body: `ui-monospace, SFMono-Regular, Consolas, Liberation Mono, monospace`.
  It preserves exact commands and hashes and remains self-hosted by definition.
- Scale: 14 / 16 / 20 / 28 / clamp(40–72) px, with body at 16px minimum.
  Technical values use tabular numerals. Text measure tops out at 72 characters.

## Spacing and layout

The base unit is 4px. Primary rhythm: 8, 12, 16, 24, 32, 48, and 80px.
Desktop uses an asymmetric 7/5 hero grid; the illustration acts as the second
column, not a background. Mobile stacks copy, primary action, proof card, then
the illustration; secondary navigation collapses without hiding core actions.
Borders are 2px (3px for primary shells), shadows offset exactly 6px with no
blur, and controls are at least 44px tall.

## Interaction grammar

Buttons move two pixels into their offset shadow when pressed. Opening a
receipt reveals events in sequence; verification changes one bounded proof
panel from “unchecked” to a word-and-icon result. Drag/drop is additive to the
keyboard-accessible file input. No action is conveyed by icon or color alone.
Destructive retention commands are CLI-only and require explicit flags.

## Motion policy

Only state changes move: evidence rows enter 8px from their prior chain link in
180ms; buttons press in 90ms; the verification stamp settles in 220ms. There
are no ambient loops or parallax. Under `prefers-reduced-motion: reduce`, all
translation and smooth scrolling are removed and results appear instantly.

## Original asset plan and provenance

`site/public/receipt-chain.webp` is a generated editorial illustration of
three physical evidence cards linked by a cryptographic chain, used to explain
the authorization → invocation → artifact sequence. It has no text, brand, or
UI screenshot, so essential information remains in HTML. Generated on
2026-08-27 with the factory Azure image deployment (`factory-image`) via
`/opt/fleet/lib/gen-image.sh`; generator metadata is retained under
`artwork/source/` and the optimized delivery asset is 80 KB WebP. Prompt:

> Use case: stylized-concept. Asset type: landing page explanatory hero.
> Primary request: an editorial neo-brutalist still life explaining a signed
> automation receipt. Scene: three thick paper evidence cards connected by a
> visible black chain, one authorization stamp, one terminal-like command strip,
> and one sealed artifact envelope; no readable words. Style: tactile cut-paper
> print, hard black outlines, slight ink misregistration, crisp studio scan.
> Composition: landscape 3:2, cards step upward left to right, isolated on warm
> cream paper with breathing room. Palette: black ink, cream paper, vermilion,
> acid chartreuse, small cobalt details. Constraints: no gradients, no realistic
> people, no logos, no watermark, no legible text, no glossy 3D, high contrast.

License: original project asset generated for this product; distributed under
the repository MIT license.
