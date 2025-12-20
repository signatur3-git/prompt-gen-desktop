---
title: Architecture Overview
---

# Architecture Overview

**Version:** 1.0.0  
**Status:** In Development (Target: Q1 2026)  
**Last Updated:** 2025-12-17

This document provides an overview of the Random Prompt Generator (RPG) specification and its core components. It serves as the entry point for understanding how the system works and how the pieces fit together.

---

## What is RPG?

The Random Prompt Generator is a **template-based system** for creating varied, contextually-aware text prompts. It enables authors to define reusable content packages that generate diverse outputs while maintaining physical plausibility and grammatical correctness.

**Key characteristics:**
- **Deterministic:** Same seed always produces same output
- **Template-based:** Define patterns, not individual outputs
- **Context-aware:** Elements coordinate (articles, pluralization)
- **Tag-filtered:** Select by properties ("flying creatures")
- **Modular:** Reusable packages with namespaces

**Target use cases:**
- Text-to-image AI prompts (Midjourney, DALL-E, Stable Diffusion)
- Creative writing prompts
- Game content generation
- Procedural storytelling

---

## System Components (v1.0.0)

### Authoring Tool (M7 ✅)

**Purpose:** Visual and text-based package creation

**Capabilities:**
- Create packages without writing YAML
- Visual editors for all component types
- Real-time validation with helpful errors
- Live preview with batch generation
- Export to YAML format

**Status:** Reference implementation complete (gitignored, will be published Q1 2026)

**Learn more:** [Getting Started Guide](../guides/getting-started.md)

---

### Package Validator (M6 ✅)

**Purpose:** Ensure packages are valid before use

**Capabilities:**
- Schema validation (YAML structure)
- Semantic validation (references exist, no cycles)
- Helpful error messages with suggestions
- Jump-to-error location information
- Warning system for non-blocking issues

**Status:** Complete with 17 tests, 9 error types

