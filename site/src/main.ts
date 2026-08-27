import './style.css';
import { verifyReceipt, type Receipt, type Verification } from './verifier';

const PRODUCT = 'automation-action-receipts';
const API = `https://api.sociobot.in/api/v1/products/${PRODUCT}`;
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `${LICENSE_KEY}:verdict`;
const DAY = 86_400_000;

function byId<T extends HTMLElement>(id: string): T | null {
  return document.getElementById(id) as T | null;
}

function setupTheme() {
  const button = byId<HTMLButtonElement>('theme-toggle');
  if (!button) return;
  const stored = localStorage.getItem('ar_theme');
  if (stored === 'dark' || stored === 'light') document.documentElement.dataset.theme = stored;
  const render = () => {
    const dark = document.documentElement.dataset.theme === 'dark' || (!document.documentElement.dataset.theme && matchMedia('(prefers-color-scheme: dark)').matches);
    button.textContent = dark ? 'Light' : 'Dark';
    button.setAttribute('aria-label', `Switch to ${dark ? 'light' : 'dark'} theme`);
  };
  button.addEventListener('click', () => {
    const isDark = document.documentElement.dataset.theme === 'dark' || (!document.documentElement.dataset.theme && matchMedia('(prefers-color-scheme: dark)').matches);
    document.documentElement.dataset.theme = isDark ? 'light' : 'dark';
    localStorage.setItem('ar_theme', isDark ? 'light' : 'dark');
    render();
  });
  render();
}

function setupConnectivity() {
  const bar = byId<HTMLElement>('offline-bar');
  if (!bar) return;
  const render = () => { bar.hidden = navigator.onLine; };
  addEventListener('online', render);
  addEventListener('offline', render);
  render();
}

function renderVerification(result: Verification) {
  const panel = byId<HTMLElement>('verification-result');
  if (!panel) return;
  panel.className = `verification-result ${result.valid ? 'is-valid' : 'is-invalid'}`;
  panel.replaceChildren();
  const mark = document.createElement('span');
  mark.className = 'result-mark';
  mark.setAttribute('aria-hidden', 'true');
  mark.textContent = result.valid ? '✓' : '×';
  const kicker = document.createElement('p');
  kicker.className = 'result-kicker';
  kicker.textContent = result.valid ? 'Cryptographically valid' : 'Verification failed';
  const title = document.createElement('h3');
  title.textContent = result.valid ? `${result.eventCount} linked event${result.eventCount === 1 ? '' : 's'}` : result.message;
  const body = document.createElement('p');
  body.textContent = result.valid
    ? `Receipt ${result.receiptId}. Every event hash and the Ed25519 signature match.`
    : 'Do not rely on this bundle. Ask the automation owner for the original signed receipt.';
  const caveat = document.createElement('p');
  caveat.className = 'result-caveat';
  caveat.textContent = 'A valid signature proves integrity—not identity, intent, authorization legitimacy, or correctness.';
  panel.append(mark, kicker, title, body, caveat);
}

async function verifyText(text: string) {
  const panel = byId<HTMLElement>('verification-result');
  if (text.length > 2_000_000) throw new Error('Receipt exceeds the 2 MB browser limit. Use the CLI verifier.');
  if (panel) {
    panel.className = 'verification-result is-loading';
    panel.innerHTML = '<span class="result-mark" aria-hidden="true">…</span><p class="result-kicker">Checking</p><h3>Recomputing the chain</h3><p>Verification stays on this device.</p>';
  }
  const receipt = JSON.parse(text) as Receipt;
  renderVerification(await verifyReceipt(receipt));
}

function setupVerifier() {
  const input = byId<HTMLInputElement>('receipt-file');
  const textarea = byId<HTMLTextAreaElement>('receipt-json');
  const button = byId<HTMLButtonElement>('verify-button');
  const sample = byId<HTMLButtonElement>('sample-button');
  const label = document.querySelector<HTMLElement>('.file-label');
  if (!input || !textarea || !button || !sample || !label) return;
  const run = async (text: string) => {
    try { await verifyText(text); }
    catch (error) {
      renderVerification({ valid: false, chainValid: false, signatureValid: false, eventCount: 0, message: error instanceof Error ? error.message : 'Could not read this receipt.' });
    }
  };
  input.addEventListener('change', async () => {
    const file = input.files?.[0];
    if (!file) return;
    const text = await file.text();
    textarea.value = text;
    await run(text);
  });
  button.addEventListener('click', () => run(textarea.value));
  sample.addEventListener('click', async () => {
    try {
      const response = await fetch('/sample.receipt.json');
      if (!response.ok) throw new Error('The sample receipt is unavailable offline until it has been opened once.');
      textarea.value = await response.text();
      await run(textarea.value);
    } catch (error) {
      renderVerification({ valid: false, chainValid: false, signatureValid: false, eventCount: 0, message: error instanceof Error ? error.message : 'Could not load the sample.' });
    }
  });
  for (const type of ['dragenter', 'dragover']) label.addEventListener(type, (event) => { event.preventDefault(); label.classList.add('is-dragging'); });
  for (const type of ['dragleave', 'drop']) label.addEventListener(type, (event) => { event.preventDefault(); label.classList.remove('is-dragging'); });
  label.addEventListener('drop', async (event) => {
    const file = event.dataTransfer?.files[0];
    if (!file) return;
    const text = await file.text();
    textarea.value = text;
    await run(text);
  });
}

