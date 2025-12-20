# Documentation Restructuring Plan - RFC-Ready Format

**Created:** 2025-12-16  
**Purpose:** Reorganize documentation for implementors and RFC publication  
**Target:** v1.0.0 release-ready structure

---

## Problem Statement

### Current Issues

1. **Scattered Information**: Architecture split across multiple files, hard to find what you need
2. **Unclear Entry Points**: Not obvious where implementors should start for specific components
3. **Mixed Concerns**: Project management mixed with technical spec
4. **Weak Component Isolation**: Hard to implement just the Renderer without reading everything
5. **Not RFC-Like**: Lacks the structure of well-known specs (IETF, OIDC, OAuth2, etc.)

### What Good Specs Look Like

**IETF RFC Structure:**
- Clear abstract and motivation
- Requirements language (MUST, SHOULD, MAY)
- Progressive disclosure (overview → details → examples)
- Component isolation (can implement parts independently)
- Normative vs informative sections clearly marked
- Complete examples and test vectors
- Security considerations
- IANA considerations (for us: package format registration)

**OpenID Connect Structure:**
- Core spec (required)
- Extension specs (optional features)
- Implementation guides per component
- Conformance test suites
- Clear separation: spec vs examples vs best practices

---

## Proposed Structure

### Top-Level Organization

```
docs/
├── spec/                       # Normative specification (RFC-ready)
│   ├── core/                   # Core specification (REQUIRED)
│   ├── extensions/             # Optional extensions
│   └── conformance/            # Compliance testing
│
├── guides/                     # Implementation guides (INFORMATIVE)
│   ├── implementors/           # Per-component implementation guides
│   ├── authors/                # Package authoring guides
│   └── tutorials/              # Step-by-step tutorials
│
├── reference/                  # Reference material (INFORMATIVE)
│   ├── examples/               # Complete working examples
│   ├── patterns/               # Design patterns and solutions
│   └── api/                    # API reference documentation
│
└── meta/                       # Project management (NON-NORMATIVE)
    ├── project/                # Internal project tracking
    └── decisions/              # ADRs and design decisions
```

---

## Detailed Structure

### 1. spec/core/ - The Normative Specification

**Goal:** Everything needed to implement a conforming component

```
spec/core/
├── 00-introduction.md          # Abstract, motivation, terminology
├── 01-architecture.md          # High-level architecture, components
├── 02-package-format.md        # Package manifest, YAML/JSON schemas
├── 03-data-model.md            # All component types (Datatype, PromptSection, etc.)
├── 04-template-language.md     # Complete EBNF, syntax rules
├── 05-rendering-engine.md      # Three-phase rendering, determinism
├── 06-context-system.md        # Context lifecycle, scopes, operations
├── 07-coordination.md          # Rules/Decisions framework (from M1)
├── 08-validation.md            # Validation rules, error codes
├── 09-security.md              # Security considerations
├── 10-conformance.md           # Compliance tiers, test requirements
└── 11-package-registry.md      # Marketplace spec (optional)
```

**Each file follows RFC structure:**
```markdown
# N. Title

## N.1 Overview
Brief summary

## N.2 Requirements
MUST/SHOULD/MAY statements

## N.3 Specification
Detailed technical content

## N.4 Examples
Complete, runnable examples

## N.5 Security Considerations
Specific to this component

## N.6 Test Vectors
Inputs and expected outputs
```

---

### 2. spec/extensions/ - Optional Features

**Goal:** Features that extend core but aren't required for basic conformance

```
spec/extensions/
├── ext-morphers.md             # Advanced text transformations
├── ext-pools.md                # Pool aggregation system
├── ext-llm-integration.md      # External processor integration
├── ext-i18n.md                 # Internationalization extensions
└── ext-custom-processors.md    # Plugin architecture
```

**Structure:**
- What problem it solves
- Core spec sections it extends
- New component types or operations
- Conformance requirements (if any)

---

### 3. guides/implementors/ - Implementation Guides

**Goal:** Help developers implement specific components

