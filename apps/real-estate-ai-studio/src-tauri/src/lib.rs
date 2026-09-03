use base64::Engine;
use ed25519_dalek::{Signature, VerifyingKey};
use printpdf::{BuiltinFont, Color, IndirectFontRef, Line, Mm, PdfDocument, Point, Rgb};
use pulldown_cmark::{html, Options, Parser};
use qrcode::{render::svg, QrCode};
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager, State};
use tauri_plugin_updater::UpdaterExt;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

mod ai;
mod gateway;

const CURRENT_SCHEMA_VERSION: u32 = 30;
const PROJECT_BUNDLE_SCHEMA_VERSION: u32 = 1;
const PREFERRED_PROVIDER_PANORAMA_WIDTH: u32 = 3840;
const PREFERRED_PROVIDER_PANORAMA_HEIGHT: u32 = 1920;
const SALES_DOCUMENT_TYPES: [&str; 7] = [
    "project_sales_brochure",
    "floorplan_book",
    "unit_sales_sheet",
    "price_availability",
    "finishes_specifications",
    "agent_kit",
    "showroom_eoi_pack",
];

fn default_locale() -> String {
    "en-AU".into()
}

fn default_measurement_units() -> String {
    "metric".into()
}

fn default_access_mode() -> String {
    "unlisted".into()
}

fn default_disclosure() -> String {
    "Disclosure not supplied".into()
}

fn default_workflow_mode() -> String {
    "standard".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoomRecord {
    id: String,
    name: String,
    status: String,
    panorama_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    identity_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hotspot_x: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    hotspot_y: Option<f32>,
    #[serde(default)]
    adjacent_room_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    panorama_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    threshold_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reverse_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    still_fallback_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    video_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poster_asset_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpeningRecord {
    id: String,
    from_room_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    to_room_id: Option<String>,
    kind: String,
    x: f32,
    y: f32,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FloorPlanVersionRecord {
    version: u32,
    asset_id: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoomGraphVersionRecord {
    version: u32,
    fingerprint: String,
    locked: bool,
    relative_path: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationJob {
    id: String,
    unit_id: String,
    room_id: String,
    room_name: String,
    asset_role: String,
    output_count: u8,
    dimensions: String,
    panorama_mode: String,
    connection_mode: String,
    price_status: String,
    status: String,
    created_at: u64,
    #[serde(default)]
    inputs: Vec<GenerationInputRecord>,
    #[serde(default)]
    camera_intent: String,
    #[serde(default)]
    required_opening_ids: Vec<String>,
    #[serde(default)]
    fixed_fixtures: Vec<String>,
    #[serde(default)]
    fixed_fixtures_status: String,
    #[serde(default)]
    prohibitions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    model_id: Option<String>,
    #[serde(default)]
    capability_status: String,
    #[serde(default)]
    requested_width: u32,
    #[serde(default)]
    requested_height: u32,
    #[serde(default)]
    provider_choice: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_amount_minor: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    customer_credit_cost: Option<u64>,
    #[serde(default)]
    approval_state: String,
    #[serde(default)]
    approval_fingerprint: String,
    #[serde(default)]
    topology_fingerprint: String,
    #[serde(default)]
    idempotency_key: String,
    #[serde(default)]
    attempt: u32,
    #[serde(default)]
    progress_percent: u8,
    #[serde(default)]
    updated_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_heartbeat_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    timeout_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failure_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failure_reason: Option<String>,
    #[serde(default)]
    retry_allowed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    approved_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    submitted_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    completed_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    provider_task_id: Option<String>,
    #[serde(default)]
    state_events: Vec<GenerationStateEvent>,
    #[serde(default)]
    outputs: Vec<GenerationOutputRecord>,
    #[serde(default)]
    correction_requests: Vec<CorrectionRequestRecord>,
    #[serde(default)]
    fallback_plans: Vec<PanoramaFallbackPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanoramaFallbackPlan {
    id: String,
    parent_output_id: String,
    mode: String,
    reason: String,
    identity_anchor_asset_id: String,
    topology_source_asset_id: String,
    continuity_contract: Vec<String>,
    tasks: Vec<PanoramaFallbackTask>,
    status: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanoramaFallbackTask {
    id: String,
    label: String,
    yaw_start_degrees: f64,
    yaw_end_degrees: f64,
    pitch_degrees: f64,
    overlap_degrees: f64,
    adjacent_task_ids: Vec<String>,
    approval_state: String,
    approval_fingerprint: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePanoramaFallbackInput {
    mode: String,
    reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationOutputRecord {
    id: String,
    version: u32,
    relative_path: String,
    checksum_sha256: String,
    mime_type: String,
    width: u32,
    height: u32,
    size_bytes: u64,
    provider_request_id: String,
    request_fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    revised_prompt: Option<String>,
    source_attempt: u32,
    status: String,
    publishability: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rejection_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reviewed_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    panorama_qa: Option<PanoramaQaRecord>,
    #[serde(default)]
    derivatives: Vec<PanoramaDerivativeRecord>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanoramaDerivativeRecord {
    kind: String,
    relative_path: String,
    checksum_sha256: String,
    width: u32,
    height: u32,
    source_checksum_sha256: String,
    process: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanoramaQaRecord {
    projection_status: String,
    dimension_status: String,
    decoded_width: u32,
    decoded_height: u32,
    seam_edge_delta: f64,
    seam_status: String,
    topology_status: String,
    confirmed_opening_ids: Vec<String>,
    horizon_status: String,
    orientation_status: String,
    yaw_degrees: f64,
    runtime_status: String,
    usage_permission_reference: String,
    overall_status: String,
    reviewer: String,
    created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PanoramaQaInput {
    topology_passed: bool,
    confirmed_opening_ids: Vec<String>,
    horizon_passed: bool,
    seam_passed: bool,
    orientation_passed: bool,
    yaw_degrees: f64,
    runtime_passed: bool,
    usage_permission_reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CorrectionRequestRecord {
    id: String,
    parent_output_id: String,
    instruction: String,
    status: String,
    created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IngestGenerationOutputInput {
    source_path: String,
    provider_request_id: String,
    request_fingerprint: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReviewGenerationOutputInput {
    decision: String,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    correction_instruction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationStateEvent {
    id: String,
    from_status: String,
    to_status: String,
    progress_percent: u8,
    actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationInputRecord {
    asset_id: String,
    role: String,
    checksum_sha256: String,
    evidence_class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreativeScene {
    order: u8,
    duration_seconds: u16,
    title: String,
    visual_direction: String,
    on_screen_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreativeBrief {
    id: String,
    kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    poster_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    a4_document_type: Option<String>,
    title: String,
    campaign_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    unit_id: Option<String>,
    audience: String,
    objective: String,
    headline: String,
    subheadline: String,
    body: String,
    call_to_action: String,
    #[serde(default)]
    project_highlights: Vec<String>,
    format: String,
    width: u32,
    height: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    duration_seconds: Option<u16>,
    visual_direction: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    image_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    generation_provider: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_status: Option<String>,
    #[serde(default)]
    evidence_asset_ids: Vec<String>,
    #[serde(default)]
    scenes: Vec<CreativeScene>,
    #[serde(default)]
    warnings: Vec<String>,
    status: String,
    workflow: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    output_relative_path: Option<String>,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReleaseRecord {
    id: String,
    version: u32,
    status: String,
    access_mode: String,
    created_at: u64,
    unit_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    public_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    verified_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    supersedes_release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    superseded_by_release_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rollback_from_release_id: Option<String>,
    #[serde(default)]
    unit_share_links: Vec<UnitShareLinkRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    verification_qa_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnitShareLinkRecord {
    unit_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    public_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    private_secret_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    qr_relative_path: Option<String>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StaticBuildFileRecord {
    path: String,
    size_bytes: u64,
    checksum_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StaticBuildReport {
    schema_version: u32,
    immutable: bool,
    generated_at: u64,
    manifest_checksum_sha256: String,
    max_texture_memory_bytes: u64,
    validations: Vec<String>,
    files: Vec<StaticBuildFileRecord>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CustomerDirectoryPublishInput {
    release_id: String,
    destination_path: String,
    access_mode: String,
    confirmed_unverified_upload: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DeploymentReadbackInput {
    release_id: String,
    base_url: String,
    expected_access_mode: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DeploymentReadbackResult {
    project: ProjectRecord,
    checks: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct VerifiedUnitShareLink {
    unit_id: String,
    url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    qr_path: Option<String>,
    private: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct VerifiedReleaseResult {
    project: ProjectRecord,
    share_links: Vec<VerifiedUnitShareLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LicenseClaims {
    schema_version: u32,
    license_id: String,
    customer: String,
    edition: String,
    issued_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expires_at: Option<u64>,
    #[serde(default)]
    entitlements: Vec<String>,
    #[serde(default)]
    allowed_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SignedLicenseDocument {
    claims: LicenseClaims,
    signature_base64: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LicenseStatus {
    installed: bool,
    valid: bool,
    detail: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    license_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    edition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    expires_at: Option<u64>,
    entitlements: Vec<String>,
    allowed_roles: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    active_role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalRoleAssignment {
    role: String,
    display_name: String,
    assigned_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LocalRoleRegistry {
    schema_version: u32,
    active_role: String,
    assignments: Vec<LocalRoleAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PublishingTargetSetting {
    id: String,
    label: String,
    kind: String,
    root_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SecretReferenceSetting {
    id: String,
    label: String,
    kind: String,
    file_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DesktopSettings {
    schema_version: u32,
    storage_mode: String,
    storage_path: String,
    update_channel: String,
    auto_check_updates: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    managed_gateway_base_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    analytics_endpoint: Option<String>,
    codex_auth_mode: String,
    #[serde(default)]
    publishing_targets: Vec<PublishingTargetSetting>,
    #[serde(default)]
    secret_references: Vec<SecretReferenceSetting>,
    updated_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateDesktopSettingsInput {
    update_channel: String,
    auto_check_updates: bool,
    #[serde(default)]
    managed_gateway_base_url: Option<String>,
    #[serde(default)]
    analytics_endpoint: Option<String>,
    #[serde(default)]
    publishing_targets: Vec<PublishingTargetSetting>,
    #[serde(default)]
    secret_references: Vec<SecretReferenceSetting>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SignedUpdateStatus {
    configured: bool,
    available: bool,
    channel: String,
    current_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    published_at: Option<String>,
    recovery_pending: bool,
    detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateRecoveryMarker {
    schema_version: u32,
    previous_version: String,
    target_version: String,
    channel: String,
    started_at: u64,
    status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnitRecord {
    id: String,
    label: String,
    summary: String,
    status: String,
    tour_available: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    floorplan_asset_id: Option<String>,
    #[serde(default)]
    rooms: Vec<RoomRecord>,
    #[serde(default)]
    room_graph_locked: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    entrance_room_id: Option<String>,
    #[serde(default)]
    openings: Vec<OpeningRecord>,
    #[serde(default)]
    floorplan_version: u32,
    #[serde(default)]
    floorplan_versions: Vec<FloorPlanVersionRecord>,
    #[serde(default)]
    room_graph_version: u32,
    #[serde(default)]
    room_graph_versions: Vec<RoomGraphVersionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetRecord {
    id: String,
    name: String,
    category: String,
    evidence_class: String,
    status: String,
    imported_at: u64,
    relative_path: String,
    size_bytes: u64,
    #[serde(default)]
    checksum_sha256: String,
    #[serde(default)]
    mime_type: String,
    #[serde(default)]
    source_owner: String,
    #[serde(default)]
    usage_permission: String,
    #[serde(default)]
    original_relative_path: String,
    #[serde(default)]
    derivative_relative_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    duplicate_of_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    unit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    height: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rejection_reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    rejection_notes: Option<String>,
    #[serde(default)]
    review_events: Vec<AssetReviewEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AssetReviewEvent {
    id: String,
    from_status: String,
    to_status: String,
    actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    notes: Option<String>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Brand {
    primary: String,
    accent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CompanyProfileSnapshot {
    id: String,
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CompanyProfile {
    id: String,
    name: String,
    locale: String,
    measurement_units: String,
    brand: Brand,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CompanyRegistry {
    #[serde(default)]
    companies: Vec<CompanyProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct CapabilityRegistry {
    schema_version: u32,
    #[serde(default)]
    records: Vec<GenerationCapabilityRecord>,
    #[serde(default)]
    updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerationCapabilityRecord {
    provider_choice: String,
    model_id: String,
    access_status: String,
    #[serde(default)]
    supported_sizes: Vec<ImageSizeCapability>,
    #[serde(default)]
    panorama_modes: Vec<String>,
    price_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_amount_minor: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    price_currency: Option<String>,
    quota_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    quota_remaining: Option<u64>,
    checked_at: u64,
    expires_at: u64,
    source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct ImageSizeCapability {
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApprovalEvent {
    id: String,
    subject_type: String,
    subject_id: String,
    decision: String,
    actor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UsageLedgerEntry {
    id: String,
    kind: String,
    job_id: String,
    idempotency_key: String,
    request_fingerprint: String,
    provider_choice: String,
    status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    customer_credit_cost: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    customer_price_amount_minor: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    customer_price_currency: Option<String>,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BackgroundJobRecord {
    id: String,
    kind: String,
    subject_id: String,
    idempotency_key: String,
    status: String,
    attempt: u32,
    progress_percent: u8,
    checkpoint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    lease_expires_at: Option<u64>,
    created_at: u64,
    updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct QaRecord {
    id: String,
    scope: String,
    status: String,
    #[serde(default)]
    checks: Vec<String>,
    reviewer: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnalyticsEventRecord {
    id: String,
    event: String,
    source: String,
    created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BundleFileRecord {
    path: String,
    size_bytes: u64,
    checksum_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectBundleManifest {
    bundle_schema_version: u32,
    project_schema_version: u32,
    project_id: String,
    exported_at: u64,
    files: Vec<BundleFileRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectModules {
    tour: String,
    creative: String,
    deployment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDesignColours {
    paper: String,
    ink: String,
    accent: String,
    muted: String,
    botanical: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ProjectDesignTypography {
    display: String,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDesignLayout {
    page: String,
    margin_mm: u8,
    grid: String,
    image_treatment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectDesignSpec {
    status: String,
    direction: String,
    source_basis: Vec<String>,
    colours: ProjectDesignColours,
    typography: ProjectDesignTypography,
    layout: ProjectDesignLayout,
}

fn default_project_design_spec() -> ProjectDesignSpec {
    ProjectDesignSpec {
        status: "not_started".into(),
        direction: "Project-specific design direction has not been approved.".into(),
        source_basis: vec![],
        colours: ProjectDesignColours {
            paper: "#f5f1e7".into(),
            ink: "#20241f".into(),
            accent: "#78917b".into(),
            muted: "#796d61".into(),
            botanical: "#66746b".into(),
        },
        typography: ProjectDesignTypography {
            display: "Didot, \"Bodoni 72\", Georgia, serif".into(),
            body: "\"Avenir Next\", \"Helvetica Neue\", Arial, sans-serif".into(),
        },
        layout: ProjectDesignLayout {
            page: "A4 portrait".into(),
            margin_mm: 18,
            grid: "Editorial asymmetric grid with generous negative space".into(),
            image_treatment: "Approved project imagery only; natural colour; no decorative tint".into(),
        },
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProjectManifest {
    schema_version: u32,
    project_id: String,
    name: String,
    company: String,
    #[serde(default)]
    company_profile: CompanyProfileSnapshot,
    location: String,
    #[serde(default = "default_locale")]
    locale: String,
    #[serde(default = "default_measurement_units")]
    measurement_units: String,
    #[serde(default = "default_access_mode")]
    access_mode: String,
    #[serde(default = "default_disclosure")]
    disclosure: String,
    #[serde(default = "default_workflow_mode")]
    workflow_mode: String,
    status: String,
    readiness: u8,
    read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    archived_at: Option<u64>,
    created_at: u64,
    updated_at: u64,
    units: Vec<UnitRecord>,
    #[serde(default)]
    assets: Vec<AssetRecord>,
    #[serde(default)]
    generation_jobs: Vec<GenerationJob>,
    #[serde(default)]
    creative_jobs: Vec<CreativeBrief>,
    #[serde(default)]
    enabled_skill_ids: Vec<String>,
    #[serde(default)]
    releases: Vec<ReleaseRecord>,
    #[serde(default)]
    approval_events: Vec<ApprovalEvent>,
    #[serde(default)]
    qa_records: Vec<QaRecord>,
    #[serde(default)]
    usage_ledger: Vec<UsageLedgerEntry>,
    #[serde(default)]
    background_jobs: Vec<BackgroundJobRecord>,
    #[serde(default)]
    analytics_events: Vec<AnalyticsEventRecord>,
    #[serde(default = "default_project_design_spec")]
    design_spec: ProjectDesignSpec,
    brand: Brand,
    modules: ProjectModules,
    #[serde(skip_serializing_if = "Option::is_none")]
    tour_preview_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectRecord {
    manifest: ProjectManifest,
    project_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateProjectInput {
    name: String,
    company: String,
    location: String,
    unit_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AddUnitInput {
    unit_id: String,
    label: String,
    summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DuplicateProjectInput {
    name: String,
    company: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImportAssetsInput {
    paths: Vec<String>,
    category: String,
    evidence_class: String,
    unit_id: Option<String>,
    source_owner: String,
    usage_permission: String,
}

struct PreparedImportSource {
    source: PathBuf,
    file_name: String,
    safe_name: String,
    size_bytes: u64,
    checksum_sha256: String,
    mime_type: String,
    dimensions: Option<(u32, u32)>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpdateProjectInput {
    name: String,
    company: String,
    location: String,
    primary: String,
    accent: String,
    expected_updated_at: Option<u64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GenerateCreativeBriefInput {
    kind: String,
    #[serde(default)]
    poster_mode: Option<String>,
    #[serde(default)]
    a4_document_type: Option<String>,
    #[serde(default)]
    request: Option<String>,
    campaign_name: String,
    unit_id: Option<String>,
    audience: String,
    objective: String,
    format: String,
    #[serde(default)]
    evidence_asset_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SaveCreativeBriefInput {
    brief: CreativeBrief,
    expected_updated_at: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreativeModelEnvelope {
    headline: String,
    subheadline: String,
    body: String,
    call_to_action: String,
    #[serde(default)]
    project_highlights: Vec<String>,
    visual_direction: String,
    #[serde(default)]
    image_prompt: String,
    #[serde(default)]
    scenes: Vec<CreativeScene>,
    #[serde(default)]
    warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct MarketplaceSkill {
    id: String,
    name: String,
    description: String,
    category: String,
    version: String,
    author: String,
    capabilities: Vec<String>,
    installed: bool,
    installed_at: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SkillRegistry {
    installed: Vec<InstalledSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InstalledSkill {
    id: String,
    version: String,
    installed_at: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiHistoryMessage {
    role: String,
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiChatInput {
    project_id: Option<String>,
    message: String,
    #[serde(default)]
    history: Vec<AiHistoryMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiProjectUpdateDraft {
    summary: String,
    #[serde(default)]
    base_updated_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    company: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    primary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    accent: Option<String>,
    #[serde(default)]
    warnings: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiModelEnvelope {
    reply_markdown: String,
    #[serde(default)]
    project_update_draft: Option<AiProjectUpdateDraft>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct AiChatResponse {
    provider: String,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    project_update_draft: Option<AiProjectUpdateDraft>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct CodexAdapterInspection {
    project: ProjectRecord,
    report: ai::CodexGenerationAdapterReport,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ManagedGatewayCapabilityResult {
    project: ProjectRecord,
    response: gateway::ManagedCapabilityResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RoomGraphNodeInput {
    room_id: String,
    hotspot_x: f32,
    hotspot_y: f32,
    adjacent_room_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct OpeningInput {
    id: String,
    from_room_id: String,
    #[serde(default)]
    to_room_id: Option<String>,
    kind: String,
    x: f32,
    y: f32,
}

fn unix_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn clean_required(value: &str, label: &str) -> Result<String, String> {
    let cleaned = value.trim();
    if cleaned.is_empty() {
        return Err(format!("{label} is required"));
    }
    if cleaned.len() > 120 {
        return Err(format!("{label} is too long"));
    }
    Ok(cleaned.to_string())
}

fn clean_optional(value: &str, label: &str) -> Result<String, String> {
    let cleaned = value.trim();
    if cleaned.len() > 160 {
        return Err(format!("{label} is too long"));
    }
    Ok(cleaned.to_string())
}

fn clean_hex_colour(value: &str, label: &str) -> Result<String, String> {
    let cleaned = value.trim().to_lowercase();
    if cleaned.len() != 7
        || !cleaned.starts_with('#')
        || !cleaned[1..]
            .chars()
            .all(|character| character.is_ascii_hexdigit())
    {
        return Err(format!("{label} must be a hex colour"));
    }
    Ok(cleaned)
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let file =
        File::open(path).map_err(|error| format!("Unable to open source for checksum: {error}"))?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("Unable to checksum source: {error}"))?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn source_mime_type(path: &Path) -> String {
    match path
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value.to_ascii_lowercase())
        .as_deref()
    {
        Some("pdf") => "application/pdf",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("tif" | "tiff") => "image/tiff",
        Some("heic") => "image/heic",
        Some("mp4" | "m4v") => "video/mp4",
        Some("mov") => "video/quicktime",
        Some("webm") => "video/webm",
        Some("doc") => "application/msword",
        Some("docx") => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        Some("txt") => "text/plain",
        Some("md") => "text/markdown",
        _ => "application/octet-stream",
    }
    .into()
}

fn allowed_source_extensions(category: &str) -> &'static [&'static str] {
    match category {
        "drawings" => &["pdf", "png", "jpg", "jpeg", "webp"],
        "renders" => &["png", "jpg", "jpeg", "webp", "tif", "tiff"],
        "photos" => &["png", "jpg", "jpeg", "webp", "heic"],
        "panoramas" => &["png", "jpg", "jpeg", "webp"],
        "videos" => &["mp4", "mov", "m4v", "webm"],
        "brand" => &["pdf", "png", "jpg", "jpeg", "svg"],
        "copy" => &["pdf", "doc", "docx", "txt", "md"],
        _ => &[],
    }
}

fn validate_source_for_import(path: &Path, category: &str) -> Result<Option<(u32, u32)>, String> {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("selected file");
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if !allowed_source_extensions(category).contains(&extension.as_str()) {
        return Err(format!(
            "{file_name} is not a supported {category} source. Choose one of: {}.",
            allowed_source_extensions(category)
                .join(", ")
                .to_ascii_uppercase()
        ));
    }
    let size = path
        .metadata()
        .map_err(|error| format!("Unable to inspect selected file: {error}"))?
        .len();
    if size == 0 {
        return Err(format!(
            "{file_name} is empty. Choose an intact source file and try again."
        ));
    }
    if matches!(extension.as_str(), "png" | "jpg" | "jpeg" | "webp") {
        let dimensions = image::image_dimensions(path).map_err(|_| {
            format!(
                "{file_name} cannot be decoded as an image. Choose an intact PNG, JPEG, or WebP file and try again."
            )
        })?;
        if dimensions.0 == 0 || dimensions.1 == 0 {
            return Err(format!(
                "{file_name} has invalid image dimensions. Choose an intact source file and try again."
            ));
        }
        return Ok(Some(dimensions));
    }
    let mut file = File::open(path)
        .map_err(|error| format!("Unable to inspect selected file contents: {error}"))?;
    let mut header = [0_u8; 16];
    let count = file
        .read(&mut header)
        .map_err(|error| format!("Unable to inspect selected file contents: {error}"))?;
    let header = &header[..count];
    let signature_valid = match extension.as_str() {
        "pdf" => header.starts_with(b"%PDF-"),
        "tif" | "tiff" => header.starts_with(b"II*\0") || header.starts_with(b"MM\0*"),
        "heic" => header.len() >= 12 && &header[4..8] == b"ftyp",
        "mp4" | "mov" | "m4v" => header.len() >= 12 && &header[4..8] == b"ftyp",
        "webm" => header.starts_with(&[0x1a, 0x45, 0xdf, 0xa3]),
        "doc" => header.starts_with(&[0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]),
        "docx" => header.starts_with(b"PK\x03\x04"),
        "txt" | "md" => std::str::from_utf8(header).is_ok(),
        "svg" => {
            let mut content = Vec::new();
            File::open(path)
                .and_then(|reader| reader.take(1024 * 1024).read_to_end(&mut content))
                .is_ok()
                && std::str::from_utf8(&content)
                    .is_ok_and(|text| text.to_ascii_lowercase().contains("<svg"))
        }
        _ => false,
    };
    if !signature_valid {
        return Err(format!(
            "{file_name} does not contain a valid {extension} file signature. Choose an intact source file and try again."
        ));
    }
    Ok(None)
}

fn portable_relative_path(path: &Path) -> Result<String, String> {
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, std::path::Component::Normal(_)))
    {
        return Err("Bundle paths must be relative and may not traverse directories".into());
    }
    let value = path.to_string_lossy().replace('\\', "/");
    if value.is_empty() || value == "bundle.json" {
        return Err("Bundle file path is invalid or reserved".into());
    }
    Ok(value)
}

fn safe_path_component(value: &str, label: &str) -> Result<String, String> {
    let value = clean_required(value, label)?;
    if value.len() > 128
        || !value
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || matches!(character, '-' | '_'))
    {
        return Err(format!(
            "{label} must be one portable ASCII path component using letters, numbers, hyphens, or underscores."
        ));
    }
    Ok(value)
}

fn collect_project_files(
    project_root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("Unable to read project directory: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Unable to inspect project file: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Unable to inspect project file type: {error}"))?;
        let relative = path
            .strip_prefix(project_root)
            .map_err(|_| "Project file escaped its root".to_string())?;
        if relative
            .components()
            .next()
            .is_some_and(|component| component.as_os_str() == "cache")
        {
            continue;
        }
        if file_type.is_symlink() {
            return Err(format!(
                "Portable export blocks symbolic links: {}",
                relative.display()
            ));
        }
        if file_type.is_dir() {
            collect_project_files(project_root, &path, files)?;
        } else if file_type.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn scan_bundle_file_for_secrets(relative: &Path, path: &Path) -> Result<(), String> {
    let name = relative
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let extension = relative
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if name == ".env"
        || name.starts_with(".env.")
        || ["pem", "key", "p12", "pfx"].contains(&extension.as_str())
    {
        return Err(format!(
            "Portable export blocked a credential-like file: {}",
            relative.display()
        ));
    }
    let text_extensions = [
        "json", "md", "txt", "html", "css", "js", "ts", "toml", "yaml", "yml", "xml", "svg",
    ];
    let size = path.metadata().map(|metadata| metadata.len()).unwrap_or(0);
    if size <= 2 * 1024 * 1024 && text_extensions.contains(&extension.as_str()) {
        let contents = fs::read_to_string(path).unwrap_or_default();
        let secret_patterns = [
            "-----BEGIN PRIVATE KEY-----",
            "OPENAI_API_KEY=",
            "ANTHROPIC_API_KEY=",
            "METRICOOL_USER_TOKEN=",
            "METRICOOL_API_TOKEN=",
        ];
        if secret_patterns
            .iter()
            .any(|pattern| contents.contains(pattern))
        {
            return Err(format!(
                "Portable export blocked credential material in {}",
                relative.display()
            ));
        }
    }
    Ok(())
}

fn build_bundle_manifest(
    project_root: &Path,
    project_id: &str,
) -> Result<ProjectBundleManifest, String> {
    let mut paths = vec![];
    collect_project_files(project_root, project_root, &mut paths)?;
    paths.sort();
    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = path
            .strip_prefix(project_root)
            .map_err(|_| "Project file escaped its root".to_string())?;
        let portable = portable_relative_path(relative)?;
        scan_bundle_file_for_secrets(relative, &path)?;
        files.push(BundleFileRecord {
            path: portable,
            size_bytes: path.metadata().map(|value| value.len()).unwrap_or(0),
            checksum_sha256: sha256_file(&path)?,
        });
    }
    if !files.iter().any(|file| file.path == "project.json") {
        return Err("Portable export requires project.json".into());
    }
    Ok(ProjectBundleManifest {
        bundle_schema_version: PROJECT_BUNDLE_SCHEMA_VERSION,
        project_schema_version: CURRENT_SCHEMA_VERSION,
        project_id: project_id.into(),
        exported_at: unix_time(),
        files,
    })
}

fn ensure_current_project(
    expected_updated_at: Option<u64>,
    actual_updated_at: u64,
) -> Result<(), String> {
    if expected_updated_at.is_some_and(|expected| expected != actual_updated_at) {
        return Err("This project changed after the AI draft was prepared. Review the latest values and prepare the draft again.".into());
    }
    Ok(())
}

fn parse_ai_model_envelope(value: &str) -> Result<AiModelEnvelope, String> {
    let trimmed = value.trim();
    let json = if trimmed.starts_with("```json") && trimmed.ends_with("```") {
        trimmed
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim()
    } else if trimmed.starts_with("```") && trimmed.ends_with("```") {
        trimmed
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    } else {
        trimmed
    };
    let envelope: AiModelEnvelope = serde_json::from_str(json)
        .map_err(|_| "Property AI returned an invalid structured response.".to_string())?;
    if envelope.reply_markdown.trim().is_empty() {
        return Err("Property AI returned an empty response.".into());
    }
    Ok(envelope)
}

fn normalize_ai_project_draft(
    manifest: &ProjectManifest,
    draft: AiProjectUpdateDraft,
) -> Result<Option<AiProjectUpdateDraft>, String> {
    if manifest.read_only {
        return Ok(None);
    }
    let mut normalized = AiProjectUpdateDraft {
        summary: clean_required(&draft.summary, "Draft summary")?,
        base_updated_at: manifest.updated_at,
        name: draft
            .name
            .map(|value| clean_required(&value, "Development name"))
            .transpose()?,
        company: draft
            .company
            .map(|value| clean_required(&value, "Customer company"))
            .transpose()?,
        location: draft
            .location
            .map(|value| clean_optional(&value, "Location"))
            .transpose()?,
        primary: draft
            .primary
            .map(|value| clean_hex_colour(&value, "Primary colour"))
            .transpose()?,
        accent: draft
            .accent
            .map(|value| clean_hex_colour(&value, "Accent colour"))
            .transpose()?,
        warnings: draft
            .warnings
            .into_iter()
            .map(|warning| warning.trim().chars().take(240).collect::<String>())
            .filter(|warning| !warning.is_empty())
            .take(5)
            .collect(),
    };
    if normalized.name.as_deref() == Some(manifest.name.as_str()) {
        normalized.name = None;
    }
    if normalized.company.as_deref() == Some(manifest.company.as_str()) {
        normalized.company = None;
    }
    if normalized.location.as_deref() == Some(manifest.location.as_str()) {
        normalized.location = None;
    }
    if normalized.primary.as_deref() == Some(manifest.brand.primary.as_str()) {
        normalized.primary = None;
    }
    if normalized.accent.as_deref() == Some(manifest.brand.accent.as_str()) {
        normalized.accent = None;
    }
    let has_change = normalized.name.is_some()
        || normalized.company.is_some()
        || normalized.location.is_some()
        || normalized.primary.is_some()
        || normalized.accent.is_some();
    Ok(has_change.then_some(normalized))
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut separator_pending = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            if separator_pending && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character.to_ascii_lowercase());
            separator_pending = false;
        } else {
            separator_pending = true;
        }
    }
    if slug.is_empty() {
        "development".into()
    } else {
        slug
    }
}

fn projects_root(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve app data directory: {error}"))?
        .join("projects");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create project library: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Unable to protect project library: {error}"))?;
        for entry in fs::read_dir(&root)
            .map_err(|error| format!("Unable to inspect project-library permissions: {error}"))?
        {
            let path = entry
                .map_err(|error| format!("Unable to inspect a project permission: {error}"))?
                .path();
            if path.is_dir() {
                fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
                    .map_err(|error| format!("Unable to protect project workspace: {error}"))?;
                let manifest = path.join("project.json");
                if manifest.is_file() {
                    fs::set_permissions(&manifest, fs::Permissions::from_mode(0o600))
                        .map_err(|error| format!("Unable to protect project manifest: {error}"))?;
                }
            }
        }
    }
    Ok(root)
}

fn company_registry_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve app data directory: {error}"))?
        .join("companies.json"))
}

fn capability_registry_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve app data directory: {error}"))?
        .join("generation-capabilities.json"))
}

fn read_capability_registry(path: &Path) -> Result<CapabilityRegistry, String> {
    if !path.exists() {
        return Ok(CapabilityRegistry {
            schema_version: 1,
            records: vec![],
            updated_at: 0,
        });
    }
    let bytes =
        fs::read(path).map_err(|error| format!("Unable to read capability registry: {error}"))?;
    let registry: CapabilityRegistry = serde_json::from_slice(&bytes)
        .map_err(|error| format!("Invalid capability registry: {error}"))?;
    if registry.schema_version != 1 {
        return Err(format!(
            "Unsupported capability registry schema {}.",
            registry.schema_version
        ));
    }
    Ok(registry)
}

fn write_capability_registry(path: &Path, registry: &CapabilityRegistry) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to prepare capability registry: {error}"))?;
    }
    let bytes = serde_json::to_vec_pretty(registry)
        .map_err(|error| format!("Unable to encode capability registry: {error}"))?;
    fs::write(path, bytes).map_err(|error| format!("Unable to save capability registry: {error}"))
}

fn evaluate_generation_capability<'a>(
    registry: &'a CapabilityRegistry,
    provider_choice: &str,
    model_id: &str,
    width: u32,
    height: u32,
    panorama_mode: &str,
    output_count: u8,
    timestamp: u64,
) -> Result<&'a GenerationCapabilityRecord, String> {
    let record = registry
        .records
        .iter()
        .find(|record| record.provider_choice == provider_choice && record.model_id == model_id)
        .ok_or_else(|| "Provider/model capability is unavailable in the registry.".to_string())?;
    if record.expires_at <= timestamp {
        return Err("Provider/model capability record is expired.".into());
    }
    if record.access_status != "available" {
        return Err(format!("Model access is {}.", record.access_status));
    }
    if !record
        .supported_sizes
        .contains(&ImageSizeCapability { width, height })
    {
        return Err(format!(
            "Exact output size {width} × {height} is unsupported or unavailable."
        ));
    }
    if !record
        .panorama_modes
        .iter()
        .any(|mode| mode == panorama_mode)
    {
        return Err(format!(
            "Panorama mode {panorama_mode} is unsupported or unavailable."
        ));
    }
    if record.price_status != "available"
        || record.price_amount_minor.is_none()
        || record.price_currency.as_deref().is_none_or(str::is_empty)
    {
        return Err("Current provider price is unavailable.".into());
    }
    if record.quota_status != "available"
        || record
            .quota_remaining
            .is_none_or(|remaining| remaining < u64::from(output_count))
    {
        return Err("Provider quota is unavailable or insufficient.".into());
    }
    Ok(record)
}

fn read_company_registry(path: &Path) -> Result<CompanyRegistry, String> {
    if !path.exists() {
        return Ok(CompanyRegistry::default());
    }
    protect_private_file(path)?;
    let bytes =
        fs::read(path).map_err(|error| format!("Unable to read company profiles: {error}"))?;
    serde_json::from_slice(&bytes)
        .map_err(|error| format!("Invalid company profile registry: {error}"))
}

fn upsert_company_profile_at(
    path: &Path,
    profile: CompanyProfile,
) -> Result<CompanyProfile, String> {
    let mut registry = read_company_registry(path)?;
    if let Some(existing) = registry
        .companies
        .iter_mut()
        .find(|item| item.id == profile.id)
    {
        let created_at = existing.created_at;
        *existing = CompanyProfile {
            created_at,
            ..profile.clone()
        };
    } else {
        registry.companies.push(profile.clone());
    }
    registry
        .companies
        .sort_by(|left, right| left.name.cmp(&right.name));
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to prepare company registry: {error}"))?;
    }
    fs::write(
        path,
        serde_json::to_vec_pretty(&registry)
            .map_err(|error| format!("Unable to encode company profiles: {error}"))?,
    )
    .map_err(|error| format!("Unable to save company profiles: {error}"))?;
    protect_private_file(path)?;
    Ok(profile)
}

fn register_company_profile(
    app: &AppHandle,
    company: &str,
    locale: &str,
    measurement_units: &str,
    brand: &Brand,
) -> Result<CompanyProfile, String> {
    let timestamp = unix_time();
    upsert_company_profile_at(
        &company_registry_path(app)?,
        CompanyProfile {
            id: slugify(company),
            name: company.into(),
            locale: locale.into(),
            measurement_units: measurement_units.into(),
            brand: brand.clone(),
            created_at: timestamp,
            updated_at: timestamp,
        },
    )
}

#[tauri::command]
fn list_company_profiles(app: AppHandle) -> Result<Vec<CompanyProfile>, String> {
    Ok(read_company_registry(&company_registry_path(&app)?)?.companies)
}

fn write_manifest(root: &Path, manifest: &ProjectManifest) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(manifest)
        .map_err(|error| format!("Unable to encode project manifest: {error}"))?;
    let path = root.join("project.json");
    fs::write(&path, bytes).map_err(|error| format!("Unable to save project manifest: {error}"))?;
    protect_private_file(&path)?;
    write_project_design_spec(root, manifest)
}

fn write_project_design_spec(root: &Path, manifest: &ProjectManifest) -> Result<(), String> {
    let design_root = root.join("creative/design-system");
    fs::create_dir_all(&design_root)
        .map_err(|error| format!("Unable to create project design-system folder: {error}"))?;
    let json_path = design_root.join("design-spec.json");
    let json = serde_json::to_vec_pretty(&manifest.design_spec)
        .map_err(|error| format!("Unable to encode project design specification: {error}"))?;
    fs::write(&json_path, json)
        .map_err(|error| format!("Unable to save project design specification: {error}"))?;
    protect_private_file(&json_path)?;
    let sources = if manifest.design_spec.source_basis.is_empty() {
        "- unavailable".to_string()
    } else {
        manifest.design_spec.source_basis.iter().map(|item| format!("- {item}")).collect::<Vec<_>>().join("\n")
    };
    let markdown = format!(
        "---\nschemaVersion: 1\ndocumentRole: project_design_specification\nprojectId: {}\nstatus: {}\n---\n\n# {} project design specification\n\n## Design direction\n\n{}\n\n## Source basis\n\n{}\n\n## Colour system\n\n| Role | Value |\n| --- | --- |\n| Paper | {} |\n| Ink | {} |\n| Accent | {} |\n| Muted | {} |\n| Botanical | {} |\n\n## Typography\n\n- Display: {}\n- Body: {}\n\n## Document grid\n\n- Page: {}\n- Margin: {} mm\n- Grid: {}\n- Image treatment: {}\n\n## Production rule\n\nEvery project document must derive its HTML and PDF presentation from `design-spec.json`. Facts and copy remain governed by the document SOT.\n",
        manifest.project_id, manifest.design_spec.status, manifest.name, manifest.design_spec.direction,
        sources, manifest.design_spec.colours.paper, manifest.design_spec.colours.ink,
        manifest.design_spec.colours.accent, manifest.design_spec.colours.muted,
        manifest.design_spec.colours.botanical, manifest.design_spec.typography.display,
        manifest.design_spec.typography.body, manifest.design_spec.layout.page,
        manifest.design_spec.layout.margin_mm, manifest.design_spec.layout.grid,
        manifest.design_spec.layout.image_treatment,
    );
    let markdown_path = design_root.join("PROJECT_DESIGN_SPEC.md");
    fs::write(&markdown_path, markdown)
        .map_err(|error| format!("Unable to save project design specification Markdown: {error}"))?;
    protect_private_file(&markdown_path)
}

fn record_funnel_event(
    manifest: &mut ProjectManifest,
    event: &str,
    source: &str,
    timestamp: u64,
) -> Result<bool, String> {
    const EVENTS: [&str; 9] = [
        "project_created",
        "first_unit_added",
        "floorplan_locked",
        "asset_matrix_completed",
        "first_generation_approved",
        "first_room_accepted",
        "local_preview_passed",
        "first_tour_published",
        "public_deployment_verified",
    ];
    if !EVENTS.contains(&event) {
        return Err("Unsupported privacy-safe funnel event.".into());
    }
    if manifest
        .analytics_events
        .iter()
        .any(|item| item.event == event)
    {
        return Ok(false);
    }
    manifest.analytics_events.push(AnalyticsEventRecord {
        id: format!("analytics-{event}-{timestamp}"),
        event: event.into(),
        source: clean_required(source, "Analytics source")?,
        created_at: timestamp,
    });
    Ok(true)
}

fn record_asset_matrix_if_complete(manifest: &mut ProjectManifest, timestamp: u64) {
    let complete = manifest.units.iter().any(|unit| {
        unit.room_graph_locked
            && !unit.rooms.is_empty()
            && unit.rooms.iter().all(|room| {
                room.identity_asset_id.is_some()
                    && (room.panorama_asset_id.is_some() || room.still_fallback_asset_id.is_some())
            })
    });
    if complete {
        let _ = record_funnel_event(
            manifest,
            "asset_matrix_completed",
            "desktop_workflow",
            timestamp,
        );
    }
}

fn normalize_manifest(mut manifest: ProjectManifest) -> ProjectManifest {
    manifest.schema_version = CURRENT_SCHEMA_VERSION;
    if manifest.company_profile.name.trim().is_empty() {
        manifest.company_profile.name = manifest.company.clone();
    }
    if manifest.company_profile.id.trim().is_empty() {
        manifest.company_profile.id = slugify(&manifest.company_profile.name);
    }
    if manifest.locale.trim().is_empty() {
        manifest.locale = default_locale();
    }
    if manifest.measurement_units.trim().is_empty() {
        manifest.measurement_units = default_measurement_units();
    }
    if manifest.access_mode.trim().is_empty() {
        manifest.access_mode = default_access_mode();
    }
    if manifest.disclosure.trim().is_empty() {
        manifest.disclosure = default_disclosure();
    }
    if !matches!(manifest.workflow_mode.as_str(), "standard" | "advanced") {
        manifest.workflow_mode = default_workflow_mode();
    }
    for unit in &mut manifest.units {
        if unit.floorplan_asset_id.is_some() && unit.floorplan_version == 0 {
            unit.floorplan_version = 1;
        }
        if unit.room_graph_locked && unit.room_graph_version == 0 {
            unit.room_graph_version = 1;
        }
    }
    for job in &mut manifest.generation_jobs {
        if job.attempt == 0 {
            job.attempt = 1;
        }
        if job.updated_at == 0 {
            job.updated_at = job.created_at;
        }
        if job.idempotency_key.is_empty() {
            job.idempotency_key = format!(
                "{:x}",
                Sha256::digest(format!(
                    "{}:{}:{}",
                    job.id, job.approval_fingerprint, job.attempt
                ))
            );
        }
    }
    manifest
}

fn read_manifest(root: &Path) -> Result<ProjectManifest, String> {
    let path = root.join("project.json");
    let contents =
        fs::read(&path).map_err(|error| format!("Unable to read {}: {error}", path.display()))?;
    serde_json::from_slice(&contents)
        .map(normalize_manifest)
        .map_err(|error| format!("Invalid manifest {}: {error}", path.display()))
}

fn validate_project_id(project_id: &str) -> Result<(), String> {
    if project_id.is_empty()
        || project_id.contains('/')
        || project_id.contains('\\')
        || project_id.contains("..")
    {
        return Err("Invalid project identifier".into());
    }
    Ok(())
}

fn editable_project(
    app: &AppHandle,
    project_id: &str,
) -> Result<(PathBuf, ProjectManifest), String> {
    validate_project_id(project_id)?;
    let root = projects_root(app)?.join(project_id);
    if !root.is_dir() {
        return Err("Project folder is unavailable".into());
    }
    let manifest = read_manifest(&root)?;
    if manifest.read_only {
        return Err("The example project is read-only".into());
    }
    if manifest.archived_at.is_some() {
        return Err("Restore this archived project before editing it".into());
    }
    Ok((root, manifest))
}

fn update_readiness(manifest: &mut ProjectManifest) {
    let floorplans = manifest
        .units
        .iter()
        .filter(|unit| unit.floorplan_asset_id.is_some())
        .count() as u8;
    let categories = ["renders", "photos", "videos", "brand", "copy"]
        .iter()
        .filter(|category| {
            manifest
                .assets
                .iter()
                .any(|asset| asset.category == **category)
        })
        .count() as u8;
    let accepted = manifest
        .assets
        .iter()
        .filter(|asset| asset.status == "accepted")
        .count()
        .min(4) as u8;
    let rooms = manifest
        .units
        .iter()
        .map(|unit| unit.rooms.len())
        .sum::<usize>()
        .min(4) as u8;
    manifest.readiness = (12
        + floorplans.saturating_mul(8)
        + categories.saturating_mul(4)
        + accepted.saturating_mul(3)
        + rooms.saturating_mul(2))
    .min(78);
    if !manifest.assets.is_empty() {
        manifest.status = "Intake in progress · Sources need review".into();
        manifest.modules.tour = "in_progress".into();
    }
}

fn validate_connected_room_graph(rooms: &[RoomRecord]) -> Result<(), String> {
    if rooms.len() <= 1 {
        return Ok(());
    }
    for room in rooms {
        for adjacent_id in &room.adjacent_room_ids {
            let adjacent = rooms
                .iter()
                .find(|candidate| &candidate.id == adjacent_id)
                .ok_or_else(|| "Room graph contains an unknown adjacent room.".to_string())?;
            if !adjacent.adjacent_room_ids.contains(&room.id) {
                return Err("Every room connection must be recorded in both directions.".into());
            }
        }
    }
    let mut visited = vec![rooms[0].id.clone()];
    let mut index = 0;
    while index < visited.len() {
        let current = visited[index].clone();
        if let Some(room) = rooms.iter().find(|room| room.id == current) {
            for adjacent in &room.adjacent_room_ids {
                if !visited.contains(adjacent) {
                    visited.push(adjacent.clone());
                }
            }
        }
        index += 1;
    }
    if visited.len() != rooms.len() {
        return Err(
            "Every room must connect to the same navigable room graph before locking.".into(),
        );
    }
    Ok(())
}

fn room_graph_fingerprint(
    nodes: &[RoomGraphNodeInput],
    entrance_room_id: &Option<String>,
    openings: &[OpeningInput],
) -> Result<String, String> {
    let mut canonical_nodes = nodes.to_vec();
    canonical_nodes.sort_by(|left, right| left.room_id.cmp(&right.room_id));
    for node in &mut canonical_nodes {
        node.adjacent_room_ids.sort();
    }
    let mut canonical_openings = openings.to_vec();
    canonical_openings.sort_by(|left, right| left.id.cmp(&right.id));
    let bytes = serde_json::to_vec(&(canonical_nodes, entrance_room_id, canonical_openings))
        .map_err(|error| format!("Unable to fingerprint room graph: {error}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn invalidate_topology_dependents(unit: &mut UnitRecord) -> bool {
    let mut invalidated = false;
    for room in &mut unit.rooms {
        if room.identity_asset_id.is_some()
            || room.panorama_asset_id.is_some()
            || room.status == "approved"
        {
            room.status = "in_review".into();
            if room.panorama_asset_id.is_some() {
                room.panorama_status = "awaiting_approval".into();
            }
            invalidated = true;
        }
    }
    for opening in &mut unit.openings {
        opening.status = "draft".into();
    }
    unit.room_graph_locked = false;
    unit.tour_available = false;
    if invalidated {
        unit.status = "Topology changed · Dependent assets require review".into();
    }
    invalidated
}

fn generation_transition_allowed(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        (
            "draft",
            "blocked_capability" | "awaiting_approval" | "cancelled"
        ) | ("blocked_capability", "awaiting_approval" | "cancelled")
            | (
                "awaiting_approval",
                "approved" | "blocked_capability" | "cancelled"
            )
            | ("approved", "queued" | "submitted" | "cancelled")
            | (
                "queued",
                "submitted" | "processing" | "failed" | "timed_out" | "cancelled"
            )
            | (
                "submitted",
                "processing" | "completed" | "failed" | "timed_out" | "cancelled"
            )
            | (
                "processing",
                "completed" | "failed" | "timed_out" | "cancelled"
            )
            | ("failed" | "timed_out", "awaiting_approval")
    )
}

fn transition_generation_job(
    job: &mut GenerationJob,
    next_status: &str,
    progress_percent: u8,
    actor: &str,
    note: Option<String>,
    timestamp: u64,
) -> Result<(), String> {
    if !generation_transition_allowed(&job.status, next_status) {
        return Err(format!(
            "Generation job cannot transition from {} to {next_status}.",
            job.status
        ));
    }
    if progress_percent > 100 {
        return Err("Generation progress must be between 0 and 100.".into());
    }
    if next_status != "awaiting_approval" && progress_percent < job.progress_percent {
        return Err("Generation progress cannot move backwards within one attempt.".into());
    }
    let previous = job.status.clone();
    job.status = next_status.into();
    job.progress_percent = if next_status == "completed" {
        100
    } else {
        progress_percent
    };
    job.updated_at = timestamp;
    job.last_heartbeat_at = ["queued", "submitted", "processing"]
        .contains(&next_status)
        .then_some(timestamp);
    if next_status == "queued" {
        job.timeout_at = Some(timestamp.saturating_add(600));
    }
    if matches!(next_status, "submitted" | "processing") {
        job.timeout_at = Some(timestamp.saturating_add(1_800));
    }
    if next_status == "submitted" {
        job.submitted_at = Some(timestamp);
    }
    if next_status == "completed" {
        job.completed_at = Some(timestamp);
        job.timeout_at = None;
        job.retry_allowed = false;
    }
    if next_status == "timed_out" {
        job.timeout_at = None;
        job.retry_allowed = true;
    }
    if next_status == "failed" {
        job.timeout_at = None;
    }
    if next_status == "cancelled" {
        job.timeout_at = None;
        job.retry_allowed = false;
    }
    job.state_events.push(GenerationStateEvent {
        id: format!(
            "job-event-{}-{timestamp}-{}",
            job.id,
            job.state_events.len() + 1
        ),
        from_status: previous,
        to_status: next_status.into(),
        progress_percent: job.progress_percent,
        actor: actor.into(),
        note,
        created_at: timestamp,
    });
    Ok(())
}

fn validate_generation_package(
    manifest: &ProjectManifest,
    job: &GenerationJob,
) -> Result<(), String> {
    if job.approval_fingerprint.trim().is_empty() {
        return Err("Generation package fingerprint is unavailable.".into());
    }
    let unit = manifest
        .units
        .iter()
        .find(|unit| unit.id == job.unit_id)
        .ok_or_else(|| "Generation package unit is unavailable.".to_string())?;
    let current_topology = unit
        .room_graph_versions
        .last()
        .map(|version| version.fingerprint.as_str())
        .unwrap_or("unavailable");
    if current_topology != job.topology_fingerprint || !unit.room_graph_locked {
        return Err(
            "Room topology changed after package creation; create a new approval package.".into(),
        );
    }
    for input in &job.inputs {
        let asset = manifest
            .assets
            .iter()
            .find(|asset| asset.id == input.asset_id)
            .ok_or_else(|| format!("Generation input {} is unavailable.", input.asset_id))?;
        if asset.status != "accepted" || asset.checksum_sha256 != input.checksum_sha256 {
            return Err(format!(
                "Generation input {} is no longer accepted or unchanged.",
                input.asset_id
            ));
        }
    }
    Ok(())
}

fn refresh_generation_approval_identity(
    project_id: &str,
    job: &mut GenerationJob,
) -> Result<(), String> {
    let payload = serde_json::json!({
        "projectId": project_id,
        "jobId": job.id,
        "unitId": job.unit_id,
        "roomId": job.room_id,
        "assetRole": job.asset_role,
        "outputCount": job.output_count,
        "inputs": job.inputs,
        "cameraIntent": job.camera_intent,
        "requiredOpeningIds": job.required_opening_ids,
        "fixedFixtures": job.fixed_fixtures,
        "fixedFixturesStatus": job.fixed_fixtures_status,
        "prohibitions": job.prohibitions,
        "modelId": job.model_id,
        "capabilityStatus": job.capability_status,
        "requestedWidth": job.requested_width,
        "requestedHeight": job.requested_height,
        "panoramaMode": job.panorama_mode,
        "providerChoice": job.provider_choice,
        "priceStatus": job.price_status,
        "priceAmountMinor": job.price_amount_minor,
        "priceCurrency": job.price_currency,
        "customerCreditCost": job.customer_credit_cost,
        "topologyFingerprint": job.topology_fingerprint,
        "attempt": job.attempt,
    });
    job.approval_fingerprint = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&payload).map_err(|error| error.to_string())?)
    );
    job.idempotency_key = format!(
        "{:x}",
        Sha256::digest(format!(
            "generation:{}:{}:{}",
            job.id, job.approval_fingerprint, job.attempt
        ))
    );
    job.approval_state = "not_approved".into();
    job.approved_at = None;
    Ok(())
}

fn recover_timed_out_jobs(manifest: &mut ProjectManifest, timestamp: u64) -> Result<u32, String> {
    let mut recovered = 0;
    for job in &mut manifest.generation_jobs {
        if ["queued", "submitted", "processing"].contains(&job.status.as_str())
            && job.timeout_at.is_some_and(|timeout| timeout <= timestamp)
        {
            job.failure_code = Some("timeout".into());
            job.failure_reason =
                Some("No provider heartbeat arrived before the persisted timeout.".into());
            let progress = job.progress_percent;
            transition_generation_job(
                job,
                "timed_out",
                progress,
                "restart_recovery",
                Some(
                    "Recovered the existing task as timed out; no replacement task was submitted."
                        .into(),
                ),
                timestamp,
            )?;
            recovered += 1;
        }
    }
    Ok(recovered)
}

fn write_source_register(root: &Path, assets: &[AssetRecord]) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(assets)
        .map_err(|error| format!("Unable to encode source register: {error}"))?;
    fs::write(root.join("sources/source-register.json"), bytes)
        .map_err(|error| format!("Unable to save source register: {error}"))
}

fn create_project_directories(root: &Path, unit_ids: &[String]) -> Result<(), String> {
    for path in [
        "sources/drawings",
        "sources/renders",
        "sources/photos",
        "sources/panoramas",
        "sources/videos",
        "sources/brand",
        "sources/copy",
        "creative/campaigns",
        "creative/design-system",
        "creative/posters",
        "creative/sales-sheets",
        "creative/social",
        "sites/tour",
        "sites/sales-pages",
        "builds",
        "releases",
        "reports",
        "cache",
    ] {
        fs::create_dir_all(root.join(path))
            .map_err(|error| format!("Unable to create {path}: {error}"))?;
    }
    for unit_id in unit_ids {
        let unit_id = safe_path_component(unit_id, "Unit ID")?;
        let unit_root = root.join("units").join(unit_id);
        for path in [
            "floorplan",
            "assets/accepted",
            "assets/generated",
            "assets/rejected",
            "assets/panoramas",
            "assets/videos",
            "qa",
        ] {
            fs::create_dir_all(unit_root.join(path))
                .map_err(|error| format!("Unable to create unit folder: {error}"))?;
        }
    }
    Ok(())
}

fn structure_only_duplicate_manifest(
    source: &ProjectManifest,
    name: String,
    company: String,
    timestamp: u64,
) -> ProjectManifest {
    let unit_count = source.units.len();
    let project_id = format!("{}-{timestamp}", slugify(&name));
    ProjectManifest {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id,
        name,
        company: company.clone(),
        company_profile: CompanyProfileSnapshot {
            id: slugify(&company),
            name: company.clone(),
        },
        location: String::new(),
        locale: source.locale.clone(),
        measurement_units: source.measurement_units.clone(),
        access_mode: default_access_mode(),
        disclosure: default_disclosure(),
        workflow_mode: default_workflow_mode(),
        status: "New project · Intake required".into(),
        readiness: 12,
        read_only: false,
        archived_at: None,
        created_at: timestamp,
        updated_at: timestamp,
        units: (1..=unit_count)
            .map(|index| UnitRecord {
                id: format!("unit-type-{index}"),
                label: format!("Unit type {index}"),
                summary: "Details not supplied".into(),
                status: "Awaiting floor plan".into(),
                tour_available: false,
                floorplan_asset_id: None,
                rooms: vec![],
                room_graph_locked: false,
                entrance_room_id: None,
                openings: vec![],
                floorplan_version: 0,
                floorplan_versions: vec![],
                room_graph_version: 0,
                room_graph_versions: vec![],
            })
            .collect(),
        assets: vec![],
        generation_jobs: vec![],
        creative_jobs: vec![],
        enabled_skill_ids: vec![],
        releases: vec![],
        approval_events: vec![],
        qa_records: vec![],
        usage_ledger: vec![],
        background_jobs: vec![],
        analytics_events: vec![],
        design_spec: default_project_design_spec(),
        brand: Brand {
            primary: "#20241f".into(),
            accent: "#78917b".into(),
        },
        modules: ProjectModules {
            tour: "not_started".into(),
            creative: "not_started".into(),
            deployment: "not_started".into(),
        },
        tour_preview_url: None,
    }
}

fn koya_manifest() -> ProjectManifest {
    let parsed: ProjectManifest = serde_json::from_str(include_str!(
        "../../resources/examples/koya-example-v1/project.json"
    ))
    .expect("bundled Koya example manifest must be valid");
    normalize_manifest(parsed)
}

const KOYA_LIVING_VIDEO_PATH: &str = "sources/videos/living-terrace-reference-scroll.mp4";
const KOYA_LIVING_VIDEO: &[u8] = include_bytes!(
    "../../resources/examples/koya-example-v1/assets/videos/living-terrace-reference-scroll.mp4"
);

#[cfg(test)]
#[allow(dead_code)]
fn legacy_koya_manifest_fixture() -> ProjectManifest {
    let timestamp = 1_788_131_200;
    ProjectManifest {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id: "koya-example".into(),
        name: "Koya".into(),
        company: "Koya".into(),
        company_profile: CompanyProfileSnapshot {
            id: "koya".into(),
            name: "Koya".into(),
        },
        location: "Australia".into(),
        locale: "en-AU".into(),
        measurement_units: "metric".into(),
        access_mode: "public".into(),
        disclosure: "Off-the-plan concept experience. Plans, areas, finishes and imagery remain subject to approved project documentation and contract disclosure.".into(),
        workflow_mode: "advanced".into(),
        status: "Example · Tour ready".into(),
        readiness: 86,
        read_only: true,
        archived_at: None,
        created_at: timestamp,
        updated_at: timestamp,
        units: vec![
            UnitRecord {
                id: "102".into(),
                label: "Apartment 102".into(),
                summary: "2 bed · 2 bath · 104 m²".into(),
                status: "Floor plan only".into(),
                tour_available: false,
                floorplan_asset_id: Some("reference-floorplan-102".into()),
                rooms: vec![],
                room_graph_locked: false,
                entrance_room_id: None,
                openings: vec![],
                floorplan_version: 1,
                floorplan_versions: vec![],
                room_graph_version: 0,
                room_graph_versions: vec![],
            },
            UnitRecord {
                id: "103".into(),
                label: "Apartment 103".into(),
                summary: "1 bed · study · 86 m²".into(),
                status: "Floor plan only".into(),
                tour_available: false,
                floorplan_asset_id: Some("reference-floorplan-103".into()),
                rooms: vec![],
                room_graph_locked: false,
                entrance_room_id: None,
                openings: vec![],
                floorplan_version: 1,
                floorplan_versions: vec![],
                room_graph_version: 0,
                room_graph_versions: vec![],
            },
            UnitRecord {
                id: "104".into(),
                label: "Apartment 104".into(),
                summary: "2 bed · 2 bath · 106 m²".into(),
                status: "Floor plan only".into(),
                tour_available: false,
                floorplan_asset_id: Some("reference-floorplan-104".into()),
                rooms: vec![],
                room_graph_locked: false,
                entrance_room_id: None,
                openings: vec![],
                floorplan_version: 1,
                floorplan_versions: vec![],
                room_graph_version: 0,
                room_graph_versions: vec![],
            },
            UnitRecord {
                id: "106".into(),
                label: "Apartment 106".into(),
                summary: "2 bed · 2 bath · MPR · 152 m²".into(),
                status: "Interactive tour ready".into(),
                tour_available: true,
                floorplan_asset_id: Some("reference-floorplan-106".into()),
                rooms: [
                    (
                        "Living room",
                        72.0,
                        68.0,
                        vec!["kitchen-dining", "private-terrace"],
                    ),
                    (
                        "Kitchen & dining",
                        57.0,
                        56.0,
                        vec!["living-room", "multipurpose-room", "bedroom-1"],
                    ),
                    ("Bedroom 1", 78.0, 33.0, vec!["kitchen-dining"]),
                    ("Bedroom 2", 28.0, 68.0, vec!["multipurpose-room"]),
                    (
                        "Multipurpose room",
                        42.0,
                        50.0,
                        vec!["kitchen-dining", "bedroom-2"],
                    ),
                    ("Private terrace", 91.0, 68.0, vec!["living-room"]),
                ]
                .into_iter()
                .map(|(name, x, y, adjacent)| RoomRecord {
                    id: slugify(name),
                    name: name.into(),
                    status: "approved".into(),
                    panorama_status: "ready".into(),
                    identity_asset_id: None,
                    hotspot_x: Some(x),
                    hotspot_y: Some(y),
                    adjacent_room_ids: adjacent.into_iter().map(str::to_string).collect(),
                    panorama_asset_id: None,
                    threshold_asset_id: None,
                    reverse_asset_id: None,
                    still_fallback_asset_id: None,
                    video_asset_id: None,
                    poster_asset_id: None,
                })
                .collect(),
                room_graph_locked: true,
                entrance_room_id: Some("entry".into()),
                openings: vec![],
                floorplan_version: 1,
                floorplan_versions: vec![],
                room_graph_version: 1,
                room_graph_versions: vec![],
            },
        ],
        assets: vec![],
        generation_jobs: vec![],
        creative_jobs: vec![
            CreativeBrief {
                id: "koya-apartment-106-poster".into(),
                kind: "poster".into(),
                poster_mode: Some("a4_sales_sheet".into()),
                a4_document_type: Some("unit_sales_sheet".into()),
                title: "Apartment 106 A4 sales sheet".into(),
                campaign_name: "Apartment 106 launch".into(),
                unit_id: Some("106".into()),
                audience: "Prospective owner-occupiers".into(),
                objective: "Invite buyers to explore the interactive apartment tour".into(),
                headline: "Apartment 106".into(),
                subheadline: "2 bed · 2 bath · MPR · 152 m²".into(),
                body: "A calm, project-branded introduction to the completed interactive experience.".into(),
                call_to_action: "Explore the interactive tour".into(),
                project_highlights: vec![],
                format: "a4_portrait".into(),
                width: 2480,
                height: 3508,
                duration_seconds: None,
                visual_direction: "Editorial typography, warm stone palette and generous whitespace.".into(),
                image_prompt: None,
                generation_provider: None,
                price_status: None,
                evidence_asset_ids: vec![],
                scenes: vec![],
                warnings: vec!["Example uses manifest facts only; final legal copy requires client approval.".into()],
                status: "preview_ready".into(),
                workflow: "deterministic_svg".into(),
                output_relative_path: Some("creative/sales-sheets/koya-apartment-106-poster/document.md".into()),
                created_at: timestamp,
                updated_at: timestamp,
            },
            CreativeBrief {
                id: "koya-apartment-106-ai-poster".into(),
                kind: "poster".into(),
                poster_mode: Some("ai_creative".into()),
                a4_document_type: None,
                title: "Apartment 106 AI creative poster".into(),
                campaign_name: "Apartment 106 launch".into(),
                unit_id: Some("106".into()),
                audience: "Design-conscious property buyers".into(),
                objective: "Create an expressive campaign visual that leads into the interactive tour".into(),
                headline: "Space to live your way".into(),
                subheadline: "Apartment 106 · 2 bed · 2 bath · MPR · 152 m²".into(),
                body: "A high-impact campaign concept grounded in the approved Apartment 106 experience.".into(),
                call_to_action: "Explore Apartment 106".into(),
                project_highlights: vec![
                    "2 bed · 2 bath".into(),
                    "Multipurpose room".into(),
                    "152 m² total".into(),
                    "Interactive tour ready".into(),
                ],
                format: "instagram_portrait".into(),
                width: 1080,
                height: 1350,
                duration_seconds: None,
                visual_direction: "Editorial architectural campaign image with warm stone tones and clean negative space for brand copy.".into(),
                image_prompt: Some("Use case: ads-marketing\nAsset type: property campaign poster main visual\nPrimary request: an editorial architectural image inspired only by approved Apartment 106 references\nComposition/framing: portrait composition with generous negative space for a separate typographic overlay\nConstraints: no text, no logo, no invented rooms, views, finishes, amenities or people; no watermark".into()),
                generation_provider: Some("openai_image_model".into()),
                price_status: Some("unavailable".into()),
                evidence_asset_ids: vec![],
                scenes: vec![],
                warnings: vec!["Example request package only; no Image Model output has been generated.".into()],
                status: "awaiting_generation".into(),
                workflow: "image_model".into(),
                output_relative_path: Some("creative/campaigns/koya-apartment-106-ai-poster/image-generation-request.json".into()),
                created_at: timestamp,
                updated_at: timestamp,
            },
            CreativeBrief {
                id: "koya-apartment-106-social-video".into(),
                kind: "video".into(),
                poster_mode: None,
                a4_document_type: None,
                title: "Apartment 106 social walkthrough".into(),
                campaign_name: "Apartment 106 launch".into(),
                unit_id: Some("106".into()),
                audience: "Mobile property buyers".into(),
                objective: "Move from project identity into the interactive apartment experience".into(),
                headline: "Step inside Apartment 106".into(),
                subheadline: "A 15-second vertical walkthrough concept".into(),
                body: "A restrained three-scene property story grounded in the approved unit structure.".into(),
                call_to_action: "Explore every room".into(),
                project_highlights: vec![],
                format: "social_vertical".into(),
                width: 1080,
                height: 1920,
                duration_seconds: Some(15),
                visual_direction: "Slow architectural movement, minimal overlays and no invented views.".into(),
                image_prompt: None,
                generation_provider: None,
                price_status: None,
                evidence_asset_ids: vec![],
                scenes: vec![
                    CreativeScene { order: 1, duration_seconds: 4, title: "Arrival".into(), visual_direction: "Project identity and Apartment 106 title card.".into(), on_screen_text: "Apartment 106".into() },
                    CreativeScene { order: 2, duration_seconds: 7, title: "Explore".into(), visual_direction: "Move through the approved interactive room sequence.".into(), on_screen_text: "2 bed · 2 bath · MPR".into() },
                    CreativeScene { order: 3, duration_seconds: 4, title: "Invitation".into(), visual_direction: "End on the tour call-to-action and project mark.".into(), on_screen_text: "Explore every room".into() },
                ],
                warnings: vec!["Storyboard package only; no MP4 or paid provider task has run.".into()],
                status: "package_ready".into(),
                workflow: "hyperframes".into(),
                output_relative_path: Some("creative/campaigns/koya-apartment-106-social-video/BRIEF.md".into()),
                created_at: timestamp,
                updated_at: timestamp,
            },
        ],
        enabled_skill_ids: vec![],
        releases: vec![ReleaseRecord {
            id: "koya-public-v1".into(),
            version: 1,
            status: "public_verified".into(),
            access_mode: "public".into(),
            created_at: timestamp,
            unit_ids: vec!["106".into()],
            public_url: Some(
                "https://hades217.github.io/Koya/?unit=106&mode=video&tour=entry-room".into(),
            ),
            verified_at: Some(timestamp),
            supersedes_release_id: None,
            superseded_by_release_id: None,
            rollback_from_release_id: None,
            unit_share_links: vec![],
            verification_qa_id: None,
        }],
        approval_events: vec![],
        qa_records: vec![],
        usage_ledger: vec![],
        background_jobs: vec![],
        analytics_events: vec![],
        design_spec: ProjectDesignSpec {
            status: "approved".into(),
            direction: "Quiet editorial property identity translated from the Koya interactive website: warm ivory fields, near-black architectural ink, restrained terracotta accent, serif display typography and generous negative space.".into(),
            source_basis: vec!["Koya interactive tour website".into(), "Koya approved project palette".into(), "Apartment 106 interactive experience".into()],
            colours: ProjectDesignColours { paper: "#f3efe5".into(), ink: "#241f1a".into(), accent: "#a96d45".into(), muted: "#817d76".into(), botanical: "#5f7567".into() },
            typography: ProjectDesignTypography { display: "Didot, \"Bodoni 72\", Georgia, serif".into(), body: "\"Avenir Next\", \"Helvetica Neue\", Arial, sans-serif".into() },
            layout: ProjectDesignLayout { page: "A4 portrait".into(), margin_mm: 18, grid: "Editorial asymmetric grid; large display title; thin rules; compact uppercase metadata".into(), image_treatment: "Use approved project imagery at natural colour with a neutral dark scrim only when text contrast requires it".into() },
        },
        brand: Brand {
            primary: "#2f241d".into(),
            accent: "#b66d45".into(),
        },
        modules: ProjectModules {
            tour: "ready".into(),
            creative: "in_progress".into(),
            deployment: "ready".into(),
        },
        tour_preview_url: Some(
            "/embedded-tour/index.html?unit=106&mode=video&tour=entry-room".into(),
        ),
    }
}

fn ensure_koya_example(root: &Path) -> Result<(), String> {
    let project_root = root.join("koya-example");
    let manifest_path = project_root.join("project.json");
    if !manifest_path.exists() {
        create_project_directories(
            &project_root,
            &["102".into(), "103".into(), "104".into(), "106".into()],
        )?;
    }

    let video_path = project_root.join(KOYA_LIVING_VIDEO_PATH);
    if let Some(parent) = video_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to create Koya example video directory: {error}"))?;
    }
    fs::write(&video_path, KOYA_LIVING_VIDEO)
        .map_err(|error| format!("Unable to install Koya example living-room video: {error}"))?;

    write_manifest(&project_root, &koya_manifest())
}

#[tauri::command]
fn list_projects(app: AppHandle) -> Result<Vec<ProjectRecord>, String> {
    let root = projects_root(&app)?;
    ensure_koya_example(&root)?;
    let mut records = Vec::new();
    for entry in
        fs::read_dir(&root).map_err(|error| format!("Unable to read project library: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("Unable to read project entry: {error}"))?
            .path();
        let manifest_path = path.join("project.json");
        if !manifest_path.is_file() {
            continue;
        }
        let manifest = read_manifest(&path)?;
        records.push(ProjectRecord {
            manifest,
            project_root: path.to_string_lossy().into_owned(),
        });
    }
    records.sort_by(|left, right| {
        right
            .manifest
            .read_only
            .cmp(&left.manifest.read_only)
            .then_with(|| right.manifest.updated_at.cmp(&left.manifest.updated_at))
    });
    Ok(records)
}

#[tauri::command]
fn create_project(app: AppHandle, input: CreateProjectInput) -> Result<ProjectRecord, String> {
    let name = clean_required(&input.name, "Development name")?;
    let company = clean_required(&input.company, "Customer company")?;
    let location = input.location.trim().to_string();
    let unit_ids: Vec<String> = input
        .unit_ids
        .iter()
        .map(|unit| slugify(unit))
        .filter(|unit| !unit.is_empty())
        .collect();
    let timestamp = unix_time();
    let project_id = format!("{}-{timestamp}", slugify(&name));
    let root = projects_root(&app)?.join(&project_id);
    if root.exists() {
        return Err("A project with this identifier already exists".into());
    }
    for skill_id in DEFAULT_PROJECT_SKILL_IDS {
        ensure_bundled_skill_installed(&app, skill_id)?;
    }
    create_project_directories(&root, &unit_ids)?;
    let mut manifest = ProjectManifest {
        schema_version: CURRENT_SCHEMA_VERSION,
        project_id,
        name,
        company: company.clone(),
        company_profile: CompanyProfileSnapshot {
            id: slugify(&company),
            name: company,
        },
        location,
        locale: default_locale(),
        measurement_units: default_measurement_units(),
        access_mode: default_access_mode(),
        disclosure: default_disclosure(),
        workflow_mode: default_workflow_mode(),
        status: "New project · Intake required".into(),
        readiness: 12,
        read_only: false,
        archived_at: None,
        created_at: timestamp,
        updated_at: timestamp,
        units: unit_ids
            .iter()
            .map(|id| UnitRecord {
                id: id.clone(),
                label: format!("Unit {id}"),
                summary: "Details not supplied".into(),
                status: "Awaiting floor plan".into(),
                tour_available: false,
                floorplan_asset_id: None,
                rooms: vec![],
                room_graph_locked: false,
                entrance_room_id: None,
                openings: vec![],
                floorplan_version: 0,
                floorplan_versions: vec![],
                room_graph_version: 0,
                room_graph_versions: vec![],
            })
            .collect(),
        assets: vec![],
        generation_jobs: vec![],
        creative_jobs: vec![],
        enabled_skill_ids: DEFAULT_PROJECT_SKILL_IDS
            .iter()
            .map(|skill_id| (*skill_id).to_string())
            .collect(),
        releases: vec![],
        approval_events: vec![],
        qa_records: vec![],
        usage_ledger: vec![],
        background_jobs: vec![],
        analytics_events: vec![],
        design_spec: default_project_design_spec(),
        brand: Brand {
            primary: "#20241f".into(),
            accent: "#78917b".into(),
        },
        modules: ProjectModules {
            tour: "not_started".into(),
            creative: "not_started".into(),
            deployment: "not_started".into(),
        },
        tour_preview_url: None,
    };
    record_funnel_event(
        &mut manifest,
        "project_created",
        "desktop_workflow",
        timestamp,
    )?;
    if !manifest.units.is_empty() {
        record_funnel_event(
            &mut manifest,
            "first_unit_added",
            "desktop_workflow",
            timestamp,
        )?;
    }
    register_company_profile(
        &app,
        &manifest.company,
        &manifest.locale,
        &manifest.measurement_units,
        &manifest.brand,
    )?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn duplicate_project_structure(
    app: AppHandle,
    project_id: String,
    input: DuplicateProjectInput,
) -> Result<ProjectRecord, String> {
    validate_project_id(&project_id)?;
    let projects = projects_root(&app)?;
    let source_root = projects.join(&project_id);
    let source = read_manifest(&source_root)?;
    let name = clean_required(&input.name, "Development name")?;
    let company = clean_required(&input.company, "Customer company")?;
    let timestamp = unix_time();
    let mut manifest = structure_only_duplicate_manifest(&source, name, company, timestamp);
    record_funnel_event(
        &mut manifest,
        "project_created",
        "structure_duplicate",
        timestamp,
    )?;
    if !manifest.units.is_empty() {
        record_funnel_event(
            &mut manifest,
            "first_unit_added",
            "structure_duplicate",
            timestamp,
        )?;
    }
    let root = projects.join(&manifest.project_id);
    if root.exists() {
        return Err("A project with this identifier already exists".into());
    }
    for skill_id in DEFAULT_PROJECT_SKILL_IDS {
        ensure_bundled_skill_installed(&app, skill_id)?;
    }
    manifest.enabled_skill_ids = DEFAULT_PROJECT_SKILL_IDS
        .iter()
        .map(|skill_id| (*skill_id).to_string())
        .collect();
    let unit_ids = manifest
        .units
        .iter()
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    create_project_directories(&root, &unit_ids)?;
    register_company_profile(
        &app,
        &manifest.company,
        &manifest.locale,
        &manifest.measurement_units,
        &manifest.brand,
    )?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn add_unit(
    app: AppHandle,
    project_id: String,
    input: AddUnitInput,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let unit_id = slugify(&clean_required(&input.unit_id, "Unit ID")?);
    if manifest.units.iter().any(|unit| unit.id == unit_id) {
        return Err("This unit type already exists".into());
    }
    create_project_directories(&root, std::slice::from_ref(&unit_id))?;
    manifest.units.push(UnitRecord {
        id: unit_id.clone(),
        label: if input.label.trim().is_empty() {
            format!("Unit {unit_id}")
        } else {
            input.label.trim().to_string()
        },
        summary: if input.summary.trim().is_empty() {
            "Details not supplied".into()
        } else {
            input.summary.trim().to_string()
        },
        status: "Awaiting floor plan".into(),
        tour_available: false,
        floorplan_asset_id: None,
        rooms: vec![],
        room_graph_locked: false,
        entrance_room_id: None,
        openings: vec![],
        floorplan_version: 0,
        floorplan_versions: vec![],
        room_graph_version: 0,
        room_graph_versions: vec![],
    });
    let timestamp = unix_time();
    manifest.updated_at = timestamp;
    record_funnel_event(
        &mut manifest,
        "first_unit_added",
        "desktop_workflow",
        timestamp,
    )?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn update_project(
    app: AppHandle,
    project_id: String,
    input: UpdateProjectInput,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    ensure_current_project(input.expected_updated_at, manifest.updated_at)?;
    manifest.name = clean_required(&input.name, "Development name")?;
    manifest.company = clean_required(&input.company, "Customer company")?;
    manifest.company_profile = CompanyProfileSnapshot {
        id: slugify(&manifest.company),
        name: manifest.company.clone(),
    };
    manifest.location = clean_optional(&input.location, "Location")?;
    manifest.brand.primary = clean_hex_colour(&input.primary, "Primary colour")?;
    manifest.brand.accent = clean_hex_colour(&input.accent, "Accent colour")?;
    manifest.updated_at = unix_time();
    register_company_profile(
        &app,
        &manifest.company,
        &manifest.locale,
        &manifest.measurement_units,
        &manifest.brand,
    )?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn set_project_archived(
    app: AppHandle,
    project_id: String,
    archived: bool,
) -> Result<ProjectRecord, String> {
    validate_project_id(&project_id)?;
    let root = projects_root(&app)?.join(&project_id);
    if !root.is_dir() {
        return Err("Project folder is unavailable".into());
    }
    let mut manifest = read_manifest(&root)?;
    if manifest.read_only {
        return Err("The example project cannot be archived".into());
    }
    manifest.archived_at = archived.then(unix_time);
    manifest.updated_at = unix_time();
    manifest.status = if archived {
        "Archived · Read-only until restored".into()
    } else if manifest.assets.is_empty() {
        "New project · Intake required".into()
    } else {
        "Intake in progress · Sources need review".into()
    };
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn set_workflow_mode(
    app: AppHandle,
    project_id: String,
    mode: String,
) -> Result<ProjectRecord, String> {
    if !matches!(mode.as_str(), "standard" | "advanced") {
        return Err("Workflow mode must be standard or advanced.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    manifest.workflow_mode = mode;
    manifest.updated_at = unix_time();
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn add_room(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    name: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let room_name = clean_required(&name, "Room name")?;
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Selected unit type is unavailable".to_string())?;
    let base_id = slugify(&room_name);
    let mut room_id = base_id.clone();
    let mut suffix = 2;
    while unit.rooms.iter().any(|room| room.id == room_id) {
        room_id = format!("{base_id}-{suffix}");
        suffix += 1;
    }
    unit.rooms.push(RoomRecord {
        id: room_id,
        name: room_name,
        status: "needs_evidence".into(),
        panorama_status: "not_started".into(),
        identity_asset_id: None,
        hotspot_x: None,
        hotspot_y: None,
        adjacent_room_ids: vec![],
        panorama_asset_id: None,
        threshold_asset_id: None,
        reverse_asset_id: None,
        still_fallback_asset_id: None,
        video_asset_id: None,
        poster_asset_id: None,
    });
    unit.status = if unit.floorplan_asset_id.is_some() {
        "Room audit in progress".into()
    } else {
        "Rooms drafted · Floor plan required".into()
    };
    manifest.updated_at = unix_time();
    update_readiness(&mut manifest);
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn save_room_graph(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    nodes: Vec<RoomGraphNodeInput>,
    entrance_room_id: Option<String>,
    openings: Vec<OpeningInput>,
    locked: bool,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Selected unit type is unavailable".to_string())?;
    if unit.floorplan_asset_id.is_none() {
        return Err("Import a unit floor plan before positioning room hotspots.".into());
    }
    if nodes.len() != unit.rooms.len()
        || unit
            .rooms
            .iter()
            .any(|room| !nodes.iter().any(|node| node.room_id == room.id))
    {
        return Err("Every confirmed room must have exactly one hotspot.".into());
    }
    let room_ids = unit
        .rooms
        .iter()
        .map(|room| room.id.clone())
        .collect::<Vec<_>>();
    let fingerprint = room_graph_fingerprint(&nodes, &entrance_room_id, &openings)?;
    let previous_fingerprint = unit
        .room_graph_versions
        .last()
        .map(|version| version.fingerprint.clone())
        .or_else(|| {
            if unit.room_graph_version == 0 {
                return None;
            }
            let existing_nodes = unit
                .rooms
                .iter()
                .map(|room| RoomGraphNodeInput {
                    room_id: room.id.clone(),
                    hotspot_x: room.hotspot_x.unwrap_or_default(),
                    hotspot_y: room.hotspot_y.unwrap_or_default(),
                    adjacent_room_ids: room.adjacent_room_ids.clone(),
                })
                .collect::<Vec<_>>();
            let existing_openings = unit
                .openings
                .iter()
                .map(|opening| OpeningInput {
                    id: opening.id.clone(),
                    from_room_id: opening.from_room_id.clone(),
                    to_room_id: opening.to_room_id.clone(),
                    kind: opening.kind.clone(),
                    x: opening.x,
                    y: opening.y,
                })
                .collect::<Vec<_>>();
            room_graph_fingerprint(&existing_nodes, &unit.entrance_room_id, &existing_openings).ok()
        });
    let was_locked = unit.room_graph_locked;
    let topology_changed = previous_fingerprint
        .as_ref()
        .is_some_and(|previous| previous != &fingerprint);
    let invalidated_dependents =
        was_locked && topology_changed && invalidate_topology_dependents(unit);
    if entrance_room_id
        .as_ref()
        .is_some_and(|room_id| !room_ids.contains(room_id))
    {
        return Err("The entrance must reference a confirmed room stop.".into());
    }
    let mut opening_ids = Vec::new();
    let mut opening_routes = Vec::new();
    for opening in &openings {
        if opening.id.trim().is_empty() || opening_ids.contains(&opening.id) {
            return Err("Every opening must have a unique non-empty ID.".into());
        }
        if !matches!(opening.kind.as_str(), "entrance" | "door" | "opening") {
            return Err("Opening kind must be entrance, door, or opening.".into());
        }
        if (opening.kind == "entrance" && opening.to_room_id.is_some())
            || (opening.kind != "entrance" && opening.to_room_id.is_none())
        {
            return Err(
                "Entrances have one room; doors and openings must connect two rooms.".into(),
            );
        }
        if !room_ids.contains(&opening.from_room_id)
            || opening.to_room_id.as_ref().is_some_and(|room_id| {
                !room_ids.contains(room_id) || room_id == &opening.from_room_id
            })
        {
            return Err("Openings must reference valid, distinct room stops.".into());
        }
        if !opening.x.is_finite()
            || !opening.y.is_finite()
            || !(0.0..=100.0).contains(&opening.x)
            || !(0.0..=100.0).contains(&opening.y)
        {
            return Err("Opening coordinates must stay inside the floor-plan boundary.".into());
        }
        let mut route = vec![opening.from_room_id.clone()];
        if let Some(to_room_id) = &opening.to_room_id {
            let connected = nodes
                .iter()
                .find(|node| node.room_id == opening.from_room_id)
                .is_some_and(|node| node.adjacent_room_ids.contains(to_room_id));
            if !connected {
                return Err("An opening cannot create a route between non-adjacent rooms.".into());
            }
            route.push(to_room_id.clone());
            route.sort();
        }
        let route_key = format!("{}:{}", opening.kind, route.join("->"));
        if opening_routes.contains(&route_key) {
            return Err("Duplicate opening routes are not allowed.".into());
        }
        opening_ids.push(opening.id.clone());
        opening_routes.push(route_key);
    }
    for node in &nodes {
        if !node.hotspot_x.is_finite()
            || !node.hotspot_y.is_finite()
            || !(0.0..=100.0).contains(&node.hotspot_x)
            || !(0.0..=100.0).contains(&node.hotspot_y)
        {
            return Err("Room hotspots must stay inside the floor-plan boundary.".into());
        }
        if node
            .adjacent_room_ids
            .iter()
            .any(|id| id == &node.room_id || !room_ids.contains(id))
        {
            return Err("Room adjacency contains an invalid or self-referencing stop.".into());
        }
        let unique_adjacent = node.adjacent_room_ids.iter().collect::<HashSet<_>>();
        if unique_adjacent.len() != node.adjacent_room_ids.len() {
            return Err("Duplicate room connections are not allowed.".into());
        }
        for adjacent_id in &node.adjacent_room_ids {
            let reverse = nodes
                .iter()
                .find(|candidate| &candidate.room_id == adjacent_id)
                .is_some_and(|candidate| candidate.adjacent_room_ids.contains(&node.room_id));
            if !reverse {
                return Err("Every room connection must be symmetrical before saving.".into());
            }
        }
    }
    for room in &mut unit.rooms {
        let node = nodes
            .iter()
            .find(|node| node.room_id == room.id)
            .expect("validated room graph node");
        room.hotspot_x = Some(node.hotspot_x);
        room.hotspot_y = Some(node.hotspot_y);
        room.adjacent_room_ids = node.adjacent_room_ids.clone();
    }
    if locked {
        validate_connected_room_graph(&unit.rooms)?;
        let entrance_id = entrance_room_id
            .as_ref()
            .ok_or_else(|| "Confirm one entrance room before locking the graph.".to_string())?;
        let entrances = openings
            .iter()
            .filter(|opening| opening.kind == "entrance")
            .collect::<Vec<_>>();
        if entrances.len() != 1 || entrances[0].from_room_id != *entrance_id {
            return Err("The locked graph requires exactly one entrance opening at the confirmed entrance room.".into());
        }
        for node in &nodes {
            for adjacent_id in &node.adjacent_room_ids {
                if node.room_id >= *adjacent_id {
                    continue;
                }
                let route_has_opening = openings.iter().any(|opening| {
                    opening.kind != "entrance"
                        && opening.to_room_id.as_ref().is_some_and(|to_room_id| {
                            (opening.from_room_id == node.room_id && to_room_id == adjacent_id)
                                || (opening.from_room_id == *adjacent_id
                                    && to_room_id == &node.room_id)
                        })
                });
                if !route_has_opening {
                    return Err(
                        "Every locked adjacency route requires a confirmed door or opening.".into(),
                    );
                }
            }
        }
    }
    unit.entrance_room_id = entrance_room_id;
    unit.openings = openings
        .into_iter()
        .map(|opening| OpeningRecord {
            id: opening.id,
            from_room_id: opening.from_room_id,
            to_room_id: opening.to_room_id,
            kind: opening.kind,
            x: opening.x,
            y: opening.y,
            status: if locked { "confirmed" } else { "draft" }.into(),
        })
        .collect();
    unit.room_graph_locked = locked;
    unit.status = if locked {
        "Room graph locked · Evidence required".into()
    } else {
        "Room graph draft".into()
    };
    let timestamp = unix_time();
    let last_lock_state = unit
        .room_graph_versions
        .last()
        .map(|version| version.locked);
    let version_changed =
        previous_fingerprint.as_ref() != Some(&fingerprint) || last_lock_state != Some(locked);
    if version_changed {
        unit.room_graph_version = unit.room_graph_version.saturating_add(1).max(1);
    }
    let graph_relative_path = format!(
        "units/{}/room-graphs/v{}.json",
        unit.id, unit.room_graph_version
    );
    if version_changed {
        unit.room_graph_versions.push(RoomGraphVersionRecord {
            version: unit.room_graph_version,
            fingerprint: fingerprint.clone(),
            locked,
            relative_path: graph_relative_path.clone(),
            created_at: timestamp,
        });
    }
    let graph_path = root.join("units").join(&unit.id).join("room-graph.json");
    let version_path = root.join(&graph_relative_path);
    if let Some(parent) = version_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to prepare graph version folder: {error}"))?;
    }
    if version_changed && version_path.exists() {
        return Err(
            "Room-graph version path already exists; refusing to overwrite immutable history."
                .into(),
        );
    }
    let graph = serde_json::json!({ "schemaVersion": 2, "unitId": unit.id, "version": unit.room_graph_version, "fingerprint": fingerprint, "locked": locked, "updatedAt": timestamp, "entranceRoomId": unit.entrance_room_id, "openings": unit.openings, "rooms": unit.rooms });
    let graph_bytes = serde_json::to_vec_pretty(&graph).map_err(|error| error.to_string())?;
    fs::write(&graph_path, &graph_bytes)
        .map_err(|error| format!("Unable to save room graph: {error}"))?;
    if version_changed {
        fs::write(&version_path, graph_bytes)
            .map_err(|error| format!("Unable to save room graph version: {error}"))?;
    }
    if invalidated_dependents {
        manifest.approval_events.push(ApprovalEvent {
            id: format!("approval-room-graph-invalidated-{timestamp}"),
            subject_type: "unit_topology".into(),
            subject_id: unit_id.clone(),
            decision: "invalidated".into(),
            actor: "system".into(),
            reason: Some("A locked room graph changed; dependent room and panorama approvals require review.".into()),
            created_at: timestamp,
        });
        manifest.qa_records.push(QaRecord {
            id: format!("qa-room-graph-invalidated-{timestamp}"),
            scope: format!("unit:{unit_id}:topology"),
            status: "failed".into(),
            checks: vec![
                "Locked room-graph fingerprint changed after dependent asset approval.".into(),
            ],
            reviewer: "system".into(),
            created_at: timestamp,
        });
    }
    manifest.updated_at = timestamp;
    update_readiness(&mut manifest);
    if locked {
        record_funnel_event(
            &mut manifest,
            "floorplan_locked",
            "desktop_workflow",
            timestamp,
        )?;
    }
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn assign_room_identity(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    room_id: String,
    asset_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let asset = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .ok_or_else(|| "Selected source asset is unavailable".to_string())?;
    if asset.status != "accepted" {
        return Err("Only an accepted source can become a room identity anchor".into());
    }
    if !["renders", "photos"].contains(&asset.category.as_str()) {
        return Err("Room identity anchors must be an accepted render or photo".into());
    }
    if asset
        .unit_id
        .as_deref()
        .is_some_and(|asset_unit| asset_unit != unit_id)
    {
        return Err("The selected source belongs to a different unit type".into());
    }
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Selected unit type is unavailable".to_string())?;
    let room = unit
        .rooms
        .iter_mut()
        .find(|room| room.id == room_id)
        .ok_or_else(|| "Selected room is unavailable".to_string())?;
    room.identity_asset_id = Some(asset_id);
    room.status = "ready_for_generation".into();
    manifest.updated_at = unix_time();
    update_readiness(&mut manifest);
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn assign_room_panorama(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    room_id: String,
    asset_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let asset = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .ok_or_else(|| "Selected panorama is unavailable".to_string())?;
    if asset.category != "panoramas" || asset.status != "accepted" {
        return Err("Only an accepted panorama source can be assigned to a room.".into());
    }
    if asset
        .unit_id
        .as_deref()
        .is_some_and(|asset_unit| asset_unit != unit_id)
    {
        return Err("The selected panorama belongs to a different unit type.".into());
    }
    let (width, height) = asset
        .width
        .zip(asset.height)
        .ok_or_else(|| "Panorama dimensions are unavailable.".to_string())?;
    if width < 4096 || width != height.saturating_mul(2) {
        return Err(format!("Panorama must be an exact 2:1 image at least 4096 px wide; this file is {width} × {height}."));
    }
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Selected unit type is unavailable".to_string())?;
    if !unit.room_graph_locked {
        return Err("Lock the room graph before assigning publishable panoramas.".into());
    }
    let room = unit
        .rooms
        .iter_mut()
        .find(|room| room.id == room_id)
        .ok_or_else(|| "Selected room is unavailable".to_string())?;
    room.panorama_asset_id = Some(asset_id);
    room.panorama_status = "ready".into();
    room.status = "approved".into();
    unit.tour_available = !unit.rooms.is_empty()
        && unit
            .rooms
            .iter()
            .all(|room| room.panorama_status == "ready");
    unit.status = if unit.tour_available {
        "Local tour ready".into()
    } else {
        "Panorama review in progress".into()
    };
    manifest.modules.tour = "in_progress".into();
    manifest.updated_at = unix_time();
    update_readiness(&mut manifest);
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn create_panorama_draft(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    room_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    if manifest.generation_jobs.iter().any(|job| {
        job.unit_id == unit_id
            && job.room_id == room_id
            && [
                "draft",
                "blocked_capability",
                "awaiting_approval",
                "approved",
                "queued",
                "submitted",
                "processing",
            ]
            .contains(&job.status.as_str())
    }) {
        return Err("An active panorama package already exists for this room".into());
    }
    let unit_snapshot = manifest
        .units
        .iter()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Selected unit type is unavailable".to_string())?;
    let room_snapshot = unit_snapshot
        .rooms
        .iter()
        .find(|room| room.id == room_id)
        .ok_or_else(|| "Selected room is unavailable".to_string())?;
    if !["ready_for_generation", "approved"].contains(&room_snapshot.status.as_str()) {
        return Err("Approve a room identity anchor before preparing panorama generation".into());
    }
    if !unit_snapshot.room_graph_locked {
        return Err("Lock the room graph before preparing a panorama package.".into());
    }
    let timestamp = unix_time();
    let room_name = room_snapshot.name.clone();
    let floorplan_asset_id = unit_snapshot.floorplan_asset_id.clone().ok_or_else(|| {
        "The generation package requires a floor-plan topology source.".to_string()
    })?;
    let identity_asset_id = room_snapshot.identity_asset_id.clone().ok_or_else(|| {
        "The generation package requires an accepted room identity anchor.".to_string()
    })?;
    let input_specs = [
        (floorplan_asset_id, "topology_source"),
        (identity_asset_id, "identity_anchor"),
    ];
    let inputs = input_specs
        .iter()
        .map(|(asset_id, role)| {
            let asset = manifest
                .assets
                .iter()
                .find(|asset| &asset.id == asset_id)
                .ok_or_else(|| format!("Required {role} asset is unavailable."))?;
            if asset.status != "accepted" {
                return Err(format!("Required {role} asset is not accepted."));
            }
            Ok(GenerationInputRecord {
                asset_id: asset.id.clone(),
                role: (*role).into(),
                checksum_sha256: asset.checksum_sha256.clone(),
                evidence_class: asset.evidence_class.clone(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    let required_opening_ids = unit_snapshot
        .openings
        .iter()
        .filter(|opening| {
            opening.from_room_id == room_id || opening.to_room_id.as_deref() == Some(&room_id)
        })
        .map(|opening| opening.id.clone())
        .collect::<Vec<_>>();
    let topology_fingerprint = unit_snapshot
        .room_graph_versions
        .last()
        .map(|version| version.fingerprint.clone())
        .unwrap_or_else(|| "unavailable".into());
    let camera_intent = "Stationary eye-level 360 panorama centred on the confirmed room hotspot; preserve every confirmed route opening.".to_string();
    let prohibitions = vec![
        "Do not add, remove, resize, or relocate rooms, doors, windows, openings, or fixed fixtures.".into(),
        "Do not invent dimensions, external views, finishes, amenities, people, text, logos, or watermarks.".into(),
        "Do not treat style references as topology evidence.".into(),
    ];
    let approval_payload = serde_json::json!({
        "projectId": project_id,
        "unitId": unit_id,
        "roomId": room_id,
        "assetRole": "panorama",
        "outputCount": 1,
        "inputs": inputs,
        "cameraIntent": camera_intent,
        "requiredOpeningIds": required_opening_ids,
        "fixedFixtures": [],
        "fixedFixturesStatus": "unavailable",
        "prohibitions": prohibitions,
        "modelId": null,
        "capabilityStatus": "unavailable",
        "requestedWidth": PREFERRED_PROVIDER_PANORAMA_WIDTH,
        "requestedHeight": PREFERRED_PROVIDER_PANORAMA_HEIGHT,
        "panoramaMode": "one_shot_2_1",
        "providerChoice": "unconfigured",
        "priceStatus": "unavailable",
        "topologyFingerprint": topology_fingerprint,
    });
    let approval_fingerprint = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&approval_payload).map_err(|error| error.to_string())?)
    );
    let idempotency_key = format!(
        "{:x}",
        Sha256::digest(format!(
            "panorama:{project_id}:{unit_id}:{room_id}:{approval_fingerprint}:1"
        ))
    );
    let job = GenerationJob {
        id: format!("panorama-{unit_id}-{room_id}-{timestamp}"),
        unit_id: unit_id.clone(),
        room_id: room_id.clone(),
        room_name,
        asset_role: "panorama".into(),
        output_count: 1,
        dimensions: "3840 × 1920 provider-native master; local 4K/8K derivatives after acceptance".into(),
        panorama_mode: "one_shot_2_1".into(),
        connection_mode: "unconfigured".into(),
        price_status: "unavailable".into(),
        status: "blocked_capability".into(),
        created_at: timestamp,
        inputs,
        camera_intent,
        required_opening_ids,
        fixed_fixtures: vec![],
        fixed_fixtures_status: "unavailable".into(),
        prohibitions,
        model_id: None,
        capability_status: "unavailable".into(),
        requested_width: PREFERRED_PROVIDER_PANORAMA_WIDTH,
        requested_height: PREFERRED_PROVIDER_PANORAMA_HEIGHT,
        provider_choice: "unconfigured".into(),
        price_amount_minor: None,
        price_currency: None,
        customer_credit_cost: None,
        approval_state: "not_approved".into(),
        approval_fingerprint,
        topology_fingerprint,
        idempotency_key,
        attempt: 1,
        progress_percent: 0,
        updated_at: timestamp,
        last_heartbeat_at: None,
        timeout_at: None,
        failure_code: Some("capability_unavailable".into()),
        failure_reason: Some(
            "Provider, model capability, quota, and current price are unavailable.".into(),
        ),
        retry_allowed: false,
        approved_at: None,
        submitted_at: None,
        completed_at: None,
        provider_task_id: None,
        state_events: vec![GenerationStateEvent {
            id: format!("job-event-panorama-{unit_id}-{room_id}-{timestamp}-1"),
            from_status: "draft".into(),
            to_status: "blocked_capability".into(),
            progress_percent: 0,
            actor: "system".into(),
            note: Some(
                "Saved without submission because provider capability and current price are unavailable."
                    .into(),
            ),
            created_at: timestamp,
        }],
        outputs: vec![],
        correction_requests: vec![],
        fallback_plans: vec![],
    };
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .expect("validated unit");
    unit.rooms
        .iter_mut()
        .find(|room| room.id == room_id)
        .expect("validated room")
        .panorama_status = "draft".into();
    manifest.generation_jobs.push(job);
    manifest.modules.tour = "in_progress".into();
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn approve_generation_job(
    app: AppHandle,
    project_id: String,
    job_id: String,
    confirmed_fingerprint: String,
) -> Result<ProjectRecord, String> {
    require_entitlement(&app, "managed_generation")?;
    require_local_role(&app, &["owner", "reviewer"])?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let job_snapshot = manifest.generation_jobs[index].clone();
    validate_generation_package(&manifest, &job_snapshot)?;
    if job_snapshot.approval_fingerprint != confirmed_fingerprint {
        return Err(
            "The reviewed package fingerprint changed; reopen and review the exact package.".into(),
        );
    }
    if job_snapshot.status != "awaiting_approval" {
        return Err(format!(
            "Package is {}, not awaiting approval.",
            job_snapshot.status
        ));
    }
    if job_snapshot.capability_status != "available"
        || job_snapshot.provider_choice == "unconfigured"
        || job_snapshot.model_id.is_none()
    {
        return Err("Provider and model capability must be available before approval.".into());
    }
    if job_snapshot.price_status != "available"
        || job_snapshot.price_amount_minor.is_none()
        || job_snapshot.price_currency.is_none()
        || (job_snapshot.provider_choice == "managed_openai"
            && job_snapshot.customer_credit_cost.is_none())
    {
        return Err("Current provider price is unavailable; approval remains blocked.".into());
    }
    let timestamp = unix_time();
    let job = &mut manifest.generation_jobs[index];
    job.approval_state = "approved".into();
    job.approved_at = Some(timestamp);
    job.failure_code = None;
    job.failure_reason = None;
    let progress = job.progress_percent;
    transition_generation_job(
        job,
        "approved",
        progress,
        "local_user",
        Some("User approved the exact fingerprinted package and current price.".into()),
        timestamp,
    )?;
    manifest.approval_events.push(ApprovalEvent {
        id: format!("approval-generation-{job_id}-{timestamp}"),
        subject_type: "generation_job".into(),
        subject_id: job_id,
        decision: "approved".into(),
        actor: "local_user".into(),
        reason: Some(confirmed_fingerprint),
        created_at: timestamp,
    });
    record_funnel_event(
        &mut manifest,
        "first_generation_approved",
        "desktop_workflow",
        timestamp,
    )?;
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn retry_generation_job(
    app: AppHandle,
    project_id: String,
    job_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[index].clone();
    if !snapshot.retry_allowed || !matches!(snapshot.status.as_str(), "failed" | "timed_out") {
        return Err("This generation attempt is not eligible for retry.".into());
    }
    validate_generation_package(&manifest, &snapshot)?;
    if snapshot.capability_status != "available"
        || snapshot.price_status != "available"
        || snapshot.price_amount_minor.is_none()
        || snapshot.price_currency.is_none()
    {
        return Err(
            "Retry remains blocked until capability, quota, and current price are available."
                .into(),
        );
    }
    let timestamp = unix_time();
    let job = &mut manifest.generation_jobs[index];
    job.attempt = job.attempt.saturating_add(1);
    job.idempotency_key = format!(
        "{:x}",
        Sha256::digest(format!(
            "{}:{}:{}",
            job.id, job.approval_fingerprint, job.attempt
        ))
    );
    job.approval_state = "not_approved".into();
    job.approved_at = None;
    job.submitted_at = None;
    job.completed_at = None;
    job.provider_task_id = None;
    job.failure_code = None;
    job.failure_reason = None;
    job.retry_allowed = false;
    transition_generation_job(
        job,
        "awaiting_approval",
        0,
        "local_user",
        Some(
            "Retry created a new attempt and idempotency key; explicit approval is required again."
                .into(),
        ),
        timestamp,
    )?;
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn cancel_generation_job(
    app: AppHandle,
    project_id: String,
    job_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job = manifest
        .generation_jobs
        .iter_mut()
        .find(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let timestamp = unix_time();
    let progress = job.progress_percent;
    transition_generation_job(
        job,
        "cancelled",
        progress,
        "local_user",
        Some("Cancelled locally; no replacement task was submitted.".into()),
        timestamp,
    )?;
    job.approval_state = "invalidated".into();
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn record_generation_progress(
    app: AppHandle,
    project_id: String,
    job_id: String,
    idempotency_key: String,
    next_status: String,
    progress_percent: u8,
    provider_task_id: Option<String>,
    failure_code: Option<String>,
    failure_reason: Option<String>,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job = manifest
        .generation_jobs
        .iter_mut()
        .find(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    if job.idempotency_key != idempotency_key {
        return Err("Idempotency key does not match the active generation attempt.".into());
    }
    if ![
        "queued",
        "submitted",
        "processing",
        "completed",
        "failed",
        "timed_out",
    ]
    .contains(&next_status.as_str())
    {
        return Err("Unsupported provider progress state.".into());
    }
    if matches!(
        next_status.as_str(),
        "submitted" | "processing" | "completed"
    ) && provider_task_id.as_deref().is_none_or(str::is_empty)
        && job.provider_task_id.is_none()
    {
        return Err("Provider task ID is required after submission.".into());
    }
    if let Some(provider_task_id) = provider_task_id {
        if job
            .provider_task_id
            .as_ref()
            .is_some_and(|current| current != &provider_task_id)
        {
            return Err("Provider task ID changed within the same idempotent attempt.".into());
        }
        job.provider_task_id = Some(provider_task_id);
    }
    if next_status == "failed" {
        let code = failure_code
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| "Failed jobs require a structured failure code.".to_string())?;
        job.retry_allowed =
            ["timeout", "provider_unavailable", "rate_limit", "transient"].contains(&code.as_str());
        job.failure_code = Some(code);
        job.failure_reason = failure_reason.filter(|value| !value.trim().is_empty());
    }
    let timestamp = unix_time();
    transition_generation_job(
        job,
        &next_status,
        progress_percent,
        "provider_adapter",
        None,
        timestamp,
    )?;
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn ingest_generation_output(
    app: AppHandle,
    project_id: String,
    job_id: String,
    input: IngestGenerationOutputInput,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[index].clone();
    if snapshot.status != "completed" || snapshot.progress_percent != 100 {
        return Err("Only a completed provider attempt can ingest a visual output.".into());
    }
    if input.request_fingerprint != snapshot.approval_fingerprint {
        return Err("Output request fingerprint does not match the approved package.".into());
    }
    let expected_provider_request = snapshot
        .provider_task_id
        .as_deref()
        .ok_or_else(|| "Completed generation is missing its provider request ID.".to_string())?;
    if input.provider_request_id.trim() != expected_provider_request {
        return Err("Output provider request ID does not match the completed attempt.".into());
    }
    let source = PathBuf::from(&input.source_path);
    if !source.is_file() {
        return Err("Generated output file is unavailable.".into());
    }
    let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let mime_type = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        _ => return Err("Generated output must be PNG, JPEG, or WebP.".into()),
    };
    let metadata = fs::metadata(&source)
        .map_err(|error| format!("Unable to inspect generated output: {error}"))?;
    if metadata.len() == 0 || metadata.len() > 200 * 1024 * 1024 {
        return Err("Generated output must be between 1 byte and 200 MB.".into());
    }
    let (width, height) = image::image_dimensions(&source)
        .map_err(|error| format!("Unable to decode generated output dimensions: {error}"))?;
    let version = snapshot
        .outputs
        .iter()
        .map(|output| output.version)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let relative_path = format!("assets/generated/{job_id}/v{version}.{extension}");
    let destination = root.join(&relative_path);
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to create generated output directory: {error}"))?;
    }
    fs::copy(&source, &destination)
        .map_err(|error| format!("Unable to copy generated output: {error}"))?;
    let checksum_sha256 = sha256_file(&destination)?;
    if snapshot.outputs.iter().any(|output| {
        output.checksum_sha256 == checksum_sha256
            && output.provider_request_id == input.provider_request_id
    }) {
        let _ = fs::remove_file(&destination);
        return Err("This provider output has already been ingested.".into());
    }
    let timestamp = unix_time();
    manifest.generation_jobs[index]
        .outputs
        .push(GenerationOutputRecord {
            id: format!("output-{job_id}-v{version}"),
            version,
            relative_path,
            checksum_sha256,
            mime_type: mime_type.into(),
            width,
            height,
            size_bytes: metadata.len(),
            provider_request_id: input.provider_request_id,
            request_fingerprint: input.request_fingerprint,
            revised_prompt: None,
            source_attempt: snapshot.attempt,
            status: "pending_review".into(),
            publishability: "blocked_visual_review".into(),
            rejection_reason: None,
            reviewed_at: None,
            panorama_qa: None,
            derivatives: vec![],
            created_at: timestamp,
        });
    manifest.qa_records.push(QaRecord {
        id: format!("qa-output-ingest-{job_id}-{version}-{timestamp}"),
        scope: format!("generation_output:{job_id}:v{version}"),
        status: "pending_visual_review".into(),
        checks: vec![
            "Provider request ID matched completed attempt".into(),
            "Approval fingerprint matched".into(),
            format!("Decoded dimensions: {width} × {height}"),
            "Publishability remains blocked until visual and panorama QA".into(),
        ],
        reviewer: "system".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn review_generation_output(
    app: AppHandle,
    project_id: String,
    job_id: String,
    output_id: String,
    input: ReviewGenerationOutputInput,
) -> Result<ProjectRecord, String> {
    if !matches!(input.decision.as_str(), "accepted" | "rejected") {
        return Err("Visual decision must be accepted or rejected.".into());
    }
    let reason = input.reason.trim();
    let correction = input.correction_instruction.trim();
    if input.decision == "rejected" && reason.is_empty() {
        return Err("Rejected visual output requires a reason.".into());
    }
    if reason.len() > 500 || correction.len() > 1_200 {
        return Err("Visual review reason or correction instruction is too long.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job = manifest
        .generation_jobs
        .iter_mut()
        .find(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let output_index = job
        .outputs
        .iter()
        .position(|output| output.id == output_id)
        .ok_or_else(|| "Generated output version is unavailable.".to_string())?;
    if job.outputs[output_index].status != "pending_review" {
        return Err("Only a pending visual output can receive a review decision.".into());
    }
    let timestamp = unix_time();
    if input.decision == "accepted" {
        for output in &mut job.outputs {
            if output.status == "accepted" {
                output.status = "superseded".into();
                output.publishability = "blocked_superseded".into();
            }
        }
        let output = &mut job.outputs[output_index];
        output.status = "accepted".into();
        output.publishability = "blocked_panorama_qa".into();
        output.reviewed_at = Some(timestamp);
    } else {
        let output = &mut job.outputs[output_index];
        output.status = "rejected".into();
        output.publishability = "blocked_rejected".into();
        output.rejection_reason = Some(reason.into());
        output.reviewed_at = Some(timestamp);
        if !correction.is_empty() {
            job.correction_requests.push(CorrectionRequestRecord {
                id: format!("correction-{output_id}-{timestamp}"),
                parent_output_id: output_id.clone(),
                instruction: correction.into(),
                status: "draft_unsubmitted".into(),
                created_at: timestamp,
            });
        }
    }
    manifest.qa_records.push(QaRecord {
        id: format!("qa-output-review-{output_id}-{timestamp}"),
        scope: format!("generation_output:{job_id}:{output_id}"),
        status: if input.decision == "accepted" {
            "visual_accepted_panorama_qa_pending"
        } else {
            "visual_rejected"
        }
        .into(),
        checks: vec![
            format!("Human visual decision: {}", input.decision),
            if correction.is_empty() {
                "No correction request created".into()
            } else {
                "Targeted correction draft created; no provider submission".into()
            },
        ],
        reviewer: "local_user".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn panorama_seam_edge_delta(path: &Path) -> Result<f64, String> {
    let image = image::open(path)
        .map_err(|error| format!("Unable to decode panorama for seam QA: {error}"))?
        .to_rgba8();
    let (width, height) = image.dimensions();
    if width < 2 || height == 0 {
        return Err("Panorama is too small for seam QA.".into());
    }
    let mut delta = 0_u64;
    for y in 0..height {
        let left = image.get_pixel(0, y).0;
        let right = image.get_pixel(width - 1, y).0;
        for channel in 0..3 {
            delta = delta.saturating_add(left[channel].abs_diff(right[channel]) as u64);
        }
    }
    Ok(delta as f64 / (height as f64 * 3.0 * 255.0))
}

#[tauri::command]
fn record_panorama_qa(
    app: AppHandle,
    project_id: String,
    job_id: String,
    output_id: String,
    input: PanoramaQaInput,
) -> Result<ProjectRecord, String> {
    if !input.yaw_degrees.is_finite() || !(-180.0..=180.0).contains(&input.yaw_degrees) {
        return Err("Panorama start orientation must be between -180 and 180 degrees.".into());
    }
    let permission = input.usage_permission_reference.trim();
    if permission.is_empty() || permission.len() > 240 {
        return Err(
            "A concise output usage-permission reference is required for publishability.".into(),
        );
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job_index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let job_snapshot = manifest.generation_jobs[job_index].clone();
    let output_snapshot = job_snapshot
        .outputs
        .iter()
        .find(|output| output.id == output_id)
        .cloned()
        .ok_or_else(|| "Generated output version is unavailable.".to_string())?;
    if output_snapshot.status != "accepted" {
        return Err("Panorama QA requires the currently accepted visual version.".into());
    }
    let path = root.join(&output_snapshot.relative_path);
    if sha256_file(&path)? != output_snapshot.checksum_sha256 {
        return Err("Accepted panorama bytes changed after ingestion.".into());
    }
    let (decoded_width, decoded_height) = image::image_dimensions(&path)
        .map_err(|error| format!("Unable to decode accepted panorama dimensions: {error}"))?;
    let projection_passed = decoded_width == decoded_height.saturating_mul(2);
    let dimension_passed = decoded_width == job_snapshot.requested_width
        && decoded_height == job_snapshot.requested_height;
    let seam_edge_delta = panorama_seam_edge_delta(&path)?;
    let mut expected_openings = job_snapshot.required_opening_ids.clone();
    expected_openings.sort();
    expected_openings.dedup();
    let mut confirmed_openings = input.confirmed_opening_ids.clone();
    confirmed_openings.sort();
    confirmed_openings.dedup();
    let topology_passed = input.topology_passed && confirmed_openings == expected_openings;
    let passed = projection_passed
        && dimension_passed
        && topology_passed
        && input.horizon_passed
        && input.seam_passed
        && input.orientation_passed
        && input.runtime_passed;
    let timestamp = unix_time();
    let qa = PanoramaQaRecord {
        projection_status: if projection_passed {
            "passed"
        } else {
            "failed"
        }
        .into(),
        dimension_status: if dimension_passed { "passed" } else { "failed" }.into(),
        decoded_width,
        decoded_height,
        seam_edge_delta,
        seam_status: if input.seam_passed {
            "passed_human_review"
        } else {
            "failed_human_review"
        }
        .into(),
        topology_status: if topology_passed { "passed" } else { "failed" }.into(),
        confirmed_opening_ids: confirmed_openings,
        horizon_status: if input.horizon_passed {
            "passed_human_review"
        } else {
            "failed_human_review"
        }
        .into(),
        orientation_status: if input.orientation_passed {
            "passed_human_review"
        } else {
            "failed_human_review"
        }
        .into(),
        yaw_degrees: input.yaw_degrees,
        runtime_status: if input.runtime_passed {
            "passed_human_review"
        } else {
            "failed_human_review"
        }
        .into(),
        usage_permission_reference: permission.into(),
        overall_status: if passed { "passed" } else { "failed" }.into(),
        reviewer: "local_user".into(),
        created_at: timestamp,
    };
    let output = manifest.generation_jobs[job_index]
        .outputs
        .iter_mut()
        .find(|output| output.id == output_id)
        .expect("output snapshot was validated");
    output.panorama_qa = Some(qa);
    output.publishability = if passed {
        "publishable"
    } else {
        "blocked_panorama_qa"
    }
    .into();
    if passed {
        let asset_id = format!("generated-{output_id}");
        if !manifest.assets.iter().any(|asset| asset.id == asset_id) {
            manifest.assets.push(AssetRecord {
                id: asset_id.clone(),
                name: format!("{} panorama v{}", job_snapshot.room_name, output_snapshot.version),
                category: "panoramas".into(),
                evidence_class: "concept_floorplan_grounded".into(),
                status: "accepted".into(),
                imported_at: timestamp,
                relative_path: output_snapshot.relative_path.clone(),
                size_bytes: output_snapshot.size_bytes,
                checksum_sha256: output_snapshot.checksum_sha256.clone(),
                mime_type: output_snapshot.mime_type.clone(),
                source_owner: "managed_generation_output".into(),
                usage_permission: permission.into(),
                original_relative_path: output_snapshot.relative_path.clone(),
                derivative_relative_paths: vec![],
                duplicate_of_asset_id: None,
                unit_id: Some(job_snapshot.unit_id.clone()),
                width: Some(decoded_width),
                height: Some(decoded_height),
                rejection_reason_code: None,
                rejection_notes: None,
                review_events: vec![AssetReviewEvent {
                    id: format!("review-generated-{output_id}-{timestamp}"),
                    from_status: "needs_review".into(),
                    to_status: "accepted".into(),
                    actor: "local_user".into(),
                    reason_code: None,
                    notes: Some("Visual and panorama QA passed; output usage permission reference recorded.".into()),
                    created_at: timestamp,
                }],
            });
        }
        let unit = manifest
            .units
            .iter_mut()
            .find(|unit| unit.id == job_snapshot.unit_id)
            .ok_or_else(|| "Panorama unit is unavailable.".to_string())?;
        let room = unit
            .rooms
            .iter_mut()
            .find(|room| room.id == job_snapshot.room_id)
            .ok_or_else(|| "Panorama room is unavailable.".to_string())?;
        room.panorama_asset_id = Some(asset_id);
        room.panorama_status = "ready".into();
        room.status = "approved".into();
        unit.tour_available = unit.room_graph_locked
            && !unit.rooms.is_empty()
            && unit
                .rooms
                .iter()
                .all(|room| room.panorama_status == "ready" && room.panorama_asset_id.is_some());
    } else {
        let asset_id = format!("generated-{output_id}");
        if let Some(asset) = manifest
            .assets
            .iter_mut()
            .find(|asset| asset.id == asset_id)
        {
            asset.status = "needs_review".into();
        }
        if let Some(room) = manifest
            .units
            .iter_mut()
            .find(|unit| unit.id == job_snapshot.unit_id)
            .and_then(|unit| {
                unit.rooms
                    .iter_mut()
                    .find(|room| room.id == job_snapshot.room_id)
            })
        {
            if room.panorama_asset_id.as_deref() == Some(asset_id.as_str()) {
                room.panorama_asset_id = None;
            }
            room.panorama_status = "awaiting_approval".into();
            room.status = "in_review".into();
        }
    }
    manifest.qa_records.push(QaRecord {
        id: format!("qa-panorama-{output_id}-{timestamp}"),
        scope: format!("generation_output:{job_id}:{output_id}"),
        status: if passed { "passed" } else { "failed" }.into(),
        checks: vec![
            format!("Projection 2:1: {}", if projection_passed { "passed" } else { "failed" }),
            format!("Decoded dimensions {decoded_width} × {decoded_height}: {}", if dimension_passed { "passed" } else { "failed" }),
            format!("Seam edge delta metric: {seam_edge_delta:.6}"),
            format!("Confirmed topology openings: {} of {}", input.confirmed_opening_ids.len(), expected_openings.len()),
            format!("Start orientation: {:.2}°", input.yaw_degrees),
            "Horizon, visible seam, orientation, topology and runtime statuses are human-reviewed, not inferred from the metric.".into(),
        ],
        reviewer: "local_user".into(),
        created_at: timestamp,
    });
    if passed {
        record_funnel_event(
            &mut manifest,
            "first_room_accepted",
            "panorama_qa",
            timestamp,
        )?;
        record_asset_matrix_if_complete(&mut manifest, timestamp);
    }
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn deterministic_seam_repair(mut image: image::RgbaImage) -> Result<image::RgbaImage, String> {
    let (width, height) = image.dimensions();
    if height == 0 || width != height.saturating_mul(2) {
        return Err("Seam repair accepts only a decoded 2:1 panorama.".into());
    }
    let band = (width / 128).clamp(8, 128).min(width / 4);
    for y in 0..height {
        for offset in 0..band {
            let left_x = offset;
            let right_x = width - 1 - offset;
            let left = image.get_pixel(left_x, y).0;
            let right = image.get_pixel(right_x, y).0;
            let weight = 1.0 - offset as f32 / band as f32;
            let mut repaired_left = left;
            let mut repaired_right = right;
            for channel in 0..3 {
                let average = (u16::from(left[channel]) + u16::from(right[channel])) as f32 / 2.0;
                repaired_left[channel] =
                    (left[channel] as f32 * (1.0 - weight) + average * weight).round() as u8;
                repaired_right[channel] =
                    (right[channel] as f32 * (1.0 - weight) + average * weight).round() as u8;
            }
            image.put_pixel(left_x, y, image::Rgba(repaired_left));
            image.put_pixel(right_x, y, image::Rgba(repaired_right));
        }
    }
    Ok(image)
}

fn deterministic_panorama_resize(
    source: &image::RgbaImage,
    width: u32,
    height: u32,
) -> Result<(image::RgbaImage, String), String> {
    if width != height.saturating_mul(2) || source.width() != source.height().saturating_mul(2) {
        return Err("Panorama derivatives require matching 2:1 source and target canvases.".into());
    }
    if source.width() == width && source.height() == height {
        return Ok((source.clone(), "accepted_source_exact_copy".into()));
    }
    if source.width() > width {
        return Ok((
            image::imageops::resize(source, width, height, image::imageops::FilterType::Lanczos3),
            "deterministic_lanczos_downsample".into(),
        ));
    }
    let mut current = source.clone();
    while current.width() < width {
        let next_width = current.width().saturating_mul(2).min(width);
        let next_height = next_width / 2;
        current = image::imageops::resize(
            &current,
            next_width,
            next_height,
            image::imageops::FilterType::Lanczos3,
        );
        current = image::imageops::unsharpen(&current, 0.8, 1);
    }
    Ok((
        current,
        "deterministic_multistep_super_resolution_resample_no_new_detail".into(),
    ))
}

fn build_panorama_derivatives(
    app: AppHandle,
    project_id: String,
    job_id: String,
    output_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job_index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let output_index = manifest.generation_jobs[job_index]
        .outputs
        .iter()
        .position(|output| output.id == output_id)
        .ok_or_else(|| "Generated output version is unavailable.".to_string())?;
    let output_snapshot = manifest.generation_jobs[job_index].outputs[output_index].clone();
    if output_snapshot.status != "accepted"
        || output_snapshot.publishability != "publishable"
        || output_snapshot
            .panorama_qa
            .as_ref()
            .is_none_or(|qa| qa.overall_status != "passed")
    {
        return Err(
            "Derivatives require an accepted panorama with passed QA and publishable rights."
                .into(),
        );
    }
    if !output_snapshot.derivatives.is_empty() {
        return Err("This accepted panorama already has a derivative set; create a new output version instead of overwriting it.".into());
    }
    let source_path = root.join(&output_snapshot.relative_path);
    if sha256_file(&source_path)? != output_snapshot.checksum_sha256 {
        return Err("Accepted panorama bytes changed before derivative processing.".into());
    }
    let decoded = image::open(&source_path)
        .map_err(|error| format!("Unable to decode accepted panorama: {error}"))?
        .to_rgba8();
    if decoded.width() != decoded.height().saturating_mul(2) {
        return Err(
            "A square or non-2:1 image cannot be stretched into panorama derivatives.".into(),
        );
    }
    let repaired = deterministic_seam_repair(decoded)?;
    let timestamp = unix_time();
    let relative_root = format!("assets/processed/{job_id}/{output_id}");
    let destination_root = root.join(&relative_root);
    if destination_root.exists() {
        return Err("Derivative destination already exists; no files were overwritten.".into());
    }
    fs::create_dir_all(&destination_root)
        .map_err(|error| format!("Unable to create derivative directory: {error}"))?;
    let processing_result = (|| -> Result<Vec<PanoramaDerivativeRecord>, String> {
        let mut records = Vec::new();
        let master_relative = format!("{relative_root}/seam-repaired-master.png");
        let master_path = root.join(&master_relative);
        repaired
            .save(&master_path)
            .map_err(|error| format!("Unable to save seam-repaired master: {error}"))?;
        records.push(PanoramaDerivativeRecord {
            kind: "seam_repaired_master".into(),
            relative_path: master_relative,
            checksum_sha256: sha256_file(&master_path)?,
            width: repaired.width(),
            height: repaired.height(),
            source_checksum_sha256: output_snapshot.checksum_sha256.clone(),
            process: "deterministic_symmetric_edge_feather".into(),
            created_at: timestamp,
        });
        for (kind, width, height, filename) in [
            ("mobile", 2048, 1024, "mobile-2048x1024.png"),
            ("4k", 4096, 2048, "4k-4096x2048.png"),
            ("8k", 8192, 4096, "8k-8192x4096.png"),
        ] {
            let (derivative, process) = deterministic_panorama_resize(&repaired, width, height)?;
            let relative_path = format!("{relative_root}/{filename}");
            let path = root.join(&relative_path);
            derivative
                .save(&path)
                .map_err(|error| format!("Unable to save {kind} panorama derivative: {error}"))?;
            records.push(PanoramaDerivativeRecord {
                kind: kind.into(),
                relative_path,
                checksum_sha256: sha256_file(&path)?,
                width,
                height,
                source_checksum_sha256: output_snapshot.checksum_sha256.clone(),
                process,
                created_at: timestamp,
            });
        }
        Ok(records)
    })();
    let records = match processing_result {
        Ok(records) => records,
        Err(error) => {
            let _ = fs::remove_dir_all(&destination_root);
            return Err(error);
        }
    };
    manifest.generation_jobs[job_index].outputs[output_index].derivatives = records.clone();
    let generated_asset_id = format!("generated-{output_id}");
    let asset = manifest
        .assets
        .iter_mut()
        .find(|asset| asset.id == generated_asset_id)
        .ok_or_else(|| "Publishable generated panorama asset is unavailable.".to_string())?;
    asset.derivative_relative_paths = records
        .iter()
        .map(|record| record.relative_path.clone())
        .collect();
    manifest.qa_records.push(QaRecord {
        id: format!("qa-panorama-derivatives-{output_id}-{timestamp}"),
        scope: format!("generation_output:{job_id}:{output_id}"),
        status: "passed".into(),
        checks: records
            .iter()
            .map(|record| {
                format!(
                    "{}: {} × {} · {} · SHA-256 {}",
                    record.kind,
                    record.width,
                    record.height,
                    record.process,
                    record.checksum_sha256
                )
            })
            .collect(),
        reviewer: "deterministic_processor".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn build_supplied_panorama_derivatives_inner(
    app: AppHandle,
    project_id: String,
    asset_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let safe_asset_id = safe_path_component(&asset_id, "Panorama asset ID")?;
    let asset_index = manifest
        .assets
        .iter()
        .position(|asset| asset.id == asset_id)
        .ok_or_else(|| "Supplied panorama asset is unavailable.".to_string())?;
    let snapshot = manifest.assets[asset_index].clone();
    if snapshot.category != "panoramas"
        || snapshot.status != "accepted"
        || snapshot.evidence_class == "unknown"
        || snapshot.usage_permission.trim().is_empty()
    {
        return Err("Derivatives require an accepted, rights-recorded panorama source.".into());
    }
    if !snapshot.derivative_relative_paths.is_empty() {
        return Err("This supplied panorama already has a derivative set; the existing files will not be overwritten.".into());
    }
    let source_path = root.join(&snapshot.relative_path);
    if sha256_file(&source_path)? != snapshot.checksum_sha256 {
        return Err("Accepted panorama bytes changed before derivative processing.".into());
    }
    let decoded = image::open(&source_path)
        .map_err(|error| format!("Unable to decode accepted panorama: {error}"))?
        .to_rgba8();
    if decoded.width() < 4096 || decoded.width() != decoded.height().saturating_mul(2) {
        return Err(format!(
            "Supplied panorama derivatives require an exact 2:1 source at least 4096 px wide; decoded {} × {}.",
            decoded.width(), decoded.height()
        ));
    }
    if snapshot.width != Some(decoded.width()) || snapshot.height != Some(decoded.height()) {
        return Err("Stored panorama dimensions no longer match the decoded source.".into());
    }
    let repaired = deterministic_seam_repair(decoded)?;
    let timestamp = unix_time();
    let relative_root = format!("assets/processed/supplied/{safe_asset_id}");
    let destination_root = root.join(&relative_root);
    if destination_root.exists() {
        return Err("Derivative destination already exists; no files were overwritten.".into());
    }
    fs::create_dir_all(&destination_root)
        .map_err(|error| format!("Unable to create derivative directory: {error}"))?;
    let processing_result = (|| -> Result<Vec<PanoramaDerivativeRecord>, String> {
        let mut records = Vec::new();
        let master_relative = format!("{relative_root}/seam-repaired-master.png");
        let master_path = root.join(&master_relative);
        repaired
            .save(&master_path)
            .map_err(|error| format!("Unable to save seam-repaired master: {error}"))?;
        records.push(PanoramaDerivativeRecord {
            kind: "seam_repaired_master".into(),
            relative_path: master_relative,
            checksum_sha256: sha256_file(&master_path)?,
            width: repaired.width(),
            height: repaired.height(),
            source_checksum_sha256: snapshot.checksum_sha256.clone(),
            process: "deterministic_symmetric_edge_feather".into(),
            created_at: timestamp,
        });
        for (kind, width, height, filename) in [
            ("mobile", 2048, 1024, "mobile-2048x1024.png"),
            ("4k", 4096, 2048, "4k-4096x2048.png"),
            ("8k", 8192, 4096, "8k-8192x4096.png"),
        ] {
            let (derivative, process) = deterministic_panorama_resize(&repaired, width, height)?;
            let relative_path = format!("{relative_root}/{filename}");
            let path = root.join(&relative_path);
            derivative
                .save(&path)
                .map_err(|error| format!("Unable to save {kind} panorama derivative: {error}"))?;
            records.push(PanoramaDerivativeRecord {
                kind: kind.into(),
                relative_path,
                checksum_sha256: sha256_file(&path)?,
                width,
                height,
                source_checksum_sha256: snapshot.checksum_sha256.clone(),
                process,
                created_at: timestamp,
            });
        }
        Ok(records)
    })();
    let records = match processing_result {
        Ok(records) => records,
        Err(error) => {
            let _ = fs::remove_dir_all(&destination_root);
            return Err(error);
        }
    };
    manifest.assets[asset_index].derivative_relative_paths = records
        .iter()
        .map(|record| record.relative_path.clone())
        .collect();
    manifest.qa_records.push(QaRecord {
        id: format!("qa-supplied-panorama-derivatives-{safe_asset_id}-{timestamp}"),
        scope: format!("source_asset:{safe_asset_id}"),
        status: "passed".into(),
        checks: records
            .iter()
            .map(|record| {
                format!(
                    "{}: {} × {} · {} · SHA-256 {}",
                    record.kind,
                    record.width,
                    record.height,
                    record.process,
                    record.checksum_sha256
                )
            })
            .collect(),
        reviewer: "deterministic_processor".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn create_fallback_tasks(
    project_id: &str,
    job: &GenerationJob,
    parent_output_id: &str,
    mode: &str,
) -> Result<Vec<PanoramaFallbackTask>, String> {
    let definitions: Vec<(String, f64, f64, f64, f64)> = match mode {
        "cubefaces" => vec![
            ("front".into(), -45.0, 45.0, 0.0, 0.0),
            ("right".into(), 45.0, 135.0, 0.0, 0.0),
            ("back".into(), 135.0, 225.0, 0.0, 0.0),
            ("left".into(), 225.0, 315.0, 0.0, 0.0),
            ("up".into(), -45.0, 45.0, 90.0, 0.0),
            ("down".into(), -45.0, 45.0, -90.0, 0.0),
        ],
        "overlapping_tiles" => (0..6)
            .map(|index| {
                let centre = f64::from(index) * 60.0;
                (
                    format!("tile-{}", index + 1),
                    centre - 38.0,
                    centre + 38.0,
                    0.0,
                    16.0,
                )
            })
            .collect(),
        _ => return Err("Fallback mode must be cubefaces or overlapping_tiles.".into()),
    };
    let ids = definitions
        .iter()
        .enumerate()
        .map(|(index, _)| format!("fallback-{parent_output_id}-{}", index + 1))
        .collect::<Vec<_>>();
    let mut tasks = Vec::new();
    for (index, (label, yaw_start, yaw_end, pitch, overlap)) in definitions.into_iter().enumerate()
    {
        let adjacent_task_ids = if mode == "overlapping_tiles" || index < 4 {
            let ring_size = if mode == "cubefaces" { 4 } else { ids.len() };
            vec![
                ids[(index + ring_size - 1) % ring_size].clone(),
                ids[(index + 1) % ring_size].clone(),
            ]
        } else {
            ids[..4].to_vec()
        };
        let payload = serde_json::json!({
            "projectId": project_id,
            "jobId": job.id,
            "parentOutputId": parent_output_id,
            "mode": mode,
            "taskId": ids[index],
            "label": label,
            "yawStartDegrees": yaw_start,
            "yawEndDegrees": yaw_end,
            "pitchDegrees": pitch,
            "overlapDegrees": overlap,
            "adjacentTaskIds": adjacent_task_ids,
            "identityFingerprint": job.approval_fingerprint,
            "topologyFingerprint": job.topology_fingerprint,
            "outputCount": 1,
        });
        tasks.push(PanoramaFallbackTask {
            id: ids[index].clone(),
            label,
            yaw_start_degrees: yaw_start,
            yaw_end_degrees: yaw_end,
            pitch_degrees: pitch,
            overlap_degrees: overlap,
            adjacent_task_ids,
            approval_state: "not_approved".into(),
            approval_fingerprint: format!(
                "{:x}",
                Sha256::digest(serde_json::to_vec(&payload).map_err(|error| error.to_string())?)
            ),
        });
    }
    Ok(tasks)
}

#[tauri::command]
fn create_panorama_fallback_plan(
    app: AppHandle,
    project_id: String,
    job_id: String,
    output_id: String,
    input: CreatePanoramaFallbackInput,
) -> Result<ProjectRecord, String> {
    let allowed_reasons = [
        "one_shot_projection_failed",
        "one_shot_topology_failed",
        "one_shot_seam_failed",
        "one_shot_dimension_failed",
        "webgl_runtime_failed",
    ];
    if !allowed_reasons.contains(&input.reason.as_str()) {
        return Err("Fallback reason is unsupported.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job_index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[job_index].clone();
    let output = snapshot
        .outputs
        .iter()
        .find(|output| output.id == output_id)
        .ok_or_else(|| "One-shot output is unavailable.".to_string())?;
    let qa_failed = output
        .panorama_qa
        .as_ref()
        .is_some_and(|qa| qa.overall_status == "failed");
    if output.status != "rejected" && !qa_failed {
        return Err(
            "A fallback plan requires a rejected one-shot output or failed panorama/runtime QA."
                .into(),
        );
    }
    if snapshot
        .fallback_plans
        .iter()
        .any(|plan| plan.parent_output_id == output_id && plan.mode == input.mode)
    {
        return Err("This one-shot output already has the selected fallback plan.".into());
    }
    let identity_anchor = snapshot
        .inputs
        .iter()
        .find(|input| input.role == "identity_anchor")
        .ok_or_else(|| "Fallback continuity requires the accepted identity anchor.".to_string())?;
    let topology_source = snapshot
        .inputs
        .iter()
        .find(|input| input.role == "topology_source")
        .ok_or_else(|| "Fallback continuity requires the accepted topology source.".to_string())?;
    let tasks = create_fallback_tasks(&project_id, &snapshot, &output_id, &input.mode)?;
    let timestamp = unix_time();
    manifest.generation_jobs[job_index]
        .fallback_plans
        .push(PanoramaFallbackPlan {
            id: format!("fallback-plan-{output_id}-{}-{timestamp}", input.mode),
            parent_output_id: output_id,
            mode: input.mode,
            reason: input.reason,
            identity_anchor_asset_id: identity_anchor.asset_id.clone(),
            topology_source_asset_id: topology_source.asset_id.clone(),
            continuity_contract: vec![
                "Every task reuses the same accepted identity anchor and topology source.".into(),
                "Adjacent tasks must preserve shared openings, fixtures, lighting direction, scale, and overlap content.".into(),
                "Each task remains separately unapproved and authorises one output only.".into(),
                "Assembly and panorama QA remain blocked until every segment is accepted.".into(),
            ],
            tasks,
            status: "draft_unapproved".into(),
            created_at: timestamp,
        });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn assign_room_still_fallback(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    room_id: String,
    asset_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let asset = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .cloned()
        .ok_or_else(|| "Still fallback asset is unavailable.".to_string())?;
    if asset.status != "accepted"
        || asset.evidence_class == "unknown"
        || !matches!(
            asset.mime_type.as_str(),
            "image/png" | "image/jpeg" | "image/webp"
        )
        || asset.usage_permission.trim().is_empty()
    {
        return Err(
            "Still fallback must be an accepted rights-recorded PNG, JPEG, or WebP evidence asset."
                .into(),
        );
    }
    if asset
        .unit_id
        .as_deref()
        .is_some_and(|assigned| assigned != unit_id)
    {
        return Err("Still fallback belongs to another unit.".into());
    }
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Fallback unit is unavailable.".to_string())?;
    let room = unit
        .rooms
        .iter_mut()
        .find(|room| room.id == room_id)
        .ok_or_else(|| "Fallback room is unavailable.".to_string())?;
    room.still_fallback_asset_id = Some(asset_id.clone());
    let timestamp = unix_time();
    manifest.qa_records.push(QaRecord {
        id: format!("qa-still-fallback-{unit_id}-{room_id}-{timestamp}"),
        scope: format!("room:{unit_id}:{room_id}"),
        status: "fallback_ready".into(),
        checks: vec![
            format!("Accepted still fallback: {asset_id}"),
            format!("Evidence class: {}", asset.evidence_class),
            "Used when WebGL or panorama runtime validation fails; it does not prove 360 navigation.".into(),
        ],
        reviewer: "local_user".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn validate_room_supporting_asset(
    asset: &AssetRecord,
    unit_id: &str,
    role: &str,
) -> Result<(), String> {
    if !matches!(
        role,
        "threshold" | "reverse" | "still_fallback" | "video" | "poster"
    ) {
        return Err("Unsupported room media role.".into());
    }
    if asset.status != "accepted"
        || asset.evidence_class == "unknown"
        || asset.usage_permission.trim().is_empty()
    {
        return Err("Room media must be accepted, classified, and rights-recorded.".into());
    }
    if asset
        .unit_id
        .as_deref()
        .is_some_and(|assigned| assigned != unit_id)
    {
        return Err("Room media belongs to another unit.".into());
    }
    if role == "video" {
        if asset.category != "videos" || asset.mime_type != "video/mp4" {
            return Err(
                "Representative room video requires an accepted browser-compatible MP4 source."
                    .into(),
            );
        }
    } else if !matches!(
        asset.mime_type.as_str(),
        "image/png" | "image/jpeg" | "image/webp"
    ) {
        return Err("This room media role requires an accepted PNG, JPEG, or WebP source.".into());
    }
    Ok(())
}

#[tauri::command]
fn assign_room_supporting_asset(
    app: AppHandle,
    project_id: String,
    unit_id: String,
    room_id: String,
    role: String,
    asset_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let asset = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .cloned()
        .ok_or_else(|| "Room media asset is unavailable.".to_string())?;
    validate_room_supporting_asset(&asset, &unit_id, &role)?;
    let unit = manifest
        .units
        .iter_mut()
        .find(|unit| unit.id == unit_id)
        .ok_or_else(|| "Room media unit is unavailable.".to_string())?;
    let room = unit
        .rooms
        .iter_mut()
        .find(|room| room.id == room_id)
        .ok_or_else(|| "Room media destination is unavailable.".to_string())?;
    match role.as_str() {
        "threshold" => room.threshold_asset_id = Some(asset_id.clone()),
        "reverse" => room.reverse_asset_id = Some(asset_id.clone()),
        "still_fallback" => room.still_fallback_asset_id = Some(asset_id.clone()),
        "video" => room.video_asset_id = Some(asset_id.clone()),
        "poster" => room.poster_asset_id = Some(asset_id.clone()),
        _ => unreachable!("role validated above"),
    }
    let timestamp = unix_time();
    manifest.qa_records.push(QaRecord {
        id: format!("qa-room-media-{unit_id}-{room_id}-{role}-{timestamp}"),
        scope: format!("room:{unit_id}:{room_id}"),
        status: "assigned".into(),
        checks: vec![
            format!("Accepted room media role {role}: {asset_id}"),
            format!("Evidence class: {}", asset.evidence_class),
            format!("MIME type: {}", asset.mime_type),
            "Source usage permission remains attached; assignment does not change evidence class."
                .into(),
        ],
        reviewer: "local_user".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn get_generation_output_data_url(
    app: AppHandle,
    project_id: String,
    job_id: String,
    output_id: String,
) -> Result<String, String> {
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    let output = manifest
        .generation_jobs
        .iter()
        .find(|job| job.id == job_id)
        .and_then(|job| job.outputs.iter().find(|output| output.id == output_id))
        .ok_or_else(|| "Generated output version is unavailable.".to_string())?;
    let bytes = fs::read(root.join(&output.relative_path))
        .map_err(|error| format!("Unable to read generated output: {error}"))?;
    if bytes.len() > 25 * 1024 * 1024 {
        return Err("Generated output is larger than the 25 MB in-app comparison limit.".into());
    }
    Ok(format!(
        "data:{};base64,{}",
        output.mime_type,
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

#[tauri::command]
fn recover_generation_jobs(app: AppHandle) -> Result<u32, String> {
    let root = projects_root(&app)?;
    let timestamp = unix_time();
    let mut recovered = 0;
    for entry in
        fs::read_dir(&root).map_err(|error| format!("Unable to read project library: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("Unable to read project entry: {error}"))?
            .path();
        if !path.join("project.json").is_file() {
            continue;
        }
        let mut manifest = read_manifest(&path)?;
        if manifest.read_only {
            continue;
        }
        let count = recover_timed_out_jobs(&mut manifest, timestamp)?;
        if count > 0 {
            manifest.updated_at = timestamp;
            write_manifest(&path, &manifest)?;
            recovered += count;
        }
    }
    Ok(recovered)
}

fn background_transition_allowed(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("queued", "running" | "cancelled")
            | ("running", "queued" | "completed" | "failed" | "cancelled")
            | ("failed", "queued" | "cancelled")
    )
}

fn background_job_idempotency_key(project_id: &str, kind: &str, subject_id: &str) -> String {
    format!(
        "{:x}",
        Sha256::digest(format!("background:{project_id}:{kind}:{subject_id}"))
    )
}

fn transition_background_job(
    job: &mut BackgroundJobRecord,
    next_status: &str,
    progress_percent: u8,
    checkpoint: String,
    timestamp: u64,
) -> Result<(), String> {
    if !background_transition_allowed(&job.status, next_status) {
        return Err(format!(
            "Background transition {} → {} is not allowed.",
            job.status, next_status
        ));
    }
    if progress_percent < job.progress_percent && next_status != "queued" {
        return Err("Background progress cannot move backwards within an attempt.".into());
    }
    job.status = next_status.into();
    job.progress_percent = if next_status == "completed" {
        100
    } else {
        progress_percent
    };
    job.checkpoint = checkpoint;
    job.updated_at = timestamp;
    job.lease_expires_at = if next_status == "running" {
        Some(timestamp.saturating_add(900))
    } else {
        None
    };
    Ok(())
}

#[tauri::command]
fn enqueue_background_job(
    app: AppHandle,
    project_id: String,
    kind: String,
    subject_id: String,
) -> Result<ProjectRecord, String> {
    if !["panorama_processing", "static_build", "upload", "download"].contains(&kind.as_str()) {
        return Err("Background job kind is unsupported.".into());
    }
    if subject_id.trim().is_empty() || subject_id.len() > 240 {
        return Err("Background job subject is invalid.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    if kind == "panorama_processing" {
        if let Some(asset_id) = subject_id.strip_prefix("asset:") {
            let asset = manifest
                .assets
                .iter()
                .find(|asset| asset.id == asset_id)
                .ok_or_else(|| "Supplied panorama processing asset is unavailable.".to_string())?;
            if asset.category != "panoramas"
                || asset.status != "accepted"
                || asset.evidence_class == "unknown"
                || asset.usage_permission.trim().is_empty()
                || !asset.derivative_relative_paths.is_empty()
            {
                return Err("Panorama processing requires an accepted, rights-recorded supplied panorama without derivatives.".into());
            }
        } else {
            let (job_id, output_id) = subject_id.split_once(':').ok_or_else(|| {
                "Panorama processing subject must be jobId:outputId or asset:assetId.".to_string()
            })?;
            let output = manifest
                .generation_jobs
                .iter()
                .find(|job| job.id == job_id)
                .and_then(|job| job.outputs.iter().find(|output| output.id == output_id))
                .ok_or_else(|| "Panorama processing output is unavailable.".to_string())?;
            if output.publishability != "publishable" || !output.derivatives.is_empty() {
                return Err(
                    "Panorama processing requires a publishable output without derivatives.".into(),
                );
            }
        }
    }
    let idempotency_key = background_job_idempotency_key(&project_id, &kind, &subject_id);
    if let Some(existing) = manifest
        .background_jobs
        .iter()
        .find(|job| job.idempotency_key == idempotency_key)
    {
        if existing.status == "completed" {
            return Err("This exact background task already completed.".into());
        }
        return Ok(ProjectRecord {
            manifest,
            project_root: root.to_string_lossy().into_owned(),
        });
    }
    let timestamp = unix_time();
    manifest.background_jobs.push(BackgroundJobRecord {
        id: format!("background-{kind}-{timestamp}"),
        kind,
        subject_id,
        idempotency_key,
        status: "queued".into(),
        attempt: 1,
        progress_percent: 0,
        checkpoint: "queued".into(),
        failure_reason: None,
        lease_expires_at: None,
        created_at: timestamp,
        updated_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn record_background_job_progress(
    app: AppHandle,
    project_id: String,
    background_job_id: String,
    idempotency_key: String,
    next_status: String,
    progress_percent: u8,
    checkpoint: String,
    failure_reason: Option<String>,
) -> Result<ProjectRecord, String> {
    if checkpoint.trim().is_empty() || checkpoint.len() > 240 {
        return Err("Background checkpoint is invalid.".into());
    }
    if !matches!(next_status.as_str(), "running" | "completed" | "failed") {
        return Err("External background progress state is unsupported.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job = manifest
        .background_jobs
        .iter_mut()
        .find(|job| job.id == background_job_id)
        .ok_or_else(|| "Background job is unavailable.".to_string())?;
    if job.idempotency_key != idempotency_key {
        return Err("Background job idempotency key does not match.".into());
    }
    if next_status == "failed"
        && failure_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty())
    {
        return Err("Failed background jobs require a reason.".into());
    }
    transition_background_job(job, &next_status, progress_percent, checkpoint, unix_time())?;
    job.failure_reason = if next_status == "failed" {
        failure_reason
    } else {
        None
    };
    manifest.updated_at = unix_time();
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn retry_background_job(
    app: AppHandle,
    project_id: String,
    background_job_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let job = manifest
        .background_jobs
        .iter_mut()
        .find(|job| job.id == background_job_id)
        .ok_or_else(|| "Background job is unavailable.".to_string())?;
    if job.status != "failed" {
        return Err("Only a failed background job can be retried.".into());
    }
    job.attempt = job.attempt.saturating_add(1);
    job.failure_reason = None;
    transition_background_job(
        job,
        "queued",
        0,
        format!("resume_from:{}", job.checkpoint),
        unix_time(),
    )?;
    manifest.updated_at = unix_time();
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn recover_background_jobs(app: AppHandle) -> Result<u32, String> {
    let projects = projects_root(&app)?;
    let timestamp = unix_time();
    let mut recovered = 0;
    for entry in fs::read_dir(&projects)
        .map_err(|error| format!("Unable to read project library: {error}"))?
    {
        let path = entry
            .map_err(|error| format!("Unable to read project entry: {error}"))?
            .path();
        if !path.join("project.json").is_file() {
            continue;
        }
        let mut manifest = read_manifest(&path)?;
        if manifest.read_only {
            continue;
        }
        let mut changed = false;
        for job in &mut manifest.background_jobs {
            if job.status == "running"
                && job
                    .lease_expires_at
                    .is_some_and(|expiry| expiry <= timestamp)
            {
                let checkpoint = format!("resume_from:{}", job.checkpoint);
                transition_background_job(
                    job,
                    "queued",
                    job.progress_percent,
                    checkpoint,
                    timestamp,
                )?;
                recovered += 1;
                changed = true;
            }
        }
        if changed {
            manifest.updated_at = timestamp;
            write_manifest(&path, &manifest)?;
        }
    }
    Ok(recovered)
}

fn run_panorama_background_job_inner(
    app: AppHandle,
    project_id: String,
    background_job_id: String,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .background_jobs
        .iter()
        .position(|job| job.id == background_job_id)
        .ok_or_else(|| "Background job is unavailable.".to_string())?;
    let snapshot = manifest.background_jobs[index].clone();
    if snapshot.kind != "panorama_processing" || snapshot.status != "queued" {
        return Err("Only a queued panorama-processing job can run locally.".into());
    }
    transition_background_job(
        &mut manifest.background_jobs[index],
        "running",
        snapshot.progress_percent,
        "decode_and_seam_repair".into(),
        unix_time(),
    )?;
    manifest.updated_at = unix_time();
    write_manifest(&root, &manifest)?;
    let processing_result = if let Some(asset_id) = snapshot.subject_id.strip_prefix("asset:") {
        build_supplied_panorama_derivatives_inner(app.clone(), project_id.clone(), asset_id.into())
    } else {
        let (generation_job_id, output_id) = snapshot
            .subject_id
            .split_once(':')
            .ok_or_else(|| "Panorama background subject is invalid.".to_string())?;
        build_panorama_derivatives(
            app.clone(),
            project_id.clone(),
            generation_job_id.into(),
            output_id.into(),
        )
    };
    match processing_result {
        Ok(mut project) => {
            let completed_at = unix_time();
            let job = project
                .manifest
                .background_jobs
                .iter_mut()
                .find(|job| job.id == background_job_id)
                .ok_or_else(|| "Background job disappeared during processing.".to_string())?;
            transition_background_job(
                job,
                "completed",
                100,
                "derivatives_verified".into(),
                completed_at,
            )?;
            project.manifest.updated_at = completed_at;
            write_manifest(Path::new(&project.project_root), &project.manifest)?;
            Ok(project)
        }
        Err(error) => {
            let (root, mut manifest) = editable_project(&app, &project_id)?;
            if let Some(job) = manifest
                .background_jobs
                .iter_mut()
                .find(|job| job.id == background_job_id)
            {
                transition_background_job(
                    job,
                    "failed",
                    job.progress_percent,
                    job.checkpoint.clone(),
                    unix_time(),
                )?;
                job.failure_reason = Some(error.clone());
                manifest.updated_at = unix_time();
                write_manifest(&root, &manifest)?;
            }
            Err(error)
        }
    }
}

#[tauri::command]
async fn run_panorama_background_job(
    app: AppHandle,
    project_id: String,
    background_job_id: String,
) -> Result<ProjectRecord, String> {
    tauri::async_runtime::spawn_blocking(move || {
        run_panorama_background_job_inner(app, project_id, background_job_id)
    })
    .await
    .map_err(|error| format!("Panorama background worker stopped unexpectedly: {error}"))?
}

#[tauri::command]
fn list_generation_capabilities(app: AppHandle) -> Result<CapabilityRegistry, String> {
    let path = capability_registry_path(&app)?;
    let registry = read_capability_registry(&path)?;
    if !path.exists() {
        write_capability_registry(&path, &registry)?;
    }
    Ok(registry)
}

#[tauri::command]
fn apply_generation_capability(
    app: AppHandle,
    project_id: String,
    job_id: String,
    provider_choice: String,
    model_id: String,
) -> Result<ProjectRecord, String> {
    if !matches!(provider_choice.as_str(), "codex" | "managed_openai") {
        return Err("Unsupported generation provider choice.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[index].clone();
    validate_generation_package(&manifest, &snapshot)?;
    if !matches!(
        snapshot.status.as_str(),
        "blocked_capability" | "awaiting_approval"
    ) {
        return Err("Capability can only be applied before generation approval.".into());
    }
    let timestamp = unix_time();
    let registry = read_capability_registry(&capability_registry_path(&app)?)?;
    let evaluation = evaluate_generation_capability(
        &registry,
        &provider_choice,
        &model_id,
        snapshot.requested_width,
        snapshot.requested_height,
        &snapshot.panorama_mode,
        snapshot.output_count,
        timestamp,
    )
    .cloned();
    let job = &mut manifest.generation_jobs[index];
    match evaluation {
        Ok(capability) => {
            job.provider_choice = capability.provider_choice;
            job.connection_mode = job.provider_choice.clone();
            job.model_id = Some(capability.model_id);
            job.capability_status = "available".into();
            job.price_status = "available".into();
            job.price_amount_minor = capability.price_amount_minor;
            job.price_currency = capability.price_currency;
            if job.provider_choice != "managed_openai" {
                job.customer_credit_cost = None;
            }
            refresh_generation_approval_identity(&project_id, job)?;
            job.failure_code = None;
            job.failure_reason = None;
            if job.status == "blocked_capability" {
                let progress = job.progress_percent;
                transition_generation_job(
                    job,
                    "awaiting_approval",
                    progress,
                    "capability_registry",
                    Some(
                        "Exact model access, dimensions, panorama mode, price, and quota passed."
                            .into(),
                    ),
                    timestamp,
                )?;
            }
        }
        Err(reason) => {
            job.provider_choice = provider_choice;
            job.model_id = Some(model_id);
            job.capability_status = if reason.contains("unsupported") {
                "unsupported"
            } else {
                "unavailable"
            }
            .into();
            job.price_status = "unavailable".into();
            job.price_amount_minor = None;
            job.price_currency = None;
            job.customer_credit_cost = None;
            job.failure_code = Some("capability_blocked".into());
            job.failure_reason = Some(reason.clone());
            job.updated_at = timestamp;
            if job.status == "awaiting_approval" {
                job.approval_state = "invalidated".into();
                let progress = job.progress_percent;
                transition_generation_job(
                    job,
                    "blocked_capability",
                    progress,
                    "capability_registry",
                    Some(reason),
                    timestamp,
                )?;
            } else {
                job.state_events.push(GenerationStateEvent {
                    id: format!(
                        "job-event-{}-{timestamp}-{}",
                        job.id,
                        job.state_events.len() + 1
                    ),
                    from_status: "blocked_capability".into(),
                    to_status: "blocked_capability".into(),
                    progress_percent: job.progress_percent,
                    actor: "capability_registry".into(),
                    note: Some(reason),
                    created_at: timestamp,
                });
            }
        }
    }
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn review_asset(
    app: AppHandle,
    project_id: String,
    asset_id: String,
    decision: String,
    reason_code: Option<String>,
    notes: Option<String>,
) -> Result<ProjectRecord, String> {
    if !["accepted", "needs_review", "rejected"].contains(&decision.as_str()) {
        return Err("Unsupported review decision".into());
    }
    let allowed_reasons = [
        "rights_missing",
        "evidence_conflict",
        "unreadable",
        "duplicate",
        "too_small",
        "incorrect_unit",
        "topology_conflict",
        "quality_failure",
        "other",
    ];
    let cleaned_reason = reason_code
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    let cleaned_notes = notes
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());
    if cleaned_notes
        .as_ref()
        .is_some_and(|value| value.len() > 500)
    {
        return Err("Review notes must be 500 characters or fewer.".into());
    }
    if decision == "rejected" {
        let reason = cleaned_reason
            .as_ref()
            .ok_or_else(|| "Choose a structured rejection reason.".to_string())?;
        if !allowed_reasons.contains(&reason.as_str()) {
            return Err("Unsupported rejection reason.".into());
        }
        if reason == "other" && cleaned_notes.is_none() {
            return Err("Add review notes when the rejection reason is Other.".into());
        }
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let timestamp = unix_time();
    let previous_status = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .map(|asset| asset.status.clone())
        .ok_or_else(|| "Source asset is unavailable".to_string())?;
    if previous_status == decision {
        return Err("This asset is already in the selected review state.".into());
    }
    let transition_allowed = matches!(
        (previous_status.as_str(), decision.as_str()),
        ("imported", "accepted" | "needs_review" | "rejected")
            | ("needs_review", "accepted" | "rejected")
            | ("accepted", "needs_review" | "rejected")
            | ("rejected", "needs_review")
    );
    if !transition_allowed {
        return Err("This asset review transition is not allowed; reopen a rejected asset for review before accepting it.".into());
    }
    let event_id = format!("asset-review-{asset_id}-{timestamp}");
    {
        let asset = manifest
            .assets
            .iter_mut()
            .find(|asset| asset.id == asset_id)
            .expect("validated asset");
        asset.status = decision.clone();
        asset.rejection_reason_code = if decision == "rejected" {
            cleaned_reason.clone()
        } else {
            None
        };
        asset.rejection_notes = if decision == "rejected" {
            cleaned_notes.clone()
        } else {
            None
        };
        asset.review_events.push(AssetReviewEvent {
            id: event_id.clone(),
            from_status: previous_status.clone(),
            to_status: decision.clone(),
            actor: "local_user".into(),
            reason_code: cleaned_reason.clone(),
            notes: cleaned_notes.clone(),
            created_at: timestamp,
        });
    }
    if decision == "rejected" {
        for unit in &mut manifest.units {
            if unit.floorplan_asset_id.as_deref() == Some(&asset_id) {
                unit.floorplan_asset_id = None;
                invalidate_topology_dependents(unit);
                unit.status = "Floor plan rejected · Replacement required".into();
            }
            let mut panorama_removed = false;
            for room in &mut unit.rooms {
                if room.identity_asset_id.as_deref() == Some(&asset_id) {
                    room.identity_asset_id = None;
                    room.status = "needs_evidence".into();
                }
                if room.panorama_asset_id.as_deref() == Some(&asset_id) {
                    room.panorama_asset_id = None;
                    room.panorama_status = "not_started".into();
                    room.status = if room.identity_asset_id.is_some() {
                        "ready_for_generation"
                    } else {
                        "needs_evidence"
                    }
                    .into();
                    panorama_removed = true;
                }
                for role_asset_id in [
                    &mut room.threshold_asset_id,
                    &mut room.reverse_asset_id,
                    &mut room.still_fallback_asset_id,
                    &mut room.video_asset_id,
                    &mut room.poster_asset_id,
                ] {
                    if role_asset_id.as_deref() == Some(&asset_id) {
                        *role_asset_id = None;
                    }
                }
            }
            if panorama_removed {
                unit.tour_available = false;
            }
        }
        for creative in &mut manifest.creative_jobs {
            if creative.evidence_asset_ids.contains(&asset_id) {
                creative.status = "draft".into();
                creative.warnings.push(format!("Evidence asset {asset_id} was rejected; re-review claims and replace the source before output."));
                creative.updated_at = timestamp;
            }
        }
    }
    manifest.approval_events.push(ApprovalEvent {
        id: event_id,
        subject_type: "asset".into(),
        subject_id: asset_id,
        decision: decision.clone(),
        actor: "local_user".into(),
        reason: cleaned_reason.or(cleaned_notes),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    update_readiness(&mut manifest);
    write_source_register(&root, &manifest.assets)?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn import_project_assets(
    app: AppHandle,
    project_id: String,
    input: ImportAssetsInput,
) -> Result<ProjectRecord, String> {
    let categories = [
        "drawings",
        "renders",
        "photos",
        "panoramas",
        "videos",
        "brand",
        "copy",
    ];
    let evidence_classes = [
        "official",
        "approved_render",
        "concept_floorplan_grounded",
        "concept_style_only",
        "unknown",
    ];
    if !categories.contains(&input.category.as_str()) {
        return Err("Unsupported source category".into());
    }
    if !evidence_classes.contains(&input.evidence_class.as_str()) {
        return Err("Unsupported evidence class".into());
    }
    if input.paths.is_empty() {
        return Err("Choose at least one source file".into());
    }
    let source_owner = clean_required(&input.source_owner, "Source owner")?;
    let usage_permission = clean_required(&input.usage_permission, "Usage permission")?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    if let Some(unit_id) = &input.unit_id {
        if !manifest.units.iter().any(|unit| &unit.id == unit_id) {
            return Err("Selected unit type is unavailable".into());
        }
    }
    let mut prepared_sources = Vec::with_capacity(input.paths.len());
    for raw_path in &input.paths {
        let source = fs::canonicalize(raw_path)
            .map_err(|error| format!("Unable to read selected file: {error}"))?;
        if !source.is_file() {
            return Err(format!("{} is not a file", source.display()));
        }
        let dimensions = validate_source_for_import(&source, &input.category)?;
        let file_name = source
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| "Selected file name is invalid".to_string())?
            .to_string();
        let safe_name = file_name
            .chars()
            .map(|character| {
                if character.is_alphanumeric() || matches!(character, '.' | '-' | '_' | ' ') {
                    character
                } else {
                    '_'
                }
            })
            .collect();
        prepared_sources.push(PreparedImportSource {
            size_bytes: source
                .metadata()
                .map_err(|error| format!("Unable to inspect selected file: {error}"))?
                .len(),
            checksum_sha256: sha256_file(&source)?,
            mime_type: source_mime_type(&source),
            source,
            file_name,
            safe_name,
            dimensions,
        });
    }
    let timestamp = unix_time();
    let mut topology_invalidations = Vec::new();
    let mut copied_paths = Vec::with_capacity(prepared_sources.len());
    for (index, prepared) in prepared_sources.into_iter().enumerate() {
        let duplicate_of_asset_id = manifest
            .assets
            .iter()
            .find(|asset| {
                !asset.checksum_sha256.is_empty()
                    && asset.checksum_sha256 == prepared.checksum_sha256
                    && asset.status != "rejected"
            })
            .map(|asset| asset.id.clone());
        let relative = if input.category == "drawings" {
            if let Some(unit_id) = &input.unit_id {
                format!(
                    "units/{unit_id}/floorplan/{timestamp}-{index}-{}",
                    prepared.safe_name
                )
            } else {
                format!(
                    "sources/drawings/{timestamp}-{index}-{}",
                    prepared.safe_name
                )
            }
        } else {
            format!(
                "sources/{}/{timestamp}-{index}-{}",
                input.category, prepared.safe_name
            )
        };
        let destination = root.join(&relative);
        if let Some(parent) = destination.parent() {
            if let Err(error) = fs::create_dir_all(parent) {
                for copied in &copied_paths {
                    let _ = fs::remove_file(copied);
                }
                return Err(format!("Unable to prepare source folder: {error}"));
            }
        }
        if let Err(error) = fs::copy(&prepared.source, &destination) {
            for copied in &copied_paths {
                let _ = fs::remove_file(copied);
            }
            return Err(format!("Unable to import {}: {error}", prepared.file_name));
        }
        copied_paths.push(destination.clone());
        let asset_id = format!("asset-{timestamp}-{index}");
        let status = if duplicate_of_asset_id.is_some() || input.evidence_class == "unknown" {
            "needs_review"
        } else {
            "imported"
        };
        manifest.assets.push(AssetRecord {
            id: asset_id.clone(),
            name: prepared.file_name,
            category: input.category.clone(),
            evidence_class: input.evidence_class.clone(),
            status: status.into(),
            imported_at: timestamp,
            relative_path: relative.clone(),
            size_bytes: prepared.size_bytes,
            checksum_sha256: prepared.checksum_sha256,
            mime_type: prepared.mime_type,
            source_owner: source_owner.clone(),
            usage_permission: usage_permission.clone(),
            original_relative_path: relative,
            derivative_relative_paths: vec![],
            duplicate_of_asset_id,
            unit_id: input.unit_id.clone(),
            width: prepared.dimensions.map(|value| value.0),
            height: prepared.dimensions.map(|value| value.1),
            rejection_reason_code: None,
            rejection_notes: None,
            review_events: vec![],
        });
        if input.category == "drawings" {
            if let Some(unit_id) = &input.unit_id {
                if let Some(unit) = manifest.units.iter_mut().find(|unit| &unit.id == unit_id) {
                    let replaced_locked_floorplan = unit.floorplan_asset_id.is_some()
                        && unit.floorplan_asset_id.as_deref() != Some(&asset_id)
                        && unit.room_graph_locked;
                    if replaced_locked_floorplan {
                        invalidate_topology_dependents(unit);
                        topology_invalidations.push(unit.id.clone());
                    }
                    unit.floorplan_asset_id = Some(asset_id);
                    unit.floorplan_version = unit.floorplan_version.saturating_add(1).max(1);
                    unit.floorplan_versions.push(FloorPlanVersionRecord {
                        version: unit.floorplan_version,
                        asset_id: unit
                            .floorplan_asset_id
                            .clone()
                            .expect("assigned floor plan"),
                        created_at: timestamp,
                    });
                    unit.status = "Floor plan imported · Review required".into();
                }
            }
        }
    }
    for (index, unit_id) in topology_invalidations.iter().enumerate() {
        manifest.approval_events.push(ApprovalEvent {
            id: format!("approval-floorplan-invalidated-{timestamp}-{index}"),
            subject_type: "unit_topology".into(),
            subject_id: unit_id.clone(),
            decision: "invalidated".into(),
            actor: "system".into(),
            reason: Some("A locked floor plan was replaced; dependent room and panorama approvals require review.".into()),
            created_at: timestamp,
        });
        manifest.qa_records.push(QaRecord {
            id: format!("qa-floorplan-invalidated-{timestamp}-{index}"),
            scope: format!("unit:{unit_id}:topology"),
            status: "failed".into(),
            checks: vec![
                "Locked floor-plan version changed after dependent asset approval.".into(),
            ],
            reviewer: "system".into(),
            created_at: timestamp,
        });
    }
    manifest.updated_at = timestamp;
    update_readiness(&mut manifest);
    write_source_register(&root, &manifest.assets)?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn open_project_folder(app: AppHandle, project_id: String) -> Result<(), String> {
    validate_project_id(&project_id)?;
    let path = projects_root(&app)?.join(project_id);
    if !path.is_dir() {
        return Err("Project folder is unavailable".into());
    }
    #[cfg(target_os = "macos")]
    let mut command = Command::new("open");
    #[cfg(target_os = "windows")]
    let mut command = Command::new("explorer");
    #[cfg(all(unix, not(target_os = "macos")))]
    let mut command = Command::new("xdg-open");
    command
        .arg(&path)
        .spawn()
        .map_err(|error| format!("Unable to open project folder: {error}"))?;
    Ok(())
}

fn write_project_bundle(
    project_root: &Path,
    project_id: &str,
    output: &Path,
) -> Result<(), String> {
    let bundle = build_bundle_manifest(project_root, project_id)?;
    let file = File::create(output)
        .map_err(|error| format!("Unable to create portable bundle: {error}"))?;
    let mut writer = ZipWriter::new(BufWriter::new(file));
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o600);
    writer
        .start_file("bundle.json", options)
        .map_err(|error| format!("Unable to start bundle manifest: {error}"))?;
    writer
        .write_all(
            &serde_json::to_vec_pretty(&bundle)
                .map_err(|error| format!("Unable to encode bundle manifest: {error}"))?,
        )
        .map_err(|error| format!("Unable to write bundle manifest: {error}"))?;
    for record in &bundle.files {
        writer
            .start_file(&record.path, options)
            .map_err(|error| format!("Unable to add {}: {error}", record.path))?;
        let source = project_root.join(&record.path);
        let mut reader = BufReader::new(
            File::open(&source)
                .map_err(|error| format!("Unable to open {}: {error}", record.path))?,
        );
        std::io::copy(&mut reader, &mut writer)
            .map_err(|error| format!("Unable to copy {}: {error}", record.path))?;
    }
    writer
        .finish()
        .map_err(|error| format!("Unable to finish portable bundle: {error}"))?;
    Ok(())
}

#[tauri::command]
fn export_project_bundle(
    app: AppHandle,
    project_id: String,
    output_path: String,
) -> Result<String, String> {
    require_entitlement(&app, "portable_export")?;
    require_local_role(&app, &["owner", "operator"])?;
    validate_project_id(&project_id)?;
    let project_root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&project_root)?;
    if manifest.read_only {
        return Err("The example project cannot be exported until its distribution rights audit is complete".into());
    }
    let output = PathBuf::from(clean_required(&output_path, "Bundle destination")?);
    if output.starts_with(&project_root) {
        return Err("Choose a bundle destination outside the project workspace".into());
    }
    write_project_bundle(&project_root, &manifest.project_id, &output)?;
    Ok(output.to_string_lossy().into_owned())
}

fn import_project_bundle_inner(projects: &Path, input: &Path) -> Result<ProjectRecord, String> {
    let file =
        File::open(input).map_err(|error| format!("Unable to open portable bundle: {error}"))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| format!("Portable bundle is not a valid ZIP archive: {error}"))?;
    let bundle: ProjectBundleManifest = {
        let mut entry = archive
            .by_name("bundle.json")
            .map_err(|_| "Portable bundle is missing bundle.json".to_string())?;
        serde_json::from_reader(&mut entry)
            .map_err(|error| format!("Portable bundle manifest is invalid: {error}"))?
    };
    if bundle.bundle_schema_version != PROJECT_BUNDLE_SCHEMA_VERSION {
        return Err(format!(
            "Unsupported bundle schema {}; this application supports {}",
            bundle.bundle_schema_version, PROJECT_BUNDLE_SCHEMA_VERSION
        ));
    }
    if bundle.project_schema_version > CURRENT_SCHEMA_VERSION {
        return Err("This project was created by a newer Estate Studio version".into());
    }
    validate_project_id(&bundle.project_id)?;
    let target = projects.join(&bundle.project_id);
    if target.exists() {
        return Err(format!(
            "Project ID {} already exists; import never overwrites an existing project",
            bundle.project_id
        ));
    }
    let mut declared = HashSet::new();
    let mut total_bytes = 0_u64;
    for record in &bundle.files {
        portable_relative_path(Path::new(&record.path))?;
        if !declared.insert(record.path.clone()) {
            return Err(format!("Bundle declares {} more than once", record.path));
        }
        total_bytes = total_bytes
            .checked_add(record.size_bytes)
            .ok_or_else(|| "Bundle size overflow".to_string())?;
    }
    if total_bytes > 20 * 1024 * 1024 * 1024 {
        return Err("Portable bundle exceeds the 20 GB import limit".into());
    }
    let mut archive_names = HashSet::new();
    for index in 0..archive.len() {
        let entry = archive
            .by_index(index)
            .map_err(|error| format!("Unable to inspect bundle entry: {error}"))?;
        let name = entry.name().to_string();
        if !archive_names.insert(name.clone()) {
            return Err(format!("Bundle contains duplicate archive entry {name}"));
        }
        if name != "bundle.json" && !declared.contains(&name) {
            return Err(format!("Bundle contains undeclared file {name}"));
        }
    }
    if archive_names.len() != declared.len() + 1 {
        return Err("Bundle file inventory does not match its manifest".into());
    }
    let temporary = projects.join(format!(
        ".import-{}-{}",
        slugify(&bundle.project_id),
        unix_time()
    ));
    if temporary.exists() {
        return Err("A temporary import workspace already exists".into());
    }
    fs::create_dir_all(&temporary)
        .map_err(|error| format!("Unable to prepare import workspace: {error}"))?;
    let result = (|| -> Result<ProjectRecord, String> {
        for record in &bundle.files {
            let mut entry = archive
                .by_name(&record.path)
                .map_err(|_| format!("Bundle is missing {}", record.path))?;
            if entry.is_dir() || entry.size() != record.size_bytes {
                return Err(format!("Bundle size or type mismatch for {}", record.path));
            }
            let destination = temporary.join(&record.path);
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("Unable to create import directory: {error}"))?;
            }
            let mut output = BufWriter::new(
                File::create(&destination)
                    .map_err(|error| format!("Unable to create {}: {error}", record.path))?,
            );
            let mut hasher = Sha256::new();
            let mut written = 0_u64;
            let mut buffer = [0_u8; 64 * 1024];
            loop {
                let count = entry
                    .read(&mut buffer)
                    .map_err(|error| format!("Unable to read {}: {error}", record.path))?;
                if count == 0 {
                    break;
                }
                output
                    .write_all(&buffer[..count])
                    .map_err(|error| format!("Unable to write {}: {error}", record.path))?;
                hasher.update(&buffer[..count]);
                written += count as u64;
            }
            output
                .flush()
                .map_err(|error| format!("Unable to finish {}: {error}", record.path))?;
            let checksum = format!("{:x}", hasher.finalize());
            if written != record.size_bytes || checksum != record.checksum_sha256 {
                return Err(format!("Checksum verification failed for {}", record.path));
            }
            scan_bundle_file_for_secrets(Path::new(&record.path), &destination)?;
        }
        let manifest = read_manifest(&temporary)?;
        if manifest.project_id != bundle.project_id {
            return Err("Bundle project ID does not match project.json".into());
        }
        if manifest.read_only {
            return Err(
                "Read-only example projects cannot be imported as customer workspaces".into(),
            );
        }
        write_manifest(&temporary, &manifest)?;
        fs::rename(&temporary, &target)
            .map_err(|error| format!("Unable to install imported project: {error}"))?;
        Ok(ProjectRecord {
            manifest,
            project_root: target.to_string_lossy().into_owned(),
        })
    })();
    if result.is_err() && temporary.exists() {
        let _ = fs::remove_dir_all(&temporary);
    }
    result
}

#[tauri::command]
fn import_project_bundle(app: AppHandle, input_path: String) -> Result<ProjectRecord, String> {
    let input = PathBuf::from(clean_required(&input_path, "Bundle file")?);
    let project = import_project_bundle_inner(&projects_root(&app)?, &input)?;
    register_company_profile(
        &app,
        &project.manifest.company,
        &project.manifest.locale,
        &project.manifest.measurement_units,
        &project.manifest.brand,
    )?;
    Ok(project)
}

#[tauri::command]
fn get_asset_data_url(
    app: AppHandle,
    project_id: String,
    asset_id: String,
) -> Result<String, String> {
    validate_project_id(&project_id)?;
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    let asset = manifest
        .assets
        .iter()
        .find(|asset| asset.id == asset_id)
        .ok_or_else(|| "Source asset is unavailable".to_string())?;
    let extension = Path::new(&asset.relative_path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let mime = match extension.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp4" => "video/mp4",
        _ => return Err("This source type does not support an in-app preview.".into()),
    };
    let path = root.join(&asset.relative_path);
    let bytes =
        fs::read(&path).map_err(|error| format!("Unable to read source preview: {error}"))?;
    if bytes.len() > 20 * 1024 * 1024 {
        return Err("Source preview is larger than the 20 MB in-app limit.".into());
    }
    Ok(format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}

fn copy_static_tour_asset(
    project_root: &Path,
    build_root: &Path,
    asset: &AssetRecord,
    label: &str,
) -> Result<String, String> {
    if asset.status != "accepted" || asset.usage_permission.trim().is_empty() {
        return Err(format!(
            "Tour {label} is not accepted with recorded usage rights."
        ));
    }
    let source = project_root.join(&asset.relative_path);
    if sha256_file(&source)? != asset.checksum_sha256 {
        return Err(format!("Tour {label} changed after acceptance."));
    }
    let extension = Path::new(&asset.relative_path)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("bin")
        .to_ascii_lowercase();
    let relative = format!("assets/{}.{extension}", asset.checksum_sha256);
    let destination = build_root.join(&relative);
    if !destination.exists() {
        fs::copy(&source, &destination)
            .map_err(|error| format!("Unable to copy tour {label}: {error}"))?;
    }
    Ok(relative)
}

fn collect_static_build_files(
    build_root: &Path,
    directory: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), String> {
    for entry in fs::read_dir(directory)
        .map_err(|error| format!("Unable to inspect static build: {error}"))?
    {
        let entry = entry.map_err(|error| format!("Unable to inspect static build: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("Unable to inspect static build entry: {error}"))?;
        let relative = path
            .strip_prefix(build_root)
            .map_err(|_| "Static build path escaped its root".to_string())?;
        portable_relative_path(relative)?;
        if file_type.is_symlink() {
            return Err(format!(
                "Static builds cannot contain symbolic links: {}",
                relative.display()
            ));
        }
        if file_type.is_dir() {
            collect_static_build_files(build_root, &path, files)?;
        } else if file_type.is_file()
            && relative != Path::new("build-manifest.json")
            && relative != Path::new("release.json")
        {
            files.push(path);
        }
    }
    Ok(())
}

fn validate_static_image(
    build_root: &Path,
    relative: &str,
    expected: Option<(u32, u32)>,
    label: &str,
) -> Result<(u32, u32), String> {
    let clean = portable_relative_path(Path::new(relative))?;
    let path = build_root.join(&clean);
    if !path.is_file() {
        return Err(format!("Static build {label} is missing: {clean}"));
    }
    let (width, height) = image::image_dimensions(&path)
        .map_err(|error| format!("Static build {label} is not a decodable image: {error}"))?;
    if width == 0 || height == 0 {
        return Err(format!("Static build {label} has empty dimensions."));
    }
    if let Some((expected_width, expected_height)) = expected {
        if (width, height) != (expected_width, expected_height) {
            return Err(format!(
                "Static build {label} decoded at {width} × {height}; expected {expected_width} × {expected_height}."
            ));
        }
    }
    Ok((width, height))
}

fn static_texture_memory(width: u32, height: u32) -> Result<u64, String> {
    if width != height.saturating_mul(2) {
        return Err(format!(
            "Panorama texture must decode at 2:1, received {width} × {height}."
        ));
    }
    let bytes = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|pixels| pixels.checked_mul(4))
        .ok_or_else(|| "Panorama texture memory calculation overflowed.".to_string())?;
    if bytes > 192 * 1024 * 1024 {
        return Err(format!(
            "Panorama texture requires {bytes} decoded bytes, above the 192 MiB release limit."
        ));
    }
    Ok(bytes)
}

fn expected_static_tier_dimensions(kind: &str) -> Result<(u32, u32), String> {
    match kind {
        "mobile" => Ok((2048, 1024)),
        "4k" => Ok((4096, 2048)),
        "8k" => Ok((8192, 4096)),
        _ => Err(format!(
            "Unsupported or blurry panorama texture tier: {kind}."
        )),
    }
}

fn validate_static_build(build_root: &Path) -> Result<StaticBuildReport, String> {
    let manifest_path = build_root.join("tour-manifest.json");
    let manifest_bytes = fs::read(&manifest_path)
        .map_err(|error| format!("Static tour manifest is unavailable: {error}"))?;
    let tour: serde_json::Value = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| format!("Static tour manifest is invalid: {error}"))?;
    if tour.get("schemaVersion").and_then(|value| value.as_u64()) != Some(1) {
        return Err("Static tour manifest schema is unsupported.".into());
    }
    let loader = fs::read_to_string(build_root.join("tour-manifest.js"))
        .map_err(|error| format!("Local static manifest loader is unavailable: {error}"))?;
    let loader_json = loader
        .strip_prefix("globalThis.__ESTATE_TOUR_MANIFEST__ = ")
        .and_then(|value| value.strip_suffix(";\n"))
        .ok_or_else(|| "Local static manifest loader has an invalid envelope.".to_string())?;
    let loader_value: serde_json::Value = serde_json::from_str(loader_json)
        .map_err(|error| format!("Local static manifest loader is invalid: {error}"))?;
    if loader_value != tour {
        return Err("JSON and local JavaScript tour manifests do not match.".into());
    }
    let units = tour
        .get("units")
        .and_then(|value| value.as_array())
        .filter(|units| !units.is_empty())
        .ok_or_else(|| "Static tour manifest has no units.".to_string())?;
    let mut referenced = HashSet::new();
    let mut max_texture_memory_bytes = 0_u64;
    let mut room_ids = HashSet::new();
    for unit in units {
        let unit_label = unit
            .get("label")
            .and_then(|value| value.as_str())
            .unwrap_or("Unnamed unit");
        let floorplan = unit
            .get("floorplan")
            .and_then(|value| value.as_str())
            .ok_or_else(|| format!("{unit_label} has no floor plan path."))?;
        referenced.insert(portable_relative_path(Path::new(floorplan))?);
        let floorplan_path = build_root.join(floorplan);
        if !floorplan_path.is_file() {
            return Err(format!("{unit_label} floor plan path is broken."));
        }
        if floorplan_path.extension().and_then(|value| value.to_str()) != Some("svg") {
            validate_static_image(build_root, floorplan, None, "floor plan")?;
        }
        let rooms = unit
            .get("rooms")
            .and_then(|value| value.as_array())
            .filter(|rooms| !rooms.is_empty())
            .ok_or_else(|| format!("{unit_label} has no tour rooms."))?;
        let unit_room_ids = rooms
            .iter()
            .filter_map(|room| room.get("id").and_then(|value| value.as_str()))
            .collect::<HashSet<_>>();
        for room in rooms {
            let room_id = room
                .get("id")
                .and_then(|value| value.as_str())
                .ok_or_else(|| format!("{unit_label} contains a room without an ID."))?;
            if !room_ids.insert(format!("{unit_label}/{room_id}")) {
                return Err(format!(
                    "{unit_label} contains duplicate room ID {room_id}."
                ));
            }
            if room
                .get("evidenceLabel")
                .and_then(|value| value.as_str())
                .is_none_or(|value| value.trim().is_empty())
            {
                return Err(format!("{unit_label} / {room_id} has no evidence label."));
            }
            for adjacent in room
                .get("adjacentRoomIds")
                .and_then(|value| value.as_array())
                .into_iter()
                .flatten()
                .filter_map(|value| value.as_str())
            {
                if !unit_room_ids.contains(adjacent) {
                    return Err(format!(
                        "{unit_label} / {room_id} points to missing adjacent room {adjacent}."
                    ));
                }
            }
            let panorama = room
                .get("panorama")
                .ok_or_else(|| format!("{unit_label} / {room_id} has no panorama."))?;
            let source = panorama
                .get("source")
                .and_then(|value| value.as_str())
                .ok_or_else(|| format!("{unit_label} / {room_id} has no panorama source."))?;
            referenced.insert(portable_relative_path(Path::new(source))?);
            let (source_width, source_height) =
                validate_static_image(build_root, source, None, "panorama source")?;
            if source_width < 2048 {
                return Err(format!(
                    "{unit_label} / {room_id} panorama source is below the 2048 px release floor."
                ));
            }
            max_texture_memory_bytes =
                max_texture_memory_bytes.max(static_texture_memory(source_width, source_height)?);
            let tiers = panorama
                .get("tiers")
                .and_then(|value| value.as_array())
                .ok_or_else(|| format!("{unit_label} / {room_id} has no texture tiers."))?;
            let mut tier_kinds = HashSet::new();
            for tier in tiers {
                let kind = tier
                    .get("kind")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| "Texture tier has no kind.".to_string())?;
                let path = tier
                    .get("path")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| format!("{kind} texture has no path."))?;
                let declared_width = tier
                    .get("width")
                    .and_then(|value| value.as_u64())
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or_else(|| format!("{kind} texture has no valid width."))?;
                let declared_height = tier
                    .get("height")
                    .and_then(|value| value.as_u64())
                    .and_then(|value| u32::try_from(value).ok())
                    .ok_or_else(|| format!("{kind} texture has no valid height."))?;
                let expected = expected_static_tier_dimensions(kind)?;
                if (declared_width, declared_height) != expected {
                    return Err(format!(
                        "{kind} texture declares {declared_width} × {declared_height}; expected {} × {}.",
                        expected.0, expected.1
                    ));
                }
                referenced.insert(portable_relative_path(Path::new(path))?);
                let (width, height) = validate_static_image(
                    build_root,
                    path,
                    Some((declared_width, declared_height)),
                    &format!("{kind} panorama texture"),
                )?;
                let declared_checksum = tier
                    .get("checksumSha256")
                    .and_then(|value| value.as_str())
                    .ok_or_else(|| format!("{kind} texture has no checksum."))?;
                if sha256_file(&build_root.join(path))? != declared_checksum {
                    return Err(format!(
                        "{kind} texture checksum does not match its manifest."
                    ));
                }
                max_texture_memory_bytes =
                    max_texture_memory_bytes.max(static_texture_memory(width, height)?);
                tier_kinds.insert(kind);
            }
            if !tier_kinds.contains("mobile") || !tier_kinds.contains("4k") {
                return Err(format!(
                    "{unit_label} / {room_id} requires mobile and 4K texture tiers."
                ));
            }
            let fallback = room
                .get("stillFallback")
                .and_then(|value| value.as_str())
                .ok_or_else(|| format!("{unit_label} / {room_id} has no still fallback."))?;
            referenced.insert(portable_relative_path(Path::new(fallback))?);
            validate_static_image(build_root, fallback, None, "still fallback")?;
            if let Some(video) = room.get("video").and_then(|value| value.as_str()) {
                let clean = portable_relative_path(Path::new(video))?;
                if !build_root.join(&clean).is_file() {
                    return Err(format!("{unit_label} / {room_id} video path is broken."));
                }
                referenced.insert(clean);
            }
        }
    }
    let mut paths = Vec::new();
    collect_static_build_files(build_root, build_root, &mut paths)?;
    let mut files = Vec::new();
    for path in paths {
        let relative = portable_relative_path(
            path.strip_prefix(build_root)
                .map_err(|_| "Static build inventory escaped its root".to_string())?,
        )?;
        scan_bundle_file_for_secrets(Path::new(&relative), &path)?;
        files.push(StaticBuildFileRecord {
            path: relative,
            size_bytes: path
                .metadata()
                .map_err(|error| format!("Unable to inspect static build file: {error}"))?
                .len(),
            checksum_sha256: sha256_file(&path)?,
        });
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    let inventory = files
        .iter()
        .map(|file| file.path.as_str())
        .collect::<HashSet<_>>();
    for reference in referenced {
        if !inventory.contains(reference.as_str()) {
            return Err(format!(
                "Static build reference is absent from inventory: {reference}"
            ));
        }
    }
    for required in [
        "index.html",
        "tour.css",
        "tour.js",
        "tour-manifest.json",
        "tour-manifest.js",
    ] {
        if !inventory.contains(required) {
            return Err(format!("Static build runtime file is missing: {required}"));
        }
    }
    Ok(StaticBuildReport {
        schema_version: 1,
        immutable: true,
        generated_at: unix_time(),
        manifest_checksum_sha256: format!("{:x}", Sha256::digest(&manifest_bytes)),
        max_texture_memory_bytes,
        validations: vec![
            "manifest_schema".into(),
            "project_relative_paths".into(),
            "room_graph_references".into(),
            "accepted_evidence_labels".into(),
            "decoded_asset_dimensions".into(),
            "texture_memory_limit".into(),
            "mobile_and_4k_tiers".into(),
            "still_fallback_paths".into(),
            "secret_scan".into(),
            "sha256_inventory".into(),
        ],
        files,
    })
}

fn copy_static_build(source: &Path, destination: &Path) -> Result<(), String> {
    if destination.exists() {
        return Err("Immutable static build destination already exists.".into());
    }
    fs::create_dir_all(destination)
        .map_err(|error| format!("Unable to prepare immutable build: {error}"))?;
    let mut paths = Vec::new();
    collect_static_build_files(source, source, &mut paths)?;
    for path in paths {
        let relative = path
            .strip_prefix(source)
            .map_err(|_| "Static build copy escaped its source root".to_string())?;
        let target = destination.join(relative);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Unable to prepare immutable build path: {error}"))?;
        }
        fs::copy(&path, &target)
            .map_err(|error| format!("Unable to copy immutable build file: {error}"))?;
    }
    Ok(())
}

#[tauri::command]
fn build_static_tour_preview(app: AppHandle, project_id: String) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let timestamp = unix_time();
    let build_root = root
        .join("website")
        .join("previews")
        .join(format!("preview-{timestamp}"));
    if build_root.exists() {
        return Err("Static tour preview destination already exists.".into());
    }
    fs::create_dir_all(build_root.join("assets"))
        .map_err(|error| format!("Unable to prepare static tour preview: {error}"))?;
    let result = (|| -> Result<serde_json::Value, String> {
        let mut units_json = Vec::new();
        for unit in &manifest.units {
            if !unit.room_graph_locked || unit.rooms.is_empty() {
                continue;
            }
            let floorplan_id = unit
                .floorplan_asset_id
                .as_ref()
                .ok_or_else(|| format!("{} has no floor plan.", unit.label))?;
            let floorplan_asset = manifest
                .assets
                .iter()
                .find(|asset| &asset.id == floorplan_id)
                .ok_or_else(|| format!("{} floor plan is unavailable.", unit.label))?;
            if !matches!(
                floorplan_asset.mime_type.as_str(),
                "image/png" | "image/jpeg" | "image/webp" | "image/svg+xml"
            ) {
                return Err(format!(
                    "{} floor plan requires an accepted web image derivative.",
                    unit.label
                ));
            }
            let floorplan =
                copy_static_tour_asset(&root, &build_root, floorplan_asset, "floor plan")?;
            let mut rooms_json = Vec::new();
            for room in &unit.rooms {
                let panorama_id = room.panorama_asset_id.as_ref().ok_or_else(|| {
                    format!("{} / {} has no accepted panorama.", unit.label, room.name)
                })?;
                let panorama_asset = manifest
                    .assets
                    .iter()
                    .find(|asset| &asset.id == panorama_id)
                    .ok_or_else(|| format!("{} panorama asset is unavailable.", room.name))?;
                let panorama_source =
                    copy_static_tour_asset(&root, &build_root, panorama_asset, "panorama")?;
                let mut tiers = Vec::new();
                for derivative in &panorama_asset.derivative_relative_paths {
                    let derivative_path = root.join(derivative);
                    if !derivative_path.is_file() {
                        return Err(format!("{} derivative is unavailable.", room.name));
                    }
                    let checksum = sha256_file(&derivative_path)?;
                    let extension = derivative_path
                        .extension()
                        .and_then(|value| value.to_str())
                        .unwrap_or("png");
                    let relative = format!("assets/{checksum}.{extension}");
                    let destination = build_root.join(&relative);
                    if !destination.exists() {
                        fs::copy(&derivative_path, &destination).map_err(|error| {
                            format!("Unable to copy panorama derivative: {error}")
                        })?;
                    }
                    let (width, height) =
                        image::image_dimensions(&derivative_path).map_err(|error| {
                            format!("Unable to decode panorama derivative: {error}")
                        })?;
                    let kind = if width >= 8192 {
                        "8k"
                    } else if width >= 4096 {
                        "4k"
                    } else if width >= 2048 {
                        "mobile"
                    } else {
                        "other"
                    };
                    tiers.push(serde_json::json!({ "kind": kind, "path": relative, "width": width, "height": height, "checksumSha256": checksum }));
                }
                let still_fallback = room
                    .still_fallback_asset_id
                    .as_ref()
                    .and_then(|id| manifest.assets.iter().find(|asset| &asset.id == id))
                    .map(|asset| {
                        copy_static_tour_asset(&root, &build_root, asset, "still fallback")
                    })
                    .transpose()?
                    // An accepted panorama is itself a decodable still image. Reuse the
                    // copied source when a separate fallback was not curated so WebGL
                    // failure remains usable without inventing or generating media.
                    .unwrap_or_else(|| panorama_source.clone());
                let video = room
                    .video_asset_id
                    .as_ref()
                    .and_then(|id| manifest.assets.iter().find(|asset| &asset.id == id))
                    .map(|asset| copy_static_tour_asset(&root, &build_root, asset, "room video"))
                    .transpose()?;
                let initial_yaw = manifest
                    .generation_jobs
                    .iter()
                    .flat_map(|job| job.outputs.iter())
                    .find(|output| format!("generated-{}", output.id) == *panorama_id)
                    .and_then(|output| output.panorama_qa.as_ref())
                    .map(|qa| qa.yaw_degrees)
                    .unwrap_or(0.0);
                rooms_json.push(serde_json::json!({
                    "id": room.id,
                    "name": room.name,
                    "adjacentRoomIds": room.adjacent_room_ids,
                    "hotspot": { "x": room.hotspot_x.unwrap_or(50.0), "y": room.hotspot_y.unwrap_or(50.0) },
                    "panorama": { "source": panorama_source, "tiers": tiers },
                    "video": video,
                    "stillFallback": still_fallback,
                    "initialYaw": initial_yaw,
                    "evidenceLabel": panorama_asset.evidence_class.replace('_', " "),
                }));
            }
            units_json.push(serde_json::json!({
                "id": unit.id,
                "label": unit.label,
                "summary": unit.summary,
                "floorplan": floorplan,
                "entranceRoomId": unit.entrance_room_id,
                "rooms": rooms_json,
            }));
        }
        if units_json.is_empty() {
            return Err("No locked unit has complete accepted tour media.".into());
        }
        Ok(serde_json::json!({
            "schemaVersion": 1,
            "generatedAt": timestamp,
            "project": {
                "id": manifest.project_id,
                "name": manifest.name,
                "company": manifest.company,
                "location": manifest.location,
                "locale": manifest.locale,
                "measurementUnits": manifest.measurement_units,
                "disclosure": manifest.disclosure,
                "accessMode": manifest.access_mode,
            },
            "theme": { "primary": manifest.brand.primary, "accent": manifest.brand.accent },
            "analytics": { "schemaVersion": 1, "enabled": false, "endpoint": null, "privacy": "minimal_no_pii" },
            "units": units_json,
        }))
    })();
    let tour_manifest = match result {
        Ok(value) => value,
        Err(error) => {
            let _ = fs::remove_dir_all(&build_root);
            return Err(error);
        }
    };
    let finalized = (|| -> Result<PathBuf, String> {
        for (name, contents) in [
            (
                "index.html",
                include_str!("../../resources/tour-runtime/index.html"),
            ),
            (
                "tour.css",
                include_str!("../../resources/tour-runtime/tour.css"),
            ),
            (
                "tour.js",
                include_str!("../../resources/tour-runtime/tour.js"),
            ),
        ] {
            fs::write(build_root.join(name), contents)
                .map_err(|error| format!("Unable to write static tour runtime: {error}"))?;
        }
        fs::write(
            build_root.join("tour-manifest.json"),
            serde_json::to_vec_pretty(&tour_manifest).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to write static tour manifest: {error}"))?;
        fs::write(
            build_root.join("tour-manifest.js"),
            format!(
                "globalThis.__ESTATE_TOUR_MANIFEST__ = {};\n",
                serde_json::to_string(&tour_manifest).map_err(|error| error.to_string())?
            ),
        )
        .map_err(|error| format!("Unable to write local static tour manifest loader: {error}"))?;
        let report = validate_static_build(&build_root)?;
        fs::write(
            build_root.join("build-manifest.json"),
            serde_json::to_vec_pretty(&report).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to write static build inventory: {error}"))?;
        Ok(build_root.join("index.html"))
    })();
    let index_path = match finalized {
        Ok(path) => path,
        Err(error) => {
            let _ = fs::remove_dir_all(&build_root);
            return Err(error);
        }
    };
    manifest.tour_preview_url = Some(format!(
        "file://{}",
        index_path.to_string_lossy().replace(' ', "%20")
    ));
    manifest.modules.tour = "ready".into();
    manifest.updated_at = timestamp;
    record_funnel_event(
        &mut manifest,
        "local_preview_passed",
        "static_runtime_validation",
        timestamp,
    )?;
    if let Err(error) = write_manifest(&root, &manifest) {
        let _ = fs::remove_dir_all(&build_root);
        return Err(error);
    }
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn create_local_release(app: AppHandle, project_id: String) -> Result<ProjectRecord, String> {
    require_entitlement(&app, "static_export")?;
    require_local_role(&app, &["owner", "operator", "publisher"])?;
    let preview = build_static_tour_preview(app.clone(), project_id.clone())?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let preview_root = root
        .join("website")
        .join("previews")
        .join(format!("preview-{}", preview.manifest.updated_at));
    if !preview_root.is_dir() {
        return Err("Validated static preview directory is unavailable.".into());
    }
    let ready_units = manifest
        .units
        .iter()
        .filter(|unit| {
            unit.tour_available
                && unit.room_graph_locked
                && !unit.rooms.is_empty()
                && unit
                    .rooms
                    .iter()
                    .all(|room| room.panorama_status == "ready" && room.panorama_asset_id.is_some())
        })
        .map(|unit| unit.id.clone())
        .collect::<Vec<_>>();
    if ready_units.is_empty() {
        return Err("No unit has a locked room graph and accepted panorama for every room.".into());
    }
    let version = manifest
        .releases
        .iter()
        .map(|release| release.version)
        .max()
        .unwrap_or(0)
        + 1;
    let timestamp = unix_time();
    let release_id = format!("release-v{version}-{timestamp}");
    let release = ReleaseRecord {
        id: release_id.clone(),
        version,
        status: "preview_ready".into(),
        access_mode: "local".into(),
        created_at: timestamp,
        unit_ids: ready_units,
        public_url: None,
        verified_at: None,
        supersedes_release_id: None,
        superseded_by_release_id: None,
        rollback_from_release_id: None,
        unit_share_links: vec![],
        verification_qa_id: None,
    };
    let release_root = root.join("releases").join(&release_id);
    let temporary_root = root.join("releases").join(format!(".{release_id}.tmp"));
    if release_root.exists() || temporary_root.exists() {
        return Err("Immutable release destination already exists.".into());
    }
    let prepared = (|| -> Result<(), String> {
        let site_root = temporary_root.join("site");
        copy_static_build(&preview_root, &site_root)?;
        let build_report = validate_static_build(&site_root)?;
        fs::write(
            temporary_root.join("build-manifest.json"),
            serde_json::to_vec_pretty(&build_report).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to save immutable build inventory: {error}"))?;
        let snapshot = serde_json::json!({
            "release": release,
            "project": manifest,
            "build": build_report,
            "generatedAt": timestamp,
            "verification": { "localPreview": true, "publicReadback": false }
        });
        fs::write(
            temporary_root.join("release.json"),
            serde_json::to_vec_pretty(&snapshot).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to save release snapshot: {error}"))?;
        scan_bundle_file_for_secrets(
            Path::new("release.json"),
            &temporary_root.join("release.json"),
        )?;
        fs::rename(&temporary_root, &release_root)
            .map_err(|error| format!("Unable to finalize immutable release: {error}"))?;
        Ok(())
    })();
    if let Err(error) = prepared {
        if temporary_root.exists() {
            let _ = fs::remove_dir_all(&temporary_root);
        }
        return Err(error);
    }
    manifest.releases.push(release);
    manifest.modules.deployment = "in_progress".into();
    manifest.status = "Local preview release ready · Public verification required".into();
    manifest.updated_at = timestamp;
    if let Err(error) = write_manifest(&root, &manifest) {
        return Err(format!(
            "Immutable release exists at {} but project index could not be updated: {error}",
            release_root.display()
        ));
    }
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn checked_release_site<'a>(
    root: &Path,
    manifest: &'a ProjectManifest,
    release_id: &str,
) -> Result<(&'a ReleaseRecord, PathBuf), String> {
    let release_id = clean_required(release_id, "Release ID")?;
    if portable_relative_path(Path::new(&release_id))? != release_id {
        return Err("Release ID is not a safe path component.".into());
    }
    let release = manifest
        .releases
        .iter()
        .find(|release| release.id == release_id)
        .ok_or_else(|| "Release is unavailable.".to_string())?;
    let site = root.join("releases").join(&release_id).join("site");
    validate_static_build(&site)?;
    Ok((release, site))
}

fn checked_external_destination(project_root: &Path, raw: &str) -> Result<PathBuf, String> {
    let destination = PathBuf::from(clean_required(raw, "Destination path")?);
    if !destination.is_absolute() {
        return Err("Export destination must be an absolute path.".into());
    }
    if destination.exists() {
        return Err("Export destination already exists; immutable exports never overwrite.".into());
    }
    let parent = destination
        .parent()
        .ok_or_else(|| "Export destination has no parent folder.".to_string())?;
    let canonical_parent = fs::canonicalize(parent)
        .map_err(|error| format!("Export parent folder is unavailable: {error}"))?;
    let canonical_project = fs::canonicalize(project_root)
        .map_err(|error| format!("Project root is unavailable: {error}"))?;
    if canonical_parent.starts_with(&canonical_project) {
        return Err("External exports cannot be written back inside the project workspace.".into());
    }
    Ok(destination)
}

fn finalize_exported_build(destination: &Path) -> Result<StaticBuildReport, String> {
    let report = validate_static_build(destination)?;
    fs::write(
        destination.join("build-manifest.json"),
        serde_json::to_vec_pretty(&report).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to write exported build inventory: {error}"))?;
    Ok(report)
}

fn set_static_build_access_mode(destination: &Path, access_mode: &str) -> Result<(), String> {
    let path = destination.join("tour-manifest.json");
    let mut tour: serde_json::Value = serde_json::from_slice(
        &fs::read(&path).map_err(|error| format!("Unable to read exported manifest: {error}"))?,
    )
    .map_err(|error| format!("Unable to parse exported manifest: {error}"))?;
    tour.get_mut("project")
        .and_then(|project| project.as_object_mut())
        .ok_or_else(|| "Exported manifest project block is invalid.".to_string())?
        .insert("accessMode".into(), serde_json::json!(access_mode));
    fs::write(
        &path,
        serde_json::to_vec_pretty(&tour).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to update exported access manifest: {error}"))?;
    fs::write(
        destination.join("tour-manifest.js"),
        format!(
            "globalThis.__ESTATE_TOUR_MANIFEST__ = {};\n",
            serde_json::to_string(&tour).map_err(|error| error.to_string())?
        ),
    )
    .map_err(|error| format!("Unable to update exported local manifest loader: {error}"))?;
    let robots = destination.join("robots.txt");
    if access_mode == "public" {
        if robots.exists() {
            fs::remove_file(robots)
                .map_err(|error| format!("Unable to remove stale robots policy: {error}"))?;
        }
    } else {
        fs::write(robots, "User-agent: *\nDisallow: /\n")
            .map_err(|error| format!("Unable to write unlisted/private robots policy: {error}"))?;
    }
    Ok(())
}

fn validate_publish_access_mode(value: &str) -> Result<String, String> {
    let value = clean_required(value, "Access intent")?;
    if !["public", "unlisted", "private"].contains(&value.as_str()) {
        return Err("Choose public, unlisted, or private-link access intent.".into());
    }
    Ok(value)
}

#[tauri::command]
fn export_static_release(
    app: AppHandle,
    project_id: String,
    release_id: String,
    destination_path: String,
) -> Result<String, String> {
    require_entitlement(&app, "static_export")?;
    require_local_role(&app, &["owner", "operator", "publisher"])?;
    validate_project_id(&project_id)?;
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    let (release, source) = checked_release_site(&root, &manifest, &release_id)?;
    let destination = checked_external_destination(&root, &destination_path)?;
    if release.status == "failed" {
        return Err("Failed releases cannot be exported.".into());
    }
    if let Err(error) = copy_static_build(&source, &destination) {
        if destination.exists() {
            let _ = fs::remove_dir_all(&destination);
        }
        return Err(error);
    }
    if let Err(error) = finalize_exported_build(&destination) {
        let _ = fs::remove_dir_all(&destination);
        return Err(error);
    }
    Ok(destination.to_string_lossy().into_owned())
}

#[tauri::command]
fn publish_release_to_customer_directory(
    app: AppHandle,
    project_id: String,
    input: CustomerDirectoryPublishInput,
) -> Result<ProjectRecord, String> {
    require_entitlement(&app, "customer_directory_publish")?;
    require_local_role(&app, &["owner", "publisher"])?;
    if !input.confirmed_unverified_upload {
        return Err(
            "Confirm that directory copy is an unverified upload, not publication proof.".into(),
        );
    }
    let access_mode = validate_publish_access_mode(&input.access_mode)?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let (_, source) = checked_release_site(&root, &manifest, &input.release_id)?;
    let destination = checked_external_destination(&root, &input.destination_path)?;
    if let Err(error) = copy_static_build(&source, &destination) {
        if destination.exists() {
            let _ = fs::remove_dir_all(&destination);
        }
        return Err(error);
    }
    if let Err(error) = set_static_build_access_mode(&destination, &access_mode) {
        let _ = fs::remove_dir_all(&destination);
        return Err(error);
    }
    if let Err(error) = finalize_exported_build(&destination) {
        let _ = fs::remove_dir_all(&destination);
        return Err(error);
    }
    let timestamp = unix_time();
    if let Err(error) = fs::write(
        destination.join("estate-studio-upload.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "schemaVersion": 1,
            "adapter": "customer_owned_directory",
            "projectId": project_id,
            "releaseId": input.release_id,
            "accessIntent": access_mode,
            "copiedAt": timestamp,
            "status": "uploaded_unverified",
            "publicReadback": false
        }))
        .map_err(|error| error.to_string())?,
    ) {
        let _ = fs::remove_dir_all(&destination);
        return Err(format!(
            "Unable to record customer-directory upload: {error}"
        ));
    }
    let release = manifest
        .releases
        .iter_mut()
        .find(|release| release.id == input.release_id)
        .ok_or_else(|| "Release disappeared during upload.".to_string())?;
    release.status = "uploaded_unverified".into();
    release.access_mode = access_mode.clone();
    release.public_url = None;
    manifest.approval_events.push(ApprovalEvent {
        id: format!("approval-customer-directory-upload-{timestamp}"),
        subject_type: "release_upload".into(),
        subject_id: input.release_id,
        decision: "uploaded_unverified".into(),
        actor: "local_user".into(),
        reason: Some(format!(
            "customer_owned_directory adapter · {} intent · public read-back pending",
            access_mode
        )),
        created_at: timestamp,
    });
    manifest.access_mode = access_mode;
    manifest.modules.deployment = "in_progress".into();
    manifest.status = "Customer-owned directory uploaded · Public verification required".into();
    manifest.updated_at = timestamp;
    record_funnel_event(
        &mut manifest,
        "first_tour_published",
        "customer_owned_directory",
        timestamp,
    )?;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn validate_deployment_url(raw: &str) -> Result<Url, String> {
    let url = Url::parse(&clean_required(raw, "Deployment URL")?)
        .map_err(|_| "Deployment URL is invalid.".to_string())?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(
            "Deployment read-back requires an HTTPS URL without embedded credentials.".into(),
        );
    }
    let host = url.host_str().unwrap_or_default().to_ascii_lowercase();
    if host == "localhost" || host.ends_with(".localhost") {
        return Err("Deployment read-back cannot target localhost.".into());
    }
    if let Ok(address) = host.parse::<std::net::IpAddr>() {
        let blocked = match address {
            std::net::IpAddr::V4(value) => {
                value.is_private()
                    || value.is_loopback()
                    || value.is_link_local()
                    || value.is_unspecified()
            }
            std::net::IpAddr::V6(value) => {
                value.is_loopback()
                    || value.is_unique_local()
                    || value.is_unicast_link_local()
                    || value.is_unspecified()
            }
        };
        if blocked {
            return Err("Deployment read-back cannot target a private or local IP address.".into());
        }
    }
    Ok(url)
}

fn deployment_child_url(base: &Url, relative: &str, preserve_query: bool) -> Result<Url, String> {
    portable_relative_path(Path::new(relative))?;
    let mut url = base
        .join(relative)
        .map_err(|_| format!("Deployment asset URL is invalid: {relative}"))?;
    if preserve_query {
        url.set_query(base.query());
    }
    if url.host_str() != base.host_str() || url.scheme() != "https" {
        return Err("Deployment asset URL escaped the verified HTTPS origin.".into());
    }
    Ok(url)
}

async fn fetch_readback_bytes(
    client: &Client,
    url: Url,
    limit: usize,
    range: bool,
) -> Result<(reqwest::StatusCode, reqwest::header::HeaderMap, Vec<u8>), String> {
    let mut request = client.get(url);
    if range {
        request = request.header(reqwest::header::RANGE, "bytes=0-1048575");
    }
    let response = request
        .send()
        .await
        .map_err(|error| format!("Deployment read-back request failed: {error}"))?;
    let status = response.status();
    let headers = response.headers().clone();
    if !status.is_success() {
        return Err(format!("Deployment read-back returned HTTP {status}."));
    }
    if headers
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|size| size > limit)
    {
        return Err(format!(
            "Deployment read-back exceeded the {limit} byte inspection limit."
        ));
    }
    let bytes = response
        .bytes()
        .await
        .map_err(|error| format!("Unable to read deployment response: {error}"))?;
    if bytes.len() > limit {
        return Err(format!(
            "Deployment read-back exceeded the {limit} byte inspection limit."
        ));
    }
    Ok((status, headers, bytes.to_vec()))
}

#[tauri::command]
async fn verify_deployment_readback(
    app: AppHandle,
    project_id: String,
    input: DeploymentReadbackInput,
) -> Result<DeploymentReadbackResult, String> {
    require_entitlement(&app, "deployment_readback")?;
    require_local_role(&app, &["owner", "reviewer", "publisher"])?;
    let expected_access_mode = validate_publish_access_mode(&input.expected_access_mode)?;
    let base = validate_deployment_url(&input.base_url)?;
    let private_link = expected_access_mode == "private";
    if private_link && base.query().is_none() {
        return Err("Private-link verification requires an explicit access query supplied only for this check.".into());
    }
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|error| format!("Unable to prepare deployment read-back: {error}"))?;
    let (_, _, index_bytes) =
        fetch_readback_bytes(&client, base.clone(), 2 * 1024 * 1024, false).await?;
    let index = String::from_utf8(index_bytes)
        .map_err(|_| "Deployment index is not UTF-8 HTML.".to_string())?;
    if !index.contains("tour.js") || !index.contains("tour-manifest.js") {
        return Err("Deployment index does not contain the Estate Studio runtime.".into());
    }
    if private_link {
        let mut anonymous = base.clone();
        anonymous.set_query(None);
        let anonymous_status = client
            .get(anonymous)
            .send()
            .await
            .map_err(|error| format!("Private-link anonymous access check failed: {error}"))?
            .status();
        if anonymous_status.is_success() {
            return Err(
                "Private-link deployment is anonymously accessible without its access query."
                    .into(),
            );
        }
    }
    let robots_url = deployment_child_url(&base, "robots.txt", private_link)?;
    let robots_response = client
        .get(robots_url)
        .send()
        .await
        .map_err(|error| format!("Deployment robots policy check failed: {error}"))?;
    if expected_access_mode == "public" {
        if robots_response.status().is_success()
            && robots_response
                .text()
                .await
                .unwrap_or_default()
                .to_ascii_lowercase()
                .contains("disallow: /")
        {
            return Err("Public deployment still blocks indexing in robots.txt.".into());
        }
    } else if !robots_response.status().is_success()
        || !robots_response
            .text()
            .await
            .unwrap_or_default()
            .to_ascii_lowercase()
            .contains("disallow: /")
    {
        return Err(
            "Unlisted/private deployment is missing its robots.txt disallow policy.".into(),
        );
    }
    let manifest_url = deployment_child_url(&base, "tour-manifest.json", private_link)?;
    let (_, _, manifest_bytes) =
        fetch_readback_bytes(&client, manifest_url, 2 * 1024 * 1024, false).await?;
    let tour: serde_json::Value = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| format!("Deployed tour manifest is invalid: {error}"))?;
    if tour.get("schemaVersion").and_then(|value| value.as_u64()) != Some(1) {
        return Err("Deployed tour manifest schema is unsupported.".into());
    }
    let project = tour
        .get("project")
        .ok_or_else(|| "Deployed tour has no project block.".to_string())?;
    if project.get("id").and_then(|value| value.as_str()) != Some(project_id.as_str()) {
        return Err("Deployed tour belongs to a different project.".into());
    }
    if project.get("accessMode").and_then(|value| value.as_str())
        != Some(expected_access_mode.as_str())
    {
        return Err(
            "Deployed manifest access mode does not match the intended release mode.".into(),
        );
    }
    let disclosure = project
        .get("disclosure")
        .and_then(|value| value.as_str())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "Deployed disclosure is missing.".to_string())?;
    let units = tour
        .get("units")
        .and_then(|value| value.as_array())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Deployed tour has no units.".to_string())?;
    let representative_unit = &units[0];
    let unit_id = representative_unit
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Representative deployed unit has no ID.".to_string())?;
    let floorplan = representative_unit
        .get("floorplan")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Representative deployed floor plan is missing.".to_string())?;
    let (_, floorplan_headers, floorplan_bytes) = fetch_readback_bytes(
        &client,
        deployment_child_url(&base, floorplan, private_link)?,
        20 * 1024 * 1024,
        false,
    )
    .await?;
    if floorplan_bytes.is_empty() {
        return Err("Representative deployed floor plan is empty.".into());
    }
    let floorplan_type = floorplan_headers
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if !floorplan_type.contains("svg") {
        image::load_from_memory(&floorplan_bytes)
            .map_err(|error| format!("Representative floor plan cannot be decoded: {error}"))?;
    }
    let rooms = representative_unit
        .get("rooms")
        .and_then(|value| value.as_array())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Representative deployed unit has no rooms.".to_string())?;
    let representative_room = &rooms[0];
    let room_id = representative_room
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Representative deployed room has no ID.".to_string())?;
    let mobile = representative_room
        .pointer("/panorama/tiers")
        .and_then(|value| value.as_array())
        .and_then(|tiers| {
            tiers
                .iter()
                .find(|tier| tier.get("kind").and_then(|value| value.as_str()) == Some("mobile"))
        })
        .ok_or_else(|| "Representative deployed room has no mobile panorama tier.".to_string())?;
    let mobile_path = mobile
        .get("path")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Representative mobile panorama path is missing.".to_string())?;
    let (_, _, mobile_bytes) = fetch_readback_bytes(
        &client,
        deployment_child_url(&base, mobile_path, private_link)?,
        32 * 1024 * 1024,
        false,
    )
    .await?;
    let mobile_image = image::load_from_memory(&mobile_bytes)
        .map_err(|error| format!("Representative mobile panorama cannot be decoded: {error}"))?;
    if (mobile_image.width(), mobile_image.height()) != (2048, 1024) {
        return Err(format!(
            "Representative mobile panorama decoded at {} × {}; expected 2048 × 1024.",
            mobile_image.width(),
            mobile_image.height()
        ));
    }
    let fallback = representative_room
        .get("stillFallback")
        .and_then(|value| value.as_str())
        .ok_or_else(|| "Representative deployed still fallback is missing.".to_string())?;
    let (_, _, fallback_bytes) = fetch_readback_bytes(
        &client,
        deployment_child_url(&base, fallback, private_link)?,
        20 * 1024 * 1024,
        false,
    )
    .await?;
    image::load_from_memory(&fallback_bytes)
        .map_err(|error| format!("Representative still fallback cannot be decoded: {error}"))?;
    let mut video_checked = false;
    if let Some(video) = representative_room
        .get("video")
        .and_then(|value| value.as_str())
    {
        let (status, _, bytes) = fetch_readback_bytes(
            &client,
            deployment_child_url(&base, video, private_link)?,
            2 * 1024 * 1024,
            true,
        )
        .await?;
        if bytes.is_empty()
            || !(status == reqwest::StatusCode::PARTIAL_CONTENT || status.is_success())
        {
            return Err("Representative deployed video is not readable.".into());
        }
        video_checked = true;
    }
    for room in rooms {
        let candidate = room
            .get("id")
            .and_then(|value| value.as_str())
            .ok_or_else(|| "Scripted navigation found a room without an ID.".to_string())?;
        let mut navigation = base.clone();
        navigation.set_query(None);
        navigation
            .query_pairs_mut()
            .append_pair("unit", unit_id)
            .append_pair("room", candidate)
            .append_pair("mode", "panorama");
        if private_link {
            for (key, value) in base.query_pairs() {
                navigation.query_pairs_mut().append_pair(&key, &value);
            }
        }
        let (_, _, bytes) =
            fetch_readback_bytes(&client, navigation, 2 * 1024 * 1024, false).await?;
        if !String::from_utf8_lossy(&bytes).contains("tour.js") {
            return Err(format!("Scripted navigation failed for room {candidate}."));
        }
    }
    let (root, mut local_manifest) = editable_project(&app, &project_id)?;
    if local_manifest.disclosure.trim() != disclosure.trim() {
        return Err("Deployed disclosure does not match the local approved disclosure.".into());
    }
    let release = local_manifest
        .releases
        .iter_mut()
        .find(|release| release.id == input.release_id)
        .ok_or_else(|| "Read-back release is unavailable locally.".to_string())?;
    if release.access_mode != expected_access_mode {
        return Err("Local release access intent changed before read-back completed.".into());
    }
    release.status = "readback_passed".into();
    if !private_link {
        let mut stored = base.clone();
        stored.set_query(None);
        stored.set_fragment(None);
        release.public_url = Some(stored.to_string());
    }
    let timestamp = unix_time();
    let checks = vec![
        "logged_out_index=passed".into(),
        format!("access_mode={expected_access_mode}"),
        "manifest_project=matched".into(),
        "approved_disclosure=matched".into(),
        "representative_floorplan=decoded".into(),
        format!("representative_room={room_id}"),
        "representative_mobile_panorama=2048x1024".into(),
        "representative_still_fallback=decoded".into(),
        format!(
            "representative_video={}",
            if video_checked {
                "range_read"
            } else {
                "not_configured"
            }
        ),
        format!("scripted_navigation_rooms={}", rooms.len()),
        "public_verification=not_finalized".into(),
    ];
    local_manifest.qa_records.push(QaRecord {
        id: format!("qa-deployment-readback-{}-{timestamp}", input.release_id),
        scope: format!("project:{project_id}:release:{}", input.release_id),
        status: "passed".into(),
        checks: checks.clone(),
        reviewer: "automated_logged_out_readback".into(),
        created_at: timestamp,
    });
    local_manifest.status =
        "Deployment read-back passed · Final release verification pending".into();
    local_manifest.updated_at = timestamp;
    write_manifest(&root, &local_manifest)?;
    Ok(DeploymentReadbackResult {
        project: ProjectRecord {
            manifest: local_manifest,
            project_root: root.to_string_lossy().into_owned(),
        },
        checks,
    })
}

fn qr_svg(value: &str) -> Result<String, String> {
    let code = QrCode::new(value.as_bytes())
        .map_err(|error| format!("Unable to encode unit share QR: {error}"))?;
    Ok(code
        .render::<svg::Color>()
        .min_dimensions(320, 320)
        .dark_color(svg::Color("#171715"))
        .light_color(svg::Color("#ffffff"))
        .build())
}

fn private_release_links_root(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve private release storage: {error}"))?
        .join("private-release-links");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create private release storage: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Unable to protect private release storage: {error}"))?;
    }
    Ok(root)
}

fn protect_private_file(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|error| format!("Unable to protect private release file: {error}"))?;
    }
    Ok(())
}

const LICENSE_PUBLIC_KEY_HEX: &str =
    "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a";

fn decode_hex_32(value: &str) -> Result<[u8; 32], String> {
    if value.len() != 64 {
        return Err("Licence public key has an invalid length.".into());
    }
    let mut bytes = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let text = std::str::from_utf8(pair).map_err(|_| "Licence public key is invalid.")?;
        bytes[index] = u8::from_str_radix(text, 16)
            .map_err(|_| "Licence public key is invalid.".to_string())?;
    }
    Ok(bytes)
}

fn edition_entitlements(edition: &str) -> Option<&'static [&'static str]> {
    match edition {
        "starter" => Some(&["static_export", "portable_export", "local_roles"]),
        "professional" => Some(&[
            "static_export",
            "portable_export",
            "local_roles",
            "managed_generation",
            "customer_directory_publish",
            "deployment_readback",
            "verified_release",
            "rollback",
            "analytics_endpoint",
        ]),
        "enterprise" => Some(&[
            "static_export",
            "portable_export",
            "local_roles",
            "managed_generation",
            "customer_directory_publish",
            "deployment_readback",
            "verified_release",
            "rollback",
            "analytics_endpoint",
            "private_links",
            "signed_updates",
        ]),
        _ => None,
    }
}

fn verify_license_document(
    document: &SignedLicenseDocument,
    now: u64,
) -> Result<LicenseClaims, String> {
    let claims = &document.claims;
    if claims.schema_version != 1 {
        return Err("Licence schema is unsupported.".into());
    }
    if clean_required(&claims.license_id, "Licence ID")? != claims.license_id
        || clean_required(&claims.customer, "Licence customer")? != claims.customer
    {
        return Err("Licence identity fields are not canonical.".into());
    }
    if claims.issued_at > now.saturating_add(300) {
        return Err("Licence issue time is in the future.".into());
    }
    if claims.expires_at.is_some_and(|expires_at| now > expires_at) {
        return Err("Licence has expired.".into());
    }
    let allowed = edition_entitlements(&claims.edition)
        .ok_or_else(|| "Licence edition is unsupported.".to_string())?;
    let mut unique_entitlements = HashSet::new();
    for entitlement in &claims.entitlements {
        if !allowed.contains(&entitlement.as_str()) || !unique_entitlements.insert(entitlement) {
            return Err(format!(
                "Licence entitlement is invalid for this edition: {entitlement}."
            ));
        }
    }
    let allowed_roles = ["owner", "operator", "reviewer", "publisher"];
    let mut unique_roles = HashSet::new();
    for role in &claims.allowed_roles {
        if !allowed_roles.contains(&role.as_str()) || !unique_roles.insert(role) {
            return Err(format!("Licence local role is invalid: {role}."));
        }
    }
    if !claims.allowed_roles.iter().any(|role| role == "owner") {
        return Err("Licence must allow one local owner role.".into());
    }
    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(&document.signature_base64)
        .map_err(|_| "Licence signature is not valid base64.".to_string())?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_| "Licence signature length is invalid.".to_string())?;
    let key = VerifyingKey::from_bytes(&decode_hex_32(LICENSE_PUBLIC_KEY_HEX)?)
        .map_err(|_| "Embedded licence public key is invalid.".to_string())?;
    let payload = serde_json::to_vec(claims)
        .map_err(|error| format!("Unable to encode licence claims: {error}"))?;
    key.verify_strict(&payload, &signature)
        .map_err(|_| "Licence signature verification failed.".to_string())?;
    Ok(claims.clone())
}

fn license_root(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve licence storage: {error}"))?
        .join("licence");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create licence storage: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Unable to protect licence storage: {error}"))?;
    }
    Ok(root)
}

fn read_role_registry(root: &Path) -> Option<LocalRoleRegistry> {
    fs::read(root.join("roles.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
}

fn verified_license_claims(app: &AppHandle) -> Result<LicenseClaims, String> {
    let root = license_root(app)?;
    let bytes = fs::read(root.join("license.json"))
        .map_err(|_| "No signed Estate Studio licence is installed.".to_string())?;
    let document: SignedLicenseDocument = serde_json::from_slice(&bytes)
        .map_err(|error| format!("Installed licence is invalid JSON: {error}"))?;
    verify_license_document(&document, unix_time())
}

fn require_entitlement(app: &AppHandle, entitlement: &str) -> Result<LicenseClaims, String> {
    let claims = verified_license_claims(app)?;
    if !claims.entitlements.iter().any(|item| item == entitlement) {
        return Err(format!(
            "The {} edition does not include the {entitlement} entitlement.",
            claims.edition
        ));
    }
    Ok(claims)
}

fn require_local_role(app: &AppHandle, roles: &[&str]) -> Result<String, String> {
    let root = license_root(app)?;
    let claims = verified_license_claims(app)?;
    let active = read_role_registry(&root)
        .map(|registry| registry.active_role)
        .unwrap_or_else(|| "owner".into());
    if !claims.allowed_roles.contains(&active) || !roles.contains(&active.as_str()) {
        return Err(format!(
            "Active local role {active} cannot perform this action; required role: {}.",
            roles.join(" or ")
        ));
    }
    Ok(active)
}

#[tauri::command]
fn get_license_status(app: AppHandle) -> Result<LicenseStatus, String> {
    let root = license_root(&app)?;
    let path = root.join("license.json");
    if !path.is_file() {
        return Ok(LicenseStatus {
            installed: false,
            valid: false,
            detail: "No signed licence is installed. Local project reading remains available; commercial actions are blocked.".into(),
            license_id: None,
            customer: None,
            edition: None,
            expires_at: None,
            entitlements: vec![],
            allowed_roles: vec![],
            active_role: None,
        });
    }
    let document: SignedLicenseDocument = match fs::read(&path)
        .map_err(|error| format!("Unable to read installed licence: {error}"))
        .and_then(|bytes| {
            serde_json::from_slice(&bytes)
                .map_err(|error| format!("Installed licence is invalid JSON: {error}"))
        }) {
        Ok(document) => document,
        Err(detail) => {
            return Ok(LicenseStatus {
                installed: true,
                valid: false,
                detail,
                license_id: None,
                customer: None,
                edition: None,
                expires_at: None,
                entitlements: vec![],
                allowed_roles: vec![],
                active_role: None,
            });
        }
    };
    match verify_license_document(&document, unix_time()) {
        Ok(claims) => Ok(LicenseStatus {
            installed: true,
            valid: true,
            detail: "Offline Ed25519 signature and expiry checks passed.".into(),
            license_id: Some(claims.license_id),
            customer: Some(claims.customer),
            edition: Some(claims.edition),
            expires_at: claims.expires_at,
            entitlements: claims.entitlements,
            allowed_roles: claims.allowed_roles,
            active_role: read_role_registry(&root)
                .map(|registry| registry.active_role)
                .or(Some("owner".into())),
        }),
        Err(detail) => Ok(LicenseStatus {
            installed: true,
            valid: false,
            detail,
            license_id: Some(document.claims.license_id),
            customer: Some(document.claims.customer),
            edition: Some(document.claims.edition),
            expires_at: document.claims.expires_at,
            entitlements: vec![],
            allowed_roles: vec![],
            active_role: None,
        }),
    }
}

#[tauri::command]
fn install_signed_license(app: AppHandle, input_path: String) -> Result<LicenseStatus, String> {
    let source = fs::canonicalize(clean_required(&input_path, "Licence file")?)
        .map_err(|error| format!("Unable to read selected licence: {error}"))?;
    if !source.is_file() {
        return Err("Selected licence is not a file.".into());
    }
    let bytes =
        fs::read(&source).map_err(|error| format!("Unable to read selected licence: {error}"))?;
    if bytes.len() > 256 * 1024 {
        return Err("Licence file exceeds the 256 KiB limit.".into());
    }
    let document: SignedLicenseDocument = serde_json::from_slice(&bytes)
        .map_err(|error| format!("Selected licence is invalid JSON: {error}"))?;
    let claims = verify_license_document(&document, unix_time())?;
    let root = license_root(&app)?;
    let temporary = root.join(".license.json.tmp");
    fs::write(&temporary, &bytes)
        .map_err(|error| format!("Unable to stage signed licence: {error}"))?;
    protect_private_file(&temporary)?;
    let installed = root.join("license.json");
    let backup = root.join(".license.json.backup");
    if backup.exists() {
        let _ = fs::remove_file(&temporary);
        return Err("A previous licence replacement backup requires recovery.".into());
    }
    if installed.exists() {
        fs::rename(&installed, &backup)
            .map_err(|error| format!("Unable to preserve installed licence: {error}"))?;
    }
    if let Err(error) = fs::rename(&temporary, &installed) {
        if backup.exists() {
            let _ = fs::rename(&backup, &installed);
        }
        return Err(format!("Unable to install signed licence: {error}"));
    }
    if backup.exists() {
        fs::remove_file(&backup).map_err(|error| {
            format!("New licence installed but old backup could not be removed: {error}")
        })?;
    }
    if read_role_registry(&root).is_none() {
        let registry = LocalRoleRegistry {
            schema_version: 1,
            active_role: "owner".into(),
            assignments: vec![LocalRoleAssignment {
                role: "owner".into(),
                display_name: claims.customer,
                assigned_at: unix_time(),
            }],
        };
        let path = root.join("roles.json");
        fs::write(
            &path,
            serde_json::to_vec_pretty(&registry).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to create local role record: {error}"))?;
        protect_private_file(&path)?;
    }
    get_license_status(app)
}

#[tauri::command]
fn set_active_local_role(
    app: AppHandle,
    role: String,
    display_name: String,
) -> Result<LicenseStatus, String> {
    let claims = require_entitlement(&app, "local_roles")?;
    let role = clean_required(&role, "Local role")?;
    if !claims.allowed_roles.contains(&role) {
        return Err("This signed licence does not allow the selected local role.".into());
    }
    let display_name = clean_required(&display_name, "Role display name")?;
    let root = license_root(&app)?;
    let mut registry = read_role_registry(&root).unwrap_or(LocalRoleRegistry {
        schema_version: 1,
        active_role: role.clone(),
        assignments: vec![],
    });
    registry.schema_version = 1;
    registry.active_role = role.clone();
    if let Some(assignment) = registry
        .assignments
        .iter_mut()
        .find(|item| item.role == role)
    {
        assignment.display_name = display_name;
    } else {
        registry.assignments.push(LocalRoleAssignment {
            role,
            display_name,
            assigned_at: unix_time(),
        });
    }
    let path = root.join("roles.json");
    fs::write(
        &path,
        serde_json::to_vec_pretty(&registry).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to save local role record: {error}"))?;
    protect_private_file(&path)?;
    get_license_status(app)
}

fn desktop_settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve desktop settings storage: {error}"))?;
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to prepare desktop settings storage: {error}"))?;
    Ok(root.join("desktop-settings.json"))
}

fn default_desktop_settings(app: &AppHandle) -> Result<DesktopSettings, String> {
    Ok(DesktopSettings {
        schema_version: 1,
        storage_mode: "app_data_managed".into(),
        storage_path: projects_root(app)?.to_string_lossy().into_owned(),
        update_channel: "stable".into(),
        auto_check_updates: true,
        managed_gateway_base_url: None,
        analytics_endpoint: None,
        codex_auth_mode: "official_local_codex".into(),
        publishing_targets: vec![],
        secret_references: vec![],
        updated_at: unix_time(),
    })
}

fn read_desktop_settings(app: &AppHandle) -> Result<DesktopSettings, String> {
    let path = desktop_settings_path(app)?;
    if !path.is_file() {
        return default_desktop_settings(app);
    }
    let mut settings: DesktopSettings = serde_json::from_slice(
        &fs::read(&path).map_err(|error| format!("Unable to read desktop settings: {error}"))?,
    )
    .map_err(|error| format!("Desktop settings are invalid: {error}"))?;
    if settings.schema_version != 1 {
        return Err("Desktop settings schema is unsupported.".into());
    }
    settings.storage_mode = "app_data_managed".into();
    settings.storage_path = projects_root(app)?.to_string_lossy().into_owned();
    settings.codex_auth_mode = "official_local_codex".into();
    Ok(settings)
}

fn normalize_managed_gateway_url(value: &str) -> Result<String, String> {
    let mut url =
        Url::parse(value.trim()).map_err(|_| "Managed subscription URL is invalid.".to_string())?;
    if url.scheme() != "https"
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err("Managed subscription URL must be credential-free HTTPS.".into());
    }
    let normalized_path = url.path().trim_end_matches('/').to_string();
    url.set_path(&normalized_path);
    Ok(url.to_string().trim_end_matches('/').to_string())
}

fn normalize_analytics_endpoint(value: &str) -> Result<String, String> {
    let value = value.trim();
    let path = Path::new(value);
    if !value.starts_with('/')
        || value.starts_with("//")
        || value.contains('?')
        || value.contains('#')
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(
            "Analytics endpoint must be a same-origin absolute path without query or traversal."
                .into(),
        );
    }
    Ok(value.to_string())
}

fn validate_desktop_settings(
    app: &AppHandle,
    input: UpdateDesktopSettingsInput,
) -> Result<DesktopSettings, String> {
    if !["stable", "beta"].contains(&input.update_channel.as_str()) {
        return Err("Update channel must be stable or beta.".into());
    }
    let managed_gateway_base_url = input
        .managed_gateway_base_url
        .filter(|value| !value.trim().is_empty())
        .map(|value| normalize_managed_gateway_url(&value))
        .transpose()?;
    let analytics_endpoint = input
        .analytics_endpoint
        .filter(|value| !value.trim().is_empty())
        .map(|value| normalize_analytics_endpoint(&value))
        .transpose()?;
    let projects = fs::canonicalize(projects_root(app)?)
        .map_err(|error| format!("Unable to validate project storage: {error}"))?;
    let mut target_ids = HashSet::new();
    let mut target_paths = HashSet::new();
    let mut publishing_targets = Vec::new();
    for target in input.publishing_targets {
        let id = slugify(&clean_required(&target.id, "Publishing target ID")?);
        let label = clean_required(&target.label, "Publishing target label")?;
        if target.kind != "customer_owned_directory" {
            return Err(
                "Only the customer-owned directory publishing adapter is supported.".into(),
            );
        }
        let root = fs::canonicalize(clean_required(&target.root_path, "Publishing target root")?)
            .map_err(|error| format!("Publishing target root is unavailable: {error}"))?;
        if !root.is_dir() || root.starts_with(&projects) {
            return Err(
                "Publishing target must be an existing directory outside project storage.".into(),
            );
        }
        let root_path = root.to_string_lossy().into_owned();
        if !target_ids.insert(id.clone()) || !target_paths.insert(root_path.clone()) {
            return Err("Publishing target IDs and roots must be unique.".into());
        }
        publishing_targets.push(PublishingTargetSetting {
            id,
            label,
            kind: target.kind,
            root_path,
        });
    }
    let allowed_secret_kinds = ["hosting_token_file", "sftp_key", "analytics_token_file"];
    let mut secret_ids = HashSet::new();
    let mut secret_paths = HashSet::new();
    let mut secret_references = Vec::new();
    for reference in input.secret_references {
        let id = slugify(&clean_required(&reference.id, "Secret reference ID")?);
        let label = clean_required(&reference.label, "Secret reference label")?;
        if !allowed_secret_kinds.contains(&reference.kind.as_str()) {
            return Err("Secret reference kind is unsupported.".into());
        }
        let path = fs::canonicalize(clean_required(
            &reference.file_path,
            "Secret reference file",
        )?)
        .map_err(|error| format!("Referenced secret file is unavailable: {error}"))?;
        if !path.is_file() || path.starts_with(&projects) {
            return Err(
                "Secret reference must point to an existing file outside project storage.".into(),
            );
        }
        let file_path = path.to_string_lossy().into_owned();
        if !secret_ids.insert(id.clone()) || !secret_paths.insert(file_path.clone()) {
            return Err("Secret reference IDs and file paths must be unique.".into());
        }
        secret_references.push(SecretReferenceSetting {
            id,
            label,
            kind: reference.kind,
            file_path,
        });
    }
    Ok(DesktopSettings {
        schema_version: 1,
        storage_mode: "app_data_managed".into(),
        storage_path: projects.to_string_lossy().into_owned(),
        update_channel: input.update_channel,
        auto_check_updates: input.auto_check_updates,
        managed_gateway_base_url,
        analytics_endpoint,
        codex_auth_mode: "official_local_codex".into(),
        publishing_targets,
        secret_references,
        updated_at: unix_time(),
    })
}

#[tauri::command]
fn get_desktop_settings(app: AppHandle) -> Result<DesktopSettings, String> {
    read_desktop_settings(&app)
}

#[tauri::command]
fn update_desktop_settings(
    app: AppHandle,
    input: UpdateDesktopSettingsInput,
) -> Result<DesktopSettings, String> {
    let settings = validate_desktop_settings(&app, input)?;
    let path = desktop_settings_path(&app)?;
    let temporary = path.with_extension("json.tmp");
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(&settings).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to stage desktop settings: {error}"))?;
    protect_private_file(&temporary)?;
    let backup = path.with_extension("json.backup");
    if backup.exists() {
        let _ = fs::remove_file(&temporary);
        return Err("A previous desktop settings backup requires recovery.".into());
    }
    if path.exists() {
        fs::rename(&path, &backup)
            .map_err(|error| format!("Unable to preserve desktop settings: {error}"))?;
    }
    if let Err(error) = fs::rename(&temporary, &path) {
        if backup.exists() {
            let _ = fs::rename(&backup, &path);
        }
        return Err(format!("Unable to save desktop settings: {error}"));
    }
    if backup.exists() {
        fs::remove_file(&backup).map_err(|error| {
            format!("Settings saved but previous backup could not be removed: {error}")
        })?;
    }
    protect_private_file(&path)?;
    Ok(settings)
}

fn updates_state_root(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve update recovery storage: {error}"))?
        .join("updates");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create update recovery storage: {error}"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Unable to protect update recovery storage: {error}"))?;
    }
    Ok(root)
}

fn signed_update_configuration(app: &AppHandle) -> Result<(String, String, String), String> {
    let settings = read_desktop_settings(app)?;
    let pubkey = option_env!("ESTATE_STUDIO_UPDATER_PUBKEY")
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "Signed updater public key is unavailable in this build.".to_string())?;
    let endpoint = if settings.update_channel == "beta" {
        option_env!("ESTATE_STUDIO_UPDATE_BETA_URL")
    } else {
        option_env!("ESTATE_STUDIO_UPDATE_STABLE_URL")
    }
    .filter(|value| !value.trim().is_empty())
    .ok_or_else(|| {
        format!(
            "{} update endpoint is unavailable in this build.",
            settings.update_channel
        )
    })?;
    let url =
        Url::parse(endpoint).map_err(|_| "Compiled update endpoint is invalid.".to_string())?;
    if url.scheme() != "https" || !url.username().is_empty() || url.password().is_some() {
        return Err("Compiled update endpoint must be credential-free HTTPS.".into());
    }
    Ok((settings.update_channel, pubkey.into(), endpoint.into()))
}

fn update_recovery_pending(app: &AppHandle) -> Result<bool, String> {
    Ok(updates_state_root(app)?.join("pending.json").is_file())
}

#[tauri::command]
async fn check_signed_update(app: AppHandle) -> Result<SignedUpdateStatus, String> {
    let current_version = app.package_info().version.to_string();
    let recovery_pending = update_recovery_pending(&app)?;
    let (channel, pubkey, endpoint) = match signed_update_configuration(&app) {
        Ok(value) => value,
        Err(detail) => {
            return Ok(SignedUpdateStatus {
                configured: false,
                available: false,
                channel: read_desktop_settings(&app)?.update_channel,
                current_version,
                target_version: None,
                notes: None,
                published_at: None,
                recovery_pending,
                detail,
            });
        }
    };
    let updater = app
        .updater_builder()
        .pubkey(pubkey)
        .endpoints(vec![
            Url::parse(&endpoint).map_err(|_| "Compiled update endpoint is invalid.")?
        ])
        .map_err(|error| format!("Unable to configure signed updater: {error}"))?
        .build()
        .map_err(|error| format!("Unable to start signed updater: {error}"))?;
    let update = updater
        .check()
        .await
        .map_err(|error| format!("Signed update check failed: {error}"))?;
    Ok(match update {
        Some(update) => SignedUpdateStatus {
            configured: true,
            available: true,
            channel,
            current_version,
            target_version: Some(update.version),
            notes: update.body.map(|body| body.chars().take(4_000).collect()),
            published_at: update.date.map(|date| date.to_string()),
            recovery_pending,
            detail:
                "A signed update is available. Installation requires a separate explicit action."
                    .into(),
        },
        None => SignedUpdateStatus {
            configured: true,
            available: false,
            channel,
            current_version,
            target_version: None,
            notes: None,
            published_at: None,
            recovery_pending,
            detail: "No newer signed update is available.".into(),
        },
    })
}

#[tauri::command]
async fn install_signed_update(
    app: AppHandle,
    expected_version: String,
) -> Result<SignedUpdateStatus, String> {
    let expected_version = clean_required(&expected_version, "Expected update version")?;
    let current_version = app.package_info().version.to_string();
    let (channel, pubkey, endpoint) = signed_update_configuration(&app)?;
    let updater = app
        .updater_builder()
        .pubkey(pubkey)
        .endpoints(vec![
            Url::parse(&endpoint).map_err(|_| "Compiled update endpoint is invalid.")?
        ])
        .map_err(|error| format!("Unable to configure signed updater: {error}"))?
        .build()
        .map_err(|error| format!("Unable to start signed updater: {error}"))?;
    let update = updater
        .check()
        .await
        .map_err(|error| format!("Signed update recheck failed: {error}"))?
        .ok_or_else(|| "The previously offered update is no longer available.".to_string())?;
    if update.version != expected_version {
        return Err(format!(
            "Available version changed from {expected_version} to {}; review it again.",
            update.version
        ));
    }
    let root = updates_state_root(&app)?;
    let marker = UpdateRecoveryMarker {
        schema_version: 1,
        previous_version: current_version.clone(),
        target_version: expected_version.clone(),
        channel: channel.clone(),
        started_at: unix_time(),
        status: "installing".into(),
    };
    let pending = root.join("pending.json");
    fs::write(
        &pending,
        serde_json::to_vec_pretty(&marker).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to write update recovery marker: {error}"))?;
    protect_private_file(&pending)?;
    if let Err(error) = update.download_and_install(|_, _| {}, || {}).await {
        let mut failed = marker;
        failed.status = "install_failed".into();
        let _ = fs::write(
            &pending,
            serde_json::to_vec_pretty(&failed).unwrap_or_default(),
        );
        return Err(format!(
            "Signed update download or installation failed: {error}"
        ));
    }
    Ok(SignedUpdateStatus {
        configured: true,
        available: false,
        channel,
        current_version,
        target_version: Some(expected_version),
        notes: None,
        published_at: None,
        recovery_pending: true,
        detail: "Signature verified and update installed. Restart Estate Studio, then confirm launch health.".into(),
    })
}

#[tauri::command]
fn confirm_update_health(app: AppHandle) -> Result<SignedUpdateStatus, String> {
    let root = updates_state_root(&app)?;
    let pending = root.join("pending.json");
    let current_version = app.package_info().version.to_string();
    let channel = read_desktop_settings(&app)?.update_channel;
    if !pending.is_file() {
        return Ok(SignedUpdateStatus {
            configured: signed_update_configuration(&app).is_ok(),
            available: false,
            channel,
            current_version,
            target_version: None,
            notes: None,
            published_at: None,
            recovery_pending: false,
            detail: "No update recovery marker is pending.".into(),
        });
    }
    let mut marker: UpdateRecoveryMarker = serde_json::from_slice(
        &fs::read(&pending)
            .map_err(|error| format!("Unable to read update recovery marker: {error}"))?,
    )
    .map_err(|error| format!("Update recovery marker is invalid: {error}"))?;
    if marker.target_version != current_version {
        return Ok(SignedUpdateStatus {
            configured: signed_update_configuration(&app).is_ok(),
            available: false,
            channel,
            current_version,
            target_version: Some(marker.target_version),
            notes: None,
            published_at: None,
            recovery_pending: true,
            detail: format!(
                "Expected updated version did not launch. Reinstall the retained signed {} package and keep app data unchanged.",
                marker.previous_version
            ),
        });
    }
    marker.status = "launch_confirmed".into();
    let completed = root.join("last-success.json");
    fs::write(
        &completed,
        serde_json::to_vec_pretty(&marker).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to save update health evidence: {error}"))?;
    protect_private_file(&completed)?;
    fs::remove_file(&pending)
        .map_err(|error| format!("Unable to clear update recovery marker: {error}"))?;
    Ok(SignedUpdateStatus {
        configured: signed_update_configuration(&app).is_ok(),
        available: false,
        channel,
        current_version,
        target_version: Some(marker.target_version),
        notes: None,
        published_at: None,
        recovery_pending: false,
        detail: "Updated version launch health confirmed; rollback marker cleared.".into(),
    })
}

#[tauri::command]
async fn finalize_verified_release(
    app: AppHandle,
    project_id: String,
    input: DeploymentReadbackInput,
) -> Result<VerifiedReleaseResult, String> {
    require_entitlement(&app, "verified_release")?;
    require_local_role(&app, &["owner", "publisher"])?;
    if input.expected_access_mode == "private" {
        require_entitlement(&app, "private_links")?;
    }
    let readback =
        verify_deployment_readback(app.clone(), project_id.clone(), input.clone()).await?;
    let base = validate_deployment_url(&input.base_url)?;
    let private_link = input.expected_access_mode == "private";
    let root = PathBuf::from(&readback.project.project_root);
    let mut manifest = read_manifest(&root)?;
    let release_index = manifest
        .releases
        .iter()
        .position(|release| release.id == input.release_id)
        .ok_or_else(|| "Read-back release is unavailable.".to_string())?;
    if manifest.releases[release_index].status != "readback_passed" {
        return Err(
            "Release must pass a fresh deployment read-back before final verification.".into(),
        );
    }
    let timestamp = unix_time();
    let artifact_root = if private_link {
        private_release_links_root(&app)?
            .join(&project_id)
            .join(&input.release_id)
    } else {
        root.join("releases").join(&input.release_id).join("share")
    };
    if artifact_root.exists() {
        return Err(
            "Share artifacts already exist; verified releases cannot be finalized twice.".into(),
        );
    }
    fs::create_dir_all(&artifact_root)
        .map_err(|error| format!("Unable to prepare unit share artifacts: {error}"))?;
    #[cfg(unix)]
    if private_link {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&artifact_root, fs::Permissions::from_mode(0o700))
            .map_err(|error| format!("Unable to protect private share artifacts: {error}"))?;
    }
    let generated =
        (|| -> Result<(Vec<UnitShareLinkRecord>, Vec<VerifiedUnitShareLink>), String> {
            let mut records = Vec::new();
            let mut outputs = Vec::new();
            for unit_id in manifest.releases[release_index].unit_ids.clone() {
                let unit = manifest
                    .units
                    .iter()
                    .find(|unit| unit.id == unit_id)
                    .ok_or_else(|| format!("Verified release unit is unavailable: {unit_id}"))?;
                let room_id = unit
                    .entrance_room_id
                    .as_deref()
                    .or_else(|| unit.rooms.first().map(|room| room.id.as_str()))
                    .ok_or_else(|| format!("{unit_id} has no shareable entry room."))?;
                let mut url = base.clone();
                let private_pairs = if private_link {
                    base.query_pairs()
                        .map(|(key, value)| (key.into_owned(), value.into_owned()))
                        .collect::<Vec<_>>()
                } else {
                    vec![]
                };
                url.set_query(None);
                {
                    let mut query = url.query_pairs_mut();
                    for (key, value) in private_pairs {
                        query.append_pair(&key, &value);
                    }
                    query
                        .append_pair("unit", &unit_id)
                        .append_pair("room", room_id)
                        .append_pair("mode", "panorama");
                }
                let safe_unit = slugify(&unit_id);
                let qr_path = artifact_root.join(format!("unit-{safe_unit}.svg"));
                fs::write(&qr_path, qr_svg(url.as_str())?)
                    .map_err(|error| format!("Unable to save unit share QR: {error}"))?;
                if private_link {
                    protect_private_file(&qr_path)?;
                }
                let secret_ref = private_link.then(|| {
                    format!(
                        "private-release-links/{project_id}/{}/unit-{safe_unit}",
                        input.release_id
                    )
                });
                records.push(UnitShareLinkRecord {
                    unit_id: unit_id.clone(),
                    public_url: (!private_link).then(|| url.to_string()),
                    private_secret_ref: secret_ref,
                    qr_relative_path: (!private_link).then(|| {
                        format!("releases/{}/share/unit-{safe_unit}.svg", input.release_id)
                    }),
                    created_at: timestamp,
                });
                outputs.push(VerifiedUnitShareLink {
                    unit_id,
                    url: url.to_string(),
                    qr_path: Some(qr_path.to_string_lossy().into_owned()),
                    private: private_link,
                });
            }
            if private_link {
                let secret_path = artifact_root.join("links.json");
                fs::write(
                    &secret_path,
                    serde_json::to_vec_pretty(&outputs).map_err(|error| error.to_string())?,
                )
                .map_err(|error| format!("Unable to save private unit links: {error}"))?;
                protect_private_file(&secret_path)?;
            }
            Ok((records, outputs))
        })();
    let (records, outputs) = match generated {
        Ok(value) => value,
        Err(error) => {
            let _ = fs::remove_dir_all(&artifact_root);
            return Err(error);
        }
    };
    let superseded_index = manifest
        .releases
        .iter()
        .enumerate()
        .filter(|(index, release)| *index != release_index && release.status == "public_verified")
        .max_by_key(|(_, release)| release.version)
        .map(|(index, _)| index);
    let superseded_id = superseded_index.map(|index| manifest.releases[index].id.clone());
    if let Some(index) = superseded_index {
        manifest.releases[index].status = "superseded".into();
        manifest.releases[index].superseded_by_release_id = Some(input.release_id.clone());
    }
    let qa_id = manifest
        .qa_records
        .iter()
        .rev()
        .find(|record| {
            record
                .scope
                .ends_with(&format!("release:{}", input.release_id))
                && record.status == "passed"
        })
        .map(|record| record.id.clone())
        .ok_or_else(|| "Fresh read-back QA evidence is unavailable.".to_string())?;
    let release = &mut manifest.releases[release_index];
    release.status = "public_verified".into();
    release.verified_at = Some(timestamp);
    release.supersedes_release_id = superseded_id;
    release.unit_share_links = records;
    release.verification_qa_id = Some(qa_id);
    if !private_link {
        let mut stored = base;
        stored.set_query(None);
        stored.set_fragment(None);
        release.public_url = Some(stored.to_string());
    } else {
        release.public_url = None;
    }
    manifest.approval_events.push(ApprovalEvent {
        id: format!("approval-release-verified-{}-{timestamp}", input.release_id),
        subject_type: "release".into(),
        subject_id: input.release_id,
        decision: "public_verified".into(),
        actor: "local_user".into(),
        reason: Some(format!(
            "Fresh logged-out read-back passed for {} access.",
            input.expected_access_mode
        )),
        created_at: timestamp,
    });
    manifest.modules.deployment = "ready".into();
    manifest.status = "Verified release ready".into();
    manifest.updated_at = timestamp;
    record_funnel_event(
        &mut manifest,
        "public_deployment_verified",
        "logged_out_readback",
        timestamp,
    )?;
    if let Err(error) = write_manifest(&root, &manifest) {
        let _ = fs::remove_dir_all(&artifact_root);
        return Err(error);
    }
    Ok(VerifiedReleaseResult {
        project: ProjectRecord {
            manifest,
            project_root: root.to_string_lossy().into_owned(),
        },
        share_links: outputs,
    })
}

#[tauri::command]
fn create_rollback_release(
    app: AppHandle,
    project_id: String,
    target_release_id: String,
) -> Result<ProjectRecord, String> {
    require_entitlement(&app, "rollback")?;
    require_local_role(&app, &["owner", "publisher"])?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let target = manifest
        .releases
        .iter()
        .find(|release| release.id == target_release_id)
        .filter(|release| matches!(release.status.as_str(), "public_verified" | "superseded"))
        .cloned()
        .ok_or_else(|| "Rollback target must be a verified or superseded release.".to_string())?;
    let source = root.join("releases").join(&target.id).join("site");
    validate_static_build(&source)?;
    let version = manifest
        .releases
        .iter()
        .map(|release| release.version)
        .max()
        .unwrap_or(0)
        + 1;
    let timestamp = unix_time();
    let release_id = format!("release-v{version}-rollback-{timestamp}");
    let release_root = root.join("releases").join(&release_id);
    let temporary_root = root.join("releases").join(format!(".{release_id}.tmp"));
    let release = ReleaseRecord {
        id: release_id.clone(),
        version,
        status: "preview_ready".into(),
        access_mode: target.access_mode.clone(),
        created_at: timestamp,
        unit_ids: target.unit_ids.clone(),
        public_url: None,
        verified_at: None,
        supersedes_release_id: None,
        superseded_by_release_id: None,
        rollback_from_release_id: Some(target.id.clone()),
        unit_share_links: vec![],
        verification_qa_id: None,
    };
    let prepared = (|| -> Result<(), String> {
        let site = temporary_root.join("site");
        copy_static_build(&source, &site)?;
        let report = finalize_exported_build(&site)?;
        let snapshot = serde_json::json!({
            "release": release,
            "projectId": project_id,
            "rollbackFromReleaseId": target.id,
            "build": report,
            "generatedAt": timestamp,
            "verification": { "localPreview": true, "publicReadback": false }
        });
        fs::write(
            temporary_root.join("release.json"),
            serde_json::to_vec_pretty(&snapshot).map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to save rollback snapshot: {error}"))?;
        fs::rename(&temporary_root, &release_root)
            .map_err(|error| format!("Unable to finalize rollback release: {error}"))?;
        Ok(())
    })();
    if let Err(error) = prepared {
        if temporary_root.exists() {
            let _ = fs::remove_dir_all(&temporary_root);
        }
        return Err(error);
    }
    manifest.releases.push(release);
    manifest.approval_events.push(ApprovalEvent {
        id: format!("approval-rollback-candidate-{timestamp}"),
        subject_type: "release_rollback".into(),
        subject_id: release_id,
        decision: "preview_ready".into(),
        actor: "local_user".into(),
        reason: Some(format!(
            "Immutable rollback candidate created from {}.",
            target.id
        )),
        created_at: timestamp,
    });
    manifest.modules.deployment = "in_progress".into();
    manifest.status = "Rollback candidate ready · Upload and read-back required".into();
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

struct BundledSkill {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    category: &'static str,
    capabilities: &'static [&'static str],
    content: &'static str,
}

const DEFAULT_PROJECT_SKILL_IDS: &[&str] = &[
    "property-project-intake",
    "offplan-evidence-audit",
    "floorplan-concept-planning",
    "panorama-production",
];

fn bundled_skills() -> Vec<BundledSkill> {
    vec![
        BundledSkill {
            id: "property-project-intake",
            name: "Project Intake Guide",
            description:
                "Guide non-technical property teams from uploaded files to the next evidence-safe production step.",
            category: "Project intake",
            capabilities: &["Chat upload guidance", "Input tier", "Next-step routing"],
            content: include_str!("../skills/property-project-intake/SKILL.md"),
        },
        BundledSkill {
            id: "offplan-evidence-audit",
            name: "Evidence Audit",
            description:
                "Classify project sources and block unsupported property claims before production.",
            category: "Project intake",
            capabilities: &["Evidence classes", "Missing-fact gates", "Readiness review"],
            content: include_str!("../skills/offplan-evidence-audit/SKILL.md"),
        },
        BundledSkill {
            id: "floorplan-concept-planning",
            name: "Floorplan Concepts",
            description: "Turn accepted floor plans into topology-safe room concept briefs.",
            category: "Image production",
            capabilities: &["Room topology", "Concept prompts", "Spatial constraints"],
            content: include_str!("../skills/floorplan-concept-planning/SKILL.md"),
        },
        BundledSkill {
            id: "panorama-production",
            name: "Panorama Production",
            description: "Prepare, stitch and validate high-resolution 360° apartment experiences.",
            category: "Interactive tours",
            capabilities: &["2:1 panorama", "Seam QA", "Mobile validation"],
            content: include_str!("../skills/panorama-production/SKILL.md"),
        },
        BundledSkill {
            id: "property-poster-studio",
            name: "Property Poster Studio",
            description:
                "Create A4 sales sheets or separately reviewed Image Model poster requests.",
            category: "Creative Studio",
            capabilities: &["Markdown to PDF", "Image Model prompts", "Generation gates"],
            content: include_str!("../skills/property-poster-studio/SKILL.md"),
        },
        BundledSkill {
            id: "property-video-storyboard",
            name: "Property Video Storyboard",
            description: "Create timed storyboards and HyperFrames-ready video authoring packages.",
            category: "Creative Studio",
            capabilities: &["Timed scenes", "HyperFrames brief", "Render gates"],
            content: include_str!("../skills/property-video-storyboard/SKILL.md"),
        },
        BundledSkill {
            id: "deployment-readback",
            name: "Deployment Read-back",
            description: "Keep local builds, public deployment and verified read-back separate.",
            category: "Deployment",
            capabilities: &["Release state", "Public verification", "Rollback evidence"],
            content: include_str!("../skills/deployment-readback/SKILL.md"),
        },
    ]
}

fn skills_root(app: &AppHandle) -> Result<PathBuf, String> {
    let root = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Unable to resolve the Skills library: {error}"))?
        .join("skills");
    fs::create_dir_all(&root)
        .map_err(|error| format!("Unable to create the Skills library: {error}"))?;
    Ok(root)
}

fn read_skill_registry(root: &Path) -> SkillRegistry {
    fs::read(root.join("registry.json"))
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

fn write_skill_registry(root: &Path, registry: &SkillRegistry) -> Result<(), String> {
    fs::write(
        root.join("registry.json"),
        serde_json::to_vec_pretty(registry).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to save the Skills registry: {error}"))
}

fn valid_skill_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

fn ensure_bundled_skill_installed(app: &AppHandle, skill_id: &str) -> Result<(), String> {
    if !valid_skill_id(skill_id) {
        return Err("Invalid Skill identifier.".into());
    }
    let skill = bundled_skills()
        .into_iter()
        .find(|item| item.id == skill_id)
        .ok_or_else(|| "This Skill is not in the approved Estate Studio catalog.".to_string())?;
    let root = skills_root(app)?;
    let target = root.join(skill.id);
    fs::create_dir_all(&target).map_err(|error| format!("Unable to install Skill: {error}"))?;
    fs::write(target.join("SKILL.md"), skill.content)
        .map_err(|error| format!("Unable to install Skill: {error}"))?;
    let mut registry = read_skill_registry(&root);
    if let Some(installed) = registry
        .installed
        .iter_mut()
        .find(|item| item.id == skill.id)
    {
        installed.version = "1.0.0".into();
    } else {
        registry.installed.push(InstalledSkill {
            id: skill.id.into(),
            version: "1.0.0".into(),
            installed_at: unix_time(),
        });
    }
    write_skill_registry(&root, &registry)
}

fn loaded_skill_context(app: &AppHandle, manifest: &ProjectManifest) -> Result<String, String> {
    let root = skills_root(app)?;
    let registry = read_skill_registry(&root);
    let mut loaded = Vec::new();
    for id in manifest.enabled_skill_ids.iter().take(6) {
        if !registry.installed.iter().any(|item| &item.id == id) {
            continue;
        }
        let path = root.join(id).join("SKILL.md");
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("Enabled Skill {id} cannot be loaded: {error}"))?;
        if content.len() > 16_000 {
            return Err(format!(
                "Enabled Skill {id} exceeds the 16 KB instruction limit."
            ));
        }
        loaded.push(format!("<skill id=\"{id}\">\n{content}\n</skill>"));
    }
    Ok(if loaded.is_empty() {
        "No project Skills enabled.".into()
    } else {
        loaded.join("\n")
    })
}

#[tauri::command]
fn list_skill_marketplace(app: AppHandle) -> Result<Vec<MarketplaceSkill>, String> {
    let root = skills_root(&app)?;
    let registry = read_skill_registry(&root);
    Ok(bundled_skills()
        .into_iter()
        .map(|skill| {
            let installed = registry.installed.iter().find(|item| item.id == skill.id);
            MarketplaceSkill {
                id: skill.id.into(),
                name: skill.name.into(),
                description: skill.description.into(),
                category: skill.category.into(),
                version: "1.0.0".into(),
                author: "Estate Studio".into(),
                capabilities: skill
                    .capabilities
                    .iter()
                    .map(|item| (*item).into())
                    .collect(),
                installed: installed.is_some(),
                installed_at: installed.map(|item| item.installed_at),
            }
        })
        .collect())
}

#[tauri::command]
fn install_marketplace_skill(
    app: AppHandle,
    skill_id: String,
) -> Result<Vec<MarketplaceSkill>, String> {
    if !valid_skill_id(&skill_id) {
        return Err("Invalid Skill identifier.".into());
    }
    ensure_bundled_skill_installed(&app, &skill_id)?;
    list_skill_marketplace(app)
}

#[tauri::command]
fn set_project_skill_enabled(
    app: AppHandle,
    project_id: String,
    skill_id: String,
    enabled: bool,
) -> Result<ProjectRecord, String> {
    if !valid_skill_id(&skill_id) {
        return Err("Invalid Skill identifier.".into());
    }
    let skill_root = skills_root(&app)?;
    if !read_skill_registry(&skill_root)
        .installed
        .iter()
        .any(|item| item.id == skill_id)
        || !skill_root.join(&skill_id).join("SKILL.md").is_file()
    {
        return Err("Install this Skill before enabling it for a project.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    if enabled && !manifest.enabled_skill_ids.contains(&skill_id) {
        manifest.enabled_skill_ids.push(skill_id.clone());
    }
    if !enabled {
        manifest.enabled_skill_ids.retain(|item| item != &skill_id);
    }
    manifest.enabled_skill_ids.sort();
    manifest.enabled_skill_ids.dedup();
    manifest.updated_at = unix_time();
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

fn creative_dimensions(
    kind: &str,
    poster_mode: Option<&str>,
    format: &str,
) -> Result<(u32, u32, Option<u16>), String> {
    match (kind, poster_mode, format) {
        ("poster", Some("a4_sales_sheet"), "a4_portrait") => Ok((2480, 3508, None)),
        ("poster", Some("ai_creative"), "instagram_portrait") => Ok((1080, 1350, None)),
        ("poster", Some("ai_creative"), "story_portrait") => Ok((1080, 1920, None)),
        ("video", _, "social_vertical") => Ok((1080, 1920, Some(15))),
        ("video", _, "social_square") => Ok((1080, 1080, Some(15))),
        ("video", _, "web_landscape") => Ok((1920, 1080, Some(30))),
        _ => Err("Unsupported creative format.".into()),
    }
}

fn parse_creative_model_envelope(value: &str) -> Result<CreativeModelEnvelope, String> {
    let trimmed = value.trim();
    let json = if trimmed.starts_with("```json") && trimmed.ends_with("```") {
        trimmed
            .trim_start_matches("```json")
            .trim_end_matches("```")
            .trim()
    } else if trimmed.starts_with("```") && trimmed.ends_with("```") {
        trimmed
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
    } else {
        trimmed
    };
    let envelope: CreativeModelEnvelope = serde_json::from_str(json)
        .map_err(|_| "Property AI returned an invalid creative brief.".to_string())?;
    if envelope.headline.trim().is_empty() || envelope.call_to_action.trim().is_empty() {
        return Err("Property AI returned an incomplete creative brief.".into());
    }
    Ok(envelope)
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn sha256_text(value: &str) -> String {
    format!("{:x}", Sha256::digest(value.as_bytes()))
}

fn yaml_scalar(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"unavailable\"".into())
}

fn sales_document_label(document_type: Option<&str>) -> &'static str {
    match document_type {
        Some("project_sales_brochure") => "Project sales brochure",
        Some("floorplan_book") => "Floorplan book",
        Some("price_availability") => "Price and availability",
        Some("finishes_specifications") => "Finishes and specifications",
        Some("agent_kit") => "Agent kit",
        Some("showroom_eoi_pack") => "Showroom and EOI pack",
        _ => "Unit sales sheet",
    }
}

fn supported_sales_document_type(value: &str) -> bool {
    SALES_DOCUMENT_TYPES.contains(&value)
}

fn sales_materials_sot_markdown(manifest: &ProjectManifest, brief: &CreativeBrief) -> String {
    let unit = brief
        .unit_id
        .as_ref()
        .and_then(|id| manifest.units.iter().find(|unit| &unit.id == id));
    let unit_section = match unit {
        Some(unit) => format!(
            "| Unit ID | {} |\n| Unit label | {} |\n| Unit summary | {} |\n| Unit status | {} |\n| Floorplan asset ID | {} |",
            unit.id,
            unit.label,
            unit.summary,
            unit.status,
            unit.floorplan_asset_id.as_deref().unwrap_or("unavailable")
        ),
        None => "| Selected unit | unavailable |".into(),
    };
    let evidence_rows = brief
        .evidence_asset_ids
        .iter()
        .filter_map(|id| manifest.assets.iter().find(|asset| &asset.id == id))
        .map(|asset| {
            format!(
                "| {} | {} | {} | {} | {} |",
                asset.id,
                asset.name.replace('|', "\\|"),
                asset.category,
                asset.evidence_class,
                asset.checksum_sha256
            )
        })
        .collect::<Vec<_>>();
    let evidence_table = if evidence_rows.is_empty() {
        "| unavailable | No accepted evidence selected | unavailable | unavailable | unavailable |"
            .into()
    } else {
        evidence_rows.join("\n")
    };
    let mut gaps = vec![
        "Commercial price and live availability are unavailable unless supplied through an approved sales schedule.",
        "Legal, deposit, EOI, refund and privacy wording are unavailable unless supplied by an authorised reviewer.",
    ];
    match brief.a4_document_type.as_deref() {
        Some("floorplan_book") | Some("unit_sales_sheet") if unit.is_none() => {
            gaps.push("A selected unit and its approved floorplan are required.");
        }
        Some("finishes_specifications") => {
            gaps.push(
                "Approved finishes, appliance, fixture and substitution schedules are required.",
            );
        }
        Some("agent_kit") => {
            gaps.push(
                "Approved sales process, contacts, FAQs and agent talking points are required.",
            );
        }
        Some("showroom_eoi_pack") => {
            gaps.push(
                "Approved showroom, appointment, EOI and privacy process details are required.",
            );
        }
        _ => {}
    }
    let gap_list = gaps
        .into_iter()
        .map(|gap| format!("- {gap}"))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nschemaVersion: 1\ndocumentRole: sales_materials_sot\nprojectId: {}\ncreativeId: {}\ndocumentType: {}\nstatus: evidence_review_required\n---\n\n# Sales materials source of truth\n\n## Document task\n\n| Field | Value |\n| --- | --- |\n| Deliverable | {} |\n| Project | {} |\n| Company | {} |\n| Location | {} |\n| Locale | {} |\n| Measurement units | {} |\n| Disclosure | {} |\n\n## Selected unit facts\n\n| Field | Value |\n| --- | --- |\n{}\n\n## Accepted evidence snapshot\n\n| Asset ID | Name | Category | Evidence class | SHA-256 |\n| --- | --- | --- | --- | --- |\n{}\n\n## Known gaps and stop rules\n\n{}\n\n## Derivation rule\n\n`document.md` may use only facts recorded above or directly supported by the accepted evidence snapshot. Missing information remains `unavailable`; it must not be estimated. `document.html` is generated from `document.md` and is never edited as a source.\n",
        yaml_scalar(&manifest.project_id),
        yaml_scalar(&brief.id),
        yaml_scalar(brief.a4_document_type.as_deref().unwrap_or("unit_sales_sheet")),
        sales_document_label(brief.a4_document_type.as_deref()),
        manifest.name,
        manifest.company,
        manifest.location,
        manifest.locale,
        manifest.measurement_units,
        manifest.disclosure,
        unit_section,
        evidence_table,
        gap_list,
    )
}

fn sales_sheet_markdown(
    manifest: &ProjectManifest,
    brief: &CreativeBrief,
    sot_sha256: &str,
) -> String {
    format!(
        "---\nproject: {}\ncompany: {}\ncreativeId: {}\ndocumentType: {}\nformat: A4\nstatus: client_review_required\nsotPath: SOT.md\nsotSha256: {}\n---\n\n# {}\n\n## {}\n\n{}\n\n**{}**\n\n---\n\n{} · {}\n\n> Source boundary: this document is derived from the fingerprinted `SOT.md`. Missing facts remain unavailable.\n",
        yaml_scalar(&manifest.name),
        yaml_scalar(&manifest.company),
        yaml_scalar(&brief.id),
        yaml_scalar(brief.a4_document_type.as_deref().unwrap_or("unit_sales_sheet")),
        sot_sha256,
        xml_escape(&brief.headline),
        xml_escape(&brief.subheadline),
        xml_escape(&brief.body),
        xml_escape(&brief.call_to_action),
        xml_escape(&manifest.company),
        xml_escape(&manifest.location)
    )
}

fn markdown_body(markdown: &str) -> &str {
    if let Some(rest) = markdown.strip_prefix("---\n") {
        if let Some(index) = rest.find("\n---\n") {
            return &rest[index + 5..];
        }
    }
    markdown
}

fn sales_sheet_html(
    manifest: &ProjectManifest,
    brief: &CreativeBrief,
    markdown: &str,
    sot_sha256: &str,
) -> String {
    let mut rendered = String::new();
    let parser = Parser::new_ext(markdown_body(markdown), Options::all());
    html::push_html(&mut rendered, parser);
    let markdown_sha256 = sha256_text(markdown);
    let design = &manifest.design_spec;
    format!(
        r##"<!doctype html><html lang="{locale}"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width"><title>{title}</title><style>
@page{{size:A4;margin:0}}*{{box-sizing:border-box}}html,body{{margin:0;background:#ded8cd}}body{{font-family:{body_font};color:{ink};font-weight:300}}.sheet{{position:relative;width:210mm;min-height:297mm;margin:0 auto;padding:{margin}mm {margin}mm 25mm;background:{paper}}}header{{display:flex;justify-content:space-between;align-items:center;padding-bottom:6mm;border-bottom:.3mm solid {muted}}}.brand{{font-size:8pt;font-weight:500;letter-spacing:.18em}}.format{{color:{muted};font-size:6.5pt;letter-spacing:.12em}}main{{position:relative;padding-top:28mm}}main::before{{content:"";position:absolute;top:12mm;left:0;width:14mm;height:.8mm;background:{accent}}}h1{{max-width:165mm;margin:0 0 5mm;color:{ink};font-family:{display_font};font-size:36pt;font-style:italic;line-height:1.02;font-weight:400;letter-spacing:-.025em}}h2{{max-width:150mm;margin:0 0 12mm;color:{ink};font-size:12pt;line-height:1.45;font-weight:300}}p,li{{max-width:145mm;font-size:9.5pt;line-height:1.65}}strong{{font-weight:600}}blockquote{{margin:12mm 0 0;padding:3mm 4mm;border-left:1mm solid {accent};color:{muted}}}footer{{position:absolute;right:{margin}mm;bottom:10mm;left:{margin}mm;display:flex;justify-content:space-between;color:{muted};font-size:6pt;letter-spacing:.05em}}@media print{{html,body{{background:white}}.sheet{{margin:0;box-shadow:none}}}}
</style><meta name="estate-studio-sot-sha256" content="{sot_sha256}"><meta name="estate-studio-markdown-sha256" content="{markdown_sha256}"></head><body><article class="sheet"><header><span class="brand">{company}</span><span class="format">{document_type} · CLIENT REVIEW</span></header><main>{rendered}</main><footer><span>{location}</span><span>SOT → MARKDOWN → HTML</span></footer></article></body></html>"##,
        title = xml_escape(&brief.title),
        locale = xml_escape(&manifest.locale),
        company = xml_escape(&manifest.company.to_uppercase()),
        document_type =
            xml_escape(&sales_document_label(brief.a4_document_type.as_deref()).to_uppercase()),
        rendered = rendered,
        location = xml_escape(&manifest.location),
        paper = xml_escape(&design.colours.paper),
        ink = xml_escape(&design.colours.ink),
        accent = xml_escape(&design.colours.accent),
        muted = xml_escape(&design.colours.muted),
        display_font = xml_escape(&design.typography.display),
        body_font = xml_escape(&design.typography.body),
        margin = design.layout.margin_mm,
        sot_sha256 = sot_sha256,
        markdown_sha256 = markdown_sha256,
    )
}

fn image_generation_request(
    manifest: &ProjectManifest,
    brief: &CreativeBrief,
) -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": 1,
        "workflow": "image_model",
        "provider": brief.generation_provider.as_deref().unwrap_or("openai_image_model"),
        "status": "prepared_not_submitted",
        "approvalState": "not_approved",
        "priceStatus": brief.price_status.as_deref().unwrap_or("unavailable"),
        "outputCount": 1,
        "dimensions": { "width": brief.width, "height": brief.height },
        "project": { "id": manifest.project_id, "name": manifest.name },
        "creativeId": brief.id,
        "evidenceAssetIds": brief.evidence_asset_ids,
        "prompt": brief.image_prompt.as_deref().unwrap_or(""),
        "overlayCopy": {
            "headline": brief.headline,
            "subheadline": brief.subheadline,
            "body": brief.body,
            "callToAction": brief.call_to_action,
            "projectHighlights": brief.project_highlights
        },
        "boundary": "This package has not been submitted to an Image Model and contains no generated image."
    })
}

fn video_brief_markdown(manifest: &ProjectManifest, brief: &CreativeBrief) -> String {
    let scenes = brief
        .scenes
        .iter()
        .map(|scene| {
            format!(
                "## Scene {} · {} ({}s)\n\nVisual: {}\n\nOn-screen text: {}\n",
                scene.order,
                scene.title,
                scene.duration_seconds,
                scene.visual_direction,
                scene.on_screen_text
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "---\nworkflow: general-video\nframework: HyperFrames\nflow: companion\nstatus: planned\nproject: {}\nformat: {}\ncanvas: {}x{}\ndurationSeconds: {}\n---\n\n# {}\n\nObjective: {}\n\nAudience: {}\n\nVisual direction: {}\n\n{}\n## Production gate\n\nThis is an authoring package only. No renderer, paid image model, Seedance task or MP4 export has run.\n",
        manifest.name, brief.format, brief.width, brief.height, brief.duration_seconds.unwrap_or_default(), brief.title, brief.objective, brief.audience, brief.visual_direction, scenes
    )
}

#[tauri::command]
async fn generate_creative_brief(
    app: AppHandle,
    project_id: String,
    input: GenerateCreativeBriefInput,
) -> Result<CreativeBrief, String> {
    let (root, manifest) = editable_project(&app, &project_id)?;
    let campaign_name = clean_required(&input.campaign_name, "Campaign name")?;
    let audience = clean_required(&input.audience, "Audience")?;
    let objective = clean_required(&input.objective, "Objective")?;
    let user_request = input
        .request
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(&objective);
    let poster_mode = if input.kind == "poster" {
        Some(
            input
                .poster_mode
                .clone()
                .ok_or_else(|| "Choose A4 sales sheet or AI creative poster.".to_string())?,
        )
    } else {
        None
    };
    let a4_document_type = if poster_mode.as_deref() == Some("a4_sales_sheet") {
        let value = input
            .a4_document_type
            .as_deref()
            .unwrap_or("unit_sales_sheet");
        if !supported_sales_document_type(value) {
            return Err("Choose a supported property A4 document type.".into());
        }
        Some(value.to_string())
    } else {
        None
    };
    let (width, height, duration_seconds) =
        creative_dimensions(&input.kind, poster_mode.as_deref(), &input.format)?;
    if let Some(unit_id) = input.unit_id.as_ref() {
        if !manifest.units.iter().any(|unit| &unit.id == unit_id) {
            return Err("The selected unit is unavailable.".into());
        }
    }
    for asset_id in &input.evidence_asset_ids {
        let asset = manifest
            .assets
            .iter()
            .find(|asset| &asset.id == asset_id)
            .ok_or_else(|| "A selected evidence asset is unavailable.".to_string())?;
        if asset.status != "accepted" {
            return Err("Creative briefs can reference accepted evidence only.".into());
        }
    }
    let project_context = serde_json::json!({
        "name": manifest.name, "company": manifest.company, "location": manifest.location,
        "units": manifest.units,
        "unit": input.unit_id.as_ref().and_then(|id| manifest.units.iter().find(|unit| &unit.id == id)),
        "acceptedEvidence": manifest.assets.iter().filter(|asset| input.evidence_asset_ids.contains(&asset.id)).collect::<Vec<_>>()
    });
    let skill_context = loaded_skill_context(&app, &manifest)?;
    let scene_instruction = if input.kind == "video" {
        format!(
            "Return 3-5 scenes whose durationSeconds add exactly to {}.",
            duration_seconds.unwrap_or_default()
        )
    } else {
        "Return scenes as an empty array.".into()
    };
    let poster_instruction = match poster_mode.as_deref() {
        Some("a4_sales_sheet") => "Create a factual property sales document matching the requested document type, with clear print hierarchy. The supported tasks are a project sales brochure, floorplan book, unit sales sheet, price and availability list, finishes and specifications schedule, agent kit, and showroom/EOI pack. Use only the project SOT snapshot. Mark missing commercial, finishes, contact, legal, privacy and EOI data unavailable; do not infer it from marketing copy or imagery. Return imagePrompt as an empty string.",
        Some("ai_creative") => "First analyse the supplied project and return 3-5 short projectHighlights that are useful on a sales poster and directly supported by project_context. Then create the poster headline, supporting copy and call to action. imagePrompt must be a concise labelled Image Model spec for a high-impact raster main visual, with portrait composition and negative space for reliable copy overlay. The generated main visual must contain no text or logo. Require no watermark and no invented property facts, rooms, views, finishes, amenities or people. If acceptedEvidence is empty, describe an abstract/editorial property campaign visual rather than claiming a specific unseen interior.",
        _ => "Return imagePrompt as an empty string.",
    };
    let prompt = format!(
        r#"You are the Creative Studio brief writer inside a desktop product for off-the-plan property teams.
Use only the supplied project facts. Never invent views, finishes, amenities, dimensions or availability. Missing evidence must become a warning. Do not run tools or edit files.
Create a concise {kind} concept for campaign {campaign:?}, audience {audience:?}, objective {objective:?}, format {format}, A4 document type {a4_document_type:?}. Treat the natural-language user request as the primary brief while keeping all project evidence constraints. {scene_instruction} {poster_instruction}
Return exactly one JSON object with no markdown fence:
{{"headline":"max 55 chars","subheadline":"max 90 chars","body":"max 220 chars","callToAction":"max 45 chars","projectHighlights":["3-5 evidence-grounded poster highlights for AI creative posters; empty for other modes"],"visualDirection":"max 240 chars","imagePrompt":"labelled prompt or empty string","scenes":[{{"order":1,"durationSeconds":5,"title":"","visualDirection":"","onScreenText":""}}],"warnings":["evidence limitation"]}}
<project_context>{context}</project_context>
<user_request>{user_request}</user_request>
<enabled_project_skills>{skill_context}</enabled_project_skills>"#,
        kind = input.kind,
        campaign = campaign_name,
        audience = audience,
        objective = objective,
        format = input.format,
        a4_document_type = a4_document_type,
        user_request = user_request,
        poster_instruction = poster_instruction,
        context = serde_json::to_string(&project_context).map_err(|error| error.to_string())?,
        skill_context = skill_context
    );
    let generated = parse_creative_model_envelope(&ai::chat(&root, &prompt).await?)?;
    if poster_mode.as_deref() == Some("ai_creative") && generated.image_prompt.trim().is_empty() {
        return Err("Property AI returned an AI poster without an Image Model prompt. Regenerate the draft.".into());
    }
    if input.kind == "video" {
        let total: u16 = generated
            .scenes
            .iter()
            .map(|scene| scene.duration_seconds)
            .sum();
        if generated.scenes.is_empty() || total != duration_seconds.unwrap_or_default() {
            return Err("Property AI returned a video timeline with the wrong total duration. Regenerate the draft.".into());
        }
    }
    let timestamp = unix_time();
    Ok(CreativeBrief {
        id: format!("{}-{}", slugify(&campaign_name), timestamp),
        kind: input.kind.clone(),
        poster_mode: poster_mode.clone(),
        a4_document_type: a4_document_type.clone(),
        title: format!(
            "{} {}",
            campaign_name,
            match poster_mode.as_deref() {
                Some("a4_sales_sheet") => match a4_document_type.as_deref() {
                    Some("project_sales_brochure") => "project sales brochure",
                    Some("floorplan_book") => "floorplan book",
                    Some("price_availability") => "price and availability list",
                    Some("finishes_specifications") => "finishes and specifications",
                    Some("agent_kit") => "agent kit",
                    Some("showroom_eoi_pack") => "showroom and EOI pack",
                    _ => "unit sales sheet",
                },
                Some("ai_creative") => "AI creative poster",
                _ => "video",
            }
        ),
        campaign_name,
        unit_id: input.unit_id,
        audience,
        objective,
        headline: generated.headline.trim().chars().take(80).collect(),
        subheadline: generated.subheadline.trim().chars().take(120).collect(),
        body: generated.body.trim().chars().take(260).collect(),
        call_to_action: generated.call_to_action.trim().chars().take(60).collect(),
        project_highlights: if poster_mode.as_deref() == Some("ai_creative") {
            generated
                .project_highlights
                .into_iter()
                .map(|value| value.trim().chars().take(80).collect::<String>())
                .filter(|value| !value.is_empty())
                .take(5)
                .collect()
        } else {
            vec![]
        },
        format: input.format,
        width,
        height,
        duration_seconds,
        visual_direction: generated
            .visual_direction
            .trim()
            .chars()
            .take(300)
            .collect(),
        image_prompt: if poster_mode.as_deref() == Some("ai_creative") {
            Some(generated.image_prompt.trim().chars().take(1200).collect())
        } else {
            None
        },
        generation_provider: if poster_mode.as_deref() == Some("ai_creative") {
            Some("openai_image_model".into())
        } else {
            None
        },
        price_status: if poster_mode.as_deref() == Some("ai_creative") {
            Some("unavailable".into())
        } else {
            None
        },
        evidence_asset_ids: input.evidence_asset_ids,
        scenes: generated.scenes,
        warnings: generated.warnings,
        status: "draft".into(),
        workflow: match poster_mode.as_deref() {
            Some("a4_sales_sheet") => "deterministic_svg".into(),
            Some("ai_creative") => "image_model".into(),
            _ => "hyperframes".into(),
        },
        output_relative_path: None,
        created_at: timestamp,
        updated_at: timestamp,
    })
}

#[tauri::command]
fn save_creative_brief(
    app: AppHandle,
    project_id: String,
    input: SaveCreativeBriefInput,
) -> Result<ProjectRecord, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    ensure_current_project(Some(input.expected_updated_at), manifest.updated_at)?;
    let mut brief = input.brief;
    validate_project_id(&brief.id)?;
    if brief.kind == "poster" && brief.poster_mode.is_none() {
        brief.poster_mode = Some(if brief.format == "a4_portrait" {
            "a4_sales_sheet".into()
        } else {
            "ai_creative".into()
        });
    }
    if manifest
        .creative_jobs
        .iter()
        .any(|item| item.id == brief.id)
    {
        return Err("This creative brief has already been saved.".into());
    }
    for asset_id in &brief.evidence_asset_ids {
        let asset = manifest
            .assets
            .iter()
            .find(|asset| &asset.id == asset_id)
            .ok_or_else(|| "A referenced evidence asset is unavailable.".to_string())?;
        if asset.status != "accepted" {
            return Err("Creative briefs can reference accepted evidence only.".into());
        }
    }
    let (width, height, duration) =
        creative_dimensions(&brief.kind, brief.poster_mode.as_deref(), &brief.format)?;
    brief.width = width;
    brief.height = height;
    brief.duration_seconds = duration;
    if brief.kind == "video" {
        let scene_total: u16 = brief
            .scenes
            .iter()
            .map(|scene| scene.duration_seconds)
            .sum();
        if brief.scenes.is_empty() || Some(scene_total) != duration {
            return Err("Video scenes must match the selected format duration.".into());
        }
    }
    let job_root = if brief.poster_mode.as_deref() == Some("a4_sales_sheet") {
        root.join("creative/sales-sheets").join(&brief.id)
    } else {
        root.join("creative/campaigns").join(&brief.id)
    };
    fs::create_dir_all(&job_root)
        .map_err(|error| format!("Unable to create creative package: {error}"))?;
    let timestamp = unix_time();
    if brief.poster_mode.as_deref() == Some("a4_sales_sheet") {
        let sot_relative = format!("creative/sales-sheets/{}/SOT.md", brief.id);
        let markdown_relative = format!("creative/sales-sheets/{}/document.md", brief.id);
        let html_relative = format!("creative/sales-sheets/{}/document.html", brief.id);
        let sot = sales_materials_sot_markdown(&manifest, &brief);
        let sot_sha256 = sha256_text(&sot);
        let markdown = sales_sheet_markdown(&manifest, &brief, &sot_sha256);
        let html = sales_sheet_html(&manifest, &brief, &markdown, &sot_sha256);
        fs::write(root.join(&sot_relative), sot)
            .map_err(|error| format!("Unable to save sales materials SOT: {error}"))?;
        fs::write(root.join(&markdown_relative), markdown)
            .map_err(|error| format!("Unable to save A4 Markdown SoT: {error}"))?;
        fs::write(root.join(&html_relative), html)
            .map_err(|error| format!("Unable to save A4 HTML preview: {error}"))?;
        brief.status = "preview_ready".into();
        brief.output_relative_path = Some(markdown_relative);
    } else if brief.poster_mode.as_deref() == Some("ai_creative") {
        if brief
            .image_prompt
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Err("An AI creative poster requires an Image Model prompt.".into());
        }
        brief.workflow = "image_model".into();
        brief.generation_provider = Some("openai_image_model".into());
        brief.price_status = Some("unavailable".into());
        let relative = format!(
            "creative/campaigns/{}/image-generation-request.json",
            brief.id
        );
        fs::write(
            root.join(&relative),
            serde_json::to_vec_pretty(&image_generation_request(&manifest, &brief))
                .map_err(|error| error.to_string())?,
        )
        .map_err(|error| format!("Unable to save Image Model request: {error}"))?;
        brief.status = "awaiting_generation".into();
        brief.output_relative_path = Some(relative);
    } else {
        let relative = format!("creative/campaigns/{}/BRIEF.md", brief.id);
        fs::write(
            root.join(&relative),
            video_brief_markdown(&manifest, &brief),
        )
        .map_err(|error| format!("Unable to save video brief: {error}"))?;
        brief.status = "package_ready".into();
        brief.output_relative_path = Some(relative);
    }
    brief.updated_at = timestamp;
    fs::write(
        job_root.join("brief.json"),
        serde_json::to_vec_pretty(&brief).map_err(|error| error.to_string())?,
    )
    .map_err(|error| format!("Unable to save creative brief: {error}"))?;
    manifest.creative_jobs.push(brief);
    manifest.modules.creative = "in_progress".into();
    manifest.status = "Creative work in progress · Client review required".into();
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
fn get_creative_a4_html(
    app: AppHandle,
    project_id: String,
    creative_id: String,
) -> Result<String, String> {
    validate_project_id(&project_id)?;
    validate_project_id(&creative_id)?;
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    let brief = manifest
        .creative_jobs
        .iter()
        .find(|item| {
            item.id == creative_id
                && item.kind == "poster"
                && item.poster_mode.as_deref() != Some("ai_creative")
        })
        .ok_or_else(|| "Poster preview is unavailable.".to_string())?;
    let job_root = root.join("creative/sales-sheets").join(&brief.id);
    let sot = fs::read_to_string(job_root.join("SOT.md"))
        .unwrap_or_else(|_| sales_materials_sot_markdown(&manifest, brief));
    let sot_sha256 = sha256_text(&sot);
    let markdown = fs::read_to_string(job_root.join("document.md"))
        .unwrap_or_else(|_| sales_sheet_markdown(&manifest, brief, &sot_sha256));
    Ok(sales_sheet_html(&manifest, brief, &markdown, &sot_sha256))
}

fn wrap_pdf_text(value: &str, max_chars: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in value.lines() {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            let next_len =
                current.chars().count() + usize::from(!current.is_empty()) + word.chars().count();
            if next_len > max_chars && !current.is_empty() {
                lines.push(current);
                current = String::new();
            }
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    lines
}

fn pdf_rgb(value: &str) -> Color {
    let cleaned = value.trim().trim_start_matches('#');
    let parsed = u32::from_str_radix(cleaned, 16).unwrap_or(0x2f241d);
    Color::Rgb(Rgb::new(
        ((parsed >> 16) & 0xff) as f32 / 255.0,
        ((parsed >> 8) & 0xff) as f32 / 255.0,
        (parsed & 0xff) as f32 / 255.0,
        None,
    ))
}

fn write_a4_pdf(
    manifest: &ProjectManifest,
    brief: &CreativeBrief,
    destination: &Path,
) -> Result<(), String> {
    let (doc, page, layer) = PdfDocument::new(&brief.title, Mm(210.0), Mm(297.0), "A4 sales sheet");
    let needs_unicode = format!(
        "{}{}{}{}{}",
        brief.headline, brief.subheadline, brief.body, brief.call_to_action, manifest.location
    )
    .chars()
    .any(|character| character as u32 > 0x024f);
    let font_path = if needs_unicode {
        "/System/Library/Fonts/Supplemental/Arial Unicode.ttf"
    } else {
        "/System/Library/Fonts/Supplemental/Arial.ttf"
    };
    let body_font: IndirectFontRef = File::open(font_path)
        .ok()
        .and_then(|file| doc.add_external_font(file).ok())
        .unwrap_or(
            doc.add_builtin_font(BuiltinFont::Helvetica)
                .map_err(|error| error.to_string())?,
        );
    let display_font = doc
        .add_builtin_font(BuiltinFont::TimesItalic)
        .map_err(|error| error.to_string())?;
    let label_font = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|error| error.to_string())?;
    let current = doc.get_page(page).get_layer(layer);

    let design = &manifest.design_spec;
    let ink = pdf_rgb(&design.colours.ink);
    let accent = pdf_rgb(&design.colours.accent);
    let muted = pdf_rgb(&design.colours.muted);

    current.set_outline_color(accent.clone());
    current.set_outline_thickness(34.0);
    current.add_line(Line {
        points: vec![
            (Point::new(Mm(7.0), Mm(0.0)), false),
            (Point::new(Mm(7.0), Mm(297.0)), false),
        ],
        is_closed: false,
    });

    current.set_fill_color(ink.clone());
    current.use_text(
        manifest.company.to_uppercase(),
        8.5,
        Mm(24.0),
        Mm(278.0),
        &label_font,
    );
    current.set_fill_color(muted.clone());
    current.use_text(
        format!("{}  ·  CLIENT REVIEW", sales_document_label(brief.a4_document_type.as_deref()).to_uppercase()),
        6.5,
        Mm(111.0),
        Mm(278.0),
        &label_font,
    );

    current.set_outline_color(muted.clone());
    current.set_outline_thickness(0.65);
    current.add_line(Line {
        points: vec![(Point::new(Mm(24.0), Mm(270.0)), false), (Point::new(Mm(192.0), Mm(270.0)), false)],
        is_closed: false,
    });

    current.set_fill_color(accent.clone());
    current.use_text(brief.campaign_name.to_uppercase(), 7.0, Mm(24.0), Mm(247.0), &label_font);
    current.set_fill_color(ink.clone());
    let mut y = 229.0;
    for line in wrap_pdf_text(&brief.headline, 23).into_iter().take(3) {
        current.use_text(line, 38.0, Mm(24.0), Mm(y), &display_font);
        y -= 14.5;
    }
    y -= 2.0;
    for line in wrap_pdf_text(&brief.subheadline, 58).into_iter().take(3) {
        current.use_text(line, 11.5, Mm(25.0), Mm(y), &body_font);
        y -= 7.0;
    }

    y -= 16.0;
    current.set_outline_color(accent.clone());
    current.set_outline_thickness(2.2);
    current.add_line(Line {
        points: vec![(Point::new(Mm(25.0), Mm(y + 5.0)), false), (Point::new(Mm(40.0), Mm(y + 5.0)), false)],
        is_closed: false,
    });
    current.set_fill_color(ink.clone());
    for line in wrap_pdf_text(&brief.body, 68).into_iter().take(10) {
        current.use_text(line, 9.5, Mm(25.0), Mm(y), &body_font);
        y -= 5.8;
    }

    current.set_fill_color(accent);
    current.use_text(&brief.call_to_action, 12.0, Mm(25.0), Mm(48.0), &label_font);
    current.set_fill_color(muted);
    current.use_text(
        format!("PROJECT DESIGN SPEC · {}", design.status.to_uppercase()),
        6.0,
        Mm(25.0),
        Mm(34.0),
        &label_font,
    );
    current.set_fill_color(ink);
    current.use_text(
        manifest.location.to_uppercase(),
        7.0,
        Mm(25.0),
        Mm(19.0),
        &label_font,
    );
    current.set_fill_color(pdf_rgb(&design.colours.muted));
    current.use_text("SOT  →  MARKDOWN  →  HTML  →  PDF", 6.0, Mm(111.0), Mm(19.0), &label_font);
    let file =
        File::create(destination).map_err(|error| format!("Unable to create PDF: {error}"))?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|error| format!("Unable to save PDF: {error}"))
}

#[tauri::command]
fn export_a4_pdf(
    app: AppHandle,
    project_id: String,
    creative_id: String,
    output_path: String,
) -> Result<String, String> {
    validate_project_id(&project_id)?;
    validate_project_id(&creative_id)?;
    let destination = PathBuf::from(output_path);
    if destination.extension().and_then(|value| value.to_str()) != Some("pdf") {
        return Err("Choose a destination ending in .pdf".into());
    }
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    let brief = manifest
        .creative_jobs
        .iter()
        .find(|item| {
            item.id == creative_id
                && item.kind == "poster"
                && item.poster_mode.as_deref() == Some("a4_sales_sheet")
        })
        .ok_or_else(|| "A4 sales sheet is unavailable.".to_string())?;

    write_a4_pdf(&manifest, brief, &destination)?;
    Ok(destination.to_string_lossy().into_owned())
}

#[tauri::command]
fn bind_managed_gateway_session(
    state: State<'_, gateway::GatewaySessionStore>,
    input: gateway::BindGatewaySessionInput,
) -> Result<gateway::GatewaySessionStatus, String> {
    state.bind(input, unix_time())
}

#[tauri::command]
fn managed_gateway_status(
    state: State<'_, gateway::GatewaySessionStore>,
) -> Result<gateway::GatewaySessionStatus, String> {
    state.status(unix_time())
}

#[tauri::command]
fn clear_managed_gateway_session(
    state: State<'_, gateway::GatewaySessionStore>,
) -> Result<(), String> {
    state.clear()
}

#[tauri::command]
async fn check_managed_gateway_capability(
    app: AppHandle,
    state: State<'_, gateway::GatewaySessionStore>,
    project_id: String,
    job_id: String,
) -> Result<ManagedGatewayCapabilityResult, String> {
    let root = projects_root(&app)?.join(&project_id);
    let manifest = read_manifest(&root)?;
    if manifest.read_only || manifest.archived_at.is_some() {
        return Err("Managed generation is unavailable for read-only or archived projects.".into());
    }
    let job = manifest
        .generation_jobs
        .iter()
        .find(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?
        .clone();
    validate_generation_package(&manifest, &job)?;
    if job.status != "blocked_capability" {
        return Err("Managed capability check requires a blocked, unapproved package.".into());
    }
    let request = gateway::ManagedCapabilityRequest {
        project_id: project_id.clone(),
        job_id: job_id.clone(),
        approval_fingerprint: job.approval_fingerprint.clone(),
        idempotency_key: job.idempotency_key.clone(),
        requested_width: job.requested_width,
        requested_height: job.requested_height,
        panorama_mode: job.panorama_mode.clone(),
        output_count: job.output_count,
    };
    let response = state.capability(&request, unix_time()).await?;
    if response.request_fingerprint != job.approval_fingerprint {
        return Err(
            "Managed gateway response fingerprint does not match the approved package.".into(),
        );
    }
    let timestamp = unix_time();
    if response.entitlement_status != "active"
        || response.capability_status != "available"
        || response.model_id.as_deref().is_none_or(str::is_empty)
        || response.supported_width != Some(job.requested_width)
        || response.supported_height != Some(job.requested_height)
        || response.panorama_mode.as_deref() != Some(job.panorama_mode.as_str())
        || response.price_status != "available"
        || response.price_amount_minor.is_none()
        || response.price_currency.as_deref().is_none_or(str::is_empty)
        || response.credit_cost.is_none_or(|credits| credits == 0)
        || response.quota_status != "available"
        || response
            .quota_remaining
            .is_none_or(|remaining| remaining < response.credit_cost.unwrap_or(u64::MAX))
        || response.expires_at <= timestamp
    {
        return Err("Managed gateway did not prove active entitlement, exact capability, current price, and sufficient quota.".into());
    }
    let model_id = response.model_id.clone().expect("validated model ID");
    let capability_path = capability_registry_path(&app)?;
    let mut registry = read_capability_registry(&capability_path)?;
    registry.records.retain(|record| {
        !(record.provider_choice == "managed_openai" && record.model_id == model_id)
    });
    registry.records.push(GenerationCapabilityRecord {
        provider_choice: "managed_openai".into(),
        model_id: model_id.clone(),
        access_status: "available".into(),
        supported_sizes: vec![ImageSizeCapability {
            width: job.requested_width,
            height: job.requested_height,
        }],
        panorama_modes: vec![job.panorama_mode.clone()],
        price_status: response.price_status.clone(),
        price_amount_minor: response.price_amount_minor,
        price_currency: response.price_currency.clone(),
        quota_status: response.quota_status.clone(),
        quota_remaining: response.quota_remaining,
        checked_at: response.checked_at,
        expires_at: response.expires_at,
        source: "managed_gateway_signed_session".into(),
    });
    registry.schema_version = 1;
    registry.updated_at = timestamp;
    write_capability_registry(&capability_path, &registry)?;
    let _ = apply_generation_capability(
        app.clone(),
        project_id.clone(),
        job_id.clone(),
        "managed_openai".into(),
        model_id,
    )?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let current = manifest
        .generation_jobs
        .iter_mut()
        .find(|candidate| candidate.id == job_id)
        .ok_or_else(|| "Generation package is unavailable after capability check.".to_string())?;
    current.customer_credit_cost = response.credit_cost;
    refresh_generation_approval_identity(&project_id, current)?;
    let final_approval_fingerprint = current.approval_fingerprint.clone();
    let final_idempotency_key = current.idempotency_key.clone();
    manifest.usage_ledger.retain(|event| {
        !(event.kind == "capability_quote"
            && event.job_id == job_id
            && event.request_fingerprint == response.request_fingerprint)
    });
    manifest.usage_ledger.push(UsageLedgerEntry {
        id: format!("usage-capability-{job_id}-{timestamp}"),
        kind: "capability_quote".into(),
        job_id,
        idempotency_key: final_idempotency_key,
        request_fingerprint: final_approval_fingerprint,
        provider_choice: "managed_openai".into(),
        status: "available_unapproved".into(),
        customer_credit_cost: response.credit_cost,
        customer_price_amount_minor: response.price_amount_minor,
        customer_price_currency: response.price_currency.clone(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    let project = ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    };
    Ok(ManagedGatewayCapabilityResult { project, response })
}

#[tauri::command]
async fn submit_managed_panorama(
    app: AppHandle,
    state: State<'_, gateway::GatewaySessionStore>,
    project_id: String,
    job_id: String,
    confirmed_fingerprint: String,
) -> Result<ProjectRecord, String> {
    require_entitlement(&app, "managed_generation")?;
    require_local_role(&app, &["owner", "operator"])?;
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[index].clone();
    validate_generation_package(&manifest, &snapshot)?;
    if snapshot.status != "approved"
        || snapshot.approval_state != "approved"
        || snapshot.approval_fingerprint != confirmed_fingerprint
    {
        return Err(
            "Managed submission requires the exact currently approved package fingerprint.".into(),
        );
    }
    if snapshot.provider_choice != "managed_openai"
        || snapshot.output_count != 1
        || snapshot
            .customer_credit_cost
            .is_none_or(|credits| credits == 0)
    {
        return Err(
            "Managed submission requires one output and a proven positive credit cost.".into(),
        );
    }
    if snapshot.requested_width != snapshot.requested_height.saturating_mul(2) {
        return Err("One-shot panorama submission requires an exact 2:1 requested canvas.".into());
    }
    let capability_request = gateway::ManagedCapabilityRequest {
        project_id: project_id.clone(),
        job_id: job_id.clone(),
        approval_fingerprint: snapshot.approval_fingerprint.clone(),
        idempotency_key: snapshot.idempotency_key.clone(),
        requested_width: snapshot.requested_width,
        requested_height: snapshot.requested_height,
        panorama_mode: snapshot.panorama_mode.clone(),
        output_count: snapshot.output_count,
    };
    let capability = state.capability(&capability_request, unix_time()).await?;
    if capability.request_fingerprint != snapshot.approval_fingerprint
        || capability.entitlement_status != "active"
        || capability.capability_status != "available"
        || capability.model_id != snapshot.model_id
        || capability.supported_width != Some(snapshot.requested_width)
        || capability.supported_height != Some(snapshot.requested_height)
        || capability.panorama_mode.as_deref() != Some(snapshot.panorama_mode.as_str())
        || capability.price_amount_minor != snapshot.price_amount_minor
        || capability.price_currency != snapshot.price_currency
        || capability.credit_cost != snapshot.customer_credit_cost
        || capability.quota_status != "available"
        || capability
            .quota_remaining
            .is_none_or(|remaining| remaining < snapshot.customer_credit_cost.unwrap_or(u64::MAX))
        || capability.expires_at <= unix_time()
    {
        return Err("Managed entitlement, exact capability, price, credit cost, or quota changed after approval; check capability and approve again.".into());
    }
    let mut managed_inputs = Vec::new();
    for generation_input in &snapshot.inputs {
        let asset = manifest
            .assets
            .iter()
            .find(|asset| asset.id == generation_input.asset_id)
            .ok_or_else(|| {
                format!(
                    "Approved input {} is unavailable.",
                    generation_input.asset_id
                )
            })?;
        if !matches!(
            asset.mime_type.as_str(),
            "image/png" | "image/jpeg" | "image/webp"
        ) {
            return Err(format!(
                "Approved input {} is not an input-capable PNG, JPEG, or WebP asset.",
                generation_input.asset_id
            ));
        }
        let bytes = fs::read(root.join(&asset.relative_path))
            .map_err(|error| format!("Unable to read approved generation input: {error}"))?;
        if bytes.is_empty() || bytes.len() > 25 * 1024 * 1024 {
            return Err("Each approved generation input must be between 1 byte and 25 MB.".into());
        }
        managed_inputs.push(gateway::ManagedGenerationInput {
            role: generation_input.role.clone(),
            checksum_sha256: generation_input.checksum_sha256.clone(),
            data_url: format!(
                "data:{};base64,{}",
                asset.mime_type,
                base64::engine::general_purpose::STANDARD.encode(bytes)
            ),
        });
    }
    let prompt = format!(
        "Create exactly one evidence-controlled 2:1 equirectangular panorama for room '{}'. Camera intent: {} Required confirmed openings: {}. Fixed fixtures: {} (status: {}). Preserve the topology source and identity anchor roles exactly. Prohibitions: {} Return no text, logo, watermark, invented room, opening, fixture, dimension, finish, view, amenity, or person.",
        snapshot.room_name,
        snapshot.camera_intent,
        snapshot.required_opening_ids.join(", "),
        if snapshot.fixed_fixtures.is_empty() { "unavailable".into() } else { snapshot.fixed_fixtures.join(", ") },
        snapshot.fixed_fixtures_status,
        snapshot.prohibitions.join(" ")
    );
    let request = gateway::ManagedGenerationRequest {
        project_id: project_id.clone(),
        job_id: job_id.clone(),
        approval_fingerprint: snapshot.approval_fingerprint.clone(),
        idempotency_key: snapshot.idempotency_key.clone(),
        requested_width: snapshot.requested_width,
        requested_height: snapshot.requested_height,
        panorama_mode: snapshot.panorama_mode.clone(),
        output_count: 1,
        inputs: managed_inputs,
        prompt,
        parameters: serde_json::json!({ "quality": "high", "output_format": "png" }),
    };
    let queued_at = unix_time();
    transition_generation_job(
        &mut manifest.generation_jobs[index],
        "queued",
        1,
        "managed_gateway_client",
        Some("One approved output reserved for idempotent managed submission.".into()),
        queued_at,
    )?;
    manifest.updated_at = queued_at;
    write_manifest(&root, &manifest)?;

    let response = state.submit_image(&request, unix_time()).await?;
    if response.status != "completed"
        || response.output_count != 1
        || response.outputs.len() != 1
        || response.approval_fingerprint != snapshot.approval_fingerprint
        || response.idempotency_key != snapshot.idempotency_key
        || response.completed_at < queued_at
        || response.completed_at > unix_time().saturating_add(300)
    {
        return Err("Managed gateway response did not match the one approved output.".into());
    }
    let provider_request_id = response
        .provider_request_id
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            "Managed gateway response is missing the provider request ID.".to_string()
        })?;
    let encoded = &response.outputs[0].b64_json;
    if encoded.len() > 275 * 1024 * 1024 {
        return Err("Managed gateway output exceeds the 200 MB decoded limit.".into());
    }
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "Managed gateway output is not valid base64 image data.".to_string())?;
    if bytes.is_empty() || bytes.len() > 200 * 1024 * 1024 {
        return Err("Managed gateway output must decode to between 1 byte and 200 MB.".into());
    }
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package disappeared after managed submission.".to_string())?;
    if manifest.generation_jobs[index].status != "queued"
        || manifest.generation_jobs[index].idempotency_key != snapshot.idempotency_key
    {
        return Err("Local generation state changed while the managed task was running.".into());
    }
    let version = manifest.generation_jobs[index]
        .outputs
        .iter()
        .map(|output| output.version)
        .max()
        .unwrap_or(0)
        .saturating_add(1);
    let relative_path = format!("assets/generated/{job_id}/v{version}.png");
    let destination = root.join(&relative_path);
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to create managed output directory: {error}"))?;
    }
    fs::write(&destination, &bytes)
        .map_err(|error| format!("Unable to persist managed panorama output: {error}"))?;
    let (width, height) = image::image_dimensions(&destination)
        .map_err(|error| format!("Managed output is not a decodable image: {error}"))?;
    let checksum_sha256 = sha256_file(&destination)?;
    let completed_at = unix_time();
    let job = &mut manifest.generation_jobs[index];
    job.provider_task_id = Some(provider_request_id.clone());
    transition_generation_job(
        job,
        "submitted",
        10,
        "managed_gateway_client",
        Some("Gateway returned the same idempotent approved task.".into()),
        completed_at,
    )?;
    transition_generation_job(
        job,
        "completed",
        100,
        "managed_gateway_client",
        Some("Exactly one provider output was received; visual review remains required.".into()),
        completed_at,
    )?;
    job.outputs.push(GenerationOutputRecord {
        id: format!("output-{job_id}-v{version}"),
        version,
        relative_path,
        checksum_sha256,
        mime_type: "image/png".into(),
        width,
        height,
        size_bytes: bytes.len() as u64,
        provider_request_id,
        request_fingerprint: snapshot.approval_fingerprint.clone(),
        revised_prompt: response.outputs[0].revised_prompt.clone(),
        source_attempt: snapshot.attempt,
        status: "pending_review".into(),
        publishability: if width == snapshot.requested_width && height == snapshot.requested_height
        {
            "blocked_visual_review"
        } else {
            "blocked_dimension_mismatch"
        }
        .into(),
        rejection_reason: None,
        reviewed_at: None,
        panorama_qa: None,
        derivatives: vec![],
        created_at: completed_at,
    });
    manifest.usage_ledger.push(UsageLedgerEntry {
        id: format!("usage-managed-completed-{job_id}-{completed_at}"),
        kind: "credit_consumed".into(),
        job_id,
        idempotency_key: snapshot.idempotency_key,
        request_fingerprint: response.request_fingerprint,
        provider_choice: "managed_openai".into(),
        status: if response.usage.is_some() {
            "gateway_usage_available"
        } else {
            "gateway_usage_unavailable"
        }
        .into(),
        customer_credit_cost: snapshot.customer_credit_cost,
        customer_price_amount_minor: snapshot.price_amount_minor,
        customer_price_currency: snapshot.price_currency,
        created_at: completed_at,
    });
    manifest.updated_at = completed_at;
    write_manifest(&root, &manifest)?;
    Ok(ProjectRecord {
        manifest,
        project_root: root.to_string_lossy().into_owned(),
    })
}

#[tauri::command]
async fn inspect_codex_generation_adapter(
    app: AppHandle,
    project_id: String,
    job_id: String,
) -> Result<CodexAdapterInspection, String> {
    let (root, mut manifest) = editable_project(&app, &project_id)?;
    let index = manifest
        .generation_jobs
        .iter()
        .position(|job| job.id == job_id)
        .ok_or_else(|| "Generation package is unavailable.".to_string())?;
    let snapshot = manifest.generation_jobs[index].clone();
    validate_generation_package(&manifest, &snapshot)?;
    if !matches!(
        snapshot.status.as_str(),
        "blocked_capability" | "awaiting_approval"
    ) {
        return Err("Codex adapter inspection must occur before approval or submission.".into());
    }
    let report = ai::generation_adapter_report(
        &project_id,
        &job_id,
        &snapshot.approval_state,
        &snapshot.approval_fingerprint,
        &snapshot.idempotency_key,
        snapshot.progress_percent,
    )
    .await?;
    let timestamp = unix_time();
    let capability_path = capability_registry_path(&app)?;
    let mut registry = read_capability_registry(&capability_path)?;
    registry.records.retain(|record| {
        !(record.provider_choice == "codex" && record.model_id == "codex-unreported-image-model")
    });
    registry.records.push(GenerationCapabilityRecord {
        provider_choice: "codex".into(),
        model_id: "codex-unreported-image-model".into(),
        access_status: "unavailable".into(),
        supported_sizes: vec![],
        panorama_modes: vec![],
        price_status: "unavailable".into(),
        price_amount_minor: None,
        price_currency: None,
        quota_status: "unavailable".into(),
        quota_remaining: None,
        checked_at: timestamp,
        expires_at: timestamp.saturating_add(600),
        source: "official_codex_app_server_account".into(),
    });
    registry.schema_version = 1;
    registry.updated_at = timestamp;
    write_capability_registry(&capability_path, &registry)?;

    let job = &mut manifest.generation_jobs[index];
    job.provider_choice = "codex".into();
    job.connection_mode = "codex".into();
    job.model_id = None;
    job.capability_status = "unavailable".into();
    job.price_status = "unavailable".into();
    job.price_amount_minor = None;
    job.price_currency = None;
    job.failure_code = Some("codex_generation_capability_unavailable".into());
    job.failure_reason = Some(report.detail.clone());
    job.updated_at = timestamp;
    if job.status == "awaiting_approval" {
        job.approval_state = "invalidated".into();
        let progress = job.progress_percent;
        transition_generation_job(
            job,
            "blocked_capability",
            progress,
            "codex_adapter",
            Some(report.detail.clone()),
            timestamp,
        )?;
    } else {
        job.state_events.push(GenerationStateEvent {
            id: format!(
                "job-event-{}-{timestamp}-{}",
                job.id,
                job.state_events.len() + 1
            ),
            from_status: "blocked_capability".into(),
            to_status: "blocked_capability".into(),
            progress_percent: job.progress_percent,
            actor: "codex_adapter".into(),
            note: Some(report.detail.clone()),
            created_at: timestamp,
        });
    }
    manifest.qa_records.push(QaRecord {
        id: format!("qa-codex-adapter-{job_id}-{timestamp}"),
        scope: format!("project:{project_id}:generation_job:{job_id}"),
        status: "blocked".into(),
        checks: vec![
            format!("official_login_authenticated={}", report.authenticated),
            "project_scope=validated".into(),
            "approval_fingerprint=present".into(),
            "progress_protocol=persistent_state_events".into(),
            "credential_transport=excluded_from_project_payload_argv_and_audit".into(),
            "codex_binding_protocol=app_server_account_read".into(),
            "image_generation_capability=unavailable".into(),
        ],
        reviewer: "codex_adapter".into(),
        created_at: timestamp,
    });
    manifest.updated_at = timestamp;
    write_manifest(&root, &manifest)?;
    Ok(CodexAdapterInspection {
        project: ProjectRecord {
            manifest,
            project_root: root.to_string_lossy().into_owned(),
        },
        report,
    })
}

#[tauri::command]
async fn check_ai_status() -> Result<ai::AiStatus, String> {
    ai::check_status().await
}

#[tauri::command]
fn open_codex_login() -> Result<(), String> {
    ai::open_login()
}

#[tauri::command]
async fn chat_with_codex(app: AppHandle, input: AiChatInput) -> Result<AiChatResponse, String> {
    let message = input.message.trim().to_string();
    if message.is_empty() {
        return Err("Message is required".into());
    }
    if message.chars().count() > 4_000 {
        return Err("Message is too long; keep it below 4,000 characters.".into());
    }
    let (working_dir, project_context, selected_manifest, skill_context) =
        if let Some(project_id) = input.project_id {
            validate_project_id(&project_id)?;
            let root = projects_root(&app)?.join(&project_id);
            if !root.is_dir() {
                return Err("The selected project is unavailable.".into());
            }
            let manifest = read_manifest(&root)?;
            let context = serde_json::to_string_pretty(&manifest)
                .map_err(|error| format!("Unable to prepare project context: {error}"))?;
            let skills = loaded_skill_context(&app, &manifest)?;
            (root, context, Some(manifest), skills)
        } else {
            let root = projects_root(&app)?;
            let projects = list_projects(app.clone())?
                .into_iter()
                .map(|project| {
                    serde_json::json!({
                        "projectId": project.manifest.project_id,
                        "name": project.manifest.name,
                        "company": project.manifest.company,
                        "readiness": project.manifest.readiness,
                        "unitTypes": project.manifest.units.len(),
                    })
                })
                .collect::<Vec<_>>();
            (
                root,
                serde_json::to_string_pretty(&projects).map_err(|error| error.to_string())?,
                None,
                "No project Skills enabled.".into(),
            )
        };
    let history = input
        .history
        .into_iter()
        .rev()
        .take(8)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .filter(|item| {
            matches!(item.role.as_str(), "assistant" | "user") && !item.text.trim().is_empty()
        })
        .map(|item| {
            format!(
                "{}: {}",
                item.role,
                item.text.trim().chars().take(1_200).collect::<String>()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let prompt = format!(
        r#"You are Property AI inside Estate Studio, a desktop workspace for off-the-plan property developers and agencies.
Do not modify files, run paid generation, deploy, or claim an action completed. Do not invent project facts. Use only the supplied project manifest; if evidence is missing, say it is unavailable. Clearly distinguish official evidence, approved renders, and concepts. Be concise and practical. Format replyMarkdown as clean GitHub-flavoured Markdown when structure helps. Use a small number of meaningful Unicode icons such as ✓, ⚠, ◉ or ✦ when they improve scanning. Never emit raw HTML.

You may propose edits only to these selected-project fields: name, company, location, primary and accent. Create a projectUpdateDraft only when the user gives an explicit action instruction such as set, change, update, rename, replace, 修改, 改成, 更新 or 帮我填. A question, review request or suggestion request must return projectUpdateDraft as null. Never infer a missing field value from weak evidence. Never propose changes for a read-only project or when no project is selected. Colours must be full #RRGGBB values. Put only changed fields in the draft; use null for every unchanged field. A draft is a proposal for human review, not a saved change.

Return exactly one JSON object with no code fence and this shape:
{{"replyMarkdown":"Markdown response","projectUpdateDraft":null}}
or
{{"replyMarkdown":"Markdown response","projectUpdateDraft":{{"summary":"Short description","baseUpdatedAt":0,"name":null,"company":null,"location":null,"primary":null,"accent":null,"warnings":[]}}}}

<project_context>
{project_context}
</project_context>

<enabled_project_skills>
{skill_context}
</enabled_project_skills>

<recent_conversation>
{history}
</recent_conversation>

<user_message>
{message}
</user_message>"#
    );
    let raw_content = ai::chat(&working_dir, &prompt).await?;
    let envelope = parse_ai_model_envelope(&raw_content)?;
    let project_update_draft = match (selected_manifest.as_ref(), envelope.project_update_draft) {
        (Some(manifest), Some(draft)) => normalize_ai_project_draft(manifest, draft)?,
        _ => None,
    };
    Ok(AiChatResponse {
        provider: "codex".into(),
        content: envelope.reply_markdown.trim().to_string(),
        project_update_draft,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(gateway::GatewaySessionStore::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            list_projects,
            get_license_status,
            install_signed_license,
            set_active_local_role,
            get_desktop_settings,
            update_desktop_settings,
            check_signed_update,
            install_signed_update,
            confirm_update_health,
            list_company_profiles,
            create_project,
            duplicate_project_structure,
            update_project,
            set_project_archived,
            set_workflow_mode,
            add_unit,
            add_room,
            save_room_graph,
            assign_room_identity,
            assign_room_panorama,
            create_panorama_draft,
            approve_generation_job,
            retry_generation_job,
            cancel_generation_job,
            record_generation_progress,
            ingest_generation_output,
            review_generation_output,
            record_panorama_qa,
            create_panorama_fallback_plan,
            assign_room_still_fallback,
            assign_room_supporting_asset,
            get_generation_output_data_url,
            build_static_tour_preview,
            recover_generation_jobs,
            enqueue_background_job,
            record_background_job_progress,
            retry_background_job,
            recover_background_jobs,
            run_panorama_background_job,
            list_generation_capabilities,
            apply_generation_capability,
            import_project_assets,
            export_project_bundle,
            import_project_bundle,
            review_asset,
            open_project_folder,
            get_asset_data_url,
            create_local_release,
            export_static_release,
            publish_release_to_customer_directory,
            verify_deployment_readback,
            finalize_verified_release,
            create_rollback_release,
            generate_creative_brief,
            save_creative_brief,
            get_creative_a4_html,
            export_a4_pdf,
            bind_managed_gateway_session,
            managed_gateway_status,
            clear_managed_gateway_session,
            check_managed_gateway_capability,
            submit_managed_panorama,
            inspect_codex_generation_adapter,
            list_skill_marketplace,
            install_marketplace_skill,
            set_project_skill_enabled,
            check_ai_status,
            open_codex_login,
            chat_with_codex
        ])
        .run(tauri::generate_context!())
        .expect("error while running Estate Studio");
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn slug_is_safe_and_stable() {
        assert_eq!(slugify("Northbank Residences"), "northbank-residences");
        assert_eq!(slugify(" Apartment 106 "), "apartment-106");
        assert_eq!(slugify("北岸项目"), "development");
    }

    #[test]
    fn required_values_are_trimmed() {
        assert_eq!(clean_required("  Koya  ", "Name").unwrap(), "Koya");
        assert!(clean_required("   ", "Name").is_err());
    }

    #[test]
    fn legacy_manifest_receives_migration_safe_project_defaults() {
        let current = koya_manifest();
        let mut legacy = serde_json::to_value(current).unwrap();
        let object = legacy.as_object_mut().unwrap();
        object.insert("schemaVersion".into(), serde_json::json!(12));
        for key in [
            "companyProfile",
            "locale",
            "measurementUnits",
            "accessMode",
            "disclosure",
            "designSpec",
            "approvalEvents",
            "qaRecords",
        ] {
            object.remove(key);
        }
        let parsed: ProjectManifest = serde_json::from_value(legacy).unwrap();
        let migrated = normalize_manifest(parsed);
        assert_eq!(migrated.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(migrated.company_profile.id, "koya");
        assert_eq!(migrated.company_profile.name, "Koya");
        assert_eq!(migrated.locale, "en-AU");
        assert_eq!(migrated.measurement_units, "metric");
        assert_eq!(migrated.access_mode, "unlisted");
        assert_eq!(migrated.disclosure, "Disclosure not supplied");
        assert_eq!(migrated.design_spec.status, "not_started");
        assert!(migrated.approval_events.is_empty());
        assert!(migrated.qa_records.is_empty());
    }

    #[test]
    fn source_checksum_and_mime_metadata_are_deterministic() {
        let path = std::env::temp_dir().join(format!(
            "estate-studio-checksum-{}-{}.pdf",
            std::process::id(),
            unix_time()
        ));
        fs::write(&path, b"estate studio evidence fixture").unwrap();
        assert_eq!(
            sha256_file(&path).unwrap(),
            "64befdb8a6a3d2e8f0b98cf855f74fe1f19ee4f4f5534b15e56ee4a2775a3291"
        );
        assert_eq!(source_mime_type(&path), "application/pdf");
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn source_import_preflight_rejects_invalid_images_and_category_mismatches() {
        let root = std::env::temp_dir().join(format!(
            "estate-studio-invalid-source-{}-{}",
            std::process::id(),
            unix_time()
        ));
        fs::create_dir_all(&root).unwrap();
        let invalid_png = root.join("broken.png");
        fs::write(&invalid_png, b"not a png").unwrap();
        let decode_error = validate_source_for_import(&invalid_png, "renders").unwrap_err();
        assert!(decode_error.contains("cannot be decoded as an image"));
        assert!(decode_error.contains("Choose an intact PNG, JPEG, or WebP file"));

        let wrong_category = root.join("notes.txt");
        fs::write(&wrong_category, b"plain text").unwrap();
        let category_error = validate_source_for_import(&wrong_category, "panoramas").unwrap_err();
        assert!(category_error.contains("not a supported panoramas source"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn structure_only_duplicate_does_not_copy_customer_content() {
        let source = koya_manifest();
        let duplicate = structure_only_duplicate_manifest(
            &source,
            "Northbank".into(),
            "Example Agency".into(),
            42,
        );
        let encoded = serde_json::to_string(&duplicate).unwrap();
        assert_eq!(duplicate.units.len(), source.units.len());
        assert!(duplicate
            .units
            .iter()
            .all(|unit| unit.rooms.is_empty() && unit.floorplan_asset_id.is_none()));
        assert!(duplicate.assets.is_empty());
        assert!(duplicate.generation_jobs.is_empty());
        assert!(duplicate.creative_jobs.is_empty());
        assert!(duplicate.releases.is_empty());
        assert!(duplicate.approval_events.is_empty());
        assert!(duplicate.qa_records.is_empty());
        assert!(!encoded.contains("Apartment 106"));
        assert!(!encoded.contains("Koya"));
        assert!(!encoded.contains("hades217"));
        assert_eq!(duplicate.disclosure, "Disclosure not supplied");
        assert_eq!(duplicate.brand.primary, "#20241f");
    }

    #[test]
    fn bundle_paths_and_secret_files_fail_closed() {
        assert!(portable_relative_path(Path::new("reports/qa.json")).is_ok());
        assert!(portable_relative_path(Path::new("../outside.json")).is_err());
        assert!(portable_relative_path(Path::new("/absolute.json")).is_err());
        assert!(portable_relative_path(Path::new("bundle.json")).is_err());
        let root = std::env::temp_dir().join(format!(
            "estate-studio-secret-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        fs::create_dir_all(&root).unwrap();
        let secret = root.join(".env");
        fs::write(&secret, b"OPENAI_API_KEY=redacted-test-value").unwrap();
        assert!(scan_bundle_file_for_secrets(Path::new(".env"), &secret).is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn portable_bundle_round_trip_preserves_identity_and_rejects_collision() {
        let base = std::env::temp_dir().join(format!(
            "estate-studio-bundle-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        let source_library = base.join("source-library");
        let import_library = base.join("import-library");
        fs::create_dir_all(&source_library).unwrap();
        fs::create_dir_all(&import_library).unwrap();
        let manifest = structure_only_duplicate_manifest(
            &koya_manifest(),
            "Northbank".into(),
            "Example Agency".into(),
            4242,
        );
        let project_root = source_library.join(&manifest.project_id);
        let unit_ids = manifest
            .units
            .iter()
            .map(|unit| unit.id.clone())
            .collect::<Vec<_>>();
        create_project_directories(&project_root, &unit_ids).unwrap();
        write_manifest(&project_root, &manifest).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(project_root.join("project.json"))
                    .unwrap()
                    .permissions()
                    .mode()
                    & 0o777,
                0o600
            );
        }
        fs::write(
            project_root.join("reports/qa.json"),
            b"{\"status\":\"pass\"}",
        )
        .unwrap();
        fs::write(project_root.join("cache/disposable.txt"), b"ignore").unwrap();
        let bundle_path = base.join("northbank.estateproject");
        write_project_bundle(&project_root, &manifest.project_id, &bundle_path).unwrap();
        let imported = import_project_bundle_inner(&import_library, &bundle_path).unwrap();
        assert_eq!(imported.manifest.project_id, manifest.project_id);
        assert_eq!(imported.manifest.schema_version, CURRENT_SCHEMA_VERSION);
        assert!(Path::new(&imported.project_root)
            .join("reports/qa.json")
            .is_file());
        assert!(!Path::new(&imported.project_root)
            .join("cache/disposable.txt")
            .exists());
        assert!(import_project_bundle_inner(&import_library, &bundle_path).is_err());
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn company_profiles_and_project_roots_remain_isolated() {
        let base = std::env::temp_dir().join(format!(
            "estate-studio-company-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        fs::create_dir_all(&base).unwrap();
        let registry_path = base.join("companies.json");
        let profile = |id: &str, name: &str, primary: &str, timestamp: u64| CompanyProfile {
            id: id.into(),
            name: name.into(),
            locale: "en-AU".into(),
            measurement_units: "metric".into(),
            brand: Brand {
                primary: primary.into(),
                accent: "#78917b".into(),
            },
            created_at: timestamp,
            updated_at: timestamp,
        };
        upsert_company_profile_at(
            &registry_path,
            profile("company-a", "Company A", "#111111", 1),
        )
        .unwrap();
        upsert_company_profile_at(
            &registry_path,
            profile("company-b", "Company B", "#222222", 2),
        )
        .unwrap();
        upsert_company_profile_at(
            &registry_path,
            profile("company-a", "Company A", "#333333", 3),
        )
        .unwrap();
        let registry = read_company_registry(&registry_path).unwrap();
        assert_eq!(registry.companies.len(), 2);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(
                fs::metadata(&registry_path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        let company_a = registry
            .companies
            .iter()
            .find(|item| item.id == "company-a")
            .unwrap();
        assert_eq!(company_a.created_at, 1);
        assert_eq!(company_a.updated_at, 3);
        assert_eq!(company_a.brand.primary, "#333333");

        let project_a = base.join("projects/company-a-project");
        let project_b = base.join("projects/company-b-project");
        fs::create_dir_all(project_a.join("reports")).unwrap();
        fs::create_dir_all(project_b.join("reports")).unwrap();
        fs::write(
            project_a.join("project.json"),
            b"{\"projectId\":\"company-a-project\"}",
        )
        .unwrap();
        fs::write(project_a.join("reports/a.txt"), b"company-a-only").unwrap();
        fs::write(
            project_b.join("project.json"),
            b"{\"projectId\":\"company-b-project\"}",
        )
        .unwrap();
        fs::write(project_b.join("reports/b.txt"), b"company-b-confidential").unwrap();
        let bundle = build_bundle_manifest(&project_a, "company-a-project").unwrap();
        assert!(bundle
            .files
            .iter()
            .all(|record| !record.path.contains("company-b")));
        assert!(bundle
            .files
            .iter()
            .all(|record| !record.checksum_sha256.is_empty()));
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn bundled_koya_example_is_versioned_read_only_and_rights_bounded() {
        let manifest = koya_manifest();
        assert_eq!(manifest.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(manifest.project_id, "koya-example");
        assert!(manifest.read_only);
        assert_eq!(manifest.units.len(), 4);
        assert_eq!(manifest.design_spec.status, "approved");
        assert_eq!(manifest.design_spec.colours.paper, "#f3efe5");
        assert_eq!(
            manifest.tour_preview_url.as_deref(),
            Some("/embedded-tour/index.html?unit=106&mode=video&tour=entry-room")
        );
        assert_eq!(
            manifest
                .releases
                .last()
                .and_then(|release| release.public_url.as_deref()),
            Some("https://hades217.github.io/Koya/?unit=106&mode=video&tour=entry-room")
        );
        let living_video = manifest
            .assets
            .iter()
            .find(|asset| asset.id == "koya-living-terrace-reference-video")
            .expect("Koya living-room video must be bundled in the example manifest");
        assert_eq!(living_video.category, "videos");
        assert_eq!(living_video.evidence_class, "concept_style_only");
        assert_eq!(living_video.status, "accepted");
        assert_eq!(living_video.relative_path, KOYA_LIVING_VIDEO_PATH);
        assert_eq!(living_video.size_bytes, KOYA_LIVING_VIDEO.len() as u64);
        assert_eq!(
            manifest
                .units
                .iter()
                .find(|unit| unit.id == "106")
                .map(|unit| unit.rooms.len()),
            Some(6)
        );
        assert_eq!(
            manifest
                .units
                .iter()
                .find(|unit| unit.id == "106")
                .and_then(|unit| unit.rooms.iter().find(|room| room.id == "living-room"))
                .and_then(|room| room.video_asset_id.as_deref()),
            Some("koya-living-terrace-reference-video")
        );
        let rights = include_str!("../../resources/examples/koya-example-v1/RIGHTS_AUDIT.md");
        assert!(rights.contains("internal_only"));
        assert!(rights.contains("Commercial redistribution approval"));
        assert!(rights.contains("blocked from portable export"));
    }

    #[test]
    fn property_ai_envelope_parses_markdown_and_project_draft() {
        let envelope = parse_ai_model_envelope(
            r#"```json
{"replyMarkdown":"**Draft ready.**","projectUpdateDraft":{"summary":"Update location","baseUpdatedAt":0,"name":null,"company":null,"location":"Brisbane, QLD","primary":null,"accent":null,"warnings":[]}}
```"#,
        )
        .unwrap();
        assert_eq!(envelope.reply_markdown, "**Draft ready.**");
        assert_eq!(
            envelope.project_update_draft.unwrap().location.as_deref(),
            Some("Brisbane, QLD")
        );
        assert!(parse_ai_model_envelope("not json").is_err());
    }

    #[test]
    fn property_ai_draft_is_scoped_normalized_and_stale_guarded() {
        let mut manifest = koya_manifest();
        manifest.read_only = false;
        manifest.updated_at = 42;
        manifest.location = "Australia".into();
        let normalized = normalize_ai_project_draft(
            &manifest,
            AiProjectUpdateDraft {
                summary: "  Update the project location  ".into(),
                base_updated_at: 0,
                name: Some(manifest.name.clone()),
                company: None,
                location: Some("  Brisbane, QLD  ".into()),
                primary: None,
                accent: Some("#AABBCC".into()),
                warnings: vec!["  Confirm this with the client.  ".into()],
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(normalized.base_updated_at, 42);
        assert!(normalized.name.is_none());
        assert_eq!(normalized.location.as_deref(), Some("Brisbane, QLD"));
        assert_eq!(normalized.accent.as_deref(), Some("#aabbcc"));
        assert_eq!(normalized.warnings, vec!["Confirm this with the client."]);
        assert!(ensure_current_project(Some(42), manifest.updated_at).is_ok());
        assert!(ensure_current_project(None, manifest.updated_at).is_ok());
        assert!(ensure_current_project(Some(41), manifest.updated_at).is_err());
    }

    #[test]
    fn project_and_unit_directories_are_isolated() {
        let root = std::env::temp_dir().join(format!(
            "estate-studio-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        create_project_directories(&root, &["201".into()]).unwrap();
        assert!(root.join("sources/drawings").is_dir());
        assert!(root.join("creative/design-system").is_dir());
        assert!(root.join("units/201/floorplan").is_dir());
        assert!(root.join("units/201/assets/panoramas").is_dir());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn large_project_navigation_reads_metadata_without_loading_media_files() {
        let root = std::env::temp_dir().join(format!(
            "estate-studio-large-project-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        fs::create_dir_all(&root).unwrap();
        let mut manifest = koya_manifest();
        manifest.read_only = false;
        let unit_template = manifest.units[0].clone();
        manifest.units = (0..20)
            .map(|index| UnitRecord {
                id: format!("type-{index:02}"),
                label: format!("Type {index:02}"),
                rooms: vec![],
                floorplan_asset_id: None,
                tour_available: false,
                ..unit_template.clone()
            })
            .collect();
        manifest.assets = (0..300)
            .map(|index| AssetRecord {
                id: format!("asset-{index:03}"),
                name: format!("source-{index:03}.webp"),
                category: "photos".into(),
                evidence_class: "official".into(),
                status: "accepted".into(),
                imported_at: 1,
                relative_path: format!("sources/photos/source-{index:03}.webp"),
                size_bytes: 1_000_000,
                checksum_sha256: format!("{index:064x}"),
                mime_type: "image/webp".into(),
                source_owner: "Test owner".into(),
                usage_permission: "Test-only fixture".into(),
                original_relative_path: format!("sources/photos/source-{index:03}.webp"),
                derivative_relative_paths: vec![],
                duplicate_of_asset_id: None,
                unit_id: Some(format!("type-{:02}", index % 20)),
                width: Some(2048),
                height: Some(1024),
                rejection_reason_code: None,
                rejection_notes: None,
                review_events: vec![],
            })
            .collect();
        write_manifest(&root, &manifest).unwrap();
        let loaded = read_manifest(&root).unwrap();
        assert_eq!(loaded.units.len(), 20);
        assert_eq!(loaded.assets.len(), 300);
        assert!(loaded
            .assets
            .iter()
            .all(|asset| !root.join(&asset.relative_path).exists()));
        fs::remove_dir_all(root).unwrap();
    }

    proptest! {
        #[test]
        fn arbitrary_unit_ids_cannot_escape_the_project_root(raw in ".{0,180}") {
            match safe_path_component(&raw, "Unit ID") {
                Ok(component) => {
                    prop_assert!(!component.is_empty());
                    prop_assert!(component.len() <= 128);
                    prop_assert!(component.chars().all(|character|
                        character.is_ascii_alphanumeric() || matches!(character, '-' | '_')));
                    prop_assert_eq!(Path::new(&component).components().count(), 1);
                }
                Err(_) => {
                    let trimmed = raw.trim();
                    prop_assert!(trimmed.is_empty()
                        || trimmed.len() > 128
                        || trimmed.chars().any(|character|
                            !character.is_ascii_alphanumeric() && !matches!(character, '-' | '_')));
                }
            }
        }
    }

    #[test]
    fn duplicate_background_jobs_have_one_stable_idempotency_identity() {
        let first = background_job_idempotency_key("project-a", "upload", "release-1");
        let duplicate = background_job_idempotency_key("project-a", "upload", "release-1");
        let other_project = background_job_idempotency_key("project-b", "upload", "release-1");
        let other_subject = background_job_idempotency_key("project-a", "upload", "release-2");
        assert_eq!(first, duplicate);
        assert_ne!(first, other_project);
        assert_ne!(first, other_subject);
        assert_eq!(first.len(), 64);
    }

    #[test]
    fn imported_floorplan_increases_readiness_without_marking_tour_ready() {
        let mut manifest = koya_manifest();
        manifest.read_only = false;
        manifest.readiness = 12;
        manifest.modules.tour = "not_started".into();
        for unit in &mut manifest.units {
            unit.rooms.clear();
            unit.floorplan_asset_id = None;
        }
        manifest.units[0].floorplan_asset_id = Some("asset-test".into());
        manifest.assets = vec![AssetRecord {
            id: "asset-test".into(),
            name: "unit-102.pdf".into(),
            category: "drawings".into(),
            evidence_class: "official".into(),
            status: "imported".into(),
            imported_at: unix_time(),
            relative_path: "units/102/floorplan/unit-102.pdf".into(),
            size_bytes: 100,
            checksum_sha256: "test-checksum".into(),
            mime_type: "application/pdf".into(),
            source_owner: "Test owner".into(),
            usage_permission: "Test fixture".into(),
            original_relative_path: "units/102/floorplan/unit-102.pdf".into(),
            derivative_relative_paths: vec![],
            duplicate_of_asset_id: None,
            unit_id: Some("102".into()),
            width: None,
            height: None,
            rejection_reason_code: None,
            rejection_notes: None,
            review_events: vec![],
        }];
        update_readiness(&mut manifest);
        assert_eq!(manifest.readiness, 20);
        assert_eq!(manifest.modules.tour, "in_progress");
        assert!(!manifest.units[0].tour_available);
    }

    #[test]
    fn room_supporting_media_is_role_and_rights_scoped() {
        let asset = |category: &str, mime_type: &str| AssetRecord {
            id: "asset-room-media".into(),
            name: "room-media".into(),
            category: category.into(),
            evidence_class: "concept_style_only".into(),
            status: "accepted".into(),
            imported_at: unix_time(),
            relative_path: "sources/room-media".into(),
            size_bytes: 100,
            checksum_sha256: "test-checksum".into(),
            mime_type: mime_type.into(),
            source_owner: "Test owner".into(),
            usage_permission: "Test fixture".into(),
            original_relative_path: "sources/room-media".into(),
            derivative_relative_paths: vec![],
            duplicate_of_asset_id: None,
            unit_id: Some("a1".into()),
            width: None,
            height: None,
            rejection_reason_code: None,
            rejection_notes: None,
            review_events: vec![],
        };
        assert!(
            validate_room_supporting_asset(&asset("videos", "video/mp4"), "a1", "video").is_ok()
        );
        assert!(
            validate_room_supporting_asset(&asset("videos", "video/quicktime"), "a1", "video")
                .is_err()
        );
        assert!(validate_room_supporting_asset(
            &asset("renders", "image/png"),
            "a1",
            "still_fallback"
        )
        .is_ok());
        assert!(
            validate_room_supporting_asset(&asset("renders", "image/png"), "a2", "poster").is_err()
        );
        assert!(validate_room_supporting_asset(
            &asset("renders", "image/png"),
            "a1",
            "unsupported"
        )
        .is_err());
    }

    #[test]
    fn room_graph_must_be_fully_connected_before_locking() {
        let room = |id: &str, adjacent: &[&str]| RoomRecord {
            id: id.into(),
            name: id.into(),
            status: "needs_evidence".into(),
            panorama_status: "not_started".into(),
            identity_asset_id: None,
            hotspot_x: Some(50.0),
            hotspot_y: Some(50.0),
            adjacent_room_ids: adjacent.iter().map(|value| value.to_string()).collect(),
            panorama_asset_id: None,
            threshold_asset_id: None,
            reverse_asset_id: None,
            still_fallback_asset_id: None,
            video_asset_id: None,
            poster_asset_id: None,
        };
        assert!(validate_connected_room_graph(&[
            room("entry", &["living"]),
            room("living", &["entry", "terrace"]),
            room("terrace", &["living"]),
        ])
        .is_ok());
        assert!(validate_connected_room_graph(&[
            room("entry", &["living"]),
            room("living", &["entry"]),
            room("terrace", &[]),
        ])
        .is_err());
        assert!(
            validate_connected_room_graph(&[room("entry", &["living"]), room("living", &[]),])
                .is_err()
        );
    }

    fn test_generation_job(status: &str) -> GenerationJob {
        serde_json::from_value(serde_json::json!({
            "id": "job-test",
            "unitId": "106",
            "roomId": "living",
            "roomName": "Living",
            "assetRole": "panorama",
            "outputCount": 1,
            "dimensions": "8192 × 4096",
            "panoramaMode": "one-shot",
            "connectionMode": "managed_openai",
            "priceStatus": "available",
            "status": status,
            "createdAt": 1,
            "approvalFingerprint": "package-fingerprint",
            "idempotencyKey": "attempt-key-1",
            "attempt": 1,
            "progressPercent": 0,
            "updatedAt": 1
        }))
        .unwrap()
    }

    #[test]
    fn generation_state_machine_is_monotonic_and_audited() {
        let mut job = test_generation_job("blocked_capability");
        transition_generation_job(
            &mut job,
            "awaiting_approval",
            0,
            "capability_registry",
            None,
            2,
        )
        .unwrap();
        transition_generation_job(&mut job, "approved", 0, "local_user", None, 3).unwrap();
        transition_generation_job(&mut job, "submitted", 5, "provider_adapter", None, 4).unwrap();
        transition_generation_job(&mut job, "processing", 40, "provider_adapter", None, 5).unwrap();
        assert!(
            transition_generation_job(&mut job, "approved", 40, "provider_adapter", None, 6)
                .is_err()
        );
        assert!(
            transition_generation_job(&mut job, "completed", 20, "provider_adapter", None, 7)
                .is_err()
        );
        transition_generation_job(&mut job, "completed", 100, "provider_adapter", None, 8).unwrap();
        assert_eq!(job.progress_percent, 100);
        assert_eq!(job.state_events.len(), 5);
        assert!(!job.retry_allowed);
    }

    #[test]
    fn paid_package_identity_changes_with_quote_and_attempt() {
        let mut job = test_generation_job("awaiting_approval");
        job.provider_choice = "managed_openai".into();
        job.model_id = Some("verified-model".into());
        job.capability_status = "available".into();
        job.price_amount_minor = Some(400);
        job.price_currency = Some("AUD".into());
        job.customer_credit_cost = Some(4);
        refresh_generation_approval_identity("project-test", &mut job).unwrap();
        let first_fingerprint = job.approval_fingerprint.clone();
        let first_idempotency = job.idempotency_key.clone();
        job.price_amount_minor = Some(450);
        refresh_generation_approval_identity("project-test", &mut job).unwrap();
        assert_ne!(job.approval_fingerprint, first_fingerprint);
        assert_ne!(job.idempotency_key, first_idempotency);
        let quoted_fingerprint = job.approval_fingerprint.clone();
        job.attempt = 2;
        refresh_generation_approval_identity("project-test", &mut job).unwrap();
        assert_ne!(job.approval_fingerprint, quoted_fingerprint);
        assert!(!generation_transition_allowed("queued", "queued"));
    }

    #[test]
    fn panorama_seam_metric_compares_wrapped_edges() {
        let path = std::env::temp_dir().join(format!(
            "estate-studio-seam-metric-{}.png",
            std::process::id()
        ));
        let mut image = image::RgbaImage::from_pixel(4, 2, image::Rgba([20, 30, 40, 255]));
        image.save(&path).unwrap();
        assert_eq!(panorama_seam_edge_delta(&path).unwrap(), 0.0);
        image.put_pixel(3, 0, image::Rgba([255, 255, 255, 255]));
        image.put_pixel(3, 1, image::Rgba([255, 255, 255, 255]));
        image.save(&path).unwrap();
        assert!(panorama_seam_edge_delta(&path).unwrap() > 0.8);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn deterministic_panorama_processing_preserves_two_to_one_contract() {
        let mut source = image::RgbaImage::from_pixel(16, 8, image::Rgba([50, 60, 70, 255]));
        for y in 0..8 {
            source.put_pixel(0, y, image::Rgba([0, 0, 0, 255]));
            source.put_pixel(15, y, image::Rgba([200, 200, 200, 255]));
        }
        let repaired = deterministic_seam_repair(source).unwrap();
        assert_eq!(repaired.get_pixel(0, 4), repaired.get_pixel(15, 4));
        let (upscaled, process) = deterministic_panorama_resize(&repaired, 32, 16).unwrap();
        assert_eq!(upscaled.dimensions(), (32, 16));
        assert!(process.contains("no_new_detail"));
        assert!(deterministic_seam_repair(image::RgbaImage::new(8, 8)).is_err());
        assert!(deterministic_panorama_resize(&repaired, 32, 32).is_err());
    }

    #[test]
    fn panorama_fallback_tasks_are_separate_and_continuity_linked() {
        let job = test_generation_job("completed");
        let cubefaces =
            create_fallback_tasks("project-test", &job, "output-1", "cubefaces").unwrap();
        assert_eq!(cubefaces.len(), 6);
        assert!(cubefaces.iter().all(
            |task| task.approval_state == "not_approved" && !task.adjacent_task_ids.is_empty()
        ));
        assert_eq!(
            cubefaces
                .iter()
                .map(|task| &task.approval_fingerprint)
                .collect::<HashSet<_>>()
                .len(),
            6
        );
        let tiles =
            create_fallback_tasks("project-test", &job, "output-1", "overlapping_tiles").unwrap();
        assert!(tiles.iter().all(|task| task.overlap_degrees == 16.0));
        assert!(create_fallback_tasks("project-test", &job, "output-1", "unknown").is_err());
    }

    #[test]
    fn background_jobs_resume_same_idempotent_work_from_checkpoint() {
        let mut job = BackgroundJobRecord {
            id: "background-1".into(),
            kind: "upload".into(),
            subject_id: "release-1".into(),
            idempotency_key: "stable-key".into(),
            status: "queued".into(),
            attempt: 1,
            progress_percent: 0,
            checkpoint: "queued".into(),
            failure_reason: None,
            lease_expires_at: None,
            created_at: 1,
            updated_at: 1,
        };
        transition_background_job(&mut job, "running", 35, "uploaded_part_3".into(), 2).unwrap();
        transition_background_job(
            &mut job,
            "queued",
            35,
            "resume_from:uploaded_part_3".into(),
            1_000,
        )
        .unwrap();
        assert_eq!(job.idempotency_key, "stable-key");
        assert_eq!(job.progress_percent, 35);
        assert_eq!(job.checkpoint, "resume_from:uploaded_part_3");
        assert!(!background_transition_allowed("completed", "queued"));
    }

    #[test]
    fn static_tour_runtime_is_manifest_driven_and_deep_linked() {
        let html = include_str!("../../resources/tour-runtime/index.html");
        let script = include_str!("../../resources/tour-runtime/tour.js");
        assert!(html.contains("tour-manifest.js"));
        assert!(script.contains("__ESTATE_TOUR_MANIFEST__"));
        assert!(script.contains("tour-manifest.json"));
        assert!(script.contains("URLSearchParams"));
        assert!(script.contains("unit:"));
        assert!(script.contains("room:"));
        assert!(script.contains("mode:"));
        assert!(script.contains("getContext('webgl'"));
        assert!(script.contains("stillFallback"));
        assert!(script.contains("room.video"));
        assert!(!script.contains("Apartment 106"));
        assert!(!script.contains("Koya"));
    }

    #[test]
    fn static_tour_runtime_supports_preview_presets_and_accessible_controls() {
        let html = include_str!("../../resources/tour-runtime/index.html");
        let css = include_str!("../../resources/tour-runtime/tour.css");
        let script = include_str!("../../resources/tour-runtime/tour.js");
        assert!(html.contains("id=\"fullscreen\""));
        assert!(html.contains("aria-live=\"polite\""));
        assert!(css.contains("touch-action: none"));
        assert!(css.contains("body.reduced-motion"));
        assert!(script.contains("navigator.deviceMemory"));
        assert!(script.contains("texture === 'mobile'"));
        assert!(script.contains("pointercancel"));
        assert!(script.contains("ArrowLeft"));
        assert!(script.contains("requestFullscreen"));
        assert!(script.contains("prefers-reduced-motion"));
        assert!(script.contains("escapeHtml"));
        assert!(script.contains("estate-studio:analytics"));
        assert!(script.contains("credentials: 'omit'"));
        assert!(script.contains("endpoint.origin !== location.origin"));
        assert!(!script.contains("navigator.userAgent"));
        assert!(!script.contains("document.referrer"));
    }

    #[test]
    fn privacy_safe_funnel_events_are_allowlisted_and_first_occurrence_only() {
        let mut manifest = legacy_koya_manifest_fixture();
        manifest.analytics_events.clear();
        assert!(
            record_funnel_event(&mut manifest, "project_created", "desktop_workflow", 1).unwrap()
        );
        assert!(
            !record_funnel_event(&mut manifest, "project_created", "desktop_workflow", 2).unwrap()
        );
        assert!(record_funnel_event(&mut manifest, "raw_url", "desktop_workflow", 3).is_err());
        assert_eq!(manifest.analytics_events.len(), 1);
        assert_eq!(manifest.analytics_events[0].event, "project_created");
        let encoded = serde_json::to_string(&manifest.analytics_events).unwrap();
        assert!(!encoded.contains("token"));
        assert!(!encoded.contains("url"));
    }

    #[test]
    fn static_build_texture_and_immutable_destination_gates_fail_closed() {
        assert_eq!(
            expected_static_tier_dimensions("mobile").unwrap(),
            (2048, 1024)
        );
        assert_eq!(expected_static_tier_dimensions("4k").unwrap(), (4096, 2048));
        assert_eq!(expected_static_tier_dimensions("8k").unwrap(), (8192, 4096));
        assert!(expected_static_tier_dimensions("other").is_err());
        assert_eq!(static_texture_memory(4096, 2048).unwrap(), 33_554_432);
        assert!(static_texture_memory(2048, 2048).is_err());
        assert!(static_texture_memory(16_384, 8_192).is_err());

        let root = std::env::temp_dir().join(format!(
            "estate-studio-immutable-build-test-{}-{}",
            std::process::id(),
            unix_time()
        ));
        let source = root.join("source");
        let destination = root.join("destination");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("index.html"), "safe").unwrap();
        fs::create_dir_all(&destination).unwrap();
        assert!(copy_static_build(&source, &destination).is_err());
        fs::remove_dir_all(&destination).unwrap();
        copy_static_build(&source, &destination).unwrap();
        assert_eq!(
            fs::read_to_string(destination.join("index.html")).unwrap(),
            "safe"
        );
        assert!(copy_static_build(&source, &destination).is_err());
        assert_eq!(validate_publish_access_mode("public").unwrap(), "public");
        assert_eq!(
            validate_publish_access_mode("unlisted").unwrap(),
            "unlisted"
        );
        assert_eq!(validate_publish_access_mode("private").unwrap(), "private");
        assert!(validate_publish_access_mode("local").is_err());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn deployment_readback_urls_are_https_origin_scoped() {
        let base = validate_deployment_url("https://customer.example.test/tour/").unwrap();
        assert_eq!(
            deployment_child_url(&base, "assets/panorama.webp", false)
                .unwrap()
                .as_str(),
            "https://customer.example.test/tour/assets/panorama.webp"
        );
        assert!(validate_deployment_url("http://customer.example.test/tour/").is_err());
        assert!(validate_deployment_url("https://localhost/tour/").is_err());
        assert!(validate_deployment_url("https://127.0.0.1/tour/").is_err());
        assert!(validate_deployment_url("https://10.0.0.4/tour/").is_err());
        assert!(validate_deployment_url("https://user:secret@example.test/tour/").is_err());
        assert!(deployment_child_url(&base, "../other-project/manifest.json", false).is_err());
    }

    #[test]
    fn verified_release_qr_is_real_svg_without_plaintext_link() {
        let link = "https://customer.example.test/tour/?unit=type-a&room=living&mode=panorama";
        let svg = qr_svg(link).unwrap();
        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<path"));
        assert!(!svg.contains(link));
    }

    #[test]
    fn signed_offline_license_rejects_tampering_expiry_and_edition_overreach() {
        use ed25519_dalek::{Signer, SigningKey};
        let seed =
            decode_hex_32("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
                .unwrap();
        let signing = SigningKey::from_bytes(&seed);
        assert_eq!(
            signing.verifying_key().to_bytes(),
            decode_hex_32(LICENSE_PUBLIC_KEY_HEX).unwrap()
        );
        let sign = |claims: LicenseClaims| {
            let payload = serde_json::to_vec(&claims).unwrap();
            SignedLicenseDocument {
                claims,
                signature_base64: base64::engine::general_purpose::STANDARD
                    .encode(signing.sign(&payload).to_bytes()),
            }
        };
        let claims = LicenseClaims {
            schema_version: 1,
            license_id: "license-test-1".into(),
            customer: "Test Customer".into(),
            edition: "professional".into(),
            issued_at: 100,
            expires_at: Some(1_000),
            entitlements: vec!["static_export".into(), "deployment_readback".into()],
            allowed_roles: vec!["owner".into(), "publisher".into()],
        };
        let document = sign(claims.clone());
        assert_eq!(
            verify_license_document(&document, 500).unwrap().edition,
            "professional"
        );
        let mut tampered = document.clone();
        tampered.claims.customer = "Different Customer".into();
        assert!(verify_license_document(&tampered, 500).is_err());
        assert!(verify_license_document(&document, 1_001).is_err());
        let mut overreach = claims;
        overreach.entitlements.push("private_links".into());
        assert!(verify_license_document(&sign(overreach), 500).is_err());
    }

    #[test]
    fn desktop_settings_keep_gateway_and_analytics_endpoints_secret_free_and_scoped() {
        assert_eq!(
            normalize_managed_gateway_url(" https://gateway.example.test/api/ ").unwrap(),
            "https://gateway.example.test/api"
        );
        assert!(normalize_managed_gateway_url("http://gateway.example.test").is_err());
        assert!(normalize_managed_gateway_url("https://user:token@gateway.example.test").is_err());
        assert!(
            normalize_managed_gateway_url("https://gateway.example.test?token=secret").is_err()
        );
        assert_eq!(
            normalize_analytics_endpoint(" /api/tour-events ").unwrap(),
            "/api/tour-events"
        );
        assert!(normalize_analytics_endpoint("https://analytics.example.test/events").is_err());
        assert!(normalize_analytics_endpoint("//analytics.example.test/events").is_err());
        assert!(normalize_analytics_endpoint("/api/../secret").is_err());
        assert!(normalize_analytics_endpoint("/events?token=secret").is_err());
    }

    #[test]
    fn restart_recovery_times_out_same_attempt_without_resubmission() {
        let mut manifest = legacy_koya_manifest_fixture();
        let mut job = test_generation_job("submitted");
        job.progress_percent = 35;
        job.timeout_at = Some(10);
        manifest.generation_jobs = vec![job];
        assert_eq!(recover_timed_out_jobs(&mut manifest, 11).unwrap(), 1);
        let recovered = &manifest.generation_jobs[0];
        assert_eq!(recovered.status, "timed_out");
        assert_eq!(recovered.idempotency_key, "attempt-key-1");
        assert_eq!(recovered.attempt, 1);
        assert_eq!(recovered.progress_percent, 35);
        assert!(recovered.retry_allowed);
        assert_eq!(recover_timed_out_jobs(&mut manifest, 12).unwrap(), 0);
    }

    #[test]
    fn capability_registry_requires_exact_fresh_price_and_quota_evidence() {
        let mut registry = CapabilityRegistry {
            schema_version: 1,
            records: vec![GenerationCapabilityRecord {
                provider_choice: "managed_openai".into(),
                model_id: "image-model-test".into(),
                access_status: "available".into(),
                supported_sizes: vec![ImageSizeCapability {
                    width: PREFERRED_PROVIDER_PANORAMA_WIDTH,
                    height: PREFERRED_PROVIDER_PANORAMA_HEIGHT,
                }],
                panorama_modes: vec!["one_shot_2_1".into()],
                price_status: "available".into(),
                price_amount_minor: Some(125),
                price_currency: Some("AUD".into()),
                quota_status: "available".into(),
                quota_remaining: Some(2),
                checked_at: 90,
                expires_at: 200,
                source: "test_adapter".into(),
            }],
            updated_at: 90,
        };
        assert!(evaluate_generation_capability(
            &registry,
            "managed_openai",
            "image-model-test",
            PREFERRED_PROVIDER_PANORAMA_WIDTH,
            PREFERRED_PROVIDER_PANORAMA_HEIGHT,
            "one_shot_2_1",
            1,
            100,
        )
        .is_ok());
        assert!(evaluate_generation_capability(
            &registry,
            "managed_openai",
            "image-model-test",
            4096,
            2048,
            "one_shot_2_1",
            1,
            100
        )
        .is_err());
        registry.records[0].price_status = "unavailable".into();
        assert!(evaluate_generation_capability(
            &registry,
            "managed_openai",
            "image-model-test",
            8192,
            4096,
            "one_shot_2_1",
            1,
            100
        )
        .is_err());
        registry.records[0].price_status = "available".into();
        registry.records[0].quota_remaining = Some(0);
        assert!(evaluate_generation_capability(
            &registry,
            "managed_openai",
            "image-model-test",
            8192,
            4096,
            "one_shot_2_1",
            1,
            100
        )
        .is_err());
        registry.records[0].quota_remaining = Some(2);
        assert!(evaluate_generation_capability(
            &registry,
            "managed_openai",
            "image-model-test",
            8192,
            4096,
            "one_shot_2_1",
            1,
            201
        )
        .is_err());
    }

    #[test]
    fn creative_formats_are_explicit_and_paid_state_is_absent() {
        assert_eq!(SALES_DOCUMENT_TYPES.len(), 7);
        assert!(SALES_DOCUMENT_TYPES
            .iter()
            .all(|document_type| supported_sales_document_type(document_type)));
        assert_eq!(
            creative_dimensions("poster", Some("a4_sales_sheet"), "a4_portrait").unwrap(),
            (2480, 3508, None)
        );
        assert_eq!(
            creative_dimensions("poster", Some("ai_creative"), "instagram_portrait").unwrap(),
            (1080, 1350, None)
        );
        assert_eq!(
            creative_dimensions("video", None, "social_vertical").unwrap(),
            (1080, 1920, Some(15))
        );
        assert!(
            creative_dimensions("poster", Some("a4_sales_sheet"), "instagram_portrait").is_err()
        );
        assert!(creative_dimensions("video", None, "cinema").is_err());
    }

    #[test]
    fn a4_markdown_is_source_of_truth_for_html_preview() {
        let manifest = koya_manifest();
        let brief = &manifest.creative_jobs[0];
        let sot = sales_materials_sot_markdown(&manifest, brief);
        let sot_sha256 = sha256_text(&sot);
        let markdown = sales_sheet_markdown(&manifest, brief, &sot_sha256);
        let html = sales_sheet_html(&manifest, brief, &markdown, &sot_sha256);
        let markdown_sha256 = sha256_text(&markdown);
        assert!(sot.contains("# Sales materials source of truth"));
        assert!(sot.contains("Missing information remains `unavailable`"));
        assert!(markdown.contains("# Apartment 106"));
        assert!(markdown.contains("format: A4"));
        assert!(markdown.contains(&format!("sotSha256: {sot_sha256}")));
        assert!(html.contains("@page{size:A4"));
        assert!(html.contains("Apartment 106"));
        assert!(html.contains("SOT → MARKDOWN → HTML"));
        assert!(html.contains(&format!(
            "estate-studio-sot-sha256\" content=\"{sot_sha256}"
        )));
        assert!(html.contains(&format!(
            "estate-studio-markdown-sha256\" content=\"{markdown_sha256}"
        )));
    }

    #[test]
    fn a4_pdf_writer_creates_a_real_pdf() {
        let manifest = koya_manifest();
        let brief = &manifest.creative_jobs[0];
        let path = std::env::temp_dir().join(format!(
            "estate-studio-a4-pdf-test-{}.pdf",
            std::process::id()
        ));
        write_a4_pdf(&manifest, brief, &path).unwrap();
        let bytes = fs::read(&path).unwrap();
        assert!(bytes.starts_with(b"%PDF-"));
        assert!(bytes.len() > 2_000);
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn ai_poster_request_discloses_generation_boundary() {
        let manifest = legacy_koya_manifest_fixture();
        let brief = &manifest.creative_jobs[1];
        let request = image_generation_request(&manifest, brief);
        assert_eq!(request["status"], "prepared_not_submitted");
        assert_eq!(request["approvalState"], "not_approved");
        assert_eq!(request["priceStatus"], "unavailable");
        assert_eq!(request["outputCount"], 1);
        assert_eq!(
            request["overlayCopy"]["projectHighlights"][0],
            "2 bed · 2 bath"
        );
        assert_eq!(
            request["overlayCopy"]["projectHighlights"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
        assert!(request["boundary"]
            .as_str()
            .unwrap()
            .contains("no generated image"));
    }

    #[test]
    fn video_package_names_hyperframes_and_discloses_render_boundary() {
        let manifest = legacy_koya_manifest_fixture();
        let brief = &manifest.creative_jobs[2];
        let markdown = video_brief_markdown(&manifest, brief);
        assert!(markdown.contains("workflow: general-video"));
        assert!(markdown.contains("framework: HyperFrames"));
        assert!(markdown.contains("status: planned"));
        assert!(markdown.contains("No renderer"));
        assert_eq!(
            brief
                .scenes
                .iter()
                .map(|scene| scene.duration_seconds)
                .sum::<u16>(),
            15
        );
    }

    #[test]
    fn marketplace_skill_ids_and_frontmatter_are_unique() {
        let skills = bundled_skills();
        assert!(DEFAULT_PROJECT_SKILL_IDS
            .iter()
            .all(|default_id| skills.iter().any(|skill| skill.id == *default_id)));
        let mut ids = skills.iter().map(|skill| skill.id).collect::<Vec<_>>();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), skills.len());
        for skill in skills {
            assert!(valid_skill_id(skill.id));
            assert!(skill.content.starts_with("---\nname: "));
            assert!(skill.content.contains("\ndescription: "));
            assert!(skill.content.contains(&format!("name: {}", skill.id)));
        }
    }
}
