# Review 3 handoff

## Delivered

Created `.factory/review-3.md` for the requested adversarial first-read review.
No product code was modified.

## Verification performed

- Audited the live product cold at 390 × 844 and 1440 × 900.
- Checked the demo banner, visible sample result, reset isolation, real-data
  sentinel preservation, request origins, console errors, direct CLI demo, all
  routes, and all crawled links.
- Created a fresh clone at `3c530dd56fe2efca28f709c8415f651f7478a5f6`, ran
  `npm ci --include=dev`, `npm run build`, every exact command in
  `.factory/claims.json`, `npm test`, and `npm run test:e2e`; all passed.

## Result and remaining work

Review verdict: **FAIL** with four minor findings. The required original social
share image is 1200 × 800 rather than 1200 × 630, and three public
capability/account/no-server statements need exact claims-manifest location and
test coverage. Details and concrete fixes are in `.factory/review-3.md`.
