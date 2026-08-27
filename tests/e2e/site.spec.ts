import AxeBuilder from '@axe-core/playwright';
import { expect, test } from 'playwright/test';

test('home is accessible and verifies the CLI sample locally', async ({ page }) => {
  const errors: string[] = [];
  page.on('console', (message) => { if (message.type() === 'error') errors.push(message.text()); });
  await page.goto('/');
  await expect(page).toHaveTitle(/Action Receipts/);
  await expect(page.locator('h1')).toHaveCount(1);
  await page.getByRole('button', { name: 'Load signed sample' }).click();
  await expect(page.getByText('Cryptographically valid')).toBeVisible();
  await expect(page.getByText('2 linked events')).toBeVisible();
  const accessibility = await new AxeBuilder({ page }).analyze();
  expect(accessibility.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  expect(errors).toEqual([]);
});

test('tampering produces a clear invalid state', async ({ page }) => {
  await page.goto('/#verify');
  await page.getByRole('button', { name: 'Load signed sample' }).click();
  const textarea = page.getByRole('textbox', { name: 'Receipt JSON' });
  await textarea.fill((await textarea.inputValue()).replace('policy-gate', 'changed-tool'));
  await page.getByRole('button', { name: 'Verify locally' }).click();
  await expect(page.getByText('Verification failed')).toBeVisible();
  await expect(page.getByText(/changed after it was recorded/)).toBeVisible();
});

test('license return is stored, stripped, and unlocks the policy kit', async ({ page }) => {
  await page.route('https://api.sociobot.in/**', (route) => route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify({ valid: true, reason: 'ok', expires_at: null }) }));
  await page.goto('/?license=test-license#pricing');
  await expect(page).toHaveURL(/\/#pricing$/);
  await expect(page.getByRole('heading', { name: 'Team policy generator' })).toBeVisible();
  expect(await page.evaluate(() => localStorage.getItem('sb_license:automation-action-receipts'))).toBe('test-license');
});

test('mobile layout stays within the viewport', async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== 'mobile', 'mobile-only layout assertion');
  await page.goto('/');
  const widths = await page.evaluate(() => ({ scroll: document.documentElement.scrollWidth, client: document.documentElement.clientWidth }));
  expect(widths.scroll).toBeLessThanOrEqual(widths.client);
  await expect(page.getByRole('link', { name: 'Install the CLI' })).toBeVisible();
  await page.getByRole('link', { name: 'Verify a receipt' }).click();
  await expect(page.getByRole('heading', { name: 'Drop the log. Check the receipt.' })).toBeVisible();
});

for (const path of ['/privacy/', '/terms/']) {
  test(`${path} has semantic structure and no serious accessibility findings`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator('main')).toHaveCount(1);
    await expect(page.locator('h1')).toHaveCount(1);
    const accessibility = await new AxeBuilder({ page }).analyze();
    expect(accessibility.violations.filter((violation) => ['serious', 'critical'].includes(violation.impact ?? ''))).toEqual([]);
  });
}
