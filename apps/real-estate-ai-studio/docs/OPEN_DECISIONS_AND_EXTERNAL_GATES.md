# Open decisions and external gates

Status date: 2026-09-02 AEST. `Unavailable` means no authoritative value or credential was supplied; it never means zero. This register is the release gate for product, rights, provider, account, domain, signing, and external-service decisions.

## Resolved implementation decisions

| Area | Resolution |
| --- | --- |
| Product boundary | Local-first desktop authoring; vendor-operated authoring storage/hosting is not part of the MVP. |
| Standard workflow | Property-language flow hides API keys, model IDs, prompts, token counts, JSON, terminal, and deployment commands. |
| Advanced workflow | Exact technical generation, evidence, state, and QA records are visible without changing accepted customer results. |
| Missing information | Render as `unavailable` or `not supplied`; never infer zero or a fact. |
| Koya example | Versioned, read-only, `internal_only`; blocked from portable commercial export until rights approval changes. |
| Project isolation | Separate company registry entries and project roots; Unix project roots `0700`, manifests/registries `0600`; bundles use portable relative paths and secret scanning. |
| AI credentials | Codex uses the supported local authentication flow. Customer OpenAI keys are not accepted. Managed provider secrets stay server-side. |
| Generation | One exact approved package authorises one provider task and one output. Capability, current price, quota, rights, and fingerprint must all be current. |
| Publishing | Local build, upload, deployment read-back, final verified release, and public state are separate gates. Releases are immutable. |
| Updates | Stable/beta channels; check is separate from install; Tauri signature verification is mandatory; previous signed package is retained for recovery. |
| Analytics | First-occurrence allowlist only; default network-off; optional credential-free same-origin path; no referrer, user agent, cookies, or cross-origin endpoint. |

## Product and commercial decisions

| Decision/gate | Current authoritative state | Owner / evidence required to close |
| --- | --- | --- |
| Customer-facing product name and legal publisher | `Estate Studio` / publisher legal display unavailable | Product/legal owner approves exact names before signing metadata and release notes. |
| Version and release channel for first customer build | Source is `0.1.0`; release designation unavailable | Product owner selects immutable version and stable/beta status. |
| Supported minimum macOS version | Unavailable; no explicit product decision recorded | Product owner selects supported OS range and clean-machine matrix; set Tauri minimum version and retest. |
| Supported Windows editions/builds | Unavailable | Product owner selects Windows 10/11 range, x64/ARM scope, WebView2 policy, MSI/NSIS support. |
| Licence editions, prices, seat counts, and entitlement contract | Edition gates exist; commercial amounts and customer licence unavailable | Commercial/legal owner supplies signed entitlement schedule and production licence. |
| Customer terms/privacy/source-permission wording | Storage fields exist; production text unavailable | Legal/rights owner approves exact versioned documents and acceptance record. |
| Analytics retention/controller/processor policy | Network analytics disabled; retention/account unavailable | Privacy owner approves policy and customer endpoint contract or keeps disabled. |
| Support, backup, recovery, and installer-retention SLA | Workflow documented; customer SLA unavailable | Product/operations owner approves support and retention periods. |

## Rights and content gates

| Decision/gate | Current authoritative state | Owner / evidence required to close |
| --- | --- | --- |
| Koya commercial redistribution | Blocked; current rights audit says `internal_only` | Rights owner supplies explicit commercial redistribution approval for exact assets. |
| Harbourlight/Sunward acceptance fixture | Synthetic metadata only; no customer media or commercial rights claim | Test operator may add explicitly test-only generated fixtures; customer use requires a real source register and permissions. |
| Paid generation inputs | Unavailable | Rights reviewer approves exact floor plan, render/photo/panorama inputs, owners, permissions, and evidence classes. |
| Exterior/views/landmarks/finishes/legal claims | Unavailable unless separately evidenced | Customer/design/legal reviewer supplies exact approved evidence and disclosure. |
| Customer disclosure | Default is `Disclosure not supplied` | Customer/legal reviewer supplies and approves exact publishable disclosure. |

## Provider, price, quota, and account gates

