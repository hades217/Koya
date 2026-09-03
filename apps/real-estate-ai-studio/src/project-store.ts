import { invoke } from '@tauri-apps/api/core';
import type { AddUnitInput, AssetRejectionReason, BindGatewaySessionInput, CapabilityRegistry, CodexAdapterInspection, CompanyProfile, CreativeBrief, CreateProjectInput, DeploymentReadbackResult, DesktopSettings, DuplicateProjectInput, GatewaySessionStatus, GenerateCreativeBriefInput, ImportAssetsInput, LicenseStatus, ManagedGatewayCapabilityResult, MarketplaceSkill, OpeningInput, ProjectManifest, ProjectRecord, RoomGraphNodeInput, RoomSupportingRole, SignedUpdateStatus, UpdateDesktopSettingsInput, UpdateProjectInput, VerifiedReleaseResult } from './types';
import { CURRENT_PROJECT_SCHEMA_VERSION, normalizeBrowserProject } from './browser-projects';
import koyaExampleManifest from '../resources/examples/koya-example-v1/project.json';

const browserStorageKey = 'estate-studio-projects-v1';

export const koyaExample: ProjectRecord = {
  projectRoot: 'Example project · local read-only bundle',
  manifest: koyaExampleManifest as ProjectManifest,
};

function isTauri() {
  return Boolean(window.__TAURI_INTERNALS__);
}

export async function getLicenseStatus(): Promise<LicenseStatus> {
  if (!isTauri()) return { installed: false, valid: false, detail: 'Signed licences are verified by the desktop application.', entitlements: [], allowedRoles: [] };
  return invoke<LicenseStatus>('get_license_status');
}

export async function installSignedLicense(inputPath: string): Promise<LicenseStatus> {
  if (!isTauri()) throw new Error('Signed licence installation is available in the desktop app.');
  return invoke<LicenseStatus>('install_signed_license', { inputPath });
}

export async function setActiveLocalRole(role: string, displayName: string): Promise<LicenseStatus> {
  if (!isTauri()) throw new Error('Local role assignment is available in the desktop app.');
  return invoke<LicenseStatus>('set_active_local_role', { role, displayName });
}

export async function getDesktopSettings(): Promise<DesktopSettings> {
  if (!isTauri()) return { schemaVersion: 1, storageMode: 'browser_preview', storagePath: 'Browser local storage', updateChannel: 'stable', autoCheckUpdates: false, codexAuthMode: 'unavailable', publishingTargets: [], secretReferences: [], updatedAt: Date.now() };
  return invoke<DesktopSettings>('get_desktop_settings');
}

export async function updateDesktopSettings(input: UpdateDesktopSettingsInput): Promise<DesktopSettings> {
  if (!isTauri()) throw new Error('Desktop settings are available in the desktop app.');
  return invoke<DesktopSettings>('update_desktop_settings', { input });
}

export async function checkSignedUpdate(): Promise<SignedUpdateStatus> {
  if (!isTauri()) return { configured: false, available: false, channel: 'stable', currentVersion: 'browser', recoveryPending: false, detail: 'Signed updates are available in installed desktop builds.' };
  return invoke<SignedUpdateStatus>('check_signed_update');
}

export async function installSignedUpdate(expectedVersion: string): Promise<SignedUpdateStatus> {
  if (!isTauri()) throw new Error('Signed update installation is available in the desktop app.');
  return invoke<SignedUpdateStatus>('install_signed_update', { expectedVersion });
}

export async function confirmUpdateHealth(): Promise<SignedUpdateStatus> {
  if (!isTauri()) throw new Error('Update recovery checks are available in the desktop app.');
  return invoke<SignedUpdateStatus>('confirm_update_health');
}

function loadBrowserProjects(): ProjectRecord[] {
  const stored = window.localStorage.getItem(browserStorageKey);
  if (!stored) return [koyaExample];
  try {
    const projects = JSON.parse(stored) as ProjectRecord[];
    return [koyaExample, ...projects.filter((project) => project.manifest.projectId !== 'koya-example').map(normalizeBrowserProject)];
  } catch {
    return [koyaExample];
  }
}

function saveBrowserProjects(projects: ProjectRecord[]) {
  window.localStorage.setItem(
    browserStorageKey,
    JSON.stringify(projects.filter((project) => project.manifest.projectId !== 'koya-example').map(normalizeBrowserProject)),
  );
}

