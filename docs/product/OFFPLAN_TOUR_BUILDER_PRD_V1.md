# Off-plan Tour Builder PRD

| Field | Value |
| --- | --- |
| Document | Product Requirements Document |
| Version | V2.3 - Multi-project local workspace |
| Status | Draft aligned to local-first authoring, subscription AI, and zero-setup onboarding |
| Date | 2026-08-31 |
| Reference implementation | Koya Apartment 106 Interactive Tour |
| Initial platforms | macOS desktop, Windows desktop, public mobile/desktop web |
| Primary market | Off-the-plan residential developers, project marketers, real-estate agencies |

## 1. Product summary

Off-plan Tour Builder is a desktop application that turns approved project drawings, floor plans, renders, photos, videos, and brand assets into a mobile-ready interactive property tour.

The authoring product is local-first and does not require a Koya-operated web authoring SaaS. Project files, source assets, manifests, generated derivatives, review records, and website builds remain in a customer-selected local workspace. AI work uses either the customer's own Codex access or a vendor-managed OpenAI subscription. The vendor charges for the desktop software licence, optional OpenAI subscription, upgrades, support, and consulting services.

The product guides a non-technical property professional through:

`Create project -> Import evidence -> Audit floor plan -> Confirm room graph -> Generate missing imagery -> Review rooms -> Build tour -> Publish -> Verify`

The published experience supports:

- development and unit-type switching;
- floor-plan hotspots;
- room-to-room navigation based on physical adjacency;
- draggable and touch-enabled 360-degree panoramas;
- optional scroll-controlled videos;
- responsive mobile and desktop layouts;
- visible concept and evidence disclosures;
- a public URL and QR code.

The product is not a measured 3D scan and must not claim six-degree-of-freedom movement unless surveyed geometry or a real 3D model is supplied.

### 1.1 Target experience principle

The default product experience is designed for property professionals with no AI, API, software-development, or image-processing knowledge. A standard customer must never be asked to create an API key, select a model slug, understand tokens, edit a prompt, install a runtime, run a terminal command, choose panorama stitching parameters, or configure a deployment build.

Customer-facing language uses property outcomes:

- `Create development`;
- `Add unit type`;
- `Upload floor plan and renders`;
- `Confirm rooms and doors`;
- `Generate room image`;
- `Generate 360° panorama`;
- `Review`;
- `Publish`.

Technical details remain available in an advanced audit drawer for consultants and support, but are not part of the primary journey.

### 1.2 Project-library principle

The desktop application manages a local library of independent property projects. Koya is one development project in that library, not the application itself and not a global asset folder.

The hierarchy is:

`Desktop installation -> Local project library -> Customer company -> Development project -> Unit types -> Tour stops and assets -> Release versions`

For the current example:

`Local project library -> Koya company/client profile -> Koya development project -> Apartments 102, 103, 104, and 106 -> Apartment 106 tour assets and releases`

Every development project owns its source register, floor plans, generation records, room graphs, panoramas, videos, manifests, QA reports, builds, and release history. Creating a new project creates a new root directory and new project ID. No asset, fact, prompt, room graph, disclosure, or brand setting from Koya may flow into another project unless the user intentionally imports an approved reusable template component.

### 1.3 Panorama production principle

The commercial product should not assume the low-cost POC production path. When an approved image API can return one sufficiently large 2:1 image, the preferred paid path is one complete panorama generation from the locked room identity and floor-plan constraints. This maximises global room coherence and avoids treating several independently attractive images as one continuous space.

The one-shot output must still pass equirectangular projection, horizon, topology, left/right wrap seam, and decoded-resolution QA. A provider's `4K` label alone does not prove that the image is a valid 360-degree panorama. Deterministic seam repair and super-resolution may be applied after acceptance; multi-face or overlapping-tile generation is a fallback when the one-shot result cannot pass QA.

## 2. Problem statement

Developers and agents commonly receive an inconsistent package of off-the-plan assets:

- floor plans and drawing PDFs;
- a small number of hero renders;
- material schedules and brand guidelines;
- isolated room images;
- occasional videos;
- several unit types with incomplete visual coverage.

Producing an interactive tour from these inputs currently requires coordination across design review, image generation, panorama processing, frontend development, mobile optimisation, hosting, and QA. Existing workflows are too technical for most property teams and make it easy to publish spatially incorrect or weakly disclosed AI imagery.

The opportunity is to productise the proven Koya workflow as a guided desktop tool while keeping factual and spatial approval under human control.

## 3. Product vision

Enable a property professional to produce a credible, branded, publicly shareable concept tour without writing code, while preserving a traceable relationship between every published visual and its official source material.

### 3.1 Product promise

"From approved off-the-plan documents to an interactive sales experience, with every room reviewed before publication."

### 3.2 Commercial product model

The same desktop application supports three commercial delivery modes:

1. **Self-service desktop licence**: a developer, agency, or creative studio buys the application and operates the workflow locally.
2. **Desktop plus assisted production**: the customer owns the project workspace and AI account while the consultant configures the project, prepares generation packages, and resolves QA issues for a project fee.
3. **Managed consulting**: the consultant uses the same desktop application to deliver the complete project from drawing audit to verified publication, charging discovery, production, revision, deployment, and support fees.

These modes use the same portable project bundle, audit records, generation gates, and static publishing pipeline. A consulting project can be handed to a licensed customer without rebuilding the tour or transferring data through a vendor cloud.

### 3.3 AI connection and billing model

The application provides two selectable AI connections:

1. **Codex connection**: the application integrates local Codex through the supported SDK or app-server interface. The customer signs in through Codex using eligible ChatGPT subscription access or API-key access. Codex availability, usage limits, model access, and tool availability remain governed by the customer's OpenAI account.
2. **Managed OpenAI subscription**: the customer signs in to the application's lightweight subscription service and receives an included generation allowance. The vendor's server-side AI gateway validates entitlement and remaining allowance, invokes the OpenAI API, records billable usage, and returns the result to the desktop application. The vendor's OpenAI key is never distributed to the desktop client.

