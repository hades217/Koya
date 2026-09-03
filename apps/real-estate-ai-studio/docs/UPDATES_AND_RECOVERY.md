# Signed updates and recovery

Estate Studio uses the Tauri v2 updater. Update signatures are mandatory and cannot be disabled. The application contains only the updater public key; the signing private key belongs in the release operator's protected build environment.

Official contract: <https://v2.tauri.app/plugin/updater/>

## Release configuration gate

Normal development builds do not contain an updater public key or endpoint, do not create updater artifacts, and report signed updates as unavailable.

Release operators must provide these process-environment values:

- `ESTATE_STUDIO_UPDATER_PUBKEY`: the Tauri updater public key content, not a path;
- `ESTATE_STUDIO_UPDATE_STABLE_URL`: credential-free HTTPS endpoint, optionally using Tauri target variables;
- `ESTATE_STUDIO_UPDATE_BETA_URL`: optional credential-free HTTPS beta endpoint;
- `TAURI_SIGNING_PRIVATE_KEY`: Tauri signing key path or content;
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: optional signing-key password.

Run `npm run tauri:release`. The preflight writes ignored `src-tauri/tauri.release.generated.conf.json` with `bundle.createUpdaterArtifacts: true`, the public key, and stable endpoint, then invokes the Tauri build in the same environment. It never writes or prints the signing private key.

Release output is incomplete until the expected platform updater bundle and `.sig` are present, the static/dynamic update JSON embeds the signature content, and a clean installed build successfully checks the HTTPS endpoint.

## Customer update flow

1. The user checks the selected stable or beta channel.
2. Estate Studio shows the offered version and notes. Nothing downloads yet.
3. The user explicitly installs that exact version.
4. The application immediately rechecks that the version is unchanged.
5. Tauri downloads the bundle and verifies its mandatory signature before installation.
6. Estate Studio writes `updates/pending.json` outside projects with previous version, target version, channel, time, and status.
7. Windows exits/restarts through the installer. macOS and Linux require restarting the application after installation.
8. After the target version launches, the user confirms launch health. The pending marker moves to `last-success.json`.

Project files, source assets, licences, private links, roles, and settings remain in app data and are not removed by an application update.

## Failure and rollback

- A download, network, signature, or installer failure leaves the current installation and a failed recovery marker; it is never reported as updated.
- If the relaunched version does not match the target, Settings shows the retained previous version required for recovery.
- Rollback is deliberately not an unsigned in-app downgrade. Reinstall the retained, previously notarised/signed installer for the recorded previous version without deleting app data.
- Keep at least the current and previous signed/notarised installers and updater signatures in release storage.
- After rollback, open the project library, verify project persistence, run a local static preview, and only then clear operational incident status. Do not mark the failed release healthy.

## Release evidence checklist

- updater public key fingerprint recorded by the release operator;
- signing secret present only in the protected release environment;
- stable/beta endpoint HTTPS and credential-free;
- platform artifact and `.sig` inventory;
- update JSON version, URL, and signature read-back;
- old-version check sees the new version;
- signature-verified install completes;
- new version launches and confirms the recovery marker;
- projects and settings persist;
- retained previous installer can restore the application without deleting app data.
