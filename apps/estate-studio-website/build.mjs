import {mkdir,writeFile,cp} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
import {resolve} from 'node:path';
import {content} from './src/content.mjs';
import {render} from './src/render.mjs';
import {products} from './src/product-content.mjs';
import {renderProduct} from './src/render-product.mjs';
import {getConfig,seoHead,escapeHtml} from './src/seo.mjs';
const root=fileURLToPath(new URL('.',import.meta.url));
const config=getConfig();
const googleTag=config.production?`<!-- Google tag (gtag.js) -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-B92J94P81M"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-B92J94P81M');
</script>`:'';
const out=resolve(process.env.BUILD_OUTPUT || root+'dist');
await mkdir(out,{recursive:true});
await cp(root+'public',out,{recursive:true});
const paths=[];
for(const language of ['en','zh']) {
 const c={...content[language]},p=products[language];
 if(config.production) {
  c.privacyTitle=language==='en'?'Privacy and analytics.':'隐私与访问统计。';
  c.privacyBody=language==='en'
   ?'This website uses Google Analytics to measure visits and usage, which may use cookies. We do not store demo form entries or upload project files. Preparing a demo email passes your entries to your chosen email application; the email is sent only when you send it. Contact hello@estatestudio.io with privacy questions.'
   :'本网站使用 Google Analytics 统计访问和使用情况，可能使用 Cookie。我们不会保存演示预约表单内容或上传项目文件。准备预约邮件时，你输入的信息会传给所选邮件应用；只有你确认发送后，邮件才会发出。隐私问题请联系 hello@estatestudio.io。';
 }
 let home=render(c).replace('<meta name="robots" content="noindex,nofollow">',seoHead({...c,path:p.home,alternate:language==='en'?'/zh/':'/'},config));
 const link=`<a href="${p.path}">${p.productLabel}</a>`;
 home=home.replace(`<nav aria-label="${c.menu}">`,`<nav aria-label="${c.menu}">${link}`);
 home=home.replace('<div class="footer-links">',`<div class="footer-links">${link}`);
 for(const [path,html] of [[p.home,home],[p.path,renderProduct(p,config)]]) {
  await mkdir(out+path,{recursive:true});
  await writeFile(out+path+'index.html',html.replace('</head>',googleTag+'</head>'));
  paths.push(path);
 }
}
await writeFile(out+'/sitemap.xml',`<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">${config.production?paths.map(p=>`<url><loc>${escapeHtml(config.origin+p)}</loc></url>`).join(''):''}</urlset>`);
await writeFile(out+'/robots.txt',`User-agent: *\nAllow: /\n${config.production?'Sitemap: '+config.origin+'/sitemap.xml\n':''}`);
await writeFile(out+'/_headers',config.production?'/*\n  X-Content-Type-Options: nosniff\n':'/*\n  X-Robots-Tag: noindex, nofollow\n  X-Content-Type-Options: nosniff\n');
await writeFile(out+'/_redirects',paths.flatMap(p=>[`${p}index.html ${p} 301`,...(p==='/'?[]:[`${p.slice(0,-1)} ${p} 301`])]).join('\n')+'\n');
await writeFile(out+'/404.html','<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="robots" content="noindex"><title>Page not found | Estate Studio</title><link rel="stylesheet" href="/styles.css"></head><body><main class="wrap section"><p class="eyebrow">404 / Estate Studio</p><h1>Page not found.</h1><p>This address does not have a page.</p><a class="button" href="/">Return home ↗</a></main></body></html>');
console.log(`Built ${paths.length} static documents (${config.production?'production':'noindex preview'}) in ${out}`);