The desktop licence and OpenAI subscription are separate products. Codex users can buy the desktop licence without the managed OpenAI subscription. Managed OpenAI subscribers pay monthly or annually for an included allowance, with explicit top-up or overage rules. The application shows remaining allowance and the exact credit cost before an approved generation task.

For either connection, the application must perform a capability check before promising image generation. If the selected Codex environment does not expose the required image capability, the UI must show `unavailable` and offer the managed OpenAI subscription instead of silently changing providers.

### 3.4 Automation position

The target is high automation with explicit approval gates, not unattended publication.

The system may automate:

- file classification and metadata extraction;
- floor-plan room suggestions;
- room-graph and hotspot proposals;
- asset-gap analysis;
- image-generation job preparation;
- deterministic panorama assembly and super-resolution;
- responsive tour assembly;
- deployment and technical verification.

The user must confirm:

- project and unit identity;
- floor-plan orientation and room labels;
- openings and room adjacency;
- image-generation inputs, cost, and task count;
- generated room identity and spatial correctness;
- disclosures and marketing copy;
- final publication.

## 4. Goals and non-goals

### 4.1 MVP goals

1. Allow a non-technical user to create and manage multiple developments and unit types.
2. Convert a readable floor plan into a user-confirmed room graph and navigable hotspot map.
3. Inventory supplied media and identify missing tour coverage by room.
4. Generate or edit missing room imagery through an image model after explicit approval.
5. Prefer a paid, single-output 2:1 panorama when the selected API supports sufficient custom dimensions, then produce responsive 4K and 8K delivery derivatives from the accepted source.
6. Assemble the approved assets into a reusable Koya-quality tour template.
7. Provide desktop and mobile preview modes with actionable QA results.
8. Publish a public or private-link tour and verify the deployed experience.
9. Preserve asset provenance, versions, rejection history, and approval status.
10. Allow a first-time property professional to complete the standard workflow without API credentials, command-line tools, model selection, prompt writing, or deployment knowledge.

### 4.2 MVP non-goals

- Fully autonomous interpretation and publication without human review.
- Claiming a generated tour is an as-built survey, digital twin, or measured 3D scan.
- Free movement through a flat panorama.
- Automatically generating unverified exterior views, landmarks, dimensions, finishes, or legal claims.
- Full BIM authoring or CAD editing.
- Multi-user real-time editing.
- Automated long-form video generation.
- CRM replacement, listing syndication, or lead management.
- Native mobile authoring applications.

## 5. Target users

### 5.1 Project marketing manager

Needs to launch consistent, branded experiences across many unit types and ensure all externally visible material has been approved.

### 5.2 Real-estate agent

Needs to turn existing floor plans and renders into a shareable experience quickly, without managing code or hosting.

### 5.3 Developer design or approvals representative

Needs to verify that AI-assisted visuals do not contradict plans, openings, material schedules, or approved design intent.

### 5.4 Creative studio operator

Needs advanced controls over source roles, generation versions, panorama processing, and client approvals while reusing one production system across projects.

### 5.5 Platform operator / consultant

Needs to create separate local client workspaces, deliver premium production services, request approvals, track billable work, publish under the client's brand, and hand over a portable project bundle to a licensed client.

## 6. Input readiness tiers

The onboarding audit assigns one of the following readiness levels.

### Tier A: production-grounded

Inputs include an audited drawing set, official unit metadata, broad room coverage or surveyed/rendered panoramas, brand assets, and approved disclosure wording.

Output: high-confidence interactive tour, subject to final human and device QA.

### Tier B: floor-plan-grounded concept

Inputs include a readable floor plan, enough official imagery to define material direction, clear unit identity, and permission to generate missing imagery.

Output: complete concept tour whose topology is controlled by the floor plan. Generated spaces, furniture, fixtures, finishes, and views remain explicitly illustrative.

### Tier C: style-led prototype

Inputs contain style imagery but lack an adequate floor plan or opening information.

Output: room-by-room pitch prototype only. The system must block claims of accurate adjacency, whole-apartment continuity, or verified dimensions.

## 7. Core user journey

### Step 1: Create a development

The user enters:

- development name;
- project address, if approved for use;
- customer company profile;
- locale and measurement units;
- brand logo, colours, and typography;
- intended access mode: public, unlisted link, or private;
- approved concept disclosure.

### Step 2: Add unit types

The user may add one or more units and upload a floor plan for each. Unit cards show facts, readiness, coverage, review state, and publication state.

### Step 3: Import source material

The user drags files or folders into the project. The application classifies likely:

- floor plans and drawing sets;
- official renders;
- material and finish references;
- supplied photos and panoramas;
- room or route videos;
- brand and sales copy;
- unknown files requiring manual classification.

The application must never silently promote an inferred classification to approved evidence.

### Step 4: Audit the floor plan

The application proposes room boundaries, labels, openings, orientation, hotspots, and adjacency. The user corrects and locks the result.

The floor plan becomes the navigation source of truth. Every tour stop must occupy one confirmed coordinate and connect only to physically adjacent stops.

### Step 5: Build the asset matrix

For every stop, the application displays required and optional asset roles:

- room identity anchor;
- panorama;
- threshold view;
- reverse view;
- still fallback;
- optional video;
- video poster.

The application recommends the smallest asset set needed to make the route understandable.

### Step 6: Generate missing imagery

For an approved generation job, the application shows:

- exact unit and room;
- requested asset role;
- every input image and its role;
- intended camera location and direction;
- required openings and fixed fixtures;
- prohibited inventions;
- output count and dimensions;
- panorama generation mode: paid one-shot, supplied panorama, or stitched fallback;
- model and generation settings;
- current estimated price, or `unavailable`;
- resulting evidence class.

