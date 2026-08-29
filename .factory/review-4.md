# Adversarial first-read review 4 — FAIL

Reviewed 2026-08-29 at
`https://automation-action-receipts.sociobot.in` from fresh Chromium contexts
at 390 × 844 and 1440 × 900. Repository base:
`aee7006ffcfec44d405eabd3a81291db90cd8dc5`.

## Verdict

**FAIL.** Five earlier findings are still incomplete and are reopened as
blocking. Five additional findings remain. All 17 listed claim commands pass,
but the manifest still omits two published promises and two passing tests check
only the presence of evidence rather than the promised outcome.

## First screen, before scrolling

The three required questions are answerable on both viewports.

- **What it does:** records and verifies automated changes in a signed receipt.
- **For whom:** teams using agents, scripts, or CI to change repositories and
  services.
- **First click:** **Try it with sample data**. The adjacent sentence says a
  signed deployment receipt will appear and that demo storage is separate.

Exact first-screen text: **“Record and verify automated changes.”**; **“For
teams using agents, scripts, or CI to change repositories and services.”**;
**“Try it with sample data”**; **“See a signed deployment receipt. The demo
uses separate sample storage.”**

This initial comprehension check passes, so it is not a blocking finding. The
rendered headline and desktop fact placement still have the specific defect in
F-4-1.

## Blocking findings

### F-1-2 (reopened) — the claimed CLI recording is a hand-made simulation

- **Location/quote:** landing caption: **“Recorded from the released v0.1.0
  binary running action-receipts demo.”** The SVG itself says **“recording ·
  action-receipts demo · v0.1.0”**.
- **Evidence:** `.factory/design.md` identifies `terminal-demo.svg` as a
  **“hand-made ... terminal recording frame.”** The SVG hard-codes a made-up
  `/tmp/action-receipts-demo-4821` path and adds **“✓ two linked events · signed
  receipt · local temporary directory”**, a line the real command does not
  print. `@claim:terminal-recording` only asserts that the image and caption
  exist; it never runs the binary or compares the recording with its output.
- **Why this fails:** the CLI demo contract requires a recording of the real
  binary doing the job. Calling a hand-authored facsimile “recorded from” the
  released binary is not honest proof for a sceptical visitor.
- **Concrete fix:** capture a real `action-receipts demo` run as a self-hosted
  SVG/asciinema recording, retain the raw capture source, and make the claim
  test run the built binary and compare its stable lines with the recording.

### F-1-7 (reopened) — the required 180 px Apple touch icon is still absent

- **Location/quote:** every route uses
  `<link rel="apple-touch-icon" href="/favicon.svg">`.
- **Evidence:** `/favicon.svg` has a 64 × 64 viewBox; there is no 180 × 180
  Apple touch asset. `@claim:site-metadata-and-routing` checks only that an
  `apple-touch-icon` link exists, not its format or dimensions.
- **Why this fails:** the earlier finding explicitly required a 180 px Apple
  icon. A link to the general SVG favicon is only a partial repair and can
  produce a missing or unsuitable saved-site icon on iOS.
- **Concrete fix:** ship a product-derived 180 × 180 PNG, reference it on every
  route, add it to the manifest/cache, and assert its decoded dimensions in the
  route metadata claim test.

### F-1-8 (reopened) — browser Back does not restore focus or announce Home

- **Location:** navigate Home → Privacy, then use browser Back.
- **Evidence:** forward navigation focuses the Privacy `h1` and announces
  “Privacy”. Back returns to `/`, but `document.activeElement` is `BODY` and
  `#route-announcement` is empty. Source only handles `popstate` in the active
  document; the back/forward-cache restoration path is not handled. The local
  test checks forward navigation only.
- **Why this fails:** keyboard and screen-reader users lose their place on a
  normal history action. This is the same route-announcement requirement as
  the earlier finding, not a new enhancement.
