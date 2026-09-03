import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/postcss';
import { defineConfig } from 'vite';

const pagesBase = process.env.KOYA_TOUR_BASE ?? '/Koya/';

export default defineConfig({
  root: 'github-pages',
  base: pagesBase,
  publicDir: '../public',
  css: { postcss: { plugins: [tailwindcss()] } },
  plugins: [
    {
      name: 'github-pages-asset-base',
      enforce: 'pre',
      transform(code, id) {
        if (!id.endsWith('/app/page.tsx') && !id.endsWith('/app/clarity/page.tsx')) return;
        return code
          .replaceAll("'/tour/", `'${pagesBase}tour/`)
          .replaceAll('"/tour/', `"${pagesBase}tour/`);
      },
    },
    react(),
  ],
  build: {
    outDir: '../pages-dist',
    emptyOutDir: true,
  },
});
