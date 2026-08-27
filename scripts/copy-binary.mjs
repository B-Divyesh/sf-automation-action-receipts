import { chmod, copyFile, mkdir } from 'node:fs/promises';

await mkdir('dist/site/downloads', { recursive: true });
await copyFile('target/release/action-receipts', 'dist/site/downloads/action-receipts-linux-amd64');
await chmod('dist/site/downloads/action-receipts-linux-amd64', 0o755);
