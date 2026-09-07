export const escapeHtml = value => String(value).replace(/[&<>"']/g, c=>({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
export const json = value => JSON.stringify(value).replace(/</g,'\\u003c');
export function getConfig(env=process.env) {
 const production = env.BUILD_ENV === 'production';
 let origin = '';
 if (env.SITE_URL) {
  const url = new URL(env.SITE_URL);
  if (url.protocol !== 'https:' || url.username || url.password || url.pathname !== '/' || url.search || url.hash || url.hostname === 'localhost' || !url.hostname.includes('.') || /^\d+\.\d+\.\d+\.\d+$/.test(url.hostname) || url.hostname.endsWith('.local')) throw new Error('SITE_URL must be a public HTTPS origin without path, credentials or query');
  origin = url.origin;
 }
 if (production && !origin) throw new Error('Production build requires SITE_URL');
 return {production,origin};
}
export function seoHead({path,alternate,lang,title,description,home,product},config) {
 const {origin,production} = config;
 let result = `<meta name="robots" content="${production?'index,follow':'noindex,nofollow'}"><meta name="twitter:card" content="summary_large_image">`;
 if (!origin) return result;
 const url = origin+path;
 const english = lang==='en'?path:alternate;
 const chinese = lang==='en'?alternate:path;
 const graph=[{'@type':'WebPage','@id':url+'#page',url,name:title,description,inLanguage:lang}];
 if(product) graph.push({'@type':'BreadcrumbList',itemListElement:[{'@type':'ListItem',position:1,name:lang==='en'?'Home':'首页',item:origin+home},{'@type':'ListItem',position:2,name:'Estate Studio',item:url}]});
 result += `<link rel="canonical" href="${escapeHtml(url)}"><link rel="alternate" hreflang="en" href="${escapeHtml(origin+english)}"><link rel="alternate" hreflang="zh-Hans" href="${escapeHtml(origin+chinese)}"><link rel="alternate" hreflang="x-default" href="${escapeHtml(origin+english)}"><meta property="og:url" content="${escapeHtml(url)}"><meta property="og:image" content="${escapeHtml(origin+'/assets/living.webp')}"><meta property="og:image:alt" content="${lang==='en'?'AI concept room imagery':'AI 概念房间画面'}"><script type="application/ld+json">${json({'@context':'https://schema.org','@graph':graph})}</script>`;
 return result;
}
