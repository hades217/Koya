import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import test from 'node:test';

const root = resolve(import.meta.dirname, '..');
const runtime = resolve(root, 'resources/tour-runtime');
const html = readFileSync(resolve(runtime, 'index.html'), 'utf8');
const css = readFileSync(resolve(runtime, 'tour.css'), 'utf8');
const script = readFileSync(resolve(runtime, 'tour.js'), 'utf8');
const desktopCss = readFileSync(resolve(root, 'src/styles.css'), 'utf8');
const desktopApp = readFileSync(resolve(root, 'src/App.tsx'), 'utf8');

const luminance = (hex) => {
  const channels = hex.slice(1).match(/../g).map((value) => Number.parseInt(value, 16) / 255)
    .map((value) => value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4);
  return 0.2126 * channels[0] + 0.7152 * channels[1] + 0.0722 * channels[2];
};
const contrast = (foreground, background) => {
  const values = [luminance(foreground), luminance(background)].sort((left, right) => right - left);
  return (values[0] + 0.05) / (values[1] + 0.05);
};

test('static runtime JavaScript parses without a build tool', () => {
  execFileSync(process.execPath, ['--check', resolve(runtime, 'tour.js')]);
});

test('static runtime is self-contained and manifest driven', () => {
  assert.match(html, /tour-manifest\.js/);
  assert.match(html, /tour\.js/);
  assert.match(html, /tour\.css/);
  assert.doesNotMatch(html, /(?:src|href)=["']https?:\/\//i);
  assert.match(script, /__ESTATE_TOUR_MANIFEST__/);
  assert.match(script, /tour-manifest\.json/);
  assert.doesNotMatch(script, /Apartment 106|Koya/);
});

test('every runtime DOM lookup has a corresponding element', () => {
  const markup = `${html}\n${script}`;
  const ids = new Set([...markup.matchAll(/\bid=[\\"']([^\\"']+)[\\"']/g)].map((match) => match[1]));
  const lookups = [...script.matchAll(/\$\(["']#([A-Za-z][\w-]*)["']\)/g)].map((match) => match[1]);
  assert.ok(lookups.length > 10, 'expected the runtime control surface to be exercised');
  for (const id of lookups) assert.ok(ids.has(id), `missing runtime element #${id}`);
});

test('runtime includes accessible fallback and constrained analytics behavior', () => {
  assert.match(html, /aria-live=["']polite["']/);
  assert.match(script, /stillFallback/);
  assert.match(script, /prefers-reduced-motion/);
  assert.match(script, /credentials:\s*["']omit["']/);
  assert.match(script, /endpoint\.origin\s*!==\s*location\.origin/);
  assert.doesNotMatch(script, /document\.cookie|localStorage|sessionStorage/);
  assert.match(css, /touch-action:\s*none/);
});

test('desktop tour preview delegates fullscreen to its embedded tour', () => {
  assert.match(desktopApp, /<iframe[^>]+allow=["']fullscreen["'][^>]+allowFullScreen/);
  assert.match(desktopApp, /koya-tour:immersive-request/);
  assert.match(desktopCss, /\.preview-shell\.immersive \.preview-toolbar \{ display: none; \}/);
});

test('primary desktop and runtime text has visible keyboard focus and AA contrast', () => {
  assert.match(desktopCss, /button:focus-visible/);
  assert.match(desktopCss, /\[tabindex\]:focus-visible/);
  assert.match(css, /button:focus-visible/);
  for (const [foreground, background] of [
    ['#23241f', '#ecece6'],
    ['#62645c', '#ecece6'],
    ['#f3f1e9', '#191a17'],
    ['#aeb0a7', '#191a17'],
  ]) assert.ok(contrast(foreground, background) >= 4.5, `${foreground} on ${background} must meet 4.5:1`);
});
