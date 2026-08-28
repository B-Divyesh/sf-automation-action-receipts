# Action Receipts v0.1.0 — repair handoff

## Independent verification repair status: **PASS and deployed** (2026-08-28)

This repair addresses every release-blocking finding in the independent report
for candidate `a3f2045f3e17b31c0c6eba7ecf033836af6141fa` (report commit
`41b397f52ba3822c31143b0452c7490de165472f`). The original report remains at
[`verification.md`](verification.md).

- **P1 — missing production binary:** The static deployment contract calls
  `npm run build:site`, but that script previously ran only Vite; the Linux
  executable was copied only by a separate `npm run build` path. `build:site`
  now builds the release Rust binary, runs Vite, then copies the executable
  into `dist/site/downloads/` after Vite has emptied the output directory.
  The deploy configuration now also gives `/downloads/*` immutable caching.
  The built file is executable (`mode 755`) and 1,364,776 bytes. A desktop and
  390px-browser regression asserts the visible link target, HTTP 200, and a
  non-empty binary response at that exact URL.
- **P2 — skip link focus:** `main#main` is programmatically focusable and the
  skip-link activation now moves focus there while preserving `#main` and
  scrolling to the landmark. A desktop and 390px keyboard regression confirms
  Tab → Enter focuses main, and the next Tab reaches the first main-content
  link rather than returning to header navigation.
- **Type safety:** Added a strict TypeScript check to the release gate and
  pinned `playwright-core` to the already pinned Playwright 1.58.2, avoiding a
  mismatched transitive 1.62 type surface. The Web Crypto base64 bytes are now
  explicitly backed by `ArrayBuffer`, satisfying the current DOM crypto types
  without changing verification behavior.

## What shipped

- A Rust single-binary CLI with `new`, `record`, `run`, `seal`, `verify`, and
  `prune` commands. It uses SHA-256 event chains, RFC 8785 canonical JSON,
  Ed25519 detached signatures, atomic receipt writes, and 0600 private-key
  permissions on Unix.
- Configurable pre-storage redaction for default sensitive JSON keys, literal
  values, and named environment variables; output capture is capped at 64 KiB.
- Artifact hashing, declared authorization/scope, a receipt-level retention
  policy, and confirmed/dry-run retention pruning.
- Portable JSON plus self-contained readable HTML receipts. Either format can
  be verified offline by the CLI with stable JSON output and exit codes.
- The public v1 JSON Schema at `schema/receipt-v1.schema.json` and a CLI-signed
  sample used by both automated tests and the browser demo.
- A responsive Vite site with an actual Web Crypto receipt verifier, explicit
  empty/loading/invalid/offline states, dark and light treatments, privacy and
  terms pages, install docs, and a downloadable Linux x64 binary.
- A $39 one-time Team policy kit using only the Sociobot hosted checkout and
  license verification contract. It supports return-token capture, once-daily
  verdict caching, offline cached unlock, restore, removal, and local policy
  JSON generation. The free verifier/export/safety functions remain ungated.
- A generated, product-specific 1200×800 WebP hero illustration (80 KiB) at
  `site/public/receipt-chain.webp`; prompt and factory-image provenance are in
  `.factory/design.md`.

## Build and verification

From a clean clone:

```sh
npm ci
npm test
npm run check
```

The deploy command is `npm run build:site` (also the target of `npm run build`)
and the deploy root is exactly `dist/site/`, with `dist/site/index.html` and
`dist/site/downloads/action-receipts-linux-amd64` present.

Checks run from a fresh `npm ci` on 2026-08-28:

- `cargo fmt --check` — passed.
- `cargo clippy --all-targets -- -D warnings` — passed.
- `npm run typecheck` — passed (`tsc --noEmit`).
- `npm test` — passed: 5 Rust library tests, 1 CLI lifecycle integration test,
  and 2 browser-verifier unit tests.
