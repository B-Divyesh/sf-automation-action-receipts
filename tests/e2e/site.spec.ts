import AxeBuilder from '@axe-core/playwright';
import { execFileSync } from 'node:child_process';
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, resolve } from 'node:path';
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
  const theme = page.getByRole('button', { name: 'Use dark theme' });
  if (!await theme.isVisible()) await page.getByRole('button', { name: 'Open menu' }).click();
  await theme.click();
  expect(await page.evaluate(() => localStorage.getItem('demo:ar_theme'))).toBe('dark');
  await page.getByRole('link', { name: 'Start for real' }).click();
  await expect(page).toHaveURL('/');
  expect(await page.evaluate(() => ({ demo: localStorage.getItem('demo:ar_theme'), real: localStorage.getItem('ar_real_marker') }))).toEqual({ demo: null, real: 'keep' });
  await page.goto('/demo/');
  await page.evaluate(() => localStorage.setItem('demo:ar_theme', 'dark'));
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

// @claim:local-verification
test('@claim:local-verification verifies signed receipts with the CLI network lock and cached browser demo offline', async ({ page, context }) => {
  const binary = resolve(process.cwd(), 'target/release/action-receipts');
  expect(existsSync(binary)).toBeTruthy();
  const run = (args: string[]) => execFileSync(binary, args, {
    encoding: 'utf8',
    env: { PATH: process.env.PATH ?? '' },
  });
  const demo = run(['demo']);
  const receipt = demo.split('\n').find(line => line.startsWith('JSON: '))?.slice(6);
  expect(receipt).toBeTruthy();
  try {
    const cliResult = JSON.parse(run(['verify', receipt!, '--json', '--offline']));
    expect(cliResult.valid).toBe(true);

    await page.goto('/demo/');
    await page.evaluate(async () => { await navigator.serviceWorker.ready; });
    await page.reload();
    await page.waitForFunction(() => navigator.serviceWorker.controller !== null);
    await context.setOffline(true);
    await page.reload();
    await expect(page.getByText('Receipt verified')).toBeVisible();
    const box = page.getByRole('textbox', { name: 'Receipt JSON' });
    await box.fill((await box.inputValue()).replace('policy-gate', 'changed-tool'));
    await page.getByRole('button', { name: 'Verify receipt' }).click();
    await expect(page.getByText('Verification failed')).toBeVisible();
  } finally {
    await context.setOffline(false);
    if (receipt) rmSync(dirname(receipt), { recursive: true, force: true });
  }
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
    for (const selector of ['link[rel="canonical"]', 'link[rel="manifest"]', 'link[rel="apple-touch-icon"][sizes="180x180"][href="/apple-touch-icon.png"]', 'meta[property="og:title"]', 'meta[property="og:description"]', 'meta[property="og:image"]', 'meta[property="og:image:width"][content="1200"]', 'meta[property="og:image:height"][content="630"]', 'meta[name="twitter:card"]', 'meta[name="twitter:title"]', 'meta[name="twitter:description"]', 'meta[name="twitter:image"]', '#route-announcement[aria-live="polite"]']) await expect(page.locator(selector)).toHaveCount(1);
    await expect(page.locator('meta[property="og:image"]')).toHaveAttribute('content', 'https://automation-action-receipts.sociobot.in/social-card.webp');
    await expect(page.locator('header nav')).toHaveAttribute('aria-label', 'Primary navigation');
    await expect(page.locator('footer nav')).toHaveAttribute('aria-label', 'Legal and project links');
  }
  await page.goto('/404.html'); await expect(page.getByRole('heading', { name: 'That receipt page does not exist.' })).toBeVisible();
  const socialSize = await page.evaluate(async () => new Promise<{ width: number; height: number }>((resolveImage, reject) => {
    const image = new Image(); image.onload = () => resolveImage({ width: image.naturalWidth, height: image.naturalHeight }); image.onerror = reject; image.src = '/social-card.webp';
  }));
  expect(socialSize).toEqual({ width: 1200, height: 630 });
  const appleSize = await page.evaluate(async () => new Promise<{ width: number; height: number }>((resolveImage, reject) => {
    const image = new Image(); image.onload = () => resolveImage({ width: image.naturalWidth, height: image.naturalHeight }); image.onerror = reject; image.src = '/apple-touch-icon.png';
  }));
  expect(appleSize).toEqual({ width: 180, height: 180 });
});

