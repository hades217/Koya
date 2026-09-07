# Applications

- `real-estate-ai-studio/` is the local-first Tauri desktop product.
- `interactive-apartment-tour/` is the browser/static tour and the desktop app's current embedded preview source.
- `estate-studio-admin/` is the Web CRM and Super Admin service, with SQLite persistence, consultation intake and email-link accounts. Run `npm run dev:admin` from the repository root; see its README for preview and production boundaries.

Keep reusable application behavior here. Customer facts, original source files and provider production records belong under `projects/<project-id>/` and should enter an app through an explicit project adapter or fixture.
