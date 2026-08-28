# Adversarial first-read review 1 — FAIL

Reviewed 2026-08-28 at `https://automation-action-receipts.sociobot.in` from fresh Chromium contexts at 390 × 844 and 1440 × 900. Repository base: `632f72095adbcc7a67c091a43b69d4ca1ee719ae`.

## Verdict

**FAIL.** There are 6 blocking findings, 74 additional findings, and no claim test manifest. The product is functional once understood, but it is not clear, tryable, or completely verifiable under this review contract.

## First screen, before scrolling

My cold read on both phone and desktop:

- **What it does:** probably a local command-line tool that records declared approval, scope, commands, output, and file hashes, then signs a receipt for offline checking.
- **For whom:** not stated. I can infer engineering or operations teams using automation, but the screen never says that.
- **What to click first:** unclear. “Install the CLI” and “Verify a receipt” compete, and neither is a sample-data trial.

The exact first-screen text that fails is the headline, **“Proof of work. For the work.”** It does not name the job. The supporting sentence says what the CLI records but not who needs it. The two adjacent actions do not identify a first path. On the 390 px screen the sample action is not visible before scrolling.

## Blocking findings

### F-1-1 — BLOCKING — first screen does not answer all three questions

- **Location/quote:** home hero: “Proof of work. For the work.”; “A local CLI that records authorization, scope, commands, outputs, and artifact hashes—then signs the whole chain for offline review.”; “Install the CLI”; “Verify a receipt”.
- **Why this fails:** the headline is a slogan, the audience is absent, and two peer actions leave a new visitor without a first step.
- **Concrete fix:** headline: **“Record and verify automated changes.”** Supporting sentence: **“For teams that let agents, scripts, or CI change repositories and services.”** Primary action: **“Try it with sample data”**, followed by **“See a signed deployment receipt; nothing is saved.”** Keep install as the secondary action.

### F-1-2 — BLOCKING — the required CLI demo does not exist

- **Evidence:** `/demo` returns 404. `/?demo=1` renders the normal landing page and does not load sample data. “Load signed sample” is below the first phone screen. After it is clicked, the browser verifier displays a realistic two-event receipt, but this only checks an already-created receipt; it does not demonstrate the CLI creating, recording, sealing, or exporting one. There is no “Demo — sample data, nothing is saved” banner, “Reset demo”, or “Start for real”. The live Linux binary returns exit 2 for both `action-receipts demo` and `action-receipts --demo`. There is no `.factory/demo.md` or bundled `examples/` flow. The landing terminal is static text, not a recording of the binary.
- **Isolation evidence:** loading and verifying the current sample made only same-origin requests and wrote no browser storage. That narrow path is private, but it is not demo mode. On `/?demo=1`, changing theme writes the real key `ar_theme`, proving the query parameter has no separate namespace.
- **Concrete fix:** ship `action-receipts demo`, make it run bundled realistic input in a newly created temporary directory, print the JSON/HTML output paths, and leave the caller’s files untouched. Add a self-hosted terminal recording of that exact command. Make `/demo` or `?demo=1` enter the isolated flow directly, with the required persistent banner, reset, and exit controls. Document it in `.factory/demo.md`.

### F-1-3 — BLOCKING — the claims contract is absent

- **Location:** `.factory/claims.json` does not exist; `rg '@claim'` finds no tagged tests.
- **Why this fails:** the landing page, README, privacy page, and terms page make many claims a visitor could rely on. None has the required manifest entry or exactly one clean-sandbox test. There were therefore zero listed claim commands to run, and every claim remains untested under the claims contract even where a general test happens to exercise similar behavior.
- **Concrete fix:** add `.factory/claims.json`; add one `@claim:<id>` test per claim listed in F-1-44 through F-1-80; remove claims that cannot be tested. Make the tests start only from the documented demo command/URL.

### F-1-4 — BLOCKING — the paid purchase action is dead

- **Location/quote:** home pricing, “Buy the team kit” → `https://api.sociobot.in/api/v1/products/automation-action-receipts/checkout`.
- **Evidence:** HEAD and GET both return HTTP 404; GET body is `{"error":"enabled factory product","status":404}`.
- **Why this fails:** the site offers a $39 product that cannot be bought. “Secure checkout” is also unsupported because no checkout opens.
- **Concrete fix:** configure the Sociobot billing product and test the exact public link through its redirect to a valid hosted checkout, or remove the paid tier and all purchase/license copy until it is available.