// @claim:terminal-recording
test('@claim:terminal-recording matches an actual release CLI demo capture', async ({ page }) => {
  const binary = resolve(process.cwd(), 'target/release/action-receipts');
  const output = execFileSync(binary, ['demo'], { encoding: 'utf8', env: { PATH: process.env.PATH ?? '' } });
  const json = output.split('\n').find(line => line.startsWith('JSON: '))?.slice(6);
  expect(json).toBeTruthy();
  const normalize = (value: string) => value.replaceAll(/\/tmp\/action-receipts-demo-\d+-\d+/g, '<DEMO_DIR>').trim();
  const capture = readFileSync(resolve('artwork/source/terminal-demo-capture.txt'), 'utf8');
  const svg = readFileSync(resolve('site/public/terminal-demo.svg'), 'utf8');
  expect(normalize(capture)).toBe(normalize(`$ action-receipts demo\n${output}`));
  const visualLines = capture.trim().split('\n').flatMap(line => line.startsWith('Verify: action-receipts verify ')
    ? ['Verify: action-receipts verify', line.slice('Verify: action-receipts verify '.length)]
    : line.match(/.{1,96}/g) ?? ['']);
  for (const line of visualLines) expect(svg).toContain(line);
  expect(svg).not.toContain('<animate');
  expect(svg).not.toContain('two linked events · signed receipt');
  await page.goto('/'); const recording = page.locator('.terminal-recording img');
  await expect(recording).toHaveAttribute('src', '/terminal-demo.svg'); await expect(recording).toBeVisible();
  await expect(page.getByText(/Captured from the released v0.1.0 binary/)).toBeVisible();
  if (json) rmSync(dirname(json), { recursive: true, force: true });
});

// @claim:linux-download
test('@claim:linux-download serves a Linux x64 executable that reports its version and runs an isolated demo', async ({ request }) => {
  const response = await request.get('/downloads/action-receipts-linux-amd64');
  expect(response.status()).toBe(200);
  const bytes = Buffer.from(await response.body());
  expect([...bytes.subarray(0, 4)]).toEqual([0x7f, 0x45, 0x4c, 0x46]);
  expect(bytes[4]).toBe(2);
  expect(bytes.readUInt16LE(18)).toBe(62);
  const root = mkdtempSync(resolve(tmpdir(), 'action-receipts-download-'));
  const caller = resolve(root, 'caller');
  const binary = resolve(root, 'action-receipts');
  writeFileSync(binary, bytes); chmodSync(binary, 0o755);
  writeFileSync(resolve(root, '.keep'), '');
  try {
    expect(execFileSync(binary, ['--version'], { encoding: 'utf8' }).trim()).toBe('action-receipts 0.1.0');
    mkdirSync(caller);
    const output = execFileSync(binary, ['demo'], { cwd: caller, encoding: 'utf8', env: { PATH: process.env.PATH ?? '' } });
    const json = output.split('\n').find(line => line.startsWith('JSON: '))?.slice(6);
    const html = output.split('\n').find(line => line.startsWith('HTML: '))?.slice(6);
    expect(json).toBeTruthy(); expect(html).toBeTruthy();
    expect(dirname(json!)).not.toBe(caller);
    expect(readdirSync(caller)).toEqual([]);
    for (const receipt of [json!, html!]) {
      const verified = JSON.parse(execFileSync(binary, ['verify', receipt, '--json'], { encoding: 'utf8' }));
      expect(verified).toMatchObject({ valid: true, event_count: 2 });
    }
    rmSync(dirname(json!), { recursive: true, force: true });
  } finally { rmSync(root, { recursive: true, force: true }); }
});

