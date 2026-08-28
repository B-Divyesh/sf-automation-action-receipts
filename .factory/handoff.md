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

All passed locally on 2026-08-28. `npm test`: 5 Rust library tests, 2 CLI integration tests, and 2 verifier unit tests. `npm run test:e2e`: 14 desktop/mobile browser tests, including full axe scans with zero violations. Every command in `.factory/claims.json` passed. Production build: JS 8.95 kB raw / 3.57 kB gzip; CSS 12.91 kB raw / 3.58 kB gzip.

## Deployment evidence

Commit and live URL verification are appended after deployment. Deploy with:

```sh
/opt/fleet/lib/deploy-static.sh automation-action-receipts dist/site
```

## Known gaps

None. The product intentionally has no paid tier until a working product registration exists.
