# Apartment 106 Interior Repair V1

Status: `CONNECTOR_ASSETS_QA_PASS_VIDEO_NOT_SUBMITTED`

## Scope

Repair only the failed interior route logic from the rejected Seedance outputs. Preserve the accepted exterior, foyer, lift, corridor and apartment-entry footage. Do not treat either rejected 30-second interior output as an approved master.

## Geometry truth

- Official plan: `../../Koya marketing plan Apartment 106.pdf`
- Unit: Apartment 106, Level 1
- Official schedule: 2 bedrooms, 2 bathrooms, 1 MPR, 2 car spaces, 99 sqm internal, 53 sqm external, 152 sqm total
- The official plan is geometry evidence. Generated interiors and furniture remain indicative concept design.

## Corrected 16:9 connector assets

1. `generated-connectors/01-turn-to-separate-laundry-bath-corrected-v2.png`
   - Same resident at the actual apartment-entry endpoint.
   - Bottle in anatomical left hand; right hand gives the invitation.
   - Separate laundry opening contains washer/cabinetry.
   - Adjacent main bathroom opening contains shower/bathroom finishes.

2. `generated-connectors/02-wet-area-to-bedroom2-corrected-v1.png`
   - Short physical connection from the wet-area thresholds to the nearby Bedroom 2 door.
   - No invented long corridor.
   - Bottle left; right hand reaches for the Bedroom 2 handle.

3. `generated-connectors/03-bedroom2-exit-to-adjacent-mpr-corrected-v1.png`
   - Camera remains inside Bedroom 2 while the resident exits through the same door.
   - Open kitchen/dining circulation is directly beyond.
   - Separate adjacent MPR door is visible immediately to the right/east.

4. `generated-connectors/04-mpr-exit-wide-living-corrected-v1.png`
   - Camera remains inside the compact MPR and sees the same physical doorway.
   - Kitchen, island, dining, living and terrace glazing resolve as one wide connected public space.
   - No narrow galley substitution.

## Rejected connector drafts

- `generated-connectors/rejected/01-turn-two-bathrooms-rejected.png`: represented two bathroom-like openings and omitted a legible laundry.
- `generated-connectors/rejected/03-bedroom2-exit-mpr-door-missing-rejected.png`: omitted the adjacent MPR door.

## Character lock

- Same athletic white woman and face family as the preserved footage
- Deep-brown ponytail
- Sage fitted long-sleeve athletic top
- Matte black full-length leggings
- White-grey trainers
- Stainless bottle always in anatomical left hand
- Anatomical right hand opens doors and performs the invitation

## Video gate

No paid Seedance repair has been submitted from this package. Before submission, present the exact retained time ranges, replacement route, full prompt, ordered reference list, endpoint parameters and current cost estimate for explicit approval.

## R1 corrected extension provider order

| Order | Provider role | File | SHA-256 |
| ---: | --- | --- | --- |
| Video 1 | `reference_video` | `../seedance-2.5-submission-v4-segment-c-retry1/outputs/koya-continuous-preview-through-apartment-entry-21s.mp4` | `f5c23b14a468a23eaf19da510b567dbcb44b7c0f648deda693c75e1be05d94ac` |
| Image 1 | `reference_image` | `references/01-turn-to-separate-laundry-bath-corrected-v2.png` | `9f23221531594c70bf50dd3fec3765b1e35d346d0628b6cf90ab0d287a036903` |
| Image 2 | `reference_image` | `references/02-wet-area-to-bedroom2-corrected-v1.png` | `5fd9daba305f4c8cc26564b0ca8b9ae04ac46e689df164568a77f5e70ebd8f7f` |

The accepted reference video is 1280x720. The two generated spatial constraints are 1672x941, effectively 16:9. They are continuity constraints only and are not official renders.

Exact corrected R1 prompt: `PROMPT_R1_EXTENSION.txt`

Exact request template: `SUBMISSION_PAYLOAD_TEMPLATE.json`

Paid gate and current estimate: `PAID_GATE.md`
