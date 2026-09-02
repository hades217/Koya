const $ = (selector) => document.querySelector(selector);
const app = $('#app');
const clamp = (value, min, max) => Math.min(max, Math.max(min, value));
const escapeHtml = (value) => String(value).replace(/[&<>"']/g, (character) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' })[character]);
let manifest;
let state;
let viewer;
let renderSequence = 0;
let analyticsState = { unitId: null, roomId: null, mode: null, panoramaInteracted: false };
const analyticsSessionId = globalThis.crypto?.randomUUID?.() ?? Array.from(globalThis.crypto?.getRandomValues?.(new Uint8Array(16)) ?? []).map((value) => value.toString(16).padStart(2, '0')).join('');
const analyticsEvents = new Set(['tour_loaded', 'unit_viewed', 'room_viewed', 'mode_viewed', 'panorama_interaction', 'video_started', 'fullscreen_changed', 'still_fallback_used']);

function track(event, context = {}) {
  if (!analyticsEvents.has(event) || !manifest?.project?.id) return;
  const payload = {
    schemaVersion: 1,
    event,
    sessionId: analyticsSessionId,
    projectId: manifest.project.id,
    unitId: context.unitId ?? state?.unit?.id ?? null,
    roomId: context.roomId ?? state?.room?.id ?? null,
    mode: context.mode ?? state?.mode ?? null,
    timestamp: Date.now(),
  };
  dispatchEvent(new CustomEvent('estate-studio:analytics', { detail: payload }));
  if (!manifest.analytics?.enabled || !manifest.analytics.endpoint || location.protocol !== 'https:') return;
  try {
    const endpoint = new URL(manifest.analytics.endpoint, location.href);
    if (endpoint.origin !== location.origin || endpoint.protocol !== 'https:') return;
    const body = JSON.stringify(payload);
    if (!navigator.sendBeacon?.(endpoint, new Blob([body], { type: 'application/json' }))) {
      fetch(endpoint, { method: 'POST', body, headers: { 'content-type': 'application/json' }, keepalive: true, credentials: 'omit' }).catch(() => {});
    }
  } catch { /* Invalid or cross-origin analytics configuration stays disabled. */ }
}

function announce(message) {
  $('#runtime-status').textContent = '';
  requestAnimationFrame(() => { $('#runtime-status').textContent = message; });
}

function route() {
  const query = new URLSearchParams(location.search);
  const unit = manifest.units.find((item) => item.id === query.get('unit')) ?? manifest.units[0];
  const room = unit.rooms.find((item) => item.id === query.get('room')) ?? unit.rooms[0];
  let mode = query.get('mode') ?? 'panorama';
  if (mode === 'video' && !room.video) mode = 'panorama';
  if (mode === 'still' && !room.stillFallback) mode = 'panorama';
  const requestedTexture = query.get('texture');
  const texture = ['mobile', '4k', '8k'].includes(requestedTexture) ? requestedTexture : 'auto';
  const dpr = query.get('dpr') === '1' ? 1 : query.get('dpr') === '2' ? 2 : null;
  return { unit, room, mode, texture, dpr };
}

function navigate(next, push = true) {
  state = { ...state, ...next };
  const query = new URLSearchParams({
    unit: state.unit.id,
    room: state.room.id,
    mode: state.mode,
    texture: state.texture,
    ...(state.dpr ? { dpr: String(state.dpr) } : {}),
  });
  history[push ? 'pushState' : 'replaceState'](null, '', `${location.pathname}?${query}`);
  render().catch(fatal);
}

class PanoramaViewer {
  constructor(canvas) {
    this.canvas = canvas;
    this.gl = canvas.getContext('webgl', { antialias: false, preserveDrawingBuffer: false });
    if (!this.gl) throw new Error('WebGL is unavailable. Use the still-image mode if one is provided.');
    this.yaw = 0;
    this.pitch = 0;
    this.fov = 75;
    this.drag = null;
    this.setup();
    this.bind();
  }

  setup() {
    const gl = this.gl;
    const shader = (type, source) => {
      const item = gl.createShader(type);
      gl.shaderSource(item, source);
      gl.compileShader(item);
      if (!gl.getShaderParameter(item, gl.COMPILE_STATUS)) throw new Error(gl.getShaderInfoLog(item));
      return item;
    };
    const program = gl.createProgram();
    gl.attachShader(program, shader(gl.VERTEX_SHADER, 'attribute vec2 p;varying vec2 uv;void main(){uv=p;gl_Position=vec4(p,0.,1.);}'));
    gl.attachShader(program, shader(gl.FRAGMENT_SHADER, 'precision mediump float;varying vec2 uv;uniform sampler2D image;uniform vec2 rotation;uniform float fov;uniform float aspect;const float PI=3.14159265359;void main(){float scale=tan(radians(fov)*.5);vec3 ray=normalize(vec3(uv.x*scale*aspect,-uv.y*scale,-1.));float cy=cos(rotation.x),sy=sin(rotation.x),cp=cos(rotation.y),sp=sin(rotation.y);ray=vec3(cy*ray.x+sy*ray.z,ray.y,-sy*ray.x+cy*ray.z);ray=vec3(ray.x,cp*ray.y-sp*ray.z,sp*ray.y+cp*ray.z);vec2 sampleUv=vec2(atan(ray.x,-ray.z)/(2.*PI)+.5,asin(clamp(ray.y,-1.,1.))/PI+.5);gl_FragColor=texture2D(image,sampleUv);}'));
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) throw new Error(gl.getProgramInfoLog(program));
    gl.useProgram(program);
    const buffer = gl.createBuffer();
    gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]), gl.STATIC_DRAW);
    const position = gl.getAttribLocation(program, 'p');
    gl.enableVertexAttribArray(position);
    gl.vertexAttribPointer(position, 2, gl.FLOAT, false, 0, 0);
    this.program = program;
    this.texture = gl.createTexture();
  }

  bind() {
    const canvas = this.canvas;
    canvas.addEventListener('pointerdown', (event) => {
      canvas.setPointerCapture(event.pointerId);
      this.drag = { x: event.clientX, y: event.clientY, yaw: this.yaw, pitch: this.pitch };
      if (!analyticsState.panoramaInteracted) {
        analyticsState.panoramaInteracted = true;
        track('panorama_interaction');
      }
    });
    canvas.addEventListener('pointermove', (event) => {
      if (!this.drag) return;
      this.yaw = this.drag.yaw - (event.clientX - this.drag.x) * .005;
      this.pitch = clamp(this.drag.pitch + (event.clientY - this.drag.y) * .004, -1.35, 1.35);
      this.draw();
    });
    const endDrag = () => { this.drag = null; };
    canvas.addEventListener('pointerup', endDrag);
    canvas.addEventListener('pointercancel', endDrag);
    canvas.addEventListener('wheel', (event) => {
      event.preventDefault();
      this.fov = clamp(this.fov + event.deltaY * .03, 35, 100);
      this.draw();
    }, { passive: false });
    canvas.addEventListener('keydown', (event) => {
      const delta = event.shiftKey ? .18 : .06;
      let handled = true;
      if (event.key === 'ArrowLeft') this.yaw -= delta;
      else if (event.key === 'ArrowRight') this.yaw += delta;
      else if (event.key === 'ArrowUp') this.pitch = clamp(this.pitch + delta, -1.35, 1.35);
      else if (event.key === 'ArrowDown') this.pitch = clamp(this.pitch - delta, -1.35, 1.35);
      else if (event.key === '+' || event.key === '=') this.fov = clamp(this.fov - 5, 35, 100);
      else if (event.key === '-' || event.key === '_') this.fov = clamp(this.fov + 5, 35, 100);
      else if (event.key === 'Home') this.reset();
      else handled = false;
      if (handled) {
        event.preventDefault();
        this.draw();
      }
    });
    canvas.addEventListener('webglcontextlost', (event) => {
      event.preventDefault();
      fatal(new Error('The 360 viewer ran out of graphics memory. Reload in mobile texture mode or use the still image.'));
    });
    addEventListener('resize', () => this.draw());
  }

  async load(url, yaw = 0) {
    const image = new Image();
    image.decoding = 'async';
    await new Promise((resolve, reject) => {
      image.onload = resolve;
      image.onerror = () => reject(new Error(`Unable to load texture: ${url}`));
      image.src = url;
    });
    const gl = this.gl;
    gl.bindTexture(gl.TEXTURE_2D, this.texture);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.REPEAT);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, image);
    this.yaw = yaw * Math.PI / 180;
    this.pitch = 0;
    this.fov = 75;
    this.draw();
  }

  reset() {
    this.yaw = (state.room.initialYaw ?? 0) * Math.PI / 180;
    this.pitch = 0;
    this.fov = 75;
    this.draw();
    announce('360 view reset.');
  }

  draw() {
    const gl = this.gl;
    const dpr = Math.min(state?.dpr ?? devicePixelRatio ?? 1, state?.texture === 'mobile' ? 1 : 2);
    this.canvas.width = Math.round(this.canvas.clientWidth * dpr);
    this.canvas.height = Math.round(this.canvas.clientHeight * dpr);
    if (!this.canvas.width || !this.canvas.height) return;
    gl.viewport(0, 0, this.canvas.width, this.canvas.height);
    gl.uniform2f(gl.getUniformLocation(this.program, 'rotation'), this.yaw, this.pitch);
    gl.uniform1f(gl.getUniformLocation(this.program, 'fov'), this.fov);
    gl.uniform1f(gl.getUniformLocation(this.program, 'aspect'), this.canvas.width / this.canvas.height);
    gl.drawArrays(gl.TRIANGLES, 0, 6);
  }
}