export async function listProjects(): Promise<ProjectRecord[]> {
  if (isTauri()) {
    await invoke<number>('recover_generation_jobs');
    await invoke<number>('recover_background_jobs');
    return invoke<ProjectRecord[]>('list_projects');
  }
  return loadBrowserProjects().map(normalizeBrowserProject);
}

export async function listCompanyProfiles(): Promise<CompanyProfile[]> {
  if (isTauri()) return invoke<CompanyProfile[]>('list_company_profiles');
  const profiles = new Map<string, CompanyProfile>();
  for (const project of loadBrowserProjects().filter((item) => !item.manifest.readOnly)) {
    const snapshot = project.manifest.companyProfile;
    profiles.set(snapshot.id, { id: snapshot.id, name: snapshot.name, locale: project.manifest.locale, measurementUnits: project.manifest.measurementUnits, brand: project.manifest.brand, createdAt: project.manifest.createdAt, updatedAt: project.manifest.updatedAt });
  }
  return [...profiles.values()].sort((left, right) => left.name.localeCompare(right.name));
}

export async function createProject(input: CreateProjectInput): Promise<ProjectRecord> {
  if (isTauri()) return invoke<ProjectRecord>('create_project', { input });

  const slug = input.name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '') || 'development';
  const createdAt = Math.floor(Date.now() / 1000);
  const manifest: ProjectManifest = {
    schemaVersion: CURRENT_PROJECT_SCHEMA_VERSION,
    projectId: `${slug}-${createdAt}`,
    name: input.name.trim(),
    company: input.company.trim(),
    companyProfile: { id: input.company.trim().toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'company', name: input.company.trim() },
    location: input.location.trim(),
    locale: 'en-AU',
    measurementUnits: 'metric',
    accessMode: 'unlisted',
    disclosure: 'Disclosure not supplied',
    workflowMode: 'standard',
    status: 'New project · Intake required',
    readiness: 12,
    readOnly: false,
    archivedAt: undefined,
    createdAt,
    updatedAt: createdAt,
    units: input.unitIds.map((id) => ({
      id,
      label: `Unit ${id}`,
      summary: 'Details not supplied',
      status: 'Awaiting floor plan',
      tourAvailable: false,
      rooms: [],
      roomGraphLocked: false,
    })),
    assets: [],
    generationJobs: [],
    creativeJobs: [],
    enabledSkillIds: ['property-project-intake', 'offplan-evidence-audit', 'floorplan-concept-planning', 'panorama-production'],
    releases: [],
    approvalEvents: [],
    qaRecords: [],
    usageLedger: [],
    backgroundJobs: [],
    analyticsEvents: [],
    designSpec: {
      status: 'not_started',
      direction: 'Project-specific design direction has not been approved.',
      sourceBasis: [],
      colours: { paper: '#f5f1e7', ink: '#20241f', accent: '#78917b', muted: '#796d61', botanical: '#66746b' },
      typography: { display: 'Didot, "Bodoni 72", Georgia, serif', body: '"Avenir Next", "Helvetica Neue", Arial, sans-serif' },
      layout: { page: 'A4 portrait', marginMm: 18, grid: 'Editorial asymmetric grid with generous negative space', imageTreatment: 'Approved project imagery only; natural colour; no decorative tint' },
    },
    brand: { primary: '#20241f', accent: '#78917b' },
    modules: { tour: 'not_started', creative: 'not_started', deployment: 'not_started' },
  };
  const project = { manifest, projectRoot: `Browser prototype/${manifest.projectId}` };
  const projects = loadBrowserProjects();
  saveBrowserProjects([...projects, project]);
  return project;
}

export async function duplicateProjectStructure(projectId: string, input: DuplicateProjectInput): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Structure-only duplication is available in the desktop app.');
  return invoke<ProjectRecord>('duplicate_project_structure', { projectId, input });
}

export async function setWorkflowMode(projectId: string, mode: 'standard' | 'advanced'): Promise<ProjectRecord> {
  if (isTauri()) return invoke<ProjectRecord>('set_workflow_mode', { projectId, mode });
  const projects = loadBrowserProjects();
  const project = projects.find((item) => item.manifest.projectId === projectId);
  if (!project || project.manifest.readOnly) throw new Error('Workflow mode cannot be changed for this project.');
  const updated = { ...project, manifest: { ...project.manifest, workflowMode: mode, updatedAt: Math.floor(Date.now() / 1000) } };
  saveBrowserProjects(projects.map((item) => item.manifest.projectId === projectId ? updated : item));
  return updated;
}

export async function openProjectFolder(projectId: string): Promise<void> {
  if (isTauri()) {
    await invoke('open_project_folder', { projectId });
  }
}

