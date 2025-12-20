---
title: RFC 0001 — Random Prompt Generator v1.0.0.rc1
---

# RFC 0001 — Random Prompt Generator v1.0.0.rc1

- **Status:** Published (Release Candidate)
- **Authors:** RPG Working Group
- **Reviewers:** TBD
- **Created:** 2024-XX-XX
- **Target Version:** v1.0.0.rc1

## Abstract

This RFC establishes the release-candidate specification for the Random Prompt Generator (RPG) ecosystem. It codifies the shared architecture, data model, templating language, and context semantics required to deliver deterministic prompt generation across independent implementations. The RFC also defines compliance expectations for the Authoring Tool, Package Validator, Rendering Engine, shared Libraries, and Marketplace workstreams.

## Motivation

Creative teams need deterministic, reusable prompts that can be authored collaboratively, validated automatically, and rendered reproducibly. Existing ad-hoc scripts lack portability and observability. RPG provides:

- A data-first package format expressible in canonical YAML/JSON.
- A declarative template language capable of advanced linguistic behaviour without hardcoding.
- A scoped context model that allows components to request and supply information dynamically.
- Governance hooks (compliance tiers, validator signatures) for marketplace-ready distribution.

## Design Tenets

1. **Determinism Everywhere:** Rendering the same package with the same seed produces identical output across conforming engines.
2. **Author Empowerment:** Logic lives in data—datatypes, contextinterfaces, morphers—not engine code.
3. **Interoperable Packages:** Namespaces reference assets across packages through versioned dependencies and capability contracts.
4. **Extensible but Stable:** New component types extend schemas without breaking existing packages.
5. **Transparent Compliance:** Validators emit machine-readable reports so marketplaces and operators can trust published assets.

## Architecture Overview

The ecosystem splits into five collaborating implementations. Detailed responsibilities live in [Architecture & Data Model](../architecture/components.md).

1. **Authoring Tool** – IDE/CLI for crafting packages, namespaces, and components with deterministic exports.
2. **Package Validator** – Static analysis pipeline enforcing schemas, semantics, and determinism.
3. **Rendering Engine** – Deterministic executor for rulebooks and templates, including context lifecycle.
4. **Reference Libraries** – Canonical namespaces (e.g., `featured.common`) and reusable datasets.
5. **Marketplace** – Registry APIs for publishing, discovering, and governing packages.

## Data Model

RPG packages bundle namespaced assets serialised to canonical YAML (source of truth) with deterministic JSON mirrors. Core entities include:

- **Package & Manifest** – Metadata (id, version, authors, license), dependencies with semantic version ranges, capability declarations (`provides`/`requires` contracts), compliance assertions, and digital signatures.
- **Namespace** – Logical grouping containing datatypes, promptsections, separatorsets, rulebooks, pools, morphers, and contextinterfaces.
- **Datatype** – List or structured dataset with optional tags for context contribution. Supports filters and unique draws.
- **PromptSection** – Template plus metadata (parameters, required interfaces, repetition constraints).
- **SeparatorSet** – Primary/secondary/tertiary separators for list rendering.
- **Morpher** – Deterministic transformations referencing tags and context.
- **Rulebook** – Weighted entry promptsections, batch configuration (count, seed derivation), and required interfaces.
- **Pool** – Collection aggregator for reuse during rendering.
- **ContextInterface** – Declares context keys, request flags, contributions, and validators.

