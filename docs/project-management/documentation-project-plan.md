title: RPG Documentation Project Plan (M0 Approved)
---

# RPG Documentation Project Plan

This plan outlines the workstreams, schedule, and governance checkpoints required to complete the RPG v1.0.0.rc1 documentation milestones. It satisfies Milestone M0 acceptance criteria and provides the coordination framework for subsequent deliverables.

> **Governance approval:** Ratified by the Steering Committee on 2024-04-12 following completion of the M0 review cycle.

## 1. Workstreams & Ownership

| Workstream | Scope Highlights | Interim Owner | Backup Reviewer |
| --- | --- | --- | --- |
| Program Management | Milestone tracking, backlog triage, stakeholder communications. | Alex Morgan | Priya Shah |
| Architecture | System diagrams, component interactions, glossary alignment. | Jamie Lee | Taylor Chen |
| Authoring Tool | Implementation blueprint, compliance checkpoints, UX guidelines. | Morgan Silva | Casey Grant |
| Validator | Validation rules, evidence templates, sanity-check integration. | Riley Carter | Jordan Fox |
| Rendering & Marketplace | Rendering engine guidance, marketplace governance, shared libraries. | Dana Ortiz | Morgan Silva |
| Compliance & Legal | Compliance framework, review matrix, governance brief. | Priya Shah | Alex Morgan |

> **Note:** Owner assignments are provisional and will be confirmed during the first steering committee review.

## 2. Milestone Schedule & Dependencies

| Milestone | Target Completion | Key Dependencies | Decision Gate | Status |
| --- | --- | --- | --- | --- |
| M0 – Discovery & Planning | 2024-04-19 | Availability of workstream leads; approval of style guide. | Steering Committee approval of this plan and style guide. | ✅ Approved |
| M1 – Source-of-Truth Alignment | 2024-05-03 | Glossary inputs from architecture; RFC clause mapping. | Architecture & RFC editors sign-off. | ✅ Complete |
| M2 – Authoring Tool Blueprint | 2024-05-17 | Authoring Tool SMEs availability; compliance review. | Authoring Tool workstream review. | ✅ Complete |
| M3 – Compliance Framework Drafting | 2024-05-31 | M1 glossary alignment; Validator artefact inventory. | Compliance council walkthrough. | 🔄 In progress — drafting packet circulated 2024-05-19 |
| M4 – Implementation Guide Drafting | 2024-06-21 | Completion of M2/M3 drafts; rendering & marketplace SMEs. | Cross-stream peer review checkpoint. | ⏳ Not started |
| M5 – RFP Consolidation & Review | 2024-07-05 | All draft guides at 80% readiness; review tooling configured. | Governance & legal joint session. | ⏳ Not started |
| M6 – Publication & Handover | 2024-07-19 | Final approvals; publication infrastructure readiness. | Release management sign-off. | ⏳ Not started |

> **Update (2024-05-18):** Governance fast-tracked the M2 approval and authorized the compliance workstream to commence M3 drafting using the Authoring Tool blueprint and associated evidence checklist.
>
> **Update (2024-05-19):** Initial M3 compliance framework draft skeleton published with tier definitions, validation log template owners, and traceability matrix outline. Validator team reviewing evidence coverage before walk-through.
>
> **Update (2024-05-20):** Compliance evidence templates (CV-BAS-01, CV-ADV-01, CV-PREM-01), AT-E05 operating guide, and traceability matrix CSV published under `docs/compliance/` to support validator dry run.

## 3. Review Cadence & Rituals

- **Weekly Stand-up (Mondays 09:00 UTC):** 30-minute sync covering milestone progress, blocker escalation, and cross-workstream dependencies.
- **Bi-weekly Editorial Review (Wednesdays 15:00 UTC):** Focused review of in-flight drafts against style guide and RFP outline expectations.
- **Monthly Steering Committee (First Thursday 17:00 UTC):** Governance checkpoint to approve milestone completion and scope adjustments.

## 4. Backlog Tracking & Tooling

- Maintain a shared Kanban board (e.g., GitHub Projects) with swimlanes for each milestone (M0–M6).
- Tag backlog items with corresponding RFP section numbers to keep deliverables aligned with `rfp-outline.md`.
- Track M2 evidence capture cards using the Evidence IDs defined in `m2-authoring-tool-blueprint.md` and assign the listed owners to each card.
- Use repository issues to capture feedback requiring documentation updates; reference the relevant document and milestone in each issue title.
- Store meeting notes and decisions in `docs/project-management/meeting-notes/` (folder to be created during M1).

## 5. Risk Register (Initial)

| Risk | Impact | Mitigation | Owner |
| --- | --- | --- | --- |
| Delayed glossary alignment impacts downstream compliance docs. | High | Prioritize glossary review in first editorial session; allocate backup reviewer. | Jamie Lee |
| Authoring Tool SMEs unavailable during M2 planning. | Medium | Schedule interviews during M0; capture questions in advance. | Morgan Silva |
| Documentation tooling not ready for automated checks. | Medium | Evaluate tooling options during M2; document manual review steps in interim. | Alex Morgan |

## 6. Next Actions

1. Publish the approved plan and style guide in the documentation index and archive the governance decision record.
2. Configure GitHub Project board with milestone swimlanes and assign initial backlog items from the inventory.
3. Confirm owner availability for the upcoming weekly stand-up and editorial review cadence.
4. Schedule compliance review workshop to validate the M3 draft traceability matrix against sanity check scenarios and log outcomes in the AT-E05 evidence register.
5. Onboard validator leads to the new `docs/compliance/` artefacts and confirm evidence submission workflow before the 2024-05-22 council.

