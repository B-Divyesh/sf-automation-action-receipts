# Independent verification 2 — PASS

**Candidate:** `c7ae79a51bf6902844ffc16acf49c3912765b37c` on `main`  
**Live URL:** https://automation-action-receipts.sociobot.in  
**Verified:** 2026-08-28 from a fresh, clean clone at the candidate SHA

## Decision

**PASS.** The product satisfies the researched CLI job: it creates local hash-chained receipts containing declared authorization, scope, structured events, command/artifact provenance and pre-storage redaction; seals JSON and readable HTML with Ed25519; and verifies both offline. The live deployment is an exact byte-level match for the relevant fresh production-build artifacts, including the Linux download that was missing in the prior verification.

This report preserves history: the earlier failed report remains in `verification.md`. Its P1 and P2 are independently confirmed fixed.

## Fresh clean-checkout quality gates

A new local clone was checked out directly at the candidate SHA, then `npm ci` was run. The worktree was clean before testing.

```sh
npm run check
npm run build:site
cargo package --allow-dirty
npm pack --dry-run
```

All commands passed. `npm run check` ran `cargo fmt --check`, Clippy with warnings denied, `tsc --noEmit`, all Rust and Vitest tests, the exact release build, and Playwright. Results were 5 Rust library tests, 1 CLI integration test, 2 browser verifier unit tests, and a passing Playwright run (desktop and 390px projects; the intended desktop skip of the mobile-only assertion is recorded by the suite). The production output contains an executable `dist/site/downloads/action-receipts-linux-amd64` (1,364,776 bytes).

`cargo package --allow-dirty` produced `action-receipts-0.1.0.crate` (147,153 bytes). I extracted it into a separate consumer directory and ran `cargo install --path ... --root ...`; the installed binary reported `action-receipts 0.1.0` and its help exposed the six documented commands: `new`, `record`, `run`, `seal`, `verify`, and `prune`. No publish was attempted. `npm pack --dry-run` also passed (36 files; 236.1 kB unpacked).

## Independent CLI exercise

Using the release binary, I created a receipt with the maximum allowed retention (3650 days), declared actor/authorization/scope and an environment redaction rule. I then recorded a structured Git event with a default-sensitive `token` key and environment secret, ran a command that emitted a literal secret and exited 7 while hashing a 22-byte artifact, sealed JSON plus HTML, and verified both offline with `--json`.

- Both outputs were `valid: true` with two linked events; the command’s exit status was preserved as 7 and its artifact SHA-256/byte count were recorded.
- Neither default secret, environment secret, nor literal secret occurred in the receipt JSON or HTML. The per-receipt private key was mode 0600.
- Altering the sealed summary made `verify --json` exit 3 and return invalid.
- Empty receipt sealing, malformed JSON on an open receipt, invalid `--retention-days 0`, and unconfirmed `prune` each failed safely (respectively exit 1, 1, 2, and 1). Malformed input did not append an event; a subsequent valid record/seal/verify recovered correctly. `prune --dry-run` left files intact.

## Browser, accessibility, privacy, PWA, and performance

The fresh production build was served locally and exercised in Chromium at desktop and 390px mobile widths.

- Loading the signed sample produced “Cryptographically valid”; a modified event produced the clear “Verification failed” state; reloading the sample recovered to valid.
- Axe found **0 serious/critical** findings. Console and page-error listeners recorded none. The first Tab focuses “Skip to content” with a visible `rgb(59, 91, 219) solid 3px` outline; Enter focuses `main`, and the next Tab focuses “Install the CLI”.
- At 390px, scroll and client widths were both 390px (no horizontal overflow); the primary install control measured 362 × 50.8 CSS px. With reduced motion, `.hero-figure` has `transform: none` and transition duration `0.00001s`.
- Recorded first-load browser requests were same-origin only. There are no analytics, telemetry, third-party scripts, or CDN fonts. The CLI has no network client. The explicitly optional license verification endpoint is constrained by CSP to `https://api.sociobot.in` and is not called on a fresh no-license visit.
- After service-worker activation and reload, the worker controlled the page and was active. With the browser offline, reload returned HTTP 200 and still rendered one `main` landmark.
- Lighthouse 13 mobile-style run against the production build: Performance **98**, Accessibility **100**, Best Practices **100**, SEO **100**; LCP 1555 ms, TBT 149 ms, CLS 0.

The delivered initial JS is 10,545 bytes raw / 4,186 gzip and CSS is 12,334 bytes raw / 3,472 gzip, below the 200 kB / 50 kB budgets. The hero WebP is 81,122 bytes; no font files ship.

## Live deployment and response policy

Fresh HTTPS requests returned the advertised download as HTTP 200, `application/octet-stream`, `Content-Length: 1364776`, and immutable one-year caching. The repair is live; it is not merely present in local output.

The live host supplies HSTS, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, a restrictive permissions policy, and CSP limiting resources to self (with only the documented Sociobot API in `connect-src`). HTML is revalidated at 30 seconds, hashed assets/downloads use one-year immutable caching, and `sw.js` is `no-cache`.

Fresh SHA-256 comparisons prove the live release is this candidate build:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `f0543ba9941b3272ba9948c6a2934158a6b066d4d15374f19c2f004a37a95fec` |
| `assets/main-B0PcnFNM.js` | `87826716bb2cb004cf663b18ec7f87901052c2b61b9a32014f6b43c0e1b526db` |
| `assets/main-D4MKtY_B.css` | `9a1ace353743c6f1673159b9884b07747a3b948f4ca5855477714108cce77232` |
| `sw.js` | `9bd9848bce55ea3f709f7ced00f0104b68e7ce83f5b4b0e538edfe2593265a0a` |
| `sample.receipt.json` | `e30bf70cfa4b5ccde629ba8372dcf5b58953af771f8f42818aeac064747cf9a5` |
| `downloads/action-receipts-linux-amd64` | `ef178a18962c8c22fd07ea966bbb5b8b57cf90069ab28113d48ba8001f0da011` |

## Defects

No release-blocking, high, medium, or low defects were found in this candidate.

Known product boundaries, not defects: the downloadable binary is Linux x64 only (other platforms can use `cargo install --path .`), and signatures prove bundle integrity rather than real-world identity, authorization legitimacy, or correctness. These limitations are disclosed in the CLI, documentation, HTML receipt, and terms.
