# Real Estate AI Studio — Product Vision and Platform PRD

| Field | Value |
| --- | --- |
| Version | V1.0 |
| Status | Product direction draft |
| Date | 2026-08-31 |
| Product form | Local-first desktop application with managed AI subscription and optional deployment gateway |
| First reference project | Koya |
| First production module | Off-plan Interactive Tour Builder |
| Primary customers | Property developers, project marketers, real-estate agencies, property creative studios |

## 1. Product definition

Real Estate AI Studio is a project-centred desktop application that helps property companies manage approved project information and turn it into repeatable sales and marketing outputs.

Each development is an independent project. The project becomes the source of truth for:

- customer and development identity;
- brand assets and visual rules;
- official project facts;
- floor plans and unit types;
- renders, photos, videos, and material references;
- approved and rejected generated assets;
- disclosures and claim restrictions;
- generation history, allowance, and review decisions;
- websites, campaigns, builds, deployments, and releases.

Production modules consume the same approved project information. A user does not re-upload or re-explain the development every time they create a panorama, poster, social image, or sales page.

## 2. Core product idea

```text
Project evidence and brand
        |
        v
Approved project workspace
        |
        +-- Interactive Tour and Panorama
        +-- Poster and Campaign Creative
        +-- Listing and Social Asset Packs
        +-- Sales Page and Mini-site
        +-- Static Build and Cloud Deployment
        +-- Future property AI modules
```

The product is not a generic AI chat window. It is a controlled property-production system whose outputs inherit the selected project's facts, evidence, branding, unit identity, and approval state.

## 3. Reference project model

Koya is one project in the local project library and serves as the first reference implementation.

```text
Local project library
  -> Koya
    -> Company and project profile
    -> Brand and disclosures
    -> Apartments 102, 103, 104, and 106
    -> Apartment 106 interactive tour
    -> Generated room imagery
    -> Accepted panoramas
    -> Videos and posters
    -> Builds and verified releases
  -> New development A
  -> New development B
```

Koya is not a global asset library. New developments receive a new project ID, project root, evidence register, brand profile, units, outputs, and release history.

The Koya example may be included in the product only when demonstration and distribution rights are confirmed. Otherwise, the product ships an anonymised Koya-derived example using synthetic or separately licensed assets.

## 4. Target user experience

The default user is experienced in property but unfamiliar with AI and software development.

The primary workflow is:

`Create project -> Upload property materials -> Confirm facts and units -> Choose an output -> Review allowance -> Generate -> Review -> Export or publish`

The standard customer is never required to:

- create or paste an API key;
- choose a model slug;
- understand tokens;
- write a technical prompt;
- run a terminal command;
- install Python, Node.js, FFmpeg, or panorama tools;
- understand image projection or stitching;
- configure a website build;
- manually upload individual build files.

Customer-facing controls use property language such as:

- `Create development`;
- `Add unit type`;
- `Upload floor plan`;
- `Confirm entry and rooms`;
- `Generate living room image`;
- `Generate 360° panorama`;
- `Create launch poster`;
- `Create social image pack`;
- `Build sales page`;
- `Publish`.

## 5. Platform modules

### 5.1 Project Manager

Purpose: manage separate developments and their approved source material.

MVP capabilities:

- local project library;
- customer company profiles;
- new-project wizard;
- project status and recent activity;
- source import and classification;
- evidence register;
- brand kit;
- unit and floor-plan management;
- accepted and rejected asset library;
- portable project export and import;
- project backup and integrity checks.

### 5.2 Off-plan Interactive Tour Builder

Purpose: create a Google-Street-View-like property experience from floor plans, approved imagery, generated room assets, panoramas, and optional videos.

Capabilities:

- floor-plan room graph and hotspots;
- unit switching;
- room identity images;
- multiple directional image generation;
- one-shot or stitched panorama production;
- deterministic 4K and 8K delivery derivatives;
- draggable and touch-enabled panorama viewer;
- optional scroll video;
- mobile and desktop QA;
- static website build;
- public-link deployment and verification.

The detailed module requirements remain in `OFFPLAN_TOUR_BUILDER_PRD_V1.md`.

### 5.3 Property Creative Studio

Purpose: generate project-consistent sales and marketing assets without rebuilding the brief for every output.

