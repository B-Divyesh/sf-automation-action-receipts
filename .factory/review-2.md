# Adversarial first-read review 2 — FAIL

Reviewed 2026-08-29 against `https://automation-action-receipts.sociobot.in`, from fresh Chromium contexts at 390 × 844 and 1440 × 900. Repository base: `10c8498011cd1b61eab608b188d8882f2db7b702`.

## First screen, before scrolling

On both sizes, the first screen answers the three initial questions.

- **What it does:** records and verifies automated repository or service changes as signed receipts.
- **For whom:** teams using agents, scripts, or CI.
- **First click:** **Try it with sample data**; the adjacent note says a signed deployment receipt will appear.

The landing page is visually distinct and readable at 390 px. The primary action is visible at y=541–592 in the fresh mobile context. This does not rescue the demo screen described below.

## Verdict

**FAIL.** There are 7 blocking findings and 10 additional findings. One command in `.factory/claims.json` fails from a clean clone, the required demo does not show the working product on its first screen, and several previously closed structure findings are still incomplete.

## Blocking findings

### F-2-1 — BLOCKING — the isolated-demo claim fails and the 390 px demo is horizontally broken

- **Location/quote:** `/demo/`: **“Demo — sample data, nothing is saved”**; **“Use dark theme”**. Claim command: `npm run test:e2e -- --grep @claim:demo-isolated`.
- **Evidence:** From a clean clone after `npm ci --include=dev` and `npm run build`, the exact claim command passed on desktop but timed out on the mobile project. Playwright records: `Action Receipts home intercepts pointer events` while trying to click **Use dark theme**. The live 390 px demo has `scrollWidth` 473 versus `clientWidth` 390. The banner is inserted as a child of the flex header: banner `[14,0,267,167]`, header `[14,0,362,169]`, brand `[305,56,67,54]`, and theme button `[396,36,76,94]`; the control is outside the viewport. The live mobile screenshot clips the left side of the demo content.
- **Why this fails:** This is both a touch-target/layout failure and a failing listed claim test. The test is the only evidence that Reset uses the demo namespace, so a failure leaves that claim unverified.
- **Concrete fix:** render the persistent demo banner above, rather than inside, `.site-header`; make it wrap at 390 px; retain a 44 px visible theme control or remove it from the compact demo header. Add a 390 px assertion for `scrollWidth === clientWidth`, successful pointer activation of every visible header control, and reset of only `demo:` keys.

### F-2-2 — BLOCKING — the one-click demo does not show the product in use before scrolling

- **Location/quote:** `/demo/`: **“Verify an automated change.”** and **“This sample records an approved documentation deployment with two linked events.”**
- **Evidence:** Although the receipt loads in the background, the first visible proof panel is below the fold: `#verification-result` starts at y=1737 on the 390 × 844 context and y=1098 on desktop 1440 × 900. The first screen is a marketing hero and illustration, not a visible verified sample.
- **Why this fails:** The demo contract requires the first screen after one click to already look like the product being used with realistic sample data. A sceptical visitor cannot confirm that the button did anything without scrolling well past the hero.
- **Concrete fix:** put the loaded receipt's **Receipt verified / 2 linked events** panel above the fold on `/demo/`; on mobile, make it the first content after the banner. Keep the sample's actor, approved scope, command, and tamper action visible there.

### F-1-2 (reopened) — BLOCKING — the required CLI terminal recording is still absent

- **History check:** Review 1 required a self-hosted recording of the real `action-receipts demo` command, not static terminal copy. The current home terminal is still a static `<pre>` beginning **“# Run a complete sample in a new temporary directory”**.
- **Evidence:** `action-receipts demo` itself works: run from a new unrelated temporary directory it printed a different OS temporary directory containing `docs-deployment.receipt.json` and `docs-deployment.receipt.html`, and left the caller directory empty. However, no recording/SVG/asciinema asset of that command is on the landing page.
- **Why this fails:** The CLI class demo contract asks a visitor to see the real binary perform the core job before installing it. Text that looks like a terminal is not evidence that the shown command ran.
- **Concrete fix:** add a self-hosted, captioned terminal recording generated from the released binary running `action-receipts demo`, showing the created JSON and HTML paths. Keep the command and sample inputs in the repository.

### F-1-7 (reopened) — BLOCKING — route metadata is not complete on every route

