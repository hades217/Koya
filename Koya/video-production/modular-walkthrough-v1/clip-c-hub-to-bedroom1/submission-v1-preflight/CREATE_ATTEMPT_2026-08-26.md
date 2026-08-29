# Seedance create attempt — rejected before task creation

Time: 2026-08-26 (Australia/Brisbane)

- Provider task created: no
- Task ID: none
- Usage returned: none
- Generation charge from this attempt: none observed
- Provider request ID: `021787745810830274564af00b8ed8c9fdfc012f24960791b9732`
- Error: `reference_video must be provided as a web url`
- Retry policy: do not resubmit the local-path request. Materialise the same fingerprinted MP4 through a verified HTTPS URL, rerun preflight and obtain approval for the revised transport package.

An earlier local CLI process that produced no output was discovered still running before any provider task appeared. It was terminated to prevent duplicate task creation. Only one controlled process was then allowed to finish, producing the provider validation error above.
