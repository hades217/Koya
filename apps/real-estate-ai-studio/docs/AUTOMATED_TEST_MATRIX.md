# Automated test matrix

Run `npm run verify` from the Estate Studio directory. The command builds the frontend and then runs all automated suites.

| Required area | Automated evidence |
| --- | --- |
| Unit | Rust tests cover deterministic helpers, state transitions, signatures, dimensions, QR generation, and URL validation. |
| Integration | Browser-local manifest migration tests, official Codex app-server account-response parsing, Rust portable-bundle round trip, immutable static export, settings persistence contracts, and Node gateway service tests cross component boundaries. |
| Migration | Legacy schema fixtures deserialize, normalize to the current schema, and retain conservative defaults. Browser localStorage records also migrate from v26 to current v30 without mutating the parsed source, while existing profile, workflow, skill, and analytics records are retained. |
| Property-based path isolation | Proptest generates arbitrary unit identifiers; only one bounded portable ASCII path component is accepted. Directory creation repeats the same validator. |
| Duplicate jobs | Stable background-job idempotency identities and gateway replay/conflict tests prove exact retries do not create or charge a second job. |
| Secret scan | Bundle tests reject environment files, private-key extensions, traversal, absolute paths, symlinks, and known credential patterns. |
| Static runtime | Node verifies syntax, local-only dependencies, manifest loading, DOM contracts, accessibility fallback, and same-origin credential-omitting analytics. Rust validates dimensions, memory limits, checksums, deep links, and referenced assets. |
| macOS QA packaging | `npm run tauri:qa:verify` requires a strict-valid bundle-level ad-hoc signature, exact QA bundle identifier, valid DMG checksum, and Gatekeeper rejection so the QA artifact cannot be mistaken for a production release. |

Developer ID/Authenticode release packaging, Gatekeeper/SmartScreen acceptance, notarisation, clean-machine installation, browser rendering, and external deployment read-back remain acceptance tests rather than unit-test claims.
