---
title: M3 Compliance Framework Draft
---

# M3 Compliance Framework Draft

This working draft captures the structure, ownership, and early content for the compliance artefacts targeted in Milestone M3. It draws on the Authoring Tool blueprint (M2) and the source-of-truth mapping (M1) to ensure terminology and traceability remain aligned while templates are piloted with the validator team.

> **Draft Status (2024-05-19):** Skeleton distributed to Compliance, Validator, and Authoring Tool workstreams for async review. Feedback window closes 2024-05-22 ahead of the compliance council walkthrough.

## 1. Compliance Tier Overview

| Tier | Focus | Key Controls | Primary Evidence |
| --- | --- | --- | --- |
| **Baseline** | Foundational governance for package submission and review. | Mandatory sanity checks, audit log retention (30 days), reviewer assignment protocol. | AT-E05 checkpoint log entries; validator sanity check exports. |
| **Advanced** | Enhanced validation coverage for regulated deployments. | Dual-review workflow, deterministic seed capture, automated regression pack. | AT-E05 log + Advanced Validation Worksheet (CV-ADV-01); CI artifacts referenced in validator backlog. |
| **Premium** | Continuous compliance with marketplace publication guarantees. | Continuous monitoring hooks, provenance signing, quarterly attestation package. | Premium Compliance Packet (CV-PREM-01) combining AT-E05 log, monitoring exports, and attestation minutes. |

### Tier Narrative Draft Notes

- Terminology mirrors RFC 0001 §Compliance Tiers; glossary sync confirmed 2024-05-19.
- Premium controls rely on forthcoming marketplace observability hooks (dependency on M4 planning); placeholders noted in Section 4.
- Deterministic seed capture process references the Template Engine canonical configuration in `source-of-truth/template-engine.md`.

## 2. Validation Evidence Templates

All templates will live under `docs/compliance/` (directory to be generated during drafting) with the following identifiers:

| Template ID | Artifact | Owner | Status | Notes |
| --- | --- | --- | --- | --- |
| **AT-E05** | Compliance Checkpoint Log (Authoring Tool) | Priya Shah | ✅ Published in blueprint | Serves as parent log referencing all validator evidence. |
| **CV-BAS-01** | Baseline Validation Checklist | Riley Carter | ✅ Published | Initial template committed under `docs/compliance/cv-bas-01-baseline-validation-checklist.md`; ready for validator pilot. |
| **CV-ADV-01** | Advanced Validation Worksheet | Riley Carter | 🟡 Drafting | Draft worksheet under `docs/compliance/cv-adv-01-advanced-validation-worksheet.md`; dual-review guidance pending validator feedback. |
| **CV-PREM-01** | Premium Compliance Packet Index | Priya Shah | 🟡 Outline | Packet index established in `docs/compliance/cv-prem-01-premium-compliance-packet-index.md`; monitoring export schema outstanding. |

### CV-BAS-01 Draft Outline

1. Header (package metadata, submission ID, deterministic seed reference).
2. Sanity check execution log linking to AT-E05 entries.
3. Validator notes on deviations, remediation owners, and deadlines.
4. Approval block (validator lead, compliance liaison, author acknowledgement).

> **Action:** Validator team to pilot the Baseline checklist during the Week of 2024-05-20 using the `sample-packages/baseline-demo` repository and record outputs via AT-E05.

## 3. Traceability Matrix Skeleton

The traceability matrix will connect compliance requirements to RFC clauses and validation artefacts. Initial skeleton is outlined below; final matrix will be stored in CSV format under `docs/compliance/m3-traceability-matrix.csv`.

| Compliance Requirement | RFC Source | Evidence Template | Notes |
| --- | --- | --- | --- |
| Sanity check completion per package | RFC 0001 §6.2 | AT-E05, CV-BAS-01 | Requires linking validator run IDs from automation logs. |
| Deterministic seed logged for reproducibility | RFC 0001 §4.3 | AT-E05, CV-ADV-01 | Dependent on automation hook update slated for 2024-05-23 deploy. |
| Marketplace publication attestation | RFC 0001 §8.1 | CV-PREM-01 | Placeholder; attestation meeting cadence to be confirmed with marketplace team. |

## 4. Dependencies & Open Questions

- **Marketplace Monitoring Hooks:** Need confirmation from Rendering & Marketplace workstream on telemetry format before Premium template can progress.
- **Automation Log Retention:** Ops team to verify audit log retention policy meets Baseline requirement (30 days) or capture variance.
- **Validator Tooling Export:** Determine whether existing validator CLI exports can populate CV-ADV-01 or if schema updates are required.

## 5. Upcoming Milestones

| Date | Activity | Outcome |
| --- | --- | --- |
| 2024-05-20 | Validator dry run with CV-BAS-01 draft | Collect annotated checklist and AT-E05 cross-links. |
| 2024-05-22 | Compliance council walkthrough | Approve tier narratives, confirm evidence template scope. |
| 2024-05-24 | Publish updated draft | Include validator feedback and traceability matrix CSV. |

## 6. Approvals & Review Log

| Reviewer | Role | Status | Notes |
| --- | --- | --- | --- |
| Priya Shah | Compliance lead | 🟡 In review | Comments due 2024-05-21 EOD. |
| Riley Carter | Validator lead | 🟡 In review | Will append pilot findings after 2024-05-20 dry run. |
| Morgan Silva | Authoring Tool lead | ⏳ Pending | Review focused on AT-E05 alignment. |

## 7. Distribution & Next Steps

1. Circulate this draft via project channel `#rpg-docs-compliance` and capture inline feedback in the repository issue tracker (label: `milestone:M3`).
2. Coordinate with Authoring Tool team to ensure AT-E05 captures checklist IDs for each submission and reference `docs/compliance/at-e05-compliance-checkpoint-log.md` as the operating guide.
3. Maintain the Traceability Matrix CSV (`docs/compliance/m3-traceability-matrix.csv`) after each validator dry run and ensure the M1 mapping references the latest artefact.
4. Prepare summary slide for 2024-05-22 compliance council with tier deltas and outstanding dependencies.
5. Publish the compliance documentation index (`docs/compliance/index.md`) to orient reviewers to the new artefacts before the walkthrough.