### F-1-5 — BLOCKING — broken routes use an unrelated Azure 404

- **Location:** `/404`, `/demo`, and an arbitrary unknown route all return the stock “Azure Static Web Apps - 404: Not found” page. `staticwebapp.config.json` has no `navigationFallback`, `responseOverrides.404`, or designed 404 rewrite.
- **Why this fails:** this is broken routing and a generic platform template, not the product identity. The stock page loads Microsoft/Azure CDN styles and scripts, contradicting the site’s broad no-runtime-CDN statement.
- **Concrete fix:** create a chain-of-custody-style `/404.html` with a clear link home, add the documented Static Web Apps 404 response override, and test arbitrary unknown paths. Add the real `/demo` route separately.

### F-1-6 — BLOCKING — prior skip-link defect is only half fixed

- **History:** prior verification defect **P2** said the skip link did not move focus into main content. The current handoff says skip focus works.
- **Evidence:** on `/`, Enter on “Skip to content” focuses `<main id="main" tabindex="-1">`; this part is fixed. On `/privacy/` and `/terms/`, `<main>` has no `tabindex`, so Enter leaves focus on the skip link. Confirmed live with keyboard and in source.
- **Why this fails:** the earlier accessibility finding regresses on two shipped routes and the handoff overstates the fix.
- **Concrete fix:** put `tabindex="-1"` on every route’s main landmark and add the same keyboard regression test for `/privacy/` and `/terms/`.

## Other product findings

### F-1-7 — metadata is incomplete

- **Location:** all three HTML routes.
- **Evidence:** there is no canonical link, Open Graph title/description/image, Twitter card, or apple-touch icon. The manifest has an empty `icons` array. Legal pages also omit `theme-color`. The home title, “Action Receipts — signed evidence for automation”, is a noun phrase rather than a plain statement of what the product does.
- **Concrete fix:** use **“Action Receipts — record and verify automated changes”**; add route-specific canonical and social metadata, a product-art 1200 × 630 image, 180 px apple icon, manifest icons, and palette theme color on every route.

### F-1-8 — navigation does not move or announce focus on route change

- **Evidence:** navigating home → Privacy leaves `document.activeElement` on `BODY`; there is no route announcement live region on Privacy or Terms. Back restores the prior scroll position correctly.
- **Concrete fix:** on each document/route arrival focus the new h1 (with `tabindex="-1"`) and announce its title in a polite live region; add forward/back tests.

### F-1-9 — header and footer are not consistent or complete

- **Evidence:** the home header has Verify/Protocol/Pricing but no Privacy; legal headers replace those links with Home and the other legal page. Legal footers omit the product one-liner. Every footer omits “Built by Param Factory” and version/build id. Footer GitHub and the checkout link do not identify themselves as external.
- **Concrete fix:** use one shared header/footer skeleton on every route, include Privacy within the four-link limit, include the one-liner, Terms, “Built by Param Factory”, and build id, and label external destinations.

### F-1-10 — axe finds a landmark structure error

- **Location:** home honesty note, `<aside>` nested inside `<main>`.
- **Evidence:** full axe runs at desktop and 390 px report `landmark-complementary-is-top-level` (moderate), one node each.
- **Concrete fix:** use a non-landmark element with `role="note"`, or move the complementary landmark outside `main`; make the full axe result, not only serious/critical findings, fail CI.

## Copy findings and proposed rewrites

Each row is a separate copy finding. The claims column later records test-contract failures independently.

