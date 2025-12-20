---
title: M1 Source-of-Truth Mapping
---

# M1 Source-of-Truth Mapping

This mapping aligns RFC 0001 clauses with supporting documentation so future milestones can reference a consistent
baseline. It treats files in `source-of-truth/` as immutable definitions and identifies where derivative documents must
stay synchronized.

## Alignment Table

| RFC 0001 Section               | Key Topics                                                                                    | Supporting Documentation                                                                                                                                                                                                                                                                                                  | Notes                                                                                                                                                                                                                             |
|--------------------------------|-----------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| §Architecture Overview         | Component responsibilities, implementation scope.                                             | [Architecture Overview](../architecture/overview.md); [`source-of-truth/component-overview.md`](../../source-of-truth/component-overview.md)                                                                                                                                                                              | Use the overview for narrative summaries; cite the source-of-truth for authoritative diagrams.                                                                                                                                    |
| §Data Model                    | Packages, namespaces, components.                                                             | [Architecture Components](../architecture/components.md); [`source-of-truth/component-overview.md`](../../source-of-truth/component-overview.md)                                                                                                                                                                          | Ensure entity definitions mirror the ER diagram and manifest requirements.                                                                                                                                                        |
| §Template Language             | PromptSection grammar, repetition, separators.                                                | [Template Syntax](../architecture/template-syntax.md); [`source-of-truth/template-engine.md`](../../source-of-truth/template-engine.md)                                                                                                                                                                                   | Keep grammar snippets synchronized; flag deviations for architecture review.                                                                                                                                                      |
| §Rendering Context & Decisions | Context lifecycle, generic operations, Decision component framework for author-defined logic. | [Context Interactions](../architecture/context-interactions.md); [`source-of-truth/context-interactions.md`](../../source-of-truth/context-interactions.md)                                                                                                                                                               | The Decision framework replaces hardcoded contribution patterns. All semantic logic (articles, gender, etc.) is now author-defined via Decision components with four processor types: Expression, Rule Set, Script, and External. |
| §Compliance Tiers              | Tier definitions and validator expectations.                                                  | [RFC 0001 — Compliance Tiers](../rfc/0001-composite-random-prompt-generator.md#compliance-tiers); [M3 Compliance Framework Draft](./m3-compliance-framework-draft.md); [Compliance Documentation Index](../compliance/index.md); [M2 Authoring Tool Blueprint](./m2-authoring-tool-blueprint.md#8-compliance-touchpoints) | Link Evidence ID AT-E05 to sanity check records and keep CV-BAS-01/CV-ADV-01/CV-PREM-01 templates synchronized with glossary terminology.                                                                                         |
| §Reference Implementations     | Workstream deliverables, observability expectations.                                          | [Milestones Roadmap](./milestones.md); [Documentation Project Plan](./documentation-project-plan.md)                                                                                                                                                                                                                      | Update plan owners when reference implementation scope changes.                                                                                                                                                                   |
| §Security & Integrity          | Signing requirements, provenance.                                                             | Future compliance & marketplace guides; `source-of-truth/component-overview.md`                                                                                                                                                                                                                                           | Capture new security controls in M3/M4 drafts.                                                                                                                                                                                    |
| §Observability & Debugging     | Logging, diagnostics, reproducibility.                                                        | [Architecture Overview](../architecture/overview.md); sanity check documentation; [AT-E05 Compliance Checkpoint Log](../compliance/at-e05-compliance-checkpoint-log.md)                                                                                                                                                   | Ensure AT-E05 log fields cover deterministic seed capture and reference regression artefacts.                                                                                                                                     |

## Change Control Notes

- **Source-of-truth files remain unedited.** When requirements evolve, document diffs in milestone trackers and confirm
  with governance before proposing updates.
- **Cross-document updates require paired PRs.** If an RFC clause changes, update the corresponding documentation listed
  here in the same change set or include clear follow-up tasks.
- **Glossary synchronization:** All terminology referenced above must use the definitions in [Glossary](../glossary.md).
  Submit glossary updates alongside any new RFC terms.

## Sanity Check Evidence Linkage *(M2–M3 Update)*

- Evidence ID **AT-E05** in
  the [Authoring Tool blueprint](./m2-authoring-tool-blueprint.md#2-evidence-capture-checklist) now anchors the
  compliance checkpoint log template that will house sanity check outputs for Authoring Tool reviews.
- Sanity check scenarios recorded in [`docs/sanity-checks/index.md`](../sanity-checks/index.md) should reference the
  AT-E05 template once compliance finalizes the evidence forms during M3; the draft template lives in
  the [M3 Compliance Framework Draft](./m3-compliance-framework-draft.md#validation-evidence-templates).
- Validator and compliance workstreams agreed (2024-05-18 stand-up) that the blueprint's evidence log will be the
  authoritative bridge between Authoring Tool actions and compliance evidence submissions. Follow-up notes from the
  2024-05-19 validator sync are captured in the same draft under *Open Questions*.

## Major Update: Decision Framework (2024-12-08)

The M1 source-of-truth has been substantially revised to replace the previous "ContextInterfaces with hardcoded
contribution logic" approach with a **Decision Framework** that keeps the engine generic while maximizing author
flexibility.

### Key Changes

1. **Removed Hardcoded Patterns**:
    - Eliminated `context.request()` and automatic contribution evaluation
    - No more "ContextInterfaces" with embedded condition-action rules
    - Engine provides only generic operations: `get`, `set`, `has`, `delete`, `list`

2. **Introduced Decision Components**:
    - New first-class component type alongside Datatypes, PromptSections, etc.
    - Four processor types: Expression, Rule Set, Script, External
    - Authors define ALL semantic logic (articles, gender, tone, etc.) as Decisions
    - Templates invoke Decisions explicitly: `{decision.call(namespace:name, inputs)}`

3. **Implications for Implementation**:
    - **Simpler engine**: Less code, fewer hardcoded behaviors
    - **More author power**: Can implement arbitrary logic including LLM calls
    - **Clearer coordination**: All interactions visible in templates
    - **Better testability**: Each Decision is an isolated, testable unit

4. **Migration Path**:
    - Existing "request-fulfill" patterns become conventions, not engine features
    - Authors document their context key naming schemes in package docs
    - Contribution rules convert to Decision components with rule processors

### Impact on Milestones

- **M1 (Complete)**: Source-of-truth now stable with Decision framework documented
- **M2-M3 (In Progress)**: Authoring Tool must support Decision editing/testing
- **M4 (Pending)**: Rendering Engine guide must cover Decision processor sandboxing
- **M5-M6**: RFP examples should showcase Decision flexibility (e.g., LLM integration)

### Documentation Updates Required

- [ ] Update component overview ER diagram to include Decision entity
- [ ] Revise template syntax guide to show `decision.call()` syntax
- [ ] Add Decision validation rules to Package Validator spec
- [ ] Document sandboxing requirements for Script and External processors
- [ ] Create example packages demonstrating each processor type
- [ ] Update glossary with Decision, Processor, and related terms

### New Source-of-Truth Documents (2024-12-08)

Three new foundational documents have been added to `source-of-truth/`:

1. **`decision-patterns-guide.md`**: Comprehensive guide showing 10 core patterns for using Decisions across the
   rendering lifecycle, including stateless transformations, context coordination, cascading Decisions, and LLM
   integration. Includes challenge-solution matrix and best practices for mixing Decisions across prompts.

2. **`article-coordination-problem.md`**: Deep dive into the deferred decision-making problem when optional elements
   affect article selection. Documents 5 solution approaches from manual (render-first, articlize-after) to automated (
   PromptSection abstractions, first-word signaling, two-pass rendering, macro expansion).

3. **`phonetic-article-selection.md`**: Addresses the phonetic vs. orthographic challenge in article selection ("an
   hour" vs "a university"). Shows progression through 4 accuracy levels: simple letter-based rules (~70%), exception
   lists (~85%), phonetic dictionaries (~95%), and LLM-based selection (~99%). Demonstrates hybrid approach and caching
   strategies.

These documents demonstrate how the Decision framework handles progressively complex problems without requiring engine
changes.

## Open Alignment Tasks

1. Validate compliance template cross-references after the validator dry run updates CV-BAS-01 and CV-ADV-01. *(Due
   2024-05-24.)*
2. Document publication workflow dependencies in the roadmap ahead of M6. *(Schedule for M6 planning.)*
3. Capture monitoring export schema once marketplace workstream delivers hook specifications (prerequisite for
   CV-PREM-01 finalization).
4. **NEW**: Update all architecture documents to reflect Decision framework terminology and remove ContextInterface
   references. *(Due 2024-12-15.)*
5. **NEW**: Create reference Decision library for `featured.common` package (article selection, pronoun agreement,
   etc.). *(Due 2024-12-20.)*