- **Concrete fix:** handle `pageshow`/BFCache restoration or persist a route
  focus marker for both directions. Add tests for Back and Forward that assert
  the new `h1` is focused and announced.

### F-1-62 (reopened) — the Linux download remains outside the claims manifest

- **Location/quote:** landing action **“Download Linux x64”**.
- **Evidence:** the link currently returns 200, but `.factory/claims.json` has
  no `linux-download` entry. No listed claim test downloads the file, checks
  that it is an executable for Linux x64, prints version 0.1.0, or runs its
  demo from a temporary directory.
- **Why this fails:** the earlier finding required an executable download
  claim test. A manually working link does not satisfy the every-claim-is-a-test
  contract.
- **Concrete fix:** add `linux-download` at the landing action and test the
  built download's response, executable format, version, and isolated demo.

### F-1-74 (reopened) — README build output is still an unlisted claim

- **Location/quote:** README: **“`npm run build:site` creates `dist/site`.”**
- **Evidence:** the build does create `dist/site`, but `.factory/claims.json`
  has no `site-build-output` entry. The prior review required the remaining
  build-output sentence to be listed or removed.
- **Why this fails:** the sentence is a concrete release promise. Passing an
  unlisted build during this review does not make it traceable to the required
  clean-sandbox claim test.
- **Concrete fix:** add a `site-build-output` entry and test required routes,
  metadata, assets, service worker, and executable download in `dist/site`; or
  remove the sentence.

## Additional findings

### F-4-1 — the headline breaks a word and pushes desktop facts below the fold

- **Location/quote:** home and demo `h1`: **“automated”** renders as
  **“AUTOMATE” / “D”** at both 390 px and 1440 px. On desktop none of the three
  product facts is visible within the 900 px first screen.
- **Why this fails:** `overflow-wrap: break-word` applies globally while the
  uppercase `h1` is constrained to `10ch`. The split makes the primary message
  look broken and consumes enough height to hide the required fact lines.
- **Concrete fix:** disable mid-word wrapping on headings, widen the heading or
  reduce its desktop size, and add screenshot/geometry assertions that words
  remain intact and all three facts appear above the 900 px fold.

### F-4-2 — “Start for real” leaves demo data behind

- **Location/quote:** demo banner action **“Start for real”**.
- **Evidence:** after choosing dark mode in the demo, local storage contains
  `demo:ar_theme=dark`. Following **Start for real** returns Home but leaves
  that key intact. **Reset demo** correctly removes it and preserves the
  non-demo sentinel `ar_real_marker=keep`.
- **Why this fails:** isolation passes, but the demo contract also says leaving
  demo mode discards demo data unless the user explicitly keeps it.
- **Concrete fix:** clear all `demo:` storage on **Start for real**, then test
  that demo keys are gone and non-demo keys are unchanged.

### F-4-3 — the terminal cursor ignores reduced-motion preferences

- **Location:** `/terminal-demo.svg`, the cursor `<animate>` element.
- **Evidence:** the SVG runs an infinite opacity animation. Because it is
  loaded through `<img>`, the parent page's reduced-motion CSS cannot disable
  the embedded SMIL animation.
- **Why this fails:** the product motion policy says there are no ambient loops
  and reduced motion makes state changes immediate.
- **Concrete fix:** remove the loop or make the cursor static under
  `prefers-reduced-motion: reduce` inside the SVG itself; add a reduced-motion
  browser assertion.

### F-4-4 — the mobile header hides every route link

- **Location:** all routes at 390 px.
- **Evidence:** the visible header contains only the wordmark and theme button;
  its Demo, Verify, How it works, and Privacy links are all hidden by the mobile
  CSS. There is no replacement menu.
- **Why this fails:** the shared-header requirement includes usable navigation
  on mobile, not only consistent hidden markup. Legal/demo routes are reachable
  only through page-specific content or the footer.
- **Concrete fix:** keep two essential links visible or provide an accessible,
  keyboard-operable compact menu with the same destinations.