| ID | Exact quote/location | Problem | Proposed rewrite |
| --- | --- | --- | --- |
| F-1-11 | Hero h1: “Proof of work. For the work.” | Mood slogan; does not name the job. | “Record and verify automated changes.” |
| F-1-12 | Hero: “A local CLI that records authorization, scope, commands, outputs, and artifact hashes—then signs the whole chain for offline review.” | Dense jargon and no audience. | “For teams using agents, scripts, or CI, it records each approved change and signs a receipt you can check offline.” |
| F-1-13 | Hero fact / protocol: “Ed25519 signed”; “Lock the chain with Ed25519.” | Algorithm jargon is presented before the user benefit. | “Signed receipts reveal later changes.” Put “Uses Ed25519” in technical details. |
| F-1-14 | Hero caption/strip: “Authorization → invocation → artifact → seal”; “Declared authority”; “Artifact digest”; “Ed25519 seal” | “Invocation”, “digest”, and “seal” require translation. | “Stated approval → recorded command → file hash → signature.” |
| F-1-15 | Verifier h2: “Drop the log. Check the receipt.” | Metaphor and inaccurate noun: the input is a receipt, not a raw log. | “Verify a signed receipt.” |
| F-1-16 | Verifier: “We recompute every event hash and verify the signature with Web Crypto.” | Implementation jargon. | “The verifier checks every recorded event and the receipt’s signature.” |
| F-1-17 | Protocol h2: “A chain reviewers can actually read.” | “Actually” is promotional and the heading does not name a section. | “How receipt verification works.” |
| F-1-18 | Step heading: “Declare the boundary” | Abstract jargon. | “Record the approved scope.” |
| F-1-19 | “Record real execution”; “structured tool events” | “Real” is an unsupported qualifier; “structured events” is unexplained. | “Record a command or tool result.” |
| F-1-20 | Step heading: “Seal and carry” | Metaphor. | “Sign and export the receipt.” |
| F-1-21 | Install h2: “From open to verified.” | Meaning depends on product lore. | “Create, record, sign, and verify.” |
| F-1-22 | Eyebrow: “Four commands” | The shown block has five command invocations and the CLI exposes six subcommands. | “Command-line quick start.” |
| F-1-23 | Pricing eyebrow: “Keep the format open” | Mood statement rather than a section name. | “Pricing.” |
| F-1-24 | Plan: “Open format” | Does not tell buyers what the plan contains. | “Free CLI and receipt format.” |
| F-1-25 | “Reusable policy presets and CI gate snippets.” | “Preset” and “gate snippet” are unexplained. | “Reusable redaction rules, retention settings, and CI checks.” |
| F-1-26 | “Secure checkout.” | Marketing adjective and currently false because the link is 404. | After repair: “Checkout is handled by Sociobot/Dodo.” |
| F-1-27 | Eyebrow: “Portable by design” | Slogan that names no section. | “Export formats.” |
| F-1-28 | “Your audit trail should outlive the tool that made it.” | Aspirational slogan rather than usable information. | “Export each receipt as JSON or self-contained HTML.” |
| F-1-29 | Footer: “Signed evidence for automated work.” | Generic slogan. | “Record and verify automated repository and service changes.” |
| F-1-30 | Theme button: “Dark” | Not a result-naming verb. | “Use dark theme” / “Use light theme”. |
| F-1-31 | Sample button: “Load signed sample” | Does not use the required, instantly understood demo wording. | “Try it with sample data.” |
| F-1-32 | Terminal button: “Copy all” | Does not say what will be copied. | “Copy commands.” |
| F-1-33 | Free-plan button: “Install free” | Awkward and inconsistent with “Install the CLI”. | “Install the CLI.” |
| F-1-34 | Closing button: “Create your first receipt” | Clicking only scrolls to install instructions; it does not create a receipt. | “Install the CLI.” |
| F-1-35 | README R1, 32 words: “Action Receipts is a local-first CLI and open JSON format for answering four questions about an automated change: what ran, with which inputs, against what declared scope, and under whose stated authorization.” | Exceeds 22 words and stacks jargon. | “Action Receipts records automated changes in a local JSON file. It shows what ran, its inputs, its approved scope, and the stated authorization.” |
| F-1-36 | README R2: “Events are SHA-256 hash chained; sealed bundles are signed with Ed25519 and verify without a server.” | “Hash chained”, “sealed bundles”, and the algorithm arrive before the benefit. | “Each event links to the one before it. A signature reveals later changes, and verification needs no server.” |
| F-1-37 | README R7: “Prebuilt Linux binaries are also exposed on the product site build under `downloads/`.” | “Exposed” and “site build” are internal jargon; no usable URL is given. | “Download the Linux x64 binary from the product site.” |
| F-1-38 | README R16: “Clap usage errors exit `2`.” | Names an implementation library instead of the user-visible condition. | “Invalid command arguments exit `2`.” |
| F-1-39 | README R18: “The public surface is intentionally small.” | Vague marketing claim. | “A receipt has five top-level parts.” |
| F-1-40 | README R21: “The Ed25519 signature covers the canonical receipt without the `proof.signature` field using RFC 8785 JSON Canonicalization (JCS).” | Dense specification language without a plain lead-in. | “For implementers: signatures use Ed25519 over RFC 8785 canonical JSON, excluding `proof.signature`.” |
| F-1-41 | README R27: “The Vite site documents the protocol and verifies receipt JSON locally using Web Crypto.” | Framework and browser-API jargon obscure the task. | “The browser verifier checks receipt JSON on your device.” |
| F-1-42 | README R28: “Uploaded receipts never leave the device.” | “Uploaded” implies a network upload that does not occur. | “Selected receipt files are read only in your browser.” |
| F-1-43 | README R29: “It includes offline shell caching, privacy and terms pages, and the optional Sociobot license unlock for the Team policy kit.” | “Shell caching” and “license unlock” are implementation jargon; three unrelated ideas share one sentence. | “After one online visit, the verifier works offline. The optional Team policy kit uses a Sociobot license.” |

