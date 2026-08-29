'use client';

import { useState } from 'react';

const oldImage = '/tour/local-clarity/living-front-1254.png';
const newImage = '/tour/local-clarity/living-front-detail-2508.png';

export default function ClarityComparison() {
  const [split, setSplit] = useState(50);
  const [zoom, setZoom] = useState(1);

  return (
    <main className="clarity-page">
      <header className="clarity-header">
        <div>
          <strong>Living room · 本地清晰度测试</strong>
          <span>左：1254 原图　右：四块细节拼接 2508</span>
        </div>
        <nav>
          <button onClick={() => setZoom((value) => value === 1 ? 2 : 1)}>{zoom === 1 ? '放大 2×' : '恢复全图'}</button>
          <a href="/">返回看房</a>
        </nav>
      </header>

      <section className="clarity-stage">
        <div className="clarity-frame" style={{ transform: `scale(${zoom})` }}>
          <img src={oldImage} alt="1254 source" draggable={false} />
          <div className="clarity-new" style={{ clipPath: `inset(0 0 0 ${split}%)` }}>
            <img src={newImage} alt="2508 detail stitched test" draggable={false} />
          </div>
        </div>
        <div className="clarity-divider" style={{ left: `${split}%` }}><i /></div>
        <span className="clarity-label old">1254</span>
        <span className="clarity-label fresh">2508 DETAIL</span>
      </section>

      <footer className="clarity-controls">
        <span>原图</span>
        <input aria-label="清晰度对比位置" type="range" min="0" max="100" value={split} onChange={(event) => setSplit(Number(event.target.value))} />
        <span>细节版</span>
      </footer>

      <style>{`
        .clarity-page { width:100vw; height:100svh; overflow:hidden; color:#f5f1e8; background:#17130f; }
        .clarity-header { position:absolute; z-index:5; top:0; left:0; right:0; height:76px; display:flex; align-items:center; justify-content:space-between; gap:20px; padding:14px 22px; background:linear-gradient(#17130f,rgba(23,19,15,.78)); }
        .clarity-header div { display:grid; gap:4px; }
        .clarity-header strong { font:400 24px/1.1 Georgia,serif; }
        .clarity-header span { color:rgba(255,255,255,.62); font-size:11px; letter-spacing:.05em; }
        .clarity-header nav { display:flex; gap:8px; }
        .clarity-header button,.clarity-header a { border:1px solid rgba(255,255,255,.3); border-radius:999px; padding:9px 13px; color:white; background:rgba(255,255,255,.08); font-size:12px; text-decoration:none; cursor:pointer; }
        .clarity-stage { position:absolute; inset:76px 0 70px; overflow:hidden; background:#0e0c0a; }
        .clarity-frame,.clarity-new { position:absolute; inset:0; }
        .clarity-frame { transform-origin:50% 52%; transition:transform .35s ease; }
        .clarity-frame>img,.clarity-new img { width:100%; height:100%; object-fit:contain; user-select:none; }
        .clarity-new { background:#0e0c0a; }
        .clarity-divider { position:absolute; z-index:3; top:0; bottom:0; width:1px; background:white; box-shadow:0 0 18px rgba(0,0,0,.75); pointer-events:none; }
        .clarity-divider i { position:absolute; top:50%; left:50%; width:32px; height:32px; border:1px solid white; border-radius:50%; background:rgba(20,16,13,.72); transform:translate(-50%,-50%); }
        .clarity-label { position:absolute; z-index:4; top:14px; padding:7px 10px; border-radius:999px; background:rgba(20,16,13,.68); font:700 10px/1 Arial,sans-serif; letter-spacing:.1em; }
        .clarity-label.old { left:14px; } .clarity-label.fresh { right:14px; }
        .clarity-controls { position:absolute; z-index:5; left:0; right:0; bottom:0; height:70px; display:grid; grid-template-columns:auto minmax(120px,560px) auto; align-items:center; justify-content:center; gap:14px; padding:0 18px; background:#17130f; }
        .clarity-controls span { color:rgba(255,255,255,.68); font-size:11px; }
        .clarity-controls input { width:min(56vw,560px); accent-color:#c89567; }
        @media(max-width:640px){ .clarity-header{height:86px;padding:12px 14px}.clarity-header strong{font-size:18px}.clarity-header span{font-size:9px}.clarity-header nav a{display:none}.clarity-stage{inset:86px 0 66px}.clarity-controls{height:66px}.clarity-frame{transform-origin:50% 55%} }
      `}</style>
    </main>
  );
}
