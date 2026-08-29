import AxeBuilder from '@axe-core/playwright';
import { expect, test } from 'playwright/test';

const origin = 'http://127.0.0.1:4173';

// @claim:demo-isolated
test(' @claim:demo-isolated demo opens in a separate namespace and resets only demo data', async ({ page }) => {
  await page.goto('/demo/');
  await expect(page.getByText('Demo — sample data, separate demo storage')).toBeVisible();
  await expect(page.getByText('Receipt verified')).toBeVisible();
  await expect(page.locator('#verification-result')).toBeInViewport();
  expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(await page.evaluate(() => document.documentElement.clientWidth));
  await page.evaluate(() => localStorage.setItem('ar_real_marker', 'keep'));
  await page.getByRole('button', { name: 'Use dark theme' }).click();
  expect(await page.evaluate(() => localStorage.getItem('demo:ar_theme'))).toBe('dark');
  await page.getByRole('button', { name: 'Reset demo' }).click();
  expect(await page.evaluate(() => ({ demo: localStorage.getItem('demo:ar_theme'), real: localStorage.getItem('ar_real_marker') }))).toEqual({ demo: null, real: 'keep' });
  await page.goto('/?demo=1');
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText('Receipt verified')).toBeVisible();
});

// @claim:browser-verification
test('@claim:browser-verification verifies a complete chain and detects tampering', async ({ page }) => {
  await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible();
  const box = page.getByRole('textbox', { name: 'Receipt JSON' });
  await box.fill((await box.inputValue()).replace('policy-gate', 'changed-tool'));
  await page.getByRole('button', { name: 'Verify receipt' }).click();
  await expect(page.getByText('Verification failed')).toBeVisible();
});

// @claim:receipt-never-uploaded
test('@claim:receipt-never-uploaded processes selected receipt text without a data request', async ({ page }) => {
  const requests: string[] = []; page.on('request', request => requests.push(request.url()));
  await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible();
  const countBefore = requests.length;
  const box = page.getByRole('textbox', { name: 'Receipt JSON' });
  const receipt = await box.inputValue();
  await box.fill(receipt); await page.getByRole('button', { name: 'Verify receipt' }).click();
  await page.getByLabel('Choose a receipt JSON').setInputFiles({ name: 'receipt.json', mimeType: 'application/json', buffer: Buffer.from(receipt) });
  await expect(page.getByText('Receipt verified')).toBeVisible();
  expect(requests.slice(countBefore)).toEqual([]);
});

// @claim:two-mb-limit
test('@claim:two-mb-limit rejects receipt text larger than two megabytes', async ({ page }) => {
  await page.goto('/demo/'); await page.getByRole('textbox', { name: 'Receipt JSON' }).fill('x'.repeat(2_000_001));
  await page.getByRole('button', { name: 'Verify receipt' }).click(); await expect(page.getByText(/exceeds the 2 MB/)).toBeVisible();
});

// @claim:offline-reload
test('@claim:offline-reload demo returns after the first cached visit', async ({ page, context }) => {
  await page.goto('/demo/');
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.reload();
  await page.waitForFunction(() => navigator.serviceWorker.controller !== null);
  await context.setOffline(true); await page.reload(); await expect(page.locator('main')).toBeVisible();
});

// @claim:no-third-party-demo-requests
test('@claim:no-third-party-demo-requests permits only same-origin demo requests', async ({ page }) => {
  const requests: string[] = []; page.on('request', r => requests.push(r.url()));
  await page.goto('/demo/'); await expect(page.getByText('Receipt verified')).toBeVisible();
  expect(requests.length).toBeGreaterThan(0);
  expect(requests.every(url => new URL(url).origin === origin)).toBeTruthy();
});

// @claim:site-metadata-and-routing
test('@claim:site-metadata-and-routing provides complete metadata and consistent route landmarks', async ({ page }) => {
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) {
    await page.goto(path);
    await expect(page.locator('h1')).toHaveCount(1); await expect(page.locator('main')).toHaveCount(1);
    for (const selector of ['link[rel="canonical"]', 'link[rel="manifest"]', 'link[rel="apple-touch-icon"]', 'meta[property="og:title"]', 'meta[property="og:description"]', 'meta[property="og:image"]', 'meta[name="twitter:card"]', 'meta[name="twitter:title"]', 'meta[name="twitter:description"]', 'meta[name="twitter:image"]', '#route-announcement[aria-live="polite"]']) await expect(page.locator(selector)).toHaveCount(1);
    await expect(page.locator('header nav')).toHaveAttribute('aria-label', 'Primary navigation');
    await expect(page.locator('footer nav')).toHaveAttribute('aria-label', 'Legal and project links');
  }
  await page.goto('/404.html'); await expect(page.getByRole('heading', { name: 'That receipt page does not exist.' })).toBeVisible();
});

// @claim:terminal-recording
test('@claim:terminal-recording shows the self-hosted released CLI demo recording', async ({ page }) => {
  await page.goto('/'); const recording = page.locator('.terminal-recording img');
  await expect(recording).toHaveAttribute('src', '/terminal-demo.svg'); await expect(recording).toBeVisible();
  await expect(page.getByText(/Recorded from the released v0.1.0 binary/)).toBeVisible();
});

test('routes, focus, mobile width, and axe have no violations', async ({ page }) => {
  await page.goto('/'); await page.keyboard.press('Tab'); await page.keyboard.press('Enter'); await expect(page.locator('main')).toBeFocused();
  await page.getByRole('link', { name: 'Privacy' }).first().click(); await expect(page.getByRole('heading', { name: 'Privacy' })).toBeFocused(); await expect(page.locator('#route-announcement')).toContainText('Privacy');
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) { await page.goto(path); const results = await new AxeBuilder({ page }).analyze(); expect(results.violations).toEqual([]); }
  await page.goto('/demo/'); expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(await page.evaluate(() => document.documentElement.clientWidth));
});
