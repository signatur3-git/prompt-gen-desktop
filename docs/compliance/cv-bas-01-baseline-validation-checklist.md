---
title: CV-BAS-01 Baseline Validation Checklist
---

# CV-BAS-01 baseline validation checklist

Use this checklist when validating packages enrolled in the Baseline compliance tier. Complete each section sequentially and capture references to supporting evidence.

> **Applicability:** Baseline tier packages targeting marketplace publication with standard governance requirements.

## Submission metadata

| Field | Value |
| --- | --- |
| Submission ID | |
| Package name | |
| Validator lead | |
| Validation date | |
| Deterministic seed | |
| AT-E05 log entry | |

## Section 1 – Intake sanity checks

| Control | Description | Evidence reference | Status |
| --- | --- | --- | --- |
| SC-01 | Package metadata validated against schema (RFC 0001 §4.1). | | |
| SC-02 | Dependency manifest scanned for banned libraries (RFC 0001 §4.4). | | |
| SC-03 | Prompt templates render with default inputs without errors. | | |
| SC-04 | Localization strings load for supported locales. | | |
| SC-05 | Security audit log hooks initialized. | | |
| SC-06 | Documentation index references package entry. | | |

> **Guidance:** Record the filename or URL that demonstrates compliance in the Evidence reference column (e.g., `logs/sc-02-scan.txt`).

## Section 2 – Review findings

| Finding ID | Description | Severity | Owner | Due date | Resolution evidence |
| --- | --- | --- | --- | --- | --- |
| | | | | | |
| | | | | | |

## Section 3 – Variance handling

If any controls failed, capture the remediation plan below. Reference the corrective-action issue ID.

| Control | Issue link | Mitigation summary | Target closure date | Compliance approval |
| --- | --- | --- | --- | --- |
| | | | | |

## Section 4 – Approvals

| Role | Name | Signature (electronic) | Date |
| --- | --- | --- | --- |
| Validator lead | | | |
| Compliance liaison | | | |
| Package author representative | | | |

> **Completion rule:** Upload the signed checklist to the evidence repository and record the filename in the AT-E05 Evidence bundle column.

