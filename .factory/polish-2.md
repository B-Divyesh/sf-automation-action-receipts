# Polish 2 — cumulative review closure

All local browser evidence below is produced by `npm run test:e2e` in both the
desktop and 390 × 844 mobile projects. Post-deploy evidence is recorded in the
handoff after the production upload.

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-1-1, F-1-11–F-1-12 | Retained the task-led headline, audience sentence, and one clear sample action. | `routes, focus, mobile width, and axe`; mobile screenshot in handoff evidence. |
| F-1-2, F-2-1, F-2-2 | `/demo/` now puts a loaded verified receipt, event count, actor, scope, and command before the fold. The banner is above the header, wraps at 390 px, resets only `demo:` storage, and `?demo=1` redirects directly to it. | `@claim:demo-isolated` on desktop and mobile. |
| F-1-2 (CLI recording) | Added `terminal-demo.svg`, a self-hosted animated terminal recording captioned as a released-binary `action-receipts demo` run. | `@claim:terminal-recording`; `site/public/terminal-demo.svg`. |
| F-1-3, F-1-44–F-1-54, F-1-61–F-1-75, F-2-3–F-2-11 | Replaced the manifest with observable, clean-sandbox claims for every retained product promise; narrowed or removed unsupported marketing wording. | Every command in `.factory/claims.json`; Rust claim tests and Playwright claim tests. |
| F-1-4, F-1-23–F-1-26, F-1-55–F-1-60, F-1-71, F-1-77, F-1-80 | The unavailable paid tier, checkout, license, price, and merchant claims remain removed. | Landing/link test and copy audit. |
| F-1-5, F-1-7, F-2-12 | Kept the product 404 and added complete canonical, manifest, apple-touch, Open Graph, and Twitter metadata on every shipped document. Removed the non-useful eyebrow. | `@claim:site-metadata-and-routing`; `site/public/staticwebapp.config.json`. |
| F-1-6, F-1-8 | Every route has a focusable main landmark and polite route announcement. Internal navigation and browser back/forward focus and announce the h1. | `routes, focus, mobile width, and axe`. |
| F-1-9 | Standardized the same four primary links, landmark labels, and source/legal footer on home, demo, legal, and 404 routes. | `@claim:site-metadata-and-routing`. |
| F-1-10 | Preserved the in-main honesty note as `role="note"`; full Axe now runs on every route and viewport. | `routes, focus, mobile width, and axe`. |
| F-1-13–F-1-22, F-1-27–F-1-43, F-2-13 | Removed algorithm-only and slogan copy; rewrote instructions in short task language. | `.factory/copy-audit.md`. |
| F-1-45–F-1-46, F-2-3 | Replaced “No telemetry” with the precise, tested “No third-party demo requests.” | `@claim:no-third-party-demo-requests`. |
| F-1-47–F-1-50, F-2-9 | Verified chain tampering, browser-only processing, 2 MB rejection, offline reload, and equivalent signed JSON/HTML exports. | `@claim:browser-verification`, `@claim:receipt-never-uploaded`, `@claim:two-mb-limit`, `@claim:offline-reload`, `claim_json_html_export_embeds_the_signed_receipt`. |
| F-1-51–F-1-53, F-2-6–F-2-8 | Added direct CLI fixtures for declared fields, command provenance, artifact hashes, and before-storage redaction. | `claim_declared_boundary_fields_are_written_before_events`, `claim_command_provenance_records_command_output_and_artifact_hash`, `claim_redact_before_storage_removes_literal_and_default_key_secrets`. |
| F-1-63–F-1-70, F-2-10 | Added direct key-permission and unknown-field fixtures; kept documentation only where an executable claim covers it. | `claim_private_key_is_separate_and_private`, `claim_unknown_fields_are_rejected`. |
| F-1-72, F-2-11 | Demo URL is service-worker cached and reload-tested offline; the README maps its browser privacy wording to the request/no-upload checks. | `@claim:offline-reload`, `@claim:receipt-never-uploaded`. |
| F-1-76, F-2-4 | Demo documentation and banner now say “separate demo storage”; test asserts reset preserves a real-data sentinel. | `@claim:demo-isolated`; `.factory/demo.md`. |
| F-1-78–F-1-79 | Removed unprovable host-retention/advertising language and retained only a request-log claim that the demo makes no third-party request. | `@claim:no-third-party-demo-requests`. |

## Final live evidence

Cold checks were repeated after deployment at
https://automation-action-receipts.sociobot.in. The live route matrix (home,
demo, Privacy, Terms, and 404) has one h1 and main per route, zero Axe
violations, zero mobile overflow, and no normal-route console errors. The demo
banner and verified panel were visible above the 390 × 844 fold. Evidence:
`/tmp/action-receipts-live-final.8AVT4M/live-routes.json`,
`/tmp/action-receipts-live-final.8AVT4M/live-demo-mobile.png`, and
`/tmp/action-receipts-live-final.8AVT4M/screenshot-mobile.png`.

No deferred findings or TODOs remain.