A full entity relationship diagram appears in [Architecture & Data Model](../architecture/components.md#data-relationships).

## Template Language

Promptsections use the RPG template language described in [Template Syntax](../architecture/template-syntax.md). Implementations MUST:

- Support datatype and promptsection references with `min`/`max` repetition, `sep` selectors, and tag filters.
- Implement the context helper suite (`context.get`, `set`, `has`, `request`, `random`).
- Honour the EBNF grammar, including escaping (`\{`, `\}`) and nested expressions.
- Enforce semantic rules via validation (e.g., `min <= max`, referenced assets exist, pool constraints satisfied).

## Rendering Context

Context semantics follow [Context Interactions](../architecture/context-interactions.md): namespaced keys, scoped lifecycles, declarative ContextInterfaces, and deterministic contribution ordering. Engines MUST:

- Initialise context per prompt and respect scope fallback order.
- Evaluate contributions in render order, resolving ties via `priority`.
- Surface deterministic diagnostics (seed, scope, key) when validation fails.
- Provide hooks for debug snapshots without altering rendering results.

## Compliance Tiers

| Tier | Scope | Requirements |
| --- | --- | --- |
| **Tier 1 — Renderer Core** | Rendering engines and CLI utilities. | Template grammar compliance, context lifecycle, deterministic randomness, manifest consumption, and seed management APIs. |
| **Tier 2 — Authoring & Validation** | Authoring Tool + Package Validator. | Tier 1 plus package creation/export, schema validation, semantic analysis, validator signatures, deterministic package archives. |
| **Tier 3 — Marketplace** | Full ecosystem with registry services. | Tier 1 & 2 plus package hosting, dependency graph APIs, compliance badge publication, signing/verification, and audit logging. |

Compliance reports MUST include:

- Implementation identifier and version.
- Tier satisfied and optional partial capabilities.
- Validator signature with timestamp and deterministic hash of the analysed package or service configuration.

## Reference Implementations

To reach v1.0.0, the working group will build reference implementations covering:

- **Libraries:** Canonical packages (`featured.common`, language packs) and reusable validator test fixtures.
- **Authoring Tool:** CLI/GUI that edits components, simulates context interactions, and exports signed packages.
- **Package Validator:** CLI service verifying schemas, semantics, and deterministic guarantees; emits machine-readable reports.
- **Rendering Engine:** Deterministic executor with batch support, observability hooks, and pool/context tooling.
- **Marketplace:** Registry with publish/install APIs, provenance checks, and capability search.

Each implementation MUST publish API surface documentation, seed handling guidance, and integration examples tied to the sanity scenarios.

## Security & Integrity

- Packages and compliance reports are signed using repository-approved cryptography (e.g., Ed25519). Signatures cover manifests, component definitions, and artifacts.
- Registries verify signatures and retain provenance metadata (submitter, validator, timestamp).
- Rendering engines expose configuration to lock down allowed packages and namespaces.

## Observability & Debugging

- Rendering sessions log seed, rulebook entry, package versions, and context snapshots (debug mode) without introducing nondeterminism.
- Validators output structured diagnostics (machine and human readable) with deterministic ordering.
- Authoring tools record export metadata (seed, dependency graph, signatures) for reproducibility.

## Roadmap

1. **Schema Finalisation:** Publish JSON Schema for all component types and manifests.
2. **Reference Packages:** Deliver `featured.common` and multilingual packs covering article, gender, and mood interfaces.
3. **Tooling MVPs:** Ship CLI prototypes for authoring, validation, and rendering to exercise the spec end to end.
4. **Marketplace Alpha:** Stand up registry APIs with publish/install flows and compliance badge ingestion.
5. **Feedback Loop:** Collect implementer feedback, update RFC errata, and stabilise for v1.0.0.

## Open Issues

- Governance for evolving shared namespaces and approving breaking schema changes.
- Localisation strategy for morphers relying on external datasets (e.g., CLDR integration).
- Version negotiation between marketplace clients and registries for capability discovery.
- Long-term metrics/analytics strategy that preserves determinism and privacy.

## References

- [Architecture & Data Model](../architecture/components.md)
- [Context Interactions](../architecture/context-interactions.md)
- [Template Syntax](../architecture/template-syntax.md)
- [Sanity Check Scenarios](../sanity-checks/index.md)
- [Operations Guide](../operations/index.md)