- **History check:** Review 1 required complete canonical, Open Graph, Twitter, favicon/apple-touch, and manifest metadata on every route.
- **Evidence:** Live `/privacy/` and `/terms/` have no manifest link. Live `/404.html` has no Open Graph tags, Twitter tags, apple-touch icon, or manifest. Home/demo/legal pages contain only `twitter:card`; they omit Twitter title, description, and image. The 404 page therefore cannot produce the required route-specific share card.
- **Why this fails:** The previous closure said metadata was complete, but it is not complete route by route.
- **Concrete fix:** add route-specific `og:title`, `og:description`, `og:image`, `twitter:card`, `twitter:title`, `twitter:description`, `twitter:image`, apple-touch, and manifest metadata to every shipped HTML document, including 404. Add an automated per-route metadata test.

### F-1-8 (reopened) — BLOCKING — route changes still do not announce the new page

- **History check:** The previous finding required focus *and* a polite announcement after navigation.
- **Evidence:** `site/src/main.ts` calls `byId('route-announcement')`, but only home contains that element. `/privacy/`, `/terms/`, and `/404.html` contain no `[aria-live="polite"]`; `/demo/` has only the verifier's result status, not a route announcement. Navigation does focus the new h1, but there is nothing to announce the page name on those routes.
- **Why this fails:** A screen-reader visitor receives no explicit route-change announcement on most real URLs. The current test covers focus only.
- **Concrete fix:** include one shared `id="route-announcement" aria-live="polite"` element in every route; set it to the new h1 on all internal navigation and on back/forward. Test focus and the announced text on home, demo, Privacy, Terms, and 404.

### F-1-9 (reopened) — BLOCKING — header/footer are not one consistent shared skeleton

- **History check:** The previous finding required a consistent header/footer on every route.
- **Evidence:** Home navigation is **Demo / Verify / How it works / Privacy**. Demo and 404 are **Demo / Privacy / Terms**. Privacy and Terms are **Demo / Verify / Privacy / Terms**. Demo and 404 footers omit the external source link that home/legal footers have. Their `nav` accessible names also differ (**“Legal and project links”** versus **“Legal links”**).
- **Why this fails:** Visitors cannot learn a stable navigation pattern, and the earlier shared-header/footer closure is not borne out by the deployed markup.
- **Concrete fix:** render one header/footer component on every route. Use the same four-or-fewer navigation destinations (for example Demo, Verify, How it works, Privacy), retain Terms and the external source link in every footer, and keep landmark labels identical.

### F-2-3 — BLOCKING — the landing’s “no telemetry” claim is not proven by its listed test

- **Location/quote:** home fact: **“No telemetry”**. Manifest entry `no-account-and-no-telemetry` claims the demo makes no third-party requests.
- **Evidence:** The test records requests and only asserts that each request has the local test-server origin. It cannot detect telemetry sent to the product's own origin, and it does not exercise the landing page where the fact appears. The fresh live demo request log did contain only same-origin requests, but that is not proof of the stronger no-telemetry statement.
- **Why this fails:** A privacy statement needs a test that proves the actual promise, not a weaker proxy.
- **Concrete fix:** either change the fact to **“No third-party requests in the demo”** and retain a complete request-log test, or add a product-wide network contract that permits only the document, static assets, service worker, and explicitly named sample fetches on every route and demo action.

## Additional findings

### F-2-4 — unlisted landing claim: “Nothing is saved”

- **Location/quote:** home action note: **“Nothing is saved.”** (3 words).
- **Why this fails:** The relevant manifest entry names only the `/demo/` banner. Its test changes a demo theme key, so it does not establish that the landing action's route writes nothing outside `demo:`.
- **Concrete fix:** add this exact location to `demo-isolated`, then test before/after localStorage and IndexedDB/OPFS namespaces while entering, using, and resetting the demo. Alternatively rewrite it as **“The demo uses separate sample storage.”**

### F-2-5 — unlisted landing claim: “Local files”

- **Location/quote:** home product fact: **“Local files”** (2 words).
- **Why this fails:** It is too vague to be useful and has no claim entry or observable test.
- **Concrete fix:** replace it with **“Creates receipt files on this device.”** and add a CLI demo test that asserts the output paths are newly created local files.

### F-2-6 — unlisted capability claim: “Start with the actor, approval reference, scope, redaction rules, and retention window.”

- **Location/quote:** home, How it works step 1 (11 words).
- **Why this fails:** The manifest has no declared-boundary-fields claim; the browser sample test does not assert these fields or their order.
- **Concrete fix:** add a `declared-boundary-fields` claim test against the CLI demo output, or rewrite **“Start by recording the approval and scope.”** and test that narrower promise.

### F-2-7 — unlisted capability claim: “Capture command output, tool data, and file hashes.”

