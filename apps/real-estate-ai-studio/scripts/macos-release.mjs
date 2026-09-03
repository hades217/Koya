import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(fileURLToPath(new URL('..', import.meta.url)));
const required = (name) => {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required for a signed and notarised macOS release.`);
  return value;
};
const presence = (names) => names.filter((name) => Boolean(process.env[name]?.trim()));

const preflight = () => {
  if (process.platform !== 'darwin') throw new Error('macOS release packaging must run on macOS.');

  const identity = required('APPLE_SIGNING_IDENTITY');
  const identities = execFileSync('/usr/bin/security', ['find-identity', '-v', '-p', 'codesigning'], {
    encoding: 'utf8',
  });
  if (!identities.includes(`\"${identity}\"`)) {
    throw new Error('APPLE_SIGNING_IDENTITY does not exactly match an available code-signing identity.');
  }

  const apiNames = ['APPLE_API_ISSUER', 'APPLE_API_KEY', 'APPLE_API_KEY_PATH'];
  const appleIdNames = ['APPLE_ID', 'APPLE_PASSWORD', 'APPLE_TEAM_ID'];
  const apiPresent = presence(apiNames);
  const appleIdPresent = presence(appleIdNames);
  const completeApi = apiPresent.length === apiNames.length;
  const completeAppleId = appleIdPresent.length === appleIdNames.length;
  if ((apiPresent.length > 0 && !completeApi) || (appleIdPresent.length > 0 && !completeAppleId)) {
    throw new Error('Notarisation credentials are partial. Supply one complete documented credential set.');
  }
  if (!completeApi && !completeAppleId) {
    throw new Error('Notarisation credentials are unavailable. Supply the App Store Connect API set or Apple ID set.');
  }
  if (completeApi && !existsSync(resolve(process.env.APPLE_API_KEY_PATH))) {
    throw new Error('APPLE_API_KEY_PATH does not reference an existing private-key file.');
  }

  for (const name of [
    'ESTATE_STUDIO_UPDATER_PUBKEY',
    'ESTATE_STUDIO_UPDATE_STABLE_URL',
    'TAURI_SIGNING_PRIVATE_KEY',
  ]) required(name);

  process.stdout.write(`macOS release preflight passed for identity: ${identity}\n`);
};

const findArtifacts = () => {
  const roots = [
    resolve(root, 'src-tauri/target/release/bundle'),
    resolve(root, 'src-tauri/target/aarch64-apple-darwin/release/bundle'),
    resolve(root, 'src-tauri/target/x86_64-apple-darwin/release/bundle'),
    resolve(root, 'src-tauri/target/universal-apple-darwin/release/bundle'),
  ].filter(existsSync);
  const files = roots.flatMap((bundleRoot) => readdirSync(bundleRoot, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isFile() && entry.name.endsWith('.dmg'))
    .map((entry) => resolve(entry.parentPath, entry.name)));
  const apps = roots.flatMap((bundleRoot) => readdirSync(bundleRoot, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isDirectory() && entry.name.endsWith('.app'))
    .map((entry) => resolve(entry.parentPath, entry.name)));
  return { apps: [...new Set(apps)], dmgs: [...new Set(files)] };
};

const run = (file, args) => execFileSync(file, args, { cwd: root, stdio: 'inherit' });
const verify = () => {
  const { apps, dmgs } = findArtifacts();
  if (apps.length !== 1 || dmgs.length !== 1) {
    throw new Error(`Expected exactly one macOS app and one DMG; found ${apps.length} app(s) and ${dmgs.length} DMG(s). Clean stale bundle outputs first.`);
  }
  const [app] = apps;
  const [dmg] = dmgs;
  run('/usr/bin/codesign', ['--verify', '--deep', '--strict', '--verbose=2', app]);
  run('/usr/sbin/spctl', ['--assess', '--type', 'execute', '--verbose=4', app]);
  run('/usr/bin/xcrun', ['stapler', 'validate', dmg]);
  run('/usr/sbin/spctl', ['--assess', '--type', 'open', '--context', 'context:primary-signature', '--verbose=4', dmg]);
  process.stdout.write(`Verified signed app and notarised DMG:\n${app}\n${dmg}\n`);
};

const mode = process.argv[2] ?? '--preflight';
if (!['--preflight', '--build', '--verify'].includes(mode)) throw new Error(`Unknown mode: ${mode}`);
if (mode === '--verify') {
  verify();
} else {
  preflight();
  if (mode === '--build') {
    run(process.execPath, [resolve(root, 'scripts/prepare-release-config.mjs'), '--build']);
    verify();
  }
}
