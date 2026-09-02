# Windows signed release acceptance

Estate Studio Windows releases must be built and tested on Windows. A Rust target installed on macOS or a cross-compiled executable is not installer evidence. Tauri documents native MSI and NSIS packaging at <https://v2.tauri.app/distribute/windows-installer/> and Windows signing at <https://v2.tauri.app/distribute/sign/windows/>.

## Protected native build gate

On a Windows 10/11 x64 test/build host with WebView2, Visual Studio C++ Build Tools, Rust, Node, and WiX/VBSCRIPT support for MSI, install an appropriate code-signing certificate in the current user's certificate store. Set:

- `ESTATE_STUDIO_WINDOWS_CERTIFICATE_THUMBPRINT` to its exact thumbprint;
- `ESTATE_STUDIO_WINDOWS_TIMESTAMP_URL` to the certificate authority's credential-free HTTPS timestamp URL;
- all updater values documented in `UPDATES_AND_RECOVERY.md`.

Run `npm run windows:release:preflight`, then `npm run windows:release`. The preflight requires Windows, verifies the exact certificate exists, rejects incomplete updater inputs, and writes only public signing metadata to an ignored mode-restricted overlay. The build produces MSI, NSIS, and signed updater artifacts; verification requires exactly one current MSI and NSIS installer and a valid Authenticode signature on each.

## Clean Windows acceptance record

Record Windows version/build, x64 architecture, WebView2 version, artifact version and SHA-256, signer subject/thumbprint, timestamp status, and each result below without recording credentials.

1. Download the exact immutable installer over HTTPS and verify SHA-256.
2. Confirm Windows reports the expected verified publisher and no invalid-signature warning.
3. Install per-user through NSIS, launch from Start, create a disposable company/project, close, relaunch, and verify persistence.
4. Uninstall from Installed apps. Verify the executable and shortcuts are removed while app data remains.
5. Reinstall through MSI and verify the disposable project persists.
6. From the prior signed version, install a signature-verified updater release and confirm version, launch health, and data persistence.
7. Exercise a failed/offline update and confirm the existing installation remains usable without a duplicate job or credit event.
8. Only for explicit full-removal acceptance, export required data, remove the dedicated Estate Studio app-data directory, reinstall, and verify a fresh library. Record the exact path and recoverability.

Task acceptance requires native signed artifacts plus install, launch, persistence, update, and uninstall evidence. Build success alone is insufficient.