## Complete landing-page sentence audit

Counts are whitespace-delimited words; standalone symbols are excluded. The audit covers all static prose/headings rendered on the landing page. No landing sentence exceeds 22 words.

| Ref | Words | Sentence or heading | Finding |
| --- | ---: | --- | --- |
| L1 | 4 | Open receipt format · v0.1.0 | — |
| L2 | 3 | Proof of work. | F-1-11 |
| L3 | 3 | For the work. | F-1-11 |
| L4 | 19 | A local CLI that records authorization, scope, commands, outputs, and artifact hashes—then signs the whole chain for offline review. | F-1-12, F-1-44 |
| L5 | 2 | Offline verifier | — |
| L6 | 3 | Drop the log. | F-1-15 |
| L7 | 3 | Check the receipt. | F-1-15 |
| L8 | 6 | The file stays in this tab. | F-1-48 |
| L9 | 12 | We recompute every event hash and verify the signature with Web Crypto. | F-1-16, F-1-49 |
| L10 | 1 | Unchecked | — |
| L11 | 3 | No receipt loaded | — |
| L12 | 9 | Choose a JSON bundle or load the signed sample. | — |
| L13 | 6 | The verifier makes no network request. | F-1-48 |
| L14 | 2 | The protocol | — |
| L15 | 6 | A chain reviewers can actually read. | F-1-17 |
| L16 | 3 | Declare the boundary | F-1-18 |
| L17 | 16 | Record the actor label, authorization reference, intended scope, redaction rules, and retention window before work begins. | F-1-51 |
| L18 | 3 | Record real execution | F-1-19 |
| L19 | 8 | Wrap a command or append structured tool events. | F-1-19, F-1-52 |
| L20 | 11 | Output is capped, sensitive values are redacted, and artifacts are hashed. | F-1-52 |
| L21 | 3 | Seal and carry | F-1-20 |
| L22 | 5 | Lock the chain with Ed25519. | F-1-13, F-1-47 |
| L23 | 12 | Export JSON for machines and a self-contained HTML view for incident review. | F-1-53 |
| L23a | 4 | Integrity is not intent. | — (useful limitation heading) |
| L24 | 10 | A valid receipt proves that the signed bundle is unchanged. | F-1-54 |
| L25 | 17 | It does not prove a person's identity, the legitimacy of authorization, or that an action was correct. | F-1-54 |
| L26 | 2 | Four commands | F-1-22 |
| L27 | 4 | From open to verified. | F-1-21 |
| L28 | 4 | Keep the format open | F-1-23 |
| L29 | 2 | Free receipts. | F-1-55 |
| L30 | 3 | Optional team kit. | F-1-58 |
| L31 | 12 | Creation, JSON and HTML export, signatures, redaction, retention, and verification stay free. | F-1-55 |
| L32 | 2 | Open format | F-1-24 |
| L33 | 1 | $0 | F-1-56 |
| L34 | 1 | Forever. | F-1-56 |
| L35 | 2 | MIT licensed. | F-1-56 |
| L36 | 3 | Team policy kit | F-1-58 |
| L37 | 3 | $39 one time | F-1-59 |
| L38 | 7 | Reusable policy presets and CI gate snippets. | F-1-25, F-1-58 |
| L39 | 2 | Secure checkout. | F-1-26, F-1-60 |
| L40 | 5 | Sociobot/Dodo is merchant of record. | F-1-60 |
| L41 | 4 | No team license stored. | — |
| L42 | 3 | Portable by design | F-1-27 |
| L43 | 10 | Your audit trail should outlive the tool that made it. | F-1-28 |
| L44 | 5 | Signed evidence for automated work. | F-1-29 |
| L45 | 4 | MIT licensed · No telemetry | F-1-46, F-1-56 |