export async function getAssetDataUrl(projectId: string, assetId: string): Promise<string> {
  if (!isTauri()) throw new Error('Source previews are available in the desktop app.');
  return invoke<string>('get_asset_data_url', { projectId, assetId });
}

export async function addUnit(projectId: string, input: AddUnitInput): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Unit editing is available in the desktop app.');
  return invoke<ProjectRecord>('add_unit', { projectId, input });
}

export async function importProjectAssets(projectId: string, input: ImportAssetsInput): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Source import is available in the desktop app.');
  return invoke<ProjectRecord>('import_project_assets', { projectId, input });
}

export async function exportProjectBundle(projectId: string, outputPath: string): Promise<string> {
  if (!isTauri()) throw new Error('Portable project export is available in the desktop app.');
  return invoke<string>('export_project_bundle', { projectId, outputPath });
}

export async function importProjectBundle(inputPath: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Portable project import is available in the desktop app.');
  return invoke<ProjectRecord>('import_project_bundle', { inputPath });
}

export async function updateProject(projectId: string, input: UpdateProjectInput): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Project editing is available in the desktop app.');
  return invoke<ProjectRecord>('update_project', { projectId, input });
}

export async function setProjectArchived(projectId: string, archived: boolean): Promise<ProjectRecord> {
  if (isTauri()) return invoke<ProjectRecord>('set_project_archived', { projectId, archived });
  const projects = loadBrowserProjects();
  const project = projects.find((item) => item.manifest.projectId === projectId);
  if (!project || project.manifest.readOnly) throw new Error('This project cannot be archived.');
  const updated: ProjectRecord = { ...project, manifest: { ...project.manifest, archivedAt: archived ? Math.floor(Date.now() / 1000) : undefined, status: archived ? 'Archived · Read-only until restored' : project.manifest.assets.length ? 'Intake in progress · Sources need review' : 'New project · Intake required', updatedAt: Math.floor(Date.now() / 1000) } };
  saveBrowserProjects(projects.map((item) => item.manifest.projectId === projectId ? updated : item));
  return updated;
}

export async function addRoom(projectId: string, unitId: string, name: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Room editing is available in the desktop app.');
  return invoke<ProjectRecord>('add_room', { projectId, unitId, name });
}

export async function reviewAsset(projectId: string, assetId: string, decision: 'accepted' | 'needs_review' | 'rejected', reasonCode?: AssetRejectionReason, notes?: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Asset review is available in the desktop app.');
  return invoke<ProjectRecord>('review_asset', { projectId, assetId, decision, reasonCode, notes });
}

export async function assignRoomIdentity(projectId: string, unitId: string, roomId: string, assetId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Room evidence assignment is available in the desktop app.');
  return invoke<ProjectRecord>('assign_room_identity', { projectId, unitId, roomId, assetId });
}

export async function assignRoomPanorama(projectId: string, unitId: string, roomId: string, assetId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama assignment is available in the desktop app.');
  return invoke<ProjectRecord>('assign_room_panorama', { projectId, unitId, roomId, assetId });
}

export async function createPanoramaDraft(projectId: string, unitId: string, roomId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama job preparation is available in the desktop app.');
  return invoke<ProjectRecord>('create_panorama_draft', { projectId, unitId, roomId });
}

export async function approveGenerationJob(projectId: string, jobId: string, confirmedFingerprint: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generation approval is available in the desktop app.');
  return invoke<ProjectRecord>('approve_generation_job', { projectId, jobId, confirmedFingerprint });
}

export async function retryGenerationJob(projectId: string, jobId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generation retry is available in the desktop app.');
  return invoke<ProjectRecord>('retry_generation_job', { projectId, jobId });
}

export async function cancelGenerationJob(projectId: string, jobId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generation cancellation is available in the desktop app.');
  return invoke<ProjectRecord>('cancel_generation_job', { projectId, jobId });
}

export async function ingestGenerationOutput(projectId: string, jobId: string, input: { sourcePath: string; providerRequestId: string; requestFingerprint: string }): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generated-output ingestion is available in the desktop app.');
  return invoke<ProjectRecord>('ingest_generation_output', { projectId, jobId, input });
}

export async function reviewGenerationOutput(projectId: string, jobId: string, outputId: string, input: { decision: 'accepted' | 'rejected'; reason?: string; correctionInstruction?: string }): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generated-output review is available in the desktop app.');
  return invoke<ProjectRecord>('review_generation_output', { projectId, jobId, outputId, input });
}