```
guides/implementors/
├── 00-overview.md              # How to use these guides
├── 01-renderer.md              # Implementing the Rendering Engine
├── 02-validator.md             # Implementing the Package Validator
├── 03-authoring-tool.md        # Implementing the Authoring Tool
├── 04-marketplace.md           # Implementing package registry
└── 05-libraries.md             # Creating reusable libraries
```

**Each guide contains:**
```markdown
# Implementing [Component]

## What You'll Build
Clear description and scope

## Prerequisites
- Knowledge requirements
- Dependencies
- Spec sections to read first

## Architecture
Component-specific architecture diagram

## Step-by-Step Implementation
1. Phase 1: Core functionality
2. Phase 2: Advanced features
3. Phase 3: Optimization

## Testing Strategy
- Unit tests
- Integration tests
- Conformance tests

## Common Pitfalls
What to watch out for

## Reference Implementation
Link to our reference impl

## Conformance Checklist
□ MUST requirements
□ SHOULD recommendations
```

---

### 4. guides/authors/ - Package Authoring Guides

**Goal:** Help package authors create content

```
guides/authors/
├── 00-getting-started.md       # Your first package
├── 01-datatypes.md             # Creating datatypes effectively
├── 02-templates.md             # Writing templates
├── 03-coordination.md          # Solving coordination problems
├── 04-testing.md               # Testing your packages
├── 05-publishing.md            # Publishing to marketplace
└── 06-best-practices.md        # Package design patterns
```

---

### 5. guides/tutorials/ - Step-by-Step Tutorials

**Goal:** Learn by building real examples

```
guides/tutorials/
├── 01-hello-world.md           # Simplest possible package
├── 02-simple-prompts.md        # "A red ball"
├── 03-optional-elements.md     # Article coordination
├── 04-pluralization.md         # Count-based forms
├── 05-complex-scenes.md        # Text-to-image prompts
└── 06-character-generator.md   # Full example: RPG character sheets
```

**Each tutorial:**
- Complete working example
- Explained step-by-step
- Common variations
- What you learned
- Next tutorial

---

### 6. reference/examples/ - Complete Examples

**Goal:** Real-world packages for reference

```
reference/examples/
├── minimal/                    # Smallest valid package
├── simple-prompts/             # Basic examples
├── text-to-image/              # Complex scene descriptions
├── character-sheets/           # RPG character generator
├── story-starters/             # Creative writing prompts
└── featured-common/            # The "standard library"
```

Each example includes:
- Complete package YAML
- README explaining the example
- Test cases with expected outputs
- Design notes

---

### 7. reference/patterns/ - Design Patterns

**Goal:** Solutions to common problems (from M1 analysis)

```
reference/patterns/
├── article-selection.md        # "a" vs "an" coordination
├── pluralization.md            # Count-based forms
├── gender-agreement.md         # Pronoun coordination
├── optional-elements.md        # Deferred decisions
├── thematic-consistency.md     # Mood/tone matching
└── spatial-relationships.md    # Complex scene composition
```

**Each pattern:**
```markdown
# Pattern: [Name]

## Problem
What challenge does this solve?

## Context
When do you need this?

## Solution
How to implement it with RPG

## Examples
Complete working examples

## Variations
Alternative approaches

## Trade-offs
Pros/cons of each approach

## Related Patterns
What else might help
```

---

### 8. reference/api/ - API Reference

**Goal:** Complete API documentation for programmatic use

```
reference/api/
├── renderer-api.md             # Rendering Engine API
├── validator-api.md            # Validator API
├── context-api.md              # Context operations
├── cli-reference.md            # Command-line tools
└── marketplace-api.md          # Registry REST API
```

**Format:** Standard API docs with request/response examples

---

### 9. meta/ - Project Management (Internal)

**Goal:** Keep project management separate from spec

```
meta/
├── project/
│   ├── milestones.md           # Development milestones
│   ├── roadmap.md              # Long-term roadmap
│   └── rfp-outline.md          # RFP materials
│
└── decisions/
    ├── adr-001-context-operations.md
    ├── adr-002-rules-vs-decisions.md
    └── ...
```

