import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import test from 'node:test';
import { CURRENT_PROJECT_SCHEMA_VERSION, normalizeBrowserProject } from '../src/browser-projects.ts';

const root = resolve(import.meta.dirname, '..');

test('browser, desktop and bundled example use one current project schema', () => {
  const rust = readFileSync(resolve(root, 'src-tauri/src/lib.rs'), 'utf8');
  const desktopVersion = Number(rust.match(/const CURRENT_SCHEMA_VERSION: u32 = (\d+);/)?.[1]);
  const example = JSON.parse(readFileSync(resolve(root, 'resources/examples/koya-example-v1/project.json'), 'utf8'));

  assert.equal(desktopVersion, CURRENT_PROJECT_SCHEMA_VERSION);
  assert.equal(example.schemaVersion, CURRENT_PROJECT_SCHEMA_VERSION);
  assert.equal(
    example.tourPreviewUrl,
    '/embedded-tour/index.html?unit=106&mode=video&tour=entry-room',
    'the bundled demo must open its packaged tour instead of depending on a public URL',
  );
  assert.equal(
    example.releases.at(-1)?.publicUrl,
    'https://hades217.github.io/Koya/?unit=106&mode=video&tour=entry-room',
    'the verified public release remains separate from the local desktop preview',
  );
});

test('legacy browser projects migrate to the current manifest without mutating stored input', () => {
  const legacy = {
    projectRoot: 'Browser prototype/legacy-project',
    manifest: {
      schemaVersion: 26,
      projectId: 'legacy-project',
      name: 'Legacy Project',
      company: 'Legacy Company',
      location: '',
      status: 'Intake',
      readiness: 12,
      readOnly: false,
      createdAt: 1,
      updatedAt: 1,
      units: [],
      assets: [],
      generationJobs: [],
      releases: [],
      brand: { primary: '#20241f', accent: '#78917b' },
      modules: { tour: 'not_started', creative: 'not_started', deployment: 'not_started' },
    },
  };

  const migrated = normalizeBrowserProject(legacy);

  assert.equal(CURRENT_PROJECT_SCHEMA_VERSION, 30);
  assert.equal(migrated.manifest.schemaVersion, CURRENT_PROJECT_SCHEMA_VERSION);
  assert.deepEqual(migrated.manifest.companyProfile, { id: 'legacy-company', name: 'Legacy Company' });
  assert.equal(migrated.manifest.locale, 'en-AU');
  assert.equal(migrated.manifest.measurementUnits, 'metric');
  assert.equal(migrated.manifest.accessMode, 'unlisted');
  assert.equal(migrated.manifest.disclosure, 'Disclosure not supplied');
  assert.equal(migrated.manifest.workflowMode, 'standard');
  assert.equal(migrated.manifest.designSpec.status, 'not_started');
  for (const key of ['approvalEvents', 'qaRecords', 'usageLedger', 'backgroundJobs', 'creativeJobs', 'enabledSkillIds', 'analyticsEvents']) {
    assert.deepEqual(migrated.manifest[key], [], `${key} should be migration-safe`);
  }
  assert.equal(legacy.manifest.schemaVersion, 26, 'normalization must not mutate localStorage input');
});

test('current browser projects preserve existing profile and workflow records', () => {
  const current = {
    projectRoot: 'Browser prototype/current-project',
    manifest: {
      schemaVersion: CURRENT_PROJECT_SCHEMA_VERSION,
      projectId: 'current-project',
      name: 'Current Project',
      company: 'Current Company',
      companyProfile: { id: 'profile-id', name: 'Profile Name' },
      location: 'Brisbane',
      locale: 'en-AU',
      measurementUnits: 'metric',
      accessMode: 'private',
      disclosure: 'Approved disclosure',
      workflowMode: 'advanced',
      status: 'Ready',
      readiness: 100,
      readOnly: false,
      createdAt: 1,
      updatedAt: 2,
      units: [],
      assets: [],
      generationJobs: [],
      creativeJobs: [],
      enabledSkillIds: ['floorplan'],
      releases: [],
      approvalEvents: [],
      qaRecords: [],
      usageLedger: [],
      backgroundJobs: [],
      analyticsEvents: [{ id: 'event-1', event: 'project_created', source: 'browser', createdAt: 1 }],
      brand: { primary: '#111111', accent: '#222222' },
      modules: { tour: 'ready', creative: 'in_progress', deployment: 'not_started' },
    },
  };

  const migrated = normalizeBrowserProject(current);
  assert.deepEqual(migrated.manifest.companyProfile, current.manifest.companyProfile);
  assert.equal(migrated.manifest.workflowMode, 'advanced');
  assert.deepEqual(migrated.manifest.enabledSkillIds, ['floorplan']);
  assert.deepEqual(migrated.manifest.analyticsEvents, current.manifest.analyticsEvents);
});