Generation cannot start until the user confirms the exact package. One approval authorises only the displayed task count and outputs.

In standard mode, the confirmation is expressed as a property outcome and subscription allowance, for example: `Living room 360° panorama · 1 output · 4 panorama credits`. Model, prompt, provider parameters, and technical dimensions are generated by the product and available under `Technical details`, not placed in the primary form.

### Step 7: Review and approve

The review workspace compares the output with the floor plan and accepted room identity. The reviewer may:

- accept;
- reject with a structured reason;
- request one targeted correction;
- compare versions;
- mark an asset as style-only;
- prevent a rejected asset from being reused.

Acceptance must not be inferred from generation success.

### Step 8: Assemble and preview

The application builds the tour manifest, web experience, texture tiers, fallback assets, and navigation UI. Users can preview:

- 15-inch desktop at 1x and 2x device pixel ratio;
- common mobile portrait and landscape sizes;
- low-memory texture fallback;
- touch, mouse, keyboard, and fullscreen interactions;
- public disclosures and unit switching.

### Step 9: Publish and verify

The user reviews a final publication summary and explicitly selects Publish. The application then:

1. creates an immutable release version;
2. deploys the tour;
3. checks the public or private URL;
4. checks the floor plan and representative media URLs;
5. performs a scripted navigation smoke test;
6. produces the share link and QR code;
7. records the verified deployment state.

A successful local build or HTTP response alone is not a verified publication.

## 8. Functional requirements

### 8.1 Project and unit management

| ID | Requirement | Priority |
| --- | --- | --- |
| PRJ-01 | Create, rename, duplicate, archive, export, and import a development project. | P0 |
| PRJ-02 | Add multiple unit types under one development. | P0 |
| PRJ-03 | Record unit facts separately from marketing copy. | P0 |
| PRJ-04 | Display readiness, asset coverage, review, build, and deployment state per unit. | P0 |
| PRJ-05 | Support reusable branding without copying Koya content or assets. | P0 |
| PRJ-06 | Keep each customer company's projects and assets in an explicit, separate local workspace. | P0 |
| PRJ-07 | Allow a studio or consultant installation to manage multiple local company profiles without mixing their branding or files. | P0 |
| PRJ-08 | Support project handover through a validated portable bundle that can be opened by another licensed installation. | P0 |
| PRJ-09 | Export the manifest, evidence register, approved assets, static website, and QA report subject to contract rights. | P0 |
| PRJ-10 | Display a project library with recent projects, customer company, unit count, readiness, last update, and publication state. | P0 |
| PRJ-11 | Create every new development in a new validated project root with a stable project ID and project manifest. | P0 |
| PRJ-12 | Prevent file paths, brand assets, facts, generation references, and release outputs from resolving across project roots by default. | P0 |
| PRJ-13 | Support `Create blank project`, `Create from approved template`, `Import project bundle`, and `Duplicate project structure without customer assets`. | P0 |
| PRJ-14 | Allow a development project to contain multiple unit types with independent floor plans, assets, QA, and availability states. | P0 |
| PRJ-15 | Mark Koya as an example project and prevent example content from being edited into a new customer project through ordinary template creation. | P0 |

### 8.2 Source intake and evidence

| ID | Requirement | Priority |
| --- | --- | --- |
| SRC-01 | Import PDF, PNG, JPEG, WebP, MP4, MOV, SVG, and approved text formats. | P0 |
| SRC-02 | Store original files unchanged and create working derivatives separately. | P0 |
| SRC-03 | Require an evidence class for every source and generated asset. | P0 |
| SRC-04 | Record source, checksum, dimensions, import time, owner, and usage permission. | P0 |
| SRC-05 | Flag missing, unreadable, duplicated, conflicting, or unusually small inputs. | P0 |
| SRC-06 | Preserve rejected versions and prevent their silent reuse. | P0 |

Evidence classes:

- `official`;
- `surveyed`;
- `approved_render`;
- `concept_floorplan_grounded`;
- `concept_style_only`;
- `unknown`;
- `rejected`.

### 8.3 Floor-plan audit and navigation

| ID | Requirement | Priority |
| --- | --- | --- |
| PLN-01 | Display the uploaded floor plan with zoom, pan, rotate, and crop controls. | P0 |
| PLN-02 | Suggest room labels, hotspots, entrances, openings, and adjacency. | P0 |
| PLN-03 | Require user confirmation before the room graph is locked. | P0 |
| PLN-04 | Allow manual hotspot and connection editing. | P0 |
| PLN-05 | Reject duplicate stop IDs, missing connections, out-of-bounds coordinates, and non-adjacent routes. | P0 |
| PLN-06 | Version floor-plan and room-graph changes. | P0 |
| PLN-07 | Warn when a locked room graph changes after imagery has been approved. | P0 |

### 8.4 AI asset production