function textureChoice(room) {
  const tiers = room.panorama?.tiers ?? [];
  const exact = (kind) => tiers.find((tier) => tier.kind === kind);
  const lowMemory = (navigator.deviceMemory && navigator.deviceMemory <= 4) || matchMedia('(max-width:720px)').matches;
  let kind = state.texture;
  if (kind === 'auto') kind = lowMemory ? 'mobile' : ((navigator.deviceMemory ?? 8) >= 8 && innerWidth * devicePixelRatio >= 2500 ? '8k' : '4k');
  const tier = exact(kind) ?? exact(kind === '8k' ? '4k' : 'mobile') ?? exact('mobile') ?? exact('4k') ?? exact('8k');
  return { url: tier?.path ?? room.panorama?.source, kind: tier?.kind ?? 'source' };
}

function roomButtons(selector, unit, room) {
  document.querySelectorAll(selector).forEach((button) => {
    const target = unit.rooms.find((item) => item.id === button.dataset.room);
    button.setAttribute('aria-label', `Open ${target?.name ?? 'room'}`);
    button.setAttribute('aria-current', target?.id === room.id ? 'true' : 'false');
    button.onclick = () => navigate({ room: target, mode: 'panorama' });
  });
}

async function render() {
  const sequence = ++renderSequence;
  const { unit, room, mode } = state;
  $('#project-name').textContent = manifest.project.name;
  $('#company').textContent = manifest.project.company;
  $('#unit-label').textContent = unit.label;
  $('#room-name').textContent = room.name;
  $('#room-status').textContent = room.evidenceLabel;
  document.documentElement.style.setProperty('--primary', manifest.theme.primary);
  document.documentElement.style.setProperty('--accent', manifest.theme.accent);
  $('#disclosure p').textContent = manifest.project.disclosure;
  $('#unit-select').innerHTML = manifest.units.map((item) => `<option value="${escapeHtml(item.id)}" ${item.id === unit.id ? 'selected' : ''}>${escapeHtml(item.label)}</option>`).join('');
  $('#room-tabs').innerHTML = unit.rooms.map((item) => `<button type="button" data-room="${escapeHtml(item.id)}" class="${item.id === room.id ? 'active' : ''}">${escapeHtml(item.name)}</button>`).join('');
  $('#adjacent-rooms').innerHTML = room.adjacentRoomIds.map((id) => unit.rooms.find((item) => item.id === id)).filter(Boolean).map((item) => `<button type="button" data-room="${escapeHtml(item.id)}">Go to ${escapeHtml(item.name)}</button>`).join('');
  $('#floorplan').src = unit.floorplan;
  $('#floorplan').alt = `${unit.label} floor plan`;
  $('#hotspots').innerHTML = unit.rooms.map((item) => `<button type="button" data-room="${escapeHtml(item.id)}" class="${item.id === room.id ? 'active' : ''}" style="left:${Number(item.hotspot.x)}%;top:${Number(item.hotspot.y)}%">${escapeHtml(item.name)}</button>`).join('');
  roomButtons('[data-room]', unit, room);
  document.querySelectorAll('[data-mode]').forEach((button) => {
    button.hidden = (button.dataset.mode === 'video' && !room.video) || (button.dataset.mode === 'still' && !room.stillFallback);
    button.classList.toggle('active', button.dataset.mode === mode);
    button.setAttribute('aria-pressed', button.dataset.mode === mode ? 'true' : 'false');
    button.onclick = () => navigate({ mode: button.dataset.mode });
  });
  const canvas = $('#panorama');
  const still = $('#still-fallback');
  const video = $('#room-video');
  canvas.hidden = mode !== 'panorama';
  still.hidden = mode !== 'still';
  video.hidden = mode !== 'video';
  video.pause();
  if (mode === 'video') {
    video.src = room.video;
    video.poster = room.stillFallback ?? '';
  } else if (mode === 'still') {
    still.src = room.stillFallback;
    still.alt = `${room.name} still image fallback`;
  } else {
    const texture = textureChoice(room);
    try {
      await viewer.load(texture.url, room.initialYaw ?? 0);
      if (sequence !== renderSequence) return;
      canvas.dataset.textureTier = texture.kind;
    } catch (error) {
      if (room.stillFallback) {
        track('still_fallback_used');
        state.mode = 'still';
        const query = new URLSearchParams({ unit: unit.id, room: room.id, mode: 'still', texture: state.texture, ...(state.dpr ? { dpr: String(state.dpr) } : {}) });
        history.replaceState(null, '', `?${query}`);
        return render();
      }
      throw error;
    }
  }
  $('#loading').hidden = true;
  app.setAttribute('aria-busy', 'false');
  if (analyticsState.unitId !== unit.id) track('unit_viewed', { unitId: unit.id, roomId: null, mode: null });
  if (analyticsState.roomId !== room.id) track('room_viewed');
  if (analyticsState.mode !== mode) track('mode_viewed');
  analyticsState = { ...analyticsState, unitId: unit.id, roomId: room.id, mode };
  announce(`${unit.label}, ${room.name}, ${mode} mode loaded.`);
}