// @claim:site-build-output
test('@claim:site-build-output includes deployable routes, metadata, offline assets, and the executable', async () => {
  const dist = resolve('dist/site');
  for (const path of ['index.html', 'demo/index.html', 'privacy/index.html', 'terms/index.html', '404.html', 'staticwebapp.config.json', 'sw.js', 'manifest.webmanifest', 'social-card.webp', 'apple-touch-icon.png', 'terminal-demo.svg', 'downloads/action-receipts-linux-amd64']) expect(existsSync(resolve(dist, path)), path).toBeTruthy();
  for (const path of ['index.html', 'demo/index.html', 'privacy/index.html', 'terms/index.html', '404.html']) {
    const html = readFileSync(resolve(dist, path), 'utf8');
    for (const marker of ['<title', 'rel="canonical"', 'rel="apple-touch-icon"', 'property="og:image"', 'name="twitter:card"', '<main']) expect(html, `${path}: ${marker}`).toContain(marker);
  }
  const binary = resolve(dist, 'downloads/action-receipts-linux-amd64');
  expect(statSync(binary).mode & 0o111).not.toBe(0);
  expect(execFileSync(binary, ['--version'], { encoding: 'utf8' }).trim()).toBe('action-receipts 0.1.0');
});

test('routes, focus, mobile width, and axe have no violations', async ({ page }) => {
  await page.goto('/'); await page.keyboard.press('Tab'); await page.keyboard.press('Enter'); await expect(page.locator('main')).toBeFocused();
  const privacy = page.getByRole('link', { name: 'Privacy' }).first();
  if (!await privacy.isVisible()) await page.getByRole('button', { name: 'Open menu' }).click();
  await privacy.click(); await expect(page.getByRole('heading', { name: 'Privacy' })).toBeFocused(); await expect(page.locator('#route-announcement')).toContainText('Privacy');
  await page.goBack(); await expect(page.getByRole('heading', { name: 'Record and verify automated changes.' })).toBeFocused(); await expect(page.locator('#route-announcement')).toContainText('Record and verify automated changes.');
  await page.goForward(); await expect(page.getByRole('heading', { name: 'Privacy' })).toBeFocused(); await expect(page.locator('#route-announcement')).toContainText('Privacy');
  for (const path of ['/', '/demo/', '/privacy/', '/terms/', '/404.html']) { await page.goto(path); const results = await new AxeBuilder({ page }).analyze(); expect(results.violations).toEqual([]); }
  await page.goto('/demo/'); expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(await page.evaluate(() => document.documentElement.clientWidth));
});

test('first-screen words stay intact and product facts remain above the fold', async ({ page }) => {
  await page.goto('/');
  const wordLines = await page.locator('h1').evaluate(heading => {
    const text = heading.firstChild;
    if (!text) return [];
    return [...(text.textContent ?? '').matchAll(/\S+/g)].map(match => {
      const range = document.createRange();
      range.setStart(text, match.index ?? 0); range.setEnd(text, (match.index ?? 0) + match[0].length);
      return range.getClientRects().length;
    });
  });
  expect(wordLines.every(lines => lines === 1)).toBeTruthy();
  for (const fact of await page.locator('.trust-list li').all()) await expect(fact).toBeInViewport();
});

test('mobile menu exposes every route and supports Escape', async ({ page }) => {
  test.skip((page.viewportSize()?.width ?? 1000) > 800, 'mobile-only behavior');
  await page.goto('/');
  const button = page.locator('#nav-menu-toggle');
  await expect(button).toBeVisible(); await button.focus(); await page.keyboard.press('Enter');
  await expect(button).toHaveAttribute('aria-expanded', 'true');
  for (const name of ['Demo', 'Verify', 'How it works', 'Privacy']) await expect(page.getByRole('link', { name, exact: true }).first()).toBeVisible();
  await page.keyboard.press('Escape'); await expect(button).toBeFocused(); await expect(button).toHaveAttribute('aria-expanded', 'false');
});

test('terminal capture has no ambient animation under reduced motion', async ({ page, request }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' }); await page.goto('/');
  await expect(page.locator('.terminal-recording img')).toBeVisible();
  expect(await (await request.get('/terminal-demo.svg')).text()).not.toContain('<animate');
  expect(await page.locator('.hero-figure').evaluate(element => getComputedStyle(element).transform)).toBe('none');
});