function setupCopyButtons() {
  document.querySelectorAll<HTMLButtonElement>('[data-copy-target]').forEach((button) => {
    button.addEventListener('click', async () => {
      const target = byId<HTMLElement>(button.dataset.copyTarget ?? '');
      if (!target) return;
      await navigator.clipboard.writeText(target.textContent ?? '');
      button.textContent = 'Copied';
      setTimeout(() => { button.textContent = 'Copy all'; }, 1500);
    });
  });
}

type CachedVerdict = { valid: boolean; reason: string; checkedAt: number };

function setUnlocked(unlocked: boolean, message: string) {
  const builder = byId<HTMLElement>('policy-builder');
  const status = byId<HTMLElement>('license-status');
  const clear = byId<HTMLButtonElement>('clear-license');
  if (builder) builder.hidden = !unlocked;
  if (status) status.textContent = message;
  if (clear) clear.hidden = !localStorage.getItem(LICENSE_KEY);
}

async function checkLicense(token: string, force = false) {
  const cached = JSON.parse(localStorage.getItem(VERDICT_KEY) ?? 'null') as CachedVerdict | null;
  if (cached?.valid) setUnlocked(true, 'Team policy kit unlocked from the last valid check.');
  if (!force && cached && Date.now() - cached.checkedAt < DAY) {
    setUnlocked(cached.valid, cached.valid ? 'Team policy kit unlocked.' : 'License no longer active.');
    return;
  }
  try {
    const response = await fetch(`${API}/verify?license=${encodeURIComponent(token)}`);
    if (!response.ok) throw new Error('License service unavailable');
    const verdict = await response.json() as { valid: boolean; reason: string };
    localStorage.setItem(VERDICT_KEY, JSON.stringify({ valid: verdict.valid, reason: verdict.reason, checkedAt: Date.now() }));
    setUnlocked(verdict.valid, verdict.valid ? 'Team policy kit unlocked.' : 'License no longer active. You can purchase or restore another license.');
  } catch {
    setUnlocked(Boolean(cached?.valid), cached?.valid ? 'Offline — using the last valid license check.' : 'Could not check the license. The free receipt tools remain available.');
  }
}

function setupLicense() {
  const params = new URLSearchParams(location.search);
  const returned = params.get('license');
  if (returned) {
    localStorage.setItem(LICENSE_KEY, returned);
    localStorage.removeItem(VERDICT_KEY);
    params.delete('license');
    history.replaceState({}, '', `${location.pathname}${params.size ? `?${params}` : ''}${location.hash}`);
  }
  const token = localStorage.getItem(LICENSE_KEY);
  if (token) void checkLicense(token);
  const form = byId<HTMLFormElement>('license-form');
  const input = byId<HTMLInputElement>('license-token');
  form?.addEventListener('submit', (event) => {
    event.preventDefault();
    const value = input?.value.trim();
    if (!value) return;
    localStorage.setItem(LICENSE_KEY, value);
    localStorage.removeItem(VERDICT_KEY);
    if (input) input.value = '';
    void checkLicense(value, true);
  });
  byId<HTMLButtonElement>('clear-license')?.addEventListener('click', () => {
    localStorage.removeItem(LICENSE_KEY);
    localStorage.removeItem(VERDICT_KEY);
    setUnlocked(false, 'License removed from this device.');
  });
}

function setupPolicyBuilder() {
  byId<HTMLFormElement>('policy-form')?.addEventListener('submit', (event) => {
    event.preventDefault();
    const policy = {
      format: 'https://actionreceipts.dev/policy/v1',
      team: byId<HTMLInputElement>('org-name')?.value,
      receipt: {
        retention_days: Number(byId<HTMLInputElement>('retention-days')?.value),
        required_scope_prefix: byId<HTMLInputElement>('required-scope')?.value,
        redact_sensitive_keys: true,
      },
      ci_gate: { require_sealed: true, require_valid_signature: true },
    };
    const url = URL.createObjectURL(new Blob([JSON.stringify(policy, null, 2)], { type: 'application/json' }));
    const link = document.createElement('a');
    link.href = url;
    link.download = 'action-receipts-policy.json';
    link.click();
    URL.revokeObjectURL(url);
  });
}

setupTheme();
setupConnectivity();
setupVerifier();
setupCopyButtons();
setupLicense();
setupPolicyBuilder();

if ('serviceWorker' in navigator) addEventListener('load', () => navigator.serviceWorker.register('/sw.js'));