**ADR Format (Architecture Decision Records):**
```markdown
# ADR-NNN: Title

**Status:** Proposed | Accepted | Deprecated | Superseded  
**Date:** YYYY-MM-DD  
**Deciders:** [names]

## Context
What's the issue?

## Decision
What did we decide?

## Consequences
What are the implications?

## Alternatives Considered
What else did we evaluate?
```

---

## Migration Plan

### Phase 1: Restructure (Part of M1)

**Tasks:**
1. ✅ Update VitePress config (COMPLETE - see VITEPRESS_MIGRATION.md)
2. Create new directory structure
3. Create stub files for all navigation entries
4. Move existing content to new locations
5. Update cross-references
6. Test VitePress locally

**Commands:**
```bash
# Create directory structure
mkdir -p docs/spec/{core,extensions,conformance}
mkdir -p docs/guides/{implementors,authors,tutorials}
mkdir -p docs/reference/{examples,patterns,api}
mkdir -p docs/meta/{project,decisions}

# Create stub files for spec/core (00-11)
for i in {00..11}; do echo "# Coming Soon" > "docs/spec/core/$(printf '%02d' $i)-placeholder.md"; done

# Create stub files for guides
for i in {00..05}; do echo "# Coming Soon" > "docs/guides/implementors/$(printf '%02d' $i)-placeholder.md"; done
for i in {00..06}; do echo "# Coming Soon" > "docs/guides/authors/$(printf '%02d' $i)-placeholder.md"; done
for i in {01..06}; do echo "# Coming Soon" > "docs/guides/tutorials/$(printf '%02d' $i)-placeholder.md"; done

# Create index files
echo "# Extensions" > docs/spec/extensions/index.md
echo "# Examples" > docs/reference/examples/index.md
echo "# Patterns" > docs/reference/patterns/index.md
echo "# API Reference" > docs/reference/api/index.md

# Test VitePress
cd docs
npm install
npx vitepress dev
```

**Priority:** Do this before M1 completes, so spec updates go to right places

**See:** VITEPRESS_MIGRATION.md for detailed steps

### Phase 2: Fill Gaps (During M2-M4)

**Tasks:**
1. Write missing spec sections
2. Extract examples from source-of-truth
3. Create implementation guides
4. Document patterns from M1 analysis

**Priority:** As we implement, document what we learn

### Phase 3: Polish (M8)

**Tasks:**
1. Ensure consistency
2. Add test vectors to all spec sections
3. Complete API documentation
4. Review against RFC guidelines

**Priority:** Before v1.0.0 release

---

## Document Templates

### Spec Section Template

```markdown
---
title: N. Section Title
status: Draft | Stable | Final
version: 1.0.0-rc1
---

# N. Section Title

## N.1 Overview

[Brief summary - 2-3 sentences]

## N.2 Conformance Requirements

This section describes [what] using the key words MUST, MUST NOT, REQUIRED, 
SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL as 
specified in RFC 2119.

## N.3 Specification

[Detailed technical content]

### N.3.1 Sub-topic
[Details]

### N.3.2 Another Sub-topic
[Details]

## N.4 Examples

### N.4.1 Example: [Name]

**Input:**
```yaml
[input]
```

**Output:**
```
[expected output]
```

**Explanation:** [why this works]

## N.5 Security Considerations

[Specific security issues for this section]

## N.6 Test Vectors

| Input | Seed | Expected Output | Notes |
|-------|------|-----------------|-------|
| ...   | ...  | ...             | ...   |

## N.7 References

- [Normative references]
- [Informative references]
```

---

### Implementation Guide Template

