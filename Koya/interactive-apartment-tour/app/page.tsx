'use client';

import { type WheelEvent, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import PanoramaViewer from './PanoramaViewer';

type SceneView = { label: string; image: string; panorama360?: boolean; midResImage?: string; highResImage?: string; initialYaw?: number };

type TourStop = {
  id: string;
  label: string;
  eyebrow: string;
  views: SceneView[];
  x: number;
  y: number;
  connections: string[];
  note: string;
};
type VideoTour = {
  id: string;
  label: string;
  route: string;
  status: string;
  src: string;
  poster: string;
  duration: number;
  rejected?: boolean;
  phases: { at: number; label: string; activeId: string }[];
};
type UnitOption = {
  id: string;
  label: string;
  summary: string;
  totalArea: string;
  tourAvailable: boolean;
};
type TourUnit = UnitOption & {
  planImage: string;
  stops: TourStop[];
  videoTours: VideoTour[];
};

const units: UnitOption[] = [
  { id: '106', label: 'Apartment 106', summary: '2 bed · 2 bath · 1 MPR', totalArea: '152 m²', tourAvailable: true },
  { id: '102', label: 'Apartment 102', summary: '2 bed · 2 bath', totalArea: '104 m²', tourAvailable: false },
  { id: '103', label: 'Apartment 103', summary: '1 bed · study · 1 bath', totalArea: '86 m²', tourAvailable: false },
  { id: '104', label: 'Apartment 104', summary: '2 bed · 2 bath', totalArea: '106 m²', tourAvailable: false },
];

const stops: TourStop[] = [
  { id: 'entry', label: 'Entry hall', eyebrow: 'Arrival', views: [{ label: 'Entry node · facing inward', image: '/tour/entry-floorplan-audited-v2.png' }, { label: 'Forward node · kitchen axis', image: '/tour/entry-kitchen-turn-audited-v1.png' }], x: 4, y: 13, connections: ['bath', 'hub'], note: 'The Entry free-explore fallback uses floor-plan-audited nodes only. The inconsistent generated 360° view has been removed. Use the default vertical-scroll Entry video for the literal threshold movement.' },
  { id: 'bath', label: 'Bath & laundry', eyebrow: 'Service rooms', views: [{ label: 'Threshold', image: '/tour/bath-threshold.png' }, { label: 'Main bath', image: '/tour/bath.png' }], x: 23, y: 31, connections: ['entry', 'bedroom2'], note: 'Separate bath and laundry sit beside the hall.' },
  { id: 'bedroom2', label: 'Bedroom 2', eyebrow: 'Private room', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/bedroom2-panorama-4k.webp', midResImage: '/tour/panoramas/bedroom2-panorama-8k.webp', panorama360: true }, { label: 'Threshold', image: '/tour/bedroom2-threshold.png' }, { label: 'Room view', image: '/tour/bedroom2.png' }], x: 21, y: 68, connections: ['bath', 'mpr'], note: 'Bedroom 2 opens from the western circulation edge. The panorama is a floor-plan-grounded AI concept, not a measured 360° survey.' },
  { id: 'mpr', label: 'Multipurpose room', eyebrow: 'Flexible space', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/mpr-panorama-4k.webp', midResImage: '/tour/panoramas/mpr-panorama-8k.webp', panorama360: true }, { label: 'From dining', image: '/tour/mpr-threshold.png' }, { label: 'Room view', image: '/tour/mpr.png' }], x: 35, y: 67, connections: ['bedroom2', 'hub'], note: 'The compact MPR connects through the large dining-side opening. The panorama is a floor-plan-grounded AI concept, not a measured 360° survey.' },
  { id: 'hub', label: 'Kitchen & dining', eyebrow: 'Living hub', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/kitchen-dining-panorama-4k.webp', midResImage: '/tour/panoramas/kitchen-dining-panorama-8k.webp', panorama360: true }, { label: 'West · MPR', image: '/tour/hub-west.png' }, { label: 'North · Kitchen', image: '/tour/kitchen.png' }, { label: 'East · Living', image: '/tour/living.png' }], x: 56, y: 55, connections: ['entry', 'mpr', 'living', 'bedroom1'], note: 'One island anchors the open-plan kitchen and dining zone. The panorama is a floor-plan-grounded AI concept, not a measured 360° survey.' },
  { id: 'living', label: 'Living room', eyebrow: 'Public space', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/living-panorama-4k.webp', midResImage: '/tour/local-clarity/living-panorama-realesrgan-8k.webp', panorama360: true }, { label: 'Original 360° comparison', image: '/tour/living-panorama-ai-360-v3-edsr-7k.webp', panorama360: true, midResImage: '/tour/living-panorama-ai-360-v3-sharp-10k.webp', highResImage: '/tour/living-panorama-ai-360-v3-desktop-12k.webp' }, { label: 'AI stitched wide', image: '/tour/living-panorama-ai-v1.png' }, { label: 'Original living', image: '/tour/living.png' }, { label: 'Terrace threshold', image: '/tour/terrace-threshold.png' }], x: 73, y: 68, connections: ['hub', 'terrace', 'bedroom1'], note: 'The living room faces the internal display wall and wraparound glazing. The enhanced panorama is a deterministic super-resolution concept composite, not a measured 360° survey.' },
  { id: 'bedroom1', label: 'Bedroom 1', eyebrow: 'Primary suite', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/bedroom1-panorama-4k.webp', midResImage: '/tour/panoramas/bedroom1-panorama-8k.webp', panorama360: true }, { label: 'Threshold', image: '/tour/bedroom1-threshold.png' }, { label: 'Bedroom', image: '/tour/bedroom1.png' }, { label: 'Walk-in robe', image: '/tour/wir.png' }], x: 70, y: 30, connections: ['hub', 'ensuite', 'living'], note: 'The primary bedroom connects to the WIR and ensuite. The panorama is a floor-plan-grounded AI concept, not a measured 360° survey.' },
  { id: 'ensuite', label: 'WIR & ensuite', eyebrow: 'Primary suite', views: [{ label: 'Walk-in robe', image: '/tour/wir.png' }, { label: 'Ensuite', image: '/tour/ensuite.png' }], x: 45, y: 29, connections: ['bedroom1'], note: 'The ensuite is reached through Bedroom 1 and the WIR.' },
  { id: 'terrace', label: 'Private terrace', eyebrow: 'Outdoor room', views: [{ label: 'Enhanced 360° panorama', image: '/tour/panoramas/terrace-panorama-4k.webp', midResImage: '/tour/panoramas/terrace-panorama-8k.webp', panorama360: true }, { label: 'Threshold', image: '/tour/terrace-threshold.png' }, { label: 'Terrace', image: '/tour/terrace.png' }], x: 87, y: 68, connections: ['living'], note: 'The private terrace wraps the living and dining edges. The panorama and outlook are floor-plan-grounded AI concepts, not a measured 360° survey or verified view.' },
];

const videoTours: VideoTour[] = [
  {
    id: 'arrival', label: 'Arrival', route: 'Koya arrival → foyer → lift → Apartment 106 door', status: 'QA passed · Building arrival sequence',
    src: '/tour/videos/full-arrival-entry-qa-pass-scroll.mp4', poster: '/tour/videos/full-arrival-entry-poster.jpg', duration: 21.056,
    phases: [
      { at: 0, label: 'Koya arrival', activeId: 'entry' },
      { at: .19, label: 'Building entrance', activeId: 'entry' },
      { at: .38, label: 'Lift lobby', activeId: 'entry' },
      { at: .57, label: 'Lift exit', activeId: 'entry' },
      { at: .71, label: 'Apartment 106 approach', activeId: 'entry' },
      { at: .84, label: 'Apartment 106 threshold', activeId: 'entry' },
      { at: .95, label: 'Entry Hall', activeId: 'entry' },
    ],
  },
  {
    id: 'entry-room', label: 'Entry room', route: 'Inside Apartment 106 → Entry Hall → Living Hub', status: 'Standalone interior video · Concept geometry under review',
    src: '/tour/videos/entry-hub-archived-rejected-scroll.mp4', poster: '/tour/entry.png', duration: 10.041667,
    phases: [
      { at: 0, label: 'Entry Hall', activeId: 'entry' },
      { at: .32, label: 'Hall turn', activeId: 'entry' },
      { at: .56, label: 'Kitchen passage', activeId: 'hub' },
      { at: .82, label: 'Living Hub', activeId: 'living' },
    ],
  },
  {
    id: 'west-rooms', label: 'West rooms', route: 'Entry → wet area → Bedroom 2 → Kitchen → MPR', status: 'Internal QA · Review master',
    src: '/tour/videos/west-rooms-review-seek-optimized.mp4', poster: '/tour/entry.png', duration: 56.666667,
    phases: [
      { at: 0, label: 'Arrival & entry', activeId: 'entry' },
      { at: .37, label: 'Bath & laundry', activeId: 'bath' },
      { at: .49, label: 'Bedroom 2 threshold', activeId: 'bedroom2' },
      { at: .58, label: 'Inside Bedroom 2', activeId: 'bedroom2' },
      { at: .68, label: 'Return to kitchen', activeId: 'hub' },
      { at: .78, label: 'Kitchen to MPR', activeId: 'hub' },
      { at: .88, label: 'Inside MPR', activeId: 'mpr' },
    ],
  },
  {
    id: 'bedroom1', label: 'Bedroom 1', route: 'Living Hub → Bedroom 1 → glazing', status: 'Local QA pass · Awaiting final acceptance',
    src: '/tour/videos/bedroom1-review-scroll.mp4', poster: '/tour/bedroom1-threshold.png', duration: 5.041667,
    phases: [{ at: 0, label: 'Living Hub', activeId: 'living' }, { at: .22, label: 'Bedroom 1 threshold', activeId: 'bedroom1' }, { at: .58, label: 'Bedroom 1', activeId: 'bedroom1' }],
  },
  {
    id: 'ensuite', label: 'Primary suite', route: 'Bedroom 1 → WIR → Ensuite', status: 'User accepted',
    src: '/tour/videos/wir-ensuite-accepted-scroll.mp4', poster: '/tour/bedroom1.png', duration: 5.041667,
    phases: [{ at: 0, label: 'Bedroom 1', activeId: 'bedroom1' }, { at: .34, label: 'Walk-in robe', activeId: 'ensuite' }, { at: .7, label: 'Ensuite', activeId: 'ensuite' }],
  },
  {
    id: 'terrace', label: 'Living & terrace', route: 'Living panorama → terrace threshold → return', status: 'Structural reference · QA passed',
    src: '/tour/videos/living-terrace-reference-scroll.mp4', poster: '/tour/living.png', duration: 8,
    phases: [{ at: 0, label: 'Living room', activeId: 'living' }, { at: .3, label: 'Terrace threshold', activeId: 'terrace' }, { at: .66, label: 'Return to Living', activeId: 'living' }],
  },
];

const tourUnits: Record<string, TourUnit> = {
  '106': {
    ...units[0],
    planImage: '/tour/apartment-106-plan.png',
    stops,
    videoTours,
  },
};
const activeUnit = tourUnits['106'];
const activeVideoTours = activeUnit.videoTours.filter((tour) => !tour.rejected);
const panoramaStops = activeUnit.stops.filter((stop) => stop.views.some((view) => view.panorama360));
const defaultPanoramaStop = panoramaStops.find((stop) => stop.id === 'living') ?? panoramaStops[0];

function panoramaViewFor(stop: TourStop) {
  return stop.views.find((view) => view.panorama360);
}

export default function Home() {
  const [activeId, setActiveId] = useState('entry');
  const [planOpen, setPlanOpen] = useState(false);
  const [infoOpen, setInfoOpen] = useState(false);
  const [pan, setPan] = useState(0);
  const [pitch, setPitch] = useState(0);
  const [isDragging, setIsDragging] = useState(false);
  const [visited, setVisited] = useState(() => new Set(['entry']));
  const [guidedMode, setGuidedMode] = useState(true);
  const [selectedTourId, setSelectedTourId] = useState('entry-room');
  const [videoDuration, setVideoDuration] = useState(10.041667);
  const [videoProgress, setVideoProgress] = useState(0);
  const [videoReady, setVideoReady] = useState(false);
  const [videoPlaying, setVideoPlaying] = useState(false);
  const [panoramaFov, setPanoramaFov] = useState(88);
  const [urlReady, setUrlReady] = useState(false);
  const dragStart = useRef({ x: 0, y: 0, pan: 0, pitch: 0 });
  const videoRef = useRef<HTMLVideoElement | null>(null);
  const videoRefs = useRef(new Map<string, HTMLVideoElement>());
  const targetVideoTime = useRef(0);
  const scrubFrame = useRef<number | null>(null);
  const videoDragStart = useRef({ y: 0, time: 0 });
  const videoDragRequested = useRef(0);
  const pendingVideoEdge = useRef<'start' | 'end'>('start');
  const active = useMemo(() => activeUnit.stops.find((stop) => stop.id === activeId) ?? activeUnit.stops[0], [activeId]);
  const activeView = panoramaViewFor(active) ?? panoramaViewFor(defaultPanoramaStop) ?? defaultPanoramaStop.views[0];
  const selectedTour = useMemo(() => activeUnit.videoTours.find((tour) => tour.id === selectedTourId) ?? activeUnit.videoTours[0], [selectedTourId]);
  const selectedTourIndex = activeVideoTours.findIndex((tour) => tour.id === selectedTour.id);
  const guidedPhase = [...selectedTour.phases].reverse().find((phase) => videoProgress >= phase.at) ?? selectedTour.phases[0];

  const scrubTo = useCallback((requestedTime: number) => {
    const video = videoRef.current;
    if (!video || !Number.isFinite(video.duration)) return;
    targetVideoTime.current = Math.max(0, Math.min(video.duration, requestedTime));
    if (scrubFrame.current !== null) return;

    const tick = () => {
      const currentVideo = videoRef.current;
      if (!currentVideo) { scrubFrame.current = null; return; }
      const nextTime = targetVideoTime.current;
      if (Math.abs(nextTime - currentVideo.currentTime) >= .015) currentVideo.currentTime = nextTime;
      scrubFrame.current = null;
    };
    scrubFrame.current = window.requestAnimationFrame(tick);
  }, []);

  useEffect(() => {
    function onKeyDown(event: KeyboardEvent) {
      if (guidedMode && event.key === 'ArrowDown') scrubTo(targetVideoTime.current + .45);
      else if (guidedMode && event.key === 'ArrowUp') scrubTo(targetVideoTime.current - .45);
      else if (!guidedMode && event.key === 'ArrowLeft') setPan((value) => value - 30);
      else if (!guidedMode && event.key === 'ArrowRight') setPan((value) => value + 30);
      else if (!guidedMode && event.key === 'ArrowUp') setPitch((value) => Math.min(58, value + 22));
      else if (!guidedMode && event.key === 'ArrowDown') setPitch((value) => Math.max(-58, value - 22));
      if (event.key === 'Escape') { setInfoOpen(false); setPlanOpen(false); }
    }
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [guidedMode, scrubTo]);

  useEffect(() => {
    function applyUrlState() {
      const params = new URLSearchParams(window.location.search);
      const mode = params.get('mode');
      const tourId = params.get('tour');
      const roomId = params.get('room');

      if (mode === 'panorama') {
        const panoramaStop = panoramaStops.find((stop) => stop.id === roomId) ?? defaultPanoramaStop;
        setGuidedMode(false);
        setActiveId(panoramaStop.id);
        setVisited((current) => new Set(current).add(panoramaStop.id));
        setPan(panoramaViewFor(panoramaStop)?.initialYaw ?? 0);
        setPitch(0);
        setPlanOpen(window.innerWidth > 760);
      } else {
        const tour = activeVideoTours.find((item) => item.id === tourId) ?? activeVideoTours.find((item) => item.id === 'entry-room') ?? activeVideoTours[0];
        setGuidedMode(true);
        setSelectedTourId(tour.id);
        setVideoDuration(tour.duration);
        setActiveId(tour.phases[0].activeId);
      }
      setUrlReady(true);
    }

    applyUrlState();
    window.addEventListener('popstate', applyUrlState);
    return () => window.removeEventListener('popstate', applyUrlState);
  }, []);

  useEffect(() => {
    if (!urlReady) return;
    const url = new URL(window.location.href);
    url.searchParams.set('unit', activeUnit.id);
    url.searchParams.set('mode', guidedMode ? 'video' : 'panorama');
    if (guidedMode) {
      url.searchParams.set('tour', selectedTour.id);
      url.searchParams.delete('room');
    } else {
      url.searchParams.set('room', active.id);
      url.searchParams.delete('tour');
    }
    window.history.replaceState({}, '', url);
  }, [active.id, guidedMode, selectedTour.id, urlReady]);

  useEffect(() => () => {
    if (scrubFrame.current !== null) window.cancelAnimationFrame(scrubFrame.current);
  }, []);

  useEffect(() => {
    if (!guidedMode) return;
    const video = videoRefs.current.get(selectedTour.id);
    if (!video) return;
    videoRef.current = video;

    const syncMetadata = () => {
      if (!Number.isFinite(video.duration) || video.duration <= 0) return;
      video.pause();
      const nextTime = pendingVideoEdge.current === 'end' ? video.duration : 0;
      video.currentTime = nextTime;
      setVideoDuration(video.duration);
      targetVideoTime.current = nextTime;
      setVideoProgress(nextTime / video.duration);
      setVideoReady(video.readyState >= HTMLMediaElement.HAVE_CURRENT_DATA);
      setVideoPlaying(false);
      pendingVideoEdge.current = 'start';
    };

    if (video.readyState >= HTMLMediaElement.HAVE_METADATA) syncMetadata();
    else video.addEventListener('loadedmetadata', syncMetadata, { once: true });
    return () => video.removeEventListener('loadedmetadata', syncMetadata);
  }, [guidedMode, selectedTour.id]);

  function goTo(id: string) {
    const nextStop = panoramaStops.find((stop) => stop.id === id);
    if (!nextStop) return;
    videoRef.current?.pause();
    setGuidedMode(false);
    setActiveId(id);
    setPan(panoramaViewFor(nextStop)?.initialYaw ?? 0);
    setPitch(0);
    setInfoOpen(false);
    setVisited((current) => new Set(current).add(id));
  }

  function showVideoMode() {
    setGuidedMode(true);
    setActiveId(selectedTour.phases[0].activeId);
    setPan(0);
    setPitch(0);
    setPlanOpen(false);
    setVideoPlaying(false);
    setInfoOpen(false);
  }

  function showPanoramaMode() {
    videoRef.current?.pause();
    const phaseStop = panoramaStops.find((stop) => stop.id === guidedPhase.activeId);
    const nextStop = phaseStop ?? defaultPanoramaStop;
    setGuidedMode(false);
    setActiveId(nextStop.id);
    setPan(panoramaViewFor(nextStop)?.initialYaw ?? 0);
    setPitch(0);
    setVideoPlaying(false);
    setInfoOpen(false);
    setPlanOpen(window.innerWidth > 760);
    setVisited((current) => new Set(current).add(nextStop.id));
  }

  function chooseVideoTour(id: string) {
    const nextTour = activeVideoTours.find((tour) => tour.id === id);
    if (!nextTour || nextTour.id === selectedTourId) return;
    pendingVideoEdge.current = 'start';
    activateVideoTour(nextTour);
  }

  function activateVideoTour(nextTour: VideoTour) {
    videoRef.current?.pause();
    setSelectedTourId(nextTour.id);
    setVideoDuration(nextTour.duration);
    setVideoProgress(0);
    setVideoReady(false);
    setVideoPlaying(false);
    setActiveId(nextTour.phases[0].activeId);
    setVisited((current) => new Set(current).add(nextTour.phases[0].activeId));
    targetVideoTime.current = 0;
    if (scrubFrame.current !== null) {
      window.cancelAnimationFrame(scrubFrame.current);
      scrubFrame.current = null;
    }
  }

  function changeVideoTour(direction: number, edge: 'start' | 'end') {
    const currentIndex = activeVideoTours.findIndex((tour) => tour.id === selectedTourId);
    const nextTour = activeVideoTours[currentIndex + direction];
    if (!nextTour) return false;
    pendingVideoEdge.current = edge;
    activateVideoTour(nextTour);
    return true;
  }

  function handleWheel(event: WheelEvent<HTMLElement>) {
    if (!guidedMode) return;
    event.preventDefault();
    if (event.deltaY > 0 && targetVideoTime.current >= videoDuration - .04 && changeVideoTour(1, 'start')) return;
    if (event.deltaY < 0 && targetVideoTime.current <= .04 && changeVideoTour(-1, 'end')) return;
    scrubTo(targetVideoTime.current + event.deltaY * .0065);
  }

  function startDrag(clientX: number, clientY: number) {
    if (guidedMode) {
      videoRef.current?.pause();
      setVideoPlaying(false);
      videoDragStart.current = { y: clientY, time: targetVideoTime.current };
      videoDragRequested.current = targetVideoTime.current;
      setIsDragging(true);
      return;
    }
    setIsDragging(true);
    dragStart.current = { x: clientX, y: clientY, pan, pitch };
  }

  function moveDrag(clientX: number, clientY: number) {
    if (!isDragging) return;
    if (guidedMode) {
      const travel = (videoDragStart.current.y - clientY) / Math.max(window.innerHeight, 480);
      const requestedTime = videoDragStart.current.time + travel * videoDuration;
      videoDragRequested.current = requestedTime;
      scrubTo(requestedTime);
      return;
    }
    if (activeView.panorama360) {
      const degrees = (clientX - dragStart.current.x) / Math.max(window.innerWidth, 480) * panoramaFov;
      setPan(dragStart.current.pan + degrees);
      const verticalDegrees = (dragStart.current.y - clientY) / Math.max(window.innerHeight, 640) * 120;
      setPitch(Math.max(-90, Math.min(90, dragStart.current.pitch + verticalDegrees)));
      return;
    }
    const wideView = activeView.label === 'AI stitched wide';
    const limit = wideView ? 40 : 10;
    const sensitivity = wideView ? 110 : 44;
    const delta = (clientX - dragStart.current.x) / Math.max(window.innerWidth, 480) * sensitivity;
    setPan(Math.max(-limit, Math.min(limit, dragStart.current.pan + delta)));
  }

  function endDrag() {
    if (guidedMode) {
      if (videoDragRequested.current > videoDuration + .2) changeVideoTour(1, 'start');
      else if (videoDragRequested.current < -.2) changeVideoTour(-1, 'end');
    }
    setIsDragging(false);
  }

  async function toggleVideoPlayback() {
    const video = videoRef.current;
    if (!video || !videoReady) return;
    if (video.paused) {
      try {
        await video.play();
        setVideoPlaying(true);
      } catch {
        setVideoPlaying(false);
      }
    } else {
      video.pause();
      setVideoPlaying(false);
    }
  }

  async function toggleFullscreen() {
    if (document.fullscreenElement) await document.exitFullscreen();
    else await document.documentElement.requestFullscreen();
  }

  return (
    <main className={`tour-shell ${guidedMode ? 'guided video-guided' : 'free'}`} onWheel={handleWheel}>
      <section
        className={`scene ${isDragging ? 'dragging' : ''}`}
        tabIndex={0}
        onPointerDown={(event) => { event.currentTarget.setPointerCapture(event.pointerId); startDrag(event.clientX, event.clientY); }}
        onPointerMove={(event) => moveDrag(event.clientX, event.clientY)}
        onPointerUp={endDrag}
        onPointerCancel={() => setIsDragging(false)}
        aria-label={guidedMode ? `Swipe-controlled ${selectedTour.label} video` : `${active.label}, interactive 360 degree concept panorama`}
      >
        {guidedMode ? activeVideoTours.map((tour, index) => {
          const isActive = tour.id === selectedTour.id;
          const preload = index === selectedTourIndex || index === selectedTourIndex + 1
            ? 'auto'
            : index === selectedTourIndex - 1 ? 'metadata' : 'none';
          return (
            <video
              key={tour.id}
              ref={(node) => {
                if (node) videoRefs.current.set(tour.id, node);
                else videoRefs.current.delete(tour.id);
              }}
              className={`scrub-video ${isActive ? 'active' : 'inactive'}`}
              src={tour.src}
              poster={tour.poster}
              muted
              playsInline
              disablePictureInPicture
              preload={preload}
              aria-hidden={!isActive}
              onCanPlay={() => { if (isActive) setVideoReady(true); }}
              onPlay={() => { if (isActive) setVideoPlaying(true); }}
              onPause={() => { if (isActive) setVideoPlaying(false); }}
              onEnded={() => { if (isActive) setVideoPlaying(false); }}
              onTimeUpdate={(event) => {
                if (!isActive) return;
                const video = event.currentTarget;
                if (scrubFrame.current === null) targetVideoTime.current = video.currentTime;
                const progress = video.duration ? video.currentTime / video.duration : 0;
                setVideoProgress(progress);
                const phase = [...tour.phases].reverse().find((item) => progress >= item.at) ?? tour.phases[0];
                setActiveId(phase.activeId);
                setVisited((current) => new Set(current).add(phase.activeId));
              }}
            />
          );
        }) : (
          <PanoramaViewer
            src={activeView.image}
            midResSrc={activeView.midResImage}
            highResSrc={activeView.highResImage}
            yaw={pan}
            pitch={pitch}
            horizontalFov={panoramaFov}
          />
        )}
        <div className="scene-shade" />
        {guidedMode && !videoReady && <div className="video-loading">Preparing local walkthrough…</div>}
      </section>

      <header className="topbar">
        <div className="brand" aria-label="Koya">
          <span className="brand-mark">koya</span>
          <span className="brand-rule" />
          <span className="brand-subtitle">{activeUnit.label} · interactive concept</span>
        </div>
        <div className="top-actions">
          <button className={`glass-button mode-toggle ${guidedMode ? 'active' : ''}`} onClick={showVideoMode}>Video</button>
          <button className={`glass-button mode-toggle ${!guidedMode ? 'active' : ''}`} onClick={showPanoramaMode}>360° panorama</button>
          <button className="glass-button" onClick={toggleFullscreen} aria-label="Toggle fullscreen">Full screen</button>
          <button className="glass-button" onClick={() => setInfoOpen((value) => !value)} aria-expanded={infoOpen}>About</button>
          <button className="glass-button plan-toggle" onClick={() => setPlanOpen((value) => !value)} aria-expanded={planOpen}>Floor plan</button>
        </div>
      </header>

      {guidedMode && (
        <div className="scroll-status" aria-live="polite">
          <span>Video scrub</span>
          <div><i style={{ height: `${videoProgress * 100}%` }} /></div>
          <strong>{Math.round(videoProgress * 100)}%</strong>
          <small>{videoDuration.toFixed(1)}s</small>
        </div>
      )}

      {guidedMode && (
        <div className="video-transport" aria-label="Video playback controls">
          <button type="button" onClick={toggleVideoPlayback} disabled={!videoReady} aria-label={videoPlaying ? 'Pause video' : 'Play video'}>
            {videoPlaying ? 'Pause' : 'Play'}
          </button>
          <input
            type="range"
            min="0"
            max="1000"
            value={Math.round(videoProgress * 1000)}
            onChange={(event) => {
              videoRef.current?.pause();
              setVideoPlaying(false);
              scrubTo(Number(event.currentTarget.value) / 1000 * videoDuration);
            }}
            aria-label="Video timeline"
          />
          <span>{(videoProgress * videoDuration).toFixed(1)}s / {videoDuration.toFixed(1)}s</span>
        </div>
      )}

      {guidedMode && (
        <nav className="video-route-selector" aria-label="Choose a local video route">
          {activeVideoTours.map((tour) => (
            <button key={tour.id} className={`${tour.id === selectedTourId ? 'active' : ''} ${tour.rejected ? 'rejected' : ''}`.trim()} onClick={() => chooseVideoTour(tour.id)}>
              <span>{tour.label}</span><small>{tour.duration.toFixed(tour.duration > 10 ? 0 : 1)}s</small>
            </button>
          ))}
        </nav>
      )}

      <div className="room-title" aria-live="polite">
        <span>{guidedMode ? `${selectedTour.status} · scroll controlled` : active.eyebrow}</span>
        <h1>{guidedMode ? guidedPhase.label : active.label}</h1>
        <p>{guidedMode ? `${(videoProgress * videoDuration).toFixed(1)}s / ${videoDuration.toFixed(1)}s · ${selectedTour.route}` : `${activeView.label} · drag to turn · drag vertically to look up or down`}</p>
      </div>

      {!guidedMode && (
        <div className="panorama-controls" aria-label="Panorama field of view controls">
          <button onClick={() => setPanoramaFov((value) => Math.min(120, value + 8))} aria-label="Zoom out panorama">−</button>
          <span>{panoramaFov}°</span>
          <button onClick={() => setPanoramaFov((value) => Math.max(64, value - 8))} aria-label="Zoom in panorama">+</button>
        </div>
      )}

      {infoOpen && (
        <aside className="info-card">
          <button onClick={() => setInfoOpen(false)} aria-label="Close information">×</button>
          <span>{guidedMode ? selectedTour.status : 'Concept Design / Artist Impression'}</span>
          <p>{guidedMode ? selectedTour.route : active.note}</p>
          <small>{guidedMode ? (selectedTour.rejected ? 'Archived visual comparison only. A later floor-plan audit invalidated this clip as Apartment 106 spatial truth; it must not be used as final sales material.' : 'The selected route uses an existing local video and preserves its recorded QA status. Routes are separate review branches, not one claimed seamless complete tour.') : 'Layout follows the Apartment 106 marketing plan. Furniture, finishes and outlook are illustrative. These are concept panoramas, not measured 360° surveys.'}</small>
        </aside>
      )}

      {!guidedMode && (
        <nav className="wayfinding" aria-label="Connected rooms">
          {active.connections.filter((id) => panoramaStops.some((stop) => stop.id === id)).map((id) => {
            const destination = panoramaStops.find((stop) => stop.id === id)!;
            return (
              <button key={id} onClick={() => goTo(id)} aria-label={`Go to ${destination.label}`}>
                <span className="wayfinding-arrow">↑</span>
                <span>{destination.label}</span>
              </button>
            );
          })}
        </nav>
      )}

      <aside className={`plan-card ${planOpen ? 'open' : ''}`} aria-label={`${activeUnit.label} floor plan navigation`}>
        <div className="plan-heading">
          <div><span>Level 1 · {panoramaStops.filter((stop) => visited.has(stop.id)).length} of {panoramaStops.length} panoramas visited</span><strong>{activeUnit.label} floor plan</strong></div>
          <button onClick={() => setPlanOpen(false)} aria-label="Close floor plan">×</button>
        </div>
        <div className="plan-canvas">
          <img src={activeUnit.planImage} alt={`${activeUnit.label} floor plan`} draggable={false} />
          {panoramaStops.map((stop) => (
            <button
              key={stop.id}
              className={`plan-dot ${stop.id === activeId ? 'active' : ''} ${visited.has(stop.id) ? 'visited' : ''}`}
              style={{ left: `${stop.x}%`, top: `${stop.y}%` }}
              onClick={() => goTo(stop.id)}
              aria-label={`View ${stop.label}`}
              title={stop.label}
            ><span /></button>
          ))}
        </div>
        <div className="unit-strip" aria-label="Apartment floor plans">
          {units.map((unit) => (
            <button
              key={unit.id}
              type="button"
              className={unit.id === activeUnit.id ? 'active' : ''}
              disabled={!unit.tourAvailable}
              title={unit.tourAvailable ? `${unit.label} interactive tour` : `${unit.label} tour coming later`}
            >
              <strong>{unit.id}</strong>
              <small>{unit.tourAvailable ? 'Live' : 'Coming'}</small>
            </button>
          ))}
        </div>
        <div className="plan-footer"><span>2 bed</span><span>2 bath</span><span>1 MPR</span><span>{activeUnit.totalArea} total</span></div>
      </aside>

      <footer className="tour-footer">
        <span className="progress">{guidedMode ? `${(videoProgress * videoDuration).toFixed(1)}s` : `${String(panoramaStops.findIndex((stop) => stop.id === activeId) + 1).padStart(2, '0')} / ${String(panoramaStops.length).padStart(2, '0')}`}</span>
        <div className="room-strip" role="list" aria-label="All rooms">
          {guidedMode ? activeVideoTours.map((tour) => (
            <button key={tour.id} className={tour.id === selectedTourId ? 'active' : ''} onClick={() => chooseVideoTour(tour.id)}>
              {tour.label}
            </button>
          )) : panoramaStops.map((stop) => (
            <button key={stop.id} className={`${stop.id === activeId ? 'active' : ''} ${visited.has(stop.id) ? 'visited' : ''}`} onClick={() => goTo(stop.id)}>
              {visited.has(stop.id) && <i aria-hidden="true" />}{stop.label}
            </button>
          ))}
        </div>
      </footer>

      {guidedMode && <div className="scroll-hint">Scroll or swipe vertically to move <span>↕</span></div>}
    </main>
  );
}
