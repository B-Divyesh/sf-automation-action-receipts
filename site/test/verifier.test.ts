import { readFile } from 'node:fs/promises';
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