- **Location/quote:** home, How it works step 2 (8 words).
- **Why this fails:** No claim test checks all three recorded outcomes.
- **Concrete fix:** add a `command-provenance` claim that runs a fixture command and asserts its output, structured tool data, and artifact hash in the sealed receipt.

### F-2-8 — unlisted capability claim: “Sensitive values are redacted first.”

- **Location/quote:** home, How it works step 2 (5 words).
- **Why this fails:** The statement says redaction occurs before persistence, a security property not listed in `claims.json`.
- **Concrete fix:** add a `redact-before-storage` fixture test that checks open and sealed files for supplied literal, environment, and default-key secrets.

### F-2-9 — unlisted capability claim: “Export JSON or a self-contained HTML report.”

- **Location/quote:** home, How it works step 3 (7 words), and closing headline **“Export each receipt as JSON or HTML.”** (7 words).
- **Why this fails:** The CLI demo manifest claim says it creates JSON and HTML, but it is not listed at either landing location and does not prove the HTML is self-contained.
- **Concrete fix:** add both locations to a `json-html-export` claim; open the report offline and assert its embedded receipt matches the JSON. Use the same wording in both locations.

### F-2-10 — unlisted README capability claims

- **Location/quote:** README: **“`new` writes a separate signing key with mode 0600.”** (9 words); **“`record` adds a tool result without running a process.”** (9 words); **“`run` records command arguments, working directory, redacted output, exit status, duration, and file hashes.”** (13 words); **“Redaction happens before receipt data is written.”** (7 words); **“Unknown fields are rejected.”** (4 words).
- **Why this fails:** Each is a concrete technical promise with no claims entry or named clean-sandbox test.
- **Concrete fix:** add separate entries/tests for private-key permissions, structured record/no child process, command provenance, redaction-before-storage, and unknown-field rejection; otherwise remove or qualify the unsupported statements.

### F-2-11 — unlisted README privacy/offline claims

- **Location/quote:** README: **“Verification works without a server.”** (5 words); **“The browser verifier checks receipt JSON in memory.”** (8 words); **“Selected files are not uploaded.”** (5 words); **“After one visit, its sample can reload offline.”** (8 words); **“The CLI does not need an account.”** (7 words).
- **Why this fails:** The current entries cover a demo request log and demo offline reload, but do not map these locations or prove the CLI account statement or all listed local-processing wording.
- **Concrete fix:** list each exact location under a narrowly named claim and test CLI no-network/no-auth, browser selected-file request behavior, and offline reload from the documented demo URL.

### F-2-12 — non-useful technical eyebrow

- **Location/quote:** home eyebrow: **“Open receipt format · v0.1.0”** (4 words).
- **Why this fails:** It neither names a page section nor tells a first-time visitor what they can do. “Open” is unexplained and the statement has no testable meaning for a visitor.
- **Concrete fix:** remove it, or use **“Version 0.1.0”** as a low-priority build label.

### F-2-13 — jargon-only sentence

- **Location/quote:** home, How it works step 3: **“Technical details use Ed25519.”** (4 words).
- **Why this fails:** It gives an algorithm without a user outcome and fails the first-read plain-language rule.
- **Concrete fix:** delete it from the landing page. Keep the algorithm in implementer documentation after the plain statement about detecting changed receipts.

## Copy audit

Word counts use whitespace-delimited words. Code examples are excluded as commands rather than sentences. No audited landing or README sentence is over 22 words.

### Landing page

