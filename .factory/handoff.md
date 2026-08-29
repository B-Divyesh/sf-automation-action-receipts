# Review 2 handoff

## Delivered

No product code was modified. Added `.factory/review-2.md`, an adversarial first-read review of the live deployment and the repository.

## Verification

Reviewed from a fresh clone at the specified base after `npm ci --include=dev` and `npm run build`:

```sh
npm run test:cli -- claim_cli_demo_lifecycle
npm run test:e2e -- --grep @claim:demo-isolated
npm run test:e2e -- --grep @claim:browser-verification
npm run test:e2e -- --grep @claim:receipt-never-uploaded
npm run test:e2e -- --grep @claim:two-mb-limit
npm run test:e2e -- --grep @claim:offline-reload
npm run test:e2e -- --grep @claim:no-account-and-no-telemetry
```

All listed claims except `demo-isolated` passed. That exact test fails on the mobile project because the demo banner/header causes horizontal overflow and puts the theme control outside the 390 px viewport. See F-2-1. The CLI demo itself was exercised from an unrelated temporary directory and left it unchanged.

## Live evidence

The live home first screen was clear at both 390 × 844 and 1440 × 900. `/`, `/demo/`, `/privacy/`, `/terms/`, the Linux binary, robots, and sitemap returned 200; an unknown route returned the designed 404 with HTTP 404. The live 390 px demo had `scrollWidth` 473 / `clientWidth` 390, and its verified result began at y=1737.

## Known gaps

The review verdict is FAIL. Blocking work remains: responsive immediate-use demo, passing mobile demo claim, CLI terminal recording, complete per-route metadata, shared route announcement, consistent header/footer, and honest/tested copy claims. See `.factory/review-2.md`.
