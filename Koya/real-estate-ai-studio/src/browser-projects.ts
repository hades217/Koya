import type { CompanyProfileSnapshot, ProjectRecord } from './types';

export const CURRENT_PROJECT_SCHEMA_VERSION = 30;

function defaultDesignSpec(project: ProjectRecord) {
  return {
    status: 'not_started' as const,
    direction: 'Project-specific design direction has not been approved.',
    sourceBasis: [] as string[],
    colours: {
      paper: '#f5f1e7',
      ink: project.manifest.brand?.primary || '#20241f',
      accent: project.manifest.brand?.accent || '#78917b',
      muted: '#796d61',
      botanical: '#66746b',
    },
    typography: {
      display: 'Didot, "Bodoni 72", Georgia, serif',
      body: '"Avenir Next", "Helvetica Neue", Arial, sans-serif',
    },
    layout: {
      page: 'A4 portrait' as const,
      marginMm: 18,
      grid: 'Editorial asymmetric grid with generous negative space',
      imageTreatment: 'Approved project imagery only; natural colour; no decorative tint',
    },
  };
}

function browserCompanyProfile(project: ProjectRecord): CompanyProfileSnapshot {
  const existing = project.manifest.companyProfile;
  if (existing?.id?.trim() && existing?.name?.trim()) return existing;
  const name = project.manifest.company?.trim() || 'Company unavailable';
  const id = name.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '') || 'company';
  return { id, name };
}

/**
 * Browser preview data is intentionally lightweight, but it must still use the
 * same current manifest contract as the desktop app. Keep this migration pure
 * so legacy localStorage records can be verified without a browser or Tauri.
 */
export function normalizeBrowserProject(project: ProjectRecord): ProjectRecord {
  const manifest = project.manifest;
  return {
    ...project,
    manifest: {
      ...manifest,
      schemaVersion: CURRENT_PROJECT_SCHEMA_VERSION,
      companyProfile: browserCompanyProfile(project),
      locale: manifest.locale || 'en-AU',
      measurementUnits: manifest.measurementUnits || 'metric',
      accessMode: manifest.accessMode || 'unlisted',
      disclosure: manifest.disclosure || 'Disclosure not supplied',
      workflowMode: manifest.workflowMode ?? 'standard',
      approvalEvents: manifest.approvalEvents ?? [],
      qaRecords: manifest.qaRecords ?? [],
      usageLedger: manifest.usageLedger ?? [],
      backgroundJobs: manifest.backgroundJobs ?? [],
      creativeJobs: (manifest.creativeJobs ?? []).map((brief) => ({
        ...brief,
        projectHighlights: brief.projectHighlights ?? [],
        ...(brief.kind === 'poster' && !brief.posterMode
          ? {
              posterMode: brief.format === 'a4_portrait' ? 'a4_sales_sheet' as const : 'ai_creative' as const,
              workflow: brief.format === 'a4_portrait' ? 'deterministic_svg' as const : brief.workflow,
            }
          : {}),
      })),
      enabledSkillIds: manifest.enabledSkillIds ?? [],
      analyticsEvents: manifest.analyticsEvents ?? [],
      designSpec: manifest.designSpec ?? defaultDesignSpec(project),
    },
  };
}
