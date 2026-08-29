# QA report

## Result

**PASS for the limited kitchen-to-closed-MPR-door approach.** The segment is accepted and joined after the existing 41-second master. It does not yet enter the MPR.

## Evidence checked

- Standalone source: `outputs/cgt-20260819220733-jd7c5-1.mp4`
- 1280x720, 24 fps, H.264 video; stereo 32 kHz AAC; 5.056 seconds.
- Full 8 fps contact sheets: `qa/contact-01.jpg`, `qa/contact-02.jpg`.
- Door interval at 16 fps: `qa/door-dense-01.jpg`, `qa/door-dense-02.jpg`.
- Key positions: `qa/keyframes-3x3.jpg`; near-final frame: `qa/final-near.png`.
- No scene-change frame exceeded the 0.18 detector threshold.
- First decoded frame vs accepted 41-second endpoint: SSIM `0.960130`.
- Audio is present and quiet: final-window peak approximately -34.7 dBFS, RMS approximately -52.1 dBFS.

## Visual findings

- The same woman, outfit, ponytail and left-hand stainless bottle remain continuous.
- Motion is a real forward walk with changing parallax, not a static-image zoom.
- Linear kitchen cabinets and the single island remain readable throughout.
- One closed timber door, one white jamb and one lever handle remain spatially stable.
- Her right hand approaches and rests on the handle without opening the door.
- No wall/cabinet dissolve, doorway teleport, MPR reveal, extra island or duplicated person was found.

## Join

- Joined output: `outputs/koya-continuous-review-through-mpr-closed-door-46s.mp4`.
- Duration: 45.936 seconds.
- The duplicate first frame of the new segment was removed; audio uses a 0.08-second crossfade.
- The 40.25-42.25 second join was inspected at 16 fps in `qa/join-dense-01.jpg` and `qa/join-dense-02.jpg`; no visible cut or geometry jump was found.

## Next gate

The next task must start from the actual generated closed-door endpoint and perform only: depress handle, open this same hinged door, cross the physical threshold. It remains unsubmitted pending review of this 45.936-second master.