```markdown
---
title: Implementing [Component]
audience: Developers
difficulty: Beginner | Intermediate | Advanced
estimated_time: X hours/days
---

# Implementing [Component]

## What You'll Build

[Clear description]

**Conformance Level:** Tier 1 | Tier 2 | Tier 3

## Prerequisites

### Knowledge
- [ ] Understand [spec section]
- [ ] Familiar with [concept]

### Dependencies
- [Language/framework requirements]
- [Libraries needed]

### Reading
- [Spec sections to read first]

## Architecture Overview

[Component diagram]

**Key Responsibilities:**
1. [Responsibility 1]
2. [Responsibility 2]

## Implementation Phases

### Phase 1: Core Functionality

**Goal:** [What works at end of phase]

**Steps:**
1. [Step 1]
2. [Step 2]

**Test:** [How to verify]

### Phase 2: [Advanced Feature]

[Repeat pattern]

## Testing Strategy

### Unit Tests
[What to test]

### Integration Tests
[End-to-end scenarios]

### Conformance Tests
[Link to conformance suite]

## Common Pitfalls

### Pitfall: [Name]
**Problem:** [What goes wrong]  
**Solution:** [How to avoid]

## Performance Considerations

[Optimization tips]

## Reference Implementation

Our reference implementation: [link]

**Key files to review:**
- [File 1] - [Purpose]
- [File 2] - [Purpose]

## Conformance Checklist

### MUST Requirements
- [ ] [Requirement 1]
- [ ] [Requirement 2]

### SHOULD Requirements
- [ ] [Recommendation 1]

### OPTIONAL Features
- [ ] [Optional feature]

## Next Steps

After completing this guide:
1. [What to do next]
2. [Related guides]
```

---

## Benefits of New Structure

### For Implementors

✅ **Clear entry point**: "I want to implement a renderer" → go to `guides/implementors/01-renderer.md`  
✅ **Progressive disclosure**: Start with guide, deep-dive into spec as needed  
✅ **Component isolation**: Can implement one piece without understanding everything  
✅ **Test-driven**: Test vectors and conformance checklists in each section

### For Package Authors

✅ **Tutorials first**: Learn by doing, not by reading specs  
✅ **Patterns library**: Solutions to common problems ready to use  
✅ **Examples**: Real packages to learn from  
✅ **Best practices**: Accumulated wisdom in one place

### For RFC Publication

✅ **Standards-compliant**: Follows IETF/W3C/OAuth patterns  
✅ **Normative clarity**: Spec vs guides vs examples clearly separated  
✅ **Version stability**: Core spec stable, extensions evolve independently  
✅ **Conformance testing**: Clear criteria for "RPG-compliant"

### For Our Development (M1-M9)

✅ **Parallel work**: Can write guides while implementing  
✅ **Knowledge capture**: Learnings from implementation feed guides  
✅ **Decision tracking**: ADRs separate from spec  
✅ **Clean extraction**: Project mgmt stays internal, spec goes public

---

## File Mapping: Old → New

### High Priority (Do in M1)

| Current Location | New Location | Notes |
|------------------|--------------|-------|
| `docs/rfc/0001-*.md` | `spec/core/00-introduction.md` | Split into multiple files |
| `docs/architecture/overview.md` | `spec/core/01-architecture.md` | Simplify |
| `docs/architecture/components.md` | `spec/core/03-data-model.md` | Rename for clarity |
| `docs/architecture/template-syntax.md` | `spec/core/04-template-language.md` | Already good |
| `docs/architecture/context-interactions.md` | `spec/core/06-context-system.md` | Will update with M1 results |
| `source-of-truth/article-coordination-problem.md` | `reference/patterns/article-selection.md` | Move to patterns |
| `source-of-truth/decision-patterns-guide.md` | `reference/patterns/` | Split into individual patterns |
| `docs/examples/text-to-image-prompts.md` | `guides/tutorials/05-complex-scenes.md` | Convert to tutorial |

### Medium Priority (M2-M4)

| Current Location | New Location | Notes |
|------------------|--------------|-------|
| `docs/sanity-checks/` | `spec/core/10-conformance.md` + test vectors | Reorganize as conformance tests |
| `poc-spa/README.md` | `guides/implementors/01-renderer.md` | Extract lessons learned |
| `source-of-truth/*.md` | `spec/core/*` or `reference/patterns/*` | Distribute appropriately |

### Low Priority (M8)

