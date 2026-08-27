# Action Receipts

Action Receipts is a local-first CLI and open JSON format for answering four
questions about an automated change: what ran, with which inputs, against what
declared scope, and under whose stated authorization. Events are SHA-256 hash
chained; sealed bundles are signed with Ed25519 and verify without a server.

It is for teams that let agents, scripts, or CI change repositories and
services. A valid receipt proves that the bundle has not changed since it was
signed. It does **not** prove the actor's real-world identity, that the action
was correct, or that the stated authorization was legitimate.

## Install

Build the single binary with Rust 1.85 or newer:

```sh
cargo install --path .
action-receipts --help
```

Prebuilt Linux binaries are also exposed on the product site build under
`downloads/`. Registry publishing is performed by the factory, not this repo.

## Usage

Create an open receipt. The signing key is written separately with mode 0600;
do not commit it.

```sh
action-receipts new \
  --out deploy.receipt.json \
  --actor "release-bot@ci" \
  --authorization "change-482 approved by ops" \
  --summary "Publish docs site" \
  --scope "repo:docs/**" \
  --retention-days 30 \
  --redact-env DEPLOY_TOKEN
```

Run a command and record its arguments, working directory, redacted output,
exit status, duration, and artifact digest:

```sh
action-receipts run \
  --receipt deploy.receipt.json \
  --artifact dist/site/index.html \
  -- npm run build
```

Declared integrations can append structured events without running a process:

```sh
action-receipts record \
  --receipt deploy.receipt.json \
  --kind tool \
  --tool git \
  --input-json '{"operation":"push","ref":"main"}' \
  --output-json '{"commit":"abc123"}'
```

Seal both portable JSON and a self-contained readable HTML report, then verify
either one offline:

```sh
action-receipts seal --receipt deploy.receipt.json --html deploy.receipt.html
action-receipts verify deploy.receipt.json
action-receipts verify deploy.receipt.html --json
```

`verify --json` is stable for CI. It exits `0` when valid, `3` when integrity
verification fails, and `1` for an I/O/runtime error. Clap usage errors exit
`2`. All commands are non-interactive.

### Public receipt format (v1)

The public surface is intentionally small. A receipt contains `subject`,
`policy`, ordered `events`, `chain_head`, and (after sealing) `proof`. Each event
hash covers its sequence, timestamp, kind, tool, redacted data, artifact hashes,
and previous hash. The Ed25519 signature covers the canonical receipt without
the `proof.signature` field using RFC 8785 JSON Canonicalization (JCS). Unknown
fields are rejected by this v1 verifier.

The machine-readable contract is [schema/receipt-v1.schema.json](schema/receipt-v1.schema.json).

Redaction is applied before anything is stored. Sensitive JSON key names are
redacted by default; add literal values with `--redact` or environment values
with `--redact-env`. Retention is declared in every receipt and can be enforced
locally with `action-receipts prune --dir receipts --older-than 30 --dry-run`.

## Landing site and browser verifier

The Vite site documents the protocol and verifies receipt JSON locally using
Web Crypto. Uploaded receipts never leave the device. It includes offline shell
caching, privacy and terms pages, and the optional Sociobot license unlock for
the Team policy kit.

```sh
npm install
npm run dev
npm test
npm run build       # -> dist/site and the CLI binary download
```

## Test and package

```sh
cargo test --all-targets
cargo package --allow-dirty
npm test
```

Tests cover the documented lifecycle, redaction, artifact hashing, HTML
extraction, tamper detection, and browser verifier behavior.

## Deploy

Run `npm run build`; publish `dist/site/` as a static site. The factory owns
deployment, product registration, DNS, and registry credentials.

## Privacy and security

There is no telemetry. CLI data stays on disk. The browser verifier processes
files in memory. Receipts can still expose commands, paths, and output, so use
redaction and a retention window appropriate to the data. See `/privacy/` and
`/terms/` on the site.

## License

MIT — see [LICENSE](LICENSE).
