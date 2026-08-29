# Review 5 handoff

## Delivered

- Completed the adversarial mobile/desktop review in `.factory/review-5.md`.
- Re-ran the entire checklist rather than reviewing only the prior diff.
- Changed no product code or product assets.

Verdict: **FAIL** with two reopened blocking findings and one minor finding:

- F-1-56: the retained MIT statements have no claim-manifest entry or test.
- F-1-72: the landing/demo offline status locations remain absent from the
  manifest even though the underlying offline tests pass.
- F-5-1: the demo's initial loading HTML skips from h1 to h3.

## Verification

Fresh clone: `/tmp/action-receipts-review5-clone.ethzXv/repo` at
`c32b4ecf66aaa46c8000857669ada24ffccfe046`.

```sh
npm ci --include=dev
# Run each of the 19 commands in .factory/claims.json separately.
npm run check
```

Results:

- 19/19 listed claim commands passed independently. Logs are under
  `/tmp/action-receipts-review5/claims/`.
- `npm run check` passed: formatting, Clippy, TypeScript, 5 Rust unit tests,
  9 CLI tests, 2 Vitest tests, production build, and 29 Playwright passes with
  one intended desktop skip.
- Fresh live Chromium checks covered 390 × 844 and 1440 × 900, demo reset and
  exit isolation, same-origin request logging, offline reload and verification,
  deep links, Back/Forward focus announcements, keyboard mobile navigation,
  every route in both themes with Axe, and the full link crawl.
- `/opt/fleet/lib/verify-url.sh` passed live Home and Demo. Evidence is in
  `/tmp/action-receipts-review5/verify-home/` and `verify-demo/`.
- The live Linux x64 binary reported 0.1.0, ran its demo from an empty temporary
  caller, left that caller unchanged, created JSON and HTML in a new OS temp
  directory, and verified the two-event JSON receipt.
- Live Home, Demo, Privacy, Terms, and 404 HTML matched the clean build
  byte-for-byte.

## Remaining work

Implement the three concrete fixes in `.factory/review-5.md`, add their
regression coverage, and repeat the full claim matrix. No deployment was
performed from this review work order.
