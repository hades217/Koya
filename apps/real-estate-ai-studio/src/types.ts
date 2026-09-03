export type UnitRecord = {
  id: string;
  label: string;
  summary: string;
  status: string;
  tourAvailable: boolean;
  floorplanAssetId?: string;
  rooms: RoomRecord[];
  roomGraphLocked: boolean;
  entranceRoomId?: string;
  openings?: OpeningRecord[];
  floorplanVersion?: number;
  floorplanVersions?: Array<{ version: number; assetId: string; createdAt: number }>;
  roomGraphVersion?: number;
  roomGraphVersions?: Array<{ version: number; fingerprint: string; locked: boolean; relativePath: string; createdAt: number }>;
};

export type OpeningRecord = {
  id: string;
  fromRoomId: string;
  toRoomId?: string;
  kind: 'entrance' | 'door' | 'opening';
  x: number;
  y: number;
  status: 'draft' | 'confirmed';
};

export type RoomRecord = {
  id: string;
  name: string;
  status: 'needs_evidence' | 'ready_for_generation' | 'in_review' | 'approved';
  panoramaStatus: 'not_started' | 'draft' | 'awaiting_approval' | 'ready';
  identityAssetId?: string;
  hotspotX?: number;
  hotspotY?: number;
  adjacentRoomIds: string[];
  panoramaAssetId?: string;
  thresholdAssetId?: string;
  reverseAssetId?: string;
  stillFallbackAssetId?: string;
  videoAssetId?: string;
  posterAssetId?: string;
};

export type RoomSupportingRole = 'threshold' | 'reverse' | 'still_fallback' | 'video' | 'poster';

export type GenerationJob = {
  id: string;
  unitId: string;
  roomId: string;
  roomName: string;
  assetRole: 'panorama';
  outputCount: 1;
  dimensions: string;
  panoramaMode: string;
  connectionMode: 'unconfigured' | 'codex' | 'managed_openai';
  priceStatus: 'unavailable' | 'available';
  status: 'draft' | 'blocked_capability' | 'awaiting_approval' | 'approved' | 'queued' | 'submitted' | 'processing' | 'completed' | 'failed' | 'timed_out' | 'cancelled';
  createdAt: number;
  inputs?: Array<{ assetId: string; role: 'topology_source' | 'identity_anchor' | 'style_reference'; checksumSha256: string; evidenceClass: EvidenceClass }>;
  cameraIntent?: string;
  requiredOpeningIds?: string[];
  fixedFixtures?: string[];
  fixedFixturesStatus?: 'available' | 'unavailable';
  prohibitions?: string[];
  modelId?: string;
  capabilityStatus?: 'available' | 'unsupported' | 'unavailable';
  requestedWidth?: number;
  requestedHeight?: number;
  providerChoice?: 'unconfigured' | 'codex' | 'managed_openai';
  priceAmountMinor?: number;
  priceCurrency?: string;
  customerCreditCost?: number;
  approvalState?: 'not_approved' | 'approved' | 'invalidated';
  approvalFingerprint?: string;
  topologyFingerprint?: string;
  idempotencyKey?: string;
  attempt?: number;
  progressPercent?: number;
  updatedAt?: number;
  lastHeartbeatAt?: number;
  timeoutAt?: number;
  failureCode?: string;
  failureReason?: string;
  retryAllowed?: boolean;
  approvedAt?: number;
  submittedAt?: number;
  completedAt?: number;
  providerTaskId?: string;
  stateEvents?: Array<{ id: string; fromStatus: GenerationJob['status']; toStatus: GenerationJob['status']; progressPercent: number; actor: string; note?: string; createdAt: number }>;
  outputs?: GenerationOutputRecord[];
  correctionRequests?: CorrectionRequestRecord[];
  fallbackPlans?: PanoramaFallbackPlan[];
};

export type PanoramaFallbackPlan = {
  id: string;
  parentOutputId: string;
  mode: 'cubefaces' | 'overlapping_tiles';
  reason: string;
  identityAnchorAssetId: string;
  topologySourceAssetId: string;
  continuityContract: string[];
  tasks: Array<{ id: string; label: string; yawStartDegrees: number; yawEndDegrees: number; pitchDegrees: number; overlapDegrees: number; adjacentTaskIds: string[]; approvalState: 'not_approved'; approvalFingerprint: string }>;
  status: 'draft_unapproved';
  createdAt: number;
};

