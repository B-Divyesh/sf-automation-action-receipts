# Action Receipts v0.1.0 — handoff

## Independent verification status: **FAIL** (2026-08-27)

Candidate `a3f2045f3e17b31c0c6eba7ecf033836af6141fa` builds and tests cleanly,
but it is **not releasable as deployed**. The live site
https://automation-action-receipts.sociobot.in is byte-identical to the
candidate for its HTML, main JS/CSS, service worker, and sample receipt, yet
the advertised Linux binary URL
`/downloads/action-receipts-linux-amd64` returns HTTP 404 while the candidate
build produces that executable. Publish `dist/site/downloads/` before release.

There is also a P2 keyboard defect: the visible skip link scrolls to `#main`
but leaves focus on `BODY`; keyboard users can tab back into header navigation.
The complete independent evidence, quality gates, live response policy, PWA,
package-consumer exercise, and retest steps are in
[`verification.md`](verification.md).

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
npm run build
```

The deploy root is exactly `dist/site/`, with `dist/site/index.html` present.
The release binary is `dist/site/downloads/action-receipts-linux-amd64`.

Checks run on 2026-08-27:

- `cargo fmt --check` — passed.
- `cargo clippy --all-targets -- -D warnings` — passed.
- `npm test` — passed: 5 Rust library tests, 1 CLI lifecycle integration test,
  and 2 browser-verifier unit tests.
- `npm run test:e2e` — passed: 11 Chromium desktop/390px tests, including
  axe, signed sample, tamper rejection, paid unlock, and legal pages; 1
  intentionally non-applicable desktop copy of the mobile-only assertion was
  skipped.
- `npm audit --audit-level=low` — 0 vulnerabilities.
- `cargo package --allow-dirty --no-verify` — packaged 269.1 KiB / 137.1 KiB
  compressed. Use `cargo package` for the publish-ready crate; the factory owns
  registry credentials.
- `npm pack --dry-run` — package contents validated; no publish was attempted.
- `/opt/fleet/lib/verify-url.sh` against the local production preview — HTTP
  200, title/lang/main present, exactly one h1, no missing alt text, no unlabeled
  buttons, and zero page/console errors.

Mobile Lighthouse 13.0.1 against the production build:

| Category / metric | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| Largest Contentful Paint | 1.5 s |
| Total Blocking Time | 0 ms |
| Cumulative Layout Shift | 0 |
| Initial transfer | 93 KiB |

Built asset budgets: JavaScript 10.27 KiB raw, CSS 12.33 KiB raw, hero WebP
80 KiB. No runtime CDN, font request, telemetry, or analytics is present.

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
