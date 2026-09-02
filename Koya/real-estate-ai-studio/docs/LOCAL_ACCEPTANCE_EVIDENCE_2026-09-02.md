# Local acceptance evidence — 2026-09-02 AEST

This record covers Task 45 on the exact locally packaged QA bundle at `src-tauri/target/debug/bundle/macos/Estate Studio QA.app`. The QA overlay uses `com.landiq.estatestudio.qa`, so it cannot be confused with or write into the installed production-ID app.

## Performance and memory

- Frontend production build passed. Output was 527,116 bytes JavaScript, 139,996 bytes CSS, and 448 bytes HTML. Vite reports the JavaScript chunk above its 500 kB advisory threshold; code splitting remains an optimisation, not an omitted result.
- The final debug QA app bundle occupied 57,320 KiB on disk.
- After native launch and project/settings navigation, `vmmap -summary` reported a 36.9 MiB physical footprint and 39.2 MiB peak for the main process. Aggregate WebKit/XPC memory is `unavailable` from this measurement and is not represented as zero.
- An automated 20-unit/300-asset fixture loads and navigates manifest metadata while all 300 referenced media files are intentionally absent, proving library/project metadata access does not eagerly load media bytes.
- Static-build tests enforce mobile, 4K, and 8K tiers, a 192 MiB decoded-texture ceiling, 2:1 dimensions, and still fallback. The runtime selects a constrained tier for low-memory/mobile clients before higher tiers.

## Accessibility

- The native accessibility tree exposes labelled Projects, production areas, Settings, search, company filter, update controls, licence state, gateway, analytics, publishing targets, secret references, and project actions.
- The tour runtime has labelled panorama, floor plan, room navigation, fullscreen, and status regions; floor plan and still fallback have text alternatives.
- Keyboard controls, visible `:focus-visible` outlines, reduced-motion behavior, and keyboard-operable panorama/floor-plan controls have automated coverage.
- Sampled primary text pairs meet WCAG AA 4.5:1: body 13.18:1, primary muted body 5.06:1, sidebar body 15.46:1, and sidebar controls 7.96:1. This is a sampled contrast check, not a claim that every arbitrary customer brand colour is compliant.

## Failure recovery and security

- The first native launch exposed and then fixed a real updater-plugin startup panic caused by a missing base updater object. The exact QA bundle subsequently launched successfully.
- `Check signed update` on the unsigned QA build returned `Signed updater public key is unavailable in this build`; it did not download, install, or claim an update.
- Automated recovery tests retain job idempotency/checkpoints, prevent duplicate charges and duplicate tasks, reject invalid state transitions, and keep current installations usable on update failure.
- Generated frontend assets contain no matched private-key, updater-private-key, Apple-password, or API-key assignment. The native binary intentionally contains secret-pattern literals used by the fail-closed scanner; no secret value was found or supplied.
- Project roots are now migrated to mode `0700` on Unix and project manifests/company registries to `0600`. Tests assert the file modes. Portable exports reject traversal, symlinks, key-file extensions, environment files, and credential patterns.

## Isolated QA DMG install lifecycle

A fresh debug disk image was built from the same QA overlay at `src-tauri/target/debug/bundle/dmg/Estate Studio QA_0.1.0_aarch64.dmg`. The current repaired artifact is 16,724,049 bytes with SHA-256 `2baf2460a47d39c04a904e4594a5d7755bf7a53e430ce3913b5b3a23ec11a371`.