### F-4-5 — the Terms page makes an unlisted future-maintenance promise

- **Location/quote:** `/terms/`: **“Material changes will be recorded in the
  project history with a new effective date.”**
- **Why this fails:** this is a claim a user could rely on, but it has no
  claims.json entry and cannot be proven for future releases in the current
  sandbox.
- **Concrete fix:** remove the future promise. If useful, replace it with the
  present-tense, testable statement **“The project history records past terms
  changes and effective dates.”** and add a repository fixture test.

## Copy audit

Counts use whitespace-delimited words and exclude standalone punctuation.
Commands are included where they form a reader-facing sentence. No landing or
README sentence exceeds 22 words. No banned marketing adjective, metaphor or
mood heading, inconsistent core term, or non-result-naming landing button was
found. The three flagged published promises are mapped to findings below.

### Landing page

| Words | Text | Result |
| ---: | --- | --- |
| 3 | Skip to content | clear action |
| 2 | Action Receipts | product name |
| 1 | Demo | clear link |
| 1 | Verify | clear link |
| 3 | How it works | clear section link |
| 1 | Privacy | clear link |
| 3 | Use dark theme | result-naming control |
| 2 | Version 0.1.0 | useful version label |
| 5 | Record and verify automated changes. | clear headline; rendering issue F-4-1 |
| 12 | For teams using agents, scripts, or CI to change repositories and services. | clear audience |
| 5 | Try it with sample data | clear primary action |
| 3 | Install the CLI | clear secondary action |
| 5 | See a signed deployment receipt. | clear result |
| 6 | The demo uses separate sample storage. | `demo-isolated` |
| 4 | Creates local receipt files | `cli-demo-lifecycle` |
| 5 | CLI demo needs no account | `cli-no-account` |
| 4 | No third-party demo requests | `no-third-party-demo-requests` |
| 7 | Stated approval → recorded command → file hash → signature | useful sequence |
| 2 | Approved scope | clear label |
| 2 | Recorded command | clear label |
| 2 | File hash | clear label |
| 1 | Signature | clear label |
| 2 | Offline verifier | clear section label |
| 5 | Verify a signed receipt. | clear heading |
| 13 | The verifier checks every recorded event and the receipt signature on your device. | `browser-verification` |
| 4 | Choose a receipt JSON | clear field action |
| 7 | or drop it here · maximum 2 MB | `two-mb-limit` |
| 3 | or paste JSON | clear alternative |
| 2 | Receipt JSON | clear label |
| 2 | Verify receipt | result-naming button |
| 5 | Try it with sample data | `demo-isolated` |
| 1 | Unchecked | clear status |
| 3 | No receipt loaded | clear empty-state heading |
| 8 | Choose a receipt JSON or try the sample. | clear empty-state action |
| 7 | Selected files stay in this browser tab. | `receipt-never-uploaded` |
| 3 | How it works | clear section heading |
| 5 | Create, record, sign, and verify. | clear process heading |
| 4 | Record the approved scope. | clear step heading |
| 6 | Start by recording the approval and scope. | `declared-boundary-fields` |
| 6 | Record a command or tool result. | clear step heading |
| 8 | Keep the command, result, and file hash together. | `command-provenance` |
| 5 | Sign and export the receipt. | clear step heading |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 4 | Integrity is not identity. | useful limitation heading |
| 9 | A valid receipt shows signed contents have not changed. | `browser-verification` |
| 13 | It does not prove approval was legitimate or an action was correct. | useful limitation |
| 3 | Command-line quick start | clear section heading |
| 8 | Run the sample or use your own change. | clear heading |
| 9 | Recorded from the released v0.1.0 binary running action-receipts demo. | F-1-2 |
| 2 | Copy commands | result-naming button |
| 3 | Download Linux x64 | F-1-62 |
| 4 | Read the source (GitHub) | explicit external result |
| 2 | Export formats | clear section heading |
| 7 | Export JSON or a self-contained HTML report. | `json-html-export` |
| 3 | Install the CLI | clear action |
| 3 | AR/ Action Receipts | product mark |
| 7 | Record and verify automated repository and service changes. | product description |
| 1 | Privacy | clear link |
| 1 | Terms | clear link |
| 2 | Source (GitHub) | explicit external link |
| 5 | Built by Param Factory · v0.1.0 | attribution/build label |