Conditional or exercised landing states: `Offline mode — local verification still works.` (6, F-1-72); `Have a license?` (3); `Paste it here` (3); `Generate a starter policy for version control.` (7, F-1-58); `Nothing is uploaded.` (3, F-1-48); `Receipt ar_33bdc145bac9f37c5fc5a9acdc36f260.` (2); `Every event hash and the Ed25519 signature match.` (8, F-1-49); `A valid signature proves integrity—not identity, intent, authorization legitimacy, or correctness.` (12, F-1-54).

Visible non-sentence controls and fragments, with word counts: `AR/ Action Receipts` (3); `Verify` (1); `Protocol` (1); `Pricing` (1); `Dark` (1, F-1-30); `Install the CLI` (3); `Verify a receipt` (3); `No account` (2, F-1-45); `No telemetry` (2, F-1-46); `Ed25519 signed` (2, F-1-13/F-1-47); `Authorization → invocation → artifact → seal` (4, F-1-14); `Declared authority` (2, F-1-14); `Tool event` (2); `Artifact digest` (2, F-1-14); `Ed25519 seal` (2, F-1-14); `Choose a receipt JSON` (4); `or drop it here · max 2 MB` (7, F-1-50); `or paste JSON` (3); `Receipt JSON` (2); `Verify locally` (2); `Load signed sample` (3, F-1-31); `Copy all` (2, F-1-32); `Download Linux x64` (3); `Read the source` (3); `Unlimited local receipts` (3, F-1-57); `JSON + HTML export` (3, F-1-53); `Offline Ed25519 verification` (3, F-1-47); `Redaction + retention commands` (3, F-1-52/F-1-71); `Install free` (2, F-1-33); `Redaction policy generator` (3, F-1-58); `Retention presets` (2, F-1-58); `GitHub Actions gate snippet` (4, F-1-58); `Future policy-kit updates` (3, F-1-59); `Buy the team kit` (4); `Restore purchase` (2); `Remove license from this device` (5); `Download policy JSON` (3); `Create your first receipt` (4, F-1-34); `Privacy` (1); `Terms` (1); `GitHub` (1).

## Complete README sentence audit

All README headings are literal section names and need no heading rewrite. Code blocks are commands, not sentences, and are excluded.