async function toggleFullscreen() {
  if (!document.fullscreenElement) await app.requestFullscreen();
  else await document.exitFullscreen();
}

function syncFullscreen() {
  const active = Boolean(document.fullscreenElement);
  $('#fullscreen').textContent = active ? 'Exit fullscreen' : 'Fullscreen';
  $('#fullscreen').setAttribute('aria-pressed', String(active));
  announce(active ? 'Fullscreen enabled.' : 'Fullscreen exited.');
  track('fullscreen_changed', { mode: active ? 'fullscreen' : state?.mode });
}

async function boot() {
  try {
    manifest = globalThis.__ESTATE_TOUR_MANIFEST__ ?? await fetch('tour-manifest.json', { cache: 'no-store' }).then((response) => {
      if (!response.ok) throw new Error(`Manifest HTTP ${response.status}`);
      return response.json();
    });
    if (manifest.schemaVersion !== 1 || !manifest.units?.length) throw new Error('Tour manifest is invalid.');
    viewer = new PanoramaViewer($('#panorama'));
    state = route();
    track('tour_loaded');
    $('#unit-select').onchange = (event) => {
      const unit = manifest.units.find((item) => item.id === event.target.value);
      navigate({ unit, room: unit.rooms[0], mode: 'panorama' });
    };
    $('#reset-view').onclick = () => viewer.reset();
    $('#fullscreen').hidden = typeof app.requestFullscreen !== 'function';
    $('#fullscreen').onclick = () => toggleFullscreen().catch(fatal);
    document.addEventListener('fullscreenchange', syncFullscreen);
    $('#room-video').addEventListener('play', () => track('video_started'));
    onpopstate = () => { state = route(); render().catch(fatal); };
    document.addEventListener('keydown', (event) => {
      if (event.key === 'Escape' && document.fullscreenElement) document.exitFullscreen().catch(fatal);
    });
    if (matchMedia('(prefers-reduced-motion: reduce)').matches) document.body.classList.add('reduced-motion');
    navigate(state, false);
  } catch (error) {
    fatal(error);
  }
}

function fatal(error) {
  $('#loading').hidden = true;
  $('#fatal').hidden = false;
  $('#fatal').textContent = error instanceof Error ? error.message : String(error);
  app.setAttribute('aria-busy', 'false');
}

boot();
