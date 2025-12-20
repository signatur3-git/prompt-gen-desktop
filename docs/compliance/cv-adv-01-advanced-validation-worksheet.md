---
title: CV-ADV-01 Advanced Validation Worksheet
---

# CV-ADV-01 advanced validation worksheet

The Advanced worksheet extends the Baseline checklist with deterministic seed capture, regression validation, and dual-review sign-offs. Use it for packages participating in regulated deployments or enhanced compliance programs.

> **Prerequisite:** Complete the Baseline checklist (CV-BAS-01) and reference the signed file in the AT-E05 Evidence bundle before filling out this worksheet.

## Section 0 – Submission linkage

| Field | Value |
| --- | --- |
| Submission ID | |
| Associated baseline checklist filename | |
| Advanced tier justification | |
| Validator primary | |
| Validator secondary reviewer | |
| Compliance liaison | |

## Section 1 – Deterministic seed validation

| Step | Description | Evidence reference | Status |
| --- | --- | --- | --- |
| DS-01 | Capture deterministic seed from validator CLI (`--seed` flag). | | |
| DS-02 | Re-run validation with captured seed to confirm reproducible output. | | |
| DS-03 | Store seed value in secure configuration vault (RFC 0001 §4.3). | | |
| DS-04 | Record vault reference ID in AT-E05 entry. | | |

## Section 2 – Regression pack execution

| Test suite | Target area | Result | Evidence reference | Notes |
| --- | --- | --- | --- | --- |
| RP-UI | Authoring Tool UI regression subset | | | |
| RP-API | Validator API regression subset | | | |
| RP-ML | Model inference regression subset | | | |
| RP-CUST | Customer-specific regression scenarios (list IDs) | | | |

> **Guidance:** Capture command output or test reports and store them under `evidence/regression/` with filenames referenced above.

## Section 3 – Dual-review findings

| Reviewer | Review scope | Summary | Approval status | Date |
| --- | --- | --- | --- | --- |
| Primary validator | Technical validation coverage | | | |
| Secondary validator | Compliance posture and remediation | | | |
| Compliance liaison | Control alignment confirmation | | | |

## Section 4 – Variances and corrective actions

| Reference | Description | Owner | Due date | Closure evidence |
| --- | --- | --- | --- | --- |
| | | | | |
| | | | | |

## Section 5 – Final approval block

| Role | Name | Signature (electronic) | Date |
| --- | --- | --- | --- |
| Validator primary | | | |
| Validator secondary | | | |
| Compliance liaison | | | |
| Program manager | | | |

> **Submission:** Combine this worksheet, the baseline checklist, and supporting evidence into a single ZIP archive and upload it to the compliance evidence repository. Reference the archive name in AT-E05.