The task-based workspace, preview/chat layout, and per-task AI session model are specified in `CREATIVE_STUDIO_TASK_SESSION_PRD_V1.md`.

Initial output types:

- development launch poster;
- unit-type feature poster;
- open-home or appointment creative;
- landscape and portrait social images;
- story and reel covers;
- digital display banners;
- email hero images;
- brochure or presentation cover images;
- agent co-branded assets;
- image variations with approved copy and calls to action.

Generation rules:

- use only approved project facts and claims;
- apply the selected project brand kit;
- keep official and generated imagery distinguishable;
- preserve required legal text and disclaimers;
- generate platform-safe sizes automatically;
- route all outputs through review before export or publication;
- never infer prices, dates, availability, incentives, views, or amenities.

### 5.4 Campaign Asset Packs

Purpose: turn one approved campaign brief into a coordinated set of assets.

Example pack:

```text
Launch campaign
  -> Website hero
  -> Instagram portrait
  -> Instagram story
  -> Facebook landscape
  -> LinkedIn landscape
  -> Email banner
  -> Display ad variants
  -> QR-code poster
```

All variants share one campaign ID, approved copy, source imagery, brand version, and disclosure state.

### 5.5 Sales Page Builder

Purpose: create simple, branded, static project or unit pages from approved project content.

Capabilities:

- template selection;
- development overview;
- unit and floor-plan cards;
- image galleries;
- interactive tour embedding;
- calls to action;
- approved contact information;
- responsive preview;
- SEO and indexing controls;
- static build and export.

### 5.6 Deployment Centre

Purpose: build, publish, verify, and manage releases without exposing hosting complexity to the property customer.

Initial publishing targets may include:

- GitHub Pages;
- Cloudflare Pages;
- customer SFTP or server;
- plain static export;
- later managed hosting.

Deployment workflow:

`Build -> Local preview -> Customer approval -> Publish -> Asset checks -> Public click-through -> Verified release`

A successful build is not a successful deployment. A successful deployment is not a verified public release until the intended page, assets, navigation, and access mode are checked.

## 6. Project source of truth

Every project contains a structured manifest rather than relying on filenames or chat history.

Required information groups:

- project identity;
- customer company;
- brand version;
- official facts;
- unavailable facts;
- claim restrictions;
- disclosures;
- units and floor plans;
- source assets and rights;
- accepted generated assets;
- rejected assets;
- campaign briefs;
- output records;
- builds and releases.

Evidence classes:

- `official`;
- `surveyed`;
- `approved_render`;
- `concept_floorplan_grounded`;
- `concept_style_only`;
- `unknown`;
- `rejected`.

An AI output does not become official because it was accepted for marketing use.

## 7. Local project structure

```text
<project-root>/
  project.json
  sources/
    source-register.json
    drawings/
    renders/
    photos/
    videos/
    brand/
    copy/
  units/
    <unit-id>/
      unit.json
      floorplan/
      room-graph.json
      assets/
      tour-manifest.json
      qa/
  creative/
    campaigns/
    posters/
    social/
    banners/
  sites/
    tour/
    sales-pages/
  builds/
  releases/
  reports/
  cache/
```

Project-relative paths cannot escape the project root. New projects cannot resolve Koya assets, facts, branding, or rejected content unless the user explicitly imports an approved reusable template component.

## 8. AI connection model

### 8.1 Managed OpenAI subscription

Default for ordinary property users.

- no API-key setup;
- customer signs in to the product subscription;
- customer sees included and remaining output credits;
- the desktop sends only approved generation inputs to the AI gateway;
- the gateway holds the provider credential server-side;
- outputs return to and are stored in the local project;
- customer-facing credits use property outputs, not tokens;
- duplicate requests are prevented through request fingerprints and idempotency keys.

### 8.2 Codex Studio mode

Advanced option for the product operator, consultant, creative studio, or technically capable customer.

- integrates local Codex through supported interfaces;
- uses the customer's eligible Codex authentication;
- can inspect the active project and run approved project tools;
- can prepare manifests, audits, generation packages, builds, and QA;
- never bypasses paid-generation or publication approval;
- reports missing capability as `unavailable` rather than silently changing provider.

## 9. Standard and advanced experiences

### Standard mode

Shows:

- project tasks;
- property outputs;
- allowance and credit cost;
- source gaps;
- visual comparisons;
- plain-language warnings;
- approve, revise, export, and publish actions.

