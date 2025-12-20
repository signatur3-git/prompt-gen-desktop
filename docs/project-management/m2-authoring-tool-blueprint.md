---
title: M2 Authoring Tool Documentation Blueprint
---

# M2 Authoring Tool Documentation Blueprint

This blueprint sequences the Authoring Tool documentation workstream for Milestone M2. It assumes the glossary, architecture overview, and source-of-truth mapping published during M1 remain the canonical references for terminology and component responsibilities.

> **Governance Status (2024-05-18):** Steering Committee approved the blueprint as submitted, confirmed evidence coverage for compliance linkage, and authorized the compliance workstream to begin Milestone M3 activities.

## 1. Documentation Outline

| Section | Purpose | Key Inputs | Acceptance Notes |
| --- | --- | --- | --- |
| **1. Executive Summary & Audience** | Frame the Authoring Tool package goals, target personas (authors, reviewers, compliance leads), and prerequisites. | Glossary; `docs/project-management/rfp-outline.md` executive summary anchors. | Highlight relationship to overall RFP scope and compliance tiers. |
| **2. Architecture Context Snapshot** | Summarize how the Authoring Tool interacts with validator, rendering engine, and marketplace services. | `docs/architecture/overview.md`; `docs/architecture/components.md`; `docs/project-management/m1-source-of-truth-mapping.md`. | Include diagram call-outs referencing canonical artefacts instead of redrawing. |
| **3. Installation & Environment Setup** | Detail supported platforms, package managers, environment variables, and workspace initialization scripts. | Operations backlog; forthcoming ops automation notes. | Provide CLI exemplars, container tags, and troubleshooting matrix placeholders. |
| **4. User Journeys** | Describe end-to-end flows for creating, editing, validating, and publishing prompt packages. | RFC 0001 workflow clauses; sanity check scenarios. | Each journey should include entry criteria, success metrics, and compliance checkpoints. |
| **5. CLI Reference** | Document commands, flags, configuration files, and exit codes. | Source-of-truth CLI spec (`source-of-truth/template-engine.md` for syntax alignment); repo CLI docs. | Use tabular format with examples and link to evidence snippets. |
| **6. API & Automation Hooks** | Capture REST/GraphQL endpoints, webhooks, and automation integration patterns. | RFC integration requirements; architecture context. | Flag authentication, rate limits, and versioning policy. |
| **7. UI Flows & Accessibility** | Outline primary screens, navigation, and accessibility commitments (ARIA, keyboard support). | UX wireframes (to be sourced from design); glossary terminology. | Provide screenshot placeholders tied to evidence checklist IDs. |
| **8. Compliance Touchpoints** | Map Baseline, Advanced, and Premium requirements to Authoring Tool features and operational controls. | [M3 Compliance Framework Draft](./m3-compliance-framework-draft.md); `docs/sanity-checks/index.md`; `docs/compliance/index.md`. | Include traceability matrix hooks for future linking and cross-reference template IDs (AT-E05, CV-BAS-01, CV-ADV-01, CV-PREM-01). |
| **9. Appendices** | Glossary references, configuration templates, sample packages, and troubleshooting guide. | Glossary; sample repos; sanity check data. | Document location for downloadable assets and version history. |

## 2. Evidence Capture Checklist

The following artefacts will be gathered during drafting and stored in the documentation assets folder (`docs/assets/authoring-tool/` to be created during drafting). Owners are aligned with the project plan backlog.

| Evidence ID | Description | Owner | Timing | Notes |
| --- | --- | --- | --- | --- |
| AT-E01 | Screenshot series covering primary UI flows (dashboard → package editor → validation results). | Morgan Silva | During UI walkthroughs (Week of 2024-05-20) | Ensure accessibility annotations accompany each screenshot. |
| AT-E02 | CLI command transcripts for package creation, validation, and publication (including error cases). | Casey Grant | After CLI dry runs with sanity checks | Capture deterministic seeds and configuration files alongside transcripts. |
| AT-E03 | Sample package repository demonstrating recommended structure and metadata. | Morgan Silva | Coordinate with Rendering workstream before 2024-05-24 | Tag release with compliance tier labels for future referencing. |
| AT-E04 | Configuration snippet library (YAML/JSON) for automation hooks and CI integration. | Alex Morgan | Post-automation dry run (Week of 2024-05-27) | Link to operations guide updates when available. |
| AT-E05 | Compliance checkpoint log template aligning authoring actions with evidence submission. | Priya Shah | Co-developed with M3 compliance draft | Operating guide published under `docs/compliance/at-e05-compliance-checkpoint-log.md`; ensure AT-E05 entries reference CV-BAS-01/CV-ADV-01 filenames captured in validator runs. |

> **Backlog Integration:** Each Evidence ID above has a corresponding card in the documentation Kanban board (swimlane “M2 – Authoring Tool Blueprint”). Owners are responsible for attaching artefacts to the repository and tagging the card with the evidence ID upon completion.

## 3. SME Interview Questionnaire

Schedule interviews with Authoring Tool maintainers (Morgan Silva, Casey Grant) and compliance liaison (Priya Shah). Interviews should be recorded and summarized in the meeting notes directory.

1. **Vision & Scope**
   - What primary user personas are in scope for the v1.0.0.rc1 Authoring Tool? Any excluded workflows we should note?
   - Which roadmap items are most at risk for scope creep during documentation drafting?
2. **User Journeys & UX**
   - Walk through the ideal flow for creating and publishing a package. Where do users typically encounter friction?
   - What accessibility standards and testing protocols are currently followed?
3. **CLI/API & Automation**
   - Which commands and endpoints are considered GA for v1.0.0.rc1 versus experimental?
   - Are there automation scripts or pipelines we should reference as implementation exemplars?
4. **Compliance & Evidence**
   - How do authors currently capture evidence required by compliance reviews, and what pain points exist?
   - What additional logging or audit features are planned before M3?
5. **Support & Maintenance**
   - What support channels exist for Authoring Tool users today, and how should they be documented?
   - How frequently do configuration defaults change, and where are these changes tracked?

## 4. Approval & Next Steps

- Circulate this blueprint to the Authoring Tool workstream and compliance leads for asynchronous review by 2024-05-15.
- Confirm evidence owners and due dates during the next weekly stand-up; update the Kanban board accordingly.
- Schedule SME interviews (one session per stakeholder) before 2024-05-17 and file notes under `docs/project-management/meeting-notes/` once the directory is created.
- Begin drafting outline sections 1–4 immediately after approvals, using Evidence IDs to track required artefacts.

> **Acceptance Alignment:** Completing the steps above satisfies M2 acceptance criteria: outline approval (Section 1), evidence checklist integration (Section 2), and SME questionnaire distribution plus scheduling (Sections 3–4).

## 5. Wrap-Up Summary

- Blueprint deliverables accepted by Authoring Tool and compliance leads with no outstanding actions.
- Evidence capture IDs (AT-E01–AT-E05) have active backlog cards; AT-E05 now anchors the compliance evidence pathway for sanity check outputs.
- M3 team will use Sections 1, 2, and 8 as seed material for the compliance framework drafts and validation template pilots.

