import { spawnSync } from 'node:child_process';
import { mkdir, rm, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';

const binary = resolve('target/release/action-receipts');
const run = spawnSync(binary, ['demo'], { encoding: 'utf8', env: { PATH: process.env.PATH ?? '' } });
if (run.status !== 0) throw new Error(run.stderr || `demo exited ${run.status}`);

const stdout = run.stdout.trimEnd();
const jsonPath = stdout.split('\n').find(line => line.startsWith('JSON: '))?.slice(6);
if (!jsonPath) throw new Error('demo output did not include a JSON path');

const capture = `$ action-receipts demo\n${stdout}\n`;
await mkdir('artwork/source', { recursive: true });
await writeFile('artwork/source/terminal-demo-capture.txt', capture);

const escapeXml = text => text.replaceAll('&', '&amp;').replaceAll('<', '&lt;').replaceAll('>', '&gt;');
const wrap = (line, width = 96) => {
  const parts = [];
  for (let offset = 0; offset < line.length; offset += width) parts.push(line.slice(offset, offset + width));
  return parts.length ? parts : [''];
};
const visualLines = capture.trimEnd().split('\n').flatMap(line => line.startsWith('Verify: action-receipts verify ')
  ? ['Verify: action-receipts verify', line.slice('Verify: action-receipts verify '.length)]
  : wrap(line));
const text = visualLines.map((line, index) => {
  const color = index === 0 ? ' fill="#a8d929"' : '';
  return `    <text x="48" y="${126 + index * 50}"${color}>${escapeXml(line)}</text>`;
}).join('\n');
const height = 176 + visualLines.length * 50;
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="1200" height="${height}" viewBox="0 0 1200 ${height}" role="img" aria-labelledby="title desc">
  <title id="title">Action Receipts demo terminal capture</title>
  <desc id="desc">An actual release binary run creates signed JSON and HTML receipts in a new temporary directory.</desc>
  <rect width="1200" height="${height}" fill="#0b0b09"/>
  <rect width="1200" height="64" fill="#b9f227"/>
  <circle cx="36" cy="32" r="10" fill="#ff5c35" stroke="#171713" stroke-width="3"/><circle cx="68" cy="32" r="10" fill="#f4f0e6" stroke="#171713" stroke-width="3"/><circle cx="100" cy="32" r="10" fill="#3b5bdb" stroke="#171713" stroke-width="3"/>
  <text x="140" y="41" fill="#171713" font-family="monospace" font-size="24" font-weight="700">capture · action-receipts demo · v0.1.0</text>
  <g font-family="monospace" font-size="20" fill="#f7f3e8">
${text}
  </g>
  <rect x="48" y="${height - 45}" width="16" height="28" fill="#ff5c35"/>
</svg>
`;
await writeFile('site/public/terminal-demo.svg', svg);
await rm(dirname(jsonPath), { recursive: true, force: true });
