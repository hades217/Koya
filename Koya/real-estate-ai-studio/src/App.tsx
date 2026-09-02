import { FormEvent, useEffect, useLayoutEffect, useMemo, useRef, useState } from 'react';
import { open as openFileDialog, save as saveFileDialog } from '@tauri-apps/plugin-dialog';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import asterMark from './assets/aster-mark-v4-1.svg';
import { chatWithCodex, checkAiStatus, openCodexLogin } from './ai-store';
import type { AiHistoryMessage, AiStatus } from './ai-store';
import { addRoom, addUnit, applyGenerationCapability, approveGenerationJob, assignRoomIdentity, assignRoomPanorama, assignRoomStillFallback, assignRoomSupportingAsset, buildPanoramaDerivatives, buildStaticTourPreview, buildSuppliedPanoramaDerivatives, cancelGenerationJob, checkManagedGatewayCapability, checkSignedUpdate, confirmUpdateHealth, createLocalRelease, createPanoramaDraft, createPanoramaFallbackPlan, createProject, createRollbackRelease, duplicateProjectStructure, exportA4Pdf, exportProjectBundle, exportStaticRelease, finalizeVerifiedRelease, generateCreativeBrief, getAssetDataUrl, getCreativeA4Html, getDesktopSettings, getGenerationOutputDataUrl, getLicenseStatus, importProjectAssets, importProjectBundle, ingestGenerationOutput, inspectCodexGenerationAdapter, installMarketplaceSkill, installSignedLicense, installSignedUpdate, listCompanyProfiles, listGenerationCapabilities, listProjects, listSkillMarketplace, openProjectFolder, publishReleaseToCustomerDirectory, recordPanoramaQa, retryBackgroundJob, retryGenerationJob, reviewAsset, reviewGenerationOutput, runPanoramaBackgroundJob, saveCreativeBrief, saveRoomGraph, setActiveLocalRole, setProjectArchived, setProjectSkillEnabled, setWorkflowMode, submitManagedPanorama, updateDesktopSettings, updateProject, verifyDeploymentReadback } from './project-store';
import type { A4DocumentType, AddUnitInput, AiProjectUpdateDraft, AssetCategory, AssetRecord, AssetRejectionReason, CompanyProfile, CreateProjectInput, CreativeBrief, DesktopSettings, EvidenceClass, GenerateCreativeBriefInput, ImportAssetsInput, LicenseStatus, MarketplaceSkill, OpeningInput, ProjectRecord, RoomGraphNodeInput, RoomRecord, RoomSupportingRole, SignedUpdateStatus, UnitRecord, UpdateProjectInput } from './types';

type View = 'projects' | 'project' | 'unit' | 'production' | 'creative' | 'skills' | 'jobs' | 'deployments';

type NavGlyphName = 'projects' | 'production' | 'creative' | 'skills' | 'jobs' | 'deployments' | 'settings';

function NavGlyph({ name }: { name: NavGlyphName }) {
  const paths: Record<NavGlyphName, React.ReactNode> = {
    projects: <><path d="M4 7.5 12 3l8 4.5V20H4Z"/><path d="M9 20v-6h6v6"/></>,
    production: <><path d="m12 3 1.4 5.6L19 10l-5.6 1.4L12 17l-1.4-5.6L5 10l5.6-1.4Z"/><path d="m18.5 16 .6 2.4 2.4.6-2.4.6-.6 2.4-.6-2.4-2.4-.6 2.4-.6Z"/></>,
    creative: <><rect x="4" y="4" width="16" height="16" rx="3"/><path d="M8 15.5 11 12l2.5 2.5L16 11l2 2.5"/><circle cx="9" cy="9" r="1"/></>,
    skills: <><path d="M12 3v4M12 17v4M3 12h4M17 12h4"/><path d="m5.6 5.6 2.8 2.8m7.2 7.2 2.8 2.8m0-12.8-2.8 2.8m-7.2 7.2-2.8 2.8"/><circle cx="12" cy="12" r="3"/></>,
    jobs: <><circle cx="12" cy="12" r="8"/><path d="M12 8v4l3 2"/></>,
    deployments: <><path d="M5 18 18 5M10 5h8v8"/><path d="M5 8v11h11"/></>,
    settings: <><circle cx="12" cy="12" r="3"/><path d="M19 12a7 7 0 0 0-.1-1l2-1.5-2-3.4-2.4 1A7 7 0 0 0 15 6l-.3-2.6h-4L10.5 6A7 7 0 0 0 9 7.1l-2.4-1-2 3.4 2 1.5a7 7 0 0 0 0 2l-2 1.5 2 3.4 2.4-1a7 7 0 0 0 1.5 1.1l.3 2.6h4L15 18a7 7 0 0 0 1.5-1.1l2.4 1 2-3.4-2-1.5a7 7 0 0 0 .1-1Z"/></>,
  };
  return <svg className="nav-glyph" viewBox="0 0 24 24" aria-hidden="true">{paths[name]}</svg>;
}

const moduleMeta = {
  tour: { name: 'Interactive Tour', description: 'Floor plans, room imagery, panorama and mobile tour.', action: 'Open tour workspace', icon: 'projects' as NavGlyphName },
  creative: { name: 'Creative Studio', description: 'Posters, social assets and campaign variations.', action: 'Create campaign assets', icon: 'creative' as NavGlyphName },
  deployment: { name: 'Deployment', description: 'Build, publish and verify public releases.', action: 'Manage releases', icon: 'deployments' as NavGlyphName },
};

const categoryMeta: Record<AssetCategory, { label: string; icon: string; accept: string[] }> = {
  drawings: { label: 'Floor plans & drawings', icon: '⌗', accept: ['pdf', 'png', 'jpg', 'jpeg', 'webp'] },
  renders: { label: 'Official renders', icon: '▧', accept: ['png', 'jpg', 'jpeg', 'webp', 'tif', 'tiff'] },
  photos: { label: 'Photos & references', icon: '◫', accept: ['png', 'jpg', 'jpeg', 'webp', 'heic'] },
  panoramas: { label: '360° panoramas', icon: '◉', accept: ['png', 'jpg', 'jpeg', 'webp'] },
  videos: { label: 'Videos', icon: '▶', accept: ['mp4', 'mov', 'm4v', 'webm'] },
  brand: { label: 'Brand kit', icon: '◆', accept: ['pdf', 'png', 'jpg', 'jpeg', 'svg'] },
  copy: { label: 'Copy & schedules', icon: '≡', accept: ['pdf', 'doc', 'docx', 'txt', 'md'] },
};

const a4DocumentTypes: Array<{ id: A4DocumentType; label: string; short: string; description: string; requirement: string }> = [
  { id: 'project_sales_brochure', label: 'Project sales brochure', short: 'Main sales story', description: 'A multi-page project story covering verified highlights, location, team, imagery and enquiry action.', requirement: 'Project SOT + brand + approved imagery' },
  { id: 'floorplan_book', label: 'Floorplan book', short: 'Compare unit types', description: 'A project-wide set of approved marketing floorplans with verified unit labels, areas and plan notes.', requirement: 'Approved plans + unit schedule' },
  { id: 'unit_sales_sheet', label: 'Unit sales sheet', short: 'One unit at a glance', description: 'Floor plan, verified areas, room mix, key features and a clear enquiry action.', requirement: 'Unit + floor plan' },
  { id: 'price_availability', label: 'Price & availability', short: 'Sales team working list', description: 'A scannable unit schedule with current price and availability supplied by the sales team.', requirement: 'Approved price schedule' },
  { id: 'finishes_specifications', label: 'Finishes & specifications', short: 'What is included', description: 'A reviewed schedule of materials, appliances, fixtures, options and substitution boundaries.', requirement: 'Approved finishes schedule' },
  { id: 'agent_kit', label: 'Agent kit', short: 'Sales enablement pack', description: 'A consistent project presentation, evidence-safe talking points, FAQs and sales process for agents.', requirement: 'Project SOT + approved sales process' },
  { id: 'showroom_eoi_pack', label: 'Showroom & EOI pack', short: 'Visit to follow-up', description: 'A customer handout covering the showroom visit, enquiry path, EOI steps, privacy and supplied terms.', requirement: 'Approved EOI + legal wording' },
];

function a4DocumentMeta(value?: A4DocumentType) {
  return a4DocumentTypes.find((item) => item.id === value) ?? a4DocumentTypes[1];
}

function formatBytes(value: number) {
  if (value < 1024 * 1024) return `${Math.max(1, Math.round(value / 1024))} KB`;
  return `${(value / 1024 / 1024).toFixed(1)} MB`;
}

function formatDate(value: number) {
  return new Intl.DateTimeFormat('en-AU', { day: 'numeric', month: 'short', year: 'numeric' }).format(value * 1000);
}

function ProjectCard({ project, onOpen }: { project: ProjectRecord; onOpen: () => void }) {
  const { manifest } = project;
  return (
    <button className="project-card" onClick={onOpen}>
      <div className={`project-visual ${manifest.projectId === 'koya-example' ? 'koya-cover' : 'default-cover'}`} style={{ '--project-accent': manifest.brand.accent } as React.CSSProperties}>
        <div className="building-lines" aria-hidden="true">
          <span /><span /><span /><span /><span />
        </div>
        <div className="project-badge">{manifest.readOnly ? 'Example project' : manifest.archivedAt ? 'Archived' : manifest.status}</div>
        <div className="project-mark">{manifest.name.slice(0, 1).toUpperCase()}</div>
      </div>
      <div className="project-card-body">
        <div>
          <p className="eyebrow">{manifest.company}</p>
          <h3>{manifest.name}</h3>
          <p className="project-location">{manifest.location || 'Location not supplied'}</p>
        </div>
        <div className="project-card-footer">
          <span>{manifest.units.length} unit types</span>
          <span>{manifest.readiness}% ready</span>
        </div>
      </div>
    </button>
  );
}

function NewProjectDialog({ onClose, onCreated }: { onClose: () => void; onCreated: (project: ProjectRecord) => void }) {
  const [form, setForm] = useState({ name: '', company: '', location: '', units: '' });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  async function submit(event: FormEvent) {
    event.preventDefault();
    setError('');
    if (!form.name.trim() || !form.company.trim()) {
      setError('Development name and customer company are required.');
      return;
    }
    const input: CreateProjectInput = {
      name: form.name,
      company: form.company,
      location: form.location,
      unitIds: form.units.split(',').map((unit) => unit.trim()).filter(Boolean),
    };
    setSaving(true);
    try {
      onCreated(await createProject(input));
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}>
      <form className="new-project-dialog" onSubmit={submit}>
        <div className="dialog-heading">
          <div>
            <p className="eyebrow">New development</p>
            <h2>Create a clean project workspace</h2>
          </div>
          <button type="button" className="icon-button" onClick={onClose} aria-label="Close">×</button>
        </div>
        <p className="dialog-copy">Nothing from Koya will be copied. This development receives its own project ID, assets, units and release history.</p>
        <label>
          Development name
          <input autoFocus value={form.name} onChange={(event) => setForm({ ...form, name: event.target.value })} placeholder="e.g. Northbank Residences" />
        </label>
        <div className="form-row">
          <label>
            Customer company
            <input value={form.company} onChange={(event) => setForm({ ...form, company: event.target.value })} placeholder="Developer or agency" />
          </label>
          <label>
            Location
            <input value={form.location} onChange={(event) => setForm({ ...form, location: event.target.value })} placeholder="Suburb, state" />
          </label>
        </div>
        <label>
          Unit types <span className="optional">optional</span>
          <input value={form.units} onChange={(event) => setForm({ ...form, units: event.target.value })} placeholder="101, 102, 201" />
          <small>Separate unit numbers with commas. Floor plans can be added next.</small>
        </label>
        {error && <div className="form-error">{error}</div>}
        <div className="dialog-actions">
          <button type="button" className="secondary-button" onClick={onClose}>Cancel</button>
          <button type="submit" className="primary-button" disabled={saving}>{saving ? 'Creating…' : 'Create project'}</button>
        </div>
      </form>
    </div>
  );
}

function DuplicateProjectDialog({ project, onClose, onCreated }: { project: ProjectRecord; onClose: () => void; onCreated: (project: ProjectRecord) => void }) {
  const [name, setName] = useState('');
  const [company, setCompany] = useState('');
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  async function submit(event: FormEvent) {
    event.preventDefault();
    if (!name.trim() || !company.trim()) return setError('New development name and customer company are required.');
    setSaving(true); setError('');
    try { onCreated(await duplicateProjectStructure(project.manifest.projectId, { name, company })); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog" onSubmit={submit}>
    <div className="dialog-heading"><div><p className="eyebrow">Safe template</p><h2>Duplicate clean structure</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">Creates {project.manifest.units.length} empty unit workspace{project.manifest.units.length === 1 ? '' : 's'}. No customer assets, facts, branding, disclosure, room graph, generation, QA or release record is copied.</p>
    <label>New development name<input autoFocus value={name} onChange={(event) => setName(event.target.value)} /></label>
    <label>New customer company<input value={company} onChange={(event) => setCompany(event.target.value)} /></label>
    {error && <div className="form-error">{error}</div>}
    <div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving}>{saving ? 'Creating…' : 'Create clean project'}</button></div>
  </form></div>;
}

function AddUnitDialog({ project, onClose, onUpdated }: { project: ProjectRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [form, setForm] = useState<AddUnitInput>({ unitId: '', label: '', summary: '' });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (!form.unitId.trim()) return setError('Unit number or type is required.');
    setSaving(true);
    setError('');
    try {
      onUpdated(await addUnit(project.manifest.projectId, form));
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}>
      <form className="new-project-dialog compact-dialog" onSubmit={submit}>
        <div className="dialog-heading"><div><p className="eyebrow">{project.manifest.name}</p><h2>Add a unit type</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
        <p className="dialog-copy">Create the unit workspace first. Its floor plan and evidence remain isolated from every other unit.</p>
        <div className="form-row">
          <label>Unit number or type<input autoFocus value={form.unitId} onChange={(event) => setForm({ ...form, unitId: event.target.value })} placeholder="e.g. 201 or Type B" /></label>
          <label>Display name <span className="optional">optional</span><input value={form.label} onChange={(event) => setForm({ ...form, label: event.target.value })} placeholder="Apartment 201" /></label>
        </div>
        <label>Approved summary <span className="optional">optional</span><input value={form.summary} onChange={(event) => setForm({ ...form, summary: event.target.value })} placeholder="2 bed · 2 bath · 104 m²" /></label>
        {error && <div className="form-error">{error}</div>}
        <div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving}>{saving ? 'Adding…' : 'Add unit'}</button></div>
      </form>
    </div>
  );
}

