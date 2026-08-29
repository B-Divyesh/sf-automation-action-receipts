# Adversarial first-read review 5 — FAIL

Reviewed 2026-08-29 at
`https://automation-action-receipts.sociobot.in` from fresh Chromium contexts
at 390 × 844 and 1440 × 900. Repository base:
`c32b4ecf66aaa46c8000857669ada24ffccfe046`.

## Verdict

**FAIL.** Two earlier claim-contract findings remain only partly fixed and are
reopened as blocking under the history rule. One additional accessibility
finding remains. All 19 listed claim commands pass, but a passing listed set
does not cover published claims that are still absent from the manifest.

## First screen, before scrolling

The cold read passes on both viewports.

- **What it does:** records and verifies automated repository or service
  changes as signed receipts.
- **For whom:** teams using agents, scripts, or CI.
- **What to click first:** **Try it with sample data**. The adjacent text says
  the result is a signed deployment receipt in separate sample storage.

Exact supporting text: **“Record and verify automated changes.”**;
**“For teams using agents, scripts, or CI to change repositories and
services.”**; **“Try it with sample data”**; and **“See a signed deployment
receipt. The demo uses separate sample storage.”**

At 390 px, the headline, audience, both actions, result note, and all three
facts finish above y=748 in an 844 px viewport. At 1440 px they finish above
y=727 in a 900 px viewport. There is no horizontal overflow or console error
on either normal landing load.

## Findings

### F-1-56 (reopened) — BLOCKING — the retained MIT license claim still has no claim entry

- **Exact quote/location:** README, **“MIT — see LICENSE.”**; `/terms/`,
  **“Action Receipts is provided under the MIT License. You may use, copy,
  modify, and distribute it under that license.”**
- **Evidence:** `LICENSE` contains the MIT grant and warranty text, so the copy
  is factually supported. However, `.factory/claims.json` has no license claim
  and no listed command checks the file or these published locations. Review 1
  F-1-56 explicitly required the retained MIT statement to be tested while
  removing the unprovable “Forever” promise. Only the latter half was done.
- **Why this fails:** licensing is a statement a visitor may rely on. It cannot
  be exempted from the repository's “every claim is a test” rule merely because
  the underlying file currently looks correct.
- **Concrete fix:** add a `mit-license` manifest entry for README and Terms.
  Its test must verify that `LICENSE` contains the MIT permission and warranty
  clauses and that both published references identify the same license.

### F-1-72 (reopened) — BLOCKING — the landing offline promise is absent from the manifest locations

- **Exact quote/location:** landing verifier eyebrow, **“Offline verifier”**;
  landing and demo offline status, **“Offline mode — local verification still
  works.”**
- **Evidence:** `offline-reload.where` lists only the privacy page and README.
  `local-verification.where` lists only the catalog description and README
  opening. Neither entry lists the landing verifier or offline status bar. The
  corresponding commands pass, and an independent live offline reload did
  show a verified receipt, but the published landing claim remains unlisted at
  its actual location. Review 1 F-1-72 named the offline bar specifically.
- **Why this fails:** a first-time visitor can rely on the landing's offline
  promise without being able to trace that promise to the manifest. This is a
  half-fix of an earlier finding, so the review contract makes it blocking.
- **Concrete fix:** add **landing verifier eyebrow and landing/demo offline
  status bar** to the relevant `where` fields. Extend
  `@claim:local-verification` to assert that the offline bar is visible while a
  valid receipt verifies and a changed receipt fails with networking disabled.

### F-5-1 — MINOR — the demo's initial HTML skips from h1 to h3

- **Exact location:** `site/demo/index.html`, the server-rendered loading panel
  under **“Verify an automated change.”** uses `<h3>Loading receipt</h3>`.
- **Evidence:** the initial document outline is h1 → h3. JavaScript replaces
  that element with an h2 after successful sample verification, so the final
  live DOM and post-load Axe scan pass. A slow request, blocked script, or
  assistive-technology read during loading still receives the skipped level.
- **Why this fails:** the required heading outline is h1 → h2 → h3 without
  skips, including empty and loading states. The current test checks only the
  settled DOM and therefore cannot catch this state.
- **Concrete fix:** change the loading heading to `<h2>Loading receipt</h2>`.
  Add a test that inspects the initial HTML, or blocks the sample response long
  enough to verify the loading-state heading order.