- `hdiutil attach -readonly` verified the disk-image CRC and mounted it at a newly created `/private/tmp/estate-studio-dmg-mount.*` path.
- The exact `Estate Studio QA.app` was copied to a newly created `/private/tmp/estate-studio-install-qa.*` directory rather than `/Applications`; its executable SHA-256 matched the mounted source byte-for-byte: `6c87934364fa11f8ef526f5a36df32b20dbcb74f2049d258294943afae611ec6`.
- The executable launched from the temporary installed path. Native accessibility read-back showed Koya, Harbourlight Residences, Koya Official Intake QA, and Cedar QA Clean Template. After quit and relaunch from the same installed path, all four projects remained visible.
- The installed process path was confirmed under the isolated temporary directory. The app was then quit, the image detached, and only the two newly created temporary directories were removed. The four QA project manifests remained intact. The authoritative QA `.app` path was rebuilt and relaunched after the DMG bundler cleaned its intermediate app.
- The first disk image exposed a real packaging defect: only the Mach-O linker signature existed, so the app reported `code has no resources but signature indicates they must be present`. The QA Tauri overlay now explicitly signs with identity `-`, creating bundle-level `_CodeSignature/CodeResources` before DMG assembly. The repaired app copied from the DMG passes `codesign --verify --deep --strict`, reports identifier `com.landiq.estatestudio.qa`, `Signature=adhoc`, and no TeamIdentifier, and launched from its temporary installed path with all four projects visible.
- `npm run tauri:qa` now reproduces DMG build, app build, strict signature/identifier verification, DMG checksum verification, and the expected Gatekeeper rejection. Gatekeeper must continue to reject this non-notarised QA package; this remains local installation evidence only, not a Developer ID signed/notarised customer installer or clean-machine acceptance.

## Novice usability and persistence

Using only visible native controls, without source editing or a terminal workflow, the operator:

1. opened the read-only Koya example library;
2. opened Settings and saw plain-language licence, update, subscription, publishing, analytics, secret-reference, and Codex states;
3. created `Harbourlight Residences` for the unrelated `Sunward Property` company in `Newcastle, NSW` with unit types `A1` and `A2`;
4. saw the guided checklist identify the next safe action as importing a readable floor plan;
5. quit and relaunched the exact QA app; and
6. confirmed Koya and Harbourlight both remained visible as separate projects, with Harbourlight retaining two unit types and its company/location.

No paid generation, external publishing, domain change, credential installation, or public mutation occurred during this local acceptance run.

## Tier B end-to-end native tour run

The unrelated Harbourlight project was then exercised further through the same exact QA bundle, using only deterministic, locally generated, rights-safe fixtures. The fixture generator is `scripts/create-tier-b-fixtures.mjs`; its ignored output is `test/fixtures/generated/tier-b/`, and `RIGHTS.md` records that the files are synthetic product-test inputs with no Koya or customer source material.

- Native evidence intake imported and accepted two 1600×1000 floor plans, one 1600×900 render, one exact 4096×2048 panorama, and one 1280×720 two-second H.264 video.
- The render, panorama, and video remained visibly classified `Concept Style Only`. Their usage permission states local product acceptance only; they were never relabelled as an official property representation.
- A1 received two operator-labelled, topology-neutral stops (`QA Zone 1` and `QA Zone 2`). The assisted draft retained those labels, proposed one adjacency/opening and an entrance, remained explicitly unapproved, and was separately reviewed and locked.
- A2 received one operator-labelled stop (`QA Zone A`), an assisted entrance candidate, and a separately locked one-room graph.
- Before panorama assignment, the native room cards visibly reported missing identity and panorama coverage. This exercised the missing-room-view recovery state.
- All three stops received the accepted style-only identity anchor and the validated exact 2:1 panorama. The app reported A1 with two ready stops and A2 with one ready stop.
- Native `Room media & delivery` processed the accepted source into a deterministic delivery set and retained the derivative paths on the source record: seam-repaired master 4096×2048 (`39cfdaa0…b1b7e`), mobile 2048×1024 (`8d9c2ee4…01339a`), 4K 4096×2048 (`39cfdaa0…b1b7e`), and 8K 8192×4096 (`2f368a0d…94f75`). The QA record identifies the 8K output as deterministic resampling with no new-detail claim.
- A second independent 4096×2048 style-only fixture exercised the persisted background path. Within about one second of clicking Build, the native accessibility tree remained responsive and exposed `Processing in background…`; the manifest recorded one idempotent `panorama_processing` task at `running / decode_and_seam_repair`. The same task later reached `completed / 100% / derivatives_verified`, attempt 1, with no failure or resubmission. Its verified outputs were mobile 2048×1024 (`74894e22…354a11`), 4K 4096×2048 (`cdfbdab4…b94243`), and 8K 8192×4096 (`cbd4a9d9…971ae3`). Direct derivative IPC entry points are not exposed to the frontend; both generated and supplied panoramas use the persisted queue.
- The same native control assigned the accepted H.264 MP4 to A1 `QA Zone 1` as representative video without changing its `Concept Style Only` evidence class.
- The local tour opened in desktop mode, navigated from A1 `QA Zone 1` to `QA Zone 2` through the floor-plan control, returned to Zone 1, and switched to the phone-portrait preview. Rotation changed the native accessibility read-back from 180° to 198°. The representative video changed from `Play` to `Pause` and reported a two-second duration. The viewer retained the durable `CONCEPT STYLE ONLY` disclosure throughout.
- Advanced mode exposed the project ID and project-relative asset paths while the customer-facing tour remained unchanged.
- Native clean-structure duplication created `Cedar QA Clean Template` for `Riverside QA` with two generic unit workspaces, zero evidence files, no customer facts, no room graph, and no releases.
- After rebuilding, quitting, and relaunching the exact QA bundle, the library retained all three projects; Harbourlight retained six evidence files, the completed background derivative record, and both A1/A2 as `LOCAL TOUR READY`. Codex capability discovery briefly showed its binding check, then returned to connected without credential entry.

