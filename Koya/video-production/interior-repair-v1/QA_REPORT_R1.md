# Interior Repair R1 Extension — QA Report

Overall result: `REJECTED_NOT_VISUALLY_SEAMLESS`

## Passed

- Exact source-to-generated first-frame continuity is visually matched.
- Same resident, wardrobe and left-hand stainless bottle remain stable.
- Laundry and main bathroom are readable as separate functions.
- Technical delivery is 10.000 seconds, 1280x720, 24 fps, with native AAC stereo audio.

## Failed

- At approximately 6.375–6.500 seconds, the bathroom view, resident and following hallway overlap as semi-transparent layers.
- The wet-area position dissolves into the next corridor instead of the camera physically turning and walking through the adjacent circulation.
- This is a visible crossfade / architecture morph and violates the literal one-camera contract.
- The resulting connector also reads longer and more corridor-like than the compact plan relationship requested.

## Evidence

- `outputs/r1-extension/qa/start-continuity-side-by-side.jpg`
- `outputs/r1-extension/qa/transition-5-to-7-0.125s.jpg`
- `outputs/r1-extension/qa/dense-contact-0.125s.jpg`
- `outputs/r1-extension/qa/contact-sheet-0.5s.jpg`
- `outputs/r1-extension/qa/ffprobe.json`

## Decision

Do not splice this output into the master and do not label it one-take. Do not submit R2 from this returned endpoint. Any future repair needs a different control strategy; repeating this prompt or automatically retrying is not authorized.