| ID | Requirement | Priority |
| --- | --- | --- |
| AI-01 | Create a room-by-room asset-gap matrix. | P0 |
| AI-02 | Generate an identity anchor before dependent threshold, reverse, fallback, or panorama assets. | P0 |
| AI-03 | Label every generation input by role. | P0 |
| AI-04 | Present the exact generation package, model, output count, and price status before submission. | P0 |
| AI-05 | Record prompt, inputs, output dimensions, model path, cost, evidence class, and job state. | P0 |
| AI-06 | Support accept, reject, compare, and targeted-correction workflows. | P0 |
| AI-07 | Never automatically upgrade generated imagery to official or surveyed evidence. | P0 |
| AI-08 | Support local background job progress and safe resumption after application restart. | P1 |
| AI-09 | Support per-project local budget warnings and approved task-count limits. | P1 |
| AI-10 | Expose the provider's actual supported dimensions and aspect-ratio limits rather than using a generic `4K` claim. | P0 |
| AI-11 | Support a Codex connection through the local SDK or app-server and report its authenticated state and available capabilities. | P0 |
| AI-12 | Support a managed OpenAI subscription through a server-side gateway without distributing the vendor API key to the desktop application. | P0 |
| AI-13 | Allow the user to choose the AI connection per approved job and never switch providers silently. | P0 |
| AI-14 | Display subscription status, included allowance, remaining allowance, task credit cost, and top-up or overage rule before generation. | P0 |
| AI-15 | Use an idempotency key so timeout, restart, or retry cannot charge or submit the same approved task twice. | P0 |
| AI-16 | Default non-technical customers to managed OpenAI subscription mode with no API-key, model, token, or prompt setup. | P0 |
| AI-17 | Express customer charges as named property outputs and credits, while retaining exact provider cost internally. | P0 |
| AI-18 | Generate the technical prompt and provider parameters from the locked manifest and asset roles. | P0 |
| AI-19 | Place model, prompt, seed, dimensions, and provider diagnostics in an advanced audit drawer rather than the default workflow. | P0 |
| AI-20 | Keep Codex connection under an Advanced or Studio workflow and never require it for a standard managed-subscription customer. | P0 |

### 8.5 Panorama production

| ID | Requirement | Priority |
| --- | --- | --- |
| PAN-01 | Prefer one paid, complete 2:1 panorama generation when the active provider supports an adequate custom output size. | P0 |
| PAN-02 | Accept supplied equirectangular panoramas or continuity-controlled directional inputs as alternative sources. | P0 |
| PAN-03 | Validate 2:1 projection, topology, left/right wrap seam, horizon, orientation, and decoded dimensions. | P0 |
| PAN-04 | Treat provider generation success and a `4K` label as insufficient until the output passes visual and runtime QA. | P0 |
| PAN-05 | Apply deterministic seam repair and super-resolution only after the one-shot room content is accepted. | P0 |
| PAN-06 | Fall back to continuity-controlled cubefaces or overlapping tiles only when one-shot generation fails projection, topology, or seam acceptance. | P0 |
| PAN-07 | Produce deterministic mobile preview, 4K, and 8K delivery derivatives. | P0 |
| PAN-08 | Reject stretching a small square image into a panorama. | P0 |
| PAN-09 | Provide yaw, pitch, field-of-view, seam, and initial-view QA controls. | P0 |
| PAN-10 | Keep the still fallback available when WebGL or a high-resolution texture fails. | P0 |

### 8.6 Tour composition

| ID | Requirement | Priority |
| --- | --- | --- |
| TOUR-01 | Build the experience from a validated, presentation-independent manifest. | P0 |
| TOUR-02 | Support development, unit, room, panorama, and video state in shareable URLs. | P0 |
| TOUR-03 | Support floor-plan hotspots and adjacent-room arrows. | P0 |
| TOUR-04 | Support drag and touch panorama exploration, reset, fullscreen, and keyboard controls. | P0 |
| TOUR-05 | Support optional scroll-controlled room videos with ordinary playback controls. | P1 |
| TOUR-06 | Display unit-level and asset-level concept disclosures. | P0 |
| TOUR-07 | Support white-label themes and customer contact calls to action. | P1 |
| TOUR-08 | Support English initially and a translation-ready content model. | P1 |

### 8.7 Preview, QA, and publication

| ID | Requirement | Priority |
| --- | --- | --- |
| QA-01 | Run manifest, file-path, evidence, room-graph, and asset-dimension validation. | P0 |
| QA-02 | Provide desktop and mobile device preview presets. | P0 |
| QA-03 | Show the actual runtime texture URL and decoded dimensions. | P0 |
| QA-04 | Detect missing assets, blurry panorama risk, excessive texture memory, and broken fallback paths. | P0 |
| QA-05 | Require a reviewer to approve every publishable stop. | P0 |
| PUB-01 | Publish a versioned public, unlisted, or private-link release. | P0 |
| PUB-02 | Verify the intended access mode from a logged-out equivalent context. | P0 |
| PUB-03 | Verify representative panorama, floor-plan, and video assets after deployment. | P0 |
| PUB-04 | Preserve release history and support rollback to a previously verified release. | P1 |
| PUB-05 | Generate a QR code and shareable unit-specific link. | P0 |

## 9. Information architecture

The desktop application uses the following primary navigation:

1. **Projects**: local project library, recent developments, customer filters, templates, and import project.
2. **Development**: project identity, customer company, branding, disclosures, units, paths, and release status.
3. **Unit workspace**:
   - Sources
   - Floor plan
   - Rooms
   - Generate
   - Review
   - Tour
   - QA
   - Publish
4. **Production Queue**: generation, panorama processing, build, and deployment tasks.
5. **Settings**: company profiles, desktop licence, Codex connection, managed OpenAI subscription, local storage, publishing targets, and updates.

### 9.1 Standard and advanced modes

**Standard mode** is the default for agents and developers. It shows tasks, property outputs, allowance, status, visual comparisons, warnings, and approval actions.

**Advanced mode** is intended for the product operator, consultant, studio, or support engineer. It exposes evidence roles, prompt records, exact dimensions, provider capability, model identifiers, task fingerprints, seams, decoded textures, build logs, and deployment diagnostics.

Changing to Advanced mode does not remove approval gates or permit silent provider switching.

The first-run experience should include a read-only Koya example project when the necessary demonstration and distribution rights are confirmed. If those rights are not confirmed, ship an anonymised Koya-derived demonstration containing only synthetic or separately licensed assets. It must not package customer-confidential, unapproved, rejected, or production-only Koya source material.

### 9.2 Local project folder contract

Recommended project structure:

```text
<project-root>/
  project.json
  sources/
    source-register.json
    drawings/
    renders/
    videos/
    brand/
  units/
    <unit-id>/
      unit.json
      floorplan/
      room-graph.json
      assets/
        accepted/
        generated/
        rejected/
        panoramas/
        videos/
      tour-manifest.json
      qa/
  builds/
  releases/
  reports/
  cache/
```

