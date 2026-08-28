# Demo sandbox

- Browser demo: `/demo/` or `/?demo=1`. It loads the signed documentation-deployment sample and displays the persistent demo banner.
- CLI demo: `action-receipts demo`. It creates a new OS temporary directory, writes a signed JSON receipt and self-contained HTML report, and prints both paths.
- Reset: choose **Reset demo**. Browser demo state uses the `demo:` localStorage namespace and is discarded. CLI output is isolated from the caller's directory.
- The browser sample is `site/public/sample.receipt.json`; the CLI sample models an approved documentation deployment with a policy check and build event.
