# Action Receipts

Record and verify automated changes in a local receipt file. It is for teams
using agents, scripts, or CI to change repositories and services.

Each event links to the event before it. A signature reveals later changes.
Verification works without a server. A signature does not prove identity,
approval legitimacy, occurrence, intent, or correctness.

## Install

Rust 1.85 or newer builds the single binary.

```sh
cargo install --path .
action-receipts --help
```

Download the Linux x64 binary from the product site, or use `cargo install`.

## Try the demo

Run this command from any directory. It creates a realistic signed receipt in
a new temporary directory and prints the JSON and HTML paths.

```sh
action-receipts demo
```

Open `/demo/` on the product site for the isolated browser sample. It shows a
documentation deployment receipt and stores demo choices under `demo:` only.

## Use your own change

```sh
action-receipts new --out deploy.receipt.json \
  --actor "release-bot@ci" --authorization "change-482 approved" \
  --summary "Publish docs" --scope "repo:docs/**"
action-receipts run --receipt deploy.receipt.json -- npm run build
action-receipts seal --receipt deploy.receipt.json --html deploy.receipt.html
action-receipts verify deploy.receipt.json --json
```

`new` writes a separate signing key with mode 0600. Do not commit that key.
`record` adds a tool result without running a process. `run` records command
arguments, working directory, redacted output, exit status, duration, and file
hashes. Redaction happens before receipt data is written.

## Receipt format

A receipt has `subject`, `policy`, ordered `events`, `chain_head`, and `proof`.
Unknown fields are rejected. For implementers, signatures use Ed25519 over RFC
8785 canonical JSON without `proof.signature`.

The machine-readable contract is [schema/receipt-v1.schema.json](schema/receipt-v1.schema.json).

## Browser verifier and privacy

The browser verifier checks receipt JSON in memory. Selected files are not
uploaded. After one visit, its sample can reload offline. The CLI does not need
an account. See the product privacy and terms pages.

## Test, package, and deploy

```sh
npm ci
npm test
npm run build:site
npm run test:e2e
cargo package --allow-dirty
```

`npm run build:site` creates `dist/site`, including the Linux download.
Publish `dist/site/` as the static site. The factory owns deployment and
registry publishing. The package is ready to publish with `cargo package`.

## License

MIT — see [LICENSE](LICENSE).
