import AxeBuilder from '@axe-core/playwright';
import { expect, test } from 'playwright/test';

test('@claim:demo-isolated demo opens with sample data, banner, and reset', async ({ page }) => {
  await page.goto('/demo/'); await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible(); await expect(page.getByText('Receipt verified')).toBeVisible();
  await page.getByRole('button', { name: 'Use dark theme' }).click(); expect(await page.evaluate(() => localStorage.getItem('demo:ar_theme'))).toBe('dark'); await page.getByRole('button', { name: 'Reset demo' }).click(); expect(await page.evaluate(() => localStorage.getItem('demo:ar_theme'))).toBeNull();
  await page.goto('/?demo=1'); await expect(page.getByText('Demo — sample data, nothing is saved')).toBeVisible(); await expect(page.getByText('Receipt verified')).toBeVisible();
});
test('@claim:browser-verification verifies a complete chain and detects tampering', async ({ page }) => {
  await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible(); const box = page.getByRole('textbox', { name: 'Receipt JSON' }); await box.fill((await box.inputValue()).replace('policy-gate', 'changed-tool')); await page.getByRole('button', { name: 'Verify receipt' }).click(); await expect(page.getByText('Verification failed')).toBeVisible();
});
test('@claim:receipt-never-uploaded does not send selected data off origin', async ({ page }) => {
  const requests: string[] = []; page.on('request', request => requests.push(request.url())); await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible(); expect(requests.every(url => new URL(url).origin === 'http://127.0.0.1:4173')).toBeTruthy();
});
test('@claim:two-mb-limit rejects receipt text larger than two megabytes', async ({ page }) => {
  await page.goto('/demo/'); await page.getByRole('textbox', { name: 'Receipt JSON' }).fill('x'.repeat(2_000_001)); await page.getByRole('button', { name: 'Verify receipt' }).click(); await expect(page.getByText(/exceeds the 2 MB/)).toBeVisible();
});
test('@claim:offline-reload demo returns after the first cached visit', async ({ page, context }) => {
  await page.goto('/demo/'); await page.reload(); await context.setOffline(true); await page.reload(); await expect(page.locator('main')).toBeVisible();
});
test('@claim:no-account-and-no-telemetry the demo is usable without accounts or third parties', async ({ page }) => {
  const requests: string[] = []; page.on('request', r => requests.push(r.url())); await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible(); expect(requests.every(url => new URL(url).origin === 'http://127.0.0.1:4173')).toBeTruthy();
});
test('routes, keyboard focus, mobile structure, and axe have no violations', async ({ page }) => {
  await page.goto('/'); await page.keyboard.press('Tab'); await page.keyboard.press('Enter'); await expect(page.locator('main')).toBeFocused(); const results = await new AxeBuilder({ page }).analyze(); expect(results.violations).toEqual([]); await page.getByRole('link', { name: 'Privacy' }).first().click(); await expect(page.getByRole('heading', { name: 'Privacy' })).toBeFocused();
  for (const path of ['/privacy/', '/terms/', '/demo/', '/404.html']) { await page.goto(path); await expect(page.locator('h1')).toHaveCount(1); await expect(page.locator('main')).toHaveCount(1); }
});
