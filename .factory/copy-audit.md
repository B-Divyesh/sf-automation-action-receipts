# Copy audit — 2026-08-29

All visitor-facing sentences below are 22 words or fewer. No banned marketing
terms appear. Commands, labels, version numbers, and URLs are excluded.

## Landing and demo

| Words | Sentence |
| ---: | --- |
| 5 | Record and verify automated changes. |
| 12 | For teams using agents, scripts, or CI to change repositories and services. |
| 5 | See a signed deployment receipt. |
| 7 | The demo uses separate sample storage. |
| 6 | Start by recording the approval and scope. |
| 9 | Keep the command, result, and file hash together. |
| 8 | Export JSON or a self-contained HTML report. |
| 9 | A valid receipt shows signed contents have not changed. |
| 13 | It does not prove approval was legitimate or an action was correct. |
| 8 | A signed documentation deployment is already loaded below. |
| 6 | Demo — sample data, separate demo storage. |

## README

| Words | Sentence |
| ---: | --- |
| 9 | Record and verify automated changes in a local receipt file. |
| 14 | It is for teams using agents, scripts, or CI to change repositories and services. |
| 8 | Each event links to the event before it. |
| 5 | A signature detects later changes. |
| 10 | The browser verifier and CLI verify receipts without a server. |
| 14 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. |

## Complete landing inventory

This inventory includes headings, controls, and short labels so copy changes do
not hide between sentence checks. All rows are at or below 22 words. No banned
word or inconsistent term is present.

| Words | Text | Result |
| ---: | --- | --- |
| 3 | Skip to content | — |
| 2 | Action Receipts | — |
| 1 | Demo | — |
| 1 | Verify | — |
| 3 | How it works | — |
| 1 | Privacy | — |
| 2 | Open menu | — |
| 2 | Close menu | — |
| 3 | Use dark theme | — |
| 2 | Version 0.1.0 | — |
| 5 | Record and verify automated changes. | — |
| 12 | For teams using agents, scripts, or CI to change repositories and services. | — |
| 5 | Try it with sample data | — |
| 3 | Install the CLI | — |
| 5 | See a signed deployment receipt. | — |
| 6 | The demo uses separate sample storage. | — |
| 4 | Creates local receipt files | `cli-demo-lifecycle` |
| 5 | CLI demo needs no account | `cli-no-account` |
| 4 | No third-party demo requests | `no-third-party-demo-requests` |
| 7 | Stated approval → recorded command → file hash → signature | — |
| 2 | Approved scope | — |
| 2 | Recorded command | — |
| 2 | File hash | — |
| 1 | Signature | — |
| 2 | Offline verifier | — |
| 5 | Verify a signed receipt. | — |
| 13 | The verifier checks every recorded event and the receipt signature on your device. | `browser-verification` |
| 4 | Choose a receipt JSON | — |
| 7 | or drop it here · maximum 2 MB | `two-mb-limit` |
| 3 | or paste JSON | — |
| 2 | Receipt JSON | — |
| 2 | Verify receipt | — |
| 5 | Try it with sample data | `demo-isolated` |
| 1 | Unchecked | — |
| 3 | No receipt loaded | — |
| 9 | Choose a receipt JSON or try the sample. | — |
| 7 | Selected files stay in this browser tab. | `receipt-never-uploaded` |
| 3 | How it works | — |
| 5 | Create, record, sign, and verify. | — |
| 4 | Record the approved scope. | — |
| 6 | Start by recording the approval and scope. | `declared-boundary-fields` |
| 6 | Record a command or tool result. | — |
| 7 | Keep the command, result, and file hash together. | `command-provenance` |
| 5 | Sign and export the receipt. | — |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 4 | Integrity is not identity. | — |
| 9 | A valid receipt shows signed contents have not changed. | `browser-verification` |
| 13 | It does not prove approval was legitimate or an action was correct. | — |
| 3 | Command-line quick start | — |
| 8 | Run the sample or use your own change. | — |
| 9 | Captured from the released v0.1.0 binary running action-receipts demo. | `terminal-recording` |
| 2 | Copy commands | — |
| 3 | Download Linux x64 | `linux-download` |
| 3 | Read the source (GitHub) | — |
| 2 | Export formats | — |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 3 | Install the CLI | — |
| 3 | AR/ Action Receipts | — |
| 7 | Record and verify automated repository and service changes. | — |
| 1 | Terms | — |
| 2 | Source (GitHub) | — |
| 5 | Built by Param Factory · v0.1.0 | — |

## Complete README inventory

| Words | Text | Result |
| ---: | --- | --- |
| 2 | Action Receipts | — |
| 9 | Record and verify automated changes in a local receipt file. | — |
| 14 | It is for teams using agents, scripts, or CI to change repositories and services. | — |
| 8 | Each event links to the event before it. | `browser-verification` |
| 5 | A signature detects later changes. | `browser-verification` |
| 10 | The browser verifier and CLI verify receipts without a server. | `local-verification` |
| 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | — |
| 1 | Install | — |
| 9 | Build the binary from this checkout, then inspect the available commands. | — |
| 2 | Try the demo | — |
| 6 | Run this command from any directory. | — |
| 18 | It creates signed JSON and HTML receipts in a new temporary directory and prints both paths. | `cli-demo-lifecycle` |
| 12 | Open `/demo/`, or `/?demo=1`, for the isolated browser sample. | `demo-isolated` |
| 11 | It loads a signed documentation deployment receipt using separate `demo:` storage. | `demo-isolated` |
| 4 | Use your own change | — |
| 9 | New receipts use a separate private signing key. | `private-key-permissions` |
| 13 | A command receipt includes its arguments, result, duration, exit status, and declared file hashes. | `command-provenance` |
| 10 | Literal and default-key secrets are redacted before receipt data is stored. | `redact-before-storage` |
| 10 | The CLI exports signed JSON and a self-contained HTML report. | `json-html-export` |
| 8 | The verifier rejects receipt JSON with unknown fields. | `unknown-fields-rejected` |
| 4 | Browser verifier and privacy | — |
| 11 | The browser processes selected receipt text without a data request. | `receipt-never-uploaded` |
| 10 | After one visit, the demo can reload offline. | `offline-reload` |
| 7 | The demo makes no third-party requests. | `no-third-party-demo-requests` |
| 10 | See the product privacy and terms pages before using sensitive receipt data. | — |
| 5 | Test, package, and deploy | — |
| 7 | `npm run build:site` creates `dist/site`. | `site-build-output` |
| 7 | Publish `dist/site/` as the static site. | — |
| 7 | The factory owns deployment and registry publishing. | — |
| 1 | License | — |
| 4 | MIT — see LICENSE. | `mit-license` |

## Terminology

| Concept | Term used |
| --- | --- |
| Signed record | receipt |
| Individual recorded action | event |
| Browser sample | demo |
| File created by the CLI | receipt file |
| Input limits | 2 MB browser limit |

## Demo controls

| Words | Text | Result |
| ---: | --- | --- |
| 6 | Demo — sample data, separate demo storage | `demo-isolated` |
| 2 | Reset demo | `demo-isolated` |
| 3 | Start for real | `demo-isolated` |

## Legal page sentences

All Privacy and Terms sentences are 22 words or fewer. The untestable future
maintenance sentence identified in F-4-5 is removed. The MIT references map
to `mit-license`. Legal pages use the same terms as the landing page:
**receipt**, **event**, and **demo**.
