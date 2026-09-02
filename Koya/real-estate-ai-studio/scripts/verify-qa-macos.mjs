import { existsSync } from 'node:fs';
import { resolve } from 'node:path';
import { spawnSync } from 'node:child_process';

if (process.platform !== 'darwin') {
  throw new Error('The macOS QA package verifier must run on macOS.');
}

const root = resolve(import.meta.dirname, '..');
const app = resolve(root, 'src-tauri/target/debug/bundle/macos/Estate Studio QA.app');
const dmg = resolve(root, 'src-tauri/target/debug/bundle/dmg/Estate Studio QA_0.1.0_aarch64.dmg');

for (const artifact of [app, dmg]) {
  if (!existsSync(artifact)) throw new Error(`Required QA artifact is unavailable: ${artifact}`);
}

function execute(command, args, expectedStatus = 0) {
  const result = spawnSync(command, args, { encoding: 'utf8' });
  if (result.status !== expectedStatus) {
    throw new Error(`${command} ${args.join(' ')} returned ${result.status}:\n${result.stdout}${result.stderr}`);
  }
  return `${result.stdout}${result.stderr}`;
}

execute('codesign', ['--verify', '--deep', '--strict', '--verbose=4', app]);
const signature = execute('codesign', ['-dv', '--verbose=4', app]);
if (!signature.includes('Identifier=com.landiq.estatestudio.qa')) {
  throw new Error('QA app bundle identifier is not com.landiq.estatestudio.qa.');
}
if (!signature.includes('Signature=adhoc') || !signature.includes('TeamIdentifier=not set')) {
  throw new Error('QA app is not an explicitly isolated ad-hoc package.');
}
execute('hdiutil', ['verify', dmg]);
const gatekeeper = spawnSync('spctl', ['--assess', '--type', 'execute', '--verbose=4', app], { encoding: 'utf8' });
if (gatekeeper.status === 0) {
  throw new Error('QA app unexpectedly passed Gatekeeper; review the QA/release boundary before distribution.');
}

console.log('QA app signature: valid ad-hoc bundle signature');
console.log('QA bundle identifier: com.landiq.estatestudio.qa');
console.log('QA DMG checksum: valid');
console.log('Gatekeeper: rejected as expected for a non-notarised QA package');