| Current Location | New Location | Notes |
|------------------|--------------|-------|
| `docs/project-management/*` | `meta/project/*` | Keep internal |
| `docs/compliance/*` | `spec/core/10-conformance.md` | Consolidate |
| `docs/operations/*` | `meta/project/operations.md` | Internal ops |

---

## Next Steps

### Add to M1 (Bonus Task)

**Task 5 (Bonus):** Restructure documentation  
- [ ] Create new directory structure
- [ ] Move core spec files to new locations
- [ ] Update INDEX.md with new structure
- [ ] Create templates for spec sections and guides
- [ ] Update cross-references

**Time:** 2-3 hours  
**Benefits:** M1 spec updates go to the right places from the start

### During M2-M4

As we implement:
- Write implementation guides based on our experience
- Extract patterns from M1 analysis into pattern library
- Create examples from our test packages
- Document APIs as we build them

### During M8 (Documentation Milestone)

- Fill any remaining gaps
- Polish for v1.0.0
- Ensure RFC-readiness
- External review

---

## Example: Renderer Implementation Guide Structure

```markdown
# Implementing the Rendering Engine

## What You'll Build

A deterministic prompt rendering engine that executes RPG packages.

**Conformance:** Tier 1 (Core Renderer)  
**Estimated Time:** 40-60 hours  
**Difficulty:** Intermediate

## Prerequisites

### Knowledge
- [ ] Read spec/core/01-architecture.md
- [ ] Read spec/core/05-rendering-engine.md
- [ ] Read spec/core/06-context-system.md
- [ ] Understand deterministic randomness

### Dependencies
Choose your language:
- **TypeScript:** Node 18+, prefer functional style
- **Python:** 3.10+, type hints required
- **Rust:** 1.70+, for high-performance

### Reading Order
1. spec/core/05-rendering-engine.md (MUST read)
2. spec/core/04-template-language.md (MUST read)
3. spec/core/03-data-model.md (reference as needed)

## Architecture Overview

```
┌─────────────┐
│   Package   │  Load package manifest
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Rulebook   │  Select entry promptsection
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────────┐
│      Three-Phase Rendering          │
│  1. Selection  →  2. Enrichment  →  │
│     3. Rendering                    │
└──────┬──────────────────────────────┘
       │
       ▼
┌─────────────┐
│   Output    │  Deterministic string
└─────────────┘
```

## Phase 1: Load Packages (4-6 hours)

**Goal:** Load and validate package YAML

**Steps:**
1. Implement Package data model
2. Write YAML parser
3. Validate manifest schema
4. Handle namespaces

**Test:**
```typescript
const pkg = loadPackage('minimal.yaml');
assert(pkg.namespace === 'example');
```

**Spec Reference:** spec/core/02-package-format.md

## Phase 2: Template Parser (8-12 hours)

**Goal:** Parse template syntax into AST

**Steps:**
1. Implement lexer
2. Build parser (recursive descent)
3. Handle references with min/max
4. Support separator syntax

**Test:**
```typescript
const ast = parseTemplate('{adjective?min=0,max=1} {noun}');
assert(ast.references.length === 2);
```

**Spec Reference:** spec/core/04-template-language.md

## Phase 3: Seeded Random (2-4 hours)

**Goal:** Deterministic RNG

**Implementation:**
```typescript
class SeededRandom {
  constructor(seed: string) {
    this.state = hashSeed(seed);
  }
  
  next(): number {
    // Xorshift or similar
  }
  
  choice<T>(array: T[]): T {
    return array[Math.floor(this.next() * array.length)];
  }
}
```

**Test:** Same seed → same sequence

**Spec Reference:** spec/core/05-rendering-engine.md §5.3.2

## Phase 4: Selection Phase (6-8 hours)

**Goal:** Select values from datatypes

**Implementation:**
```typescript
function phaseSelection(
  template: Template,
  context: Context,
  rng: SeededRandom
): void {
  for (const ref of template.references) {
    const count = rng.intRange(ref.min, ref.max);
    const values = selectFromDatatype(ref.datatype, count, rng);
    context.set(ref.name, values, 'prompt');
  }
}
```

**Test:** Same seed → same selections

## Phase 5: Context System (6-8 hours)

**Goal:** Scoped key-value store

**Implementation:**
```typescript
class Context {
  private scopes = {
    '.global': new Map(),
    '.prompt': new Map(),
  };
  