| Ref | Words | Copy | Result |
| --- | ---: | --- | --- |
| L1 | 3 | Action Receipts | label |
| L2 | 1 | Demo | link |
| L3 | 1 | Verify | link |
| L4 | 3 | How it works | link/section label |
| L5 | 1 | Privacy | link |
| L6 | 3 | Use dark theme | result-naming control |
| L7 | 4 | Open receipt format · v0.1.0 | F-2-12 |
| L8 | 5 | Record and verify automated changes. | clear headline |
| L9 | 12 | For teams using agents, scripts, or CI to change repositories and services. | clear audience |
| L10 | 5 | Try it with sample data | clear primary action |
| L11 | 3 | Install the CLI | clear secondary action |
| L12 | 5 | See a signed deployment receipt. | demo outcome |
| L13 | 3 | Nothing is saved. | F-2-4 |
| L14 | 2 | Local files | F-2-5 |
| L15 | 2 | No account | mapped to existing account test |
| L16 | 2 | No telemetry | F-2-3 |
| L17 | 7 | Stated approval → recorded command → file hash → signature | explanatory caption |
| L18 | 2 | Approved scope | label |
| L19 | 2 | Recorded command | label |
| L20 | 2 | File hash | label |
| L21 | 1 | Signature | label |
| L22 | 2 | Offline verifier | section label |
| L23 | 4 | Verify a signed receipt. | clear heading |
| L24 | 13 | The verifier checks every recorded event and the receipt signature on your device. | mapped to browser-verification |
| L25 | 4 | Choose a receipt JSON | field label |
| L26 | 7 | or drop it here · maximum 2 MB | mapped to two-mb-limit |
| L27 | 3 | or paste JSON | field label |
| L28 | 2 | Receipt JSON | field label |
| L29 | 2 | Verify receipt | result-naming control |
| L30 | 5 | Try it with sample data | clear control |
| L31 | 1 | Unchecked | status |
| L32 | 3 | No receipt loaded | empty-state heading |
| L33 | 9 | Choose a receipt JSON or try the sample. | clear empty-state instruction |
| L34 | 7 | Selected files stay in this browser tab. | mapped to receipt-never-uploaded |
| L35 | 3 | How it works | section label |
| L36 | 5 | Create, record, sign, and verify. | clear heading |
| L37 | 4 | Record the approved scope. | step heading |
| L38 | 11 | Start with the actor, approval reference, scope, redaction rules, and retention window. | F-2-6 |
| L39 | 6 | Record a command or tool result. | step heading |
| L40 | 8 | Capture command output, tool data, and file hashes. | F-2-7 |
| L41 | 5 | Sensitive values are redacted first. | F-2-8 |
| L42 | 5 | Sign and export the receipt. | step heading |
| L43 | 7 | Export JSON or a self-contained HTML report. | F-2-9 |
| L44 | 4 | Technical details use Ed25519. | F-2-13 |
| L45 | 4 | Integrity is not identity. | useful limitation heading |
| L46 | 9 | A valid receipt shows signed contents have not changed. | mapped to browser-verification |
| L47 | 13 | It does not prove approval was legitimate or an action was correct. | useful limitation |
| L48 | 4 | Command-line quick start | section label |
| L49 | 8 | Run the sample or use your own change. | clear heading |
| L50 | 2 | Copy commands | result-naming control |
| L51 | 3 | Download Linux x64 | result-naming link |
| L52 | 3 | Read the source (GitHub) | explicit external destination |
| L53 | 2 | Export formats | section label |
| L54 | 7 | Export each receipt as JSON or HTML. | F-2-9 |
| L55 | 3 | Install the CLI | clear action |
| L56 | 7 | Record and verify automated repository and service changes. | product description |
| L57 | 2 | Source (GitHub) | explicit external destination |
| L58 | 5 | Built by Param Factory · v0.1.0 | footer metadata |

### README

| Ref | Words | Sentence | Result |
| --- | ---: | --- | --- |
| R1 | 9 | Record and verify automated changes in a local receipt file. | product description |
| R2 | 14 | It is for teams using agents, scripts, or CI to change repositories and services. | audience |
| R3 | 8 | Each event links to the event before it. | mapped only partially; add chain assertion |
| R4 | 5 | A signature reveals later changes. | mapped only partially; add signature assertion |
| R5 | 5 | Verification works without a server. | F-2-11 |
| R6 | 12 | A signature does not prove identity, approval legitimacy, occurrence, intent, or correctness. | useful limitation |
| R7 | 8 | Rust 1.85 or newer builds the single binary. | unlisted build compatibility claim |
| R8 | 11 | Download the Linux x64 binary from the product site, or use `cargo install`. | unlisted live-download claim |
| R9 | 6 | Run this command from any directory. | mapped to cli-demo-lifecycle |
| R10 | 18 | It creates a realistic signed receipt in a new temporary directory and prints the JSON and HTML paths. | mapped to cli-demo-lifecycle |
| R11 | 10 | Open `/demo/` on the product site for the isolated browser sample. | mapped to demo-isolated, currently failing |
| R12 | 12 | It shows a documentation deployment receipt and stores demo choices under `demo:` only. | mapped to demo-isolated, currently failing |
| R13 | 9 | `new` writes a separate signing key with mode 0600. | F-2-10 |
| R14 | 5 | Do not commit that key. | instruction |
| R15 | 9 | `record` adds a tool result without running a process. | F-2-10 |
| R16 | 13 | `run` records command arguments, working directory, redacted output, exit status, duration, and file hashes. | F-2-10 |
| R17 | 7 | Redaction happens before receipt data is written. | F-2-10 |
| R18 | 9 | A receipt has `subject`, `policy`, ordered `events`, `chain_head`, and `proof`. | unlisted format claim |
| R19 | 4 | Unknown fields are rejected. | F-2-10 |
| R20 | 11 | For implementers, signatures use Ed25519 over RFC 8785 canonical JSON without `proof.signature`. | unlisted conformance claim |
| R21 | 5 | The machine-readable contract is schema/receipt-v1.schema.json. | link/documentation |
| R22 | 8 | The browser verifier checks receipt JSON in memory. | F-2-11 |
| R23 | 5 | Selected files are not uploaded. | F-2-11 |
| R24 | 8 | After one visit, its sample can reload offline. | F-2-11 |
| R25 | 7 | The CLI does not need an account. | F-2-11 |
| R26 | 8 | See the product privacy and terms pages. | navigation |
| R27 | 7 | `npm run build:site` creates `dist/site`, including the Linux download. | unlisted build-output claim |
| R28 | 6 | Publish `dist/site/` as the static site. | deploy instruction |
| R29 | 7 | The factory owns deployment and registry publishing. | process statement |
| R30 | 9 | The package is ready to publish with `cargo package`. | unlisted package-readiness claim |
| R31 | 4 | MIT — see LICENSE. | license reference |

