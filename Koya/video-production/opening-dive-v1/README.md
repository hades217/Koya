# Koya opening geographic dive v1 — rejected generation audit

Date: 2026-08-16  
Status: rejected; there is no active video candidate in this folder  
Model used: `doubao-seedance-2-5-260628`

## Rejected provider tasks

All three paid attempts are retained under `rejected-tests/` for audit only. None may be reused, concatenated or presented as a candidate:

- `cgt-20260815232853-2l42w`
- `cgt-20260815233231-q8nld`
- `cgt-20260815233722-s7hv8`

## Rejection reasons

- The first two attempts were separate 5-second generations, so they could not satisfy the requested one-take continuity.
- The third attempt was one 10-second first/last-frame generation, but Seedance 2.5 supports a single 30-second output. The shortened request discarded the available duration and left the middle of the move under-constrained.
- The 10-second result did not follow the approved storyboard closely enough and invented an unapproved intermediate building/space impression.
- Basic codec, black-frame and continuity checks do not establish architectural fidelity. The prior review incorrectly treated technical validity as creative approval.

## New production gate

No further paid generation is authorised from this folder. Before the next task, use the locally saved Seedance 2.5 capability pack and obtain approval of the single 30-second, 16:9, full-route reference manifest and time-coded one-take prompt.
