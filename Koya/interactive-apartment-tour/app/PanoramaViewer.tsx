'use client';

import { useEffect, useRef, useState } from 'react';
import * as THREE from 'three';

type PanoramaViewerProps = {
  src: string;
  midResSrc?: string;
  highResSrc?: string;
  yaw: number;
  pitch?: number;
  horizontalFov: number;
};

const MAX_VERTICAL_FOV = 88;

function verticalFov(horizontalFov: number, aspect: number) {
  const projected = THREE.MathUtils.radToDeg(
    2 * Math.atan(Math.tan(THREE.MathUtils.degToRad(horizontalFov) / 2) / aspect),
  );
  return Math.min(MAX_VERTICAL_FOV, projected);
}

export default function PanoramaViewer({ src, midResSrc, highResSrc, yaw, pitch = 0, horizontalFov }: PanoramaViewerProps) {
  const mountRef = useRef<HTMLDivElement | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const frameRef = useRef<number | null>(null);
  const [ready, setReady] = useState(false);

  function renderFrame() {
    if (rendererRef.current && cameraRef.current && sceneRef.current) {
      rendererRef.current.render(sceneRef.current, cameraRef.current);
    }
  }

  useEffect(() => {
    const mount = mountRef.current;
    if (!mount) return;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(55, 1, 0.1, 220);
    camera.rotation.order = 'YXZ';
    const renderer = new THREE.WebGLRenderer({ antialias: true, powerPreference: 'high-performance' });
    renderer.outputColorSpace = THREE.SRGBColorSpace;
    renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2.5));
    mount.appendChild(renderer.domElement);

    sceneRef.current = scene;
    cameraRef.current = camera;
    rendererRef.current = renderer;

    const geometry = new THREE.SphereGeometry(100, 192, 128);
    geometry.scale(-1, 1, 1);
    const material = new THREE.MeshBasicMaterial({ color: 0xffffff });
    scene.add(new THREE.Mesh(geometry, material));

    const resize = () => {
      const width = Math.max(1, mount.clientWidth);
      const height = Math.max(1, mount.clientHeight);
      camera.aspect = width / height;
      camera.fov = verticalFov(horizontalFov, camera.aspect);
      camera.updateProjectionMatrix();
      renderer.setSize(width, height, false);
      renderFrame();
    };
    const observer = new ResizeObserver(resize);
    observer.observe(mount);
    resize();

    const loader = new THREE.TextureLoader();
    const preferredSrc = highResSrc && renderer.capabilities.maxTextureSize >= 12288
      ? highResSrc
      : midResSrc && renderer.capabilities.maxTextureSize >= 10240
        ? midResSrc
        : src;
    let texture: THREE.Texture | null = null;
    let disposed = false;

    const loadTexture = (candidate: string) => {
      texture = loader.load(
        candidate,
        (loaded) => {
          if (disposed) {
            loaded.dispose();
            return;
          }
          loaded.colorSpace = THREE.SRGBColorSpace;
          loaded.wrapS = THREE.RepeatWrapping;
          loaded.generateMipmaps = false;
          loaded.minFilter = THREE.LinearFilter;
          loaded.magFilter = THREE.LinearFilter;
          loaded.anisotropy = renderer.capabilities.getMaxAnisotropy();
          material.map = loaded;
          material.needsUpdate = true;
          mount.dataset.textureSrc = candidate;
          mount.dataset.textureWidth = String(loaded.image.width);
          mount.dataset.textureHeight = String(loaded.image.height);
          setReady(true);
          renderFrame();
        },
        undefined,
        () => {
          if (candidate !== src) {
            texture?.dispose();
            loadTexture(src);
          } else setReady(false);
        },
      );
    };

    loadTexture(preferredSrc);

    return () => {
      disposed = true;
      observer.disconnect();
      if (frameRef.current !== null) window.cancelAnimationFrame(frameRef.current);
      texture?.dispose();
      material.dispose();
      geometry.dispose();
      renderer.dispose();
      renderer.domElement.remove();
      rendererRef.current = null;
      cameraRef.current = null;
      sceneRef.current = null;
    };
  }, [highResSrc, midResSrc, src]);

  useEffect(() => {
    const mount = mountRef.current;
    const camera = cameraRef.current;
    if (!mount || !camera) return;
    camera.rotation.y = THREE.MathUtils.degToRad(yaw);
    camera.rotation.x = THREE.MathUtils.degToRad(Math.max(-58, Math.min(58, pitch)));
    camera.aspect = Math.max(1, mount.clientWidth) / Math.max(1, mount.clientHeight);
    camera.fov = verticalFov(horizontalFov, camera.aspect);
    camera.updateProjectionMatrix();
    if (frameRef.current !== null) window.cancelAnimationFrame(frameRef.current);
    frameRef.current = window.requestAnimationFrame(renderFrame);
  }, [horizontalFov, pitch, yaw]);

  return (
    <div ref={mountRef} className={`panorama-webgl continuous ${ready ? 'ready' : ''}`} style={{ backgroundImage: `url(${src})` }}>
      {!ready && <span>Preparing continuous panorama…</span>}
    </div>
  );
}