Hides:

- API credentials;
- model slugs;
- tokens;
- raw prompts;
- JSON;
- task fingerprints;
- shell commands;
- build logs;
- deployment commands;
- panorama projection settings.

### Advanced mode

Shows technical records needed by consultants and support:

- evidence roles;
- exact prompts and parameters;
- model and provider;
- input and output dimensions;
- task IDs and fingerprints;
- provider usage and internal cost;
- panorama seams and decoded textures;
- build and deployment diagnostics.

Advanced mode does not remove approvals or evidence controls.

## 10. Automation rules

The system may automate:

- source classification;
- project fact extraction;
- asset-gap analysis;
- unit and room suggestions;
- output-size selection;
- prompt preparation;
- image generation;
- deterministic resizing and panorama processing;
- campaign variant production;
- static website assembly;
- deployment and technical verification.

The user must confirm:

- official project identity and facts;
- floor-plan orientation, openings, and room relationships;
- asset rights;
- paid output count and credits;
- generated visual acceptance;
- marketing copy and claims;
- disclosure wording;
- final publication.

Automation cannot turn missing information into a factual claim.

## 11. Commercial model

### Desktop software

- Solo / Agent licence;
- Studio licence;
- Agency / Enterprise desktop licence;
- paid major upgrades;
- annual support and template updates.

### Managed OpenAI subscription

- monthly or annual plans;
- included image and panorama credits;
- explicit top-ups or overage policy;
- credits mapped to named property outputs;
- provider cost, payment fees, abuse reserve, support, and target margin included in pricing.

### Professional services

- project setup;
- drawing and evidence audit;
- managed image and panorama production;
- campaign asset production;
- custom templates and pages;
- deployment and domain setup;
- training and project handover;
- ongoing campaign support.

## 12. Product boundary

The initial product is not:

- a general-purpose AI chatbot;
- a property CRM;
- a listing portal;
- a full social scheduling platform;
- a BIM or CAD editor;
- a measured 3D scanning product;
- an unattended factual-content publisher;
- a replacement for legal, design, or marketing approval.

These boundaries keep the first release deliverable while leaving room for integrations later.

## 13. Delivery sequence

### Phase 1: reusable project foundation

- extract Koya into a validated example-project bundle;
- define project schema, folder contract, evidence model, and portable export;
- separate reusable UI and processing tools from Koya content;
- build local project library and new-project wizard.

### Phase 2: interactive tour product

- floor-plan and unit workflow;
- asset matrix;
- managed image generation;
- panorama production;
- Koya-quality tour template;
- local preview and static export.

### Phase 3: subscription and deployment

- customer subscription and allowance;
- server-side AI gateway;
- request idempotency and cost reconciliation;
- publishing adapters;
- deployment verification and release history.

### Phase 4: property creative studio

- project brand kit;
- poster templates;
- social and banner sizes;
- campaign briefs and asset packs;
- copy and disclosure review;
- bulk export.

### Phase 5: broader automation

- sales page templates;
- campaign packs;
- approved deployment automation;
- analytics and reusable customer workflows;
- optional integrations after customer validation.

## 14. Platform acceptance criteria

The platform direction is proven when a non-technical property professional can:

1. open Koya as a read-only example project;
2. create a separate new development without copying Koya content;
3. upload a floor plan, renders, brand files, and approved copy;
4. confirm facts and unit structure;
5. generate and approve one room image;
6. generate and approve one panorama;
7. build an interactive unit tour;
8. generate a coordinated launch poster and social variant from the same project;
9. preview every output without technical tooling;
10. approve one static release;
11. publish through a customer-selected target;
12. open the result publicly and complete a verified click-through;
13. retain all accepted, rejected, generated, and release records inside the correct project;
14. complete the standard workflow without encountering an API key, model slug, token count, raw prompt, JSON, terminal command, or deployment command.

## 15. Immediate decisions

1. Confirm the product name or working codename.
2. Confirm whether Koya can be distributed as a named example or must be anonymised.
3. Define the first three subscription output classes: standard image, high-resolution panorama, and campaign asset pack.
4. Select the first publishing adapter.
5. Freeze the first release to Project Manager plus Interactive Tour Builder before implementing the broader Creative Studio.
6. Use the first paying projects to validate Poster and Campaign modules before expanding template volume.