`cache/` is disposable. Original sources, accepted assets, rejection history, manifests, QA, and releases are not cache and must survive rebuilds. Project-relative paths may not escape `<project-root>`.

## 10. Data model

```text
DesktopInstallation
  -> SignedLicence
  -> AIConnectionReference
  -> SubscriptionSessionReference
  -> LocalProjectLibrary
    -> CompanyProfile
      -> DevelopmentProject
        -> LocalWorkspace
        -> ServiceEngagement (optional local record)
        -> UnitType
          -> FloorPlanVersion
          -> RoomGraphVersion
            -> TourStop
              -> View
                -> AssetVersion
                  -> SourceRecord
                  -> GenerationRecord
                  -> QARecord
          -> TourManifestVersion
          -> Release
            -> DeploymentVerification
        -> LocalUsageLedger

MinimalSubscriptionService
  -> SubscriberAccount
  -> PlanAndEntitlement
  -> AllowanceLedger
  -> AIRequestFingerprint
  -> ProviderUsageAndCostRecord
```

### 10.1 Required state machines

Asset:

`imported -> classified -> review_required -> accepted | rejected -> publishable`

Generation job:

`draft -> awaiting_approval -> approved -> queued -> running -> succeeded | failed -> visual_review -> accepted | rejected`

Unit:

`intake -> floorplan_review -> asset_production -> tour_review -> qa_ready -> publish_ready -> published -> superseded`

Deployment:

`not_started -> building -> deploying -> deployed_unverified -> public_verified | verification_failed`

## 11. Manifest contract

The tour manifest is the content and navigation source of truth. Presentation code must not contain customer-specific room data.

Required top-level groups:

- project and locale;
- brand;
- developments and units;
- floor-plan source and dimensions;
- texture policy;
- ordered stops;
- disclosures;
- publication metadata.

Each stop requires:

- stable ID, label, and zone;
- normalized floor-plan coordinates;
- physically valid connections;
- one or more views;
- source path and evidence class per view;
- factual note and applicable disclosure;
- panorama delivery tiers and initial orientation when relevant;
- generation and QA metadata when relevant.

## 12. Technical architecture

### 12.1 Desktop client

Recommended shell: Tauri with a web-based React interface.

Responsibilities:

- project and local file management;
- source preview and classification;
- floor-plan annotation;
- job preparation and approval;
- panorama processing orchestration;
- local tour preview;
- background uploads and downloads;
- secure authentication to the product backend;
- signed application updates.

Bundled local helper processes may provide PDF rendering, image inspection, panorama conversion, deterministic super-resolution, and FFmpeg media preparation.

### 12.2 Local AI adapter layer

Responsibilities:

- detect and connect to the supported local Codex runtime;
- expose Codex authentication, threads, approvals, and streamed progress through a stable adapter;
- call the vendor AI gateway when the customer selects managed OpenAI subscription mode;
- maintain explicit provider choice, task ID, request fingerprint, and local job state;
- route approved local tools for PDF inspection, image processing, panorama QA, website builds, and deployment;
- prevent duplicate paid jobs after restart or timeout;
- report unavailable authentication, model, image tool, price, or quota without inventing a fallback.

The vendor's provider credentials must never be shipped in the desktop application. Desktop subscription credentials are short-lived and revocable and must not enter project bundles, manifests, prompts, logs, analytics, crash reports, or command-line arguments.

### 12.3 Minimal subscription and AI gateway

This is not a web authoring product. It exists only to provide:

- customer sign-in and subscription entitlement;
- plan, allowance, top-up, and cancellation state;
- short-lived desktop access tokens;
- server-side OpenAI authentication;
- idempotent AI request submission;
- per-request credit, provider usage, and cost records;
- rate limits and abuse controls;
- transient handling of explicitly approved generation inputs and outputs;
- subscription support and reconciliation.

The gateway and desktop client must hide infrastructure complexity from the normal customer. Sign-in, subscription selection, payment, allowance status, generation approval, and top-up must use ordinary product screens. No API dashboard, developer account, environment variable, terminal, or external credential setup is required.

The gateway must not become the source of truth for floor plans, manifests, approvals, or website projects. Only the exact inputs approved for a generation task are transmitted. Retention and deletion behaviour must be disclosed before transmission.

### 12.4 Static build and publishing adapters

The application produces a self-contained static website bundle. Publishing adapters may support GitHub Pages, Cloudflare Pages, customer SFTP/server upload, or plain export. Provider tokens belong to the customer and use the same local secret-handling rules as AI credentials.

No Koya-operated hosting account is required for the desktop-only MVP. A successful export is distinct from a successful deployment, and a successful deployment remains distinct from public verification.

### 12.5 Published tour runtime

Responsibilities:

- progressive image and panorama loading;
- WebGL panorama rendering with image fallback;
- unit and room navigation;
- video playback and scroll interaction;
- responsive controls;
- analytics events;
- visible disclosures;
- stable query-parameter or path-based deep links.

## 13. Local ownership and review roles

MVP roles:

| Role | Capabilities |
| --- | --- |
| Licence holder | Desktop licence, company profiles, local projects, publish and export |
| Editor | Import, annotate, generate drafts, assemble tours |
| Reviewer | Accept or reject spatial and visual assets, approve publish readiness |
| Publisher | Publish an approved release and manage domains |
| Viewer | Read-only review and comments |
| Consultant operator | Work inside a separately selected client workspace and prepare a handover bundle |

For the desktop-only MVP these are recorded workflow responsibilities rather than centrally managed cloud identities. One person may hold every role, but the project must still preserve who recorded each approval. Separate local company workspaces must never share relative asset paths, branding, exports, or secrets.

## 14. Safety, evidence, and compliance requirements

