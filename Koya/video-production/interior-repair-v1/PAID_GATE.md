# Paid Gate — Interior Repair V1

Status: `R1_SUBMITTED_ONCE_REJECTED_NO_R2_AUTHORIZED`

## What will be submitted

Exactly three sequential Seedance 2.5 tasks:

1. R1: apartment entry -> separate laundry/main bathroom -> Bedroom 2 threshold, 10 seconds.
2. R2: Bedroom 2 same-door loop -> immediately adjacent MPR threshold, 10 seconds.
3. R3: compact MPR same-door loop -> wide kitchen/dining/living/terrace reveal, 10 seconds.

Only R1 is submitted first. R2 is not submitted unless R1 passes dense transition, character, handedness and topology QA. R3 is not submitted unless R2 passes. A failed leg stops the chain and avoids paying for downstream footage built on a bad endpoint.

## Preserved footage

- Preserve the accepted 21.047-second exterior-to-apartment-entry video unchanged.
- Do not use either rejected 30-second interior candidate as a master.
- Useful rejected frames are appearance evidence only, not approved motion.

## Model and parameters

- Endpoint: `ep-20260812221158-hb576`
- Model: `doubao-seedance-2-5-260628`
- Task type: video extension (`extend`), continuing from the accepted 21.047-second `reference_video`
- Duration: `10` seconds per task
- Aspect ratio: `adaptive`, inherited from the 1280x720 reference video as 16:9
- Resolution: `720p`
- Generate audio: `true`
- Return last frame: `true`
- Watermark: `false`
- Output: `mp4`
- Number of outputs: exactly `1` per task
- Submission policy: one raw provider request only; never retry blindly if the client returns no immediate task ID. Reconcile the provider task list before any further request.

## Cost estimate

Current verified video-input (`V2VCompletion`) price: CNY 0.042 per 1,000 completion tokens. R1 uses the accepted 21.047-second film as its reference video.

The two latest like-for-like 30-second 720p extension jobs each used 1,101,600 completion tokens. Using that observed output-token rate:

- Estimated 10-second task: 367,200 tokens = CNY 15.4224
- Maximum for all three passing tasks: 1,101,600 tokens = CNY 46.2672

This is an evidence-based estimate, not the final split bill. Formal billing can differ and is only authoritative after provider reconciliation.

## Stop gates

- Stop after R1 if resident continuity, left-hand bottle, separate laundry/bathroom, short circulation or Bedroom 2 threshold fails.
- Stop after R2 if the camera does not exit Bedroom 2 through the same door or the MPR is not immediately adjacent.
- Stop after R3 if the camera does not exit the MPR through the same door, the public zone is narrow, or any hard cut/collision appears.
- Do not call the result one-take until boundary inspection at 0.125-second intervals passes.

## Exact approval phrase

`确认重新提交 Interior Repair R1 Extension，预计 CNY 15.4224；R1 通过后再分别确认 R2 和 R3。`
