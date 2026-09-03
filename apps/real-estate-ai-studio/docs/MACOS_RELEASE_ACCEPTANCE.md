# macOS signed release acceptance

Estate Studio direct-download releases require both Developer ID signing and Apple notarisation. An ad-hoc signature or an unsigned DMG is a development artifact, not a customer release.

Official contracts:

- <https://v2.tauri.app/distribute/sign/macos/>
- <https://v2.tauri.app/distribute/dmg/>

## Protected build gate

Set `APPLE_SIGNING_IDENTITY` to the exact installed Developer ID Application identity. Supply exactly one complete notarisation credential set documented by Tauri:

- `APPLE_API_ISSUER`, `APPLE_API_KEY`, and `APPLE_API_KEY_PATH`; or
- `APPLE_ID`, `APPLE_PASSWORD`, and `APPLE_TEAM_ID`.

Also supply the signed-updater environment documented in `UPDATES_AND_RECOVERY.md`. Run `npm run macos:release:preflight`, then `npm run macos:release`. Secrets must remain in the protected process environment and must not be printed, copied into arguments, project bundles, release notes, or Git.

The release command refuses partial credentials, a non-matching identity, a missing API private-key file, or missing updater signing inputs. After Tauri builds and notarises, it requires exactly one app bundle and one DMG and verifies code signing, Gatekeeper assessment, the stapled notarisation ticket, and DMG assessment.

## Clean-machine acceptance record

Use a test account or clean supported Mac. Record the OS and architecture, artifact version and SHA-256, certificate authority/team, notarisation request result, and each result below without recording credentials.

1. Download the exact immutable DMG over HTTPS and verify its recorded SHA-256.
2. Confirm Gatekeeper accepts the DMG and open it normally.
3. Drag Estate Studio to Applications and launch it from Finder without bypassing Gatekeeper.
4. Create a disposable company and project, import a small permitted source file, close the app, relaunch, and verify persistence.
5. Install a signature-verified update from the prior release and confirm app-data persistence and update health.
6. Quit the app and move only `/Applications/Estate Studio.app` to Trash. Do not delete app data.
7. Reinstall the same signed/notarised build and verify the disposable project still exists.
8. For a full-removal test only, export anything needed first, then remove the dedicated Estate Studio app-data directory and verify a fresh library on reinstall. Record the exact directory removed and recovery status.

Task acceptance requires the signed/notarised artifact and this clean-machine evidence. A successful local compile, an available identity, or an Apple submission alone is insufficient.