The local release control rejected Harbourlight with `No signed Estate Studio licence is installed.` Portable export reached the native Save panel and then failed closed for the same licence gate; no bundle file was written. Settings independently reported the offline licence as not installed, Codex CLI `0.151.0` bound through the official app-server account interface as `ChatGPT team · app-server · CONNECTED`, the managed subscription gateway not configured, and commercial actions blocked. The binding status omits account email and tokens and explicitly keeps image-generation capability as a separate check. Fresh release preflights also failed closed: macOS requires an explicitly selected `APPLE_SIGNING_IDENTITY` plus the protected notarisation/updater inputs, and Windows requires a real Windows host. These are verified product states, not inferred omissions.

## Local static URL deep-link run

The release control plane now exposes the licence-free validation build separately from the licensed immutable-release action. The visible buttons are `Build local URL preview` and `Build licensed release`; a local preview is explicitly described as neither exported nor published.

- The exact QA app built Harbourlight preview `preview-1788283506` through the native deployment screen. No release record was created.
- The immutable build inventory contains 12 files and passed manifest schema, project-relative path, room-graph reference, accepted-evidence label, decoded-dimension, 128 MiB peak texture, mobile/4K tier, still fallback, secret-scan, and SHA-256 inventory checks.
- When a room has no separately curated still fallback, the builder reuses its already accepted panorama source as the deterministic decodable fallback. It does not generate, relabel, or invent media.
- A loopback HTTP server exposed the exact validated directory only for this QA run. A browser loaded `?unit=a1&room=qa-zone-1&mode=panorama`; visible state reported `Unit a1, QA Zone 1, panorama mode loaded.`
- Clicking the adjacent-room control changed the address to `?unit=a1&room=qa-zone-2&mode=panorama&texture=auto` and visible state to `QA Zone 2`.
- Selecting Unit a2 changed the address to `?unit=a2&room=qa-zone-a&mode=panorama&texture=auto` and visible state to `Unit a2, QA Zone A, panorama mode loaded.` Browser console errors remained empty.
- This proves stable unit/room URL state for criterion 10 only. It is not customer hosting, a public/unlisted release, TLS, logged-out external read-back, or a share/QR claim; criteria 11, 12, and 15 remain blocked.

## Approved local Codex audit

Criterion 17 requires an authenticated Codex connection and one approved test without exposing Codex credentials; it does not require the separately gated paid image provider. The PRD's immediate action explicitly calls for a streamed read-only project audit with no credential recorded in the project.

