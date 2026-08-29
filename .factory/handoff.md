# Polish 4 handoff

## Delivered

- Closed every finding from reviews 1–4. The ID-by-ID receipt is in
  `.factory/polish-4.md`.
- Replaced the simulated terminal graphic with an actual release-binary
  capture. Its raw output and generator are retained.
- Added a real 180 × 180 Apple icon and outcome-level metadata test.
- Restored h1 focus and announcements across browser Back and Forward.
- Added clean-sandbox claims for the Linux x64 download and `dist/site` build.
- Prevented heading word splits and kept all first-screen facts above the fold.
- Made **Start for real** discard every demo key while preserving real storage.
- Removed terminal animation and the unprovable future Terms promise.
- Added a keyboard-operable mobile menu containing every primary route.
- Corrected inverse surfaces so every route passes Axe in light and dark.
- Updated the catalog line to an 83-character verb-first description.

Implementation commits:

- `46e7f24b3a3952553e2c694b23305d5fb11dde6c` — cumulative review repair.
- `4e7a91fae38684ee1a3e0116fb0e8377ce4afd26` — both-theme contrast repair
  found during cold live verification.

## Verification

Final clean clone:
`/tmp/action-receipts-polish4-final.ztX7KS/repo` at `4e7a91f`.

```sh
npm ci --include=dev
npm run check
# Each of the 19 exact .factory/claims.json commands, run separately
cargo package --allow-dirty
```

Results:

- `npm run check`: PASS. It includes formatting, Clippy with warnings denied,
  TypeScript, 5 Rust unit tests, 9 CLI integration tests, 2 verifier tests, the
  production build, and 29 Playwright passes on desktop and 390 × 844. The one
  skip is the desktop instance of the mobile-only menu test.
- Claims: 19/19 PASS independently. Per-claim logs:
  `/work/.evidence/automation-action-receipts-polish-4/final-claim-*.log`.
- Accessibility: zero Axe violations on Home, Demo, Privacy, Terms, and 404 in
  light and dark on both Playwright projects. Skip links, the mobile menu, and
  forward/back focus are asserted.
- Privacy/offline: only same-origin demo requests; select/paste/verify adds no
  request; demo exit/reset remove only `demo:` keys; cached demo reload and
  verification pass offline.
- Build sizes: initial JS 10.21 kB raw / 3.95 kB gzip; CSS 14.75 kB raw /
  3.91 kB gzip; hero WebP 80 kB.
- Live Lighthouse mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; LCP 1.2 s, CLS 0, TBT 30 ms. Raw report:
  `/work/.evidence/automation-action-receipts-polish-4/lighthouse-live.json`.
- `cargo package`: PASS; 517.5 KiB package, 259.5 KiB compressed.

## Deployment and cold live evidence

Final deployment id: `31c36be8-b125-4d3f-8209-ea2919e3dd8a`.

Live URL: `https://automation-action-receipts.sociobot.in`.

Fresh contexts after deployment confirmed:

- Home and `/?demo=1` return 200 with no console errors. Worker reports are in
  `verify-final-home/verify.json` and `verify-final-demo/verify.json`.
- The demo query redirects to `/demo/`; its banner, verified state, and
  two-event count appear before scrolling at 390 × 844.
- The headline has no broken word. All facts end above 748 px on mobile and
  727 px on desktop. Neither viewport overflows.
- Back/Forward focuses and announces each route h1.
- All demo requests are same-origin. Leaving demo removed its demo key and
  preserved a real-data sentinel. Offline reload remained verified.
- Every live route has zero Axe violations in both themes. The custom 404
  returns HTTP 404. The Apple icon decodes to 180 × 180.
- The terminal asset contains no animation. The live download is ELF64 x86-64,
  reports version 0.1.0, creates temp output, and verifies its sample.

Primary evidence:

- `/work/.evidence/automation-action-receipts-polish-4/live-audit.json`
- `/work/.evidence/automation-action-receipts-polish-4/live-home-desktop.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-home-mobile.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-demo-mobile.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-mobile-menu.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-terminal-capture.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-home-dark.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-privacy.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-terms.png`
- `/work/.evidence/automation-action-receipts-polish-4/live-404.png`

## Known gaps and next steps

No known product, review, test, accessibility, privacy, offline, build, or live
deployment gap remains. Registry publishing stays factory-owned; the verified
package and Linux binary are ready for that separate release process.