1. Missing facts are shown as `unavailable` or `not supplied`, never inferred as zero or invented.
2. Official imagery used only as style reference must be labelled as such.
3. Every AI-generated asset carries a durable evidence class and generation record.
4. Generated imagery cannot silently change a confirmed opening, room, fixed fixture, window, or circulation path.
5. Exterior outlooks and landmarks require explicit evidence or an illustrative disclosure.
6. The application must retain terms acceptance and source-usage permission records.
7. A rejected asset remains available for audit but cannot become publishable without a new review event.
8. Paid generation and public publication require explicit user actions.
9. No price is estimated when the active provider price cannot be retrieved; show `unavailable` and require acknowledgement.
10. All generated concept tours display the customer-approved disclaimer in the published experience.

## 15. Non-functional requirements

### Performance

- A project containing 20 unit types and 300 source assets remains navigable without loading all media into memory.
- Desktop preview becomes interactive using the fallback or mobile texture before the 8K texture finishes loading.
- Mobile devices receive a constrained texture tier rather than risking WebGL failure.
- Video assets use seekable web encodes and intentional posters.

### Reliability

- All long-running jobs are resumable and idempotent.
- Application restart does not duplicate a paid generation request.
- Publication creates a new release rather than mutating a previously verified release in place.
- Local project metadata is recoverable from a portable export.

### Security

- No vendor API credential is distributed with the application.
- Customer secrets remain outside the project bundle and renderer process.
- Codex authentication is delegated to the supported local Codex authentication flow.
- Managed OpenAI mode uses short-lived subscription tokens; the provider key remains only in the server-side gateway.
- Local exports contain a secret scan and fail if credentials or access tokens are detected.
- Audit records cover paid jobs, approvals, publishing, domain changes, exports, and rollbacks.

### Accessibility

- Keyboard navigation for primary controls.
- Reduced-motion-safe behaviour.
- Text alternatives for floor plans and still fallbacks.
- Sufficient contrast and scalable control sizes.

### Usability

- The first project uses a guided checklist with one primary action per stage.
- Unknown property information is described in plain language and linked to the exact drawing or source that is missing.
- Errors provide a customer action such as `Upload a clearer floor plan`, `Confirm the entry door`, or `Add panorama credits`, not a raw provider error.
- Standard mode does not show API keys, token counts, model slugs, stack traces, shell output, JSON, panorama projection terminology, or deployment commands.
- Destructive actions and paid generation use clear confirmation summaries with output count and customer credits.

## 16. Analytics and success metrics

### Product funnel

- project created;
- first unit added;
- floor plan locked;
- asset matrix completed;
- first generation approved;
- first room accepted;
- local preview passed;
- first tour published;
- public deployment verified.

### Primary success metrics

- percentage of created projects reaching verified publication;
- median active operator time from complete intake to publish-ready tour;
- first-pass acceptance rate for generated room identity anchors;
- percentage of published stops with complete provenance;
- deployment verification pass rate;
- mobile tour completion and room-engagement rate;
- number of published unit types per licensed installation or customer company profile.

### Quality guardrail metrics

- spatial QA rejection rate;
- panorama seam or resolution rejection rate;
- paid duplicate-job count;
- rejected-asset reuse incidents;
- published broken-asset incidents;
- disclosure omission incidents.

## 17. Desktop commercial operating model

The business combines desktop software revenue with optional professional services. It does not depend on operating a multi-tenant authoring SaaS.

### 17.1 Revenue components

| Component | Charging basis | Purpose |
| --- | --- | --- |
| Desktop licence | Perpetual major-version licence or annual licence by edition and permitted seats | Access to local authoring, review, panorama production, and export |
| Upgrade and support plan | Annual maintenance or paid major upgrade | Application updates, compatibility work, support, and new templates |
| Codex AI usage | Covered by the customer's eligible Codex/ChatGPT access or API-key access | Lets existing Codex customers use their own entitlement |
| Managed OpenAI subscription | Monthly or annual with included generation allowance and explicit top-ups or overage | Creates recurring revenue and covers provider cost, gateway operation, support, and margin |
| Project setup | Fixed fee by development or scoped unit package | Intake, workspace setup, source register, and template configuration |
| Drawing and spatial audit | Fixed or quoted professional-service fee | Converts supplied drawings into a defensible room graph and evidence base |
| Managed visual production | Fixed room/unit package plus controlled revisions | Identity images, panoramas, processing, review, and corrections |
| Custom experience | Quoted scope | Bespoke brand, interactions, integrations, or campaign pages |
| Deployment assistance | Setup plus optional support | Static hosting configuration, domain, analytics, release verification, and maintenance |
| Training and handover | Workshop or onboarding package | Moves a consulting customer into self-service operation |

Provider cost remains distinct internally from software and professional-service value. Subscription pricing must be based on measured provider cost, failed-task policy, support load, payment fees, abuse reserve, and target gross margin. Customer-facing credits must map to a documented output or task class rather than an unexplained token balance.

### 17.2 Sales motions

#### Product-led self-service

Customer buys and activates a signed desktop licence, creates a local workspace, selects Codex or a managed OpenAI subscription, imports evidence, and publishes through a standard template.

Best fit: agents, boutique agencies, property photographers, and repeat studio users.

#### Assisted project

Customer buys a defined package. The platform operator prepares the project and generation work; the customer remains responsible for factual, spatial, and final approval.

Best fit: developers and project marketers who want speed but retain internal approvals.

#### Fully managed consulting

Customer supplies the drawing and marketing package. The operator conducts discovery, audits the sources, produces the visual experience, coordinates approvals, and delivers a verified release.

Best fit: first-time customers, premium off-the-plan campaigns, incomplete source packages, and bespoke launch experiences.

### 17.3 Commercial guardrails

