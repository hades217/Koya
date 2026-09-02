import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(fileURLToPath(new URL('..', import.meta.url)));
const output = resolve(root, 'test/fixtures/generated/tier-b');
const font = '/System/Library/Fonts/HelveticaNeue.ttc';
mkdirSync(output, { recursive: true });

const magick = (name, args) => execFileSync('/opt/homebrew/bin/magick', [...args, resolve(output, name)], { stdio: 'inherit' });

const plan = (name, label, variant) => magick(name, [
  '-size', '1600x1000', 'xc:#f6f2e9',
  '-fill', 'none', '-stroke', '#252722', '-strokewidth', '10',
  '-draw', variant === 'a1'
    ? 'rectangle 120,150 1480,850 rectangle 120,150 720,520 rectangle 720,150 1480,520 rectangle 120,520 900,850 rectangle 900,520 1480,850 line 720,520 720,610 line 900,520 900,610'
    : 'rectangle 120,150 1480,850 rectangle 120,150 900,500 rectangle 900,150 1480,500 rectangle 120,500 650,850 rectangle 650,500 1120,850 rectangle 1120,500 1480,850 line 900,500 900,590',
  '-fill', '#252722', '-stroke', 'none', '-font', font, '-pointsize', '52',
  '-annotate', '+120+95', `${label} · QA FLOOR PLAN`,
  '-pointsize', '30', '-annotate', '+120+930', 'SYNTHETIC TEST FIXTURE · NOT FOR SALE · NO DIMENSIONS OR REAL PROPERTY CLAIMS',
]);

plan('harbourlight-a1-floorplan.png', 'HARBOURLIGHT A1', 'a1');
plan('harbourlight-a2-floorplan.png', 'HARBOURLIGHT A2', 'a2');

magick('harbourlight-style-render.png', [
  '-size', '1600x900', 'gradient:#8ca3ac-#ead9c3',
  '-fill', '#f5eee4', '-stroke', '#463f38', '-strokewidth', '8',
  '-draw', 'rectangle 180,170 1420,760 rectangle 260,250 700,680 rectangle 760,250 1340,680 line 180,760 1420,760',
  '-fill', '#252722', '-stroke', 'none', '-font', font, '-pointsize', '54',
  '-annotate', '+180+110', 'HARBOURLIGHT · QA STYLE RENDER',
  '-pointsize', '30', '-annotate', '+180+840', 'SYNTHETIC TEST FIXTURE · NOT AN APPROVED PROPERTY REPRESENTATION',
]);

magick('harbourlight-test-panorama.png', [
  '-size', '4096x2048', 'gradient:#6f8790-#dfc9ae',
  '-stroke', '#f3eee5', '-strokewidth', '6', '-fill', 'none',
  '-draw', 'line 0,1024 4096,1024 line 1024,0 1024,2048 line 2048,0 2048,2048 line 3072,0 3072,2048',
  '-fill', '#252722', '-stroke', 'none', '-font', font, '-pointsize', '82',
  '-annotate', '+120+150', 'HARBOURLIGHT · 4096 × 2048 QA PANORAMA',
  '-pointsize', '44', '-annotate', '+120+1950', 'SYNTHETIC TEST FIXTURE · 2:1 DELIVERY CONTRACT ONLY · NOT FOR SALE',
]);

magick('harbourlight-queue-panorama.png', [
  '-size', '4096x2048', 'gradient:#657e86-#d8c2a6',
  '-stroke', '#f3eee5', '-strokewidth', '6', '-fill', 'none',
  '-draw', 'line 0,1024 4096,1024 line 1024,0 1024,2048 line 2048,0 2048,2048 line 3072,0 3072,2048',
  '-fill', '#252722', '-stroke', 'none', '-font', font, '-pointsize', '82',
  '-annotate', '+120+150', 'HARBOURLIGHT · BACKGROUND QUEUE QA',
  '-pointsize', '44', '-annotate', '+120+1950', 'SYNTHETIC TEST FIXTURE · 2:1 DELIVERY CONTRACT ONLY · NOT FOR SALE',
]);

execFileSync('/opt/homebrew/bin/ffmpeg', [
  '-hide_banner', '-loglevel', 'error', '-y',
  '-f', 'lavfi', '-i', 'testsrc2=size=1280x720:rate=24',
  '-t', '2', '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
  resolve(output, 'harbourlight-test-video.mp4'),
], { stdio: 'inherit' });

writeFileSync(resolve(output, 'RIGHTS.md'), `# Harbourlight Tier B QA fixtures

Generated locally by Estate Studio's deterministic fixture script on demand.

- Owner: Estate Studio QA fixture
- Permission: local product acceptance only
- Commercial/property representation: prohibited
- Real project facts, dimensions, outlooks, finishes, addresses, and customer assets: none
- Koya assets used: none
`);

writeFileSync(resolve(output, 'broken-image.png'), 'This is an intentionally invalid PNG used to verify visible import recovery.\n');

process.stdout.write(`Generated rights-safe Tier B QA fixtures at ${output}\n`);