## Copy audit

Counts are visible word tokens; standalone arrows, middle dots, and dashes are
not words. Code blocks are commands and are excluded from sentence counts.
Repeated copy is listed at each distinct landing location. No item exceeds 22
words. No banned marketing adjective, metaphor heading, inconsistent product
term, or non-result-naming button was found. The two flagged claim statements
map to the findings above rather than requiring a wording change.

### Landing page sentences

| Location | Words | Exact text | Result |
| --- | ---: | --- | --- |
| Hero h1 | 5 | Record and verify automated changes. | pass |
| Hero audience | 12 | For teams using agents, scripts, or CI to change repositories and services. | pass |
| Hero action note | 5 | See a signed deployment receipt. | pass |
| Hero action note | 6 | The demo uses separate sample storage. | pass |
| Verifier h2 | 4 | Verify a signed receipt. | pass |
| Verifier introduction | 13 | The verifier checks every recorded event and the receipt signature on your device. | pass |
| Verifier empty-state h3 | 3 | No receipt loaded | pass |
| Verifier empty state | 8 | Choose a receipt JSON or try the sample. | pass |
| Verifier privacy note | 7 | Selected files stay in this browser tab. | pass |
| Process h2 | 5 | Create, record, sign, and verify. | pass |
| Process step 1 h3 | 4 | Record the approved scope. | pass |
| Process step 1 | 7 | Start by recording the approval and scope. | pass |
| Process step 2 h3 | 6 | Record a command or tool result. | pass |
| Process step 2 | 8 | Keep the command, result, and file hash together. | pass |
| Process step 3 h3 | 5 | Sign and export the receipt. | pass |
| Process step 3 | 7 | Export JSON or a self-contained HTML report. | pass |
| Integrity heading | 4 | Integrity is not identity. | pass |
| Integrity note | 9 | A valid receipt shows signed contents have not changed. | pass |
| Integrity note | 12 | It does not prove approval was legitimate or an action was correct. | pass |
| CLI h2 | 8 | Run the sample or use your own change. | pass |
| CLI figure caption | 9 | Captured from the released v0.1.0 binary running action-receipts demo. | pass |
| Closing h2 | 7 | Export JSON or a self-contained HTML report. | pass |
| Footer | 8 | Record and verify automated repository and service changes. | pass |
| Offline status, conditional | 6 | Offline mode — local verification still works. | F-1-72 |
| Valid-result sentence, conditional | 2 | Receipt `{receipt_id}`. | pass |
| Valid-result sentence, conditional | 7 | Every recorded event and the signature match. | pass |
| Invalid-result sentence, conditional | 6 | Do not rely on this receipt. | pass |
| Invalid-result sentence, conditional | 6 | Ask for the original signed file. | pass |
| Result caveat, conditional | 9 | A valid signature proves integrity, not identity or correctness. | pass |
| Size error, conditional | 7 | Receipt exceeds the 2 MB browser limit. | pass |
| Size error, conditional | 4 | Use the CLI verifier. | pass |
| Parse error, conditional | 5 | Could not read this receipt. | pass |
| Offline sample error, conditional | 11 | The sample is unavailable offline until it has been opened once. | pass |
| Sample error, conditional | 5 | Could not load the sample. | pass |

### Landing headings, labels, facts, controls, and image text