### README

| Words | Text | Result |
| ---: | --- | --- |
| 2 | Action Receipts | product name |
| 9 | Record and verify automated changes in a local receipt file. | clear description |
| 14 | It is for teams using agents, scripts, or CI to change repositories and services. | clear audience |
| 8 | Each event links to the event before it. | `browser-verification` |
| 5 | A signature detects later changes. | `browser-verification` |
| 10 | The browser verifier and CLI verify receipts without a server. | `local-verification` |
| 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | useful limitation |
| 1 | Install | clear task heading |
| 9 | Build the binary from this checkout, then inspect the available commands. | clear instruction |
| 3 | Try the demo | clear task heading |
| 6 | Run this command from any directory. | clear instruction |
| 18 | It creates signed JSON and HTML receipts in a new temporary directory and prints both paths. | `cli-demo-lifecycle` |
| 9 | Open `/demo/`, or `/?demo=1`, for the isolated browser sample. | `demo-isolated` |
| 11 | It loads a signed documentation deployment receipt using separate `demo:` storage. | `demo-isolated` |
| 4 | Use your own change | clear task heading |
| 9 | New receipts use a separate private signing key. | `private-key-permissions` |
| 14 | A command receipt includes its arguments, result, duration, exit status, and declared file hashes. | `command-provenance` |
| 11 | Literal and default-key secrets are redacted before receipt data is stored. | `redact-before-storage` |
| 10 | The CLI exports signed JSON and a self-contained HTML report. | `json-html-export` |
| 8 | The verifier rejects receipt JSON with unknown fields. | `unknown-fields-rejected` |
| 4 | Browser verifier and privacy | clear section heading |
| 11 | The browser processes selected receipt text without a data request. | `receipt-never-uploaded` |
| 10 | After one visit, the demo can reload offline. | `offline-reload` |
| 7 | The demo makes no third-party requests. | `no-third-party-demo-requests` |
| 10 | See the product privacy and terms pages before using sensitive receipt data. | clear safety instruction |
| 5 | Test, package, and deploy | clear section heading |
| 5 | `npm run build:site` creates `dist/site`. | F-1-74 |
| 7 | Publish `dist/site/` as the static site. | clear deploy instruction |
| 7 | The factory owns deployment and registry publishing. | process ownership |
| 1 | License | clear section heading |
| 3 | MIT — see LICENSE. | concise license reference |

Terminology is consistent: the signed record is a **receipt**, one recorded
action is an **event**, the browser sample is the **demo**, and CLI outputs are
**receipt files**.

## Demo and sandbox behavior

- One landing click opens `/demo/`. At 390 px the banner, task headline,
  realistic deployment context, **Receipt verified**, and **2 linked events**
  are visible on the first screen. Desktop shows the same product state.
- The persistent banner reads **“Demo — sample data, separate demo storage”**
  and includes **Reset demo** and **Start for real**.
- Reset removes `demo:ar_theme` and preserves `ar_real_marker`. F-4-2 records
  the separate exit-path defect.
- The fresh live demo request log contains only the product origin: document,
  hashed JS/CSS, and `sample.receipt.json`. Selecting, pasting, verifying, and
  tampering with receipt text generate no data request.
- The release CLI demo, run with an empty environment from a fresh caller
  directory, created JSON and HTML under a newly named OS temporary directory.
  The caller directory remained empty.

## Claims execution

Fresh clone: `/tmp/action-receipts-review4.Hxle4x/repo` at the stated base.
Setup: `npm ci --include=dev && npm run build`. Every exact manifest command
was then run independently.