| Ref | Words | Sentence | Finding |
| --- | ---: | --- | --- |
| R1 | 32 | Action Receipts is a local-first CLI and open JSON format for answering four questions about an automated change: what ran, with which inputs, against what declared scope, and under whose stated authorization. | F-1-35, F-1-44 |
| R2 | 16 | Events are SHA-256 hash chained; sealed bundles are signed with Ed25519 and verify without a server. | F-1-36, F-1-47 |
| R3 | 14 | It is for teams that let agents, scripts, or CI change repositories and services. | — |
| R4 | 14 | A valid receipt proves that the bundle has not changed since it was signed. | F-1-54 |
| R5 | 20 | It does **not** prove the actor's real-world identity, that the action was correct, or that the stated authorization was legitimate. | F-1-54 |
| R6 | 9 | Build the single binary with Rust 1.85 or newer: | F-1-61 |
| R7 | 13 | Prebuilt Linux binaries are also exposed on the product site build under `downloads/`. | F-1-37, F-1-62 |
| R8 | 10 | Registry publishing is performed by the factory, not this repo. | — |
| R9 | 4 | Create an open receipt. | F-1-44 |
| R10 | 13 | The signing key is written separately with mode 0600; do not commit it. | F-1-63 |
| R11 | 17 | Run a command and record its arguments, working directory, redacted output, exit status, duration, and artifact digest: | F-1-64 |
| R12 | 10 | Declared integrations can append structured events without running a process: | F-1-65 |
| R13 | 15 | Seal both portable JSON and a self-contained readable HTML report, then verify either one offline: | F-1-47, F-1-53 |
| R14 | 6 | `verify --json` is stable for CI. | F-1-66 |
| R15 | 16 | It exits `0` when valid, `3` when integrity verification fails, and `1` for an I/O/runtime error. | F-1-66 |
| R16 | 5 | Clap usage errors exit `2`. | F-1-38, F-1-66 |
| R17 | 4 | All commands are non-interactive. | F-1-66 |
| R18 | 6 | The public surface is intentionally small. | F-1-39 |
| R19 | 12 | A receipt contains `subject`, `policy`, ordered `events`, `chain_head`, and (after sealing) `proof`. | F-1-67 |
| R20 | 16 | Each event hash covers its sequence, timestamp, kind, tool, redacted data, artifact hashes, and previous hash. | F-1-67 |
| R21 | 17 | The Ed25519 signature covers the canonical receipt without the `proof.signature` field using RFC 8785 JSON Canonicalization (JCS). | F-1-40, F-1-67 |
| R22 | 8 | Unknown fields are rejected by this v1 verifier. | F-1-68 |
| R23 | 5 | The machine-readable contract is [schema/receipt-v1.schema.json](schema/receipt-v1.schema.json). | F-1-67 |
| R24 | 7 | Redaction is applied before anything is stored. | F-1-69 |
| R25 | 18 | Sensitive JSON key names are redacted by default; add literal values with `--redact` or environment values with `--redact-env`. | F-1-69 |
| R26 | 19 | Retention is declared in every receipt and can be enforced locally with `action-receipts prune --dir receipts --older-than 30 --dry-run`. | F-1-70 |
| R27 | 14 | The Vite site documents the protocol and verifies receipt JSON locally using Web Crypto. | F-1-41, F-1-49 |
| R28 | 6 | Uploaded receipts never leave the device. | F-1-42, F-1-48 |
| R29 | 20 | It includes offline shell caching, privacy and terms pages, and the optional Sociobot license unlock for the Team policy kit. | F-1-43, F-1-72/F-1-58 |
| R30 | 16 | Tests cover the documented lifecycle, redaction, artifact hashing, HTML extraction, tamper detection, and browser verifier behavior. | F-1-73 |
| R31 | 11 | Run `npm run build:site`; publish exactly `dist/site/` as a static site. | F-1-74 |
| R32 | 19 | This command builds the release CLI before Vite clears the output directory, then copies `downloads/action-receipts-linux-amd64` into the deploy root. | F-1-74 |
| R33 | 10 | The factory owns deployment, product registration, DNS, and registry credentials. | — |
| R34 | 4 | There is no telemetry. | F-1-46 |
| R35 | 5 | CLI data stays on disk. | F-1-75 |
| R36 | 7 | The browser verifier processes files in memory. | F-1-48 |
| R37 | 19 | Receipts can still expose commands, paths, and output, so use redaction and a retention window appropriate to the data. | — (safety warning) |
| R38 | 7 | See `/privacy/` and `/terms/` on the site. | — |
| R39 | 4 | MIT — see [LICENSE](LICENSE). | F-1-56 |

README average: 12 words across 39 sentences. R1 is the only sentence over the 22-word hard cap.

## Unlisted claim findings

Every row below is a separate unlisted-claim finding. Similar quotes are grouped only where one claim entry and one end-to-end test can prove the same observable promise.