**Learn more:** [Validation System](./components.md#validation)

---

### Rendering Engine (M3-M5 ✅)

**Purpose:** Execute templates and generate output

**Capabilities:**
- Three-phase rendering (Selection → Enrichment → Rendering)
- Deterministic seeded RNG
- Tag filtering for value selection
- Context system for coordination
- Nested template support (10 levels deep)
- List generation with natural formatting

**Status:** Complete with full test coverage

**Learn more:** [Template Syntax](./template-syntax.md), [Tutorial 1-4](../guides/tutorial-series/)

---

### CLI Tool (M6 ✅)

**Purpose:** Command-line interface for package operations

**Commands:**
- `validate` - Check package validity
- `render` - Generate prompts from package
- `info` - Display package information

**Status:** Complete, embedded in desktop app

**Availability:** Q1 2026 (with desktop app release)

---

## Core Data Model

The data model defines the structure of packages and their components.

### Package (M2 ✅)

**Top-level container** for all content.

```yaml
id: my.package
version: 1.0.0
metadata:
  name: My Package
  description: Package description
  authors: ["Author Name"]
  bypass_filters: false
namespaces: {}
dependencies: []
```

**Contains:**
- Unique ID (reverse domain notation)
- Semantic version
- Metadata (name, description, authors)
- One or more namespaces
- Optional dependencies on other packages

**Learn more:** [Tutorial 1](../guides/tutorial-series/01-basic-package.md)

---

### Namespace (M2 ✅)

**Logical grouping** for related components.

**Contains:**
- Datatypes (value lists)
- PromptSections (templates)
- SeparatorSets (list formatters)
- Rules (context coordination)
- Decisions (future - v1.1.0+)

**Purpose:** Organize content and prevent naming conflicts

**Example:** `fantasy` namespace for fantasy content, `scifi` for sci-fi content

**Learn more:** [Components](./components.md#namespace)

---

### Datatype (M2 ✅)

**List of weighted values** with optional tags.

```yaml
creatures:
  name: creatures
  values:
    - text: swan
      weight: 1
      tags:
        can_fly: true
        article: a
    - text: eagle
      weight: 1
      tags:
        can_fly: true
        article: an
  extends: null
  override_tags: {}
```

**Features:**
- Values with weights (probability)
- Tags for filtering and coordination
- Can extend other datatypes
- Tag overrides for specialized variants

**Learn more:** [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md)

---

### PromptSection (M2-M5 ✅)

**Template** that combines datatypes to create output.

```yaml
scene:
  name: scene
  template: "{article} {creature} flying"
  references:
    article:
      target: context:article
      min: 1
      max: 1
      separator: null
      unique: false
      filter: null
    creature:
      target: fantasy:creatures
      min: 1
      max: 1
      separator: null
      unique: false
      filter: null
```

**Features:**
- Template with `{placeholder}` syntax
- References to datatypes or context
- Min/max for selecting multiple values
- Separators for natural list formatting
- Unique flag to prevent duplicates
- Tag filters for constrained selection
- Can reference other promptsections (nesting)

**Learn more:** [Tutorial 4: Lists and Separators](../guides/tutorial-series/04-lists-separators.md)

---

### SeparatorSet (M5 ✅)

**Defines how to format lists** naturally.

```yaml
comma_and:
  name: comma_and
  two: "{0} and {1}"
  start: "{0}, "
  middle: "{0}, "
  end: "{0} and {1}"
```

**Example:** `["red", "blue", "green"]` → `"red, blue and green"`

**Learn more:** [Tutorial 4](../guides/tutorial-series/04-lists-separators.md)

---

### Rules (M4 ✅)

**Coordinate between template elements** using context.

```yaml
rules:
  - set: article
    from: ref:creature.tags.article
```

**Purpose:** Solve coordination problems like:
- Article selection ("a" vs "an")
- Property sharing (color, habitat)
- Conditional logic

**Learn more:** [Tutorial 3: Context Rules](../guides/tutorial-series/03-context-rules.md)

---

## Rendering Pipeline (M3 ✅)

The engine uses a **three-phase pipeline** for deterministic rendering:

### Phase 1: Selection

- Parse template to find references
- For each reference, select value(s) from target datatype
- Apply tag filters if specified
- Respect min/max/unique constraints
- Use seeded RNG for deterministic selection

### Phase 2: Enrichment

- Execute rules in order
- Rules read from selected values (refs)
- Rules write to context
- Context becomes available for Phase 3

### Phase 3: Rendering

- Replace template placeholders with:
  - Selected values (from Phase 1)
  - Context values (from Phase 2)
  - Nested promptsection results (recursive)
- Apply separators for lists
- Return final string

**Learn more:** [Template Engine](../../source-of-truth/template-engine.md)

---

## Context System (M4 ✅)

The **context** is a key-value store that enables coordination between template elements.

**Key concepts:**
- Rules write to context during Enrichment phase
- Templates read from context during Rendering phase
- Context persists across nested templates
- Special namespace: `context:keyname`

**Common uses:**
- Article coordination
- Shared properties (color, habitat)
- Conditional selections
- Cross-element state

**Learn more:** [Context Interactions](./context-interactions.md), [Tutorial 3](../guides/tutorial-series/03-context-rules.md)

---

## Tag Filtering (M4-M5 ✅)

**Tag filtering** enables selection based on value properties.

**Syntax:** `{reference#{expression}}`

**Examples:**
```yaml
# Boolean tags
{creature#{tags.can_fly}}

# Logical AND
{creature#{tags.can_fly && tags.is_magical}}

# Logical OR
{creature#{tags.can_fly || tags.can_swim}}

# Logical NOT
{creature#{!tags.is_dangerous}}

# String comparison
{creature#{tags.size == 'large'}}

# Numeric comparison
{item#{tags.level > 5}}
```

**Purpose:** Ensure physical plausibility and thematic coherence

**Learn more:** [Tag Filtering](./tag-filtering.md), [Tutorial 2](../guides/tutorial-series/02-tag-filtering.md)

---

## Features Not in v1.0.0

The following features are **planned for future versions**:

### Rulebooks (v1.1.0+)

Wrappers around entry promptsections with:
- Multiple entry points
- Weight-based selection
- Context initialization
- Batch configuration

**Status:** Deferred - basic promptsections sufficient for v1.0.0

---

### Decisions (Future)

Advanced context operations for complex logic.

**Status:** Deferred - rules handle most coordination needs

---

### Pools (Future)

Fragment aggregation for cross-template reuse.

**Status:** Deferred - nested templates cover most use cases

---

### Morphers (Future)

Value transformations (pluralization, case conversion).

**Status:** Deferred - tag-based solutions work for v1.0.0

---

### Marketplace (Future)

Package distribution, discovery, and verification.

**Status:** Planned for post-v1.0.0

---

## For Package Authors

**Start here:**
1. [Getting Started Guide](../guides/getting-started.md) - Overview and first package
2. [Tutorial Series](../guides/tutorial-series/) - Step-by-step learning path

**Reference documentation:**
- [Components](./components.md) - Detailed component specs
- [Template Syntax](./template-syntax.md) - Complete syntax reference
- [Context Interactions](./context-interactions.md) - Rules and coordination
- [Tag Filtering](./tag-filtering.md) - Selection by properties

---

## For Implementers

**Building your own implementation?**

1. Read this overview
2. Study [Components](./components.md) for data models
3. Review [Template Engine](../../source-of-truth/template-engine.md) for rendering
4. Check [Test Packages](../../reference-impl/rpg-desktop/test-packages/) for examples
5. Reference the Rust implementation (when available)

**Key requirements:**
- Deterministic RNG (seeded)
- Three-phase rendering pipeline
- Tag filtering support
- Context system with rules
- Nested template resolution

---

## Version History

**v1.0.0** (Target: Q1 2026)
- Core data model complete
- Three-phase rendering
- Tag filtering
- Context system with rules
- Desktop authoring tool
- CLI validator and renderer
- Complete documentation

**Current Status:** Reference implementation complete, documentation in progress

---

## Next Steps

**For users:** Read the [Getting Started Guide](../guides/getting-started.md)  
**For learners:** Follow the [Tutorial Series](../guides/tutorial-series/)  
**For implementers:** Study [Components](./components.md) and [Source of Truth](../../source-of-truth/)

---

**This overview reflects the v1.0.0 specification as implemented in M1-M7.** For historical design decisions and evolution, see [DEVELOPMENT_PLAN.md](../../DEVELOPMENT_PLAN.md).