| Claim | Exact command | Result |
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
| `site-metadata-and-routing` | `npm run test:e2e -- --grep @claim:site-metadata-and-routing` | PASS, insufficient for F-1-7 |
| `terminal-recording` | `npm run test:e2e -- --grep @claim:terminal-recording` | PASS, insufficient for F-1-2 |

The broader clean-clone checks also pass: `npm test` (5 Rust unit, 9 CLI,
2 verifier tests) and `npm run test:e2e` (20 desktop/mobile tests). The latter
reports zero Axe violations on all routes. `/opt/fleet/lib/verify-url.sh` passes
for Home and Demo with no console errors.

## History audit

Every earlier review, polish document, verification note, and handoff was read.
Ranges below are inclusive and account for every earlier numbered finding.

| Earlier findings | Independent result |
| --- | --- |
| F-1-1 | Fixed: job, audience, and primary sample action are visible. |
| F-1-2 | **Reopened above:** CLI command exists, but its “recording” is hand-authored. |
| F-1-3–F-1-6 | Fixed: manifest, paid-tier removal, designed 404, and skip focus are present. |
| F-1-7 | **Reopened above:** metadata exists, but the required 180 px Apple icon does not. |
| F-1-8 | **Reopened above:** forward focus works; browser Back does not. |
| F-1-9–F-1-61 | Fixed or removed: shared chrome, axe issue, copy, paid claims, and listed capabilities were rechecked. |
| F-1-62 | **Reopened above:** download works live, but the required manifest test is absent. |
| F-1-63–F-1-73 | Fixed or removed: key, provenance, schema, redaction, offline, and self-test claims were rechecked. |
| F-1-74 | **Reopened above:** the narrowed build-output sentence remains unlisted. |
| F-1-75–F-1-80 | Fixed or removed: broad network, storage, billing, host, and future-kit claims are absent or tested. |
| F-2-1–F-2-13 | Fixed: mobile demo, immediate proof, metadata, route chrome, claim wording, and copy were rechecked. |
| F-3-1–F-3-4 | Fixed: 1200 × 630 social card and three claim mappings/tests are present. |

## Structure, accessibility, links, and identity

- Home, Demo, Privacy, Terms, and 404 have route-specific titles, one `h1`,
  one `main`, descriptions, canonicals, OG/Twitter metadata, favicon, manifest,
  skip link, common footer, and route announcement region. F-1-7 is the Apple
  icon exception.
- An unknown route returns the designed product 404 with HTTP 404. All crawled
  internal documents, hash destinations, the Linux download, and GitHub source
  returned 200 or resolved to an existing target.
- Forward navigation focuses and announces the route. F-1-8 records the
  Back/Forward failure.
- No horizontal overflow or console error occurred at either viewport. Local
  Axe integration found zero violations. F-4-1, F-4-3, and F-4-4 are manual
  responsive/motion/navigation defects that Axe does not detect.
- The chain-of-custody neo-brutalist visual identity is distinct and matches
  `.factory/design.md`; it is not a generic SaaS template. The social image is
  the required 1200 × 630 product-derived asset.
- Production JavaScript is 3.61 kB gzip, well below the static-product budget.

## Missed leverage

No AI feature is warranted. Model interpretation would not strengthen signed
provenance and would blur the local trust boundary. The CLI already provides
the obvious JSON and self-contained HTML export path; sync would contradict the
local-first brief. No provider key is embedded.

## What would make this perfect

Replace the simulated terminal frame with an auditable capture of the real
binary; add and test the 180 px Apple icon; repair Back/Forward focus and route
announcement; list and test the Linux download and site-build promises; stop
mid-word headline wrapping; keep usable mobile navigation; clear demo keys on
exit; respect reduced motion inside the SVG; and remove the untestable future
Terms promise. Then rerun this full checklist from fresh mobile/desktop contexts
and a clean clone. Zero findings, including minor ones, is the pass condition.