export type GenerationOutputRecord = {
  id: string;
  version: number;
  relativePath: string;
  checksumSha256: string;
  mimeType: string;
  width: number;
  height: number;
  sizeBytes: number;
  providerRequestId: string;
  requestFingerprint: string;
  revisedPrompt?: string;
  sourceAttempt: number;
  status: 'pending_review' | 'accepted' | 'rejected' | 'superseded';
  publishability: 'blocked_visual_review' | 'blocked_panorama_qa' | 'blocked_dimension_mismatch' | 'blocked_rejected' | 'blocked_superseded' | 'publishable';
  rejectionReason?: string;
  reviewedAt?: number;
  panoramaQa?: PanoramaQaRecord;
  derivatives?: PanoramaDerivativeRecord[];
  createdAt: number;
};

export type PanoramaDerivativeRecord = {
  kind: 'seam_repaired_master' | 'mobile' | '4k' | '8k';
  relativePath: string;
  checksumSha256: string;
  width: number;
  height: number;
  sourceChecksumSha256: string;
  process: string;
  createdAt: number;
};

export type PanoramaQaRecord = {
  projectionStatus: string;
  dimensionStatus: string;
  decodedWidth: number;
  decodedHeight: number;
  seamEdgeDelta: number;
  seamStatus: string;
  topologyStatus: string;
  confirmedOpeningIds: string[];
  horizonStatus: string;
  orientationStatus: string;
  yawDegrees: number;
  runtimeStatus: string;
  usagePermissionReference: string;
  overallStatus: 'passed' | 'failed';
  reviewer: string;
  createdAt: number;
};

export type CorrectionRequestRecord = { id: string; parentOutputId: string; instruction: string; status: 'draft_unsubmitted'; createdAt: number };

export type GenerationCapabilityRecord = {
  providerChoice: 'codex' | 'managed_openai';
  modelId: string;
  accessStatus: 'available' | 'unavailable' | 'denied';
  supportedSizes: Array<{ width: number; height: number }>;
  panoramaModes: string[];
  priceStatus: 'available' | 'unavailable';
  priceAmountMinor?: number;
  priceCurrency?: string;
  quotaStatus: 'available' | 'unavailable' | 'exhausted';
  quotaRemaining?: number;
  checkedAt: number;
  expiresAt: number;
  source: string;
};

export type CapabilityRegistry = { schemaVersion: number; records: GenerationCapabilityRecord[]; updatedAt: number };

export type CodexGenerationAdapterReport = {
  provider: 'codex';
  projectId: string;
  jobId: string;
  installed: boolean;
  authenticated: boolean;
  accountLabel?: string;
  approvalState: string;
  approvalFingerprint: string;
  idempotencyKey: string;
  progressPercent: number;
  capabilityStatus: 'unavailable';
  modelAccessStatus: 'unavailable';
  exactDimensionsStatus: 'unavailable';
  panoramaModeStatus: 'unavailable';
  priceStatus: 'unavailable';
  quotaStatus: 'unavailable';
  submissionStatus: 'blocked_capability';
  progressProtocol: string[];
  credentialPolicy: string;
  detail: string;
};

export type CodexAdapterInspection = { project: ProjectRecord; report: CodexGenerationAdapterReport };

export type BindGatewaySessionInput = {
  baseUrl: string;
  accessToken: string;
  subscriptionId: string;
  expiresAt: number;
};

export type GatewaySessionStatus = {
  configured: boolean;
  authenticated: boolean;
  subscriptionId?: string;
  expiresAt?: number;
  baseOrigin?: string;
  credentialStorage: 'memory_only';
  detail: string;
};

export type ManagedCapabilityResponse = {
  requestFingerprint: string;
  entitlementStatus: string;
  capabilityStatus: string;
  modelId?: string;
  supportedWidth?: number;
  supportedHeight?: number;
  panoramaMode?: string;
  priceStatus: 'available' | 'unavailable';
  priceAmountMinor?: number;
  priceCurrency?: string;
  creditCost?: number;
  quotaStatus: 'available' | 'unavailable' | 'exhausted';
  quotaRemaining?: number;
  checkedAt: number;
  expiresAt: number;
};

export type ManagedGatewayCapabilityResult = { project: ProjectRecord; response: ManagedCapabilityResponse };

export type UsageLedgerEntry = {
  id: string;
  kind: 'capability_quote' | 'credit_reserved' | 'credit_consumed' | 'credit_released' | 'reconciliation';
  jobId: string;
  idempotencyKey: string;
  requestFingerprint: string;
  providerChoice: 'codex' | 'managed_openai';
  status: string;
  customerCreditCost?: number;
  customerPriceAmountMinor?: number;
  customerPriceCurrency?: string;
  createdAt: number;
};

export type BackgroundJobRecord = {
  id: string;
  kind: 'panorama_processing' | 'static_build' | 'upload' | 'download';
  subjectId: string;
  idempotencyKey: string;
  status: 'queued' | 'running' | 'completed' | 'failed' | 'cancelled';
  attempt: number;
  progressPercent: number;
  checkpoint: string;
  failureReason?: string;
  leaseExpiresAt?: number;
  createdAt: number;
  updatedAt: number;
};