- In the exact QA app, Settings visibly reported `OpenAI Codex`, `codex-cli 0.151.0 · ChatGPT account`, `CONNECTED`, and `Ready for project-aware chat and human-reviewed project-profile drafts.` The managed subscription remained `NOT CONFIGURED`, keeping the two providers distinct.
- With Harbourlight selected, the operator submitted one explicitly bounded request: read-only audit the accepted evidence, both units' local-tour status, and publication blockers; do not propose/save fields, generate images, or submit a Provider task; render missing facts as `unavailable`.
- The native UI streamed `Reviewing project context…` and then returned a structured audit. It identified the two official accepted floor plans, four accepted but `concept_style_only` media assets, A1 with two ready QA zones, A2 with one ready QA zone, and the local preview as validation rather than publication.
- The response listed missing disclosure, source-review completion, unit property details, approved property imagery, creative output, deployment, release, publication approval/destination/account/timezone/date/public URL, and public read-back. Missing facts were labelled `unavailable` rather than inferred.
- The response ended: `No files, fields, provider tasks, or generation jobs were changed or submitted.` No project-update draft appeared.
- The authoritative `project.json` remained last modified at `2026-09-02 03:25:07 +1000`, before the audit. It retained six accepted assets, two locked/tour-ready units, zero generation jobs, one pre-existing completed background job, and zero releases. No project file changed during the audit, and a scan found no API-key, access-token, refresh-token, authorisation, bearer, or password field pattern in the project root.

This passes the Codex-specific criterion. It does not approve an image-generation package, prove Codex image capability, configure the managed subscription, switch providers, or satisfy criteria 6, 18, 20, or 21.

## Koya official-render intake

Criterion 3 also requires an official render rather than only synthetic style evidence. The repository's `references/SOURCE_REGISTER.md` identifies `production-assets/assets/official/` as user-provided official material (`SRC-007`), while `production-assets/ASSET_BIBLE.md` records the 6000×4000 two-bedroom living/kitchen image as A-level official truth source `OFF-INT-001`.

- Through the exact QA app, the operator created the isolated writable project `Koya Official Intake QA` (`koya-official-intake-qa-1788282526`) for company `Koya`; the bundled Koya example remained read-only and unchanged.
- Native intake selected `Official renders` and `Official supplied file`, recorded source owner `Koya developer-provided official material · SRC-007`, and restricted usage to internal local product acceptance because commercial redistribution remains unavailable.
- `koya-2br-living-kitchen.jpg` decoded as 6000×4000 and moved from `IMPORTED` to `OFFICIAL / ACCEPTED` through a separate visible operator action.
- The original and project copy are both 3,218,456 bytes with SHA-256 `7a1f0bf3ddedaeafd6be04b8a4396d7a3bd800d21e215b5dbf621850b256f7cd`. The original file was not modified.
- After quitting and relaunching the exact QA bundle, the library exposed four isolated projects and the writable Koya QA project retained one `OFFICIAL / ACCEPTED` evidence file. No external publication or redistribution claim was made.

## Failed-input recovery

The source importer now validates every selected file before copying any member of the batch into the project. Backend category-extension rules mirror the visible file picker; PNG, JPEG, and WebP inputs must decode with non-zero dimensions, while supported document/video/container formats require their expected file signatures. If a later batch copy fails, already copied members are removed before returning the error.

- The deterministic ignored fixture set includes `broken-image.png`, a 77-byte non-image payload with a PNG filename and no customer content.
- In the exact native QA app, the operator selected that file under `Official renders` and clicked Import. The form remained open and displayed: `broken-image.png cannot be decoded as an image. Choose an intact PNG, JPEG, or WebP file and try again.`
- The authoritative project manifest still contained exactly one pre-existing asset, and no `broken-image.png` existed anywhere in the project root.
- Without technical support or leaving the evidence form, the operator chose the valid A-level `koya-2br-kitchen.jpg`, restored the recorded `SRC-007` owner and internal-only permission, imported it, and separately accepted it.
- The recovered file decoded at 6000×4000. Its original and project copy share SHA-256 `2e71d9a9e30571e3c44f599f25c52f9df5fcb0e6623847a40238c6778e1f0feb`; the project now contains two official accepted files and no failed-input residue.
