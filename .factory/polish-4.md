# Polish 4 — cumulative zero-finding closure

Release candidate `aee7006` was repaired in `46e7f24` and `4e7a91f`. The
final deployment is `31c36be8-b125-4d3f-8209-ea2919e3dd8a` at
`https://automation-action-receipts.sociobot.in`.

## Evidence key

- **C**: final clean clone `/tmp/action-receipts-polish4-final.ztX7KS/repo`;
  `npm run check` log at `/work/.evidence/automation-action-receipts-polish-4/final-clean-check.log`.
- **Q**: all 19 exact claim commands passed; logs are
  `/work/.evidence/automation-action-receipts-polish-4/final-claim-<id>.log`.
- **H**: `live-home-desktop.png` and `live-home-mobile.png`; live `/`.
- **D**: `live-demo-mobile.png`; live `/?demo=1` and `/demo/`.
- **M**: `live-mobile-menu.png`; live `/` at 390 × 844.
- **T**: `live-terminal-capture.png`; live `/#install`.
- **L**: `live-privacy.png`, `live-terms.png`, and `live-404.png`; live legal
  routes and an unknown URL.
- **A**: `live-audit.json`; cold geometry, storage, requests, offline,
  history focus, assets, 404, binary, and both-theme Axe results.
- All screenshot/audit paths above are under
  `/work/.evidence/automation-action-receipts-polish-4/`.

