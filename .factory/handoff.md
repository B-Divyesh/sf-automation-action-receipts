# Polish 1 handoff

## Delivered

- Added a real `action-receipts demo` command. It makes an approved documentation-deployment receipt, signs JSON and HTML, and prints paths inside a new temporary directory.
- Added a direct isolated browser demo at `/demo/` and `?demo=1`, with automatic sample loading, banner, reset, exit, and `demo:` storage namespace.
- Rewrote first-screen, verifier, CLI, legal, and README copy; removed the unavailable paid tier rather than advertising a dead checkout.
- Added claims, claim tests, complete route metadata, a product 404 page, Static Web Apps 404 policy, responsive/accessible navigation, focus transfer, and consistent legal footer/header.

## Verification

Run from a clean checkout:

```sh
npm ci
npm test
npm run build:site
npm run test:e2e
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo package --allow-dirty
npm pack --dry-run
```

All passed locally on 2026-08-28 and again from a fresh clone at `/tmp/tmp.3GidLUhRFS`. `npm test`: 5 Rust library tests, 2 CLI integration tests, and 2 verifier unit tests. `npm run test:e2e`: 14 desktop/mobile browser tests, including full axe scans with zero violations. Every command in `.factory/claims.json` passed. Production build: JS 8.95 kB raw / 3.57 kB gzip; CSS 12.91 kB raw / 3.58 kB gzip.

## Deployment evidence

Commit `0056cb3cec9c7e82858b715972c72a10e75d9b10` is pushed to `main` and was
deployed with `/opt/fleet/lib/deploy-static.sh automation-action-receipts dist/site`.
Cold live checks on 2026-08-28 passed:

- `https://automation-action-receipts.sociobot.in/demo/` loaded title **Demo — Action Receipts**, one demo banner, and one verified receipt at 390 × 844.
- Full live Axe returned `[]`; browser console errors returned `[]`.
- `/opt/fleet/lib/verify-url.sh` returned HTTP 200 with `lang=en`, one h1, one main, alt text, labeled controls, and no console errors.
- `https://automation-action-receipts.sociobot.in/not-a-real-route` returned 404 with the product 404 wording, not the Azure page.
- Live screenshot: `/tmp/action-receipts-live-demo.png`; verifier evidence: `/tmp/tmp.PE28WVoUdG` (worker-local evidence paths).

## Known gaps

None. The product intentionally has no paid tier until a working product registration exists.
