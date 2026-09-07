import {createServer} from 'node:http';
import {realpathSync} from 'node:fs';
import {readFile,stat,realpath} from 'node:fs/promises';
import {resolve,extname,sep} from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const defaultRoot=fileURLToPath(new URL('./dist',import.meta.url));
const types={'.html':'text/html; charset=utf-8','.css':'text/css; charset=utf-8','.js':'text/javascript; charset=utf-8','.webp':'image/webp','.svg':'image/svg+xml','.xml':'application/xml; charset=utf-8','.txt':'text/plain; charset=utf-8'};
export function createPreviewServer(directory=defaultRoot) {
 const root=realpathSync(resolve(directory));
 return createServer(async(req,res)=>{
  const headers={'Cache-Control':'no-store','X-Content-Type-Options':'nosniff','X-Robots-Tag':'noindex, nofollow'};
  if(!['GET','HEAD'].includes(req.method)){res.writeHead(405,{...headers,Allow:'GET, HEAD'}).end();return;}
  try {
   const url=new URL(req.url,'http://localhost');
   const pathname=decodeURIComponent(url.pathname);
   let path=resolve(root,'.'+pathname);
   if(path!==root&&!path.startsWith(root+sep)){res.writeHead(403,headers).end();return;}
   const info=await stat(path);
   const actual=await realpath(path);
   if(actual!==root&&!actual.startsWith(root+sep)){res.writeHead(403,headers).end();return;}
   let target;
   if(info.isDirectory()&&!pathname.endsWith('/')) target=pathname+'/';
   else if(pathname.endsWith('/index.html')) target=pathname.slice(0,-10);
   if(target){res.writeHead(301,{...headers,Location:encodeURI(target)+url.search}).end();return;}
   if(info.isDirectory()) path=resolve(path,'index.html');
   const body=await readFile(path);
   res.writeHead(200,{...headers,'Content-Type':types[extname(path)]||'application/octet-stream'});
   res.end(req.method==='HEAD'?undefined:body);
  }catch{
   res.writeHead(404,{...headers,'Content-Type':'text/html; charset=utf-8'});
   const body=await readFile(resolve(root,'404.html')).catch(()=>Buffer.from('Not found'));
   res.end(req.method==='HEAD'?undefined:body);
  }
 });
}
if(process.argv[1]&&import.meta.url===pathToFileURL(resolve(process.argv[1])).href) {
 const port=Number(process.env.PORT||18765);
 createPreviewServer().listen(port,'127.0.0.1',()=>console.log(`Estate Studio preview: http://127.0.0.1:${port}/zh/products/estate-studio/`));
}
