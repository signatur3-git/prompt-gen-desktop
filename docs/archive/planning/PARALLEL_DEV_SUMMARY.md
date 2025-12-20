# Consistency Check & Development Plan Summary

**Created:** 2025-12-16  
**Purpose:** Summary of consistency analysis and path forward

---

## Executive Summary

The Random Prompt Generator specification repository has reached a critical juncture where **documentation and reference implementation need to evolve in parallel**. This document summarizes the current state and outlines the development plan.

---

## Current State

### Repository Structure

```
prompt-gen-spec/
├── docs/                    ✅ Formal specification (namespace-aware)
├── source-of-truth/        ✅ Detailed design documents
├── poc-spa/                 ⚠️  Simplified demo (no namespaces)
├── reference-impl/         🆕 Reference implementation (gitignored except docs)
└── DEVELOPMENT_PLAN.md     🆕 Parallel development roadmap
```

### Key Findings from Consistency Analysis

**Major Inconsistencies Identified:**

1. **Namespacing**: PoC lacks namespaces entirely; formal docs require them
2. **Context Interactions**: Conflicting approaches documented
   - Formal docs: Engine provides `context.request()`, `context.random()`
   - Source-of-truth: "Engine does NOT provide these operations"
3. **Decisions Framework**: Comprehensive in source-of-truth, absent from PoC and formal docs
4. **Template Syntax**: PoC is simplified subset
5. **Rules vs Decisions**: Two different approaches to coordination logic

**What Works Well:**

- ✅ Phase-based rendering (Selection → Enrichment → Rendering)
- ✅ Separator system (primary, secondary, tertiary)
- ✅ Tag system for metadata
- ✅ Deterministic RNG with seeds

---

## The Parallel Development Approach

### Philosophy

Instead of treating PoC and spec as separate, we're adopting a **symbiotic approach**:

1. **Documentation leads** - Spec concepts documented first
2. **Implementation validates** - Reference impl proves the design
3. **Feedback loop** - Implementation findings refine the spec
4. **Thought experiments first** - Validate on paper before coding

### Git Strategy

**What's tracked:**
- Specification documents (`docs/`, `source-of-truth/`)
- Decision logs (`reference-impl/*.md`)
- Project management documents

**What's gitignored:**
- Implementation code (`reference-impl/typescript/`, `reference-impl/python/`)
- Build artifacts, dependencies, etc.

**Why?** Keeps spec repo clean while allowing parallel development. Implementation will be extracted to separate repo when ready (Milestone 9).

---

## Critical Decision: Context Interaction Strategy

### The Big Question

How should authors coordinate between template elements?

**Use Cases:**
- Article selection (A vs An) with optional adjectives
- Pluralization (cat/cats) based on count
- Gender agreement (his/her/their)
- Tone consistency across elements
- Physical plausibility (swan swims, not flies)

### Two Approaches Under Consideration

#### Option A: Simple Rules (PoC approach)
```yaml
rules:
  - name: compute_article
    phase: enrichment
    logic:
      - set: article
        from: first_selected([ref:adjective, ref:noun]).tags.article
```

**Pros:** Simple, easy to understand, works for common cases  
**Cons:** Limited flexibility, hard to extend, no external integration

#### Option B: Decisions Framework (Source-of-truth)
```json
{
  "namespace": "featured.common",
  "name": "select_article",
  "inputs": {"word": "string"},
  "outputs": {"article": "string"},
  "processor": {
    "type": "rule_set",
    "rules": [
      {"condition": "word[0] in ['a','e','i','o','u']", "output": {"article": "an"}},
      {"condition": "true", "output": {"article": "a"}}
    ]
  }
}
```

**Pros:** Very flexible, extensible, supports LLM integration  
**Cons:** Complex for simple cases, steeper learning curve

#### Option C: Hybrid
- Use Rules for simple enrichment
- Use Decisions for complex/reusable logic

**Pros:** Best of both worlds  
**Cons:** Two systems to learn

### Resolution Process (Milestone 1)

**We won't decide by debate** - we'll decide by **thought experiments**:

1. Define 15+ real text-to-image prompts (simple → complex)
2. Reverse-engineer what package authors would need to define
3. Test each approach against real scenarios
4. See where simple Rules break down
5. See if Decisions add unnecessary complexity
6. Make informed decision based on evidence

**See:** `docs/examples/text-to-image-prompts.md` for scenarios

---

## Development Milestones

### Milestone 1: Design Validation (Weeks 1-2) 🔴 Current Focus

**Goals:**
- Validate context interaction strategy through real scenarios
- Resolve DEC-0001 (context operations) and DEC-0002 (Rules vs Decisions)
- Define minimal engine primitive set

**Deliverables:**
- [ ] 15 example prompts with complexity analysis
- [ ] Reverse-engineered package designs for each
- [ ] Evaluation of Rules vs Decisions for each scenario
- [ ] Updated spec with chosen approach

**Success Criteria:**
- Team consensus on strategy
- All scenarios solvable without adding scripting language
- Clear boundary between engine and author logic

### Milestone 2: Foundation (Weeks 3-4)

Package system, data models, YAML loading, validation

### Milestone 3: Basic Rendering (Weeks 5-6)

Three-phase pipeline, template parsing, deterministic RNG

### Milestone 4: Context & Coordination (Weeks 7-8)

