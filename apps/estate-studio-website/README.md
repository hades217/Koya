# Estate Studio static website

Independent, complete HTML documents. No SPA or client-side router.

| Page | English | Chinese |
| --- | --- | --- |
| Brand homepage | `/` | `/zh/` |
| SEO product introduction | `/products/estate-studio/` | `/zh/products/estate-studio/` |

Requirements: [Product SEO PRD V2](../../docs/product/ESTATE_STUDIO_PRODUCT_SEO_PRD_V2.md).

## Local preview

```sh
npm --prefix apps/estate-studio-website run dev
```

Default port: 18765. If occupied, use `PORT=18766 npm --prefix apps/estate-studio-website run dev`. The active preview for the independent product pages is at http://127.0.0.1:18766/zh/products/estate-studio/ . Rebuild and refresh after source changes; no dependency installation is needed.

```sh
npm --prefix apps/estate-studio-website run verify
```

Verification covers static links and anchors, full product HTML, production metadata and language alternates, JSON-LD, sitemap, missing-domain rejection, directory/index redirects, real 404 responses and preview noindex isolation. Production tests use a temporary `.invalid` domain fixture and remove their output.

## Production build

Set `BUILD_ENV=production` and `SITE_URL` to the confirmed public HTTPS origin, then run the build command. A missing origin, HTTP origin, credentials or an origin with a path fails validation. Do not infer a deployment domain from the contact email.

- `dist/` contains four HTML documents, assets, `404.html`, `robots.txt`, `sitemap.xml`, `_headers` and `_redirects`.
- Production emits absolute self-canonicals, reciprocal `en`/`zh-Hans`/`x-default` alternates, social metadata and WebPage JSON-LD. Product pages also contain BreadcrumbList JSON-LD.
- Preview output remains noindex and its sitemap contains no indexable URLs. The loopback-only `serve.mjs` is deliberately always noindex, even when serving a production build; it is not the production host.
- `_headers` and `_redirects` use Netlify/Cloudflare-style static-host configuration. A different host must implement equivalent headers, canonical redirects and real 404 handling. Never configure an SPA fallback to the homepage.
- There is no public deployment, domain verification, Search Console submission or indexing claim in this delivery. Public asset rights and final sharing imagery must be confirmed before launch.

## Source and behaviour

- `src/product-content.mjs`: separate, substantive English and Chinese product copy.
- `src/render-product.mjs`: complete product documents. Navigation, table of contents, FAQ and email contact work without scripting; production builds add Google Analytics through `build.mjs`.
- `src/seo.mjs`: build environment, canonical, hreflang and structured-data generation.
- `src/content.mjs` and `src/render.mjs`: existing homepage content and template; build adds real product-page links to navigation and footer.
- `public/product.css`: readable product layout with responsive table of contents and feature sections.
- Existing homepage interactions remain in `public/app.js`; they are not loaded by the product pages.

Contact uses the owner-confirmed `hello@estatestudio.io`. Product-page links open a mail application. The visitor must send the message; no receipt or scheduled meeting is claimed. Production builds load Google Analytics (`G-B92J94P81M`) once on each of the four pages. Preview builds omit analytics. No third-party fonts or server-side form collection are introduced.
