# Full arrival and Apartment 106 Entry — web integration QA

- Integration status: `local_qa_candidate`
- Source status: `SUCCEEDED_QA_PASS`
- Source QA: `PASS_LITERAL_APARTMENT_ENTRY_THRESHOLD`
- Website interaction: vertical wheel and touch scrub

## Correct source

`../../../video-production/seedance-2.5-submission-v4-segment-c-retry1/outputs/koya-continuous-preview-through-apartment-entry-21s.mp4`

- SHA-256: `f5c23b14a468a23eaf19da510b567dbcb44b7c0f648deda693c75e1be05d94ac`
- Route: Koya arrival, building entrance, foyer, lift, Apartment 106 approach, apartment door threshold, compact Entry Hall
- Source duration: 21.047 seconds
- Source technical format: H.264 + AAC, 1280x720, 24 fps

The previously integrated five-second file was only the final apartment-door
Segment C. It was incorrectly presented as the complete Entry experience and
has been removed from the active route.

## Web encode

`public/tour/videos/full-arrival-entry-qa-pass-scroll.mp4`

- SHA-256: `f79255d2ca40465f7f1b2f4bff8044eee05dc5dabdaf10a2f0f4a5d0f7a438ca`
- H.264 + AAC, 1280x720, 24 fps
- Duration: 21.056 seconds
- Size: 9,757,655 bytes
- `moov` atom begins at byte 36; media data begins at byte 17,408
- Keyframe interval: 0.5 seconds
- Encoding changed delivery characteristics only; it did not change the route or imagery

## Browser acceptance gates

- default page opens directly in vertical-scroll video mode
- play control must enable even when metadata loads before React hydration
- desktop wheel must seek forward and backward
- mobile vertical drag must seek forward and backward
- HTTP Range response: PASS, `206 Partial Content`, `Accept-Ranges: bytes`, correct `Content-Range`
- browser source/duration read-back: PASS, exact web source loaded at 21.056 seconds
- desktop vertical wheel: PASS from arrival through Apartment 106 approach and Entry Hall
- mobile vertical drag: PASS, 0.000 to 10.247 seconds in one upward gesture
- chapter chaining: PASS, further downward scroll at Entry end opens West rooms at 0.000 seconds
- reverse chapter chaining: PASS, upward scroll at West rooms start restores Entry at 21.056 seconds
- mobile chapter chaining: PASS, upward gesture at Entry end opens West rooms at 0.000 seconds
- user visual acceptance: pending