  get(key: string, scope = '.prompt'): any {
    // Hierarchical fallback
  }
  
  set(key: string, value: any, scope = '.prompt'): void {
    // Store in scope
  }
}
```

**Spec Reference:** spec/core/06-context-system.md

## Phase 6: Enrichment Phase (4-6 hours)

**Goal:** Execute Rules/Decisions (from M1)

[Will be filled after M1 completes]

## Phase 7: Rendering Phase (6-8 hours)

**Goal:** Replace template placeholders

**Implementation:**
```typescript
function phaseRendering(
  template: Template,
  context: Context
): string {
  let output = template.text;
  
  for (const ref of template.references) {
    const value = context.get(ref.name);
    const text = formatValue(value, ref.separator);
    output = output.replace(`{${ref.name}}`, text);
  }
  
  return output;
}
```

**Test:** Context values → final string

## Testing Strategy

### Unit Tests
- [ ] Package loading
- [ ] Template parsing
- [ ] Seeded RNG
- [ ] Each phase independently

### Integration Tests
- [ ] Full rendering pipeline
- [ ] Multiple prompts in batch
- [ ] Nested promptsections

### Conformance Tests
Run official test suite:
```bash
npm run conformance-test
```

## Common Pitfalls

### Pitfall: Non-Deterministic Randomness
**Problem:** Using `Math.random()` breaks determinism  
**Solution:** Always use SeededRandom instance

### Pitfall: Scope Confusion
**Problem:** Setting values in wrong scope  
**Solution:** Be explicit: `context.set(key, value, '.prompt')`

## Performance Considerations

- Parse templates once, reuse AST
- Pool string allocations
- Cache datatype lookups
- Lazy-load large packages

## Reference Implementation

Our TypeScript reference: `reference-impl/typescript/packages/core/`

**Key files:**
- `renderer/engine.ts` - Main rendering loop
- `parser/template-parser.ts` - Template parsing
- `context/context-store.ts` - Context system

## Conformance Checklist

### MUST Requirements
- [ ] Deterministic with same seed
- [ ] Three-phase rendering
- [ ] Context scopes
- [ ] Template syntax (EBNF)
- [ ] Error codes from spec

### SHOULD Requirements
- [ ] Performance benchmarks
- [ ] Debug mode
- [ ] Logging hooks

## Next Steps

1. **Add Validation:** guides/implementors/02-validator.md
2. **Optimize:** Reference section on performance
3. **Extend:** spec/extensions/ for optional features
```

This guide structure makes it MUCH easier to implement!

---

## Summary

**Proposed Structure:**
- **spec/** - Normative (RFC-ready)
  - core/ - Required specification
  - extensions/ - Optional features
  - conformance/ - Test suites
  
- **guides/** - Informative
  - implementors/ - Per-component guides
  - authors/ - Package authoring
  - tutorials/ - Learn by doing
  
- **reference/** - Examples & patterns
  - examples/ - Complete packages
  - patterns/ - Solutions library
  - api/ - API documentation
  
- **meta/** - Internal (not published)
  - project/ - Milestones, roadmap
  - decisions/ - ADRs

**Benefits:**
- Clear entry points for different audiences
- Component isolation (implement parts independently)
- RFC-ready structure for v1.0.0
- Separates spec from guides from project management
- Progressive disclosure (tutorials → guides → spec)

**Migration:**
- Phase 1 (M1): Restructure directories, move core files
- Phase 2 (M2-M4): Write guides as we implement
- Phase 3 (M8): Polish for publication

This structure matches how successful specs like OAuth2, OpenID Connect, and IETF RFCs are organized, making it much easier for external implementors to understand and adopt.