| Location | Words | Exact text | Result |
| --- | ---: | --- | --- |
| Skip link | 3 | Skip to content | pass |
| Wordmark | 2 | Action Receipts | pass |
| Navigation | 1 | Demo | pass |
| Navigation | 1 | Verify | pass |
| Navigation | 3 | How it works | pass |
| Navigation | 1 | Privacy | pass |
| Mobile control | 2 | Open menu | pass |
| Mobile control, conditional | 2 | Close menu | pass |
| Theme control | 3 | Use dark theme | pass |
| Theme control, conditional | 3 | Use light theme | pass |
| Hero label | 2 | Version 0.1.0 | pass |
| Primary action | 5 | Try it with sample data | pass |
| Secondary action | 3 | Install the CLI | pass |
| Hero fact | 4 | Creates local receipt files | pass |
| Hero fact | 5 | CLI demo needs no account | pass |
| Hero fact | 4 | No third-party demo requests | pass |
| Hero image alt | 13 | Evidence cards linked by a black chain, ending in a sealed artifact envelope. | pass |
| Figure caption | 7 | Stated approval → recorded command → file hash → signature | pass |
| Chain label | 2 | Approved scope | pass |
| Chain label | 2 | Recorded command | pass |
| Chain label | 2 | File hash | pass |
| Chain label | 1 | Signature | pass |
| Verifier eyebrow | 2 | Offline verifier | F-1-72 |
| File control | 4 | Choose a receipt JSON | pass |
| File limit | 7 | or drop it here · maximum 2 MB | pass |
| Input alternative | 3 | or paste JSON | pass |
| Field label | 2 | Receipt JSON | pass |
| Verifier action | 2 | Verify receipt | pass |
| Verifier sample action | 5 | Try it with sample data | pass |
| Empty-state status | 1 | Unchecked | pass |
| Section eyebrow | 3 | How it works | pass |
| CLI eyebrow | 3 | Command-line quick start | pass |
| Terminal image alt | 16 | Terminal capture of action-receipts demo creating a JSON receipt and HTML report in a temporary directory. | pass |
| Terminal comment | 9 | Run a complete sample in a new temporary directory | pass |
| Terminal comment | 7 | Or create and sign your own receipt | pass |
| Copy control | 2 | Copy commands | pass |
| Copy status, conditional | 2 | Copied commands | pass |
| Download action | 3 | Download Linux x64 | pass |
| External link | 4 | Read the source (GitHub) | pass |
| Closing eyebrow | 2 | Export formats | pass |
| Closing action | 3 | Install the CLI | pass |
| Footer wordmark | 3 | AR/ Action Receipts | pass |
| Footer link | 1 | Privacy | pass |
| Footer link | 1 | Terms | pass |
| Footer external link | 2 | Source (GitHub) | pass |
| Footer build label | 5 | Built by Param Factory · v0.1.0 | pass |
| Valid status, conditional | 2 | Receipt verified | pass |
| Invalid status, conditional | 2 | Verification failed | pass |
| Valid result h3, conditional | 3 | 2 linked events | pass |

### README sentences and headings

| Location | Words | Exact text | Result |
| --- | ---: | --- | --- |
| h1 | 2 | Action Receipts | pass |
| Opening | 10 | Record and verify automated changes in a local receipt file. | pass |
| Opening | 14 | It is for teams using agents, scripts, or CI to change repositories and services. | pass |
| Opening | 8 | Each event links to the event before it. | pass |
| Opening | 5 | A signature detects later changes. | pass |
| Opening | 10 | The browser verifier and CLI verify receipts without a server. | pass |
| Opening | 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | pass |
| h2 | 1 | Install | pass |
| Install | 11 | Build the binary from this checkout, then inspect the available commands. | pass |
| h2 | 3 | Try the demo | pass |
| Demo | 6 | Run this command from any directory. | pass |
| Demo | 16 | It creates signed JSON and HTML receipts in a new temporary directory and prints both paths. | pass |
| Demo | 9 | Open `/demo/`, or `/?demo=1`, for the isolated browser sample. | pass |
| Demo | 11 | It loads a signed documentation deployment receipt using separate `demo:` storage. | pass |
| h2 | 4 | Use your own change | pass |
| Usage | 8 | New receipts use a separate private signing key. | pass |
| Usage | 14 | A command receipt includes its arguments, result, duration, exit status, and declared file hashes. | pass |
| Usage | 11 | Literal and default-key secrets are redacted before receipt data is stored. | pass |
| Usage | 10 | The CLI exports signed JSON and a self-contained HTML report. | pass |
| Usage | 8 | The verifier rejects receipt JSON with unknown fields. | pass |
| h2 | 4 | Browser verifier and privacy | pass |
| Privacy | 10 | The browser processes selected receipt text without a data request. | pass |
| Privacy | 8 | After one visit, the demo can reload offline. | pass |
| Privacy | 6 | The demo makes no third-party requests. | pass |
| Privacy | 12 | See the product privacy and terms pages before using sensitive receipt data. | pass |
| h2 | 4 | Test, package, and deploy | pass |
| Build | 5 | `npm run build:site` creates `dist/site`. | pass |
| Deploy | 6 | Publish `dist/site/` as the static site. | pass |
| Deploy | 7 | The factory owns deployment and registry publishing. | pass |
| h2 | 1 | License | pass |
| License | 3 | MIT — see LICENSE. | F-1-56 |