| ID | Exact claim/location | Required concrete fix |
| --- | --- | --- |
| F-1-44 | Hero/README: records automated changes, stated authorization/scope/inputs, and creates an open receipt. | Add `receipt-lifecycle` and test the full demo-created receipt fields. |
| F-1-45 | Hero: “No account”. | Add `no-account` and complete the demo/install/verify path without authentication. |
| F-1-46 | Hero/footer/README/privacy: “No telemetry”; “does not add tracking identifiers”. | Add `no-telemetry`; record all CLI and browser requests during the full demo. |
| F-1-47 | Hero/protocol/README: Ed25519 signing and offline JSON/HTML verification. | Add `offline-signature-verification`; create, tamper, and verify both exports with networking disabled. |
| F-1-48 | Verifier/README/privacy: file stays in the tab, makes no network request, is processed in memory, and is not uploaded. | Add `receipt-never-uploaded`; inspect the request log through select/paste/verify/tamper/reset. |
| F-1-49 | Landing/README: every event hash and signature are recomputed/verified in the browser. | Add `browser-full-chain-verification`; alter an early event and separately alter the signature. |
| F-1-50 | File control: “max 2 MB”. | Add `browser-two-mb-limit`; measure acceptance at the boundary and rejection above it. |
| F-1-51 | Protocol: actor, authorization, scope, redaction, and retention are recorded before work. | Add `declared-boundary-fields`; assert the demo receipt order and contents. |
| F-1-52 | Protocol/plan: commands/tool events are recorded; output is capped, sensitive values redacted, artifacts hashed. | Add separate manifest claims/tests for capture, cap boundary, redaction, and artifact hash; one sentence currently combines four promises. |
| F-1-53 | Protocol/plan/README: exports JSON and self-contained readable HTML. | Add `json-html-export`; open HTML without local dependencies and compare its signed payload with JSON. |
| F-1-54 | Landing/README/terms: a valid receipt proves unchanged signed contents but not identity, legitimate authorization, occurrence, intent, or correctness. | Add `integrity-not-identity` covering the verifier result/caveat, or retain as a formally documented security limitation with an executable conformance test. |
| F-1-55 | Pricing: core creation, export, signatures, redaction, retention, and verification “stay free”. | Add `free-core-features`; test without a license. |
| F-1-56 | Pricing/footer/README: `$0`, “Forever”, MIT licensed. | Test license presence and ungated core behavior; remove “Forever”, which cannot be proved. |
| F-1-57 | Plan: “Unlimited local receipts”. | Remove “Unlimited” or add a defensible measured boundary test. |
| F-1-58 | Pricing/README/terms: optional Team kit, policy generator, redaction/retention presets, GitHub Actions gate snippet. | Add a fixture-license test that verifies every listed deliverable; the current test checks only that the generator heading becomes visible. |
| F-1-59 | Pricing/terms: `$39 one time`; “Future policy-kit updates”. | Add a billing/catalog contract test for price; remove the untestable future-update promise. |
| F-1-60 | Pricing/privacy/terms: “Secure checkout”; Sociobot/Dodo merchant of record and checkout behavior. | Repair the 404, replace “Secure” with a factual statement, and test the expected hosted-checkout redirect without purchase. |
| F-1-61 | README: builds with Rust 1.85 or newer. | Add `rust-min-version` in a pinned Rust 1.85 clean container. |
| F-1-62 | README: prebuilt Linux binary is exposed on the product site. | Add `linux-download`; assert 200, executable format, version, and a demo run from a temp directory. |
| F-1-63 | README: signing key is separate and mode 0600. | Add `private-key-permissions`; assert path separation and mode in the demo temp dir. |
| F-1-64 | README: `run` records arguments, cwd, redacted output, status, duration, and artifact digest. | Add `command-provenance` and assert every field from the demo command. |
| F-1-65 | README: integrations append structured events without running a process. | Add `structured-record`; assert no child process and the resulting event. |
| F-1-66 | README: stable CI JSON, documented exit codes 0/1/2/3, and all commands non-interactive. | Add focused boundary tests for every exit code/stdout schema and a no-stdin test. |
| F-1-67 | README: exact v1 fields, hash/signature coverage, RFC 8785 canonicalization, and schema contract. | Add conformance vectors shared by Rust, browser, and schema validation. |
| F-1-68 | README: unknown fields are rejected. | Add `unknown-fields-rejected` for top-level and nested fields in CLI and browser. |
| F-1-69 | README/privacy: redaction happens before storage; default, literal, and environment values are redacted. | Add `redact-before-storage`; inspect intermediate and final files for supplied secrets. |
| F-1-70 | README/privacy: retention is declared and locally enforceable. | Add `retention-prune`; cover dry run, confirmation, before/after boundary, and unrelated files. |
| F-1-71 | Landing plan: redaction and retention commands are available. | Either merge exact observable wording into F-1-69/F-1-70 manifest locations or remove the duplicate claim. |
| F-1-72 | Offline bar/README/privacy: verifier works offline after caching/first visit. | Add `offline-reload` using a fresh context and documented demo entry, then set offline and reload/reset. |
| F-1-73 | README: tests cover lifecycle, redaction, hashes, HTML, tampering, and browser behavior. | Add a manifest-backed coverage check or replace this self-claim with links to named tests/status. |
| F-1-74 | README: `build:site` creates deploy-ready `dist/site` including the Linux binary. | Add `site-build-output`; assert required routes/assets and executable download in a clean clone. |
| F-1-75 | README/privacy: CLI data stays on disk and the CLI sends no data/keys anywhere. | Add `cli-no-network`; run the full demo under network interception and inspect files. |
| F-1-76 | Privacy: theme, license token, and verdict storage keys; removal control clears them. | Add `browser-storage`; assert exact keys and clearing behavior in a fresh real-data namespace. |
| F-1-77 | Privacy: license verification sends the token to Sociobot no more than once per day. | Add `license-check-frequency`; use a clock and request fixture at the 24-hour boundary. |
| F-1-78 | Privacy: host may keep short-lived security/delivery logs and the product adds no identifiers. | This cannot be proved from this repository; cite the host retention contract precisely or remove “short-lived”. |
| F-1-79 | Privacy: no advertising, behavioral analytics, third-party fonts, or runtime CDN scripts. | Add `no-third-party-runtime` across every route including 404. It currently fails on the stock Azure 404. |
| F-1-80 | Terms: refund revokes the corresponding license. | Add a billing webhook/license fixture test, or remove the promise until that integration exists. |