export type CreativeScene = {
  order: number;
  durationSeconds: number;
  title: string;
  visualDirection: string;
  onScreenText: string;
};

export type A4DocumentType =
  | 'project_sales_brochure'
  | 'floorplan_book'
  | 'unit_sales_sheet'
  | 'price_availability'
  | 'finishes_specifications'
  | 'agent_kit'
  | 'showroom_eoi_pack';

export type CreativeBrief = {
  id: string;
  kind: 'poster' | 'video';
  posterMode?: 'a4_sales_sheet' | 'ai_creative';
  a4DocumentType?: A4DocumentType;
  title: string;
  campaignName: string;
  unitId?: string;
  audience: string;
  objective: string;
  headline: string;
  subheadline: string;
  body: string;
  callToAction: string;
  projectHighlights: string[];
  format: 'instagram_portrait' | 'a4_portrait' | 'story_portrait' | 'social_vertical' | 'social_square' | 'web_landscape';
  width: number;
  height: number;
  durationSeconds?: number;
  visualDirection: string;
  imagePrompt?: string;
  generationProvider?: 'openai_image_model';
  priceStatus?: 'unavailable' | 'available';
  evidenceAssetIds: string[];
  scenes: CreativeScene[];
  warnings: string[];
  status: 'draft' | 'preview_ready' | 'awaiting_generation' | 'package_ready';
  workflow: 'deterministic_svg' | 'image_model' | 'hyperframes';
  outputRelativePath?: string;
  createdAt: number;
  updatedAt: number;
};

export type GenerateCreativeBriefInput = {
  kind: 'poster' | 'video';
  posterMode?: 'a4_sales_sheet' | 'ai_creative';
  a4DocumentType?: A4DocumentType;
  request?: string;
  campaignName: string;
  unitId?: string;
  audience: string;
  objective: string;
  format: CreativeBrief['format'];
  evidenceAssetIds: string[];
};

export type AssetCategory = 'drawings' | 'renders' | 'photos' | 'panoramas' | 'videos' | 'brand' | 'copy';
export type EvidenceClass = 'official' | 'approved_render' | 'concept_floorplan_grounded' | 'concept_style_only' | 'unknown';

export type AssetRecord = {
  id: string;
  name: string;
  category: AssetCategory;
  evidenceClass: EvidenceClass;
  status: 'imported' | 'accepted' | 'needs_review' | 'rejected';
  importedAt: number;
  relativePath: string;
  sizeBytes: number;
  checksumSha256: string;
  mimeType: string;
  sourceOwner: string;
  usagePermission: string;
  originalRelativePath: string;
  derivativeRelativePaths: string[];
  duplicateOfAssetId?: string;
  unitId?: string;
  width?: number;
  height?: number;
  rejectionReasonCode?: AssetRejectionReason;
  rejectionNotes?: string;
  reviewEvents?: Array<{ id: string; fromStatus: AssetRecord['status']; toStatus: AssetRecord['status']; actor: string; reasonCode?: AssetRejectionReason; notes?: string; createdAt: number }>;
};

export type AssetRejectionReason = 'rights_missing' | 'evidence_conflict' | 'unreadable' | 'duplicate' | 'too_small' | 'incorrect_unit' | 'topology_conflict' | 'quality_failure' | 'other';

export type CompanyProfileSnapshot = {
  id: string;
  name: string;
};

export type CompanyProfile = {
  id: string;
  name: string;
  locale: string;
  measurementUnits: 'metric' | 'imperial';
  brand: { primary: string; accent: string };
  createdAt: number;
  updatedAt: number;
};

export type ApprovalEvent = {
  id: string;
  subjectType: string;
  subjectId: string;
  decision: string;
  actor: string;
  reason?: string;
  createdAt: number;
};

export type QaRecord = {
  id: string;
  scope: string;
  status: string;
  checks: string[];
  reviewer: string;
  createdAt: number;
};

export type ProjectModules = {
  tour: 'ready' | 'in_progress' | 'not_started';
  creative: 'ready' | 'in_progress' | 'not_started';
  deployment: 'ready' | 'in_progress' | 'not_started';
};

export type ProjectDesignSpec = {
  status: 'not_started' | 'draft' | 'approved';
  direction: string;
  sourceBasis: string[];
  colours: {
    paper: string;
    ink: string;
    accent: string;
    muted: string;
    botanical: string;
  };
  typography: {
    display: string;
    body: string;
  };
  layout: {
    page: 'A4 portrait';
    marginMm: number;
    grid: string;
    imageTreatment: string;
  };
};

