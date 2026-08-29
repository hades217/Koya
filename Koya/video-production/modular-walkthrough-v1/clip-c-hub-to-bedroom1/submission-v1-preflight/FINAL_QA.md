# Clip C1 Seedance output — final QA

Date: 2026-08-26 (Australia/Brisbane)

Decision: **PASS local technical, continuity and topology QA; pending user acceptance**

## Provider result

- Task ID: `cgt-20260826201457-2wj85`
- Provider status: `succeeded`
- Model: `doubao-seedance-2-5-260628`
- Output: `outputs/cgt-20260826201457-2wj85.mp4`
- Completion tokens: 488,025
- Actual cost: CNY 16.163388

## Technical inspection

- Container duration: 5.056 seconds
- Video duration: 5.041667 seconds
- Resolution: 1920x1080
- Aspect ratio: 16:9
- Frame rate: 24 fps
- Video codec: HEVC
- Audio: AAC stereo, 32 kHz
- Audio level: mean -33.1 dB, maximum -7.8 dB
- File size: 7,425,705 bytes

## Continuity and topology inspection

- PASS: one uninterrupted camera move; no cut, dissolve or room replacement across all 121 video frames.
- PASS: the Bedroom 1 doorway is visible from the starting Living-hub composition and remains the same doorway throughout the approach.
- PASS: both jambs enlarge with forward parallax and the camera passes between them rather than through a wall.
- PASS: the main right turn begins only after entry into Bedroom 1.
- PASS: the bed remains on the required north wall and is revealed from its foot.
- PASS: the rightward inspection continues beyond the bed to the east/right terrace glazing.
- PASS: the endpoint retains the bed on the left and clearly shows the right wall, glazing, balcony edge and leafy Level 1 outlook.
- PASS: no extra doorway, duplicated room, visible person, operator reflection, text, logo or watermark appears.
- PASS: daylight, exposure, materials and room identity remain stable throughout the clip.

## Buyer-viewing result

The clip communicates the intended short room visit: orient from the Living hub, walk through the actual Bedroom 1 door, understand the bed placement, then inspect the glazing and outlook. The final image is useful for a buyer and does not resemble a static-image zoom.

## Evidence

- `qa-final-v1/ffprobe.json`
- `qa-final-v1/full-8fps-contact.jpg`
- `qa-final-v1/threshold-turn-12fps-contact.jpg`
- `qa-final-v1/frame-by-frame-24fps/second-01.jpg` through `second-06.jpg`
- `qa-final-v1/start.png`
- `qa-final-v1/end.png`
- `qa-final-v1/audio-volumedetect.txt`

No additional Seedance task is authorised. Clip C2 or another branch must not begin until the user accepts this Clip C1 output or explicitly changes priority.
