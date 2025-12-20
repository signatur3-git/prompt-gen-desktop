---
title: Glossary
---

# Glossary

| Term | Definition |
| --- | --- |
| **Authoring Tool** | IDE or CLI for creating packages, namespaces, and component assets with deterministic exports and signing workflows. |
| **Compliance Tier** | Level of conformance defined in RFC 0001 (Tier 1 Renderer Core, Tier 2 Authoring & Validation, Tier 3 Marketplace) that dictates required capabilities and evidence. |
| **Context Scope** | Qualifier appended to context keys (e.g., `.prompt`, `.section.<id>`, `.global`, or custom scopes) to control read/write visibility with hierarchical fallbacks. |
| **ContextInterface** | Namespaced contract that declares context keys, request flags, contribution rules, and validators so tagged datatypes or promptsections can satisfy runtime needs declaratively. |
| **Datatype** | Namespaced dataset whose values may carry tags used for filtering or context contributions during rendering. |
| **Deterministic Rendering** | Guarantee that identical seeds, inputs, and packages yield identical outputs across compliant engines. |
| **Library Package** | Canonical namespace bundle (e.g., `featured.common`) distributed for reuse across implementations. |
| **Marketplace** | Registry service that stores packages, dependency metadata, compliance badges, and provenance signatures. |
| **Morpher** | Deterministic transformation rules (pluralisation, conjugation, casing) applied during rendering using context and tags. |
| **Namespace** | Logical grouping of assets inside a package, addressed via `namespace:component`. |
| **Package** | Distribution unit containing manifests, namespaces, component definitions, dependencies, and compliance assertions. |
| **Package Validator** | Tooling that enforces schemas, semantics, determinism, and compliance tiers before publication. |
| **Pool** | Optional namespaced collection that aggregates rendered values or component outputs for deterministic reuse later in the same prompt. |
| **PromptSection** | Template-driven component combining static text with expressions, references, and context operations. |
| **Reference Library** | Collection of reusable namespaces and datasets published as part of the shared library workstream (e.g., `featured.common`). |
| **Rendering Context** | Scoped key-value store maintained during rendering so components can request, contribute, and reuse information without hardcoded logic. |
| **Rendering Engine** | Service or library that executes rulebooks, resolves templates, manages context, and produces deterministic prompts. |
| **Rulebook** | Namespaced configuration listing weighted entry promptsections, batch settings, and required interfaces. |
| **SeparatorSet** | Namespaced definition of primary, secondary, and tertiary separators for grammatical list formatting. |
| **Source of Truth** | Repository folder (`source-of-truth/`) containing immutable architectural references that downstream documents summarise and cite. |