Terminology is consistent: **receipt** is the signed record, **event** is one
recorded action, **demo** is the sample mode, and **receipt file** is CLI
output. The technical terms CLI, CI, JSON, signature, and file hash are needed
for this developer-facing job and are used consistently.

## Demo and sandbox behavior

- The first click opens `/demo/`. At 390 px, its persistent banner, task
  headline, **Receipt verified**, **2 linked events**, and signed receipt ID are
  visible before scrolling. The sample is an approved documentation deployment
  by `release-bot@ci`, scoped to `repo:docs/**`, with `npm run build` recorded.
- The banner reads **“Demo — sample data, separate demo storage”** and provides
  **Reset demo** and **Start for real**. This wording is accurate: the sample
  remains in the tab, while demo preferences use `demo:` keys.
- An independent live reset removed a seeded `demo:` key, preserved a non-demo
  sentinel, reloaded the initial sample, and returned to **Receipt verified**.
  **Start for real** also removed the demo key, preserved the sentinel, and
  returned Home without a demo banner.
- The full live flow made only same-origin document, script, stylesheet, image,
  and sample-receipt requests. Selecting, changing, and verifying receipt data
  is covered by the passing no-upload claim.
- After an online visit and service-worker control, `/demo/` reloaded with
  status 200 while the context was offline and still verified the sample.
- The live Linux x64 download was run from a fresh caller directory with an
  empty environment except `PATH`. Version 0.1.0 created signed JSON and HTML
  in a new OS temporary directory, left the caller empty, and verified the JSON
  with two linked events.

## Claim execution

Fresh clone: `/tmp/action-receipts-review5-clone.ethzXv/repo` at the stated
base. Setup: `npm ci --include=dev`. Each command below was then run separately.

| Claim | Exact manifest command | Result |
| --- | --- | --- |
| `cli-demo-lifecycle` | `npm run test:cli -- claim_cli_demo_lifecycle_creates_isolated_signed_outputs` | PASS |
| `cli-no-account` | `npm run test:cli -- claim_cli_demo_needs_no_account_or_environment_credentials` | PASS |
| `declared-boundary-fields` | `npm run test:cli -- claim_declared_boundary_fields_are_written_before_events` | PASS |
| `command-provenance` | `npm run test:cli -- claim_command_provenance_records_command_output_and_artifact_hash` | PASS |
| `redact-before-storage` | `npm run test:cli -- claim_redact_before_storage_removes_literal_and_default_key_secrets` | PASS |
| `json-html-export` | `npm run test:cli -- claim_json_html_export_embeds_the_signed_receipt` | PASS |
| `private-key-permissions` | `npm run test:cli -- claim_private_key_is_separate_and_private` | PASS |
| `unknown-fields-rejected` | `npm run test:cli -- claim_unknown_fields_are_rejected` | PASS |
| `demo-isolated` | `npm run test:e2e -- --grep @claim:demo-isolated` | PASS |
| `browser-verification` | `npm run test:e2e -- --grep @claim:browser-verification` | PASS |
| `receipt-never-uploaded` | `npm run test:e2e -- --grep @claim:receipt-never-uploaded` | PASS |
| `two-mb-limit` | `npm run test:e2e -- --grep @claim:two-mb-limit` | PASS |
| `offline-reload` | `npm run test:e2e -- --grep @claim:offline-reload` | PASS |
| `local-verification` | `npm run test:e2e -- --grep @claim:local-verification` | PASS |
| `no-third-party-demo-requests` | `npm run test:e2e -- --grep @claim:no-third-party-demo-requests` | PASS |
| `site-metadata-and-routing` | `npm run test:e2e -- --grep @claim:site-metadata-and-routing` | PASS |
| `terminal-recording` | `npm run test:e2e -- --grep @claim:terminal-recording` | PASS |
| `linux-download` | `npm run test:e2e -- --grep @claim:linux-download` | PASS |
| `site-build-output` | `npm run test:e2e -- --grep @claim:site-build-output` | PASS |

