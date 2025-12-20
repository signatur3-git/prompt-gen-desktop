---
title: RPG v1.0.0.rc1 Documentation Roadmap
---

# RPG v1.0.0.rc1 Documentation Roadmap

This roadmap focuses exclusively on the documentation artefacts required to provide a complete, implementation-ready
specification for the Random Prompt Generator (RPG) v1.0.0.rc1 program. Each milestone clarifies the writing objectives,
intermediate deliverables, and acceptance criteria so contributors can track progress toward a finished documentation
set.

## Milestone Overview

| Milestone | Summary                                | Primary Workstreams               | Status                                                               |
|-----------|----------------------------------------|-----------------------------------|----------------------------------------------------------------------|
| M0        | Documentation discovery & planning     | Program Management                | ✅ Complete                                                           |
| M1        | Source-of-truth alignment              | Architecture, RFC, Glossary       | ✅ Complete                                                           |
| M2        | Authoring Tool documentation blueprint | Authoring Tool, Compliance        | ✅ Complete                                                           |
| M3        | Compliance framework drafting          | Compliance, Validator             | 🔄 In progress (framework + evidence templates published 2024-05-20) |
| M4        | Implementation guide drafting          | Rendering, Libraries, Marketplace | ⏳ Not started                                                        |
| M5        | RFP consolidation & review             | Program Management, Legal         | ⏳ Not started                                                        |
| M6        | Publication & handover                 | All                               | ⏳ Not started                                                        |

## Milestone Detail & Acceptance Criteria

### M0 – Documentation Discovery & Planning *(Complete)*

- **Purpose:** Establish the documentation backlog, contributors, and editorial standards before drafting begins.
- **Deliverables:**
    - Inventory of existing documents mapped to required deliverables.
    - Documentation style guide covering formatting, citations, and change control.
    - Project plan with writing schedule, owners, and review cadence.
- **Acceptance Criteria:**
    1. Inventory published in repository with gaps tagged and prioritized.
    2. Style guide reviewed by all workstream leads and linked from documentation index.
    3. Project plan approved with sign-off from governance and documentation editors.

### M1 – Source-of-Truth Alignment *(Complete)*

- **Purpose:** Align terminology, data models, and architectural references across all documentation tracks.
- **Deliverables:**
    - Updated glossary synchronized with RFC 0001 terminology.
    - Architecture overview summarizing core components, interfaces, and data flows for reference by writers.
    - Source-of-truth mapping that links RFC clauses to supporting documents and is stored at
      `docs/project-management/m1-source-of-truth-mapping.md`.
    - **NEW (2024-12-08)**: Decision Framework specification replacing hardcoded ContextInterfaces with author-defined logic components.
- **Acceptance Criteria:**
    1. Glossary approved by architecture and RFC editors.
    2. Architecture overview reviewed by at least two documentation authors outside the architecture team.
    3. Mapping document stored in repository at `docs/project-management/m1-source-of-truth-mapping.md` and referenced
       in subsequent milestone briefs.
    4. **NEW**: Decision framework documented with complete specifications for four processor types (Expression, Rule Set, Script, External).
    5. **NEW**: Challenge-solution patterns documented showing how to solve common problems (article coordination, phonetic selection) using Decisions.

> **Major Update (2024-12-08):** M1 was substantially enhanced with the Decision Framework, which replaces the previous ContextInterface approach. The new framework keeps the engine completely generic while giving authors maximum flexibility to define natural language logic—from simple rules to LLM integration. Three new source-of-truth documents were added:
> - `context-interactions.md` (rewritten): Generic context + Decision component spec
> - `decision-patterns-guide.md`: 10 patterns for solving real challenges across the rendering lifecycle
> - `article-coordination-problem.md`: Solutions for deferred decision-making with optional elements
> - `phonetic-article-selection.md`: Progressive accuracy levels from simple rules (~70%) to LLM-based (~99%)

> **Dependency Note:** Remaining follow-up items (e.g., compliance traceability hooks and publication workflow
> checkpoints) are now tracked under the M3 and M6 milestones so M1 can close without additional work.

### M2 – Authoring Tool Documentation Blueprint *(Complete)*

- **Purpose:** Define the structure, content outline, and research tasks for the Authoring Tool documentation package.
- **Deliverables:**
    - Detailed outline covering user journeys, CLI/API references, UI flows (if applicable), and compliance touchpoints,
      published at [`docs/project-management/m2-authoring-tool-blueprint.md`](./m2-authoring-tool-blueprint.md).
    - Evidence checklist specifying screenshots, data samples, and configuration snippets to be captured, aligned with
      the documentation backlog.
    - Interview/SME questionnaire for Authoring Tool maintainers with scheduling guidance.
- **Acceptance Criteria:**
    1. Outline approved by Authoring Tool workstream and compliance leads.
    2. Evidence checklist integrated into documentation backlog with owners assigned.
    3. SME questionnaire distributed and interview schedule confirmed.