| Decision/gate | Current authoritative state | Owner / evidence required to close |
| --- | --- | --- |
| Image provider/model | Unavailable | Provider operator supplies an authenticated capability record naming exact provider/model. |
| Native panorama dimensions/mode | GPT Image 2 documentation permits 3840×1920 as an exact 2:1 request; live account capability, price, and quota remain unavailable | Capability read-back must confirm exact 3840×1920 one-shot 2:1 support or declare unavailable. Accepted output is then processed into deterministic 4K/8K delivery derivatives. |
| Current provider price and currency | Unavailable | Fresh provider/account evidence with expiry; no estimate permitted. |
| Provider quota/allowance | Unavailable | Fresh authenticated quota evidence sufficient for exactly one approved output. |
| Managed gateway URL and deployed revision | Unavailable | Service owner deploys the gateway and supplies credential-free HTTPS base URL plus revision/read-back. |
| Server-side provider credential | Unavailable and must never enter desktop/project/Git | Gateway operator configures it only in protected server environment. |
| Managed subscription/customer identity | Unavailable | Commercial service issues subscription/activation contract; desktop exchanges it for a short-lived in-memory token. |
| Top-up/payment authorisation | No live account/payment supplied | Authorised billing system confirms payment; payer self-report is not fulfilment evidence. |
| Completed usage/internal-cost reconciliation | Unavailable without a live completed task | Gateway/provider read-back supplies request fingerprint, usage, and internal cost; gaps remain `unavailable`. |

## Signing, packaging, and update gates

| Decision/gate | Current authoritative state | Owner / evidence required to close |
| --- | --- | --- |
| macOS Developer ID identity | One valid `Developer ID Application: JR Academy Pty Ltd (X5YTQTN7ZM)` is installed; not explicitly selected for release | Release owner authorises exact identity via `APPLE_SIGNING_IDENTITY`. |
| Apple notarisation authentication | Unavailable | Release owner supplies one complete protected Tauri-documented credential set. |
| macOS signed/notarised artifact | Unavailable; existing release DMG is unsigned/rejected and must not be distributed | Run protected macOS release workflow and clean-machine acceptance. |
| Windows native host | Unavailable | Provide Windows test/build host meeting documented prerequisites. |
| Windows signing certificate/timestamp | Unavailable | Release owner supplies exact installed thumbprint and CA HTTPS timestamp URL on Windows. |
| Windows MSI/NSIS artifact and acceptance | Unavailable | Run protected native build, Authenticode verification, install/update/uninstall tests. |
| Updater public/private key pair | Unavailable | Release owner provisions keys; private key stays only in protected release environment. |
| Stable/beta updater endpoints | Unavailable | Release owner supplies credential-free HTTPS endpoints and immutable artifact/signature storage. |
| Prior signed installer retention | Unavailable because no signed release exists | Release operations retain current and previous signed/notarised installers and hashes. |
| Production offline licence issuer/private key | Private signing infrastructure unavailable and must not ship | Commercial security owner issues exact customer licence; desktop contains public verification key only. |
| Second signed licensed installation | Unavailable | Provide isolated test installation for portable handover criterion 28. |

## Hosting, domain, and public verification gates

| Decision/gate | Current authoritative state | Owner / evidence required to close |
| --- | --- | --- |
| Customer-owned publishing adapter target | No directory/account configured | Customer/host owner supplies target and authorises a publish action. |
| Hosting provider/account | Unavailable | Customer chooses and authorises provider/account; secrets remain external file references. |
| Public hostname/domain | Unavailable | Customer supplies exact hostname and DNS authority. |
| TLS certificate and HTTPS readiness | Unavailable | Hosting/DNS read-back proves valid public HTTPS. |
| Access mode | No Harbourlight release decision | Publisher explicitly selects public, unlisted, or private link for the exact release. |
| Deployment/public URL | Unavailable | Successful upload followed by cookie-free HTTPS read-back. |
| Representative public assets/navigation/disclosure | Unavailable without deployed URL | Read-back verifies floor plan, panorama, representative video, disclosure, unit/room navigation, and access mode. |
| Final release/share links/QR | Unavailable | Publisher entitlement plus passing deployment read-back creates the verified immutable record. |

No item in this register authorises a paid task, credential creation, signing submission, publication, domain mutation, deployment, commit, or push. Those actions require the named authority and exact current evidence.
