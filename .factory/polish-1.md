# Polish 1 — review finding closure

All checks below use the local production build. The final deployment check is
recorded in `handoff.md`.

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-1-1, F-1-11, F-1-12, F-1-31 | Rewrote the first screen with the requested job, audience, primary sample action, and plain next-step text. | `tests/e2e/site.spec.ts` first-screen/demo tests; `/` and `/demo/` live check. |
| F-1-2 | Added `action-receipts demo`, realistic two-event temp-directory output, `/demo/`, `?demo=1`, a persistent banner, reset, exit, docs, and example scenario. | `claim_cli_demo_lifecycle`, `@claim:demo-isolated`; `/demo/` live check. |
| F-1-3 | Added the manifest and one tagged clean-sandbox test for each remaining visitor claim. Unprovable marketing was deleted. | `.factory/claims.json`; every listed command passed. |
| F-1-4, F-1-23–F-1-26, F-1-55–F-1-60, F-1-77, F-1-80 | Removed the unavailable paid tier, checkout, license storage, price, future-update, merchant, and refund claims. | Link crawl in `npm run test:e2e`; `/` live check. |
| F-1-5, F-1-7 | Added product-styled `404.html`, Static Web Apps override, `/demo/`, canonical/OG/Twitter/theme metadata, manifest icon, and sitemap route. | route/axe e2e test; `/404` and unknown-route live check. |
| F-1-6, F-1-8, F-1-9 | Made main focusable on every route; navigation transfers focus to h1; shared header/footer now include legal links, product line, factory, version, and external labels. | route/keyboard e2e test; `/privacy/`, `/terms/` live check. |
| F-1-10 | Replaced the nested `aside` landmark with a note role. | full axe run in `npm run test:e2e`. |
| F-1-13–F-1-22, F-1-27–F-1-34 | Rewrote jargon, slogans, vague headings, labels, and ambiguous controls in plain language. | `.factory/copy-audit.md`; `/` live check. |
| F-1-35–F-1-43 | Rewrote README into short task-led sentences, documented real demo/install/test/deploy paths, and removed obsolete payment language. | `README.md`; `npm test`, `cargo package`. |
| F-1-44 | Retained the concrete receipt lifecycle as the CLI demo claim. | `@claim:cli-demo-lifecycle`. |
| F-1-45–F-1-46 | Retained account/telemetry facts and added a request-log claim test. | `@claim:no-account-and-no-telemetry`. |
| F-1-47, F-1-49, F-1-54 | Retained only verifiable signature/integrity wording and tamper warning. | `@claim:browser-verification`; CLI integration test. |
| F-1-48 | Retained browser-local file wording and tested outgoing requests. | `@claim:receipt-never-uploaded`. |
| F-1-50 | Retained the 2 MB limit and tested rejection above it. | `@claim:two-mb-limit`. |
| F-1-51–F-1-53, F-1-61–F-1-70 | Kept the CLI capabilities, simplified documentation, and exercised lifecycle/redaction/HTML/CLI behavior in Rust integration tests. | `npm test`; `claim_cli_demo_lifecycle`; existing Rust library/integration tests. |
| F-1-56–F-1-57, F-1-59–F-1-60 | Deleted permanent-price, unlimited, future, merchant, and checkout promises. | copy audit and link crawl. |
| F-1-71 | Removed duplicate plan claim with the paid plan. | `/` live check. |
| F-1-72 | Retained qualified offline wording and tested cached demo reload. | `@claim:offline-reload`. |
| F-1-73–F-1-75 | Removed self-congratulatory test/network claims; retained build and local-operation evidence in commands/tests. | `npm run build:site`; test suite. |
| F-1-76 | Replaced storage description with the actual theme and `demo:` namespace behavior. | `@claim:demo-isolated`. |
| F-1-78–F-1-79 | Removed host-retention and broad CDN/analytics promises; the request-log test covers the remaining concrete demo behavior. | `@claim:no-account-and-no-telemetry`; 404 live check. |

No deferred or stubbed findings remain.
