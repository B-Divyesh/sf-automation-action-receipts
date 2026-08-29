# Adversarial first-read review 6 — PASS

Reviewed 2026-08-29 against `https://automation-action-receipts.sociobot.in`,
commit `017d8ffee572bd6adb42422ec51509dd0d56b8ef`.

## Verdict

**PASS.** Zero blocking findings, zero minor findings, zero unlisted claims,
and zero untested claims remain.

## Cold first read

Fresh Chromium contexts at 390 × 844 and 1440 × 900 showed this before
scrolling:

- **What it does:** records and verifies automated changes as signed receipts.
- **For whom:** teams using agents, scripts, or CI to change repositories and
  services.
- **What to click first:** **Try it with sample data**; it opens a signed
  deployment receipt in the demo.

The exact first-screen copy, “Record and verify automated changes.” and “For
teams using agents, scripts, or CI to change repositories and services.”,
states the job and audience. The primary action is visible and visually
dominant at both sizes. The second action, **Install the CLI**, is correctly
secondary. The 390 px layout has no horizontal overflow or split words.

## Copy audit

Counts are whitespace-delimited. Commands and version identifiers are not
prose sentences. No entry is over 22 words. I found no banned marketing term,
empty slogan, inconsistent core term, unexplained metaphor, or non-result-
naming action button. `CLI`, `CI`, `JSON`, `HTML`, and `file hash` are
necessary technical terms for the named audience and are contextualized.

### Landing page

| Words | Copy | Result |
| ---: | --- | --- |
| 5 | Record and verify automated changes. | Clear job headline. |
| 12 | For teams using agents, scripts, or CI to change repositories and services. | Clear audience. |
| 5 | Try it with sample data | Result-naming action. |
| 3 | Install the CLI | Result-naming action. |
| 5 | See a signed deployment receipt. | Clear result. |
| 7 | The demo uses separate sample storage. | `demo-isolated` |
| 4 | Creates local receipt files | `cli-demo-lifecycle` |
| 5 | CLI demo needs no account | `cli-no-account` |
| 5 | No third-party demo requests | `no-third-party-demo-requests` |
| 7 | Stated approval → recorded command → file hash → signature | Concrete sequence. |
| 2 | Approved scope | Clear label. |
| 2 | Recorded command | Clear label. |
| 2 | File hash | Clear label. |
| 1 | Signature | Clear label. |
| 2 | Offline verifier | Section label. |
| 5 | Verify a signed receipt. | Clear heading. |
| 13 | The verifier checks every recorded event and the receipt signature on your device. | `browser-verification` |
| 4 | Choose a receipt JSON | Clear file action. |
| 7 | or drop it here · maximum 2 MB | `two-mb-limit` |
| 3 | or paste JSON | Clear alternative. |
| 2 | Receipt JSON | Bound input label. |
| 2 | Verify receipt | Result-naming action. |
| 5 | Try it with sample data | Result-naming action. |
| 1 | Unchecked | Status. |
| 3 | No receipt loaded | Clear empty state. |
| 9 | Choose a receipt JSON or try the sample. | Clear next step. |
| 7 | Selected files stay in this browser tab. | `receipt-never-uploaded` |
| 3 | How it works | Section label. |
| 5 | Create, record, sign, and verify. | Concrete process. |
| 4 | Record the approved scope. | Concrete step. |
| 6 | Start by recording the approval and scope. | `declared-boundary-fields` |
| 6 | Record a command or tool result. | Concrete step. |
| 7 | Keep the command, result, and file hash together. | `command-provenance` |
| 5 | Sign and export the receipt. | Concrete step. |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 4 | Integrity is not identity. | Useful limitation. |
| 9 | A valid receipt shows signed contents have not changed. | `browser-verification` |
| 13 | It does not prove approval was legitimate or an action was correct. | Accurate limitation. |
| 3 | Command-line quick start | Section label. |
| 8 | Run the sample or use your own change. | Concrete heading. |
| 9 | Captured from the released v0.1.0 binary running action-receipts demo. | `terminal-recording` |
| 2 | Copy commands | Result-naming action. |
| 3 | Download Linux x64 | `linux-download` |
| 3 | Read the source (GitHub) | Explicit external destination. |
| 2 | Export formats | Section label. |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 3 | Install the CLI | Correct destination. |
| 7 | Record and verify automated repository and service changes. | Specific footer description. |
| 5 | Built by Param Factory · v0.1.0 | Attribution/build ID. |

The transient demo inventory also passes: “Demo — sample data, separate demo
storage” (6), “Reset demo” (2), “Start for real” (3), “Verify an automated
change.” (4), “A signed documentation deployment is already loaded below.”
(9), “The sample loads automatically and stays in this browser tab.” (9), and
“Use action-receipts demo to create JSON and HTML in a new temporary
directory.” (12). The first six describe the demonstrated sandbox and map to
`demo-isolated`; the last maps to `cli-demo-lifecycle`.

### README

