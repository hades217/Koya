import { cpSync, mkdirSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const studioRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const tourRoot = resolve(studioRoot, '../interactive-apartment-tour');
const tourBuild = resolve(tourRoot, 'pages-dist');
const embeddedTour = resolve(studioRoot, 'public/embedded-tour');

const build = spawnSync(
  process.platform === 'win32' ? 'npm.cmd' : 'npm',
  ['run', 'build:pages'],
  {
    cwd: tourRoot,
    env: { ...process.env, KOYA_TOUR_BASE: '/embedded-tour/' },
    stdio: 'inherit',
  },
);

if (build.status !== 0) {
  process.exit(build.status ?? 1);
}

rmSync(embeddedTour, { recursive: true, force: true });
mkdirSync(embeddedTour, { recursive: true });
cpSync(tourBuild, embeddedTour, { recursive: true });

console.log(`Embedded Koya tour prepared at ${embeddedTour}`);
