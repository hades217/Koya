import assert from 'node:assert/strict';
import {readFile, access} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
const root = fileURLToPath(new URL('./dist/', import.meta.url));
for (const page of ['index.html','zh/index.html']) {
 const html = await readFile(root+page,'utf8');
 assert.equal((html.match(/<h1[ >]/g)||[]).length,1,`${page}: exactly one main heading`);
 const ids = [...html.matchAll(/\bid="([^"]+)"/g)].map(m=>m[1]);
 assert.equal(ids.length,new Set(ids).size,`${page}: unique element IDs`);
 for(const [,id] of html.matchAll(/href="#([^"]+)"/g)) assert(ids.includes(id),`Missing anchor ${id}`);
 for(const [,path] of html.matchAll(/(?:src|href)="(\/[^"?#]*)"/g)) await access(root+path.slice(1)+(path.endsWith('/')?'index.html':''));
 assert(!html.includes('local-first desktop workspace')&&!html.includes('本地优先的桌面工作空间'));
 assert(!/undefined|\[object Object\]/.test(html),'No missing content values');
 console.log(`${page}: local links, assets, anchors, content and heading checks passed`);
}

// Exercise public-document and deployment boundaries, not just the local preview.
const {mkdtemp,rm} = await import('node:fs/promises');
const {tmpdir} = await import('node:os');
const {join} = await import('node:path');
const {execFileSync} = await import('node:child_process');
const {createPreviewServer} = await import('./serve.mjs');
const {getConfig} = await import('./src/seo.mjs');
assert.throws(()=>getConfig({BUILD_ENV:'production'}),/SITE_URL/);
for(const SITE_URL of ['http://site.com','https://localhost','https://site.com/path','https://user:secret@site.com']) assert.throws(()=>getConfig({BUILD_ENV:'production',SITE_URL}));
const fixture=await mkdtemp(join(tmpdir(),'estate-seo-'));
const routes=['/','/zh/','/products/estate-studio/','/zh/products/estate-studio/'];
for(const route of routes) {
 const preview=await readFile(root+route+'index.html','utf8');
 assert(!preview.includes('googletagmanager.com'),route+': preview does not load analytics');
}
try {
 execFileSync(process.execPath,[fileURLToPath(new URL('./build.mjs',import.meta.url))],{env:{...process.env,BUILD_ENV:'production',SITE_URL:'https://seo-fixture.invalid',BUILD_OUTPUT:fixture},stdio:'pipe'});
 for(const route of routes){
  const html=await readFile(fixture+route+'index.html','utf8');
  assert(!html.includes('noindex'),route+': production is indexable');
  assert.equal((html.match(/src="https:\/\/www.googletagmanager.com\/gtag\/js\?id=G-B92J94P81M"/g)||[]).length,1,route+': one Google tag loader');
  assert.equal((html.match(/gtag\('config', 'G-B92J94P81M'\)/g)||[]).length,1,route+': one GA configuration');
  assert.equal((html.match(/<h1[ >]/g)||[]).length,1);
  assert(html.includes(`rel="canonical" href="https://seo-fixture.invalid${route}"`));
  for(const locale of ['en','zh-Hans','x-default']) assert(html.includes(`hreflang="${locale}"`));
  const ids=[...html.matchAll(/\bid="([^"]+)"/g)].map(m=>m[1]);
  assert.equal(ids.length,new Set(ids).size);
  for(const [,id] of html.matchAll(/href="#([^"]+)"/g)) assert(ids.includes(id),'Missing '+id);
  for(const [,path] of html.matchAll(/(?:href|src)="(\/[^"?#]*)"/g)) await access(fixture+path+(path.endsWith('/')?'index.html':''));
  const graph=JSON.parse(html.match(/<script type="application\/ld\+json">([\s\S]*?)<\/script>/)[1]);
  assert.equal(graph['@graph'][0].url,'https://seo-fixture.invalid'+route);
  if(route.includes('/products/')){
   assert(!html.includes('src="/app.js"'),'Product navigation does not load homepage JavaScript');
   assert.equal((html.match(/<details/g)||[]).length,9,'Full native FAQ and table of contents');
   assert(html.includes('mailto:hello@estatestudio.io'));
   assert.equal(graph['@graph'][1].itemListElement.length,2);
   for(const id of ['overview','features','inputs','workflow','scenarios','faq']) assert(ids.includes(id));
  } else assert(html.includes('href="'+(route==='/zh/'?'/zh':'')+'/products/estate-studio/"'));
 }
 const sitemap=await readFile(fixture+'/sitemap.xml','utf8');
 assert.equal((sitemap.match(/<loc>/g)||[]).length,4);
 assert(!(await readFile(fixture+'/_headers','utf8')).includes('noindex'));
 const server=createPreviewServer(fixture);
 await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
 try{
  const base='http://127.0.0.1:'+server.address().port;
  for(const route of routes){
   const response=await fetch(base+route);
   assert.equal(response.status,200);
   assert(response.headers.get('x-robots-tag').includes('noindex'),'Local server always stays noindex');
   const raw=await response.text();
   assert(raw.includes('<h1>'),'Full HTML without executing JS');
  }
  for(const [alias,target] of [['/products/estate-studio','/products/estate-studio/'],['/zh/products/estate-studio/index.html','/zh/products/estate-studio/'],['/index.html','/']]){
   const response=await fetch(base+alias+'?utm_source=test',{redirect:'manual'});
   assert.equal(response.status,301);assert.equal(response.headers.get('location'),target+'?utm_source=test');
  }
  assert.equal((await fetch(base+'/missing-page/')).status,404);
  assert.equal((await fetch(base+'/products/estate-studio/',{method:'POST'})).status,405);
 }finally{await new Promise(resolve=>server.close(resolve));}
 console.log('Production SEO, full HTML, native navigation, redirects, 404 and preview isolation passed.');
}finally{await rm(fixture,{recursive:true,force:true});}
