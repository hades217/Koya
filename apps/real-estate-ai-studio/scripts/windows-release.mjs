import { execFileSync } from 'node:child_process';
import { existsSync, readdirSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = resolve(fileURLToPath(new URL('..', import.meta.url)));
const required = (name) => {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required for a signed Windows release.`);
  return value;
};
const httpsUrl = (name) => {
  const value = required(name);
  const parsed = new URL(value.replaceAll('{{target}}', 'windows-x86_64').replaceAll('{{arch}}', 'x86_64').replaceAll('{{current_version}}', '0.0.0'));
  if (parsed.protocol !== 'https:' || parsed.username || parsed.password) {
    throw new Error(`${name} must be credential-free HTTPS.`);
  }
  return value;
};
const run = (file, args, options = {}) => execFileSync(file, args, { cwd: root, stdio: 'inherit', ...options });

const releaseInputs = () => {
  if (process.platform !== 'win32') throw new Error('Windows installer production and acceptance must run on a Windows host.');
  const thumbprint = required('ESTATE_STUDIO_WINDOWS_CERTIFICATE_THUMBPRINT').replaceAll(' ', '').toUpperCase();
  if (!/^[A-F0-9]{40,64}$/.test(thumbprint)) throw new Error('The Windows signing certificate thumbprint is invalid.');
  const certificateFound = execFileSync('powershell.exe', [
    '-NoProfile',
    '-NonInteractive',
    '-Command',
    "$match = Get-ChildItem Cert:\\CurrentUser\\My | Where-Object { $_.Thumbprint -eq $env:ESTATE_STUDIO_WINDOWS_CERTIFICATE_THUMBPRINT }; if ($match) { 'found' } else { exit 2 }",
  ], { encoding: 'utf8', env: { ...process.env, ESTATE_STUDIO_WINDOWS_CERTIFICATE_THUMBPRINT: thumbprint } }).trim();
  if (certificateFound !== 'found') throw new Error('The selected Windows code-signing certificate is unavailable.');

  const pubkey = required('ESTATE_STUDIO_UPDATER_PUBKEY');
  const stableEndpoint = httpsUrl('ESTATE_STUDIO_UPDATE_STABLE_URL');
  const betaEndpoint = process.env.ESTATE_STUDIO_UPDATE_BETA_URL?.trim();
  if (betaEndpoint) httpsUrl('ESTATE_STUDIO_UPDATE_BETA_URL');
  required('TAURI_SIGNING_PRIVATE_KEY');
  const timestampUrl = httpsUrl('ESTATE_STUDIO_WINDOWS_TIMESTAMP_URL');
  return { thumbprint, pubkey, stableEndpoint, betaEndpoint, timestampUrl };
};

const prepare = () => {
  const inputs = releaseInputs();
  const output = resolve(root, 'src-tauri/tauri.windows.release.generated.conf.json');
  writeFileSync(output, `${JSON.stringify({
    bundle: {
      targets: ['nsis', 'msi'],
      createUpdaterArtifacts: true,
      windows: {
        certificateThumbprint: inputs.thumbprint,
        digestAlgorithm: 'sha256',
        timestampUrl: inputs.timestampUrl,
        nsis: { installMode: 'currentUser' },
      },
    },
    plugins: { updater: { pubkey: inputs.pubkey, endpoints: [inputs.stableEndpoint], windows: { installMode: 'passive' } } },
  }, null, 2)}\n`, { mode: 0o600 });
  process.stdout.write(`Windows release preflight passed; prepared public release overlay: ${output}\n`);
  return { ...inputs, output };
};

const findArtifacts = () => {
  const bundle = resolve(root, 'src-tauri/target/release/bundle');
  if (!existsSync(bundle)) return [];
  return readdirSync(bundle, { recursive: true, withFileTypes: true })
    .filter((entry) => entry.isFile() && (entry.name.endsWith('.msi') || entry.name.endsWith('-setup.exe')))
    .map((entry) => resolve(entry.parentPath, entry.name));
};
const verify = () => {
  if (process.platform !== 'win32') throw new Error('Windows artifact verification must run on Windows.');
  const artifacts = findArtifacts();
  const msis = artifacts.filter((path) => path.endsWith('.msi'));
  const installers = artifacts.filter((path) => path.endsWith('-setup.exe'));
  if (msis.length !== 1 || installers.length !== 1) {
    throw new Error(`Expected one MSI and one NSIS installer; found ${msis.length} MSI and ${installers.length} NSIS artifact(s). Clean stale bundles first.`);
  }
  for (const artifact of artifacts) {
    run('powershell.exe', [
      '-NoProfile',
      '-NonInteractive',
      '-Command',
      "$signature = Get-AuthenticodeSignature -LiteralPath $env:ESTATE_STUDIO_VERIFY_ARTIFACT; if ($signature.Status -ne 'Valid') { Write-Error $signature.StatusMessage; exit 3 }",
    ], { env: { ...process.env, ESTATE_STUDIO_VERIFY_ARTIFACT: artifact } });
  }
  process.stdout.write(`Verified Authenticode signatures for:\n${artifacts.join('\n')}\n`);
};

const mode = process.argv[2] ?? '--preflight';
if (!['--preflight', '--build', '--verify'].includes(mode)) throw new Error(`Unknown mode: ${mode}`);
if (mode === '--verify') {
  verify();
} else {
  const inputs = prepare();
  if (mode === '--build') {
    run(resolve(root, 'node_modules/.bin/tauri.cmd'), ['build', '--config', inputs.output], {
      env: {
        ...process.env,
        ESTATE_STUDIO_UPDATE_STABLE_URL: inputs.stableEndpoint,
        ESTATE_STUDIO_UPDATE_BETA_URL: inputs.betaEndpoint ?? '',
        ESTATE_STUDIO_UPDATER_PUBKEY: inputs.pubkey,
      },
    });
    verify();
  }
}
