# Independent verification — FAIL

**Candidate:** `a3f2045f3e17b31c0c6eba7ecf033836af6141fa` on `main`  
**Live URL:** https://automation-action-receipts.sociobot.in  
**Verified:** 2026-08-27 from a clean checkout at the candidate SHA

## Decision

**FAIL.** The locally built candidate is functional, but the live deployment
does not publish the Linux binary that its own install page advertises. This is
a production release mismatch, not a local build failure. The candidate build
contains `dist/site/downloads/action-receipts-linux-amd64` (1,364,776 bytes),
while the live URL for that exact linked path returns HTTP 404.

## Reproduction of the blocker

```sh
npm run build
test -x dist/site/downloads/action-receipts-linux-amd64
# succeeds

curl -I https://automation-action-receipts.sociobot.in/downloads/action-receipts-linux-amd64
# HTTP/2 404
```

The landing page link is `/downloads/action-receipts-linux-amd64`; a fresh
390px browser visit also received 404 for it. The normal live verifier still
loads and validates the signed sample, so this is a scoped deployment artifact
omission rather than a stale whole-site deployment.

## What passed

### Clean install, static checks, tests, and build

All commands were run from the clean candidate checkout:

```sh
npm ci
cargo fmt --check
cargo clippy --all-targets -- -D warnings
npm test
npm run build
npm run test:e2e
```

They passed. `npm test` reported 5 Rust library tests, 1 CLI integration test,
and 2 browser-verifier unit tests. Playwright reported 12 tests passed across
desktop/mobile (including axe checks; one desktop instance of the mobile-only
assertion is skipped by design). No separate TypeScript type/lint command is
defined in `package.json`.

### CLI and package consumer exercise

The release binary and a clean consumer install were independently exercised:

```sh
cargo package --allow-dirty
tar -xzf target/package/action-receipts-0.1.0.crate -C <clean-dir>
cargo install --path <clean-dir>/action-receipts-0.1.0 --root <clean-prefix>
<clean-prefix>/bin/action-receipts --help
```

`cargo package` verified 38 files (273.8 KiB / 139.2 KiB compressed). The
installed public binary reported `action-receipts 0.1.0` and exposed `new`,
`record`, `run`, `seal`, `verify`, and `prune`. `npm pack --dry-run` also
completed (34 files, 125.4 KiB compressed); no publishing was attempted.

Manual end-to-end receipt exercise covered a retained upper boundary (3650
days), default sensitive-key redaction, `--redact-env`, literal command-output
redaction, structured event recording, a command exiting 7, artifact-free
seal, and offline JSON/HTML verification. The sealed bundle had two events and
contained none of the supplied secret values. JSON and HTML verification
exited 0 and returned `valid: true`; a changed summary returned `valid: false`
and exit 3.

Negative/recovery paths behaved as follows: `--retention-days 0` exited 2;
malformed `--input-json` exited 1 with a clear JSON error without recording an
event; empty receipts cannot seal; appending to a sealed receipt is refused;
and `prune` refuses deletion without `--confirm` (or `--dry-run`).

### Browser, accessibility, PWA, and privacy checks

Against the production build on a local Vite preview:

- Desktop sample verification, malformed-paste invalid state, and recovery by
  reloading the signed sample all worked; there were zero page errors and zero
  console errors.
- Axe found **zero serious or critical** issues. The first keyboard Tab focuses
  the visible skip link with a `3px` blue outline. At 390px there is no
  horizontal overflow (`390px` scroll/client width); primary controls measure
  about 362 x 51px. Under reduced motion the hero transform is `none` and
  button transitions are `0.00001s`.
- Normal browsing made only same-origin document/script/style/image requests
  plus the explicit same-origin sample receipt fetch. No analytics, telemetry,
  CDN fonts, or third-party scripts were observed. The optional license API is
  only contacted when a stored/returned license exists.
- The worker installed and became controlling after a reload; with networking
  disabled, a subsequent reload returned 200 and rendered the page/main from
  cache. `registration.update()` completed with an active worker. Live
  `/sw.js` is served `Cache-Control: no-cache`.
- Lighthouse 13 mobile, against the production build: Performance **100**,
  Accessibility **100**, Best Practices **100**, SEO **100**; LCP 1504.5 ms,
  TBT 0 ms, CLS 0.

Built browser assets meet the stated budget: JS 10,272 bytes raw (4,080 gzip),
CSS 12,334 bytes raw (3,460 gzip), and the LCP illustration is 81,122 bytes
WebP. No font files are shipped.

### Live-deployment identity and response policy

Fresh SHA-256 comparisons showed the live `/`, main JS, CSS, `/sw.js`, and
`/sample.receipt.json` are byte-for-byte equal to this candidate's build:

| File | SHA-256 |
| --- | --- |
| `index.html` | `494ad3844f3297e19037ef70f415b7816aed4974a9a60097a6c1ec4ed62e8402` |
| `assets/main-jtQcCfkb.js` | `e6f4cd02a7c8dbe24df51e83a9c1a3bd705b35a4ac29bf652c00f3de635fab42` |
| `assets/main-D4MKtY_B.css` | `9a1ace353743c6f1673159b9884b07747a3b948f4ca5855477714108cce77232` |
| `sw.js` | `9bd9848bce55ea3f709f7ced00f0104b68e7ce83f5b4b0e538edfe2593265a0a` |

The live host returns HTTPS, HSTS, `X-Content-Type-Options: nosniff`,
`Referrer-Policy: no-referrer`, restrictive permissions policy, and a CSP that
permits only self resources plus the documented Sociobot API connection.
Hashed JS/CSS and the image are immutable for one year; HTML is revalidated at
30 seconds; and the worker is no-cache.

## Defects

### P1 — advertised production binary is missing

**Evidence:** `dist/site/downloads/action-receipts-linux-amd64` is built and
executable locally, but live
`https://automation-action-receipts.sociobot.in/downloads/action-receipts-linux-amd64`
returns 404. The visible “Download Linux x64” link targets this path.

**Impact:** A user following the supported no-build install route cannot obtain
the CLI. Publish the `downloads/` directory from `dist/site/` and rerun the
live smoke test before releasing.

### P2 — skip link does not move keyboard focus into main content

**Evidence:** Keyboard Tab exposes the skip link and Enter changes the URL to
`#main` and scrolls to the main landmark, but `document.activeElement` is
`BODY`, not the main landmark or its first control. The next keyboard Tab can
therefore resume header navigation instead of continuing in main content.

**Impact:** Keyboard users do not get the expected bypass-blocks behavior.
Make the main target programmatically focusable (for example, `tabindex="-1"`)
and focus it when the skip link is activated, then regression-test keyboard
tab order. This is not the deployment blocker but must be resolved for the
stated keyboard accessibility baseline.

## Retest instructions

After publishing the built download artifact and fixing the skip-focus path:

```sh
npm ci && cargo fmt --check && cargo clippy --all-targets -- -D warnings
npm test && npm run build && npm run test:e2e
curl -fI https://automation-action-receipts.sociobot.in/downloads/action-receipts-linux-amd64
```

Then repeat a keyboard-only skip-link test and the signed sample/tamper browser
smoke test on the live host.