export async function recordPanoramaQa(projectId: string, jobId: string, outputId: string, input: { topologyPassed: boolean; confirmedOpeningIds: string[]; horizonPassed: boolean; seamPassed: boolean; orientationPassed: boolean; yawDegrees: number; runtimePassed: boolean; usagePermissionReference: string }): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama QA is available in the desktop app.');
  return invoke<ProjectRecord>('record_panorama_qa', { projectId, jobId, outputId, input });
}

export async function buildPanoramaDerivatives(projectId: string, jobId: string, outputId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama derivative processing is available in the desktop app.');
  const queued = await invoke<ProjectRecord>('enqueue_background_job', { projectId, kind: 'panorama_processing', subjectId: `${jobId}:${outputId}` });
  const backgroundJob = queued.manifest.backgroundJobs.find((job) => job.kind === 'panorama_processing' && job.subjectId === `${jobId}:${outputId}` && job.status !== 'completed');
  if (!backgroundJob) throw new Error('Panorama background job is unavailable after enqueue.');
  return invoke<ProjectRecord>('run_panorama_background_job', { projectId, backgroundJobId: backgroundJob.id });
}

export async function buildSuppliedPanoramaDerivatives(projectId: string, assetId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Supplied panorama derivative processing is available in the desktop app.');
  const subjectId = `asset:${assetId}`;
  const queued = await invoke<ProjectRecord>('enqueue_background_job', { projectId, kind: 'panorama_processing', subjectId });
  const backgroundJob = queued.manifest.backgroundJobs.find((job) => job.kind === 'panorama_processing' && job.subjectId === subjectId && job.status !== 'completed');
  if (!backgroundJob) throw new Error('Supplied panorama background job is unavailable after enqueue.');
  return invoke<ProjectRecord>('run_panorama_background_job', { projectId, backgroundJobId: backgroundJob.id });
}

export async function enqueueBackgroundJob(projectId: string, kind: 'panorama_processing' | 'static_build' | 'upload' | 'download', subjectId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Background queues are available in the desktop app.');
  return invoke<ProjectRecord>('enqueue_background_job', { projectId, kind, subjectId });
}

export async function runPanoramaBackgroundJob(projectId: string, backgroundJobId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama background processing is available in the desktop app.');
  return invoke<ProjectRecord>('run_panorama_background_job', { projectId, backgroundJobId });
}

export async function retryBackgroundJob(projectId: string, backgroundJobId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Background retry is available in the desktop app.');
  return invoke<ProjectRecord>('retry_background_job', { projectId, backgroundJobId });
}

export async function createPanoramaFallbackPlan(projectId: string, jobId: string, outputId: string, input: { mode: 'cubefaces' | 'overlapping_tiles'; reason: string }): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Panorama fallback planning is available in the desktop app.');
  return invoke<ProjectRecord>('create_panorama_fallback_plan', { projectId, jobId, outputId, input });
}

export async function assignRoomStillFallback(projectId: string, unitId: string, roomId: string, assetId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Still fallback assignment is available in the desktop app.');
  return invoke<ProjectRecord>('assign_room_still_fallback', { projectId, unitId, roomId, assetId });
}

export async function assignRoomSupportingAsset(projectId: string, unitId: string, roomId: string, role: RoomSupportingRole, assetId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Room media assignment is available in the desktop app.');
  return invoke<ProjectRecord>('assign_room_supporting_asset', { projectId, unitId, roomId, role, assetId });
}

export async function getGenerationOutputDataUrl(projectId: string, jobId: string, outputId: string): Promise<string> {
  if (!isTauri()) throw new Error('Generated-output comparison is available in the desktop app.');
  return invoke<string>('get_generation_output_data_url', { projectId, jobId, outputId });
}

export async function listGenerationCapabilities(): Promise<CapabilityRegistry> {
  if (!isTauri()) return { schemaVersion: 1, records: [], updatedAt: 0 };
  return invoke<CapabilityRegistry>('list_generation_capabilities');
}

export async function applyGenerationCapability(projectId: string, jobId: string, providerChoice: 'codex' | 'managed_openai', modelId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Generation capability checks are available in the desktop app.');
  return invoke<ProjectRecord>('apply_generation_capability', { projectId, jobId, providerChoice, modelId });
}

export async function inspectCodexGenerationAdapter(projectId: string, jobId: string): Promise<CodexAdapterInspection> {
  if (!isTauri()) throw new Error('Codex generation adapter inspection is available in the desktop app.');
  return invoke<CodexAdapterInspection>('inspect_codex_generation_adapter', { projectId, jobId });
}