## Demo and sandbox result

**FAIL (F-1-2).** The supplied receipt is realistic: a documentation deployment approved by ops with a policy-gate event and an npm build event. Clicking the buried sample button immediately shows “Cryptographically valid” and two linked events. The verifier sample made only these requests: the document, same-origin JS/CSS/art, and `/sample.receipt.json`. Browser storage stayed empty during that flow. Offline reload and sample verification succeeded after the first online visit.

Those narrow checks do not satisfy a CLI demo. There is no creation workflow, temp-directory output, reset, persistent demo notice, direct demo entry, or storage namespace.

## Claims execution result

`.factory/claims.json` is missing, so there were no listed claim commands to execute. This is not a pass with zero tests. The clean clone still produced this baseline evidence:

```text
npm test: 5 Rust unit + 1 CLI integration + 2 Vitest tests passed
npm run build: passed; dist/site produced; JS 10.55 kB raw / 4.18 kB gzip
npm run test:e2e: 15 passed, 1 intended mobile-only skip
verify-url.sh (live): HTTP 200, no console errors, title/lang/one h1/main/alt/button labels passed
axe (live desktop and 390 px): 1 moderate violation on each viewport
```

These general tests are not tagged claim tests and do not replace the missing manifest.

## Structure, links, privacy, and visual identity

- **Pass:** `/`, `/privacy/`, and `/terms/` return 200; each has `lang=en`, a descriptive title, one h1, a main landmark, and a meta description. SVG favicon, robots, sitemap, reduced-motion rules, visible home skip focus, mobile width, and first-load JS budget pass.
- **Pass:** home hash deep links reload correctly; browser back restores the previous home scroll position.
- **Pass:** the live Linux x64 download is fixed: HTTP 200 and 1,364,776 bytes. GitHub returns 200. All other crawled same-origin published links return 200.
- **Pass:** fresh home and sample request logs are same-origin only; offline reload and sample verification work. Source dependencies show no CLI network client.
- **Pass:** the neo-brutalist evidence-envelope design is distinct, product-specific, consistent with `.factory/design.md`, and uses an original image with provenance. It is not a generic SaaS template.
- **Fail:** metadata, 404, route focus, shared header/footer, checkout, demo, and axe issues are F-1-2 and F-1-4 through F-1-10.

## History verification

There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files. I read `.factory/handoff.md`, `verification.md`, and `verification-2.md`.

- Prior **P1** (missing Linux download): confirmed fixed live and in the build.
- Prior **P2** (skip link focus): fixed only on home; still fails on Privacy and Terms, so it is reopened as blocking F-1-6.
- Prior claims of no console errors, offline reload, no third-party requests on the normal first load, 390 px fit, and low asset size were reconfirmed.
- The handoff’s “No defects were found” conclusion does not survive this stricter first-read/demo/claims/route review.

## Missed leverage

No AI feature is warranted. This is a provenance and integrity tool; model-generated interpretation would not strengthen the signed evidence and could weaken trust. No provider keys are embedded. JSON and HTML export already cover the obvious portability need, and sync would conflict with the local-first scope unless explicitly requested. The obvious missing value is the real one-command sandbox demo already recorded in F-1-2.

## What would make this perfect

Resolve every finding above: make the first phone screen name the job, audience, and sample action; ship the isolated CLI demo and direct demo route; establish and pass the complete claims manifest; repair or remove checkout; provide the product 404 and complete metadata; make navigation/focus/header/footer behavior consistent; fix the axe error; and replace every flagged slogan, jargon phrase, ambiguous control, and untestable claim. A subsequent review must repeat the full checklist from a fresh context and produce zero findings.