export type ProjectManifest = {
  schemaVersion: number;
  projectId: string;
  name: string;
  company: string;
  companyProfile: CompanyProfileSnapshot;
  location: string;
  locale: string;
  measurementUnits: 'metric' | 'imperial';
  accessMode: 'public' | 'unlisted' | 'private';
  disclosure: string;
  workflowMode?: 'standard' | 'advanced';
  status: string;
  readiness: number;
  readOnly: boolean;
  archivedAt?: number;
  createdAt: number;
  updatedAt: number;
  units: UnitRecord[];
  assets: AssetRecord[];
  generationJobs: GenerationJob[];
  creativeJobs: CreativeBrief[];
  enabledSkillIds: string[];
  releases: ReleaseRecord[];
  approvalEvents: ApprovalEvent[];
  qaRecords: QaRecord[];
  usageLedger: UsageLedgerEntry[];
  backgroundJobs: BackgroundJobRecord[];
  analyticsEvents?: Array<{ id: string; event: string; source: string; createdAt: number }>;
  designSpec?: ProjectDesignSpec;
  brand: {
    primary: string;
    accent: string;
  };
  modules: ProjectModules;
  tourPreviewUrl?: string;
};

export type MarketplaceSkill = {
  id: string;
  name: string;
  description: string;
  category: string;
  version: string;
  author: string;
  capabilities: string[];
  installed: boolean;
  installedAt?: number;
};

export type ReleaseRecord = {
  id: string;
  version: number;
  status: 'preview_ready' | 'uploaded_unverified' | 'readback_passed' | 'public_verified' | 'superseded' | 'failed';
  accessMode: 'local' | 'public' | 'unlisted' | 'private';
  createdAt: number;
  unitIds: string[];
  publicUrl?: string;
  verifiedAt?: number;
  supersedesReleaseId?: string;
  supersededByReleaseId?: string;
  rollbackFromReleaseId?: string;
  unitShareLinks?: Array<{ unitId: string; publicUrl?: string; privateSecretRef?: string; qrRelativePath?: string; createdAt: number }>;
  verificationQaId?: string;
};

export type ProjectRecord = {
  manifest: ProjectManifest;
  projectRoot: string;
};

export type DeploymentReadbackResult = {
  project: ProjectRecord;
  checks: string[];
};

export type VerifiedReleaseResult = {
  project: ProjectRecord;
  shareLinks: Array<{ unitId: string; url: string; qrPath?: string; private: boolean }>;
};

export type LicenseStatus = {
  installed: boolean;
  valid: boolean;
  detail: string;
  licenseId?: string;
  customer?: string;
  edition?: string;
  expiresAt?: number;
  entitlements: string[];
  allowedRoles: string[];
  activeRole?: string;
};

export type DesktopSettings = {
  schemaVersion: number;
  storageMode: string;
  storagePath: string;
  updateChannel: 'stable' | 'beta';
  autoCheckUpdates: boolean;
  managedGatewayBaseUrl?: string;
  analyticsEndpoint?: string;
  codexAuthMode: string;
  publishingTargets: Array<{ id: string; label: string; kind: 'customer_owned_directory'; rootPath: string }>;
  secretReferences: Array<{ id: string; label: string; kind: 'hosting_token_file' | 'sftp_key' | 'analytics_token_file'; filePath: string }>;
  updatedAt: number;
};

export type UpdateDesktopSettingsInput = Pick<DesktopSettings, 'updateChannel' | 'autoCheckUpdates' | 'managedGatewayBaseUrl' | 'analyticsEndpoint' | 'publishingTargets' | 'secretReferences'>;

export type SignedUpdateStatus = {
  configured: boolean;
  available: boolean;
  channel: string;
  currentVersion: string;
  targetVersion?: string;
  notes?: string;
  publishedAt?: string;
  recoveryPending: boolean;
  detail: string;
};

export type CreateProjectInput = {
  name: string;
  company: string;
  location: string;
  unitIds: string[];
};

export type AddUnitInput = {
  unitId: string;
  label: string;
  summary: string;
};

export type DuplicateProjectInput = {
  name: string;
  company: string;
};

export type ImportAssetsInput = {
  paths: string[];
  category: AssetCategory;
  evidenceClass: EvidenceClass;
  unitId?: string;
  sourceOwner: string;
  usagePermission: string;
};

export type UpdateProjectInput = {
  name: string;
  company: string;
  location: string;
  primary: string;
  accent: string;
  expectedUpdatedAt?: number;
};

export type AiProjectUpdateDraft = {
  summary: string;
  baseUpdatedAt: number;
  name?: string;
  company?: string;
  location?: string;
  primary?: string;
  accent?: string;
  warnings: string[];
};

export type RoomGraphNodeInput = {
  roomId: string;
  hotspotX: number;
  hotspotY: number;
  adjacentRoomIds: string[];
};

export type OpeningInput = Omit<OpeningRecord, 'status'>;