| Words | Copy | Result |
| ---: | --- | --- |
| 9 | Record and verify automated changes in a local receipt file. | Clear job. |
| 14 | It is for teams using agents, scripts, or CI to change repositories and services. | Clear audience. |
| 8 | Each event links to the event before it. | `browser-verification` |
| 5 | A signature detects later changes. | `browser-verification` |
| 10 | The browser verifier and CLI verify receipts without a server. | `local-verification` |
| 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | Accurate limitation. |
| 9 | Build the binary from this checkout, then inspect the available commands. | Direct instruction. |
| 6 | Run this command from any directory. | Direct instruction. |
| 18 | It creates signed JSON and HTML receipts in a new temporary directory and prints both paths. | `cli-demo-lifecycle` |
| 12 | Open `/demo/`, or `/?demo=1`, for the isolated browser sample. | `demo-isolated` |
| 11 | It loads a signed documentation deployment receipt using separate `demo:` storage. | `demo-isolated` |
| 9 | New receipts use a separate private signing key. | `private-key-permissions` |
| 13 | A command receipt includes its arguments, result, duration, exit status, and declared file hashes. | `command-provenance` |
| 10 | Literal and default-key secrets are redacted before receipt data is stored. | `redact-before-storage` |
| 10 | The CLI exports signed JSON and a self-contained HTML report. | `json-html-export` |
| 8 | The verifier rejects receipt JSON with unknown fields. | `unknown-fields-rejected` |
| 11 | The browser processes selected receipt text without a data request. | `receipt-never-uploaded` |
| 10 | After one visit, the demo can reload offline. | `offline-reload` |
| 7 | The demo makes no third-party requests. | `no-third-party-demo-requests` |
| 10 | See the product privacy and terms pages before using sensitive receipt data. | Direct safety instruction. |
| 7 | `npm run build:site` creates `dist/site`. | `site-build-output` |
| 7 | Publish `dist/site/` as the static site. | Direct instruction. |
| 7 | The factory owns deployment and registry publishing. | Clear responsibility. |
| 4 | MIT — see LICENSE. | `mit-license` |

The README headings name their sections. Its code blocks are copy-paste
commands, not prose claims.

## Demo, privacy, and claims

`/demo/` opens in one click to a usable, verified documentation-deployment
receipt at 390 px. Above the fold it shows the persistent sample-storage
banner, **Reset demo**, **Start for real**, verified state, two linked events,
actor, approved scope, and recorded command. It is not an empty form or a
marketing simulation.

Fresh-context demo request logging contained only the product origin: document,
self-hosted JS/CSS, and `sample.receipt.json`; no console error occurred.
Source inspection confirms `demo:` keys, reset removal of only that namespace,
and Start for real removal before Home. The automated sandbox assertion also
checks that real-key sentinels survive. The CLI demo writes JSON and HTML into
a new operating-system temporary directory.

All 20 exact commands in `.factory/claims.json` were run from a clean clone at
`/tmp/action-receipts-review6-LNu1M7/repo`; every command passed. This includes
request logging, offline caching, storage isolation, tamper, executable,
build-output, and MIT checks. In that clone `npm test` passed (5 Rust unit,
9 CLI integration, 3 Vitest), and `npm run build` produced `dist/site/`.
Every visitor-reliant landing, demo, README, Privacy, and Terms statement maps
to a listed claim or is a limitation/instruction rather than a capability
promise.

## Structure, routing, accessibility, and leverage

Home, Demo, Privacy, Terms, 404, and an unknown live URL each have the correct
route title, one h1, main landmark, description, canonical, OG/Twitter
metadata, favicon, and 180 px Apple icon. The designed unknown route returns
HTTP 404 and has a home action. `robots.txt` and `sitemap.xml` list public
routes. Internal destinations, download, and the labelled GitHub source link
all returned successfully.

The shared header/footer, skip link, mobile menu, route focus/announcement,
and legal links are consistent. The paper, ink, vermilion, chartreuse, hard
rules, offset shadows, and custody-card art match the documented
chain-of-custody identity; this is not a generic SaaS template. The brief does
not imply an AI step, import, export beyond existing JSON/HTML export, or sync,
so no obvious leverage is missing.

## Earlier-finding regression check

Every earlier ID was rechecked on the live site and source rather than accepted
from its status label. All are fixed:

| Earlier IDs | Confirmation |
| --- | --- |
| F-1-1, F-1-11–F-1-12, F-1-31 | Job, audience, primary sample action, and immediate result are visible at both viewport sizes. |
| F-1-2, F-2-1–F-2-5, F-4-2–F-4-3 | Direct isolated browser/CLI demos, reset/exit, responsive proof, temp files, and release-derived static capture work. |
| F-1-3, F-1-44–F-1-54, F-1-61–F-1-70, F-1-72–F-1-76, F-1-78–F-1-79, F-2-3–F-2-11, F-3-2–F-3-4, F-5-1 | Manifest, observable tagged tests, privacy/offline coverage, and demo h1→h2 outline pass. |
| F-1-4, F-1-23–F-1-26, F-1-55–F-1-60, F-1-71, F-1-77, F-1-80 | Unsupported paid, checkout, price, merchant, future, license-storage, and refund wording remains absent. |
| F-1-5, F-1-7–F-1-10, F-2-12, F-3-1, F-4-4 | Product 404, metadata/social art/Apple icon, shared chrome, focus, landmarks, and keyboard mobile navigation work. |
| F-1-13–F-1-22, F-1-27–F-1-43, F-2-13, F-4-1, F-4-5 | Landing/README/Terms copy and layout use short task language with no headline split or future-maintenance promise. |
| F-1-56, F-1-62, F-1-74 | MIT, Linux download, and build-output claims are listed and have passing focused tests. |

No previously fixed item is half-fixed or regressed.

## What would make this perfect

Nothing actionable remains under this review contract. Preserve the existing
claim-to-test mapping when changing copy or demo behavior, then repeat this
cold-context review.