## Claims and sandbox behaviour

All commands were run from a fresh clone at the stated base. `npm ci` in this sandbox omitted development dependencies, so the first Playwright invocation could not import `playwright`; after the standard explicit `npm ci --include=dev` test setup and production build, the claim results were:

| Claim | Exact manifest command | Result |
| --- | --- | --- |
| cli-demo-lifecycle | `npm run test:cli -- claim_cli_demo_lifecycle` | pass |
| demo-isolated | `npm run test:e2e -- --grep @claim:demo-isolated` | **fail** on mobile: header overlap/time-out |
| browser-verification | `npm run test:e2e -- --grep @claim:browser-verification` | pass (desktop and mobile) |
| receipt-never-uploaded | `npm run test:e2e -- --grep @claim:receipt-never-uploaded` | pass (desktop and mobile) |
| two-mb-limit | `npm run test:e2e -- --grep @claim:two-mb-limit` | pass (desktop and mobile) |
| offline-reload | `npm run test:e2e -- --grep @claim:offline-reload` | pass (desktop and mobile) |
| no-account-and-no-telemetry | `npm run test:e2e -- --grep @claim:no-account-and-no-telemetry` | pass, but insufficient for the stronger landing wording; see F-2-3 |

The live browser demo made only same-origin requests (document, JS, CSS, artwork, and `/sample.receipt.json`) in a fresh context. The CLI demo was also run from an unrelated temporary directory; it wrote its output only to a newly created OS temporary directory and did not alter the caller directory. These pass observations do not remove the failed mobile claim test.

## History check

Read all earlier review, polish, verification, and handoff documents.

- F-1-1, F-1-3 existence of a manifest, F-1-4, F-1-5, F-1-6, F-1-10, F-1-11 through F-1-80 copy/payment/capability removals or rewrites were checked in live markup and source where applicable. The first-screen wording, real CLI demo, direct browser route, 404, skip-link targets, and paid-tier removal are present.
- F-1-2 is reopened because the required terminal recording was never added; its current browser demo also fails the stronger immediate-use check.
- F-1-7, F-1-8, and F-1-9 are reopened above because the live route markup still lacks required metadata, route announcements, and one shared header/footer.
- Earlier successful evidence for offline reload, low initial JS, no normal first-load console errors, and same-origin demo requests was independently consistent with this run.

## Structure and links

Home, demo, Privacy, Terms, 404, robots, sitemap, and the Linux download return the expected HTTP statuses. All internal and GitHub links discovered across the five pages returned 200 (hash links resolve to their home document). Each route has one h1 and one main; titles follow the route pattern. The designed 404 is present and unknown live routes return it with HTTP 404. The metadata, announcement, and shared-navigation exceptions are recorded as blocking findings above.

## Missed leverage

No AI feature is warranted: model interpretation would not improve tamper evidence and could undermine the product's trust boundary. JSON and HTML export already cover the expected portability path. The missing high-leverage experience is a visible, working demo result on the first demo screen and a real terminal recording of the existing CLI demo.

## What would make this perfect

Make `/demo/` responsive and immediately show the verified realistic receipt; make every listed claim pass; add the missing real CLI recording; implement the same metadata, live announcement, header, and footer on every route; then either test each remaining concrete capability/privacy statement or remove it. Re-run the complete claim matrix from a clean clone, including the 390 px mobile project, before marking the review clear.
