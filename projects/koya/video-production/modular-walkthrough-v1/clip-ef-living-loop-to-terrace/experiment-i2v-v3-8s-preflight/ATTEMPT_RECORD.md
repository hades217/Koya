# V3 submission attempt record

Status: `CREATE TRANSPORT/RESPONSE FAILURE / NO TASK CREATED / DO NOT AUTO-RETRY`

The one authorised V3 create attempt exited after the provider/client returned:

> RequestError code: 400, err: invalid character 'E' looking for beginning of value

No provider request ID or generation task ID was returned.

Immediate read-back evidence:

- Generation task count remained `21`.
- Latest task remained the older `cgt-20260828173707-5hxv6`.
- No new task usage entry appeared.
- No `arkcli +gen` process remained active.

The exact prompt, input and parameters were not changed. The package fingerprint remains `c6f6062b91f5f778633675add9a96a551c06cf84a5cc205b8ea5ec167c7e9a10`, but the prior one-attempt authorization has been consumed. A retry requires a fresh explicit user approval and must not run automatically.
