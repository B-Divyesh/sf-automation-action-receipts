# Polish 5 handoff

## Delivered

- Repaired every remaining cumulative review finding from reviews 1–5.
- Added the testable `mit-license` claim and mapped both the README and Terms
  statements to it.
- Mapped the landing/demo offline UI to its claims and extended the offline
  regression to keep the status bar visible for both valid and tampered
  receipts while networking is disabled.
- Corrected the initial demo heading sequence from h1 → h3 to h1 → h2 and
  added a direct initial-markup test.
- Updated the catalog description: “Record automated changes as signed local
  receipts, then verify them without a server.”

Source repair commit: `e95a3e107ca943accf3ef051010b76e9cf21d71a` (pushed to
`origin/main`). Static deployment:
`8c12dfda-10c3-407b-9c89-518fdd6e2c55` at
https://automation-action-receipts.sociobot.in.

## Run and verify

```sh
npm ci --include=dev
npm run check
while IFS=$'\t' read -r id command; do bash -lc "$command"; done < \
  <(jq -r '.[] | [.id, .test] | @tsv' .factory/claims.json)
cargo package --allow-dirty
```

`npm run build` produces `dist/site/`; the factory deploys that directory as
the static site. The CLI package is ready for the factory registry workflow;
do not publish it from this checkout.

## Exact evidence

- Independent clean clone:
  `/tmp/action-receipts-polish5-clean-eGlyHR/repo` at `e95a3e1`.
- `npm run check`: PASS — formatting, Clippy with warnings denied,
  TypeScript, 5 Rust unit tests, 9 CLI integration tests, 3 Vitest tests,
  production build, and 31 Playwright passes (one expected desktop skip).
  Log: `/tmp/action-receipts-polish5-clean-eGlyHR/full-check.log`.
- Every one of the 20 exact claim commands passed independently. Per-claim
  logs are `/tmp/action-receipts-polish5-clean-eGlyHR/claim-<id>.log`; summary:
  `/tmp/action-receipts-polish5-clean-eGlyHR/claims-summary.txt`.
- `cargo package --allow-dirty`: PASS — 62 files, 552.3 KiB (267.3 KiB
  compressed); package verification compiled successfully.
- Cold live verification: `/opt/fleet/lib/verify-url.sh` passed with no
  console errors. Public screenshots and audit are in
  `/work/.evidence/automation-action-receipts-polish5-live-xjYPCg/`.
- Live Playwright Axe integration found zero violations on Home, Demo, Privacy,
  Terms, and 404 in both themes. The standalone Axe CLI could not start its
  Selenium Chrome driver in this container, so the installed Playwright Axe
  integration was used instead.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices
  100, SEO 100; LCP 1.21 s, CLS 0, TBT 0. JSON:
  `/work/.evidence/automation-action-receipts-polish5-live-xjYPCg/lighthouse-mobile.json`.

## Known gaps

None. The full cumulative finding map is in `.factory/polish-5.md`.