export async function bindManagedGatewaySession(input: BindGatewaySessionInput): Promise<GatewaySessionStatus> {
  if (!isTauri()) throw new Error('Managed subscriptions are available in the desktop app.');
  return invoke<GatewaySessionStatus>('bind_managed_gateway_session', { input });
}

export async function managedGatewayStatus(): Promise<GatewaySessionStatus> {
  if (!isTauri()) return { configured: false, authenticated: false, credentialStorage: 'memory_only', detail: 'Managed subscription is unavailable in the browser preview.' };
  return invoke<GatewaySessionStatus>('managed_gateway_status');
}

export async function clearManagedGatewaySession(): Promise<void> {
  if (!isTauri()) return;
  await invoke('clear_managed_gateway_session');
}

export async function checkManagedGatewayCapability(projectId: string, jobId: string): Promise<ManagedGatewayCapabilityResult> {
  if (!isTauri()) throw new Error('Managed capability checks are available in the desktop app.');
  return invoke<ManagedGatewayCapabilityResult>('check_managed_gateway_capability', { projectId, jobId });
}

export async function submitManagedPanorama(projectId: string, jobId: string, confirmedFingerprint: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Managed panorama submission is available in the desktop app.');
  return invoke<ProjectRecord>('submit_managed_panorama', { projectId, jobId, confirmedFingerprint });
}

export async function saveRoomGraph(projectId: string, unitId: string, nodes: RoomGraphNodeInput[], entranceRoomId: string | undefined, openings: OpeningInput[], locked: boolean): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Room-graph editing is available in the desktop app.');
  return invoke<ProjectRecord>('save_room_graph', { projectId, unitId, nodes, entranceRoomId, openings, locked });
}

export async function createLocalRelease(projectId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Release snapshots are available in the desktop app.');
  return invoke<ProjectRecord>('create_local_release', { projectId });
}

export async function exportStaticRelease(projectId: string, releaseId: string, destinationPath: string): Promise<string> {
  if (!isTauri()) throw new Error('Static release export is available in the desktop app.');
  return invoke<string>('export_static_release', { projectId, releaseId, destinationPath });
}

export async function publishReleaseToCustomerDirectory(projectId: string, releaseId: string, destinationPath: string, accessMode: 'public' | 'unlisted' | 'private'): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Customer-owned directory publishing is available in the desktop app.');
  return invoke<ProjectRecord>('publish_release_to_customer_directory', {
    projectId,
    input: { releaseId, destinationPath, accessMode, confirmedUnverifiedUpload: true },
  });
}

export async function verifyDeploymentReadback(projectId: string, releaseId: string, baseUrl: string, expectedAccessMode: 'public' | 'unlisted' | 'private'): Promise<DeploymentReadbackResult> {
  if (!isTauri()) throw new Error('Deployment read-back is available in the desktop app.');
  return invoke<DeploymentReadbackResult>('verify_deployment_readback', {
    projectId,
    input: { releaseId, baseUrl, expectedAccessMode },
  });
}

export async function finalizeVerifiedRelease(projectId: string, releaseId: string, baseUrl: string, expectedAccessMode: 'public' | 'unlisted' | 'private'): Promise<VerifiedReleaseResult> {
  if (!isTauri()) throw new Error('Final release verification is available in the desktop app.');
  return invoke<VerifiedReleaseResult>('finalize_verified_release', {
    projectId,
    input: { releaseId, baseUrl, expectedAccessMode },
  });
}

export async function createRollbackRelease(projectId: string, targetReleaseId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Rollback release creation is available in the desktop app.');
  return invoke<ProjectRecord>('create_rollback_release', { projectId, targetReleaseId });
}

export async function buildStaticTourPreview(projectId: string): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Static tour previews are available in the desktop app.');
  return invoke<ProjectRecord>('build_static_tour_preview', { projectId });
}

export async function generateCreativeBrief(projectId: string, input: GenerateCreativeBriefInput): Promise<CreativeBrief> {
  if (!isTauri()) throw new Error('Codex creative drafting is available in the desktop app.');
  return invoke<CreativeBrief>('generate_creative_brief', { projectId, input });
}

export async function saveCreativeBrief(projectId: string, brief: CreativeBrief, expectedUpdatedAt: number): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Creative saving is available in the desktop app.');
  return invoke<ProjectRecord>('save_creative_brief', { projectId, input: { brief, expectedUpdatedAt } });
}

