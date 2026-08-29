# Apartment 106 Literal Route Ledger V2

Status: `ACTUAL_21S_ENDPOINT_LOCKED_START_CONNECTOR_READY_FOR_REVIEW`  
Spatial truth: `../Koya marketing plan Apartment 106.pdf` and `../production-assets/assets/official/floorplans/apartment-106.png`  
Continuity contract: `literal_walkthrough`

Route overlay: `../production-assets/contact-sheets/apartment-106-literal-route-v2.png`

## Correct physical route

```text
Entry
  -> hall-side main bathroom and laundry
  -> continue to Bedroom 2
  -> enter Bedroom 2
  -> exit through the same Bedroom 2 door
  -> turn east and join the kitchen circulation axis
  -> enter MPR from its kitchen/dining-side door
  -> exit through the same MPR door
  -> continue east along kitchen
  -> enter Bedroom 1 from the kitchen-side door
  -> move through Bedroom 1 into WIR
  -> continue from WIR into ensuite
  -> return through the same ensuite/WIR sequence
  -> cross Bedroom 1 and exit through the same Bedroom 1 door
  -> regain kitchen
  -> visibly clear the island end
  -> dining and invitation
  -> living
  -> cross the real glazing threshold to terrace
```

The MPR is not on the entry hall. Its door is reached from the kitchen/dining side. Any storyboard or video that moves directly from Bedroom 2 into MPR without regaining the hall/kitchen axis is rejected.

## Visual continuity ledger

| Beat | Time | Camera position and action | Required visual anchor | State |
| ---: | --- | --- | --- | --- |
| K00 | 21.047s | Existing camera and resident at the hall/kitchen threshold, facing east | `continuity-audit/apartment-106-actual-21s-end-frame.png` | Extracted from preserved 21.047s output; 1280x720; locked start evidence |
| K01 | 21-23s | Camera stays at the same threshold and yaws west while resident turns naturally; wet rooms resolve camera-right | `apartment-106-k01-actual21s-turn-to-wet-area-v2.png` | Actual-endpoint-derived, floor-plan audited candidate |
| K02 | 24-29s | Reveal separate laundry and main bathroom from hall | `CON-U106-BATH-LDRY-001` | Candidate exists |
| K03 | 29s | Back out of the north-side wet-area threshold, yaw south, and open Bedroom 2's single northwest door | `apartment-106-k03-wet-area-to-bedroom2-connector-v3.png` | Floor-plan and left-hand-bottle audited candidate |
| K04 | 29-35s | Cross Bedroom 2 door, reveal bed/robe/light | `CON-U106-BED2-DOOR-001` | Candidate exists |
| K05 | 35s | Reverse the same movement and visibly exit the same door | `apartment-106-k05-bedroom2-same-door-exit-v3.png` | Floor-plan audited candidate |
| K06 | 35-39s | Regain hall and turn east into the kitchen circulation axis | `apartment-106-k06-hall-to-kitchen-axis-v1.png` | Candidate exists |
| K07 | 39s | MPR door appears on the correct kitchen/dining side | `apartment-106-k07-kitchen-to-mpr-threshold-v2.png` | Floor-plan audited candidate |
| K08 | 39-44s | Enter compact MPR, small reveal, return | `CON-U106-MPR-001` | Candidate exists |
| K09 | 44s | Exit same MPR door and recover the same kitchen axis | `apartment-106-k09-mpr-same-door-exit-v3.png` | Floor-plan audited candidate |
| K10 | 44-49s | Move east through galley, cabinetry left and island right | `CON-U106-GALLEY-CINE-001` | Candidate exists |
| K11 | 49s | Reach and open Bedroom 1 door from kitchen side | `apartment-106-k11-kitchen-to-bedroom1-threshold-v1.png` | Candidate exists |
| K12 | 49-55s | Cross into Bedroom 1 and establish bed/light/WIR | `CON-U106-BED1-001` | Candidate exists |
| K13 | 55-61s | Pass physically through WIR into ensuite | `CON-U106-WIR-ENS-001` | Candidate exists |
| K14 | 61-66s | Reverse ensuite -> WIR -> Bedroom 1 -> same bedroom door | `apartment-106-k14-suite-return-to-kitchen-v2.png` | Floor-plan audited candidate |
| K15 | 66-68s | Regain kitchen and visibly clear island end | `CON-U106-DINING-TURN-CINE-002` | Candidate exists |
| K16 | 68-71s | Resident clears route and invites camera into living | `CON-U106-WELCOME-CINE-003` | Floor-plan and hand-continuity audited candidate |
| K17 | 71-75s | Camera passes resident into living | `CON-U106-LIVING-FINAL-CINE-001` | Candidate exists |
| K18 | 75-78s | Cross real glazing threshold into private terrace | `CON-U106-TERRACE-001` | Candidate exists |

## Current completion truth

- Room order and physical route: complete and corrected against the Type 106 plan.
- Time-coded camera and character logic: complete at director-plan level.
- Room appearance anchors: available for every required room.
- Transition anchors: actual 21.047-second endpoint extracted and a replacement K01 v2 generated from it; the former K01 v1 is rejected for camera teleport and orientation mismatch.
- Paid Seedance payloads: not prepared for approval and not submitted.

Start-continuity sheet: `../production-assets/contact-sheets/apartment-106-21s-to-wet-area-continuity-v1.jpg`. Audited connector sheet: `../production-assets/contact-sheets/apartment-106-continuity-connectors-v2-floorplan-audited.jpg`. Active full sequence: `../production-assets/contact-sheets/apartment-106-full-sequence-v3-actual-endpoint-audited.jpg`. The older v2 full sheet is superseded because it contains the rejected K01 v1 and K03 v2. The route is not paid-generation-ready until the replacement K01/K03 and every later candidate connector are visually approved.

Expanded start-to-Bedroom-2 audit sheet: `../production-assets/contact-sheets/apartment-106-start-sequence-v3-actual-endpoint-audited.jpg`. K05 and K06 were re-inspected after the K03 replacement: they retain the same Bedroom 2 door, bottle stays in the anatomical left hand, and the camera visibly regains the kitchen axis; no replacement is required at this stage.
