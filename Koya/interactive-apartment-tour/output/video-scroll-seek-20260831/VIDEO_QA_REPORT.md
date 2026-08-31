# Scroll video seek optimisation QA

Status: `accepted_local_for_web_delivery`

## Problem diagnosis

- The original `west-rooms-review-scroll.mp4` contained 7 keyframes across 56.67 seconds.
- Average keyframe gap: 8.194 seconds.
- Maximum keyframe gap: 10.458 seconds.
- The page scrub loop also issued several progressively smaller seeks for one target, increasing decoder and network work.

## Delivery correction

- Web file: `public/tour/videos/west-rooms-review-seek-optimized.mp4`
- Source preserved: `public/tour/videos/west-rooms-review-scroll.mp4`
- Codec: H.264 High, yuv420p
- Dimensions: 1280 x 720
- Frame rate: 24 fps
- Duration: 56.666667 seconds
- File size: 16,884,827 bytes
- Video bitrate: 2,381,388 bit/s
- Keyframes: 114
- Average keyframe gap: 0.500 seconds
- Maximum keyframe gap: 0.542 seconds
- Faststart: pass; `moov` atom offset 36, before `mdat`
- Audio: unavailable in source; output remains silent

## Interaction correction

The scroll controller now coalesces input into at most one direct seek per animation frame. It no longer performs a chain of intermediate seeks for the same requested time.

## Visual evidence

- `west-rooms-seek-optimized-contact-sheet.jpg`
- Six checkpoints cover the complete route and show no re-encode-induced geometry or framing change.

## Remaining gate

Public GitHub Pages HTTP Range and interactive forward/backward seek must be verified after deployment before status can become `published`.