- Consulting scope must state included units, rooms, outputs, revisions, languages, access mode, hosting term, and handover rights.
- Paid model jobs are not described as unlimited revisions.
- Spatial or factual changes requested after approval create a new version and may create a change request.
- Customer ownership, platform licence, source usage rights, hosting period, data retention, and export rights must be stated in the contract.
- A fixed-fee project must still require explicit approval for any provider task whose price or count exceeds the contracted allowance.
- Product telemetry is opt-in, contains no customer imagery, prompts, floor plans, credentials, or project paths, and is not required for authoring.

## 18. Desktop licence packaging hypothesis

### Solo / Agent licence

- one operator installation;
- local developments and unit types;
- supplied imagery and floor-plan tour building;
- standard templates;
- static website export and QR code;
- Codex connection or optional managed OpenAI subscription.

### Studio licence

- multiple local company profiles, developments, and unit types;
- AI gap completion and panorama production;
- white label and custom domains;
- portable project handover;
- release history and deployment adapters.

### Agency / Enterprise desktop licence

- agreed seats and company-profile use rights;
- advanced asset and provider controls;
- approval policies and audit exports;
- bulk unit creation;
- service-level support, deployment assistance, and custom integrations.

### Managed Launch

- discovery and source audit;
- agreed number of unit types and room stops;
- AI image and panorama production allowance;
- customer review rounds;
- branded public experience;
- deployment verification and handover;
- optional ongoing hosting and campaign support.

Pricing amounts are intentionally outside this PRD and require provider-cost and customer-discovery evidence.

## 19. MVP release scope

### Included

- macOS and Windows desktop shells;
- signed desktop licence activation with an offline-verifiable entitlement file;
- local company profiles and isolated project folders;
- multi-project library and new-project wizard;
- read-only Koya example project or rights-safe anonymised equivalent;
- Codex local connection and authentication status;
- managed OpenAI subscription sign-in, allowance display, and server-side AI gateway;
- explicit AI provider selection and capability checks;
- standard property workflow with no developer setup;
- advanced technical audit mode for consultants and support;
- development and multi-unit management;
- file intake and evidence register;
- manual plus assisted floor-plan hotspots and room graph;
- asset matrix;
- image-model generation approval workflow;
- versioned room review;
- deterministic panorama processing and 4K/8K derivatives;
- Koya-quality responsive tour template;
- unit and room deep links;
- desktop/mobile preview;
- static website export and at least one customer-owned publishing adapter;
- deployment verification, public URL, and QR code.

### Deferred

- advanced drawing-set parsing;
- multi-user live collaboration;
- vendor-operated cloud authoring projects, long-term asset storage, or website hosting;
- customer-supplied OpenAI API keys in the MVP;
- custom template marketplace;
- CRM and portal integrations;
- customer-managed model providers;
- analytics dashboards beyond essential events;
- generated walkthrough video;
- BIM/IFC or true 3D geometry workflows;
- native iOS and Android authoring apps.

## 20. Delivery phases

### Phase 0: product foundation

- extract current Koya tour content into a strict manifest;
- separate reusable UI from Koya branding and assets;
- define schema, state machines, evidence classes, and project export format;
- create a licensed sample project.

Exit condition: the existing Apartment 106 experience can be recreated from manifest and assets without hard-coded unit content.

### Phase 1: local desktop authoring

- desktop shell;
- project and unit management;
- source import and evidence register;
- floor-plan annotation and room graph;
- asset matrix;
- local build and device preview.

Exit condition: a new project using supplied accepted assets can be authored locally without editing code.

### Phase 2: AI production and review

- local Codex SDK or app-server adapter;
- minimal subscription and server-side OpenAI gateway;
- generation package and cost approval;
- image-model calls;
- provider capability registry for aspect ratio, native pixel budget, single-output mode, price, and panorama test status;
- paid one-shot 2:1 panorama path with wrap-seam and projection QA;
- version comparison and rejection history;
- panorama pipeline;
- background job recovery.

Exit condition: a Tier B project can fill missing room coverage while retaining approved topology and provenance.

### Phase 3: publication product

- static builds, customer-owned publishing adapters, domains, releases, and rollback;
- deployed asset and navigation verification;
- QR code and share links;
- basic analytics.

Exit condition: a non-technical user can publish and verify a new multi-unit project from the desktop application.

### Phase 4: commercial scale

- additional desktop licence editions and seats;
- consulting engagements, service packages, local usage ledger, and portable customer handover;
- bulk unit workflows;
- advanced analytics and integrations;
- optional video and 3D modules.

## 21. MVP acceptance criteria

The MVP is accepted when a test operator who has not edited the source code can:

1. install the application on a supported macOS or Windows machine;
2. create a development and two unit types;
3. import a floor plan, official render, supplied panorama, and video;
4. classify evidence and lock a room graph;
5. identify one missing room view;
6. review and approve an exact image-generation package;
7. accept one output and reject another without the rejected output being reused;
8. create valid 4K and 8K panorama derivatives;
9. preview the experience at a 15-inch desktop viewport and a mobile viewport;
10. switch units and rooms through shareable URLs;
11. publish an unlisted or public release;
12. open the resulting URL outside the authoring session;
13. rotate a panorama, jump through the floor plan, and play a representative video;
14. see the correct disclosure and evidence-sensitive copy;
15. receive a verified release record, share link, and QR code.
16. create two local company profiles with different branding and confirm that their project files and exports remain separated;
17. connect through Codex, report its authenticated capabilities, and complete one approved test without exposing Codex credentials;
18. switch an approved test job to the managed OpenAI subscription without changing the project evidence or silently reusing the prior provider;
19. export a portable project bundle, open it on another licensed test installation, and reproduce the accepted tour without a vendor cloud account.
20. reject an expired or exhausted subscription before provider submission and complete an authorised top-up without duplicating the original task;
21. reconcile the desktop credit event, gateway request fingerprint, provider usage, and internal cost record for one completed generation.
22. complete the standard managed-OpenAI journey without seeing or entering an API key, model name, token count, prompt, JSON, terminal command, or deployment command;
23. understand every required action using property-language labels and recover from one failed input using the displayed guidance without technical support;
24. open Advanced mode and retrieve the exact technical generation and QA record without changing the accepted customer-facing result.
25. open Koya as an example project, return to the project library, and create an unrelated second development with a different company and branding;
26. generate, build, and export from the second project without adding, changing, or resolving any file inside the Koya project root;
27. duplicate only an approved project structure and confirm that no Koya customer asset, fact, disclosure, room graph, or rejected generation is present in the new project;
28. import an exported project bundle into another licensed installation and retain its project ID, unit structure, evidence register, QA, and accepted releases.

