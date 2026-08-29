# Polish 2 handoff

## Delivered

Release repair commit: `5e22aa3cdb0f40a4b54b78a0ed8db1db5e56a5c9`, with final
claim-coverage commit `e56d1b7dbd9ea1addcf6672b30452a5709dd9bf7`. All are pushed
to `origin/main`; the product artifact is deployed to
https://automation-action-receipts.sociobot.in.

The browser demo is now an isolated, first-screen working receipt: `/demo/`
and `/?demo=1` load the same verified sample, banner, reset control, actor,
scope, and command at 390 px. The reset removes only `demo:` storage. The
landing page now includes the required self-hosted terminal recording of the
real CLI demo. Metadata, route announcements, focus handling, shared chrome,
404, copy, and claim coverage were repaired across every route.

## Verification

- Fresh clone: `/tmp/action-receipts-clean3.nowo8H` at `e56d1b7` ran
  `npm ci --include=dev`, `npm test`, `npm run build`, and every exact command
  in `.factory/claims.json`; all passed (`test-results/.last-run.json` records
  `"status": "passed"`).
- Latest local gate: `npm run check` passed. It includes formatting, clippy,
  TypeScript, 14 Rust tests, 2 Vitest tests, production build, and 18
  Playwright checks across desktop and 390 px mobile.
- Production build: initial JavaScript is 9.11 kB raw / 3.61 kB gzip and CSS
  is 14.18 kB raw / 3.79 kB gzip. `dist/site` contains the release binary.
- Deployment: Static Web Apps deployment `36937edc-a768-4e37-8831-219e8dfce5d8`
  completed successfully.
- Cold live verification: `/opt/fleet/lib/verify-url.sh` passed at
  https://automation-action-receipts.sociobot.in/ with HTTP 200, title/lang,
  one h1, main landmark, alt text, labeled buttons, and no console errors.
  Evidence: `/tmp/action-receipts-live-final.8AVT4M/verify.json` and
  `/tmp/action-receipts-live-final.8AVT4M/screenshot-mobile.png`.
- Live Playwright+Axe check passed on `/`, `/demo/`, `/privacy/`, `/terms/`,
  and `/404.html`: zero violations, one h1 and main each, no mobile overflow,
  and no console errors. `/demo/` had a visible banner, verified panel, and
  actor before the 390 px fold. Evidence:
  `/tmp/action-receipts-live-final.8AVT4M/live-routes.json` and
  `/tmp/action-receipts-live-final.8AVT4M/live-demo-mobile.png`.
- The standalone `@axe-core/cli` could not start because its Selenium Chrome
  binary is absent in this worker. The repository and live checks use the
  installed Playwright Chromium with `@axe-core/playwright`, which completed
  successfully on every route and viewport.

## Run and deploy

```sh
npm ci --include=dev
npm run check
npm run build:site
/opt/fleet/lib/deploy-static.sh automation-action-receipts dist/site
```

## Known gaps

None. No review finding, TODO, stub, or deferred acceptance item remains.
