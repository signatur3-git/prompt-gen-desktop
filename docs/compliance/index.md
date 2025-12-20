---
title: Compliance Documentation Index
---

# Compliance documentation index

This index introduces the compliance artefacts produced during Milestone M3. It links to the validation evidence templates, traceability matrix, and supporting guidance required to demonstrate coverage of the RPG v1.0.0.rc1 compliance tiers.

## Artefact catalogue

| ID | Document | Purpose |
| --- | --- | --- |
| AT-E05 | [Compliance checkpoint log](./at-e05-compliance-checkpoint-log.md) | Canonical register of validator runs created by the Authoring Tool blueprint during M2. |
| CV-BAS-01 | [Baseline validation checklist](./cv-bas-01-baseline-validation-checklist.md) | Step-by-step checklist covering mandatory sanity checks for the Baseline tier. |
| CV-ADV-01 | [Advanced validation worksheet](./cv-adv-01-advanced-validation-worksheet.md) | Evidence capture worksheet for the Advanced tier, layering deterministic seed, regression, and dual-review validations. |
| CV-PREM-01 | [Premium compliance packet index](./cv-prem-01-premium-compliance-packet-index.md) | Assembly instructions and document index for continuous compliance obligations in the Premium tier. |
| TM-M3 | [Traceability matrix CSV](./m3-traceability-matrix.csv) | Cross-reference between compliance requirements, RFC 0001 clauses, and evidence artefacts. |

> **Review cadence:** The compliance workstream reviews these artefacts every Wednesday at 15:00 UTC during the editorial review ritual (see project plan §3).

## Usage guidance

1. Begin each validator engagement by opening the AT-E05 log and recording a new submission ID.
2. Apply the tier-appropriate checklist or worksheet in sequence. Capture supporting files (screenshots, CLI logs) in the submission folder and reference them by filename in the template tables.
3. After completing the run, update the traceability matrix CSV with the submission ID, evidence pointers, and status to maintain a single source of truth for audits.
4. Escalate discrepancies or missing artefacts to the compliance channel `#rpg-docs-compliance` and tag the owning workstream.

## Document custodians

| Artefact | Custodian | Backup |
| --- | --- | --- |
| AT-E05 checkpoint log | Morgan Silva | Riley Carter |
| CV-BAS-01 checklist | Riley Carter | Priya Shah |
| CV-ADV-01 worksheet | Riley Carter | Dana Ortiz |
| CV-PREM-01 packet index | Priya Shah | Alex Morgan |
| Traceability matrix | Priya Shah | Morgan Silva |