export async function getCreativeA4Html(projectId: string, creativeId: string): Promise<string> {
  if (!isTauri()) {
    const project = loadBrowserProjects().find((item) => item.manifest.projectId === projectId);
    const brief = project?.manifest.creativeJobs.find((item) => item.id === creativeId);
    if (!project || !brief || brief.posterMode === 'ai_creative') throw new Error('A4 preview is unavailable.');
    return `<!doctype html><html><head><style>@page{size:A4;margin:0}*{box-sizing:border-box}html,body{margin:0;background:#e4dfd4}body{font-family:"Sweet Sans Pro","Avenir Next",Avenir,"Helvetica Neue",Arial,sans-serif;color:#1f1a16;font-weight:300}.sheet{position:relative;width:210mm;min-height:297mm;margin:auto;padding:18mm;background:#f5f1e7}header{display:flex;justify-content:space-between;padding-bottom:6mm;border-bottom:.3mm solid #796b5e;font-size:8pt;font-weight:500;letter-spacing:.18em;text-transform:uppercase}header:after{content:'A4 · CLIENT REVIEW';color:#796d61;font-size:6.5pt}main{padding-top:45mm}.campaign{margin:0;color:#725f4e;font-size:7pt;font-weight:500;letter-spacing:.16em;text-transform:uppercase}h1{max-width:165mm;margin:6mm 0 4mm;color:#38261d;font-family:"Pilot",Didot,"Bodoni 72",serif;font-size:36pt;font-style:italic;font-weight:400;line-height:1.02;letter-spacing:-.02em}h2{max-width:150mm;margin:0;color:#38261d;font-size:12pt;font-weight:300;line-height:1.45}.body{max-width:140mm;margin-top:18mm;color:#1f1a16;font-size:10pt;font-weight:300;line-height:1.7}.cta{display:inline-block;margin-top:17mm;padding:4mm 6mm;border:1px solid #38261d;color:#f5f1e7;background:#38261d;font-size:8pt;font-weight:500}</style></head><body><article class="sheet"><header>${project.manifest.company}</header><main><p class="campaign">${brief.campaignName}</p><h1>${brief.headline}</h1><h2>${brief.subheadline}</h2><div class="body">${brief.body}</div><div class="cta">${brief.callToAction}</div></main></article></body></html>`;
  }
  return invoke<string>('get_creative_a4_html', { projectId, creativeId });
}

export async function exportA4Pdf(projectId: string, creativeId: string, outputPath: string): Promise<string> {
  if (!isTauri()) throw new Error('PDF export is available in the desktop app.');
  return invoke<string>('export_a4_pdf', { projectId, creativeId, outputPath });
}

const browserSkillCatalog: MarketplaceSkill[] = [
  ['offplan-evidence-audit', 'Evidence Audit', 'Classify project sources and block unsupported claims.', 'Project intake'],
  ['floorplan-concept-planning', 'Floorplan Concepts', 'Create topology-safe room concept briefs.', 'Image production'],
  ['panorama-production', 'Panorama Production', 'Prepare, stitch and validate 360° experiences.', 'Interactive tours'],
  ['property-poster-studio', 'Property Poster Studio', 'Create Markdown-to-PDF A4 sales sheets or separately reviewed Image Model poster requests.', 'Creative Studio'],
  ['property-video-storyboard', 'Property Video Storyboard', 'Create timed HyperFrames-ready storyboards.', 'Creative Studio'],
  ['deployment-readback', 'Deployment Read-back', 'Verify local and public release states.', 'Deployment'],
].map(([id, name, description, category]) => ({ id, name, description, category, version: '1.0.0', author: 'Estate Studio', capabilities: [], installed: false }));

export async function listSkillMarketplace(): Promise<MarketplaceSkill[]> {
  if (isTauri()) return invoke<MarketplaceSkill[]>('list_skill_marketplace');
  return browserSkillCatalog;
}

export async function installMarketplaceSkill(skillId: string): Promise<MarketplaceSkill[]> {
  if (!isTauri()) throw new Error('Skill installation is available in the desktop app.');
  return invoke<MarketplaceSkill[]>('install_marketplace_skill', { skillId });
}

export async function setProjectSkillEnabled(projectId: string, skillId: string, enabled: boolean): Promise<ProjectRecord> {
  if (!isTauri()) throw new Error('Project Skill activation is available in the desktop app.');
  return invoke<ProjectRecord>('set_project_skill_enabled', { projectId, skillId, enabled });
}