function ImportSourceDialog({ project, onClose, onUpdated }: { project: ProjectRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [category, setCategory] = useState<AssetCategory>('drawings');
  const [evidenceClass, setEvidenceClass] = useState<EvidenceClass>('official');
  const [unitId, setUnitId] = useState('');
  const [sourceOwner, setSourceOwner] = useState(project.manifest.company);
  const [usagePermission, setUsagePermission] = useState('Approved for this project');
  const [paths, setPaths] = useState<string[]>([]);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');

  async function chooseFiles() {
    setError('');
    try {
      const selected = await openFileDialog({ multiple: true, directory: false, filters: [{ name: categoryMeta[category].label, extensions: categoryMeta[category].accept }] });
      if (selected) setPaths(Array.isArray(selected) ? selected : [selected]);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function submit(event: FormEvent) {
    event.preventDefault();
    if (paths.length === 0) return setError('Choose at least one source file.');
    if (!sourceOwner.trim()) return setError('Record the source owner.');
    if (!usagePermission.trim()) return setError('Record the usage permission.');
    const input: ImportAssetsInput = { paths, category, evidenceClass, unitId: unitId || undefined, sourceOwner, usagePermission };
    setSaving(true);
    setError('');
    try {
      onUpdated(await importProjectAssets(project.manifest.projectId, input));
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}>
      <form className="new-project-dialog import-dialog" onSubmit={submit}>
        <div className="dialog-heading"><div><p className="eyebrow">Evidence intake</p><h2>Import project sources</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
        <p className="dialog-copy">Files are copied into {project.manifest.name}. Classification remains reviewable and never makes an AI image official.</p>
        <div className="category-picker">
          {(Object.keys(categoryMeta) as AssetCategory[]).map((key) => <button type="button" className={category === key ? 'selected' : ''} key={key} onClick={() => { setCategory(key); setPaths([]); }}><span>{categoryMeta[key].icon}</span>{categoryMeta[key].label}</button>)}
        </div>
        <div className="form-row">
          <label>Evidence class<select value={evidenceClass} onChange={(event) => setEvidenceClass(event.target.value as EvidenceClass)}><option value="official">Official supplied file</option><option value="approved_render">Approved render</option><option value="concept_floorplan_grounded">Floor-plan-grounded concept</option><option value="concept_style_only">Style-only concept</option><option value="unknown">Unknown · needs review</option></select></label>
          <label>Unit type <span className="optional">optional</span><select value={unitId} onChange={(event) => setUnitId(event.target.value)}><option value="">Project-wide source</option>{project.manifest.units.map((unit) => <option value={unit.id} key={unit.id}>{unit.label}</option>)}</select></label>
        </div>
        <div className="form-row">
          <label>Source owner<input value={sourceOwner} onChange={(event) => setSourceOwner(event.target.value)} placeholder="Customer, architect or photographer" /></label>
          <label>Usage permission<input value={usagePermission} onChange={(event) => setUsagePermission(event.target.value)} placeholder="Approved project use, licensed campaign use…" /></label>
        </div>
        <button type="button" className={`file-drop ${paths.length ? 'has-files' : ''}`} onClick={chooseFiles}><span>{paths.length ? '✓' : '＋'}</span><strong>{paths.length ? `${paths.length} file${paths.length === 1 ? '' : 's'} selected` : 'Choose source files'}</strong><small>{paths.length ? paths.map((path) => path.split('/').pop()).join(' · ') : categoryMeta[category].accept.map((ext) => ext.toUpperCase()).join(', ')}</small></button>
        {error && <div className="form-error">{error}</div>}
        <div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving || paths.length === 0}>{saving ? 'Importing…' : `Import ${paths.length || ''} file${paths.length === 1 ? '' : 's'}`}</button></div>
      </form>
    </div>
  );
}

function EditProjectDialog({ project, aiDraft, onClose, onUpdated }: { project: ProjectRecord; aiDraft?: AiProjectUpdateDraft; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [form, setForm] = useState<UpdateProjectInput>({ name: aiDraft?.name ?? project.manifest.name, company: aiDraft?.company ?? project.manifest.company, location: aiDraft?.location ?? project.manifest.location, primary: aiDraft?.primary ?? project.manifest.brand.primary, accent: aiDraft?.accent ?? project.manifest.brand.accent, expectedUpdatedAt: aiDraft?.baseUpdatedAt ?? project.manifest.updatedAt });
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  async function submit(event: FormEvent) {
    event.preventDefault(); setSaving(true); setError('');
    try { onUpdated(await updateProject(project.manifest.projectId, form)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog" onSubmit={submit}>
    <div className="dialog-heading"><div><p className="eyebrow">Project profile</p><h2>Edit development</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">These details become shared project context for every production module.</p>
    {aiDraft && <div className="ai-form-draft"><span>✦</span><div><strong>AI draft applied to this form</strong><p>{aiDraft.summary} Review every field, then choose Save project. Nothing has been written yet.</p></div></div>}
    <label>Development name<input value={form.name} onChange={(event) => setForm({ ...form, name: event.target.value })} /></label>
    <div className="form-row"><label>Customer company<input value={form.company} onChange={(event) => setForm({ ...form, company: event.target.value })} /></label><label>Location<input value={form.location} onChange={(event) => setForm({ ...form, location: event.target.value })} /></label></div>
    <div className="form-row colour-row"><label>Primary colour<input type="color" value={form.primary} onChange={(event) => setForm({ ...form, primary: event.target.value })} /></label><label>Accent colour<input type="color" value={form.accent} onChange={(event) => setForm({ ...form, accent: event.target.value })} /></label></div>
    {error && <div className="form-error">{error}</div>}<div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving}>{saving ? 'Saving…' : 'Save project'}</button></div>
  </form></div>;
}

function AddRoomDialog({ project, unit, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [name, setName] = useState(''); const [saving, setSaving] = useState(false); const [error, setError] = useState('');
  async function submit(event: FormEvent) { event.preventDefault(); if (!name.trim()) return setError('Room name is required.'); setSaving(true); try { onUpdated(await addRoom(project.manifest.projectId, unit.id, name)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog" onSubmit={submit}>
    <div className="dialog-heading"><div><p className="eyebrow">{unit.label}</p><h2>Add a room or tour stop</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">This creates a draft stop only. Its position and connections must still be confirmed against the floor plan.</p>
    <label>Room name<input autoFocus value={name} onChange={(event) => setName(event.target.value)} placeholder="e.g. Living room" /></label>
    {error && <div className="form-error">{error}</div>}<div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving}>{saving ? 'Adding…' : 'Add room'}</button></div>
  </form></div>;
}

function IdentityAssetDialog({ project, unit, room, assets, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; room: RoomRecord; assets: AssetRecord[]; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [assetId, setAssetId] = useState(assets[0]?.id ?? ''); const [saving, setSaving] = useState(false); const [error, setError] = useState('');
  async function submit(event: FormEvent) { event.preventDefault(); if (!assetId) return; setSaving(true); try { onUpdated(await assignRoomIdentity(project.manifest.projectId, unit.id, room.id, assetId)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog" onSubmit={submit}>
    <div className="dialog-heading"><div><p className="eyebrow">{unit.label} · {room.name}</p><h2>Choose identity anchor</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">The selected accepted render or photo becomes the locked visual identity for this room. Its evidence class remains unchanged.</p>
    <div className="identity-options">{assets.map((asset) => <label className={assetId === asset.id ? 'selected' : ''} key={asset.id}><input type="radio" name="identity" value={asset.id} checked={assetId === asset.id} onChange={() => setAssetId(asset.id)} /><span>{categoryMeta[asset.category].icon}</span><div><strong>{asset.name}</strong><small>{asset.evidenceClass.replaceAll('_', ' ')} · {asset.unitId ? `Unit ${asset.unitId}` : 'Project-wide'}</small></div></label>)}</div>
    {error && <div className="form-error">{error}</div>}<div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving || !assetId}>{saving ? 'Assigning…' : 'Confirm identity anchor'}</button></div>
  </form></div>;
}

function PanoramaAssetDialog({ project, unit, room, assets, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; room: RoomRecord; assets: AssetRecord[]; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [assetId, setAssetId] = useState(assets[0]?.id ?? ''); const [saving, setSaving] = useState(false); const [error, setError] = useState('');
  async function submit(event: FormEvent) { event.preventDefault(); if (!assetId) return; setSaving(true); try { onUpdated(await assignRoomPanorama(project.manifest.projectId, unit.id, room.id, assetId)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog" onSubmit={submit}>
    <div className="dialog-heading"><div><p className="eyebrow">PAN-03 · Supplied source</p><h2>Assign 360° panorama</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">Only an accepted, exact 2:1 panorama at least 4096 pixels wide can become a ready tour stop. The source evidence class remains unchanged.</p>
    <div className="identity-options">{assets.map((asset) => <label className={assetId === asset.id ? 'selected' : ''} key={asset.id}><input type="radio" name="panorama" value={asset.id} checked={assetId === asset.id} onChange={() => setAssetId(asset.id)} /><span>◉</span><div><strong>{asset.name}</strong><small>{asset.width && asset.height ? `${asset.width} × ${asset.height}` : 'Dimensions unavailable'} · {asset.evidenceClass.replaceAll('_', ' ')}</small></div></label>)}</div>
    {error && <div className="form-error">{error}</div>}<div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving || !assetId}>{saving ? 'Validating…' : 'Validate and assign'}</button></div>
  </form></div>;
}

const roomSupportingRoleMeta: Record<RoomSupportingRole, { label: string; description: string }> = {
  threshold: { label: 'Threshold view', description: 'A still confirming the transition at an opening.' },
  reverse: { label: 'Reverse view', description: 'An optional still looking back through the route.' },
  still_fallback: { label: 'Still fallback', description: 'Shown when panorama or WebGL playback is unavailable.' },
  video: { label: 'Representative video', description: 'A browser-compatible MP4 shown with this tour stop.' },
  poster: { label: 'Video poster', description: 'A still shown before representative video playback.' },
};

function RoomMediaDialog({ project, unit, room, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; room: RoomRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const accepted = project.manifest.assets.filter((asset) => asset.status === 'accepted' && asset.evidenceClass !== 'unknown' && Boolean(asset.usagePermission.trim()) && (!asset.unitId || asset.unitId === unit.id));
  const panoramaAssets = accepted.filter((asset) => asset.category === 'panoramas' && ['image/png', 'image/jpeg', 'image/webp'].includes(asset.mimeType));
  const availableRoles = (Object.keys(roomSupportingRoleMeta) as RoomSupportingRole[]).filter((role) => accepted.some((asset) => role === 'video' ? asset.category === 'videos' && asset.mimeType === 'video/mp4' : ['image/png', 'image/jpeg', 'image/webp'].includes(asset.mimeType)));
  const [role, setRole] = useState<RoomSupportingRole>(availableRoles.includes('video') ? 'video' : availableRoles[0] ?? 'still_fallback');
  const eligibleAssets = accepted.filter((asset) => role === 'video' ? asset.category === 'videos' && asset.mimeType === 'video/mp4' : ['image/png', 'image/jpeg', 'image/webp'].includes(asset.mimeType));
  const [assetId, setAssetId] = useState(eligibleAssets[0]?.id ?? '');
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [panoramaId, setPanoramaId] = useState(room.panoramaAssetId ?? panoramaAssets[0]?.id ?? '');
  const panorama = panoramaAssets.find((asset) => asset.id === panoramaId);
  const deliveryKinds = panorama?.derivativeRelativePaths.map((path) => path.split('/').pop()?.split('-')[0] ?? '').filter(Boolean) ?? [];
  useEffect(() => { setAssetId(eligibleAssets[0]?.id ?? ''); }, [role]);
  async function assign(event: FormEvent) {
    event.preventDefault();
    if (!assetId) return setError('Choose an accepted source for this role.');
    setSaving(true); setError('');
    try { onUpdated(await assignRoomSupportingAsset(project.manifest.projectId, unit.id, room.id, role, assetId)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }
  async function buildDeliverySet() {
    if (!panorama) return;
    setSaving(true); setError('');
    try { onUpdated(await buildSuppliedPanoramaDerivatives(project.manifest.projectId, panorama.id)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }
  const currentRoleAssetId: Partial<Record<RoomSupportingRole, string | undefined>> = { threshold: room.thresholdAssetId, reverse: room.reverseAssetId, still_fallback: room.stillFallbackAssetId, video: room.videoAssetId, poster: room.posterAssetId };
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><form className="new-project-dialog compact-dialog room-media-dialog" onSubmit={assign}>
    <div className="dialog-heading"><div><p className="eyebrow">{unit.label} · {room.name}</p><h2>Room media and delivery</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">Assign accepted supporting media without changing its evidence class. A supplied panorama can also receive deterministic mobile, 4K and 8K derivatives.</p>
    <div className="delivery-set-card"><div><strong>Panorama delivery set</strong><select aria-label="Accepted panorama source" value={panoramaId} onChange={(event) => setPanoramaId(event.target.value)}><option value="">Choose an accepted panorama</option>{panoramaAssets.map((asset) => <option key={asset.id} value={asset.id}>{asset.name} · {asset.width} × {asset.height}{asset.id === room.panoramaAssetId ? ' · assigned to room' : ''}</option>)}</select><small>{deliveryKinds.length ? `${deliveryKinds.join(', ')} verified` : 'Not built · 8K resampling never claims recovered detail'}</small></div><button type="button" className="secondary-button" disabled={saving || !panorama || deliveryKinds.length > 0} onClick={buildDeliverySet}>{deliveryKinds.length ? 'Delivery set ready' : saving ? 'Processing in background…' : 'Build mobile + 4K + 8K'}</button></div>
    <label>Supporting role<select value={role} onChange={(event) => setRole(event.target.value as RoomSupportingRole)}>{availableRoles.map((value) => <option key={value} value={value}>{roomSupportingRoleMeta[value].label}{currentRoleAssetId[value] ? ' · assigned' : ''}</option>)}</select><small>{roomSupportingRoleMeta[role].description}</small></label>
    <label>Accepted source<select value={assetId} onChange={(event) => setAssetId(event.target.value)}><option value="">Choose a source</option>{eligibleAssets.map((asset) => <option key={asset.id} value={asset.id}>{asset.name} · {asset.evidenceClass.replaceAll('_', ' ')}</option>)}</select></label>
    {error && <div className="form-error">{error}</div>}
    <div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving || !assetId}>{saving ? 'Saving…' : `Assign ${roomSupportingRoleMeta[role].label.toLowerCase()}`}</button></div>
  </form></div>;
}

function PanoramaDraftDialog({ project, unit, room, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; room: RoomRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const [saving, setSaving] = useState(false); const [error, setError] = useState('');
  const floorplanAsset = project.manifest.assets.find((asset) => asset.id === unit.floorplanAssetId);
  const identityAsset = project.manifest.assets.find((asset) => asset.id === room.identityAssetId);
  const roomOpenings = (unit.openings ?? []).filter((opening) => opening.fromRoomId === room.id || opening.toRoomId === room.id);
  const topologyFingerprint = unit.roomGraphVersions?.at(-1)?.fingerprint ?? 'unavailable';
  async function saveDraft() { setSaving(true); try { onUpdated(await createPanoramaDraft(project.manifest.projectId, unit.id, room.id)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  return <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}><div className="new-project-dialog approval-dialog">
    <div className="dialog-heading"><div><p className="eyebrow">Generation approval package</p><h2>{room.name} panorama</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
    <p className="dialog-copy">Saving this package does not submit or charge an AI task. Provider capability and current price must be available before approval.</p>
    <div className="approval-summary"><div><span>Project / unit</span><strong>{project.manifest.name} · {unit.label}</strong></div><div><span>Asset role</span><strong>Interactive 360° panorama · 1 output</strong></div><div><span>Topology source</span><strong>{floorplanAsset ? `${floorplanAsset.name} · ${floorplanAsset.checksumSha256.slice(0, 10)}…` : 'Unavailable'}</strong></div><div><span>Identity anchor</span><strong>{identityAsset ? `${identityAsset.name} · ${identityAsset.checksumSha256.slice(0, 10)}…` : 'Unavailable'}</strong></div><div><span>Camera intent</span><strong>Stationary eye-level at confirmed hotspot</strong></div><div><span>Required openings</span><strong>{roomOpenings.length ? roomOpenings.map((opening) => opening.id).join(', ') : 'Unavailable'}</strong></div><div><span>Fixed fixtures</span><strong>Unavailable · not supplied</strong></div><div><span>Topology fingerprint</span><strong>{topologyFingerprint === 'unavailable' ? topologyFingerprint : `${topologyFingerprint.slice(0, 12)}…`}</strong></div><div><span>Provider output / mode</span><strong>3840 × 1920 · one-shot 2:1</strong><small>Accepted source is processed locally into 4096 × 2048 and 8192 × 4096 delivery derivatives.</small></div><div className="blocked"><span>Model capability / provider</span><strong>Unavailable · unconfigured</strong></div><div className="blocked"><span>Current price</span><strong>Unavailable</strong></div></div>
    <div className="package-prohibitions"><strong>Locked prohibitions</strong><span>No topology, opening, fixture, dimension, view, finish, amenity, person, text, logo, or watermark invention. Style evidence cannot replace topology evidence.</span></div>
    <div className="approval-warning"><strong>Submission blocked</strong><span>No API task will be sent until capability, exact provider settings and price are known and explicitly approved.</span></div>
    {error && <div className="form-error">{error}</div>}<div className="dialog-actions"><button type="button" className="secondary-button" onClick={onClose}>Cancel</button><button className="primary-button" disabled={saving} onClick={saveDraft}>{saving ? 'Saving…' : 'Save approval draft'}</button></div>
  </div></div>;
}

function RoomGraphDialog({ project, unit, onClose, onUpdated }: { project: ProjectRecord; unit: UnitRecord; onClose: () => void; onUpdated: (project: ProjectRecord) => void }) {
  const initialNodes = unit.rooms.map((room, index): RoomGraphNodeInput => ({ roomId: room.id, hotspotX: room.hotspotX ?? 18 + (index % 3) * 30, hotspotY: room.hotspotY ?? 25 + Math.floor(index / 3) * 38, adjacentRoomIds: room.adjacentRoomIds ?? [] }));
  const [nodes, setNodes] = useState(initialNodes);
  const [selectedId, setSelectedId] = useState(unit.rooms[0]?.id ?? '');
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [floorplanPreview, setFloorplanPreview] = useState('');
  const [planView, setPlanView] = useState({ zoom: 1, panX: 0, panY: 0, rotation: 0, crop: 0 });
  const [entranceRoomId, setEntranceRoomId] = useState(unit.entranceRoomId ?? '');
  const [openings, setOpenings] = useState<OpeningInput[]>((unit.openings ?? []).map(({ status: _status, ...opening }) => opening));
  const [suggestion, setSuggestion] = useState<{ nodes: RoomGraphNodeInput[]; entranceRoomId: string; openings: OpeningInput[]; edgeCount: number } | null>(null);
  const [suggestionApplied, setSuggestionApplied] = useState(false);
  const selected = nodes.find((node) => node.roomId === selectedId);
  useEffect(() => {
    if (!unit.floorplanAssetId || project.manifest.readOnly) return;
    getAssetDataUrl(project.manifest.projectId, unit.floorplanAssetId).then(setFloorplanPreview).catch(() => setFloorplanPreview(''));
  }, [project.manifest.projectId, project.manifest.readOnly, unit.floorplanAssetId]);
  function positionSelected(event: React.MouseEvent<HTMLDivElement>) {
    if (project.manifest.readOnly || !selectedId) return;
    const box = event.currentTarget.getBoundingClientRect();
    const worldWidth = box.width * .88;
    const worldHeight = box.height * .88;
    const translatedX = event.clientX - box.left - box.width / 2 - planView.panX;
    const translatedY = event.clientY - box.top - box.height / 2 - planView.panY;
    const radians = planView.rotation * Math.PI / 180;
    const unrotatedX = (translatedX * Math.cos(radians) + translatedY * Math.sin(radians)) / planView.zoom;
    const unrotatedY = (-translatedX * Math.sin(radians) + translatedY * Math.cos(radians)) / planView.zoom;
    const hotspotX = Math.max(3, Math.min(97, ((unrotatedX + worldWidth / 2) / worldWidth) * 100));
    const hotspotY = Math.max(3, Math.min(97, ((unrotatedY + worldHeight / 2) / worldHeight) * 100));
    setNodes((current) => current.map((node) => node.roomId === selectedId ? { ...node, hotspotX, hotspotY } : node));
  }
  function adjustView(change: Partial<typeof planView>) {
    setPlanView((current) => ({
      ...current,
      ...change,
      zoom: Math.max(.5, Math.min(3, change.zoom ?? current.zoom)),
      panX: Math.max(-240, Math.min(240, change.panX ?? current.panX)),
      panY: Math.max(-240, Math.min(240, change.panY ?? current.panY)),
      crop: Math.max(0, Math.min(25, change.crop ?? current.crop)),
    }));
  }
  function moveSelected(deltaX: number, deltaY: number) {
    if (project.manifest.readOnly || !selectedId) return;
    setNodes((current) => current.map((node) => node.roomId === selectedId ? {
      ...node,
      hotspotX: Math.max(3, Math.min(97, node.hotspotX + deltaX)),
      hotspotY: Math.max(3, Math.min(97, node.hotspotY + deltaY)),
    } : node));
  }
  function handleCanvasKeyDown(event: React.KeyboardEvent<HTMLDivElement>) {
    if (event.target !== event.currentTarget) return;
    const step = event.shiftKey ? 24 : 10;
    const hotspotStep = event.shiftKey ? 5 : 1;
    if (event.altKey && ['ArrowLeft', 'ArrowRight', 'ArrowUp', 'ArrowDown'].includes(event.key)) {
      event.preventDefault();
      moveSelected(event.key === 'ArrowLeft' ? -hotspotStep : event.key === 'ArrowRight' ? hotspotStep : 0, event.key === 'ArrowUp' ? -hotspotStep : event.key === 'ArrowDown' ? hotspotStep : 0);
      return;
    }
    const viewActions: Record<string, () => void> = {
      ArrowLeft: () => adjustView({ panX: planView.panX - step }),
      ArrowRight: () => adjustView({ panX: planView.panX + step }),
      ArrowUp: () => adjustView({ panY: planView.panY - step }),
      ArrowDown: () => adjustView({ panY: planView.panY + step }),
      '+': () => adjustView({ zoom: planView.zoom + .1 }),
      '=': () => adjustView({ zoom: planView.zoom + .1 }),
      '-': () => adjustView({ zoom: planView.zoom - .1 }),
      r: () => adjustView({ rotation: (planView.rotation + 90) % 360 }),
      R: () => adjustView({ rotation: (planView.rotation + 90) % 360 }),
      '0': () => setPlanView({ zoom: 1, panX: 0, panY: 0, rotation: 0, crop: 0 }),
    };
    const action = viewActions[event.key];
    if (action) { event.preventDefault(); action(); }
  }
  function toggleConnection(otherId: string) {
    if (!selected || project.manifest.readOnly) return;
    const connected = selected.adjacentRoomIds.includes(otherId);
    setNodes((current) => current.map((node) => {
      if (node.roomId === selectedId) return { ...node, adjacentRoomIds: connected ? node.adjacentRoomIds.filter((id) => id !== otherId) : [...node.adjacentRoomIds, otherId] };
      if (node.roomId === otherId) return { ...node, adjacentRoomIds: connected ? node.adjacentRoomIds.filter((id) => id !== selectedId) : [...node.adjacentRoomIds, selectedId] };
      return node;
    }));
  }
  function suggestSpatialDraft() {
    if (!nodes.length) return;
    const suggestedNodes = nodes.map((node) => ({ ...node, adjacentRoomIds: [] as string[] }));
    const sorted = suggestedNodes.slice().sort((a, b) => a.hotspotY - b.hotspotY || a.hotspotX - b.hotspotX || a.roomId.localeCompare(b.roomId));
    const connected = [sorted[0]];
    const remaining = sorted.slice(1);
    const edges: Array<[RoomGraphNodeInput, RoomGraphNodeInput]> = [];
    while (remaining.length) {
      let best = { connectedIndex: 0, remainingIndex: 0, distance: Number.POSITIVE_INFINITY };
      connected.forEach((from, connectedIndex) => remaining.forEach((to, remainingIndex) => {
        const distance = Math.hypot(from.hotspotX - to.hotspotX, from.hotspotY - to.hotspotY);
        if (distance < best.distance) best = { connectedIndex, remainingIndex, distance };
      }));
      const from = connected[best.connectedIndex];
      const [to] = remaining.splice(best.remainingIndex, 1);
      from.adjacentRoomIds.push(to.roomId);
      to.adjacentRoomIds.push(from.roomId);
      edges.push([from, to]);
      connected.push(to);
    }
    const entrance = sorted[0];
    const suggestedOpenings: OpeningInput[] = [{ id: 'suggested-entrance-1', fromRoomId: entrance.roomId, kind: 'entrance', x: entrance.hotspotX, y: entrance.hotspotY }];
    edges.forEach(([from, to], index) => suggestedOpenings.push({ id: `suggested-opening-${index + 1}`, fromRoomId: from.roomId, toRoomId: to.roomId, kind: 'opening', x: (from.hotspotX + to.hotspotX) / 2, y: (from.hotspotY + to.hotspotY) / 2 }));
    setSuggestion({ nodes: suggestedNodes, entranceRoomId: entrance.roomId, openings: suggestedOpenings, edgeCount: edges.length });
    setSuggestionApplied(false);
  }
  function applySpatialSuggestion() {
    if (!suggestion || project.manifest.readOnly) return;
    setNodes(suggestion.nodes);
    setEntranceRoomId(suggestion.entranceRoomId);
    setOpenings(suggestion.openings);
    setSuggestion(null);
    setSuggestionApplied(true);
  }
  function setEntrance(roomId: string) {
    setEntranceRoomId(roomId);
    setOpenings((current) => {
      const withoutEntrance = current.filter((opening) => opening.kind !== 'entrance');
      const node = nodes.find((candidate) => candidate.roomId === roomId);
      return roomId && node ? [{ id: 'entrance-1', fromRoomId: roomId, kind: 'entrance', x: node.hotspotX, y: node.hotspotY }, ...withoutEntrance] : withoutEntrance;
    });
  }
  function addOpening() {
    const from = nodes.find((node) => node.roomId === selectedId) ?? nodes[0];
    const to = nodes.find((node) => node.roomId !== from?.roomId);
    if (!from || !to) return;
    let suffix = openings.length + 1;
    while (openings.some((opening) => opening.id === `opening-${suffix}`)) suffix += 1;
    setOpenings((current) => [...current, { id: `opening-${suffix}`, fromRoomId: from.roomId, toRoomId: to.roomId, kind: 'opening', x: (from.hotspotX + to.hotspotX) / 2, y: (from.hotspotY + to.hotspotY) / 2 }]);
  }
  function updateOpening(id: string, change: Partial<OpeningInput>) {
    setOpenings((current) => current.map((opening) => opening.id === id ? { ...opening, ...change } : opening));
  }
  async function save(locked: boolean) {
    setSaving(true); setError('');
    try { onUpdated(await saveRoomGraph(project.manifest.projectId, unit.id, nodes, entranceRoomId || undefined, openings, locked)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }
  return <div className="dialog-backdrop graph-backdrop"><section className="room-graph-dialog">
    <header className="graph-toolbar"><div><p className="eyebrow">PLN-03 · Navigation truth</p><h2>{unit.label} room graph</h2><span>{unit.floorplanAssetId ? `Floor plan v${unit.floorplanVersion ?? 1}` : 'Floor plan required'} · Graph v{unit.roomGraphVersion ?? 0} · {unit.roomGraphLocked ? 'Locked' : 'Draft'}</span></div><button className="icon-button" onClick={onClose}>×</button></header>
    <div className="graph-layout"><div className={`graph-canvas ${floorplanPreview ? 'has-floorplan' : ''}`} onClick={positionSelected} onKeyDown={handleCanvasKeyDown} tabIndex={0} role="group" aria-label="Interactive floor plan. Arrow keys pan, plus and minus zoom, R rotates, zero resets, and Alt plus arrow keys move the selected hotspot.">
      <div className="plan-controls" onClick={(event) => event.stopPropagation()}>
        <div className="plan-control-group" aria-label="Zoom controls"><button type="button" aria-label="Zoom out" onClick={() => adjustView({ zoom: planView.zoom - .1 })}>−</button><output aria-live="polite">{Math.round(planView.zoom * 100)}%</output><button type="button" aria-label="Zoom in" onClick={() => adjustView({ zoom: planView.zoom + .1 })}>+</button></div>
        <div className="plan-control-group pan-controls" aria-label="Pan controls"><button type="button" aria-label="Pan left" onClick={() => adjustView({ panX: planView.panX - 20 })}>←</button><button type="button" aria-label="Pan up" onClick={() => adjustView({ panY: planView.panY - 20 })}>↑</button><button type="button" aria-label="Pan down" onClick={() => adjustView({ panY: planView.panY + 20 })}>↓</button><button type="button" aria-label="Pan right" onClick={() => adjustView({ panX: planView.panX + 20 })}>→</button></div>
        <button type="button" className="plan-control-text" aria-label="Rotate floor plan clockwise 90 degrees" onClick={() => adjustView({ rotation: (planView.rotation + 90) % 360 })}>Rotate {planView.rotation}°</button>
        <label className="crop-control">Crop <input type="range" min="0" max="25" value={planView.crop} onChange={(event) => adjustView({ crop: Number(event.target.value) })} /><output>{planView.crop}%</output></label>
        <button type="button" className="plan-control-text" onClick={() => setPlanView({ zoom: 1, panX: 0, panY: 0, rotation: 0, crop: 0 })}>Reset</button>
      </div>
      <div className="graph-world" style={{ transform: `translate(${planView.panX}px, ${planView.panY}px) rotate(${planView.rotation}deg) scale(${planView.zoom})` }}>
        {floorplanPreview ? <img className="floorplan-preview" style={{ clipPath: `inset(${planView.crop}%)` }} src={floorplanPreview} alt={`${unit.label} floor plan`} /> : <div className="plan-outline" style={{ clipPath: `inset(${planView.crop}%)` }}><span>ENTRY</span><i /><i /><i /><i /></div>}
        <svg className="graph-lines" viewBox="0 0 100 100" preserveAspectRatio="none">{nodes.flatMap((node) => node.adjacentRoomIds.filter((id) => node.roomId < id).map((id) => { const other = nodes.find((item) => item.roomId === id); return other ? <line key={`${node.roomId}-${id}`} x1={node.hotspotX} y1={node.hotspotY} x2={other.hotspotX} y2={other.hotspotY} /> : null; }))}</svg>
        {openings.map((opening) => <span key={opening.id} className={`opening-marker ${opening.kind}`} style={{ left: `${opening.x}%`, top: `${opening.y}%` }} title={`${opening.kind}: ${opening.fromRoomId}${opening.toRoomId ? ` to ${opening.toRoomId}` : ''}`} aria-hidden="true">{opening.kind === 'entrance' ? 'E' : '◇'}</span>)}
        {nodes.map((node, index) => <button key={node.roomId} className={`graph-hotspot ${selectedId === node.roomId ? 'selected' : ''}`} style={{ left: `${node.hotspotX}%`, top: `${node.hotspotY}%` }} onClick={(event) => { event.stopPropagation(); setSelectedId(node.roomId); }}><span>{index + 1}</span><strong>{unit.rooms.find((room) => room.id === node.roomId)?.name}</strong></button>)}
      </div>
      <div className="graph-help">Click to place · Arrows pan · +/− zoom · R rotate · Alt+arrows move hotspot</div>
    </div><aside className="graph-inspector"><span className="inspector-label">Selected room</span><h3>{unit.rooms.find((room) => room.id === selectedId)?.name ?? 'No room'}</h3><p>Confirm only physically adjacent destinations visible through a valid route.</p>{project.manifest.workflowMode === 'advanced' && <div className="coordinate-row"><label>X<input type="number" min="0" max="100" value={selected?.hotspotX.toFixed(1) ?? ''} disabled={project.manifest.readOnly} onChange={(event) => setNodes((current) => current.map((node) => node.roomId === selectedId ? { ...node, hotspotX: Number(event.target.value) } : node))} /></label><label>Y<input type="number" min="0" max="100" value={selected?.hotspotY.toFixed(1) ?? ''} disabled={project.manifest.readOnly} onChange={(event) => setNodes((current) => current.map((node) => node.roomId === selectedId ? { ...node, hotspotY: Number(event.target.value) } : node))} /></label></div>}<span className="inspector-label">Adjacent rooms</span><div className="adjacency-list">{unit.rooms.filter((room) => room.id !== selectedId).map((room) => <label key={room.id}><input type="checkbox" checked={selected?.adjacentRoomIds.includes(room.id) ?? false} disabled={project.manifest.readOnly} onChange={() => toggleConnection(room.id)} /><span>{room.name}</span></label>)}</div>
      {!project.manifest.readOnly && <><div className="spatial-assist"><span className="inspector-label">Assisted draft</span><button type="button" className="secondary-button" disabled={!unit.floorplanAssetId || !nodes.length} onClick={suggestSpatialDraft}>Suggest spatial draft</button>{suggestion && <div className="suggestion-review"><strong>Unapproved suggestion</strong><span>Rooms: retain {unit.rooms.length} user-entered label{unit.rooms.length === 1 ? '' : 's'}; no label is guessed.</span><span>Entrance: candidate at {unit.rooms.find((room) => room.id === suggestion.entranceRoomId)?.name}.</span><span>Hotspots: {suggestion.nodes.length} candidate positions.</span><span>Openings: {suggestion.openings.length} candidates · Adjacency: {suggestion.edgeCount} nearest-neighbour routes.</span><small>Heuristic only. Compare every item with the source plan before applying.</small><div><button type="button" onClick={() => setSuggestion(null)}>Dismiss</button><button type="button" onClick={applySpatialSuggestion}>Apply to editable draft</button></div></div>}{suggestionApplied && <div className="suggestion-applied" role="status">Suggestion applied to the draft, not approved. Save or lock separately after plan review.</div>}</div>
      <div className="opening-editor"><div className="opening-editor-heading"><span className="inspector-label">Entrance and openings</span><button type="button" disabled={nodes.length < 2} onClick={addOpening}>+ Opening</button></div><label className="entrance-select">Confirmed entrance room<select value={entranceRoomId} onChange={(event) => setEntrance(event.target.value)}><option value="">Not confirmed</option>{unit.rooms.map((room) => <option key={room.id} value={room.id}>{room.name}</option>)}</select></label>{openings.filter((opening) => opening.kind !== 'entrance').map((opening) => <div className="opening-row" key={opening.id}><div><select aria-label={`${opening.id} type`} value={opening.kind} onChange={(event) => updateOpening(opening.id, { kind: event.target.value as OpeningInput['kind'] })}><option value="opening">Opening</option><option value="door">Door</option></select><button type="button" aria-label={`Remove ${opening.id}`} onClick={() => setOpenings((current) => current.filter((candidate) => candidate.id !== opening.id))}>×</button></div><div><select aria-label={`${opening.id} from room`} value={opening.fromRoomId} onChange={(event) => updateOpening(opening.id, { fromRoomId: event.target.value })}>{unit.rooms.map((room) => <option key={room.id} value={room.id}>{room.name}</option>)}</select><span>→</span><select aria-label={`${opening.id} to room`} value={opening.toRoomId ?? ''} onChange={(event) => updateOpening(opening.id, { toRoomId: event.target.value })}>{unit.rooms.map((room) => <option key={room.id} value={room.id}>{room.name}</option>)}</select></div><div className="opening-coordinates"><label>X<input type="number" min="0" max="100" value={opening.x.toFixed(1)} onChange={(event) => updateOpening(opening.id, { x: Number(event.target.value) })} /></label><label>Y<input type="number" min="0" max="100" value={opening.y.toFixed(1)} onChange={(event) => updateOpening(opening.id, { y: Number(event.target.value) })} /></label></div></div>)}</div></>}
      <div className="graph-rule"><strong>Lock requirement</strong><span>Every hotspot must be inside the plan and all rooms must form one connected graph.</span></div></aside></div>
    {error && <div className="form-error graph-error">{error}</div>}<footer className="graph-actions"><span>{project.manifest.readOnly ? 'Example graph is read-only.' : unit.roomGraphLocked ? 'Changing this locked topology will invalidate dependent approvals.' : 'Saving a draft does not approve spatial accuracy.'}</span><div><button className="secondary-button" onClick={onClose}>Cancel</button>{!project.manifest.readOnly && <button className="secondary-button" disabled={saving || !unit.floorplanAssetId || !unit.rooms.length} onClick={() => save(false)}>Save draft</button>}{!project.manifest.readOnly && <button className="primary-button" disabled={saving || !unit.floorplanAssetId || !unit.rooms.length} onClick={() => save(true)}>{saving ? 'Saving…' : 'Lock room graph'}</button>}</div></footer>
  </section></div>;
}

function ProjectDetail({ project, onBack, onTour, onCreative, onFiles, onAddUnit, onImport, onEdit, onArchive, onDuplicate, onExport, onOpenUnit, onReviewAsset, onSetWorkflowMode }: { project: ProjectRecord; onBack: () => void; onTour: () => void; onCreative: () => void; onFiles: () => void; onAddUnit: () => void; onImport: () => void; onEdit: () => void; onArchive: () => void; onDuplicate: () => void; onExport: () => void; onOpenUnit: (unit: UnitRecord) => void; onReviewAsset: (assetId: string, decision: 'accepted' | 'needs_review' | 'rejected', reasonCode?: AssetRejectionReason, notes?: string) => Promise<void>; onSetWorkflowMode: (mode: 'standard' | 'advanced') => void }) {
  const { manifest } = project;
  const advancedMode = manifest.workflowMode === 'advanced';
  const [rejectingAssetId, setRejectingAssetId] = useState('');
  const [rejectionReason, setRejectionReason] = useState<AssetRejectionReason>('quality_failure');
  const [rejectionNotes, setRejectionNotes] = useState('');
  const rejectingAsset = manifest.assets.find((asset) => asset.id === rejectingAssetId);
  const acceptedVisualEvidence = manifest.assets.some((asset) => asset.status === 'accepted' && ['renders', 'photos'].includes(asset.category));
  const allFloorplansReady = manifest.units.length > 0 && manifest.units.every((unit) => Boolean(unit.floorplanAssetId));
  const hasRooms = manifest.units.some((unit) => unit.rooms.length > 0);
  const allRoomGraphsLocked = manifest.units.length > 0 && manifest.units.every((unit) => unit.rooms.length > 0 && unit.roomGraphLocked);
  const allIdentitiesReady = hasRooms && manifest.units.flatMap((unit) => unit.rooms).every((room) => Boolean(room.identityAssetId));
  const productionGrounded = allFloorplansReady && acceptedVisualEvidence && allRoomGraphsLocked && allIdentitiesReady && manifest.units.flatMap((unit) => unit.rooms).every((room) => room.panoramaStatus === 'ready') && manifest.assets.some((asset) => asset.status === 'accepted' && asset.category === 'brand') && manifest.disclosure !== 'Disclosure not supplied';
  const floorplanGrounded = allFloorplansReady && acceptedVisualEvidence && allRoomGraphsLocked;
  const readinessTier = productionGrounded ? 'Tier A · Production-grounded' : floorplanGrounded ? 'Tier B · Floor-plan-grounded concept' : acceptedVisualEvidence && !allFloorplansReady ? 'Tier C · Style-led prototype' : 'Unavailable · Intake incomplete';
  const nextIncompleteUnit = manifest.units.find((unit) => !unit.floorplanAssetId || !unit.rooms.length || !unit.roomGraphLocked || unit.rooms.some((room) => !room.identityAssetId));
  async function rejectAsset(event: FormEvent) {
    event.preventDefault();
    if (!rejectingAsset) return;
    await onReviewAsset(rejectingAsset.id, 'rejected', rejectionReason, rejectionNotes);
    setRejectingAssetId('');
    setRejectionNotes('');
  }
  return (
    <main className="detail-view">
      <div className="detail-topbar">
        <button className="back-button" onClick={onBack}>← Projects</button>
        <div className="detail-actions">
          {manifest.readOnly && <span className="read-only-pill">Read-only example</span>}
          {!manifest.readOnly && <button className="secondary-button mode-switch" onClick={() => onSetWorkflowMode(advancedMode ? 'standard' : 'advanced')}>{advancedMode ? 'Advanced mode' : 'Standard mode'} · switch</button>}
          {!manifest.readOnly && !manifest.archivedAt && <button className="secondary-button" onClick={onEdit}>Edit project</button>}
          <button className="secondary-button" onClick={onDuplicate}>Duplicate clean structure</button>
          {!manifest.readOnly && <button className="secondary-button" onClick={onExport}>Export bundle</button>}
          {!manifest.readOnly && <button className="secondary-button" onClick={onArchive}>{manifest.archivedAt ? 'Restore project' : 'Archive project'}</button>}
          <button className="secondary-button" onClick={onFiles}>Project files</button>
          {!manifest.readOnly && !manifest.archivedAt && <button className="secondary-button" onClick={onImport}>+ Import sources</button>}
          <button className="primary-button" disabled={Boolean(manifest.archivedAt) || (!manifest.tourPreviewUrl && !manifest.units.some((unit) => unit.tourAvailable))} onClick={onTour}>Open tour</button>
        </div>
      </div>

      <section className={`project-hero project-hero-visual ${manifest.projectId === 'koya-example' ? 'koya-detail-cover' : 'default-detail-cover'}`}>
        <div className="project-hero-copy">
          <p className="eyebrow">{manifest.company} · {manifest.location}</p>
          <h1>{manifest.name}</h1>
          <p>{manifest.status}</p>
        </div>
        <div className="project-readiness">
          <span>Project readiness</span>
          <div><strong>{manifest.readiness}%</strong><small>{manifest.readiness >= 80 ? 'Ready for review' : 'Production in progress'}</small></div>
          <div className="readiness-track"><i style={{ width: `${manifest.readiness}%` }} /></div>
          <p>{manifest.units.filter((unit) => unit.tourAvailable).length} of {manifest.units.length} unit types have an interactive tour.</p>
        </div>
      </section>

      <section className="project-facts">
        <div><span>{advancedMode ? 'Project ID' : 'Workflow'}</span><strong>{advancedMode ? manifest.projectId : 'Standard property mode'}</strong></div>
        <div><span>Unit types</span><strong>{manifest.units.length}</strong></div>
        <div><span>Evidence files</span><strong>{manifest.assets.length}</strong></div>
        <div><span>Last updated</span><strong>{formatDate(manifest.updatedAt)}</strong><small>Local · private workspace</small></div>
      </section>

      {!manifest.readOnly && !floorplanGrounded && <section className="onboarding-panel"><div className="onboarding-summary"><p className="eyebrow">Guided project setup</p><h2>Complete the evidence foundation</h2><p>Readiness is based only on confirmed project records. Missing information stays unavailable and blocks spatial or production claims.</p><span className={`readiness-tier ${readinessTier.startsWith('Tier C') ? 'tier-c' : ''}`}>{readinessTier}</span></div><div className="onboarding-checklist"><div className="complete"><span>✓</span><strong>Project workspace</strong><small>Created and isolated</small></div><div className={manifest.units.length ? 'complete' : ''}><span>{manifest.units.length ? '✓' : '2'}</span><strong>Unit types</strong><small>{manifest.units.length ? `${manifest.units.length} recorded` : 'Add at least one unit'}</small></div><div className={allFloorplansReady ? 'complete' : ''}><span>{allFloorplansReady ? '✓' : '3'}</span><strong>Floor plans</strong><small>{allFloorplansReady ? 'Assigned to every unit' : 'Import each readable plan'}</small></div><div className={acceptedVisualEvidence ? 'complete' : ''}><span>{acceptedVisualEvidence ? '✓' : '4'}</span><strong>Style evidence</strong><small>{acceptedVisualEvidence ? 'Accepted render or photo' : 'Import and accept a source'}</small></div><div className={allRoomGraphsLocked ? 'complete' : ''}><span>{allRoomGraphsLocked ? '✓' : '5'}</span><strong>Room routes</strong><small>{allRoomGraphsLocked ? 'Topology locked' : 'Add rooms, openings and lock'}</small></div></div><div className="onboarding-recovery"><strong>Next safe action</strong><p>{!manifest.units.length ? 'Add the first unit type. No floor-plan assumptions will be made.' : !allFloorplansReady ? 'Import a readable floor plan for every incomplete unit.' : !acceptedVisualEvidence ? 'Import and explicitly accept an official render or photo. Imported does not mean approved.' : !hasRooms ? 'Open a unit and add only rooms visible on the source plan.' : 'Open the incomplete unit, confirm its entrance and openings, then lock the room graph.'}</p>{!manifest.units.length ? <button className="primary-button" onClick={onAddUnit}>Add first unit</button> : !allFloorplansReady || !acceptedVisualEvidence ? <button className="primary-button" onClick={onImport}>Import sources</button> : nextIncompleteUnit ? <button className="primary-button" onClick={() => onOpenUnit(nextIncompleteUnit)}>Open {nextIncompleteUnit.label}</button> : null}{readinessTier.startsWith('Tier C') && <small>Tier C blocks claims of accurate adjacency, continuous apartment topology, and verified dimensions.</small>}</div></section>}

      <section className="section-block">
        <div className="section-heading">
          <div><p className="eyebrow">Production workspace</p><h2>Project modules</h2></div>
          <span>Each module uses this project&apos;s approved evidence and brand.</span>
        </div>
        <div className="module-grid">
          {(Object.keys(moduleMeta) as Array<keyof typeof moduleMeta>).map((key) => {
            const meta = moduleMeta[key];
            const status = manifest.modules[key];
            return (
              <button className={`module-card ${status}`} key={key} onClick={key === 'creative' ? onCreative : key === 'tour' && (manifest.tourPreviewUrl || manifest.units.some((unit) => unit.tourAvailable)) ? onTour : undefined}>
                <div className="module-icon"><NavGlyph name={meta.icon} /></div>
                <div><h3>{meta.name}</h3><p>{meta.description}</p></div>
                <footer><span className="module-status">{status.replace('_', ' ')}</span><strong>{meta.action} →</strong></footer>
              </button>
            );
          })}
        </div>
      </section>

      <section className="section-block">
        <div className="section-heading">
          <div><p className="eyebrow">Development structure</p><h2>Unit types</h2></div>
          {!manifest.readOnly && !manifest.archivedAt && <button className="secondary-button" onClick={onAddUnit}>+ Add unit</button>}
        </div>
        <div className="unit-list">
          {manifest.units.length === 0 ? (
            <div className="empty-state"><strong>No unit types yet</strong><span>Add the first floor plan to begin the project audit.</span></div>
          ) : manifest.units.map((unit) => (
            <div className="unit-row" key={unit.id}>
              <div className="unit-plan-mini"><span>{unit.id}</span></div>
              <div className="unit-main"><strong>{unit.label}</strong><span>{unit.summary}</span></div>
              <span className={`unit-status ${unit.tourAvailable ? 'ready' : ''}`}>{unit.status}</span>
              <button className="row-action" onClick={() => onOpenUnit(unit)}>{unit.tourAvailable ? 'Tour workspace' : 'Open unit'} <span>→</span></button>
            </div>
          ))}
        </div>
      </section>

      <section className="section-block" id="evidence-register">
        <div className="section-heading">
          <div><p className="eyebrow">Evidence register</p><h2>Source assets</h2></div>
          <span>{manifest.assets.length} imported file{manifest.assets.length === 1 ? '' : 's'} · inferred categories still require review.</span>
        </div>
        <div className="asset-register">
          {manifest.assets.length === 0 ? (
            <button className="asset-empty" disabled={manifest.readOnly} onClick={manifest.readOnly ? undefined : onImport}><span>＋</span><strong>{manifest.readOnly ? 'No bundled source register in this example' : 'Import the first approved source'}</strong><small>Floor plans, renders, photos, videos, brand files or sales copy</small></button>
          ) : manifest.assets.slice().reverse().map((asset) => (
            <div className="asset-row" key={asset.id}>
              <div className="asset-icon">{categoryMeta[asset.category].icon}</div>
              <div className="asset-name"><strong>{asset.name}</strong><span>{advancedMode ? asset.relativePath : 'Local source preserved'}</span></div>
              <div className="asset-meta"><span>{asset.unitId ? `Unit ${asset.unitId}` : 'Project'}</span><span>{formatBytes(asset.sizeBytes)}</span></div>
              <span className={`evidence-pill ${asset.evidenceClass}`}>{asset.evidenceClass.replaceAll('_', ' ')}</span>
              <span className={`asset-status ${asset.status}`}>{asset.status.replace('_', ' ')}</span>
              {asset.duplicateOfAssetId && <span className="asset-status needs_review">duplicate</span>}
              {!manifest.readOnly && !manifest.archivedAt && <div className="review-actions">{asset.status !== 'rejected' && <button className={asset.status === 'accepted' ? 'active' : ''} onClick={() => onReviewAsset(asset.id, 'accepted')}>Accept</button>}<button className={asset.status === 'needs_review' ? 'active warning' : ''} onClick={() => onReviewAsset(asset.id, 'needs_review')}>{asset.status === 'rejected' ? 'Reopen' : 'Review'}</button><button className={asset.status === 'rejected' ? 'active danger' : 'danger'} disabled={asset.status === 'rejected'} onClick={() => setRejectingAssetId(asset.id)}>Reject</button></div>}
            </div>
          ))}
        </div>
      </section>
      {rejectingAsset && <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && setRejectingAssetId('')}><form className="new-project-dialog compact-dialog rejection-dialog" onSubmit={rejectAsset}><div className="dialog-heading"><div><p className="eyebrow">Immutable review event</p><h2>Reject source asset</h2></div><button type="button" className="icon-button" onClick={() => setRejectingAssetId('')}>×</button></div><p className="dialog-copy"><strong>{rejectingAsset.name}</strong> will remain in project history but cannot be assigned or reused while rejected. Existing dependent outputs return to review.</p><label>Reason<select value={rejectionReason} onChange={(event) => setRejectionReason(event.target.value as AssetRejectionReason)}><option value="rights_missing">Usage rights missing</option><option value="evidence_conflict">Conflicts with approved evidence</option><option value="unreadable">Unreadable or corrupt</option><option value="duplicate">Duplicate source</option><option value="too_small">Insufficient dimensions</option><option value="incorrect_unit">Incorrect unit</option><option value="topology_conflict">Topology conflict</option><option value="quality_failure">Quality failure</option><option value="other">Other</option></select></label><label>Review notes<textarea rows={4} maxLength={500} required={rejectionReason === 'other'} value={rejectionNotes} onChange={(event) => setRejectionNotes(event.target.value)} placeholder="Optional reviewer context; required for Other." /></label><div className="dialog-actions"><button type="button" className="secondary-button" onClick={() => setRejectingAssetId('')}>Cancel</button><button className="primary-button danger-button">Reject and invalidate reuse</button></div></form></div>}
    </main>
  );
}

function UnitWorkspace({ project, unit, onBack, onImport, onAddRoom, onEditGraph, onTour, onAssignIdentity, onAssignPanorama, onPreparePanorama, onManageMedia }: { project: ProjectRecord; unit: UnitRecord; onBack: () => void; onImport: () => void; onAddRoom: () => void; onEditGraph: () => void; onTour: () => void; onAssignIdentity: (room: RoomRecord) => void; onAssignPanorama: (room: RoomRecord) => void; onPreparePanorama: (room: RoomRecord) => void; onManageMedia: (room: RoomRecord) => void }) {
  const rooms = unit.rooms ?? [];
  const unitAssets = (project.manifest.assets ?? []).filter((asset) => asset.unitId === unit.id);
  const identityAssets = (project.manifest.assets ?? []).filter((asset) => asset.status === 'accepted' && ['renders', 'photos'].includes(asset.category) && (!asset.unitId || asset.unitId === unit.id));
  const panoramaAssets = (project.manifest.assets ?? []).filter((asset) => asset.status === 'accepted' && asset.category === 'panoramas' && (!asset.unitId || asset.unitId === unit.id));
  const jobs = (project.manifest.generationJobs ?? []).filter((job) => job.unitId === unit.id);
  return <main className="detail-view unit-workspace">
    <div className="detail-topbar"><button className="back-button" onClick={onBack}>← {project.manifest.name}</button><div className="detail-actions"><button className="secondary-button" onClick={onImport}>+ Import unit sources</button>{unit.tourAvailable && <button className="primary-button" onClick={onTour}>Open live tour</button>}</div></div>
    <section className="unit-hero"><div><p className="eyebrow">Unit production workspace</p><h1>{unit.label}</h1><p>{unit.summary}</p></div><div className="unit-progress"><strong>{rooms.length}</strong><span>tour stops</span><strong>{unitAssets.length}</strong><span>unit assets</span></div></section>
    <section className="unit-gates"><div className={unit.floorplanAssetId ? 'complete' : ''}><span>1</span><strong>Floor plan</strong><small>{unit.floorplanAssetId ? (project.manifest.readOnly ? 'Reference confirmed' : 'Imported · needs audit') : 'Required'}</small></div><div className={unit.roomGraphLocked ? 'complete' : rooms.length ? 'active' : ''}><span>2</span><strong>Room graph</strong><small>{unit.roomGraphLocked ? `${rooms.length} locked stops` : rooms.length ? `${rooms.length} draft stops` : 'Not started'}</small></div><div><span>3</span><strong>Panoramas</strong><small>{rooms.filter((room) => room.panoramaStatus === 'ready').length} ready</small></div><div className={unit.tourAvailable ? 'complete' : ''}><span>4</span><strong>Publish</strong><small>{unit.tourAvailable ? 'Live reference' : 'Blocked'}</small></div></section>
    <section className="section-block"><div className="section-heading"><div><p className="eyebrow">Floor-plan-grounded route</p><h2>Rooms and tour stops</h2></div><div className="section-actions"><button className="secondary-button" disabled={!unit.floorplanAssetId || !rooms.length} onClick={onEditGraph}>{unit.roomGraphLocked ? 'Review room graph' : 'Position hotspots'}</button>{!project.manifest.readOnly && <button className="secondary-button" onClick={onAddRoom}>+ Add room</button>}</div></div>
      <div className="room-grid">{rooms.length === 0 ? <div className="room-empty"><span>⌗</span><strong>No rooms confirmed yet</strong><p>Import the floor plan, then add room labels and confirm their physical adjacency.</p><div><button className="secondary-button" onClick={onImport}>Import floor plan</button>{!project.manifest.readOnly && <button className="primary-button" onClick={onAddRoom}>Add first room</button>}</div></div> : rooms.map((room, index) => { const panorama = project.manifest.assets.find((asset) => asset.id === room.panoramaAssetId); return <article className="room-card" key={room.id}><div className="room-card-top"><span>{String(index + 1).padStart(2, '0')}</span><i className={room.status} /></div><h3>{room.name}</h3><p>{room.status.replaceAll('_', ' ')}</p><div className="coverage-row"><span>Identity image</span><strong>{room.identityAssetId || room.status === 'approved' ? 'Ready' : 'Missing'}</strong></div><div className="coverage-row"><span>360° panorama</span><strong className={room.panoramaStatus}>{room.panoramaStatus.replaceAll('_', ' ')}</strong></div><div className="coverage-row"><span>Delivery set</span><strong>{panorama?.derivativeRelativePaths.length ? 'Ready' : 'Required'}</strong></div><div className="coverage-row"><span>Room video</span><strong>{room.videoAssetId ? 'Ready' : 'Optional'}</strong></div><div className="room-card-actions">{room.panoramaStatus === 'ready' ? <button onClick={onTour}>Review panorama</button> : room.status === 'needs_evidence' ? <button disabled={!identityAssets.length} onClick={() => onAssignIdentity(room)}>{identityAssets.length ? 'Choose identity anchor' : 'Accept a render first'}</button> : panoramaAssets.length && unit.roomGraphLocked ? <button onClick={() => onAssignPanorama(room)}>Assign supplied panorama</button> : <button onClick={() => onPreparePanorama(room)}>Prepare generation</button>}{!project.manifest.readOnly && <button onClick={() => onManageMedia(room)}>Room media & delivery</button>}</div></article>; })}</div>
    </section>
    <section className="section-block"><div className="section-heading"><div><p className="eyebrow">Generation centre</p><h2>Approval packages</h2></div><span>{jobs.length} package{jobs.length === 1 ? '' : 's'} · saving a draft never submits a paid task.</span></div><div className="job-list">{jobs.length === 0 ? <div className="job-empty">No panorama generation packages have been prepared for this unit.</div> : jobs.slice().reverse().map((job) => <div className="job-row" key={job.id}><span className="job-icon">◉</span><div><strong>{job.roomName} · 360° panorama</strong><small>{job.outputCount} output · {job.dimensions}</small></div><div><span>Connection</span><strong>{job.connectionMode.replace('_', ' ')}</strong></div><div><span>Price</span><strong>{job.priceStatus}</strong></div><span className={`job-status ${job.status}`}>{job.status.replaceAll('_', ' ')}</span></div>)}</div></section>
    <section className="section-block"><div className="section-heading"><div><p className="eyebrow">Approval gates</p><h2>Production readiness</h2></div></div><div className="gate-notice"><span>!</span><div><strong>Panorama generation is not authorised yet</strong><p>The exact room, inputs, evidence roles, output count, dimensions and current cost must be reviewed before any paid task can be submitted.</p></div><button disabled>Prepare approval package</button></div></section>
  </main>;
}

function LocalTourPreview({ project }: { project: ProjectRecord }) {
  const units = project.manifest.units.filter((unit) => unit.tourAvailable && unit.rooms.some((room) => room.panoramaAssetId));
  const [unitId, setUnitId] = useState(units[0]?.id ?? '');
  const unit = units.find((item) => item.id === unitId) ?? units[0];
  const [roomId, setRoomId] = useState(unit?.rooms.find((room) => room.panoramaAssetId)?.id ?? '');
  const room = unit?.rooms.find((item) => item.id === roomId) ?? unit?.rooms.find((item) => item.panoramaAssetId);
  const [image, setImage] = useState('');
  const [video, setVideo] = useState('');
  const [videoState, setVideoState] = useState<'idle' | 'loading' | 'ready' | 'error'>('idle');
  const [floorplan, setFloorplan] = useState('');
  const [yaw, setYaw] = useState(50);
  const [dragStart, setDragStart] = useState<{ x: number; yaw: number }>();
  useEffect(() => {
    if (!room?.panoramaAssetId) return;
    setImage(''); setYaw(50);
    getAssetDataUrl(project.manifest.projectId, room.panoramaAssetId).then(setImage).catch(() => setImage(''));
  }, [project.manifest.projectId, room?.panoramaAssetId]);
  useEffect(() => {
    setVideo('');
    if (!room?.videoAssetId) { setVideoState('idle'); return; }
    setVideoState('loading');
    getAssetDataUrl(project.manifest.projectId, room.videoAssetId)
      .then((value) => { setVideo(value); setVideoState('ready'); })
      .catch(() => { setVideo(''); setVideoState('error'); });
  }, [project.manifest.projectId, room?.videoAssetId]);
  useEffect(() => {
    setFloorplan('');
    if (unit?.floorplanAssetId) getAssetDataUrl(project.manifest.projectId, unit.floorplanAssetId).then(setFloorplan).catch(() => setFloorplan(''));
  }, [project.manifest.projectId, unit?.floorplanAssetId]);
  useEffect(() => { if (unit && !unit.rooms.some((item) => item.id === roomId && item.panoramaAssetId)) setRoomId(unit.rooms.find((item) => item.panoramaAssetId)?.id ?? ''); }, [roomId, unit]);
  if (!unit || !room) return <div className="preview-empty">No locally assigned panorama tour is ready.</div>;
  const neighbours = room.adjacentRoomIds.map((id) => unit.rooms.find((item) => item.id === id)).filter((item): item is RoomRecord => Boolean(item?.panoramaAssetId));
  return <div className="local-tour-preview">
    <div className="local-tour-stage" role="region" aria-label={`${room.name} panorama, orientation ${Math.round(yaw * 3.6)} degrees`} tabIndex={0} style={image ? { backgroundImage: `url(${image})`, backgroundPosition: `${yaw}% center` } : undefined} onKeyDown={(event) => { if (event.key === 'ArrowLeft' || event.key === 'ArrowRight') { event.preventDefault(); setYaw((current) => ((current + (event.key === 'ArrowLeft' ? -5 : 5)) % 100 + 100) % 100); } }} onPointerDown={(event) => { event.currentTarget.setPointerCapture(event.pointerId); setDragStart({ x: event.clientX, yaw }); }} onPointerMove={(event) => { if (dragStart) setYaw(((dragStart.yaw - (event.clientX - dragStart.x) / 8) % 100 + 100) % 100); }} onPointerUp={() => setDragStart(undefined)} onPointerCancel={() => setDragStart(undefined)}>
      {!image && <div className="panorama-loading">Loading validated panorama…</div>}
      <div className="tour-disclosure">{project.manifest.name} · {unit.label}<strong>{room.name}</strong><span>Drag to inspect · {project.manifest.assets.find((asset) => asset.id === room.panoramaAssetId)?.evidenceClass.replaceAll('_', ' ')}</span></div>
      <div className="tour-orientation-controls"><button onClick={() => setYaw((value) => (value + 95) % 100)} aria-label="Rotate panorama left">←</button><span>Orientation {Math.round(yaw * 3.6)}°</span><button onClick={() => setYaw((value) => (value + 5) % 100)} aria-label="Rotate panorama right">→</button><button className="reset-yaw" onClick={() => setYaw(50)}>Reset view</button></div>
      <div className="adjacent-links">{neighbours.map((neighbour) => <button key={neighbour.id} onClick={() => setRoomId(neighbour.id)}>↑<span>{neighbour.name}</span></button>)}</div>
    </div>
    <div className="local-tour-evidence-row">
      <div className="tour-floorplan"><strong>Floor-plan navigation</strong><div>{floorplan ? <img src={floorplan} alt={`${unit.label} floor plan`} /> : <span>Loading floor plan…</span>}{unit.rooms.filter((item) => item.panoramaAssetId).map((item, index) => <button key={item.id} className={item.id === room.id ? 'active' : ''} style={{ left: `${item.hotspotX ?? 50}%`, top: `${item.hotspotY ?? 50}%` }} aria-label={`Open ${item.name} from floor plan`} onClick={() => setRoomId(item.id)}>{index + 1}</button>)}</div></div>
      <div className="tour-room-video"><strong>Representative video</strong>{videoState === 'ready' && video ? <video controls preload="metadata" poster={undefined} src={video}>Representative room video is unavailable in this runtime.</video> : <span>{videoState === 'loading' ? 'Loading representative video…' : videoState === 'error' ? 'Assigned video could not be loaded.' : 'No accepted room video assigned.'}</span>}</div>
    </div>
    <div className="local-tour-nav"><div><span>Unit</span><select value={unit.id} onChange={(event) => setUnitId(event.target.value)}>{units.map((item) => <option key={item.id} value={item.id}>{item.label}</option>)}</select></div><div className="tour-room-tabs">{unit.rooms.filter((item) => item.panoramaAssetId).map((item) => <button className={item.id === room.id ? 'active' : ''} key={item.id} onClick={() => setRoomId(item.id)}>{item.name}</button>)}</div><span className="local-only-pill">Local preview</span></div>
  </div>;
}

function TourPreview({ project, onClose }: { project: ProjectRecord; onClose: () => void }) {
  const src = project.manifest.tourPreviewUrl;
  const [preset, setPreset] = useState<'desktop-1x' | 'desktop-2x' | 'mobile-portrait' | 'mobile-landscape'>('desktop-1x');
  const [immersive, setImmersive] = useState(false);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const mobilePreset = preset.startsWith('mobile');
  const previewQuery = `texture=${mobilePreset ? 'mobile' : 'auto'}&dpr=${preset === 'desktop-2x' ? '2' : '1'}`;
  useEffect(() => {
    function onMessage(event: MessageEvent) {
      if (event.source !== iframeRef.current?.contentWindow || event.data?.type !== 'koya-tour:immersive-request') return;
      setImmersive((current) => {
        const next = event.data.active === 'toggle' ? !current : Boolean(event.data.active);
        iframeRef.current?.contentWindow?.postMessage({ type: 'koya-tour:immersive-state', active: next }, '*');
        return next;
      });
    }
    window.addEventListener('message', onMessage);
    return () => window.removeEventListener('message', onMessage);
  }, []);
  return (
    <div className={`preview-shell ${immersive ? 'immersive' : ''}`}>
      <div className="preview-toolbar">
        <div><span className="preview-dot" /><strong>{project.manifest.name}</strong><span>Interactive Tour · live reference</span></div>
        <div><div className="preview-presets" aria-label="Preview viewport"><button className={preset === 'desktop-1x' ? 'active' : ''} aria-pressed={preset === 'desktop-1x'} onClick={() => setPreset('desktop-1x')}>Desktop 1×</button><button className={preset === 'desktop-2x' ? 'active' : ''} aria-pressed={preset === 'desktop-2x'} onClick={() => setPreset('desktop-2x')}>Desktop 2×</button><button className={preset === 'mobile-portrait' ? 'active' : ''} aria-pressed={preset === 'mobile-portrait'} onClick={() => setPreset('mobile-portrait')}>Phone portrait</button><button className={preset === 'mobile-landscape' ? 'active' : ''} aria-pressed={preset === 'mobile-landscape'} onClick={() => setPreset('mobile-landscape')}>Phone landscape</button></div><button className="secondary-button" onClick={() => setImmersive(true)}>Full screen</button><button className="secondary-button" disabled={!src} onClick={() => src && window.open(src, '_blank')}>Open in browser</button><button className="icon-button" aria-label="Close tour preview" onClick={onClose}>×</button></div>
      </div>
      <div className={`preview-viewport ${preset}`}>{src ? <iframe ref={iframeRef} src={`${src}${src.includes('?') ? '&' : '?'}${previewQuery}`} title={`${project.manifest.name} interactive tour`} allow="fullscreen" allowFullScreen /> : <LocalTourPreview project={project} />}</div>
      {immersive && <button className="preview-exit-immersive" onClick={() => setImmersive(false)}>Exit full screen</button>}
    </div>
  );
}

function AiProductionWorkspace({ projects, project, onSelectProject, onOpenUnit, onAskAi }: { projects: ProjectRecord[]; project?: ProjectRecord; onSelectProject: (project: ProjectRecord) => void; onOpenUnit: (unit: UnitRecord) => void; onAskAi: (prompt: string) => void }) {
  if (!project) {
    return (
      <main className="detail-view production-view">
        <header className="page-header"><div><p className="eyebrow">AI production</p><h1>Choose a project</h1><p>Generation packages must belong to one development and one evidence register.</p></div></header>
        <div className="production-project-picker">{projects.map((item) => <button key={item.manifest.projectId} onClick={() => onSelectProject(item)}><span>{item.manifest.readOnly ? 'Example' : `${item.manifest.readiness}% ready`}</span><strong>{item.manifest.name}</strong><small>{item.manifest.units.length} unit types · {item.manifest.company}</small></button>)}</div>
      </main>
    );
  }
  const units = project.manifest.units;
  const rooms = units.flatMap((unit) => unit.rooms.map((room) => ({ unit, room })));
  const acceptedAssets = project.manifest.assets.filter((asset) => asset.status === 'accepted');
  const hasFloorplan = units.some((unit) => Boolean(unit.floorplanAssetId));
  const hasStyleEvidence = acceptedAssets.some((asset) => ['renders', 'photos'].includes(asset.category));
  const tier = project.manifest.readOnly ? 'Licensed example' : hasFloorplan && hasStyleEvidence ? 'Tier B · Floor-plan-grounded concept' : hasFloorplan ? 'Intake incomplete · style evidence required' : 'Tier C · Style-led prototype only';
  const identityReady = rooms.filter(({ room }) => Boolean(room.identityAssetId) || room.status === 'approved').length;
  const panoramaReady = rooms.filter(({ room }) => room.panoramaStatus === 'ready').length;
  const activeJobs = project.manifest.generationJobs.filter((job) => ['draft', 'blocked_capability', 'awaiting_approval', 'submitted'].includes(job.status));
  return (
    <main className="detail-view production-view">
      <div className="detail-topbar"><button className="back-button" onClick={() => onSelectProject(project)}>Current project · {project.manifest.name}</button><div className="detail-actions"><button className="secondary-button" onClick={() => onAskAi(`Review the AI production readiness for ${project.manifest.name}. Use its evidence register and room statuses; list the three most important next actions without claiming generation has run.`)}>✦ Review with AI</button></div></div>
      <section className="production-hero"><div><p className="eyebrow">Evidence-controlled workflow</p><h1>AI production</h1><p>Fill room coverage without losing floor-plan topology, provenance or approval history.</p></div><div className="production-tier"><span>Input classification</span><strong>{tier}</strong><small>{project.manifest.readOnly ? 'Reference outputs are locked.' : 'Recalculated from accepted project evidence.'}</small></div></section>
      <section className="production-metrics"><div><span>Tour stops</span><strong>{rooms.length}</strong><small>Across {units.length} unit types</small></div><div><span>Identity anchors</span><strong>{identityReady}/{rooms.length || 0}</strong><small>Approved or assigned</small></div><div><span>Panoramas ready</span><strong>{panoramaReady}/{rooms.length || 0}</strong><small>Runtime-ready status</small></div><div><span>Active packages</span><strong>{activeJobs.length}</strong><small>No silent submission</small></div></section>
      {!project.manifest.readOnly && (!hasFloorplan || !hasStyleEvidence) && <div className="production-warning"><span>!</span><div><strong>Generation is not evidence-ready</strong><p>{!hasFloorplan ? 'Import and assign a readable floor plan before spatial generation.' : 'Accept at least one project render or photo as style evidence before creating room identities.'}</p></div></div>}
      <section className="section-block"><div className="section-heading"><div><p className="eyebrow">AI-01 asset-gap matrix</p><h2>Room coverage</h2></div><span>Identity first, then dependent panorama and fallback assets.</span></div>
        <div className="asset-matrix">
          <div className="matrix-head"><span>Unit / room</span><span>Floor plan</span><span title="Room identity anchor">Identity</span><span>Panorama</span><span title="View through the route into this room">Threshold</span><span title="Reverse view back through the route">Reverse</span><span title="Non-WebGL fallback image">Still fallback</span><span>Video</span><span>Poster</span><span>Next action</span></div>
          {rooms.map(({ unit, room }) => {
            const job = project.manifest.generationJobs.find((item) => item.unitId === unit.id && item.roomId === room.id);
            const identity = Boolean(room.identityAssetId) || room.status === 'approved';
            const role = (assetId: string | undefined, required: boolean) => <span className={assetId ? 'matrix-ok' : required ? 'matrix-missing' : 'matrix-optional'}>{assetId ? 'Ready' : required ? 'Required' : 'Optional'}</span>;
            return <div className="matrix-row" key={`${unit.id}-${room.id}`}><div><strong>{room.name}</strong><small>{unit.label}</small></div><span className={unit.floorplanAssetId ? 'matrix-ok' : 'matrix-missing'}>{unit.floorplanAssetId ? 'Available' : 'Missing'}</span><span className={identity ? 'matrix-ok' : 'matrix-missing'}>{identity ? 'Ready' : 'Required'}</span><span className={room.panoramaStatus === 'ready' ? 'matrix-ok' : 'matrix-missing'}>{room.panoramaStatus === 'ready' ? 'Ready' : 'Required'}</span>{role(room.thresholdAssetId, true)}{role(room.reverseAssetId, false)}{role(room.stillFallbackAssetId, true)}{role(room.videoAssetId, false)}{role(room.posterAssetId, false)}<button onClick={() => onOpenUnit(unit)}>{job ? 'Open package' : room.panoramaStatus === 'ready' ? 'Review stop' : identity ? 'Prepare package' : 'Assign evidence'}</button></div>;
          })}
          {rooms.length === 0 && <div className="matrix-empty"><strong>No audited rooms yet</strong><span>Open a unit, import its floor plan and add confirmed room stops before preparing AI assets.</span></div>}
        </div>
      </section>
      <section className="production-gates"><div><span>1</span><strong>Evidence</strong><small>Floor plan and accepted references</small></div><i>→</i><div><span>2</span><strong>Identity anchor</strong><small>One canonical room view</small></div><i>→</i><div><span>3</span><strong>Approval package</strong><small>Inputs, dimensions and price status</small></div><i>→</i><div><span>4</span><strong>Human approval</strong><small>One task and one output</small></div><i>→</i><div><span>5</span><strong>Panorama QA</strong><small>Projection, seam and runtime</small></div></section>
    </main>
  );
}

function CreativeStudioWorkspace({ projects, project, aiReady, onSelectProject, onUpdated, onSettings }: { projects: ProjectRecord[]; project?: ProjectRecord; aiReady: boolean; onSelectProject: (project: ProjectRecord) => void; onUpdated: (project: ProjectRecord) => void; onSettings: () => void }) {
  const [creating, setCreating] = useState(false);
  const [generating, setGenerating] = useState(false);
  const [saving, setSaving] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [exportedPath, setExportedPath] = useState('');
  const [creationMode, setCreationMode] = useState<'codex' | 'traditional'>('codex');
  const [codexRequest, setCodexRequest] = useState('');
  const [error, setError] = useState('');
  const [draft, setDraft] = useState<CreativeBrief>();
  const [selectedId, setSelectedId] = useState<string>();
  const [a4Html, setA4Html] = useState('');
  const a4ViewportRef = useRef<HTMLDivElement>(null);
  const [a4PreviewScale, setA4PreviewScale] = useState(0.5);
  const [previewingA4, setPreviewingA4] = useState(false);
  const [form, setForm] = useState<GenerateCreativeBriefInput>({ kind: 'poster', posterMode: 'ai_creative', a4DocumentType: 'unit_sales_sheet', campaignName: '', unitId: undefined, audience: 'Prospective property buyers', objective: 'Introduce the development and invite enquiry', format: 'instagram_portrait', evidenceAssetIds: [] });
  const briefs = project?.manifest.creativeJobs ?? [];
  const defaultBrief = briefs.find((item) => item.posterMode === 'ai_creative') ?? briefs[0];
  const selected = draft ?? (selectedId === '' ? undefined : briefs.find((item) => item.id === selectedId) ?? defaultBrief);
  const aiPosterCount = briefs.filter((item) => item.posterMode === 'ai_creative').length;
  const isAiPoster = selected?.posterMode === 'ai_creative';
  const a4Briefs = briefs.filter((item) => item.kind === 'poster' && item.posterMode !== 'ai_creative');
  const activeA4Index = selected ? a4Briefs.findIndex((item) => item.id === selected.id) : -1;
  const selectedA4Meta = a4DocumentMeta(selected?.a4DocumentType);
  const salesTasks = a4DocumentTypes.map((item, index) => {
    const outputs = a4Briefs.filter((brief) => brief.a4DocumentType === item.id);
    const latest = outputs.at(-1);
    return {
      ...item,
      taskId: `SALES-${String(index + 1).padStart(2, '0')}`,
      outputs,
      latest,
      state: outputs.some((brief) => brief.status === 'preview_ready') ? 'preview_ready' : latest ? 'draft' : 'not_started',
    } as const;
  });
  const readySalesTaskCount = salesTasks.filter((task) => task.state === 'preview_ready').length;
  const designSpecState = project?.manifest.designSpec?.status ?? 'not_started';

  useEffect(() => {
    window.scrollTo({ top: 0, behavior: 'auto' });
    const jobs = project?.manifest.creativeJobs ?? [];
    const initial = jobs.find((item) => item.posterMode === 'ai_creative') ?? jobs[0];
    setSelectedId(initial?.id);
    if (initial) {
      setForm((current) => initial.kind === 'video'
        ? { ...current, kind: 'video', posterMode: undefined, format: initial.format }
        : initial.posterMode === 'ai_creative'
          ? { ...current, kind: 'poster', posterMode: 'ai_creative', format: initial.format }
          : { ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: initial.a4DocumentType ?? 'unit_sales_sheet', format: 'a4_portrait' });
    }
    setDraft(undefined);
    setA4Html('');
  }, [project?.manifest.projectId]);
  useEffect(() => {
    if (!project || !selected || selected.posterMode === 'ai_creative' || selected.kind !== 'poster' || selected.status === 'draft') { setA4Html(''); return; }
    getCreativeA4Html(project.manifest.projectId, selected.id).then(setA4Html).catch(() => setA4Html(''));
  }, [project, selected?.id, selected?.kind, selected?.status]);
  useLayoutEffect(() => {
    const viewport = a4ViewportRef.current;
    if (!viewport || !selected || selected.kind !== 'poster' || isAiPoster || draft) return;
    const fitA4ToViewport = () => {
      const availableWidth = Math.max(viewport.clientWidth - 24, 1);
      const availableHeight = Math.max(viewport.clientHeight - 24, 1);
      setA4PreviewScale(Math.min(availableWidth / 794, availableHeight / 1123));
    };
    fitA4ToViewport();
    const observer = new ResizeObserver(fitA4ToViewport);
    observer.observe(viewport);
    return () => observer.disconnect();
  }, [selected?.id, selected?.kind, isAiPoster, draft]);

  function updateDraft(field: keyof Pick<CreativeBrief, 'headline' | 'subheadline' | 'body' | 'callToAction' | 'visualDirection' | 'imagePrompt'>, value: string) {
    setDraft((current) => current ? { ...current, [field]: value } : current);
  }

  function updateHighlights(value: string) {
    setDraft((current) => current ? { ...current, projectHighlights: value.split('\n').map((item) => item.trim()).filter(Boolean).slice(0, 5) } : current);
  }

  async function generate(event: FormEvent) {
    event.preventDefault();
    if (!project) return;
    if (!aiReady) { onSettings(); return; }
    if (creationMode === 'codex' && !codexRequest.trim()) { setError('Tell Codex what you want to create.'); return; }
    setGenerating(true); setError('');
    const input = creationMode === 'codex' ? {
      ...form,
      request: codexRequest.trim(),
      campaignName: form.campaignName.trim() || `${project.manifest.name} campaign`,
      audience: form.audience.trim() || 'Prospective property buyers',
      objective: codexRequest.trim(),
    } : form;
    try { setDraft(await generateCreativeBrief(project.manifest.projectId, input)); setCreating(false); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setGenerating(false); }
  }

  async function save() {
    if (!project || !draft) return;
    setSaving(true); setError('');
    try {
      const updated = await saveCreativeBrief(project.manifest.projectId, draft, project.manifest.updatedAt);
      onUpdated(updated); setSelectedId(draft.id); setDraft(undefined);
    } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setSaving(false); }
  }

  async function exportPdf() {
    if (!project || !selected || selected.posterMode !== 'a4_sales_sheet') return;
    setExporting(true); setError(''); setExportedPath('');
    try {
      const destination = await saveFileDialog({ defaultPath: `${selected.id}.pdf`, filters: [{ name: 'PDF document', extensions: ['pdf'] }] });
      if (!destination) return;
      setExportedPath(await exportA4Pdf(project.manifest.projectId, selected.id, destination));
    }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setExporting(false); }
  }

  function turnA4(direction: -1 | 1) {
    if (activeA4Index < 0) return;
    const next = a4Briefs[activeA4Index + direction];
    if (next) {
      setDraft(undefined);
      setSelectedId(next.id);
    }
  }

  function selectCreativeBrief(item: CreativeBrief) {
    setDraft(undefined);
    setSelectedId(item.id);
    setForm((current) => item.kind === 'video'
      ? { ...current, kind: 'video', posterMode: undefined, format: item.format }
      : item.posterMode === 'ai_creative'
        ? { ...current, kind: 'poster', posterMode: 'ai_creative', format: item.format }
        : { ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: item.a4DocumentType ?? 'unit_sales_sheet', format: 'a4_portrait' });
  }

  function chooseWorkbenchRoute(route: 'ai' | 'a4' | 'video') {
    const example = briefs.find((item) => route === 'ai'
      ? item.posterMode === 'ai_creative'
      : route === 'a4'
        ? item.kind === 'poster' && item.posterMode !== 'ai_creative'
        : item.kind === 'video');
    if (example) selectCreativeBrief(example);
    else {
      setDraft(undefined);
      setForm((current) => route === 'ai'
        ? { ...current, kind: 'poster', posterMode: 'ai_creative', format: 'instagram_portrait' }
        : route === 'a4'
          ? { ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: current.a4DocumentType ?? 'unit_sales_sheet', format: 'a4_portrait' }
          : { ...current, kind: 'video', posterMode: undefined, format: 'social_vertical' });
    }
    setCodexRequest('');
    setCreationMode('codex');
  }

  function openSalesTask(task: (typeof salesTasks)[number]) {
    if (!project) return;
    setCreationMode('codex');
    setDraft(undefined);
    setForm((current) => ({ ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: task.id, format: 'a4_portrait' }));
    if (task.latest) {
      selectCreativeBrief(task.latest);
      setCodexRequest('');
      return;
    }
    setSelectedId('');
    setCodexRequest(`Create the ${task.label.toLowerCase()} for ${project.manifest.name}. First organise the accepted project evidence into SOT.md, then create document.md, then derive document.html. Mark missing facts unavailable.`);
  }

  function openDesignSpecTask() {
    if (!project) return;
    setCreationMode('codex');
    setDraft(undefined);
    setSelectedId('');
    setCodexRequest(`Create or refine the independent Project Design Specification for ${project.manifest.name}. Analyse only its approved website, brand assets and project evidence. Define colour, typography, A4 grid, image treatment and reusable document components. Save PROJECT_DESIGN_SPEC.md and design-spec.json before deriving any sales document.`);
  }

  if (!project) return <main className="detail-view creative-view"><header className="page-header"><div><p className="eyebrow">Project creative system</p><h1>Creative Studio</h1><p>Select a development. Every brief, source and output stays isolated inside that project.</p></div></header><div className="creative-project-picker">{projects.map((item) => <button key={item.manifest.projectId} onClick={() => onSelectProject(item)}><span style={{ background: item.manifest.brand.accent }} /><div><strong>{item.manifest.name}</strong><small>{item.manifest.company} · {item.manifest.creativeJobs.length} creative items</small></div><b>Open →</b></button>)}</div></main>;

  return <main className="detail-view creative-view creative-workbench-view">
    <header className="creative-workbench-header"><div><p className="eyebrow">{project.manifest.name} · Creative Studio</p><h1>Property marketing workspace</h1></div><div><span>{readySalesTaskCount}/7 sales tasks</span><span>{aiPosterCount} poster</span><span>{briefs.filter((item) => item.kind === 'video').length} video</span>{!project.manifest.readOnly && <button className="secondary-button" onClick={() => setCreating(true)}>Advanced form</button>}</div></header>
    {error && <div className="page-error">Creative Studio: {error}</div>}
    <div className="creative-three-column">
      <aside className="creative-left-column"><div className="creative-column-title"><p className="eyebrow">Create</p><h2>Creative types</h2></div><div className="creative-route-stack"><button className={form.posterMode === 'ai_creative' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('ai')}><span>✦</span><div><strong>AI poster</strong><small>Image Model visual</small></div></button><button className={form.posterMode === 'a4_sales_sheet' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('a4')}><span>A4</span><div><strong>Property document</strong><small>Sales sheets and packs</small></div></button><button className={form.kind === 'video' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('video')}><span>▶</span><div><strong>Video</strong><small>Storyboard package</small></div></button></div><div className="creative-output-list"><div><strong>Project outputs</strong><span>{briefs.length}</span></div>{briefs.map((item) => { const aiPoster = item.posterMode === 'ai_creative'; return <button key={item.id} className={selected?.id === item.id && !draft ? 'active' : ''} onClick={() => selectCreativeBrief(item)}><span>{item.kind === 'video' ? '▶' : aiPoster ? '✦' : 'A4'}</span><div><small>{aiPoster ? 'AI poster' : item.kind === 'video' ? 'Video' : a4DocumentMeta(item.a4DocumentType).label}</small><strong>{item.title}</strong></div></button>; })}{briefs.length === 0 && <p>No saved outputs yet.</p>}</div></aside>

      <section className="creative-codex-column"><div className="creative-column-title"><p className="eyebrow">Creative workspace</p><h2>Ask Codex</h2><span>{project.manifest.name} and its accepted evidence are in context.</span></div><div className="creative-center-tools"><div className="creative-center-routes"><button className={form.posterMode === 'ai_creative' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('ai')}><span>✦</span>AI poster</button><button className={form.posterMode === 'a4_sales_sheet' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('a4')}><span>A4</span>Property document</button><button className={form.kind === 'video' ? 'active' : ''} onClick={() => chooseWorkbenchRoute('video')}><span>▶</span>Video</button></div><label><span>Project output</span><select value={selected && !draft ? selected.id : ''} onChange={(event) => { const item = briefs.find((brief) => brief.id === event.target.value); if (item) selectCreativeBrief(item); }}><option value="">New request</option>{briefs.map((item) => <option key={item.id} value={item.id}>{item.title}</option>)}</select></label></div><section className="creative-sales-tasks"><div><strong>Project design & sales tasks</strong><span>{readySalesTaskCount}/7 sales ready</span></div><small>Design specification first · then every document: SOT → Markdown → HTML → PDF</small><button className={`creative-design-spec-task ${designSpecState}`} onClick={openDesignSpecTask}><span>D01</span><div><strong>Project Design Specification</strong><small>{designSpecState === 'approved' ? 'Approved · all outputs inherit this system' : designSpecState === 'draft' ? 'Draft · approval required' : 'Not started · define this project’s visual language'}</small></div><b>{designSpecState === 'approved' ? '✓' : '+'}</b></button><div className="creative-sales-task-grid">{salesTasks.map((task) => <button key={task.id} className={`${form.posterMode === 'a4_sales_sheet' && form.a4DocumentType === task.id ? 'active' : ''} ${task.state}`} onClick={() => openSalesTask(task)}><span>{task.taskId.replace('SALES-', 'S')}</span><div><strong>{task.label}</strong><small>{task.state === 'preview_ready' ? 'Preview ready' : task.state === 'draft' ? 'Draft saved' : 'Not started'}</small></div><b>{task.state === 'preview_ready' ? '✓' : task.state === 'draft' ? '•' : '+'}</b></button>)}</div></section><div className="creative-chat-thread"><article className="assistant creative-primary-message"><span>✦</span><div><strong>{selected ? `Working on ${selected.title}` : 'What should we make?'}</strong><p>{selected ? 'Tell me what to create or change. I will use this project context and keep unsupported facts out.' : 'Choose a creative type above, then tell me the result in plain language.'}</p><div className="creative-chat-actions"><button onClick={() => setCodexRequest('Make the message more premium and concise. Keep every claim grounded in the project evidence.')}>Premium + concise</button><button onClick={() => setCodexRequest('Review the selling points and remove anything that is not supported by accepted project evidence.')}>Check claims</button><button onClick={() => setCodexRequest('Create a sales-ready version for an owner-occupier audience.')}>Owner-occupier</button></div>
        {form.posterMode === 'a4_sales_sheet' && <><p className="creative-chat-prompt-label">Or start a property document:</p><div className="creative-chat-actions document-prompts">{a4DocumentTypes.map((item) => <button key={item.id} onClick={() => { setForm((current) => ({ ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: item.id, format: 'a4_portrait' })); setCodexRequest(`Create a ${item.label.toLowerCase()} for ${project.manifest.name}. Use only accepted project evidence and mark missing facts unavailable.`); }}>{item.label}</button>)}</div></>}
        {selected && <p className="creative-chat-context-note">Context: {selected.evidenceAssetIds.length ? `${selected.evidenceAssetIds.length} accepted source asset(s)` : 'project manifest facts only'} · Review before saving</p>}
      </div></article>
        {codexRequest && <article className="user"><p>{codexRequest}</p></article>}
        {draft && <article className="assistant operation-message compact"><span>→</span><div><strong>Draft ready for review</strong><p>The project has not been changed yet.</p><div className="creative-chat-review-actions"><button className="secondary-button" onClick={() => setDraft(undefined)}>Discard</button><button className="primary-button" disabled={saving} onClick={save}>{saving ? 'Saving…' : 'Review & save'}</button></div></div></article>}
      </div><form className="creative-inline-composer" onSubmit={generate}><textarea rows={4} value={codexRequest} onChange={(event) => setCodexRequest(event.target.value)} placeholder={form.posterMode === 'a4_sales_sheet' ? `Create a ${a4DocumentMeta(form.a4DocumentType).label.toLowerCase()} for ${project.manifest.name}…` : form.posterMode === 'ai_creative' ? 'Describe the poster, audience and strongest message…' : 'Describe the video story and business goal…'} /><div><span>{aiReady ? 'Codex connected' : 'Connection required'}</span><button disabled={project.manifest.readOnly || generating || !codexRequest.trim()}>{generating ? 'Working…' : 'Send ↑'}</button></div></form>{project.manifest.readOnly && <p className="creative-example-note">Koya is a locked example. Duplicate it into a customer project to create new outputs.</p>}</section>

      <section className="creative-preview-column"><div className="creative-preview-head"><div><p className="eyebrow">Live preview</p><h2>{selected?.title ?? 'Nothing selected'}</h2></div>{selected && <span className={`workflow-pill ${isAiPoster ? 'ai-poster' : selected.kind}`}>{selected.workflow === 'hyperframes' ? 'Video package' : selected.workflow === 'image_model' ? 'Image Model' : 'A4 · 100% height'}</span>}</div>{selected ? selected.kind === 'poster' && !isAiPoster && !draft ? <section className="a4-reader creative-a4-preview"><div className="a4-reader-toolbar"><div><strong>{selectedA4Meta.label}</strong><span>{selectedA4Meta.description}</span></div><div className="a4-page-controls"><button aria-label="Previous A4 document" disabled={activeA4Index <= 0} onClick={() => turnA4(-1)}>‹</button><b>{activeA4Index + 1} / {Math.max(a4Briefs.length, 1)}</b><button aria-label="Next A4 document" disabled={activeA4Index < 0 || activeA4Index >= a4Briefs.length - 1} onClick={() => turnA4(1)}>›</button></div><div className="a4-reader-actions"><span>100% height</span><button className="secondary-button" disabled={exporting || !a4Html} onClick={exportPdf}>{exporting ? 'Exporting…' : 'PDF'}</button></div></div><div className="a4-reader-viewport" ref={a4ViewportRef}><div className="a4-reader-page" style={{ width: 794 * a4PreviewScale, minWidth: 794 * a4PreviewScale, height: 1123 * a4PreviewScale }}>{a4Html ? <iframe srcDoc={a4Html} title={`${selected.title} A4 preview`} style={{ transform: `scale(${a4PreviewScale})` }} /> : <div className="a4-preview-loading">Preparing A4 page…</div>}</div></div></section> : selected.kind === 'poster' ? <div className="creative-poster-preview-wrap">{isAiPoster ? <div className="ai-poster-preview" style={{ background: `linear-gradient(145deg, ${project.manifest.brand.primary}, ${project.manifest.brand.accent})` }}><small>IMAGE MODEL MAIN VISUAL + RELIABLE TEXT OVERLAY</small><span>✦</span><h3>{selected.headline}</h3><p>{selected.subheadline}</p><ul>{selected.projectHighlights.map((highlight) => <li key={highlight}>{highlight}</li>)}</ul><b>{selected.status === 'awaiting_generation' ? 'Main visual awaiting generation' : 'AI brief · review before generation'}</b></div> : <div className="poster-draft-preview a4-preview" style={{ background: project.manifest.brand.primary }}><h3>{selected.headline}</h3><p>{selected.subheadline}</p></div>}</div> : <div className="creative-video-preview"><div className="video-frame" style={{ background: project.manifest.brand.primary }}><span>9:16 STORYBOARD</span><strong>{selected.headline}</strong><small>{selected.durationSeconds}s · MP4 not rendered</small></div><div className="scene-list">{selected.scenes.map((scene) => <article key={scene.order}><span>{String(scene.order).padStart(2, '0')}</span><div><strong>{scene.title}</strong><p>{scene.onScreenText}</p></div><b>{scene.durationSeconds}s</b></article>)}</div></div> : <div className="creative-empty-stage"><span>✦</span><strong>Choose a creative type</strong><p>The preview appears here.</p></div>}</section>
    </div>
    <details className="creative-boundary"><summary>Output and approval boundaries</summary><span>A4 uses one Markdown source of truth, a derived HTML preview and native PDF export. AI Creative Poster saves a reviewed one-image request package; it is not generated until an Image Model connection returns an actual asset. Video remains a HyperFrames authoring package, not an MP4.</span></details>
    {creating && <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && setCreating(false)}><form className="new-project-dialog creative-dialog" onSubmit={generate}><div className="dialog-heading"><div><p className="eyebrow">Codex-style creation flow</p><h2>New creative brief</h2></div><button type="button" className="icon-button" onClick={() => setCreating(false)}>×</button></div><p className="dialog-copy">Say what you need in plain language. For AI Creative Poster, Codex summarises verified project highlights, writes the copy and prepares the Image Model main visual. The traditional form remains available.</p>
      <div className="creative-input-mode"><button type="button" className={creationMode === 'codex' ? 'active' : ''} onClick={() => setCreationMode('codex')}>✦ Ask Codex</button><button type="button" className={creationMode === 'traditional' ? 'active' : ''} onClick={() => setCreationMode('traditional')}>Traditional form</button></div>
      <div className="creative-type-switch"><button type="button" className={form.kind === 'poster' && form.posterMode === 'a4_sales_sheet' ? 'active' : ''} onClick={() => setForm((current) => ({ ...current, kind: 'poster', posterMode: 'a4_sales_sheet', a4DocumentType: current.a4DocumentType ?? 'unit_sales_sheet', format: 'a4_portrait' }))}><span>A4</span><strong>Property documents</strong><small>Sales sheets · lists · packs</small></button><button type="button" className={form.posterMode === 'ai_creative' ? 'active ai' : ''} onClick={() => setForm((current) => ({ ...current, kind: 'poster', posterMode: 'ai_creative', format: 'instagram_portrait' }))}><span>✦</span><strong>AI Creative Poster</strong><small>Image Model main visual</small></button><button type="button" className={form.kind === 'video' ? 'active' : ''} onClick={() => setForm((current) => ({ ...current, kind: 'video', posterMode: undefined, format: 'social_vertical' }))}><span>▶</span><strong>Video</strong><small>Storyboard + HyperFrames package</small></button></div>
      {form.posterMode === 'a4_sales_sheet' && <section className="a4-template-picker"><div><p className="eyebrow">What does the sales team need?</p><strong>{a4DocumentMeta(form.a4DocumentType).label}</strong><span>{a4DocumentMeta(form.a4DocumentType).description}</span></div><div>{a4DocumentTypes.map((item) => <button type="button" key={item.id} className={form.a4DocumentType === item.id ? 'active' : ''} onClick={() => setForm((current) => ({ ...current, a4DocumentType: item.id }))}><strong>{item.label}</strong><small>{item.requirement}</small></button>)}</div></section>}
      {creationMode === 'codex' ? <div className="codex-creative-composer"><div className="codex-context-line"><span>Working context</span><strong>{project.manifest.name}</strong><b>{project.manifest.units.length} unit types</b><b>{project.manifest.assets.filter((asset) => asset.status === 'accepted').length} accepted sources</b></div><label>Tell Codex what to create<textarea autoFocus required rows={6} value={codexRequest} onChange={(event) => setCodexRequest(event.target.value)} placeholder={form.posterMode === 'a4_sales_sheet' ? `Create a ${a4DocumentMeta(form.a4DocumentType).label.toLowerCase()} for ${project.manifest.name}. Use only verified project and unit facts; mark missing sales data unavailable.` : form.posterMode === 'ai_creative' ? `Make a premium launch poster for ${project.manifest.name}. Summarise the strongest verified project highlights, write a short headline and CTA, then prepare an Image Model visual for Instagram.` : `Create a 15-second vertical launch video for ${project.manifest.name}.`} /></label><p>You can simply say: “突出大户型、MPR 和 360° 看房，整体高级一点。” Codex reads the project first; you review every claim and visual instruction before generation.</p></div> : <><label>Campaign name<input required value={form.campaignName} onChange={(event) => setForm((current) => ({ ...current, campaignName: event.target.value }))} placeholder="Apartment 106 launch" /></label><div className="form-grid"><label>Unit type<select value={form.unitId ?? ''} onChange={(event) => setForm((current) => ({ ...current, unitId: event.target.value || undefined }))}><option value="">Whole project</option>{project.manifest.units.map((unit) => <option value={unit.id} key={unit.id}>{unit.label}</option>)}</select></label><label>Format<select value={form.format} onChange={(event) => setForm((current) => ({ ...current, format: event.target.value as CreativeBrief['format'] }))}>{form.kind === 'video' ? <><option value="social_vertical">Vertical · 15s</option><option value="social_square">Square · 15s</option><option value="web_landscape">Landscape · 30s</option></> : form.posterMode === 'ai_creative' ? <><option value="instagram_portrait">Instagram · 1080 × 1350</option><option value="story_portrait">Story · 1080 × 1920</option></> : <option value="a4_portrait">A4 print · 2480 × 3508</option>}</select></label></div><label>Audience<input required value={form.audience} onChange={(event) => setForm((current) => ({ ...current, audience: event.target.value }))} /></label><label>Objective<textarea required rows={3} value={form.objective} onChange={(event) => setForm((current) => ({ ...current, objective: event.target.value }))} /></label></>}
      <div className={`creative-source-note ${form.posterMode === 'ai_creative' ? 'ai' : ''}`}><strong>{project.manifest.assets.filter((asset) => asset.status === 'accepted').length} accepted source assets available</strong><span>{form.posterMode === 'ai_creative' ? 'Codex will prepare one evidence-grounded Image Model prompt. Saving creates a reviewed request package only; provider price remains unavailable until the managed image connection is configured.' : form.kind === 'poster' ? 'A4 saves Markdown as the source of truth, renders an HTML preview and exports a native PDF.' : 'Video saves a structured storyboard package. Rendering remains a separate production step.'}</span></div><div className="dialog-actions"><button type="button" className="secondary-button" onClick={() => setCreating(false)}>Cancel</button><button className="primary-button" disabled={generating}>{generating ? 'Codex is drafting…' : aiReady ? creationMode === 'codex' ? 'Send to Codex' : 'Generate draft with Codex' : 'Connect Codex'}</button></div></form></div>}
  </main>;
}

function SkillsMarketplace({ projects, project, onSelectProject, onUpdated }: { projects: ProjectRecord[]; project?: ProjectRecord; onSelectProject: (project: ProjectRecord) => void; onUpdated: (project: ProjectRecord) => void }) {
  const [skills, setSkills] = useState<MarketplaceSkill[]>([]);
  const [query, setQuery] = useState('');
  const [category, setCategory] = useState('All');
  const [busy, setBusy] = useState('');
  const [error, setError] = useState('');

  async function refresh() { try { setSkills(await listSkillMarketplace()); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } }
  useEffect(() => { void refresh(); }, []);
  const categories = ['All', ...Array.from(new Set(skills.map((skill) => skill.category)))];
  const filtered = skills.filter((skill) => (category === 'All' || skill.category === category) && (!query.trim() || `${skill.name} ${skill.description} ${skill.category}`.toLowerCase().includes(query.trim().toLowerCase())));
  const enabled = project?.manifest.enabledSkillIds ?? [];

  async function install(skill: MarketplaceSkill) {
    setBusy(skill.id); setError('');
    try { setSkills(await installMarketplaceSkill(skill.id)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setBusy(''); }
  }
  async function toggle(skill: MarketplaceSkill) {
    if (!project || project.manifest.readOnly) return;
    setBusy(skill.id); setError('');
    try { onUpdated(await setProjectSkillEnabled(project.manifest.projectId, skill.id, !enabled.includes(skill.id))); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setBusy(''); }
  }

  return <main className="detail-view skills-marketplace"><header className="page-header"><div><p className="eyebrow">Local capability library</p><h1>Skills Marketplace</h1><p>Install approved property workflows, then enable only the capabilities each project needs.</p></div><button className="secondary-button" onClick={refresh}>↻ Refresh library</button></header>
    <section className="skills-overview"><div><span>Marketplace</span><strong>{skills.length}</strong><small>Approved Estate Studio Skills</small></div><div><span>Installed</span><strong>{skills.filter((skill) => skill.installed).length}</strong><small>Real SKILL.md packages on this Mac</small></div><div><span>Active context</span><strong>{enabled.length}</strong><small>{project ? `Enabled for ${project.manifest.name}` : 'Select a project'}</small></div></section>
    <section className="skills-project-bar"><div><span>Project context</span><strong>{project?.manifest.name ?? 'No project selected'}</strong><small>{project?.manifest.readOnly ? 'Example project is read-only' : project ? 'Enabled Skills load into this project’s Codex context.' : 'Choose where installed Skills should be enabled.'}</small></div><select value={project?.manifest.projectId ?? ''} onChange={(event) => { const match = projects.find((item) => item.manifest.projectId === event.target.value); if (match) onSelectProject(match); }}><option value="">Select project</option>{projects.map((item) => <option value={item.manifest.projectId} key={item.manifest.projectId}>{item.manifest.name}{item.manifest.readOnly ? ' · Example' : ''}</option>)}</select></section>
    {error && <div className="page-error">Skills Marketplace: {error}</div>}
    <div className="skills-toolbar"><div className="skill-categories">{categories.map((item) => <button className={category === item ? 'active' : ''} key={item} onClick={() => setCategory(item)}>{item}</button>)}</div><label>⌕<input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Search Skills" /></label></div>
    <div className="marketplace-grid">{filtered.map((skill) => { const active = enabled.includes(skill.id); return <article className={`marketplace-card ${active ? 'enabled' : ''}`} key={skill.id}><div className="marketplace-card-top"><span className="marketplace-icon"><NavGlyph name="skills" /></span><div><small>{skill.category}</small><h2>{skill.name}</h2></div><span className={`install-state ${skill.installed ? 'installed' : ''}`}>{skill.installed ? 'Installed' : 'Available'}</span></div><p>{skill.description}</p><div className="capability-chips">{skill.capabilities.map((item) => <span key={item}>{item}</span>)}</div><div className="skill-package-meta"><span>v{skill.version}</span><span>{skill.author}</span><span>SKILL.md</span></div><footer>{!skill.installed ? <button className="primary-button" disabled={busy === skill.id} onClick={() => install(skill)}>{busy === skill.id ? 'Installing…' : 'Install Skill'}</button> : <><span className="installed-proof">✓ Installed locally</span><button className={active ? 'skill-toggle active' : 'skill-toggle'} disabled={!project || project.manifest.readOnly || busy === skill.id} onClick={() => toggle(skill)}><i />{active ? 'Enabled for project' : 'Enable for project'}</button></>}</footer></article>; })}</div>
    <div className="skills-safety"><strong>Trusted installation boundary</strong><span>Marketplace installation can write only catalogued packages to Estate Studio’s private Skills directory. Project activation stores only the Skill ID; Codex loads the installed SKILL.md at request time. Arbitrary folders, duplicate names and external scripts are not accepted.</span></div>
  </main>;
}

function JobsWorkspace({ projects, onOpen, onUpdated }: { projects: ProjectRecord[]; onOpen: (project: ProjectRecord, unit: UnitRecord) => void; onUpdated: (project: ProjectRecord) => void }) {
  const [busy, setBusy] = useState('');
  const [error, setError] = useState('');
  const [outputPreviews, setOutputPreviews] = useState<Record<string, string>>({});
  const [capabilities, setCapabilities] = useState<Awaited<ReturnType<typeof listGenerationCapabilities>>>({ schemaVersion: 1, records: [], updatedAt: 0 });
  useEffect(() => { listGenerationCapabilities().then(setCapabilities).catch((cause) => setError(cause instanceof Error ? cause.message : String(cause))); }, []);
  const jobs = projects.flatMap((project) => project.manifest.generationJobs.map((job) => ({ project, job, unit: project.manifest.units.find((unit) => unit.id === job.unitId) }))).sort((left, right) => right.job.createdAt - left.job.createdAt);
  const backgroundJobs = projects.flatMap((project) => (project.manifest.backgroundJobs ?? []).map((job) => ({ project, job }))).sort((left, right) => right.job.createdAt - left.job.createdAt);
  const blocked = jobs.filter(({ job }) => job.status === 'blocked_capability').length;
  const approval = jobs.filter(({ job }) => job.status === 'awaiting_approval').length;
  const complete = jobs.filter(({ job }) => job.status === 'completed').length;
  async function action(key: string, run: () => Promise<ProjectRecord>) { setBusy(key); setError(''); try { onUpdated(await run()); setCapabilities(await listGenerationCapabilities()); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setBusy(''); } }
  async function ingest(project: ProjectRecord, job: ProjectRecord['manifest']['generationJobs'][number]) {
    const sourcePath = await openFileDialog({ multiple: false, directory: false, filters: [{ name: 'Generated image', extensions: ['png', 'jpg', 'jpeg', 'webp'] }] });
    if (!sourcePath || Array.isArray(sourcePath)) return;
    await action(`${project.manifest.projectId}-${job.id}-ingest`, () => ingestGenerationOutput(project.manifest.projectId, job.id, { sourcePath, providerRequestId: job.providerTaskId ?? '', requestFingerprint: job.approvalFingerprint ?? '' }));
  }
  async function preview(projectId: string, jobId: string, outputId: string) {
    const key = `${projectId}-${jobId}-${outputId}`; setBusy(key); setError('');
    try { const dataUrl = await getGenerationOutputDataUrl(projectId, jobId, outputId); setOutputPreviews((current) => ({ ...current, [key]: dataUrl })); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
    finally { setBusy(''); }
  }
  async function decide(projectId: string, jobId: string, outputId: string, decision: 'accepted' | 'rejected') {
    let reason = ''; let correctionInstruction = '';
    if (decision === 'rejected') {
      reason = window.prompt('Why is this output rejected? This becomes part of the audit record.')?.trim() ?? '';
      if (!reason) return;
      correctionInstruction = window.prompt('Optional targeted correction. This creates a draft only and does not submit another paid task.')?.trim() ?? '';
    }
    await action(`${projectId}-${jobId}-${outputId}`, () => reviewGenerationOutput(projectId, jobId, outputId, { decision, reason, correctionInstruction }));
  }
  async function runQa(project: ProjectRecord, job: ProjectRecord['manifest']['generationJobs'][number], outputId: string) {
    const topologyPassed = window.confirm(`Topology check: does this panorama preserve every confirmed route for ${job.roomName}?`);
    const horizonPassed = window.confirm('Horizon check: is the horizon level throughout the 360° view?');
    const seamPassed = window.confirm('Seam check: after rotating through the full panorama, is the wrap seam visually acceptable?');
    const orientationPassed = window.confirm('Orientation check: does the starting view face the intended direction?');
    const runtimePassed = window.confirm('Runtime check: did this exact panorama open, rotate and return to its start correctly in the viewer?');
    const yawValue = window.prompt('Starting orientation in degrees (-180 to 180).', '0');
    if (yawValue === null) return;
    const yawDegrees = Number(yawValue);
    const usagePermissionReference = window.prompt('Enter the output usage-rights or subscription-terms reference required for publication.')?.trim() ?? '';
    if (!usagePermissionReference) return;
    await action(`${project.manifest.projectId}-${job.id}-${outputId}-qa`, () => recordPanoramaQa(project.manifest.projectId, job.id, outputId, { topologyPassed, confirmedOpeningIds: topologyPassed ? (job.requiredOpeningIds ?? []) : [], horizonPassed, seamPassed, orientationPassed, yawDegrees, runtimePassed, usagePermissionReference }));
  }
  async function planFallback(project: ProjectRecord, job: ProjectRecord['manifest']['generationJobs'][number], outputId: string) {
    const modeValue = window.prompt('Fallback mode: enter cubefaces or overlapping_tiles.', 'cubefaces');
    if (modeValue !== 'cubefaces' && modeValue !== 'overlapping_tiles') return;
    const reason = window.prompt('Reason: one_shot_projection_failed, one_shot_topology_failed, one_shot_seam_failed, one_shot_dimension_failed, or webgl_runtime_failed.', 'one_shot_seam_failed')?.trim() ?? '';
    if (!reason) return;
    await action(`${project.manifest.projectId}-${job.id}-${outputId}-fallback`, () => createPanoramaFallbackPlan(project.manifest.projectId, job.id, outputId, { mode: modeValue, reason }));
  }
  async function chooseStillFallback(project: ProjectRecord, job: ProjectRecord['manifest']['generationJobs'][number]) {
    const eligible = project.manifest.assets.filter((asset) => asset.status === 'accepted' && ['image/png', 'image/jpeg', 'image/webp'].includes(asset.mimeType) && (!asset.unitId || asset.unitId === job.unitId));
    const assetId = window.prompt(`Accepted still asset ID for ${job.roomName}:\n${eligible.map((asset) => `${asset.id} — ${asset.name}`).join('\n')}`)?.trim() ?? '';
    if (!assetId) return;
    await action(`${project.manifest.projectId}-${job.id}-still`, () => assignRoomStillFallback(project.manifest.projectId, job.unitId, job.roomId, assetId));
  }
  return <main className="detail-view jobs-view"><header className="page-header"><div><p className="eyebrow">Local production queue</p><h1>Production Queue</h1><p>Every generation package remains tied to its project, evidence and approval state.</p></div></header>
    {error && <div className="page-error">Generation job: {error}</div>}
    <div className={`capability-registry-status ${capabilities.records.length ? 'available' : 'blocked'}`}><strong>Capability registry</strong><span>{capabilities.records.length ? `${capabilities.records.length} provider/model record(s) · every package still requires an exact size, mode, price, quota and expiry check` : 'No verified provider/model records · model access, exact dimensions, panorama mode, current price and quota remain unavailable'}</span></div>
    <section className="production-metrics jobs-metrics"><div><span>All packages</span><strong>{jobs.length}</strong><small>Across {projects.length} projects</small></div><div><span>Capability blocked</span><strong>{blocked}</strong><small>No provider submission</small></div><div><span>Awaiting approval</span><strong>{approval}</strong><small>Human decision required</small></div><div><span>Completed</span><strong>{complete}</strong><small>Still requires visual QA</small></div></section>
    <section className="section-block"><div className="section-heading"><div><p className="eyebrow">Audit queue</p><h2>Generation packages</h2></div><span>Draft is not submitted; provider success is not visual acceptance.</span></div><div className="global-job-list">{jobs.length === 0 ? <div className="matrix-empty jobs-empty"><span className="empty-state-icon"><NavGlyph name="jobs" /></span><strong>No generation packages yet</strong><span>Open a project unit, lock an identity anchor, then prepare its first panorama package.</span><small>Project → Unit → Room evidence → Prepare package</small></div> : jobs.map(({ project, job, unit }) => { const key = `${project.manifest.projectId}-${job.id}`; const cancellable = ['draft', 'blocked_capability', 'awaiting_approval', 'approved', 'queued', 'submitted', 'processing'].includes(job.status); const capability = capabilities.records.find((record) => record.supportedSizes.some((size) => size.width === job.requestedWidth && size.height === job.requestedHeight) && record.panoramaModes.includes(job.panoramaMode)); return <div className="global-job-row" key={key}><span className="job-icon"><NavGlyph name="jobs" /></span><div><small>{project.manifest.name} · {unit?.label ?? job.unitId}</small><strong>{job.roomName} · 360° panorama</strong><span>Attempt {job.attempt ?? 1} · {job.progressPercent ?? 0}% · {job.outputCount} output</span>{job.failureReason && <span title={job.failureReason}>{job.failureReason}</span>}</div><div><small>Idempotency</small><strong>{job.idempotencyKey ? `${job.idempotencyKey.slice(0, 10)}…` : 'Unavailable'}</strong></div><div><small>Price</small><strong>{job.priceStatus}</strong></div><span className={`job-status ${job.status}`}>{job.status.replaceAll('_', ' ')}</span><div className="job-row-actions"><button disabled={!unit} onClick={() => unit && onOpen(project, unit)}>Open</button>{job.status === 'blocked_capability' && <button disabled={busy === key} onClick={() => action(key, async () => (await checkManagedGatewayCapability(project.manifest.projectId, job.id)).project)}>Check subscription</button>}{job.status === 'blocked_capability' && <button disabled={busy === key} onClick={() => action(key, async () => (await inspectCodexGenerationAdapter(project.manifest.projectId, job.id)).project)}>Inspect Codex</button>}{job.status === 'blocked_capability' && capability && <button disabled={busy === key} onClick={() => action(key, () => applyGenerationCapability(project.manifest.projectId, job.id, capability.providerChoice, capability.modelId))}>Check capability</button>}{job.status === 'awaiting_approval' && <button disabled={busy === key} onClick={() => action(key, () => approveGenerationJob(project.manifest.projectId, job.id, job.approvalFingerprint ?? ''))}>Approve</button>}{job.retryAllowed && <button disabled={busy === key} onClick={() => action(key, () => retryGenerationJob(project.manifest.projectId, job.id))}>Retry</button>}{cancellable && <button className="danger" disabled={busy === key} onClick={() => action(key, () => cancelGenerationJob(project.manifest.projectId, job.id))}>Cancel</button>}</div></div>; })}</div></section>
    {jobs.some(({ job }) => job.status === 'approved') && <section className="section-block approved-submit-section"><div className="section-heading"><div><p className="eyebrow">Final paid gate</p><h2>Approved managed tasks</h2></div><span>One approval authorises one task and one output only.</span></div><div className="approved-submit-list">{jobs.filter(({ job }) => job.status === 'approved').map(({ project, job }) => { const key = `${project.manifest.projectId}-${job.id}-submit`; return <article key={key}><div><small>{project.manifest.name} · {job.roomName}</small><strong>1 × 2:1 panorama · {job.customerCreditCost ?? 'unavailable'} credits</strong><span>{job.priceCurrency ?? 'Currency unavailable'} {job.priceAmountMinor !== undefined ? (job.priceAmountMinor / 100).toFixed(2) : 'unavailable'} · fingerprint {job.approvalFingerprint?.slice(0, 12)}…</span></div><button className="primary-button" disabled={busy === key || job.providerChoice !== 'managed_openai'} onClick={() => action(key, () => submitManagedPanorama(project.manifest.projectId, job.id, job.approvalFingerprint ?? ''))}>{busy === key ? 'Submitting one task…' : 'Submit one approved output'}</button></article>; })}</div></section>}
    <section className="section-block output-review-section"><div className="section-heading"><div><p className="eyebrow">Visual review</p><h2>Generated output versions</h2></div><span>Accepting a visual keeps publication blocked until panorama QA passes.</span></div><div className="output-review-list">{jobs.filter(({ job }) => job.status === 'completed' || (job.outputs?.length ?? 0) > 0).map(({ project, job }) => <article className="output-review-job" key={`outputs-${project.manifest.projectId}-${job.id}`}><header><div><small>{project.manifest.name}</small><strong>{job.roomName}</strong><span>{job.outputs?.length ?? 0} ingested version(s) · {job.correctionRequests?.length ?? 0} correction draft(s)</span></div>{job.status === 'completed' && <button className="secondary-button" disabled={busy.includes(job.id)} onClick={() => ingest(project, job)}>Import provider output</button>}</header><div className="output-version-grid">{(job.outputs ?? []).map((output) => { const key = `${project.manifest.projectId}-${job.id}-${output.id}`; return <div className={`output-version-card ${output.status}`} key={output.id}>{outputPreviews[key] ? <img src={outputPreviews[key]} alt={`${job.roomName} generated version ${output.version}`} /> : <button className="output-preview-placeholder" disabled={busy === key} onClick={() => preview(project.manifest.projectId, job.id, output.id)}>{busy === key ? 'Loading…' : `Preview v${output.version}`}</button>}<div><strong>Version {output.version}</strong><span>{output.width} × {output.height} · attempt {output.sourceAttempt}</span><small>{output.checksumSha256.slice(0, 12)}… · {output.status.replaceAll('_', ' ')}</small><small>Publishability: {output.publishability.replaceAll('_', ' ')}</small>{output.rejectionReason && <p>{output.rejectionReason}</p>}</div>{output.status === 'pending_review' && <footer><button onClick={() => decide(project.manifest.projectId, job.id, output.id, 'accepted')}>Accept visual</button><button className="danger" onClick={() => decide(project.manifest.projectId, job.id, output.id, 'rejected')}>Reject / correct</button></footer>}</div>; })}</div>{(job.outputs?.length ?? 0) === 0 && <p className="output-empty">The provider attempt is complete, but no visual file has been ingested. Completion alone is not acceptance.</p>}</article>)}</div></section>
    {jobs.some(({ job }) => (job.outputs ?? []).some((output) => output.status === 'accepted' && output.publishability !== 'publishable')) && <section className="section-block panorama-qa-section"><div className="section-heading"><div><p className="eyebrow">Panorama QA gate</p><h2>Accepted visuals awaiting runtime QA</h2></div><span>System checks and human review are recorded separately.</span></div><div className="approved-submit-list">{jobs.flatMap(({ project, job }) => (job.outputs ?? []).filter((output) => output.status === 'accepted' && output.publishability !== 'publishable').map((output) => ({ project, job, output }))).map(({ project, job, output }) => { const key = `${project.manifest.projectId}-${job.id}-${output.id}-qa`; return <article key={key}><div><small>{project.manifest.name} · {job.roomName} · v{output.version}</small><strong>{output.width} × {output.height} · {output.publishability.replaceAll('_', ' ')}</strong><span>{output.panoramaQa ? `Last QA: ${output.panoramaQa.overallStatus} · seam metric ${output.panoramaQa.seamEdgeDelta.toFixed(6)}` : 'No panorama QA record yet'}</span></div><button className="primary-button" disabled={busy === key} onClick={() => runQa(project, job, output.id)}>{busy === key ? 'Recording QA…' : output.panoramaQa ? 'Run QA again' : 'Run panorama QA'}</button></article>; })}</div></section>}
    {jobs.some(({ job }) => (job.outputs ?? []).some((output) => output.publishability === 'publishable' && (output.derivatives?.length ?? 0) === 0)) && <section className="section-block derivative-section"><div className="section-heading"><div><p className="eyebrow">Delivery processing</p><h2>Panorama derivative sets</h2></div><span>Accepted 2:1 sources only; square images are never stretched.</span></div><div className="approved-submit-list">{jobs.flatMap(({ project, job }) => (job.outputs ?? []).filter((output) => output.publishability === 'publishable' && (output.derivatives?.length ?? 0) === 0).map((output) => ({ project, job, output }))).map(({ project, job, output }) => { const key = `${project.manifest.projectId}-${job.id}-${output.id}-derivatives`; return <article key={key}><div><small>{project.manifest.name} · {job.roomName} · v{output.version}</small><strong>Seam-repaired master + mobile + 4K + 8K</strong><span>Upscaling is recorded as deterministic resampling with no claim of recovered detail.</span></div><button className="primary-button" disabled={busy === key} onClick={() => action(key, () => buildPanoramaDerivatives(project.manifest.projectId, job.id, output.id))}>{busy === key ? 'Processing locally…' : 'Build delivery set'}</button></article>; })}</div></section>}
    {jobs.some(({ job }) => (job.outputs ?? []).some((output) => output.status === 'rejected' || output.panoramaQa?.overallStatus === 'failed')) && <section className="section-block fallback-section"><div className="section-heading"><div><p className="eyebrow">Continuity fallback</p><h2>Failed one-shot recovery</h2></div><span>Fallback segments remain individually unapproved; nothing is resubmitted automatically.</span></div><div className="approved-submit-list">{jobs.flatMap(({ project, job }) => (job.outputs ?? []).filter((output) => output.status === 'rejected' || output.panoramaQa?.overallStatus === 'failed').map((output) => ({ project, job, output }))).map(({ project, job, output }) => { const key = `${project.manifest.projectId}-${job.id}-${output.id}-fallback`; const plans = (job.fallbackPlans ?? []).filter((plan) => plan.parentOutputId === output.id); return <article key={key}><div><small>{project.manifest.name} · {job.roomName} · v{output.version}</small><strong>{plans.length ? `${plans.length} unapproved fallback plan(s)` : 'No continuity fallback plan'}</strong><span>{plans.length ? plans.map((plan) => `${plan.mode}: ${plan.tasks.length} separately gated task(s)`).join(' · ') : 'Create cubefaces or overlapping tiles only after the failed one-shot is recorded.'}</span></div><div className="fallback-actions"><button disabled={busy === key} onClick={() => planFallback(project, job, output.id)}>Plan fallback</button><button disabled={busy.endsWith('-still')} onClick={() => chooseStillFallback(project, job)}>Choose still fallback</button></div></article>; })}</div></section>}
    {backgroundJobs.length > 0 && <section className="section-block background-queue-section"><div className="section-heading"><div><p className="eyebrow">Persistent workers</p><h2>Background processing</h2></div><span>Checkpoints and logical idempotency survive restart.</span></div><div className="global-job-list">{backgroundJobs.map(({ project, job }) => { const key = `${project.manifest.projectId}-${job.id}-background`; return <div className="global-job-row" key={key}><span className="job-icon"><NavGlyph name="jobs" /></span><div><small>{project.manifest.name} · {job.kind.replaceAll('_', ' ')}</small><strong>{job.subjectId}</strong><span>Attempt {job.attempt} · checkpoint {job.checkpoint}</span>{job.failureReason && <span>{job.failureReason}</span>}</div><div><small>Progress</small><strong>{job.progressPercent}%</strong></div><div><small>Idempotency</small><strong>{job.idempotencyKey.slice(0, 9)}…</strong></div><span className={`job-status ${job.status}`}>{job.status}</span><div className="job-row-actions">{job.kind === 'panorama_processing' && job.status === 'queued' && <button disabled={busy === key} onClick={() => action(key, () => runPanoramaBackgroundJob(project.manifest.projectId, job.id))}>Resume</button>}{job.status === 'failed' && <button disabled={busy === key} onClick={() => action(key, () => retryBackgroundJob(project.manifest.projectId, job.id))}>Retry</button>}</div></div>; })}</div></section>}
    <div className="jobs-policy"><strong>Fail-closed provider policy</strong><span>An unavailable capability or price cannot become zero, approved or submitted. Retrying a future paid task must reuse its idempotency key.</span></div>
  </main>;
}

function DeploymentReadbackControl({ project, release, onUpdated }: { project: ProjectRecord; release: ProjectRecord['manifest']['releases'][number]; onUpdated: (project: ProjectRecord) => void }) {
  const [url, setUrl] = useState('');
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState('');
  const [checks, setChecks] = useState<string[]>([]);
  const [shareLinks, setShareLinks] = useState<Array<{ unitId: string; url: string }>>([]);
  async function verify() { setBusy(true); setError(''); setChecks([]); try { const result = await verifyDeploymentReadback(project.manifest.projectId, release.id, url, release.accessMode as 'public' | 'unlisted' | 'private'); onUpdated(result.project); setChecks(result.checks); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setBusy(false); } }
  async function finalize() { setBusy(true); setError(''); try { const result = await finalizeVerifiedRelease(project.manifest.projectId, release.id, url, release.accessMode as 'public' | 'unlisted' | 'private'); onUpdated(result.project); setShareLinks(result.shareLinks); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setBusy(false); } }
  const readyToFinalize = release.status === 'readback_passed';
  return <article className="readback-control"><div><small>{project.manifest.name} · v{release.version} · {release.accessMode}</small><strong>{readyToFinalize ? 'Finalize verified release' : 'Logged-out deployment read-back'}</strong><span>The URL is fetched without cookies. Private-link query values are used in memory and are not written to the project.</span></div><div className="readback-form"><input type={release.accessMode === 'private' ? 'password' : 'url'} value={url} onChange={(event) => setUrl(event.target.value)} placeholder="https://customer-owned.example/tour/" aria-label="Deployed tour URL" /><button disabled={busy || !url.trim()} onClick={readyToFinalize ? finalize : verify}>{busy ? 'Checking…' : readyToFinalize ? 'Recheck + finalize' : 'Verify actual URL'}</button></div>{error && <p className="readback-error">{error}</p>}{checks.length > 0 && !readyToFinalize && <p className="readback-pass">Passed {checks.length} checks · final release verification remains separate.</p>}{shareLinks.length > 0 && <div className="share-link-results">{shareLinks.map((link) => <span key={link.unitId}><strong>{link.unitId}</strong><code>{link.url}</code></span>)}</div>}</article>;
}

function DeploymentsWorkspace({ projects, onUpdated, onPreview }: { projects: ProjectRecord[]; onUpdated: (project: ProjectRecord) => void; onPreview: (project: ProjectRecord) => void }) {
  const [savingId, setSavingId] = useState(''); const [error, setError] = useState('');
  const [accessMode, setAccessMode] = useState<'public' | 'unlisted' | 'private'>('unlisted');
  async function buildPreview(project: ProjectRecord) { setSavingId(`${project.manifest.projectId}-preview`); setError(''); try { const updated = await buildStaticTourPreview(project.manifest.projectId); onUpdated(updated); onPreview(updated); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSavingId(''); } }
  async function build(project: ProjectRecord) { setSavingId(project.manifest.projectId); setError(''); try { onUpdated(await createLocalRelease(project.manifest.projectId)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSavingId(''); } }
  async function chooseDestination(project: ProjectRecord, releaseId: string) { const parent = await openFileDialog({ multiple: false, directory: true, title: 'Choose a customer-owned destination folder' }); if (typeof parent !== 'string') return; return `${parent.replace(/[\\/]$/, '')}/${project.manifest.projectId}-${releaseId}`; }
  async function exportRelease(project: ProjectRecord, releaseId: string) { const destination = await chooseDestination(project, releaseId); if (!destination) return; setSavingId(`${project.manifest.projectId}-export`); setError(''); try { await exportStaticRelease(project.manifest.projectId, releaseId, destination); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSavingId(''); } }
  async function uploadRelease(project: ProjectRecord, releaseId: string) { const destination = await chooseDestination(project, releaseId); if (!destination) return; setSavingId(`${project.manifest.projectId}-upload`); setError(''); try { onUpdated(await publishReleaseToCustomerDirectory(project.manifest.projectId, releaseId, destination, accessMode)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } finally { setSavingId(''); } }
  return <main className="detail-view deployments-view"><header className="page-header"><div><p className="eyebrow">Release control plane</p><h1>Deployments</h1><p>Local preview, public deployment and verified read-back remain separate evidence states.</p></div></header>
    {error && <div className="page-error deployment-error">Deployment action failed: {error}</div>}
    <div className="deployment-grid">{projects.map((project) => { const releases = project.manifest.releases ?? []; const latest = releases.slice().sort((a, b) => b.createdAt - a.createdAt)[0]; const locallyReady = project.manifest.units.some((unit) => unit.tourAvailable && unit.roomGraphLocked && unit.rooms.every((room) => room.panoramaStatus === 'ready' && Boolean(room.panoramaAssetId))); return <article className="deployment-card" key={project.manifest.projectId}><div className="deployment-card-head"><div><span>{project.manifest.company}</span><h2>{project.manifest.name}</h2></div><span className={`release-state ${latest?.status ?? 'none'}`}>{latest?.status.replaceAll('_', ' ') ?? 'No release'}</span></div><div className="deployment-facts"><div><span>Latest version</span><strong>{latest ? `v${latest.version}` : 'Unavailable'}</strong></div><div><span>Access</span><strong>{latest?.accessMode ?? 'Unavailable'}</strong></div><div><span>Units</span><strong>{latest?.unitIds.join(', ') || 'Unavailable'}</strong></div></div><p>{latest?.status === 'public_verified' ? 'Public URL has a recorded verified release state.' : latest?.status === 'uploaded_unverified' ? 'Files were copied to a customer-owned directory; URL and access remain unverified.' : latest?.status === 'preview_ready' ? 'Local snapshot exists; no public access has been claimed.' : project.manifest.tourPreviewUrl ? 'Validated local URL preview exists; it is neither exported nor published.' : 'Build a local URL preview after every included unit passes the tour gates.'}</p>{latest && !project.manifest.readOnly && <div className="publishing-adapter"><label>Access intent<select value={accessMode} onChange={(event) => setAccessMode(event.target.value as typeof accessMode)}><option value="public">Public</option><option value="unlisted">Unlisted</option><option value="private">Private link</option></select></label><span>Customer-owned directory · copy only · read-back still required</span></div>}<div className="deployment-actions">{project.manifest.tourPreviewUrl && <button className="secondary-button" onClick={() => onPreview(project)}>Open local URL preview</button>}{!project.manifest.tourPreviewUrl && locallyReady && <button className="secondary-button" onClick={() => onPreview(project)}>Open native preview</button>}{!project.manifest.readOnly && <button className="secondary-button" disabled={!locallyReady || Boolean(savingId)} onClick={() => buildPreview(project)}>{savingId === `${project.manifest.projectId}-preview` ? 'Building preview…' : 'Build local URL preview'}</button>}{latest && !project.manifest.readOnly && <button className="secondary-button" disabled={Boolean(savingId)} onClick={() => exportRelease(project, latest.id)}>Plain export</button>}{latest && !project.manifest.readOnly && <button className="secondary-button" disabled={Boolean(savingId)} onClick={() => uploadRelease(project, latest.id)}>{savingId === `${project.manifest.projectId}-upload` ? 'Copying…' : 'Copy to customer host'}</button>}{latest?.publicUrl && <button className="secondary-button" onClick={() => window.open(latest.publicUrl, '_blank')}>Open public</button>}{!project.manifest.readOnly && <button className="primary-button" disabled={!locallyReady || Boolean(savingId)} onClick={() => build(project)}>{savingId === project.manifest.projectId ? 'Building…' : 'Build licensed release'}</button>}</div>{releases.length > 0 && <div className="release-history">{releases.slice().reverse().map((release) => <div key={release.id}><span>v{release.version}</span><strong>{release.status.replaceAll('_', ' ')}</strong><small>{formatDate(release.createdAt)} · {release.accessMode}</small></div>)}</div>}</article>; })}</div>
    {projects.flatMap((project) => (project.manifest.releases ?? []).filter((release) => ['uploaded_unverified', 'readback_passed'].includes(release.status)).map((release) => <DeploymentReadbackControl key={`${project.manifest.projectId}-${release.id}`} project={project} release={release} onUpdated={onUpdated} />))}
    {projects.flatMap((project) => (project.manifest.releases ?? []).filter((release) => ['public_verified', 'superseded'].includes(release.status)).map((release) => <article className="rollback-control" key={`rollback-${project.manifest.projectId}-${release.id}`}><div><small>{project.manifest.name} · v{release.version}</small><strong>{release.status === 'public_verified' ? 'Current verified release' : `Superseded by ${release.supersededByReleaseId ?? 'a later release'}`}</strong><span>{release.unitShareLinks?.length ?? 0} unit share link(s) · immutable history retained</span></div>{!project.manifest.readOnly && <button onClick={async () => { try { onUpdated(await createRollbackRelease(project.manifest.projectId, release.id)); } catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); } }}>Create rollback candidate</button>}</article>))}
    <div className="jobs-policy"><strong>External publication boundary</strong><span>GitHub Pages, customer domains and managed hosting require an explicitly configured publishing target. A local snapshot or HTTP response alone never becomes public_verified.</span></div>
  </main>;
}

type ChatMessage = AiHistoryMessage & { projectUpdateDraft?: AiProjectUpdateDraft; projectId?: string };

function ProjectUpdateDraftCard({ project, draft, sourceProjectId, onReview }: { project?: ProjectRecord; draft: AiProjectUpdateDraft; sourceProjectId?: string; onReview: (draft: AiProjectUpdateDraft) => void }) {
  const rows = [
    ['Development name', project?.manifest.name, draft.name],
    ['Customer company', project?.manifest.company, draft.company],
    ['Location', project?.manifest.location || 'Unavailable', draft.location],
    ['Primary colour', project?.manifest.brand.primary, draft.primary],
    ['Accent colour', project?.manifest.brand.accent, draft.accent],
  ].filter((row): row is string[] => Boolean(row[2]));
  const sameProject = Boolean(project && project.manifest.projectId === sourceProjectId && !project.manifest.readOnly);
  return <section className="ai-draft-card">
    <header><span>✦</span><div><small>Project edit draft</small><strong>{draft.summary}</strong></div></header>
    <div className="ai-draft-fields">{rows.map(([label, before, after]) => <div key={label}><span>{label}</span><small>{before}</small><i>→</i><strong>{after}</strong></div>)}</div>
    {draft.warnings.length > 0 && <div className="ai-draft-warning">⚠ {draft.warnings.join(' ')}</div>}
    <button disabled={!sameProject} onClick={() => onReview(draft)}>{sameProject ? 'Apply to project form' : 'Open the original editable project'}</button>
    <p>Review step only · Saving remains a separate action.</p>
  </section>;
}

function MarkdownMessage({ text }: { text: string }) {
  return (
    <div className="message-content">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        components={{
          a: ({ children, ...props }) => <a {...props} target="_blank" rel="noreferrer">{children}</a>,
        }}
      >
        {text}
      </ReactMarkdown>
    </div>
  );
}

function AiSettingsDialog({ status, refreshing, onRefresh, onClose }: { status?: AiStatus; refreshing: boolean; onRefresh: () => void; onClose: () => void }) {
  const [loginError, setLoginError] = useState('');
  const [settings, setSettings] = useState<DesktopSettings>();
  const [license, setLicense] = useState<LicenseStatus>();
  const [saving, setSaving] = useState(false);
  const [roleName, setRoleName] = useState('');
  const [updateStatus, setUpdateStatus] = useState<SignedUpdateStatus>();
  useEffect(() => { Promise.all([getDesktopSettings(), getLicenseStatus()]).then(([nextSettings, nextLicense]) => { setSettings(nextSettings); setLicense(nextLicense); setRoleName(nextLicense.customer ?? 'Local operator'); }).catch((cause) => setLoginError(cause instanceof Error ? cause.message : String(cause))); }, []);
  async function bindCodex() {
    setLoginError('');
    try { await openCodexLogin(); }
    catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); }
  }
  async function installLicense() { const path = await openFileDialog({ multiple: false, directory: false, filters: [{ name: 'Signed Estate Studio licence', extensions: ['json', 'license'] }] }); if (typeof path !== 'string') return; setSaving(true); setLoginError(''); try { const next = await installSignedLicense(path); setLicense(next); setRoleName(next.customer ?? roleName); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  async function changeRole(role: string) { setSaving(true); setLoginError(''); try { setLicense(await setActiveLocalRole(role, roleName)); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  async function addPublishingTarget() { if (!settings) return; const path = await openFileDialog({ multiple: false, directory: true, title: 'Choose customer-owned publishing root' }); if (typeof path !== 'string') return; const number = settings.publishingTargets.length + 1; setSettings({ ...settings, publishingTargets: [...settings.publishingTargets, { id: `customer-host-${number}`, label: `Customer host ${number}`, kind: 'customer_owned_directory', rootPath: path }] }); }
  async function addSecretReference() { if (!settings) return; const path = await openFileDialog({ multiple: false, directory: false, title: 'Reference a local secret file (contents are not imported)' }); if (typeof path !== 'string') return; const number = settings.secretReferences.length + 1; setSettings({ ...settings, secretReferences: [...settings.secretReferences, { id: `hosting-secret-${number}`, label: `Hosting secret ${number}`, kind: 'hosting_token_file', filePath: path }] }); }
  async function saveSettings() { if (!settings) return; setSaving(true); setLoginError(''); try { setSettings(await updateDesktopSettings(settings)); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  async function checkUpdate() { setSaving(true); setLoginError(''); try { setUpdateStatus(await checkSignedUpdate()); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  async function installUpdate() { if (!updateStatus?.targetVersion) return; setSaving(true); setLoginError(''); try { setUpdateStatus(await installSignedUpdate(updateStatus.targetVersion)); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  async function confirmHealth() { setSaving(true); setLoginError(''); try { setUpdateStatus(await confirmUpdateHealth()); } catch (cause) { setLoginError(cause instanceof Error ? cause.message : String(cause)); } finally { setSaving(false); } }
  return (
    <div className="dialog-backdrop" role="presentation" onMouseDown={(event) => event.target === event.currentTarget && onClose()}>
      <section className="new-project-dialog ai-settings-dialog desktop-settings-dialog">
        <div className="dialog-heading"><div><p className="eyebrow">Desktop control plane</p><h2>Estate Studio settings</h2></div><button type="button" className="icon-button" onClick={onClose}>×</button></div>
        <p className="dialog-copy">Project storage, provider addresses and local secret references remain outside portable project data. Secret file contents are never imported into settings.</p>
        <div className={`provider-card licence-card ${license?.valid ? 'connected' : ''}`}><div className="provider-mark">◇</div><div><span>Offline licence</span><strong>{license?.valid ? `${license.edition} · ${license.customer}` : license?.installed ? 'Installed but invalid' : 'Not installed'}</strong><p>{license?.detail ?? 'Checking signed licence…'}</p><small>{license?.expiresAt ? `Expires ${formatDate(license.expiresAt)}` : license?.valid ? 'No expiry recorded' : 'Commercial actions blocked'}</small></div><button className="secondary-button" disabled={saving} onClick={installLicense}>Install signed file</button></div>
        {license?.valid && <div className="settings-role-row"><label>Active local role<select value={license.activeRole ?? 'owner'} onChange={(event) => changeRole(event.target.value)}>{license.allowedRoles.map((role) => <option key={role} value={role}>{role}</option>)}</select></label><label>Role display name<input value={roleName} onChange={(event) => setRoleName(event.target.value)} /></label><span>Role records stay on this device.</span></div>}
        {settings && <div className="desktop-settings-grid"><section><h3>Storage</h3><label>Managed project library<input value={settings.storagePath} readOnly /></label><small>Read-only location · relocation requires a separately verified migration.</small></section><section><h3>Signed updates</h3><label>Channel<select value={settings.updateChannel} onChange={(event) => setSettings({ ...settings, updateChannel: event.target.value as 'stable' | 'beta' })}><option value="stable">Stable</option><option value="beta">Beta</option></select></label><label className="settings-check"><input type="checkbox" checked={settings.autoCheckUpdates} onChange={(event) => setSettings({ ...settings, autoCheckUpdates: event.target.checked })} /> Check automatically</label><div className="update-actions"><button onClick={checkUpdate}>Check signed update</button>{updateStatus?.available && updateStatus.targetVersion && <button onClick={installUpdate}>Install {updateStatus.targetVersion}</button>}{updateStatus?.recoveryPending && <button onClick={confirmHealth}>Confirm launch health</button>}</div><small>{updateStatus?.detail ?? 'Release builds require an embedded updater public key and HTTPS endpoint.'}</small></section><section><h3>Managed subscription</h3><label>HTTPS gateway address<input value={settings.managedGatewayBaseUrl ?? ''} onChange={(event) => setSettings({ ...settings, managedGatewayBaseUrl: event.target.value || undefined })} placeholder="https://gateway.customer.example" /></label><small>No token is stored here; active subscription tokens remain memory-only.</small></section><section><h3>Runtime analytics</h3><label>Same-origin endpoint path<input value={settings.analyticsEndpoint ?? ''} onChange={(event) => setSettings({ ...settings, analyticsEndpoint: event.target.value || undefined })} placeholder="/api/tour-events" /></label><small>Blank keeps network analytics disabled.</small></section><section className="settings-list-section"><h3>Publishing targets</h3>{settings.publishingTargets.map((target, index) => <div key={target.id}><input value={target.label} onChange={(event) => setSettings({ ...settings, publishingTargets: settings.publishingTargets.map((item, itemIndex) => itemIndex === index ? { ...item, label: event.target.value } : item) })} /><code>{target.rootPath}</code><button onClick={() => setSettings({ ...settings, publishingTargets: settings.publishingTargets.filter((_, itemIndex) => itemIndex !== index) })}>Remove</button></div>)}<button className="secondary-button" onClick={addPublishingTarget}>+ Customer directory</button></section><section className="settings-list-section"><h3>Secret references</h3>{settings.secretReferences.map((reference, index) => <div key={reference.id}><input value={reference.label} onChange={(event) => setSettings({ ...settings, secretReferences: settings.secretReferences.map((item, itemIndex) => itemIndex === index ? { ...item, label: event.target.value } : item) })} /><code>{reference.filePath}</code><button onClick={() => setSettings({ ...settings, secretReferences: settings.secretReferences.filter((_, itemIndex) => itemIndex !== index) })}>Remove</button></div>)}<button className="secondary-button" onClick={addSecretReference}>+ Reference local file</button></section></div>}
        <div className={`provider-card ${status?.available ? 'connected' : ''}`}>
          <div className="provider-mark">✦</div>
          <div><span>Local AI provider</span><strong>OpenAI Codex</strong><p>{status?.detail ?? 'Checking the Codex connection…'}</p><small>{status?.version ?? 'Version unavailable'}{status?.accountLabel ? ` · ${status.accountLabel}` : ''}{status?.bindingProtocol ? ` · ${status.bindingProtocol}` : ''}</small></div>
          <span className={`connection-pill ${status?.available ? 'ready' : ''}`}>{status?.available ? 'Connected' : refreshing ? 'Checking' : 'Action required'}</span>
        </div>
        <div className={`provider-card future-provider ${settings?.managedGatewayBaseUrl ? 'connected' : ''}`}>
          <div className="provider-mark muted">◎</div>
          <div><span>Commercial plan</span><strong>Managed OpenAI subscription</strong><p>{settings?.managedGatewayBaseUrl ? 'Gateway address configured. Authentication remains a separate short-lived in-memory session.' : 'Configure the customer gateway address above; do not paste tokens into settings.'}</p></div>
          <span className={`connection-pill ${settings?.managedGatewayBaseUrl ? 'ready' : ''}`}>{settings?.managedGatewayBaseUrl ? 'Address ready' : 'Not configured'}</span>
        </div>
        {loginError && <div className="form-error">{loginError}</div>}
        <div className="settings-safety"><strong>Controlled-edit boundary</strong><span>Only the selected project manifest and recent chat are disclosed. AI can prepare a project-profile draft, but the user must apply it to the form and save separately. Paid generation and deployment retain their own approval gates.</span></div>
        <div className="dialog-actions"><button type="button" className="secondary-button" onClick={onRefresh} disabled={refreshing}>{refreshing ? 'Checking…' : 'Refresh Codex'}</button>{status?.installed && !status.available && status.canLaunchLogin && <button type="button" className="secondary-button" onClick={bindCodex}>Sign in with Codex</button>}<button type="button" className="primary-button" disabled={saving || !settings} onClick={saveSettings}>{saving ? 'Saving…' : 'Save settings'}</button><button type="button" className="primary-button" onClick={onClose}>Done</button></div>
      </section>
    </div>
  );
}

function AiChat({ project, aiStatus, requestedPrompt, onPromptLoaded, onSettings, onClose, onCreateProject, onOpenKoya, onImport, onAddUnit, onReviewProjectDraft }: { project?: ProjectRecord; aiStatus?: AiStatus; requestedPrompt?: string; onPromptLoaded: () => void; onSettings: () => void; onClose: () => void; onCreateProject: () => void; onOpenKoya: () => void; onImport: () => void; onAddUnit: () => void; onReviewProjectDraft: (draft: AiProjectUpdateDraft) => void }) {
  const [draft, setDraft] = useState('');
  const [sending, setSending] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([
    { role: 'assistant', text: '**Property AI**\n\nI’m ready to help with the current project workspace.' },
  ]);

  useEffect(() => {
    if (!requestedPrompt) return;
    setDraft(requestedPrompt);
    onPromptLoaded();
  }, [requestedPrompt, onPromptLoaded]);

  async function send(event: FormEvent) {
    event.preventDefault();
    const text = draft.trim();
    if (!text || sending) return;
    if (!aiStatus?.available) { onSettings(); return; }
    const history = messages.slice(-8).map(({ role, text }) => ({ role, text }));
    setMessages((current) => [...current, { role: 'user', text }]);
    setDraft('');
    setSending(true);
    try {
      const response = await chatWithCodex(project?.manifest.projectId, text, history);
      setMessages((current) => [...current, { role: 'assistant', text: response.content, projectUpdateDraft: response.projectUpdateDraft, projectId: project?.manifest.projectId }]);
    } catch (cause) {
      setMessages((current) => [...current, { role: 'assistant', text: `⚠ **I couldn't complete that request.**\n\n${cause instanceof Error ? cause.message : String(cause)}` }]);
    } finally { setSending(false); }
  }

  return (
    <aside className="ai-chat">
      <header className="chat-header">
        <div className="assistant-avatar"><img src={asterMark} alt="" /></div>
        <div><strong>Property AI</strong><span><i className={aiStatus?.available ? 'ready' : 'offline'} /> {aiStatus?.available ? 'Connected · Codex' : 'Binding required'}</span></div>
        <div className="chat-header-actions"><button className="chat-more" aria-label="AI settings" onClick={onSettings}>•••</button><button className="chat-close" aria-label="Close Property AI" onClick={onClose}>×</button></div>
      </header>
      <div className="chat-context">
        <span>Working context</span>
        <strong>{project ? project.manifest.name : 'Project library'}</strong>
        <small>{project ? `${project.manifest.company} · ${project.manifest.units.length} unit types · ${project.manifest.enabledSkillIds.length} Skills active` : 'No project selected'}</small>
      </div>
      <div className="chat-messages">
        {messages.map((message, index) => (
          <div className={`chat-message ${message.role}`} key={`${message.role}-${index}`}>
            {message.role === 'assistant' && <span className="message-mark">✦</span>}
            <div className="message-stack"><MarkdownMessage text={message.text} />{message.projectUpdateDraft && <ProjectUpdateDraftCard project={project} draft={message.projectUpdateDraft} sourceProjectId={message.projectId} onReview={onReviewProjectDraft} />}</div>
          </div>
        ))}
        {sending && <div className="chat-message"><span className="message-mark">✦</span><div className="message-content thinking-message">Reviewing project context…</div></div>}
        <div className="chat-suggestions">
          <button onClick={onCreateProject}>Create a new project</button>
          {!project && <button onClick={onOpenKoya}>Show me the Koya example</button>}
          {project && !project.manifest.readOnly && <button onClick={onImport}>Import project sources</button>}
          {project && !project.manifest.readOnly && <button onClick={onAddUnit}>Add a unit type</button>}
          {project && <button onClick={() => setDraft(`Review ${project.manifest.name} and tell me what is missing`)}>Review project readiness</button>}
          {project && <button onClick={() => setDraft('Prepare the panorama production workflow')}>Prepare panorama workflow</button>}
        </div>
      </div>
      <form className="chat-composer" onSubmit={send}>
        <textarea aria-label="Message Property AI" rows={3} value={draft} onChange={(event) => setDraft(event.target.value)} placeholder={project ? `Ask about ${project.manifest.name}…` : 'Ask about a property project…'} />
        <div><button type="button" className="composer-status" onClick={onSettings}>{aiStatus?.available ? 'Codex · controlled edits' : 'Bind AI in Settings'}</button><button type="submit" disabled={!draft.trim() || sending} aria-label="Send message">↑</button></div>
      </form>
    </aside>
  );
}

export default function App() {
  const [projects, setProjects] = useState<ProjectRecord[]>([]);
  const [companyProfiles, setCompanyProfiles] = useState<CompanyProfile[]>([]);
  const [selectedId, setSelectedId] = useState<string>();
  const [view, setView] = useState<View>('projects');
  const [loading, setLoading] = useState(true);
  const [showArchived, setShowArchived] = useState(false);
  const [companyFilter, setCompanyFilter] = useState('all');
  const [dialogOpen, setDialogOpen] = useState(false);
  const [duplicateOpen, setDuplicateOpen] = useState(false);
  const [previewOpen, setPreviewOpen] = useState(false);
  const [addUnitOpen, setAddUnitOpen] = useState(false);
  const [importOpen, setImportOpen] = useState(false);
  const [editOpen, setEditOpen] = useState(false);
  const [addRoomOpen, setAddRoomOpen] = useState(false);
  const [selectedUnitId, setSelectedUnitId] = useState<string>();
  const [selectedRoomId, setSelectedRoomId] = useState<string>();
  const [identityOpen, setIdentityOpen] = useState(false);
  const [panoramaAssetOpen, setPanoramaAssetOpen] = useState(false);
  const [panoramaDraftOpen, setPanoramaDraftOpen] = useState(false);
  const [roomMediaOpen, setRoomMediaOpen] = useState(false);
  const [roomGraphOpen, setRoomGraphOpen] = useState(false);
  const [error, setError] = useState('');
  const [query, setQuery] = useState('');
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [aiStatus, setAiStatus] = useState<AiStatus>();
  const [aiRefreshing, setAiRefreshing] = useState(false);
  const [assistantOpen, setAssistantOpen] = useState(false);
  const [requestedAiPrompt, setRequestedAiPrompt] = useState<string>();
  const [aiProjectDraft, setAiProjectDraft] = useState<AiProjectUpdateDraft>();

  const selectedProject = useMemo(() => projects.find((project) => project.manifest.projectId === selectedId), [projects, selectedId]);
  const selectedUnit = useMemo(() => selectedProject?.manifest.units.find((unit) => unit.id === selectedUnitId), [selectedProject, selectedUnitId]);
  const selectedRoom = useMemo(() => (selectedUnit?.rooms ?? []).find((room) => room.id === selectedRoomId), [selectedUnit, selectedRoomId]);
  const eligibleIdentityAssets = useMemo(() => (selectedProject?.manifest.assets ?? []).filter((asset) => asset.status === 'accepted' && ['renders', 'photos'].includes(asset.category) && (!asset.unitId || asset.unitId === selectedUnitId)), [selectedProject, selectedUnitId]);
  const eligiblePanoramaAssets = useMemo(() => (selectedProject?.manifest.assets ?? []).filter((asset) => asset.status === 'accepted' && asset.category === 'panoramas' && (!asset.unitId || asset.unitId === selectedUnitId)), [selectedProject, selectedUnitId]);
  const filteredProjects = useMemo(() => {
    const term = query.trim().toLowerCase();
    return projects.filter(({ manifest }) => Boolean(manifest.archivedAt) === showArchived && (companyFilter === 'all' || manifest.companyProfile.id === companyFilter) && (!term || [manifest.name, manifest.company, manifest.location].some((value) => value.toLowerCase().includes(term))));
  }, [projects, query, showArchived, companyFilter]);

  useEffect(() => {
    Promise.all([listProjects(), listCompanyProfiles()])
      .then(([projectRecords, profiles]) => { setProjects(projectRecords); setCompanyProfiles(profiles); })
      .catch((cause) => setError(cause instanceof Error ? cause.message : String(cause)))
      .finally(() => setLoading(false));
  }, []);

  async function refreshAiStatus() {
    setAiRefreshing(true);
    try { setAiStatus(await checkAiStatus()); }
    catch (cause) { setAiStatus({ provider: 'codex', installed: false, authenticated: false, available: false, canLaunchLogin: false, detail: cause instanceof Error ? cause.message : String(cause) }); }
    finally { setAiRefreshing(false); }
  }

  useEffect(() => { void refreshAiStatus(); }, []);

  useLayoutEffect(() => {
    window.scrollTo({ top: 0, behavior: 'instant' });
  }, [view, selectedId, selectedUnitId]);

  function openProject(project: ProjectRecord) {
    setSelectedId(project.manifest.projectId);
    setView('project');
  }

  function handleCreated(project: ProjectRecord) {
    setProjects((current) => [...current, project]);
    setCompanyProfiles((current) => current.some((profile) => profile.id === project.manifest.companyProfile.id) ? current : [...current, { ...project.manifest.companyProfile, locale: project.manifest.locale, measurementUnits: project.manifest.measurementUnits, brand: project.manifest.brand, createdAt: project.manifest.createdAt, updatedAt: project.manifest.updatedAt }].sort((left, right) => left.name.localeCompare(right.name)));
    setDialogOpen(false);
    setDuplicateOpen(false);
    openProject(project);
  }

  function handleUpdated(project: ProjectRecord) {
    setProjects((current) => current.map((existing) => existing.manifest.projectId === project.manifest.projectId ? project : existing));
    setCompanyProfiles((current) => { const profile = { ...project.manifest.companyProfile, locale: project.manifest.locale, measurementUnits: project.manifest.measurementUnits, brand: project.manifest.brand, createdAt: project.manifest.createdAt, updatedAt: project.manifest.updatedAt }; return [...current.filter((item) => item.id !== profile.id), profile].sort((left, right) => left.name.localeCompare(right.name)); });
    setAddUnitOpen(false);
    setImportOpen(false);
    setEditOpen(false);
    setAddRoomOpen(false);
    setIdentityOpen(false);
    setPanoramaAssetOpen(false);
    setPanoramaDraftOpen(false);
    setRoomMediaOpen(false);
    setRoomGraphOpen(false);
    setAiProjectDraft(undefined);
  }

  async function handleFiles(project: ProjectRecord) {
    try {
      await openProjectFolder(project.manifest.projectId);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleArchive(project: ProjectRecord) {
    const archived = !project.manifest.archivedAt;
    if (archived && !window.confirm(`Archive ${project.manifest.name}? Files and history remain recoverable.`)) return;
    try {
      const updated = await setProjectArchived(project.manifest.projectId, archived);
      handleUpdated(updated);
      setShowArchived(archived);
      setView('projects');
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleExport(project: ProjectRecord) {
    setError('');
    try {
      const outputPath = await saveFileDialog({ defaultPath: `${project.manifest.projectId}.estateproject`, filters: [{ name: 'Estate Studio project', extensions: ['estateproject'] }] });
      if (!outputPath) return;
      await exportProjectBundle(project.manifest.projectId, outputPath);
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleImportBundle() {
    setError('');
    try {
      const inputPath = await openFileDialog({ multiple: false, directory: false, filters: [{ name: 'Estate Studio project', extensions: ['estateproject'] }] });
      if (!inputPath || Array.isArray(inputPath)) return;
      handleCreated(await importProjectBundle(inputPath));
    } catch (cause) {
      setError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  async function handleAssetReview(assetId: string, decision: 'accepted' | 'needs_review' | 'rejected', reasonCode?: AssetRejectionReason, notes?: string) {
    if (!selectedProject) return;
    try { handleUpdated(await reviewAsset(selectedProject.manifest.projectId, assetId, decision, reasonCode, notes)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
  }

  async function handleWorkflowMode(mode: 'standard' | 'advanced') {
    if (!selectedProject) return;
    try { handleUpdated(await setWorkflowMode(selectedProject.manifest.projectId, mode)); }
    catch (cause) { setError(cause instanceof Error ? cause.message : String(cause)); }
  }

  function openUnit(unit: UnitRecord) { setSelectedUnitId(unit.id); setView('unit'); }
  function chooseIdentity(room: RoomRecord) { setSelectedRoomId(room.id); setIdentityOpen(true); }
  function choosePanorama(room: RoomRecord) { setSelectedRoomId(room.id); setPanoramaAssetOpen(true); }
  function preparePanorama(room: RoomRecord) { setSelectedRoomId(room.id); setPanoramaDraftOpen(true); }
  function manageRoomMedia(room: RoomRecord) { setSelectedRoomId(room.id); setRoomMediaOpen(true); }

  return (
    <div className={`app-shell ${assistantOpen ? 'assistant-open' : ''}`}>
      <aside className="sidebar">
        <div className="brand-lockup"><div className="brand-symbol"><img src={asterMark} alt="" /></div><div><strong>estate studio</strong><span>Property AI workspace</span></div></div>
        <nav>
          <span className="nav-section-label">Workspace</span>
          <button className={!['production', 'creative', 'skills', 'jobs', 'deployments'].includes(view) ? 'active' : ''} onClick={() => setView('projects')}><span><NavGlyph name="projects" /></span> Projects</button>
          <span className="nav-section-label">Production</span>
          <button className={view === 'production' ? 'active' : ''} onClick={() => setView('production')}><span><NavGlyph name="production" /></span> AI production</button>
          <button className={view === 'creative' ? 'active' : ''} onClick={() => setView('creative')}><span><NavGlyph name="creative" /></span> Creative Studio</button>
          <button className={view === 'skills' ? 'active' : ''} onClick={() => setView('skills')}><span><NavGlyph name="skills" /></span> Skills</button>
          <button className={view === 'jobs' ? 'active' : ''} onClick={() => setView('jobs')}><span><NavGlyph name="jobs" /></span> Production Queue</button>
          <button className={view === 'deployments' ? 'active' : ''} onClick={() => setView('deployments')}><span><NavGlyph name="deployments" /></span> Deployments</button>
        </nav>
        <div className="sidebar-spacer" />
        <div className={`subscription-card ${aiStatus?.available ? 'connected' : ''}`}><span>Property AI</span><strong>{aiStatus?.available ? 'Codex connected' : 'Binding required'}</strong><p>{aiStatus?.available ? 'Project-aware chat and controlled profile drafts are ready. Generation stays separately approval-gated.' : 'Connect the official Codex session. No customer API key required.'}</p></div>
        <button className="settings-button" onClick={() => setSettingsOpen(true)}><span><NavGlyph name="settings" /></span>Settings</button>
      </aside>

      <div className="workspace">
        {view === 'projects' ? (
          <main className="projects-view">
            <header className="page-header">
              <div><p className="eyebrow">Local project library</p><h1>Property projects</h1><p>Every development keeps its own evidence, units, AI assets and releases.</p></div>
              <div className="detail-actions"><button className="secondary-button" onClick={handleImportBundle}>Import bundle</button><button className="primary-button large" onClick={() => setDialogOpen(true)}>+ New project</button></div>
            </header>
            <div className="library-toolbar">
              <div className="search-box">⌕ <input aria-label="Search projects" placeholder="Search developments or companies" value={query} onChange={(event) => setQuery(event.target.value)} /></div>
              <select aria-label="Filter by customer company" value={companyFilter} onChange={(event) => setCompanyFilter(event.target.value)}><option value="all">All companies</option>{companyProfiles.map((profile) => <option key={profile.id} value={profile.id}>{profile.name}</option>)}</select>
              <button className="secondary-button" onClick={() => setShowArchived((value) => !value)}>{showArchived ? 'Show active projects' : 'Show archived projects'}</button>
              <div className="library-count">{filteredProjects.length} project{filteredProjects.length === 1 ? '' : 's'}</div>
            </div>
            {error && <div className="page-error">Unable to load the local project library: {error}</div>}
            {loading ? <div className="loading-state">Opening local project library…</div> : (
              <div className="project-grid">
                {filteredProjects.map((project) => <ProjectCard key={project.manifest.projectId} project={project} onOpen={() => openProject(project)} />)}
                {filteredProjects.length === 0 && <div className="empty-state"><strong>No matching projects</strong><span>Try a development name, company or location.</span></div>}
                {!showArchived && <button className="create-card" onClick={() => setDialogOpen(true)}><span>+</span><strong>Create a new development</strong><small>Starts with a clean, isolated project workspace</small></button>}
              </div>
            )}
            <section className="workflow-strip">
              <div><span>1</span><strong>Create project</strong><small>Brand, facts and units</small></div>
              <i>→</i>
              <div><span>2</span><strong>Generate</strong><small>Images and panoramas</small></div>
              <i>→</i>
              <div><span>3</span><strong>Review</strong><small>Spatial and visual QA</small></div>
              <i>→</i>
              <div><span>4</span><strong>Publish</strong><small>Build and verify</small></div>
            </section>
          </main>
        ) : view === 'production' ? (
          <AiProductionWorkspace projects={projects} project={selectedProject} onSelectProject={(project) => { setSelectedId(project.manifest.projectId); setView('production'); }} onOpenUnit={openUnit} onAskAi={setRequestedAiPrompt} />
        ) : view === 'creative' ? (
          <CreativeStudioWorkspace projects={projects} project={selectedProject} aiReady={Boolean(aiStatus?.available)} onSelectProject={(project) => { setSelectedId(project.manifest.projectId); setView('creative'); }} onUpdated={handleUpdated} onSettings={() => setSettingsOpen(true)} />
        ) : view === 'skills' ? (
          <SkillsMarketplace projects={projects} project={selectedProject} onSelectProject={(project) => { setSelectedId(project.manifest.projectId); setView('skills'); }} onUpdated={handleUpdated} />
        ) : view === 'jobs' ? (
          <JobsWorkspace projects={projects} onUpdated={handleUpdated} onOpen={(project, unit) => { setSelectedId(project.manifest.projectId); setSelectedUnitId(unit.id); setView('unit'); }} />
        ) : view === 'deployments' ? (
          <DeploymentsWorkspace projects={projects} onUpdated={handleUpdated} onPreview={(project) => { setSelectedId(project.manifest.projectId); setPreviewOpen(true); }} />
        ) : view === 'unit' && selectedProject && selectedUnit ? (
          <UnitWorkspace project={selectedProject} unit={selectedUnit} onBack={() => setView('project')} onImport={() => setImportOpen(true)} onAddRoom={() => setAddRoomOpen(true)} onEditGraph={() => setRoomGraphOpen(true)} onTour={() => setPreviewOpen(true)} onAssignIdentity={chooseIdentity} onAssignPanorama={choosePanorama} onPreparePanorama={preparePanorama} onManageMedia={manageRoomMedia} />
        ) : selectedProject ? (
          <ProjectDetail project={selectedProject} onBack={() => setView('projects')} onTour={() => setPreviewOpen(true)} onCreative={() => setView('creative')} onFiles={() => handleFiles(selectedProject)} onAddUnit={() => setAddUnitOpen(true)} onImport={() => setImportOpen(true)} onEdit={() => { setAiProjectDraft(undefined); setEditOpen(true); }} onArchive={() => handleArchive(selectedProject)} onDuplicate={() => setDuplicateOpen(true)} onExport={() => handleExport(selectedProject)} onOpenUnit={openUnit} onReviewAsset={handleAssetReview} onSetWorkflowMode={handleWorkflowMode} />
        ) : null}
      </div>

      {assistantOpen ? <AiChat
          project={selectedProject}
          aiStatus={aiStatus}
          requestedPrompt={requestedAiPrompt}
          onPromptLoaded={() => setRequestedAiPrompt(undefined)}
          onSettings={() => setSettingsOpen(true)}
          onClose={() => setAssistantOpen(false)}
          onCreateProject={() => setDialogOpen(true)}
          onOpenKoya={() => {
            const koya = projects.find((project) => project.manifest.projectId === 'koya-example');
            if (koya) openProject(koya);
          }}
          onImport={() => selectedProject && !selectedProject.manifest.readOnly && setImportOpen(true)}
          onAddUnit={() => selectedProject && !selectedProject.manifest.readOnly && setAddUnitOpen(true)}
          onReviewProjectDraft={(draft) => { if (selectedProject && !selectedProject.manifest.readOnly) { setAiProjectDraft(draft); setEditOpen(true); } }}
        /> : <button className="assistant-launcher" onClick={() => setAssistantOpen(true)}><span><img src={asterMark} alt="" /></span><strong>Property AI</strong><small>{aiStatus?.available ? 'Connected' : 'Set up AI'}</small></button>}

      {dialogOpen && <NewProjectDialog onClose={() => setDialogOpen(false)} onCreated={handleCreated} />}
      {duplicateOpen && selectedProject && <DuplicateProjectDialog project={selectedProject} onClose={() => setDuplicateOpen(false)} onCreated={handleCreated} />}
      {addUnitOpen && selectedProject && <AddUnitDialog project={selectedProject} onClose={() => setAddUnitOpen(false)} onUpdated={handleUpdated} />}
      {importOpen && selectedProject && <ImportSourceDialog project={selectedProject} onClose={() => setImportOpen(false)} onUpdated={handleUpdated} />}
      {editOpen && selectedProject && <EditProjectDialog project={selectedProject} aiDraft={aiProjectDraft} onClose={() => { setEditOpen(false); setAiProjectDraft(undefined); }} onUpdated={handleUpdated} />}
      {addRoomOpen && selectedProject && selectedUnit && <AddRoomDialog project={selectedProject} unit={selectedUnit} onClose={() => setAddRoomOpen(false)} onUpdated={handleUpdated} />}
      {identityOpen && selectedProject && selectedUnit && selectedRoom && <IdentityAssetDialog project={selectedProject} unit={selectedUnit} room={selectedRoom} assets={eligibleIdentityAssets} onClose={() => setIdentityOpen(false)} onUpdated={handleUpdated} />}
      {panoramaAssetOpen && selectedProject && selectedUnit && selectedRoom && <PanoramaAssetDialog project={selectedProject} unit={selectedUnit} room={selectedRoom} assets={eligiblePanoramaAssets} onClose={() => setPanoramaAssetOpen(false)} onUpdated={handleUpdated} />}
      {panoramaDraftOpen && selectedProject && selectedUnit && selectedRoom && <PanoramaDraftDialog project={selectedProject} unit={selectedUnit} room={selectedRoom} onClose={() => setPanoramaDraftOpen(false)} onUpdated={handleUpdated} />}
      {roomMediaOpen && selectedProject && selectedUnit && selectedRoom && <RoomMediaDialog project={selectedProject} unit={selectedUnit} room={selectedRoom} onClose={() => setRoomMediaOpen(false)} onUpdated={handleUpdated} />}
      {roomGraphOpen && selectedProject && selectedUnit && <RoomGraphDialog project={selectedProject} unit={selectedUnit} onClose={() => setRoomGraphOpen(false)} onUpdated={handleUpdated} />}
      {previewOpen && selectedProject && <TourPreview project={selectedProject} onClose={() => setPreviewOpen(false)} />}
      {settingsOpen && <AiSettingsDialog status={aiStatus} refreshing={aiRefreshing} onRefresh={refreshAiStatus} onClose={() => setSettingsOpen(false)} />}
    </div>
  );
}