- `npm run check` — passed: all static/type/test/build gates plus Playwright:
  15 passed across Chromium desktop and 390px mobile; 1 intentionally
  non-applicable desktop copy of the mobile-only assertion was skipped.
- `npm audit --audit-level=low` — 0 vulnerabilities.
- `cargo package --allow-dirty` — verified and packaged 40 files, 284.2 KiB /
  142.7 KiB compressed. A fresh extracted consumer install exposed the public
  `action-receipts` binary and all six documented commands. Use `cargo package`
  for the publish-ready crate; the factory owns registry credentials.
- `npm pack --dry-run` — validated 36 files, 233.2 KiB unpacked / 129.6 KiB
  compressed; no publishing was attempted.
- `/opt/fleet/lib/verify-url.sh` against the local production preview — HTTP
  200, title/lang/main present, exactly one h1, no missing alt text, no unlabeled
  buttons, and zero page/console errors. The local deployment URL returned the
  advertised binary with HTTP 200 and `Content-Length: 1364776`.
- PWA/privacy smoke: after activation and reload, the worker controlled the
  page; an offline reload returned HTTP 200 with one main landmark;
  `registration.update()` retained an active worker; and a fresh no-license
  visit made no cross-origin requests. No telemetry, CDN font, or third-party
  script is used.

Mobile Lighthouse 13.0.1 against the repaired production build:

| Category / metric | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| Largest Contentful Paint | 1.4 s |
| Total Blocking Time | 20 ms |
| Cumulative Layout Shift | 0 |
| Initial transfer | 95 KiB |

Built asset budgets: JavaScript 10.55 KiB raw, CSS 12.33 KiB raw, hero WebP
81.12 KiB, and Linux download 1.364 MiB. No runtime CDN, font request,
telemetry, or analytics is present.

## Deployment and live confirmation

The product-source repair commit `c98cb09` was pushed to `main` and deployed
with the work-order static configuration exactly as specified:

```sh
npm ci && npm run build:site
/opt/fleet/lib/deploy-static.sh automation-action-receipts dist/site
```

Azure Static Web Apps deployment `a0ce05dd-0d2e-4143-bb78-9493610e8adf`
succeeded (1,519,085-byte upload); the custom domain was Ready and HTTPS was
200. Live verification at `https://automation-action-receipts.sociobot.in`
found zero console errors and the required title, language, one h1, main
landmark, and image/button labels.

- `GET /downloads/action-receipts-linux-amd64` is **200** with
  `Content-Type: application/octet-stream`, `Content-Length: 1364776`,
  immutable one-year cache control, HSTS, CSP, `nosniff`, and the intended
  referrer and permissions policies.
- Live `index.html`, main JS/CSS, `sw.js`, sample receipt, and the Linux binary
  matched the local deploy output byte-for-byte by SHA-256. The binary hash is
  `ef178a18962c8c22fd07ea966bbb5b8b57cf90069ab28113d48ba8001f0da011`.
- A live keyboard check confirmed Skip to content → focused `main#main` with
  `#main`, then Tab → “Install the CLI”.

## Known boundaries

- The bundled download is Linux x64 only. macOS/Windows users can build the
  single binary with `cargo install --path .`; release automation can add
  cross-compiled artifacts later.
- The factory must register the paid product slug before checkout and live
  license verification will succeed. No product ID or payment-provider code is
  embedded here.
- Browser verification requires evergreen Web Crypto Ed25519 support; the CLI
  remains the offline fallback.
- A receipt proves signed-bundle integrity only. It cannot prove that an event
  happened, that a declared actor is a real identity, that authorization was
  legitimate, or that the work was correct. This limitation is repeated in the
  CLI, HTML export, site, README, and terms.

## Suggested next steps

- Add factory release jobs for macOS arm64/x64 and Windows x64 binaries.
- Register `automation-action-receipts` with the Sociobot billing factory and
  smoke-test the production checkout return URL.
- Pilot the open format against real CI and agent runs, then use reviewer
  outcomes to decide which team policy checks belong in a future v1.1.
