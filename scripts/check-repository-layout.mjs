import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { extname, join } from 'node:path';
import process from 'node:process';

const root = process.cwd();
const required = [
  'apps/interactive-apartment-tour/package.json',
  'apps/real-estate-ai-studio/package.json',
  'docs/product',
  'projects/koya/AGENTS.md',
  'projects/koya/production-assets',
  'projects/koya/video-production/SEEDANCE_2_5_RULES.md',
];
const allowedTopLevel = new Set([
  '.autonomous',
  '.claude',
  '.git',
  '.gitattributes',
  '.github',
  '.gitignore',
  'AGENTS.md',
  'CLAUDE.md',
  'README.md',
  'apps',
  'docs',
  'package.json',
  'projects',
  'scripts',
]);

const failures = [];

for (const path of required) {
  if (!existsSync(join(root, path))) failures.push(`missing required path: ${path}`);
}

for (const entry of readdirSync(root)) {
  if (!allowedTopLevel.has(entry) && entry !== '.DS_Store') {
    failures.push(`unexpected top-level entry: ${entry}`);
  }
  if (['.mp4', '.mov', '.pdf', '.png', '.jpg', '.jpeg', '.webp'].includes(extname(entry).toLowerCase())) {
    failures.push(`binary asset must not live at repository root: ${entry}`);
  }
}

const workflowPath = join(root, '.github/workflows/deploy-koya-tour-pages.yml');
if (existsSync(workflowPath)) {
  const workflow = readFileSync(workflowPath, 'utf8');
  if (workflow.includes('Koya/interactive-apartment-tour')) {
    failures.push('GitHub Pages workflow still references the retired Koya/ app path');
  }
}

if (failures.length) {
  console.error('Repository layout check failed:');
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log('Repository layout check passed.');
