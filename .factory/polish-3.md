# Polish 3 — cumulative zero-finding closure

This table closes every numbered finding in reviews 1–3. Ranges enumerate each
individual finding in the range; no severity is deferred.

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-1-1, F-1-11–F-1-12, F-1-31 | Kept the task-led first screen: audience, one sample action, immediate result note, and sample-storage wording. | `@claim:demo-isolated`; live 390 px screenshot `/work/.evidence/automation-action-receipts-polish-3/live-home-mobile.png`; `https://automation-action-receipts.sociobot.in/`. |
| F-1-2, F-2-1–F-2-2 | Kept the one-click browser and CLI demos: direct `?demo=1` redirect, isolated `demo:` keys, reset/start-real controls, responsive banner, visible verified sample, and released-binary terminal recording. | `@claim:demo-isolated`, `@claim:terminal-recording`, `claim_cli_demo_lifecycle_creates_isolated_signed_outputs`; live demo screenshot `/work/.evidence/automation-action-receipts-polish-3/live-demo-cold-mobile.png`; `https://automation-action-receipts.sociobot.in/?demo=1`. |
| F-1-3, F-1-44–F-1-54, F-1-61–F-1-70, F-1-72–F-1-76, F-1-78–F-1-79, F-2-3–F-2-11 | Expanded `claims.json` locations and retained one observable sandbox test per published promise. Added `local-verification`; its CLI path uses `verify --offline` to block Linux networking and its browser path verifies from the cached demo offline. | All 17 exact manifest commands passed from clean clone `/tmp/action-receipts-clean.f1iWVn`; `@claim:local-verification`, `@claim:no-third-party-demo-requests`, CLI claim tests, and `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`. |
| F-1-4, F-1-23–F-1-26, F-1-55–F-1-60, F-1-71, F-1-77, F-1-80 | Kept the unavailable paid tier, checkout, merchant, price, and future-update promises removed. | `npm run test:e2e`; live link/routing audit `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`. |
| F-1-5, F-1-7, F-2-12, F-3-1 | Kept the product 404 and route metadata, then added original `social-card.webp` at exactly 1200 × 630. Every OG/Twitter image now uses it; it is cached and precached. | `@claim:site-metadata-and-routing`; `/work/.evidence/automation-action-receipts-polish-3/live-social-card.webp` (1200 × 630); live `/404.html` is 200 and `/not-a-receipt` is 404. |
| F-1-6, F-1-8 | Preserved focusable main landmarks, skip-focus behavior, h1 route focus, and polite announcements across home, demo, legal, and 404 routes. | `routes, focus, mobile width, and axe have no violations`; live audit records `skipTarget: main` and `Privacy` focus/announcement. |
| F-1-9 | Preserved the common header/footer skeleton, landmark names, legal links, external source label, factory line, and build label on every route. | `@claim:site-metadata-and-routing`; `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`. |
| F-1-10 | Kept the honesty content as a non-landmark note and ran full Axe on every route and viewport. | `routes, focus, mobile width, and axe have no violations`; live Axe arrays are empty in `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`. |
| F-1-13–F-1-22, F-1-27–F-1-43, F-2-13 | Preserved plain-language copy, result-naming controls, and short section names; updated the account fact to the precise “CLI demo needs no account.” | `.factory/copy-audit.md`; cold live home and demo screenshots in `/work/.evidence/automation-action-receipts-polish-3/`. |
| F-1-45–F-1-50, F-2-3–F-2-5, F-2-9, F-2-11 | Mapped local processing, no-third-party requests, 2 MB rejection, local file creation, demo isolation, cached offline reload, and both verifier paths to exact claim locations and sandbox tests. | `@claim:receipt-never-uploaded`, `@claim:no-third-party-demo-requests`, `@claim:two-mb-limit`, `@claim:offline-reload`, `@claim:local-verification`. |
| F-1-51–F-1-54, F-2-6–F-2-10 | Mapped declared scope, command provenance, redaction-before-storage, JSON/HTML equivalence, private key permissions, and unknown-field rejection to direct CLI fixtures. | `claim_declared_boundary_fields_are_written_before_events`, `claim_command_provenance_records_command_output_and_artifact_hash`, `claim_redact_before_storage_removes_literal_and_default_key_secrets`, `claim_json_html_export_embeds_the_signed_receipt`, `claim_private_key_is_separate_and_private`, `claim_unknown_fields_are_rejected`. |
| F-1-56–F-1-57, F-1-59–F-1-60 | Removed the unimplemented permanent-price, unlimited, future, merchant, and checkout statements. | Landing copy audit and live source/link audit. |
| F-1-58 | The paid feature statement is absent with the rest of the unavailable paid tier. | Landing copy audit and `npm run test:e2e` link crawl. |
| F-1-73–F-1-75 | Kept only concrete, testable build and local-operation documentation. | Clean-clone `npm run build`, `npm test`, and `npm run test:e2e`. |
| F-1-76 | Kept the precise separate-demo-storage wording and reset behavior. | `@claim:demo-isolated`; live reset result in `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`. |
| F-3-2 | Renamed and mapped the landing file fact to `cli-demo-lifecycle`; the fixture asserts fresh OS-temp JSON and HTML files and verifies both. | `claim_cli_demo_lifecycle_creates_isolated_signed_outputs`; clean-clone manifest run. |
| F-3-3 | Narrowed the landing account promise to the CLI demo and mapped it to its empty-environment process test, including the privacy-page account statement. | `claim_cli_demo_needs_no_account_or_environment_credentials`; live home screenshot. |
| F-3-4 | Added `local-verification`, which verifies a signed receipt with the CLI network lock and a cached browser sample while its browser context is offline. | `@claim:local-verification` in Chromium and mobile; clean-clone manifest run. |

## Live closure

Static deployment completed with deployment id
`b13d5a0f-af22-4671-9485-20dc3453bbc1`. Cold live checks at
`https://automation-action-receipts.sociobot.in` found no console errors, no
mobile overflow, zero Axe violations, and the designed 404 for an unknown URL.
The raw result is `/work/.evidence/automation-action-receipts-polish-3/live-audit.json`.

No finding remains open.
