# Demo sandbox

- Browser demo: `/demo/` or `/?demo=1`. The query route redirects into `/demo/`, where the signed documentation-deployment sample is already verified above the fold.
- CLI demo: `action-receipts demo`. It creates a new OS temporary directory, writes a signed JSON receipt and self-contained HTML report, and prints both paths.
- Reset: choose **Reset demo**. Browser demo state uses the `demo:` localStorage namespace and is discarded without changing non-demo storage.
- Exit: choose **Start for real**. It clears every `demo:` key before opening Home and leaves non-demo keys unchanged.
- CLI isolation: demo output is written to a new OS temporary directory, never the caller's directory.
- The browser sample is `site/public/sample.receipt.json`; the CLI sample models an approved documentation deployment with a policy check and build event.
