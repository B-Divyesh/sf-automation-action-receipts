# Adversarial first-read review 3 — FAIL

Reviewed 2026-08-29 at `https://automation-action-receipts.sociobot.in` from
new Chromium contexts at 390 × 844 and 1440 × 900. Repository base:
`3c530dd56fe2efca28f709c8415f651f7478a5f6`.

## Verdict

**FAIL.** The core experience is now clear, functional, and unusually close to
complete, but four minor, concrete contract findings remain. The acceptance
standard for this review is zero findings.

## First screen, before scrolling

- **What it does:** records and verifies automated changes in a signed local
  receipt.
- **For whom:** teams using agents, scripts, or CI to change repositories and
  services.
- **First click:** **Try it with sample data**. The adjacent text says a signed
  deployment receipt will appear and that the demo uses separate sample
  storage.

All three answers are present at 390 px and desktop. On the phone the primary
link is fully visible at y=541–592, with no horizontal overflow. This is not a
blocking first-read failure.

## Findings

### F-3-1 — social share image does not meet the required 1200 × 630 size

- **Location/quote:** every route's `og:image` and `twitter:image` point to
  `/receipt-chain.webp`; home markup declares the same image at
  `width="1200" height="800"`.
- **Why this fails:** the site-structure contract requires a real 1200 × 630
  product-derived social image. The current 3:2 hero art is not that asset,
  even though it is original and visually on-thesis.
- **Concrete fix:** create and ship an original 1200 × 630 crop/composition
  (for example `/social-card.webp`), update OG and Twitter image references on
  every route, and add its dimensions to the metadata route test.

### F-3-2 — the landing claim “Creates local receipt files” has no matching listed claim location

- **Location/quote:** home first-screen fact: **“Creates local receipt files”**.
- **Why this fails:** `cli-demo-lifecycle` proves that the *demo* creates JSON
  and HTML in a temporary directory, but its `where` field is only “CLI help,
  README.” No manifest entry declares this landing promise or verifies the
  general local-file wording at its published location.
- **Concrete fix:** add `landing facts` to `cli-demo-lifecycle` and rename the
  claim to match this sentence, with the existing fresh-temp-dir output-path
  assertion retained. Alternatively remove the fact.

### F-3-3 — the landing claim “No account needed” is not mapped to its claim test

- **Location/quote:** home first-screen fact: **“No account needed”**.
- **Why this fails:** `cli-no-account` tests a fresh CLI process without
  environment credentials, but its declared locations are only “CLI help,
  README.” The landing makes the product-wide statement without an entry that
  says it appears there. This leaves a reader unable to trace the fact to the
  promised test.
- **Concrete fix:** add `landing facts` to `cli-no-account` and assert the
  browser demo loads in a fresh unauthenticated context as part of that claim;
  or change the fact to **“CLI demo needs no account”** and keep it tied to the
  existing CLI test.

### F-3-4 — README’s no-server claim is broader than any listed claim

- **Location/quote:** README opening: **“The browser verifier and CLI verify
  receipts without a server.”**
- **Why this fails:** `browser-verification` proves a browser chain/tamper
  result, and `cli-no-account` proves absent credentials. Neither named claim
  says that both verifiers require no server, nor does either test demonstrate
  the complete stated boundary for both products.
- **Concrete fix:** add a `local-verification` claim at this README location.
  Test the CLI verifier with networking disabled and test a cached browser demo
  verification with networking disabled. Otherwise split/remove the sentence.

## Copy audit

Counts are whitespace-delimited. Code blocks are commands, not reader-facing
sentences. Every landing and README sentence is at or below 22 words. No banned
marketing adjective, metaphor/mood heading, inconsistent product term, or
non-result-naming button was found. “—” means no copy finding.

### Landing page

| Words | Sentence, heading, or control | Result |
| ---: | --- | --- |
| 3 | Skip to content | — |
| 2 | Action Receipts | — |
| 1 | Demo | — |
| 1 | Verify | — |
| 3 | How it works | — |
| 1 | Privacy | — |
| 3 | Use dark theme | — |
| 2 | Version 0.1.0 | — |
| 5 | Record and verify automated changes. | — |
| 12 | For teams using agents, scripts, or CI to change repositories and services. | — |
| 5 | Try it with sample data | — |
| 3 | Install the CLI | — |
| 5 | See a signed deployment receipt. | — |
| 6 | The demo uses separate sample storage. | — |
| 4 | Creates local receipt files | F-3-2 |
| 3 | No account needed | F-3-3 |
| 4 | No third-party demo requests | — |
| 7 | Stated approval → recorded command → file hash → signature | — |
| 2 | Offline verifier | — |
| 5 | Verify a signed receipt. | — |
| 13 | The verifier checks every recorded event and the receipt signature on your device. | — |
| 4 | Choose a receipt JSON | — |
| 7 | or drop it here · maximum 2 MB | — |
| 2 | Receipt JSON | — |
| 2 | Verify receipt | — |
| 5 | Try it with sample data | — |
| 1 | Unchecked | — |
| 3 | No receipt loaded | — |
| 8 | Choose a receipt JSON or try the sample. | — |
| 7 | Selected files stay in this browser tab. | — |
| 3 | How it works | — |
| 5 | Create, record, sign, and verify. | — |
| 4 | Record the approved scope. | — |
| 6 | Start by recording the approval and scope. | — |
| 6 | Record a command or tool result. | — |
| 7 | Keep the command, result, and file hash together. | — |
| 5 | Sign and export the receipt. | — |
| 7 | Export JSON or a self-contained HTML report. | — |
| 4 | Integrity is not identity. | — |
| 9 | A valid receipt shows signed contents have not changed. | — |
| 13 | It does not prove approval was legitimate or an action was correct. | — |
| 3 | Command-line quick start | — |
| 7 | Run the sample or use your own change. | — |
| 8 | Recorded from the released v0.1.0 binary running action-receipts demo. | — |
| 2 | Copy commands | — |
| 3 | Download Linux x64 | — |
| 3 | Read the source (GitHub) | — |
| 2 | Export formats | — |
| 7 | Export JSON or a self-contained HTML report. | — |
| 3 | Install the CLI | — |
| 3 | AR/ Action Receipts | — |
| 7 | Record and verify automated repository and service changes. | — |
| 1 | Terms | — |
| 2 | Source (GitHub) | — |
| 5 | Built by Param Factory · v0.1.0 | — |