> **Governance Update (2024-05-18):** Steering Committee fast-tracked approval of the blueprint and confirmed the
> compliance evidence linkage captured during M2, clearing the milestone for closure and unlocking M3 drafting activities.

### M3 – Compliance Framework Drafting *(In progress)*

- **Purpose:** Document the compliance tiers and validation pathways that govern Authoring Tool outputs and package
  lifecycle.
- **Deliverables:**
    - Draft compliance guide detailing Baseline, Advanced, and Premium criteria for authoring, validation, and review
      workflows (see [`m3-compliance-framework-draft.md`](./m3-compliance-framework-draft.md)).
- Validation evidence templates (checklists, report formats) to be used in the RFP and support materials, beginning with
  the CV-BAS-01 Baseline checklist pilot and extending through CV-ADV-01 and CV-PREM-01 outlines stored in
  `docs/compliance/`.
- Traceability matrix linking compliance criteria to RFC clauses and sanity checks (CSV published at
  `docs/compliance/m3-traceability-matrix.csv`).
- **Acceptance Criteria:**
    1. Draft compliance guide circulated for cross-stream review with tracked feedback resolved.
    2. Evidence templates tested against at least one reference package walkthrough.
    3. Traceability matrix stored in repository and referenced from compliance guide draft.

> **Current Focus (2024-05-20):** Walk validator team through the published compliance artefacts (`docs/compliance/`),
> execute the Baseline checklist pilot (CV-BAS-01) on `sample-packages/baseline-demo`, and update the traceability matrix
> CSV with pilot results before the 2024-05-22 council. The [M3 draft](./m3-compliance-framework-draft.md) tracks open
> questions.

### M4 – Implementation Guide Drafting

- **Purpose:** Produce cohesive drafts for each implementation guidance document referenced by the RFP (Authoring Tool,
  Validator, Rendering Engine, Shared Libraries, Marketplace).
- **Deliverables:**
    - Draft Authoring Tool implementation guide incorporating compliance workflows and usability standards.
    - Draft Validator, Rendering Engine, Shared Libraries, and Marketplace guides with consistent structure and
      cross-references.
    - Peer review notes capturing outstanding questions and risks per guide.
- **Acceptance Criteria:**
    1. All five drafts completed to 80% content readiness (placeholders resolved, diagrams stubbed).
    2. Cross-reference checks confirm consistent terminology and link integrity.
    3. Peer review notes logged with owners and due dates for resolution.

### M5 – RFP Consolidation & Review

- **Purpose:** Integrate the drafts into a cohesive Request for Proposal ready for stakeholder review.
- **Deliverables:**
    - Full RFP draft incorporating executive summary, scope, compliance framework, implementation guides, and submission
      instructions.
    - Review matrix capturing comments, resolutions, and approvers.
    - Summary brief for governance including open risks and decisions required.
- **Acceptance Criteria:**
    1. RFP draft completes editorial review cycle with all critical comments resolved.
    2. Review matrix signed by governance, legal, and technical approvers.
    3. Summary brief presented at governance review with decisions recorded.

### M6 – Publication & Handover

- **Purpose:** Finalize, publish, and transition the documentation package to implementors and procurement teams.
- **Deliverables:**
    - Published RFP (PDF/HTML) with supporting appendices and evidence templates.
    - Handover packet summarizing how to maintain and update documentation post-publication.
    - Retrospective report capturing lessons learned and backlog for future revisions.
- **Acceptance Criteria:**
    1. Final artifacts uploaded to documentation site with version tags and release notes.
    2. Handover session completed with procurement and implementation leads; attendance documented.
    3. Retrospective outcomes logged with actionable items tracked in backlog.

## Definition of Done for v1.0.0.rc1 Documentation

The documentation program is considered complete when:

1. Milestones M0–M6 meet their acceptance criteria with evidence stored in the repository.
2. Compliance tiers (Baseline, Advanced, Premium) are fully documented with traceability to RFC clauses and validation
   assets.
3. Implementation guides provide actionable, end-to-end instructions for building and verifying each reference
   component, with specific emphasis on the Authoring Tool.
4. The final RFP and supporting appendices are published, versioned, and approved by governance, legal, and technical
   leads.

## Final Documentation Deliverables Checklist

- Documentation inventory, style guide, and project plan.
- Updated glossary, architecture overview, and source-of-truth mapping.
- Authoring Tool documentation blueprint with evidence checklist and SME interview outputs.
- Compliance framework draft, validation evidence templates, and traceability matrix.
- Implementation guides for Authoring Tool, Validator, Rendering Engine, Shared Libraries, and Marketplace.
- Consolidated RFP draft, review matrix, and governance brief.
- Published RFP package, handover packet, and retrospective report.

