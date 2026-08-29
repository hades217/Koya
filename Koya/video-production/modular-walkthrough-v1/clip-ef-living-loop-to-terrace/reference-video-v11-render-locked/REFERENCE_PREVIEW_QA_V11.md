# Apartment 106 EF deterministic reference V11

Status: `rendered_local_qa_passed`

## Why V10 was rejected

V10 proved a continuous fixed-scene route but its visual world did not match the accepted V7 rendered storyboard closely enough. It used generic maquette materials, different terrace furniture and camera framings that over-emphasised the kitchen bench. Its earlier local QA therefore does not transfer to V11.

## V11 gates

- One fixed Three.js scene and one physical camera for all eight seconds.
- Floor-plan topology remains authoritative.
- The accepted V7 storyboard is the checkpoint-level visual target for scale, furniture, palette and glazing rhythm.
- The terrace remains an open walking strip with a continuous planted edge, matching the accepted arrival and look-back frames.
- No still-image zoom, cut, crossfade, dissolve, morph, teleport or geometry penetration.
- A new render hash and frame-by-frame QA are required; the V10 MP4 must never be reused or relabelled.

## Current gate

The updated Studio preview was accepted for continuation and a fresh local MP4 was rendered. Frame-level review covered the whole clip at 8 fps, the threshold sequence at 12 fps and the terrace turn/look-back at 12 fps. The clip contains no black frame or detected freeze, uses one fixed scene and physically crosses the same east opening without a cut, dissolve or teleport.

## Locked render

- File: `renders/apartment-106-ef-v11-render-locked.mp4`
- SHA-256: `0693dcc7cf4040465b6ab5ea4fce68d73af713f9f6f42b46b14a353fb9b2a4d1`
- Video: H.264 High, 1920 × 1080, 30 fps, 8.000 seconds, 240 frames, no audio.
- QA: `qa-final/contact-whole-8fps.jpg`, `qa-final/contact-threshold-12fps.jpg`, `qa-final/contact-turn-lookback-12fps.jpg`, `qa-final/ffprobe.json`, `qa-final/automated-video-check.txt`.

This local pass authorises the V11 file to enter a new Seedance preflight review only. It is not a paid-task approval and does not itself prove that Seedance will honour all multimodal references.
