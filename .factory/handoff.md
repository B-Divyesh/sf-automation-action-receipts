# Polish 3 handoff

## Delivered

- Commit `ae240be05bc046752eb2988625072493b7d53336` closes review 3.
- Added original product-derived `site/public/social-card.webp` at 1200 × 630,
  used by OG and Twitter metadata on every route and included in the service
  worker and immutable cache policy.
- Mapped the landing, README, demo, and legal-page promises to claim locations.
  Added the `local-verification` claim and an actual Linux `verify --offline`
  mode that blocks network syscalls while the CLI verifies a local receipt.
- Kept the direct isolated demo, visible mobile proof panel, reset/start-real
  controls, route focus/announcement, designed 404, and product-specific
  chain-of-custody visual system intact.
- Updated the verb-first catalog description and expanded the copy audit.

## Verification

Fresh clone `/tmp/action-receipts-clean.f1iWVn` at `ae240be` completed:

```sh
npm ci --include=dev
npm run build
# every exact command in .factory/claims.json (17 total)
npm test
npm run test:e2e
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
npm pack --dry-run
```

Results: 5 Rust library tests, 9 CLI integration tests, 2 browser-verifier
unit tests, and 20 Playwright desktop/mobile tests passed. The production build
contains `dist/site/social-card.webp` (1200 × 630) and the executable Linux
x64 CLI download.

The static work-order build/output was published as deployment
`b13d5a0f-af22-4671-9485-20dc3453bbc1` to
https://automation-action-receipts.sociobot.in.

Post-deploy checks:

- `/opt/fleet/lib/verify-url.sh` passed for home and `/demo/` with no console
  errors, one h1/main, lang/title, and complete image alt/button labels.
- Cold live Playwright+Axe covered home, demo, Privacy, Terms, and 404 at
 390 × 844: zero Axe violations, zero horizontal overflow, all routes use the
  1200 × 630 social card, and an unknown route returns HTTP 404.
- Cold `?demo=1` redirected to `/demo/`, showed the banner and verified sample,
  made only same-origin requests, and reset `demo:ar_theme` while preserving
  `ar_real_marker`.
- Lighthouse mobile retry: Performance 100, Accessibility 100, Best Practices
 100, SEO 100; LCP 1204 ms and CLS 0.

Evidence: `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`,
`live-demo-cold-mobile.png`, `live-social-card.webp`, `live-home/verify.json`,
`live-demo/verify.json`, and `lighthouse-mobile-retry.json`.

## Run and deploy

```sh
npm ci --include=dev
npm run build:site
npm test
npm run test:e2e
```

Publish `dist/site/` with the static work-order deployment. The factory owns
registry publishing; prepare the CLI with `cargo package` but do not publish it
from this repository.

## Remaining work

None.