Implement chosen approach from M1, solve real-world prompts

### Milestone 5: Advanced Features (Weeks 9-10)

Nested templates, tag filtering, pools (if needed)

### Milestone 6: Validation & CLI (Weeks 11-12)

Comprehensive validator, CLI tools, error messages

### Milestone 7: Web Authoring Tool (Weeks 13-14)

Visual package editor, live preview, improved over current PoC

### Milestone 8: Documentation (Weeks 15-16)

Finalize spec, write guides, ensure consistency

### Milestone 9: Extraction (Weeks 17-18)

Move implementation to separate repo, publish v1.0.0-rc1 spec

**See:** `DEVELOPMENT_PLAN.md` for full details

---

## Design Philosophy

### Templates, Not Scripts

**Good (declarative):**
```yaml
template: "A {adjective?min=0,max=1} {noun}"
references:
  adjective: adjectives
  noun: animals
```

**Bad (imperative):**
```javascript
if (random() < 0.5) {
  output += selectAdjective() + " ";
}
output += selectNoun();
```

### Low-Level Primitives Only

**Engine should provide:**
- ✅ Random selection with seed
- ✅ Context get/set
- ✅ Template parsing
- ✅ Phase orchestration
- ✅ Basic list operations?

**Engine should NOT provide:**
- ❌ Article selection logic
- ❌ Pluralization rules
- ❌ Gender agreement
- ❌ Tone matching
- ❌ Any domain-specific semantics

**Authors define domain logic** through tags, rules, and/or decisions

### Real-World Driven

Design decisions validated against real text-to-image prompts:
- From simple ("A red ball")
- To complex ("Three knights with ornate armor beside their horses in a torch-lit courtyard")

If the system can elegantly handle scene descriptions, it can handle most prompt types.

---

## Action Items

### This Week (M1 Start)

- [ ] Review `docs/examples/text-to-image-prompts.md`
- [ ] Add more scenarios if needed (gaming prompts, character sheets, etc.)
- [ ] Create `docs/examples/authoring-analysis.md` - reverse engineer package designs
- [ ] Test Rules-only approach on S1-S3 (simple prompts)
- [ ] Test Decisions approach on S1-S3
- [ ] Document findings

### Next Week (M1 Completion)

- [ ] Test both approaches on M1-M3 (medium complexity)
- [ ] Try C1-C4 (complex scenes)
- [ ] Identify breaking points
- [ ] Make the call: Rules, Decisions, or Hybrid
- [ ] Update `reference-impl/DECISIONS.md` with DEC-0001, DEC-0002 resolutions
- [ ] Update formal spec documents
- [ ] Proceed to M2 with confidence

---

## Open Questions (Tracking in reference-impl/DECISIONS.md)

### Critical (M1)
- **DEC-0001**: Are `context.request()` and `context.random()` engine primitives or patterns?
- **DEC-0002**: Rules vs Decisions vs Hybrid?
- **DEC-0005**: Morphers depth (verb conjugation, case declension)?

### Important (M4)
- **DEC-0003**: Template reference format
- **DEC-0004**: Namespace resolution strategy

### Nice to Have (M6)
- External processor support (LLMs)
- Package marketplace
- Version compatibility

---

## Resources

### Documentation
- `CONSISTENCY_ANALYSIS.md` - Detailed inconsistency findings
- `DEVELOPMENT_PLAN.md` - Full milestone details
- `docs/examples/text-to-image-prompts.md` - Validation scenarios
- `reference-impl/QUICKSTART.md` - Implementation setup guide
- `reference-impl/DECISIONS.md` - Decision log
- `reference-impl/COMPLIANCE.md` - Feature tracking

### Source-of-Truth Documents
- `source-of-truth/article-coordination-problem.md` - Deep dive on deferred decisions
- `source-of-truth/context-interactions.md` - Decisions framework spec
- `source-of-truth/decision-patterns-guide.md` - 10 patterns for challenges
- `source-of-truth/template-engine.md` - Detailed syntax

### Current Implementations
- `poc-spa/` - Simple demo (simplified, no namespaces)
- `reference-impl/` - Will become full implementation (gitignored)

---

## Success Metrics

### Short-term (End of M1)
- [ ] DEC-0001 and DEC-0002 resolved
- [ ] All 15 scenarios have package designs
- [ ] Team aligned on approach
- [ ] Spec updated, ready for implementation

### Mid-term (End of M4)
- [ ] All example prompts render correctly
- [ ] Article coordination solved
- [ ] Pluralization works
- [ ] System feels like templates

### Long-term (End of M9)
- [ ] Complete reference implementation
- [ ] Validated spec (v1.0.0-rc1)
- [ ] Ready for external implementers
- [ ] Clean separation: spec repo vs impl repo

---

## Conclusion

The path forward is clear:

1. **Validate design through thought experiments** (M1)
2. **Build reference implementation in parallel with spec refinement** (M2-M8)
3. **Extract and publish when ready** (M9)

The PoC stays as a simple demo. The reference implementation lives gitignored in `reference-impl/` until it's ready to be extracted to its own repository.

**All semantic logic is author-defined.** The engine provides only generic primitives. Whether through Rules, Decisions, or both - that's what M1 will determine.

---

**Next Step: Start the thought experiments! 🚀**

See `docs/examples/text-to-image-prompts.md` to begin.