## Finding map

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Direct job headline, audience, primary sample action, and result note. | first-screen geometry test; H; `/`. |
| F-1-2 | Real isolated browser/CLI demos and an actual release-binary capture with raw source. | `demo-isolated`, `terminal-recording`, `cli-demo-lifecycle`; D/T; `/demo/`, `/#install`. |
| F-1-3 | Nineteen unique manifest claims with one observable test each. | Q; A; live demo/download. |
| F-1-4 | Dead paid tier and checkout remain removed. | copy audit; H; `/`. |
| F-1-5 | Product 404 and response override. | metadata/routing test; L/A; unknown live URL returns 404. |
| F-1-6 | Focusable mains and working skip links on every route. | route/focus/Axe test; H/L; all routes. |
| F-1-7 | Real 180 × 180 PNG Apple icon on every route, manifest, and cache. | metadata/routing claim; A; all routes. |
| F-1-8 | BFCache/pageshow Back/Forward h1 focus and announcement. | route/focus/Axe test; A; Home ↔ Privacy. |
| F-1-9 | Shared header/footer, legal links, source label, factory and version lines. | metadata/routing claim; H/D/L; all routes. |
| F-1-10 | Integrity note is not a landmark; Axe runs every route in both themes. | route/focus/Axe test; A; all routes. |
| F-1-11 | “Record and verify automated changes.” | copy audit; H; `/`. |
| F-1-12 | Short sentence names teams using agents, scripts, or CI. | copy audit; H; `/`. |
| F-1-13 | User benefit precedes signing-algorithm detail. | copy audit; H; `/`. |
| F-1-14 | Plain approval → command → file hash → signature sequence. | copy audit; H; `/`. |
| F-1-15 | “Verify a signed receipt.” | copy audit; H; `/#verify`. |
| F-1-16 | Plain on-device event/signature explanation. | browser-verification claim; H; `/#verify`. |
| F-1-17 | “Create, record, sign, and verify.” | copy audit; H; `/#protocol`. |
| F-1-18 | “Record the approved scope.” | declared-boundary claim; H; `/#protocol`. |
| F-1-19 | “Record a command or tool result.” | command-provenance claim; H; `/#protocol`. |
| F-1-20 | “Sign and export the receipt.” | JSON/HTML claim; H; `/#protocol`. |
| F-1-21 | “Run the sample or use your own change.” | copy audit; T; `/#install`. |
| F-1-22 | Accurate “Command-line quick start” label. | copy audit; T; `/#install`. |
| F-1-23 | Removed unavailable pricing section and mood copy. | copy audit; H; `/`. |
| F-1-24 | Removed vague paid-plan label. | copy audit; H; `/`. |
| F-1-25 | Removed unavailable policy-kit promises. | copy audit; H; `/`. |
| F-1-26 | Removed false checkout statement. | copy audit; H; `/`. |
| F-1-27 | Concrete “Export formats” label. | copy audit; H; `/`. |
| F-1-28 | Direct JSON and self-contained HTML statement. | JSON/HTML claim; H; `/`. |
| F-1-29 | Footer states the product job. | copy audit; H/L; all routes. |
| F-1-30 | Result-naming theme control. | route/focus/Axe test; H/M; `/`. |
| F-1-31 | “Try it with sample data.” | demo-isolated claim; H/D; `/`. |
| F-1-32 | “Copy commands.” | copy audit; T; `/#install`. |
| F-1-33 | “Install the CLI.” | copy audit; H; `/`. |
| F-1-34 | Closing link accurately targets installation. | copy audit; H; `/`. |
| F-1-35 | Short task-led README opening. | copy audit; C; repository README. |
| F-1-36 | Plain linked-event and change-detection wording. | browser-verification claim; C; README. |
| F-1-37 | Direct product-site download wording. | linux-download claim; T/A; live download. |
| F-1-38 | Removed implementation-library exit wording. | copy audit; C; README. |
| F-1-39 | Removed vague public-surface claim. | copy audit; C; README. |
| F-1-40 | Removed dense untested canonicalization copy. | copy audit; C; README. |
| F-1-41 | README says browser checks JSON on-device. | receipt-never-uploaded claim; D; demo. |
| F-1-42 | Uses “selected,” not “uploaded,” files. | receipt-never-uploaded claim; D; demo. |
| F-1-43 | Separate short privacy and offline statements. | offline/no-upload claims; D/A; demo. |
| F-1-44 | Full demo lifecycle records, signs, exports, and verifies. | CLI lifecycle claim; Q/A; live binary. |
| F-1-45 | Account promise narrowed to the credential-free CLI demo. | cli-no-account claim; H; `/`. |
| F-1-46 | Broad telemetry text replaced by same-origin demo request claim. | no-third-party claim; D/A; demo. |
| F-1-47 | CLI network lock and cached browser verification. | local-verification claim; D/A; demo. |
| F-1-48 | Select/paste/verify sends no data request. | receipt-never-uploaded claim; D/A; demo. |
| F-1-49 | Early-event tampering must fail browser verification. | browser-verification claim; D; demo. |
| F-1-50 | 2,000,000-character browser limit is asserted. | two-mb-limit claim; D; demo. |
| F-1-51 | Actor, approval, scope, retention, and order asserted. | declared-boundary claim; Q; sample. |
| F-1-52 | Provenance and pre-storage redaction have outcome tests. | command-provenance/redaction claims; Q; sample. |
| F-1-53 | JSON and HTML decode to the same signed receipt. | JSON/HTML claim; Q/A; live binary. |
| F-1-54 | Integrity limitation remains explicit. | browser-verification claim; H/L; protocol/Terms. |
| F-1-55 | Paid gating removed; shipped core remains ungated. | C/copy audit; H; `/`. |
| F-1-56 | Unprovable “Forever” removed. | copy audit; H; `/`. |
| F-1-57 | “Unlimited” removed. | copy audit; H; `/`. |
| F-1-58 | Unavailable Team kit removed. | copy audit; H; `/`. |
| F-1-59 | Price and future-update promises removed. | copy audit; H/L; Home/Terms. |
| F-1-60 | Checkout and merchant statements removed. | copy audit; H; `/`. |
| F-1-61 | Untested Rust-minimum sentence removed. | copy audit; C; README. |
| F-1-62 | Live download checks HTTP, ELF64 x86-64, version, demo, and verification. | linux-download claim; T/A; live download. |
| F-1-63 | Separate key and Unix mode 0600 asserted. | private-key claim; Q; README. |
| F-1-64 | Arguments, result, duration, status, and file SHA asserted. | command-provenance claim; Q; README. |
| F-1-65 | Untested integration/no-process promise removed. | copy audit; C; README. |
| F-1-66 | Broad CI/exit/no-stdin promises removed. | copy audit; C; README. |
| F-1-67 | Untested detailed v1/canonicalization promise removed. | copy audit; C; README. |
| F-1-68 | Unknown receipt fields are rejected. | unknown-fields claim; Q; README. |
| F-1-69 | Default-key and literal secrets never reach saved text. | redaction claim; Q; README. |
| F-1-70 | Broader retention-enforcement promise removed. | copy audit; C; README. |
| F-1-71 | Duplicate redaction/retention marketing removed. | copy audit; H; `/`. |
| F-1-72 | Cached demo reloads and verifies offline. | offline-reload claim; D/A; demo. |
| F-1-73 | Self-referential coverage claim removed. | copy audit; C; README. |
| F-1-74 | Build claim inspects routes, metadata, offline assets, and executable. | site-build-output claim; Q; live site. |
| F-1-75 | Network wording narrowed to tested offline/no-request behavior. | local/no-upload claims; D/A; demo. |
| F-1-76 | `demo:` namespace, reset, and exit cleanup are exact. | demo-isolated claim; D/A; Privacy/Demo. |
| F-1-77 | License storage/frequency text removed. | copy audit; L; Privacy. |
| F-1-78 | Unprovable host-retention text removed. | copy audit; L; Privacy. |
| F-1-79 | Complete demo request log must stay same-origin. | no-third-party claim; D/A; demo. |
| F-1-80 | Refund/license promise removed. | copy audit; L; Terms. |
| F-2-1 | Separate namespace, real-key preservation, and 390 px reflow. | demo-isolated claim; D/A; demo. |
| F-2-2 | Verified two-event sample is visible before scrolling. | demo-isolated claim; D/A; query demo. |
| F-2-3 | No-third-party test covers the full demo request list. | no-third-party claim; D/A; demo. |
| F-2-4 | Exact separate-storage wording and real exit cleanup. | demo-isolated claim; D/A; demo. |
| F-2-5 | Local receipt-file fact maps to temp JSON/HTML. | CLI lifecycle claim; T/A; live binary. |
| F-2-6 | Boundary statement maps to receipt-field assertions. | declared-boundary claim; Q; sample. |
| F-2-7 | Output/file-hash statement maps to provenance. | command-provenance claim; Q; sample. |
| F-2-8 | Pre-storage wording maps to secret absence. | redaction claim; Q; README. |
| F-2-9 | Export wording maps to JSON/HTML equivalence. | JSON/HTML claim; Q/A; live binary. |
| F-2-10 | Retained README capabilities are listed; unsupported ones removed. | Q/copy audit; repository README. |
| F-2-11 | Privacy/offline text maps to request and cached-offline tests. | no-upload/offline claims; D/A; demo. |
| F-2-12 | Jargon-only eyebrow removed. | copy audit; H; `/`. |
| F-2-13 | Jargon-only sentence replaced with task language. | copy audit; H; `/`. |
| F-3-1 | Product social card is exactly 1200 × 630. | metadata/routing claim; H; all routes. |
| F-3-2 | Local-file fact is mapped to CLI lifecycle. | CLI lifecycle claim; H; `/`. |
| F-3-3 | Account fact is narrowed and mapped. | cli-no-account claim; H; `/`. |
| F-3-4 | No-server statement maps to network lock and offline browser. | local-verification claim; D/A; demo. |
| F-4-1 | No mid-word heading breaks; all facts remain above both folds. | first-screen geometry test; H/A; `/`. |
| F-4-2 | Start for real clears all demo keys and preserves real keys. | demo-isolated claim; D/A; demo. |
| F-4-3 | Static cursor; SVG contains no animation. | reduced-motion test; T/A; live SVG. |
| F-4-4 | Keyboard mobile menu exposes all routes and returns focus on Escape. | mobile-menu test; M/A; `/`. |
| F-4-5 | Future-maintenance sentence and empty heading removed. | copy audit; L/A; Terms. |

## Final verification

- Final clean commit: `4e7a91fae38684ee1a3e0116fb0e8377ce4afd26`.
- `npm run check`: PASS — 5 Rust unit, 9 CLI integration, 2 verifier, and
  29 Playwright passes; one desktop skip is the mobile-only menu case.
- All 19 exact claim commands: PASS independently in the final clean clone.
- `cargo package --allow-dirty`: PASS; 517.5 KiB, 259.5 KiB compressed.
- Live Lighthouse: 100 performance, accessibility, best practices, and SEO;
  LCP 1.2 s, CLS 0, TBT 30 ms.
- Live audit: no console errors; zero Axe violations on five routes in both
  themes; 404 response correct; live ELF64 x86-64 binary ran version 0.1.0.

No finding remains open.
