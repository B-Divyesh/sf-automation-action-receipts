import { readFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { verifyReceipt } from '../src/verifier';

describe('browser receipt verifier', () => {
  it('verifies the signed CLI sample', async () => {
    const receipt = JSON.parse(await readFile('site/public/sample.receipt.json', 'utf8'));
    const result = await verifyReceipt(receipt);
    expect(result.valid).toBe(true);
    expect(result.eventCount).toBe(2);
  });

  it('rejects an event changed after sealing', async () => {
    const receipt = JSON.parse(await readFile('site/public/sample.receipt.json', 'utf8'));
    receipt.events[0].tool = 'changed-tool';
    const result = await verifyReceipt(receipt);
    expect(result.valid).toBe(false);
    expect(result.message).toContain('changed');
  });
});

describe('published repository claims', () => {
  it('@claim:mit-license keeps the README and Terms references aligned with the MIT License', async () => {
    const [license, readme, terms] = await Promise.all([
      readFile(resolve(process.cwd(), 'LICENSE'), 'utf8'),
      readFile(resolve(process.cwd(), 'README.md'), 'utf8'),
      readFile(resolve(process.cwd(), 'site/terms/index.html'), 'utf8'),
    ]);
    expect(license).toContain('Permission is hereby granted, free of charge, to any person obtaining a copy');
    expect(license).toContain('THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND');
    expect(readme).toContain('MIT — see [LICENSE](LICENSE).');
    expect(terms).toContain('provided under the MIT License');
    expect(terms).toContain('provided as-is, without warranty');
  });
});