Result: **19/19 listed commands pass.** F-1-56 and F-1-72 concern
published locations omitted by that list, not failures inside the 19 commands.

The broader clean-clone `npm run check` also passes: formatting, Clippy with
warnings denied, TypeScript, 5 Rust unit tests, 9 CLI tests, 2 Vitest tests,
the production build, and 29 passing Playwright tests with one intended
desktop skip. Built initial assets are 10.21 kB JavaScript / 3.95 kB gzip and
14.75 kB CSS / 3.91 kB gzip.

## History audit

Every earlier review, polish document, verification report, and current
handoff was read. The live HTML for Home, Demo, Privacy, Terms, and 404 matches
the clean production build byte-for-byte.

| Earlier findings | Independent result on live site and in code |
| --- | --- |
| F-1-1 | Fixed: the job, audience, and primary sample action are above both folds. |
| F-1-2 | Fixed: direct isolated browser demo, temp-directory CLI demo, and real binary-derived terminal capture all work. |
| F-1-3–F-1-5 | Fixed: claim manifest exists, unavailable payment is absent, and unknown routes return the designed 404 with status 404. |
| F-1-6–F-1-10 | Fixed: skip focus, full metadata, forward/back focus announcements, common chrome, and Axe landmark checks pass. |
| F-1-11–F-1-55 | Fixed or removed: the live/source copy audit, capability tests, privacy flow, and absent paid tier confirm the earlier repairs. |
| F-1-56 | **Reopened above:** “Forever” is gone, but the retained MIT claim still has no manifest test. |
| F-1-57–F-1-71 | Fixed or removed: unlimited/paid claims are absent; download, key, provenance, schema, redaction, and export claims pass. |
| F-1-72 | **Reopened above:** offline behavior works, but the landing/status claim locations are still absent from the manifest. |
| F-1-73–F-1-80 | Fixed or removed: build, network, storage, host, billing, and refund wording is either tested or absent. |
| F-2-1–F-2-13 | Fixed: mobile demo fit, immediate proof, real terminal evidence, route metadata/chrome, precise privacy wording, and plain copy pass. |
| F-3-1–F-3-4 | Fixed: the social image is 1200 × 630 and file/account/no-server claims have passing tests. |
| F-4-1–F-4-5 | Fixed: words do not split, demo exit clears its namespace, terminal motion is absent, mobile navigation works, and the future Terms promise is gone. |

## Structure, accessibility, links, and identity

- Home, Demo, Privacy, Terms, and 404 have the expected route-specific title,
  one h1, one main, `lang=en`, description, canonical, SVG favicon, 180 × 180
  Apple icon, 1200 × 630 OG/Twitter image, manifest, skip link, and polite route
  announcer. F-5-1 is the initial demo-outline exception.
- Header navigation is consistently Demo / Verify / How it works / Privacy.
  Every footer consistently includes the product line, Privacy, Terms, Source
  (GitHub), Param Factory, and v0.1.0.
- Browser Back and Forward focus and announce the destination h1. Fresh deep
  links to `#verify`, `#protocol`, and `#install` reach existing targets. The
  keyboard mobile menu exposes all four routes and returns focus on Escape.
- A crawl of all links on all five documents found 14 unique links. Every
  document, download, GitHub destination, and hash target returned 200 or
  resolved to an existing target.
- Playwright Axe found zero violations on all five routes at desktop and 390 px
  in both light and dark themes. `verify-url.sh` passed Home and Demo with no
  console errors, missing alt text, or unlabeled buttons.
- The hard-rule, warm-paper, vermilion/chartreuse evidence-envelope visual
  system is recognizably product-specific and matches `.factory/design.md`.
  It is not a centered generic SaaS hero or a three-card template.

## Missed leverage

No additional feature finding is warranted. The obvious import path is the
browser JSON file input, and the CLI already exports signed JSON plus
self-contained HTML. Sync would conflict with the local-first brief. Model
interpretation would not strengthen tamper evidence and would blur the clear
trust boundary; no AI feature or provider key is present.

## What would make this perfect

Add and test the MIT license claim, list and assert the landing/demo offline
status locations, and correct the demo's initial h1 → h3 loading outline. Then
rerun every claim command and the full clean-clone check. Nothing else remains
from this review.
