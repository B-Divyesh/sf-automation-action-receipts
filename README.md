# Action Receipts

Record and verify automated changes in a local receipt file. It is for teams
using agents, scripts, or CI to change repositories and services.

Each event links to the event before it. A signature detects later changes.
The browser verifier and CLI verify receipts without a server. A signature does
not prove identity, approval legitimacy, occurrence, intent, or correctness.

## Install

Build the binary from this checkout, then inspect the available commands.

```sh
cargo install --path .
action-receipts --help
```

## Try the demo

Run this command from any directory. It creates signed JSON and HTML receipts
in a new temporary directory and prints both paths.

```sh
action-receipts demo
```

Open `/demo/`, or `/?demo=1`, for the isolated browser sample. It loads a
signed documentation deployment receipt using separate `demo:` storage.

## Use your own change

```sh
action-receipts new --out deploy.receipt.json \
  --actor "release-bot@ci" --authorization "change-482 approved" \
  --summary "Publish docs" --scope "repo:docs/**"
action-receipts run --receipt deploy.receipt.json -- npm run build
action-receipts seal --receipt deploy.receipt.json --html deploy.receipt.html
action-receipts verify deploy.receipt.json --json
```

New receipts use a separate private signing key. A command receipt includes
its arguments, result, duration, exit status, and declared file hashes.
Literal and default-key secrets are redacted before receipt data is stored.
The CLI exports signed JSON and a self-contained HTML report. The verifier
rejects receipt JSON with unknown fields.

## Browser verifier and privacy

The browser processes selected receipt text without a data request. After one
visit, the demo can reload offline. The demo makes no third-party requests.
See the product privacy and terms pages before using sensitive receipt data.

## Test, package, and deploy

```sh
npm ci --include=dev
npm test
npm run build:site
npm run test:e2e
cargo package --allow-dirty
```

`npm run build:site` creates `dist/site`. Publish `dist/site/` as the static
site. The factory owns deployment and registry publishing.

## License

MIT — see [LICENSE](LICENSE).