No acceptance item may be replaced by a build log or provider job-success status without the corresponding visual or public read-back.

## 22. Known risks and mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| AI produces attractive but spatially incorrect rooms | Misleading sales content | Lock room graph, role-labelled references, structured spatial review, explicit acceptance |
| Small source imagery appears blurry in panorama | Poor desktop and mobile experience | Asset-size audit, overlap-aware production, deterministic 4K/8K derivatives, runtime dimension QA |
| Independently generated views fail to stitch | Broken panorama continuity | Identity anchor first, controlled overlaps or cubefaces, seam and orientation QA |
| User assumes generated imagery is official | Compliance and trust risk | Durable evidence labels, visible disclosure, blocked claim language |
| Paid jobs are duplicated after errors | Unexpected customer cost | Idempotent job IDs, persistent task state, explicit retry approval |
| Desktop client exposes provider credentials | Security incident | Never ship vendor keys; use Codex-managed auth or short-lived subscription tokens with server-side OpenAI authentication |
| Large media causes mobile failures | Lost audience | Progressive texture tiers, fallbacks, memory checks, web video encoding |
| A local build is mistaken for publication | Customer cannot access tour | Separate build, deploy, and public-verified states |
| Koya-specific assumptions leak into other projects | Incorrect reusable product | Manifest-driven content, licensed sample data, no hard-coded unit topology |
| Consulting work becomes unrepeatable custom development | Low margin and delivery bottleneck | Standard packages, shared pipeline, change requests, reusable manifests and themes |
| Local company workspaces are accidentally mixed | Confidentiality breach | Explicit workspace roots, path validation, isolated exports, and automated cross-workspace tests |
| AI subscription cost exceeds revenue | Margin loss | Measured provider cost, task credits, allowance caps, rate limits, failed-task policy, top-ups, and cost reconciliation |
| Codex account lacks the required image capability | Core workflow unavailable | Capability detection, visible `unavailable` state, and explicit OpenAI API alternative |
| Codex app-server interfaces change | Desktop integration breaks | Pin and test a supported Codex runtime; prefer stable SDK interfaces and versioned adapter contracts |
| Product exposes AI complexity to property customers | Low activation and high support cost | Managed OpenAI default, guided property language, hidden technical controls, outcome-based credits, and novice usability testing |
| Assets or facts leak between development projects | Incorrect and potentially confidential output | Stable project IDs, strict project roots, path traversal rejection, rights-safe templates, and cross-project isolation tests |

## 23. Open product decisions

These decisions require customer discovery or implementation spikes before final commitment:

1. Whether the first paid plan charges by development, published unit, storage, AI usage, or a hybrid.
2. Whether the desktop licence is perpetual by major version, annual, or offers both choices.
3. Which customer-owned publishing target is required first: GitHub Pages, Cloudflare Pages, SFTP, or plain export only.
4. Whether agencies require client-specific approval portals in the first commercial release.
5. Which custom-domain and analytics capabilities are essential for initial developer customers.
6. Whether generated panoramas use one model pipeline or a configurable provider abstraction.
7. Which regions require additional statutory disclaimer templates or data residency.
8. Whether Koya is presented as a named case study or only as an anonymised reference project.
9. Whether the initial go-to-market leads with a desktop licence, managed consulting, or a consulting-to-desktop conversion path.
10. Which services remain operator-only and which become self-service controls.
11. Whether Codex integration ships through the stable SDK, local app-server, or an adapter supporting both after a production compatibility spike.
12. Whether signed offline licence files are sufficient or online activation is commercially necessary later.

## 24. Immediate next actions

1. Review this PRD with one developer-side and one agency-side user.
2. Convert the existing Apartment 106 data in `interactive-apartment-tour/app/page.tsx` into a versioned manifest.
3. Define the project export bundle and JSON schemas.
4. Produce low-fidelity screens for onboarding, floor-plan audit, asset matrix, review, and publish.
5. Build a thin desktop proof of concept that imports a manifest and launches the existing tour preview.
6. Validate the complete workflow using Koya as the sample and a second, deliberately incomplete unit as the Tier B test case.
7. Run one separately approved paid API spike for a Koya room: request the provider's largest practical single 2:1 output, preserve the exact cost and parameters, test wrap closure and topology, then compare it with the existing stitched/upscaled POC panorama.
8. Define one self-service package, one assisted package, and one managed launch statement of work using the same unit and usage model.
9. Test the commercial proposition with at least one developer/project marketer and one agency or property-media operator before fixing price points.
10. Build a desktop integration spike that detects local Codex, completes its supported authentication flow, streams one read-only project audit, and records no credential in the project.
11. Build a minimal subscription-gateway spike that validates one test entitlement, submits one separately approved OpenAI image task with a server-side key, records exact provider cost, and returns the result without storing the complete customer project.
12. Run a moderated usability test in which a property professional creates a development, uploads one unit, approves one image task, and previews a panorama without instructions about AI, APIs, prompts, or deployment.
13. Extract Koya into the first versioned example-project bundle, complete a rights and rejection audit, and verify that creating a blank project writes only to its own new project root.