### README

| Words | Sentence or heading | Result |
| ---: | --- | --- |
| 2 | Action Receipts | — |
| 9 | Record and verify automated changes in a local receipt file. | — |
| 14 | It is for teams using agents, scripts, or CI to change repositories and services. | — |
| 8 | Each event links to the event before it. | — |
| 5 | A signature detects later changes. | — |
| 10 | The browser verifier and CLI verify receipts without a server. | F-3-4 |
| 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | — |
| 1 | Install | — |
| 2 | Try the demo | — |
| 6 | Run this command from any directory. | — |
| 18 | It creates signed JSON and HTML receipts in a new temporary directory and prints both paths. | — |
| 12 | Open `/demo/`, or `/?demo=1`, for the isolated browser sample. | — |
| 11 | It loads a signed documentation deployment receipt using separate `demo:` storage. | — |
| 4 | Use your own change | — |
| 9 | New receipts use a separate private signing key. | — |
| 13 | A command receipt includes its arguments, result, duration, exit status, and declared file hashes. | — |
| 10 | Literal and default-key secrets are redacted before receipt data is stored. | — |
| 10 | The CLI exports signed JSON and a self-contained HTML report. | — |
| 8 | The verifier rejects receipt JSON with unknown fields. | — |
| 4 | Browser verifier and privacy | — |
| 11 | The browser processes selected receipt text without a data request. | — |
| 10 | After one visit, the demo can reload offline. | — |
| 7 | The demo makes no third-party requests. | — |
| 10 | See the product privacy and terms pages before using sensitive receipt data. | — |
| 5 | Test, package, and deploy | — |
| 7 | `npm run build:site` creates `dist/site`. | — |
| 7 | Publish `dist/site/` as the static site. | — |
| 7 | The factory owns deployment and registry publishing. | — |
| 1 | License | — |
| 4 | MIT — see LICENSE. | — |

## Demo, sandbox, and claims

- The first click from the live landing opens `/demo/`. At 390 px the persistent
  banner, headline, realistic actor/scope/command, and **RECEIPT VERIFIED / 2
  linked events** panel are visible without scrolling. No horizontal overflow
  occurred at either viewport.
- The banner reads **“Demo — sample data, separate demo storage”** and offers
  **Reset demo** and **Start for real**. In an independent live check, resetting
  removed `demo:ar_theme` while preserving a pre-existing `ar_real_marker`.
- A fresh live request log for the demo contained only the product origin. The
  page raised no console or page errors. The clean browser context had no
  pre-existing storage.
- The release `action-receipts demo` command was run from a new temporary
  caller directory. It created a newly named OS temporary directory with
  `docs-deployment.receipt.json` and `docs-deployment.receipt.html`; the caller
  directory remained empty.
- From a fresh clone, after `npm ci --include=dev` and `npm run build`, all 16
  exact commands in `.factory/claims.json` passed: 8 Rust/CLI claims and 8
  Playwright claims (each desktop and 390 px). `npm test` and the complete
  `npm run test:e2e` also passed (18 browser tests). The one prerequisite build
  is necessary because Playwright serves `dist/site`.

## History check

Every earlier review finding was rechecked on the live site and in source.
The previously reopened demo, terminal recording, route metadata fields,
route-announcement, shared chrome, mobile layout, first-screen wording,
download, and no-third-party-demo-request repairs are present and working.
No earlier finding is reopened. The social-image dimension and claim-location
coverage findings above are newly identified gaps rather than regressions.

## Structure and routing

Home, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, robots, sitemap, and the
Linux download returned 200; an unknown route returned the designed 404 with
HTTP 404. Crawled internal links and the GitHub source link returned 200. Each
checked route has one `h1`, one `main`, title, description, canonical, favicon,
apple touch icon, manifest, OG/Twitter fields, consistent header/footer,
skip link, and polite route-announcement region. The full local Playwright+Axe
run found zero violations. The visual system is product-specific
chain-of-custody neo-brutalism rather than a generic SaaS template.

## Missed leverage

No AI feature is expected here: model interpretation would weaken the clear,
local evidence boundary. JSON and self-contained HTML already supply the
obvious export path. No provider keys are present.

## What would make this perfect

Ship the correctly sized original social card and make the three remaining
published capability statements traceable to exact, clean-sandbox claim tests.
Then rerun the same cold live check and full claim matrix.
