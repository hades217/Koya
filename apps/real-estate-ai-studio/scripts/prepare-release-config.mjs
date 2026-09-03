import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const required = (name) => {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required for a signed release build.`);
  return value;
};
const httpsEndpoint = (name) => {
  const value = required(name);
  const parsed = new URL(value.replaceAll('{{target}}', 'darwin').replaceAll('{{arch}}', 'aarch64').replaceAll('{{current_version}}', '0.0.0'));
  if (parsed.protocol !== 'https:' || parsed.username || parsed.password) throw new Error(`${name} must be credential-free HTTPS.`);
  return value;
};

const pubkey = required('ESTATE_STUDIO_UPDATER_PUBKEY');
const stableEndpoint = httpsEndpoint('ESTATE_STUDIO_UPDATE_STABLE_URL');
const betaEndpoint = process.env.ESTATE_STUDIO_UPDATE_BETA_URL?.trim();
if (betaEndpoint) httpsEndpoint('ESTATE_STUDIO_UPDATE_BETA_URL');
required('TAURI_SIGNING_PRIVATE_KEY');

const output = resolve(root, 'src-tauri/tauri.release.generated.conf.json');
mkdirSync(dirname(output), { recursive: true });
writeFileSync(output, `${JSON.stringify({
  bundle: { createUpdaterArtifacts: true },
  plugins: { updater: { pubkey, endpoints: [stableEndpoint], windows: { installMode: 'passive' } } },
}, null, 2)}\n`, { mode: 0o600 });
process.stdout.write(`Prepared credential-free release overlay: ${output}\n`);

if (process.argv.includes('--build')) {
  const tauri = resolve(root, 'node_modules/.bin/tauri');
  execFileSync(tauri, ['build', '--config', output], {
    cwd: root,
    env: {
      ...process.env,
      ESTATE_STUDIO_UPDATE_STABLE_URL: stableEndpoint,
      ESTATE_STUDIO_UPDATE_BETA_URL: betaEndpoint ?? '',
      ESTATE_STUDIO_UPDATER_PUBKEY: pubkey,
    },
    stdio: 'inherit',
  });
}
