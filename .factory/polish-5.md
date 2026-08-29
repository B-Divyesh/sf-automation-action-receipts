# Polish 5 — cumulative zero-finding closure

Repair commit: `e95a3e107ca943accf3ef051010b76e9cf21d71a`.
Deployment: `8c12dfda-10c3-407b-9c89-518fdd6e2c55` at
`https://automation-action-receipts.sociobot.in`.

## Evidence key

- **C** — independent clean clone at
  `/tmp/action-receipts-polish5-clean-eGlyHR/repo`, checked out at `e95a3e1`.
  `full-check.log` records `npm run check`; `claim-<id>.log` records each
  exact manifest command; `claims-summary.txt` records all 20 passes.
- **L** — cold public-site evidence in
  `/work/.evidence/automation-action-receipts-polish5-live-xjYPCg/`:
  `verify.json`, `screenshot-desktop.png`, `screenshot-mobile.png`,
  `live-demo-mobile.png`, `live-audit.json`, and `lighthouse-mobile.json`.

Every range below is inclusive: every individual ID in the range has the
listed repair and evidence. Reopened IDs are included with their original ID.

| Finding IDs | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the task-led first screen, named teams using agents/scripts/CI, one sample action, and an immediate result note. | C first-screen Playwright test; L `screenshot-mobile.png`; live `/`. |
| F-1-2 | Kept the direct `/demo/` and `?demo=1` isolated sample, banner/reset/exit controls, temp-directory CLI demo, and release-binary terminal capture. | C `demo-isolated`, `cli-demo-lifecycle`, `terminal-recording`; L `live-demo-mobile.png`; live `/demo/`. |
| F-1-3 | Retained a complete claims manifest and added the missing MIT claim, bringing it to 20 exact commands. | C `claims-summary.txt`; `.factory/claims.json`. |
| F-1-4 | Kept unavailable paid checkout and related copy absent. | C link/routing suite; L cold home screenshot and route audit. |
| F-1-5 | Kept the product-styled 404 and Static Web Apps response override. | C `site-metadata-and-routing`; L `live-audit.json` records designed unknown-route 404. |
| F-1-6 | Kept focusable main landmarks and skip-link behavior on every route. | C routes/focus/Axe test; L all-route audit. |
| F-1-7 | Kept route-specific metadata, social card, manifest, and the 180 px Apple icon. | C `site-metadata-and-routing`; L all-route audit. |
| F-1-8 | Kept route focus, polite announcement, Back/Forward restoration, and consistent deep links. | C routes/focus/Axe test; L all-route audit. |
| F-1-9 | Kept the shared header/footer, legal links, source label, factory attribution, and build label. | C `site-metadata-and-routing`; L all-route audit. |
| F-1-10 | Kept integrity text as a non-landmark note; live Axe is empty in both themes. | C routes/focus/Axe test; L `live-audit.json`. |
| F-1-11–F-1-14 | Kept the direct job headline, audience, plain receipt sequence, and benefit-first signing language. | C copy audit and first-screen test; L home screenshots. |
| F-1-15–F-1-20 | Kept plain verifier and process wording, with observable browser/CLI verification tests. | C `browser-verification`, `declared-boundary-fields`, `command-provenance`, and `json-html-export`. |
| F-1-21–F-1-34 | Kept accurate CLI/install, export, control, and section wording; removed mood and false-action copy. | C `.factory/copy-audit.md`; L home and demo screenshots. |
| F-1-35–F-1-43 | Kept concise README task, demo, privacy, and verifier wording. | C copy audit; `local-verification`, `offline-reload`, and `receipt-never-uploaded`. |
| F-1-44 | Kept the full CLI sample lifecycle: record, sign, export, and verify. | C `claim-cli-demo-lifecycle.log`. |
| F-1-45–F-1-50 | Kept precisely scoped account, privacy, tamper, limit, and local-processing statements. | C `cli-no-account`, `browser-verification`, `receipt-never-uploaded`, and `two-mb-limit`; L `live-audit.json`. |
| F-1-51–F-1-54 | Kept declared boundary, provenance, redaction, JSON/HTML, and integrity-limit behavior. | C `declared-boundary-fields`, `command-provenance`, `redact-before-storage`, and `json-html-export`. |
| F-1-55, F-1-57–F-1-60 | Kept unsupported paid, permanent-price, unlimited, merchant, and future-update assertions removed. | C copy/link suite; L home and Terms audit. |
| F-1-56 | Added `mit-license` to the manifest and a focused test of LICENSE permission/warranty clauses plus README and Terms references. | C `claim-mit-license.log`; L live Terms source check. |
| F-1-61–F-1-70 | Kept only tested Rust, download, key, provenance, schema, redaction, export, and retention statements. | C `linux-download`, `private-key-permissions`, `command-provenance`, `unknown-fields-rejected`, and `redact-before-storage`. |
| F-1-71 | Kept duplicate redaction/retention marketing absent. | C copy audit; L home audit. |
| F-1-72 | Added the landing verifier eyebrow and landing/demo offline status bar to both relevant claim locations. The local-verification test now observes the visible offline bar for a valid receipt and for tampering failure. | C `claim-local-verification.log`; L `live-audit.json` (`validBeforeTamper`, `statusBar`, `tamperFails`). |
| F-1-73–F-1-80 | Kept only testable build, storage, network, demo, host, billing, and refund wording. | C `site-build-output`, `demo-isolated`, `no-third-party-demo-requests`, and copy audit. |
| F-2-1–F-2-5 | Kept mobile demo reflow, immediate sample proof, `demo:` isolation, terminal truthfulness, and local CLI files. | C `demo-isolated`, `terminal-recording`, `cli-demo-lifecycle`; L `live-demo-mobile.png`. |
| F-2-6–F-2-11 | Kept claim mappings for boundary fields, provenance, redaction, exports, README capabilities, privacy, and offline behavior. | C matching claim logs; L same-origin/offline audit. |
| F-2-12–F-2-13 | Kept the plain, task-named section labels and text. | C copy audit; L home screenshot. |
| F-3-1 | Kept the product-derived 1200 × 630 social image on every route. | C `site-metadata-and-routing`; L route audit. |
| F-3-2–F-3-4 | Kept landing local-file/account facts and README no-server statement mapped to observable tests. | C `cli-demo-lifecycle`, `cli-no-account`, and `local-verification`. |
| F-4-1–F-4-5 | Kept intact headline words, real demo cleanup, static terminal capture, keyboard mobile menu, and no future Terms promise. | C first-screen, demo-isolated, reduced-motion, mobile-menu, and copy tests; L screenshots. |
| F-5-1 | Changed the server-rendered demo loading heading from `h3` to `h2`; added a direct initial-HTML outline regression test. | C `demo loading markup keeps a complete heading outline`; L `live-audit.json` (`loadingUsesH2: true`, `loadingUsesH3: false`). |

## Final live check

The deployed public URL was opened from cold contexts after deployment. Home,
Demo, Privacy, Terms, and 404 each have one h1/main, their correct titles, no
console errors, and zero Axe violations in light and dark modes. The demo
redirect, persistent banner, reset isolation, same-origin request set, valid
offline verification, offline tamper rejection, initial h1 → h2 loading
outline, and designed 404 all passed. Lighthouse mobile scored 100 for
performance, accessibility, best practices, and SEO (LCP 1.21 s, CLS 0,
TBT 0). No finding remains open.
