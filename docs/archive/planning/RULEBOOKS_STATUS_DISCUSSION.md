# Rulebooks Status & 1.0.0 Plan Discussion

**Date:** 2025-12-17  
**Question:** Are Rulebooks part of v1.0.0 or deferred?  
**Answer:** SPEC'D BUT NOT IMPLEMENTED YET

---

## TL;DR

**Rulebooks are:**
- ✅ **Fully specified** in the source-of-truth docs
- ✅ **Part of RFC 0001** (v1.0.0.rc1)
- ✅ **Documented** in architecture and sanity checks
- ❌ **NOT implemented** in M1-M7 reference implementation
- ⏳ **Deferred** to post-M7 (likely M8 or separate milestone)

**Current workaround:** Users select promptsections directly (like PoC)

---

## What Rulebooks Are (From Spec)

### Definition (from source-of-truth/component-overview.md)

> **Rulebook**: Namespaced config with weighted entry promptsections (e.g., `{start: 0.7, twist: 0.3}`) for batch generation.

### Full Specification (from RFC 0001)

> **Rulebook** – Weighted entry promptsections, batch configuration (count, seed derivation), and required interfaces.

### Key Features

1. **Entry Points** - Multiple weighted promptsections to choose from
2. **Batch Generation** - Generate N prompts, selecting entry point per prompt
3. **Context Setup** - Can set initial context values before rendering
4. **Required Interfaces** - Declare what context contracts are needed
5. **Weighted Selection** - Different entry promptsections with different probabilities

---

## Example from Documentation

From `docs/sanity-checks/example-sentences.md`:

```yaml
rulebooks:
  adventure_daily:
    namespace: rulebooks.adventure.daily
    entrypoint: prompt_sections.daily_quest
    depends_on:
      - ontologies.en.articles
      - ontologies.de.articles
      - prompt_sections.creature_intro
      - prompt_sections.daily_quest
    context_contracts:
      - contracts.random.seeded
      - contracts.language.articles
      - contracts.language.articles.de
```

**This shows:**
- Named rulebook (`adventure_daily`)
- Entry point (`daily_quest` promptsection)
- Dependencies (required packages/components)
- Context contracts (what interfaces are needed)

---

## What's Currently Implemented (M1-M7)

### Data Model (models.rs)

```rust
pub struct Namespace {
    pub id: String,
    pub datatypes: HashMap<String, Datatype>,
    pub prompt_sections: HashMap<String, PromptSection>,
    pub separator_sets: HashMap<String, SeparatorSet>,
    pub rules: Vec<Rule>,
    pub decisions: Vec<Decision>,
    // ❌ NO RULEBOOKS FIELD!
}
```

**Missing:**
- `rulebooks` field in Namespace
- Rulebook struct definition
- Weighted entry point selection
- Batch configuration

### Current User Flow

**What users do now:**
1. Load package
2. Select a **specific promptsection** directly
3. Render that promptsection
4. Repeat for batch (manual selection each time)

**What's missing:**
- Can't define "give me variety from these 3 templates"
- Can't weight probabilities (70% start, 30% twist)
- Can't set up context before rendering
- Can't declare required interfaces

---

## Why Rulebooks Matter

### Problem They Solve

**Without Rulebooks:**
```
User: "Generate 100 adventure prompts"
System: "Which promptsection?"
User: "Uh... adventure_start? But I want variety..."
System: "Pick one. I'll generate 100 of the same structure."
```

**With Rulebooks:**
```
User: "Generate 100 adventure prompts using 'adventure_daily' rulebook"
System: "Got it. I'll mix:
  - 70% standard adventures (adventure_start)
  - 20% plot twists (adventure_twist)  
  - 10% side quests (adventure_side)"
Result: 100 varied prompts!
```

### Use Cases

1. **Variety in Batch Generation**
   - Mix different prompt structures
   - Weighted probabilities for different types

2. **Context Initialization**
   - Set up global context before rendering
   - "All prompts in this batch are 'daily quest' flavor"

3. **Interface Requirements**
   - Declare dependencies (article selection, gender agreement)
   - Validator can check if all contracts are met

4. **Package Organization**
   - Entry point for users ("use this rulebook!")
   - Better than exposing 50 promptsections

---

## Current Status in Implementation

### M1-M7 Reference Implementation

**What's Built:**
- ✅ M1: Architecture & decisions
- ✅ M2: Package loading/saving
- ✅ M3: Basic rendering (promptsections)
- ✅ M4: Context store & rules
- ✅ M5: Advanced features (article, plural, coordination)
- ✅ M6: CLI validator
- ✅ M7: Authoring tool (in progress - Phase 2 complete)

**Rulebook Support:**
- ❌ Not in data model
- ❌ Not in authoring tool
- ❌ Not in renderer
- ❌ Not in validator

### Why Deferred

**Complexity:**
- Need weighted selection algorithm
- Need batch coordination
- Need context pre-initialization
- Need interface validation

**Priority:**
- M1-M7 focused on **core rendering** (templates work)
- M1-M7 focused on **visual authoring** (no YAML editing)
- Rulebooks are **orchestration layer** on top

**Decision:**
- Get core working first
- Add orchestration later
- Users can manually batch generate for now

---

## What This Means for v1.0.0

### Option 1: v1.0.0 WITHOUT Rulebooks (Current Track)

**Release as:**
- ✅ Core rendering engine working
- ✅ Full template syntax
- ✅ Context system with rules
- ✅ Visual authoring tool
- ✅ Validator
- ❌ Rulebooks (marked as future feature)

**Users can:**
- Create packages
- Author datatypes/promptsections/rules
- Render individual promptsections
- Batch generate (manual selection)

**Users cannot:**
- Define variety in a single call
- Weight multiple entry points
- Use rulebooks from marketplace

**Version:** `1.0.0-rc1` or `1.0.0-alpha`

---

### Option 2: v1.0.0 WITH Rulebooks (Extended Timeline)

**Add before v1.0.0:**
- Rulebook data model
- Weighted entry point selection
- Batch rendering with variety
- Context pre-initialization
- Authoring tool support (RulebookEditor)
- Validator support

**Timeline Impact:**
- Current: M7 → M8 (docs) → M9 (publish) → v1.0.0 (2-3 weeks)
- With Rulebooks: +M10 (Rulebooks) → v1.0.0 (4-5 weeks)

**Version:** `1.0.0` (full spec)

---

### Option 3: v1.0.0 Core + v1.1.0 Rulebooks (Recommended)

**v1.0.0 Release:**
- All current features (M1-M9)
- Mark Rulebooks as "specified, coming in v1.1.0"
- Users can render individual promptsections

**v1.1.0 Release:**
- Add Rulebook support
- Backward compatible (packages still work)
- Enhanced user experience

**Benefits:**
- Get v1.0.0 out faster
- Validate core before adding complexity
- Users get value sooner
- Spec remains complete (just phased implementation)

**Version Path:** `1.0.0` → `1.1.0` (3-4 weeks later)

---

## Recommendation

### My Suggestion: Option 3 (Phased Release)

**Reasons:**

1. **M1-M7 Momentum** 
   - We're crushing milestones (90% faster than estimated)
   - Don't slow down to add complex feature
   - Get to v1.0.0 quickly

2. **Validate Core First**
   - Rendering engine is working
   - Context system is proven
   - Visual authoring tool is amazing
   - Let users test before adding orchestration

3. **Spec Completeness**
   - RFC 0001 already has Rulebooks
   - Documentation is complete
   - Just implementation is deferred
   - No spec changes needed

4. **Clean v1.1.0 Story**
   - "v1.0.0: Author and render prompts"
   - "v1.1.0: Batch variety with Rulebooks"
   - Clear value proposition for each

5. **Less Risk**
   - Rulebooks touch many systems (renderer, validator, authoring tool)
   - Better to add when core is stable
   - Can gather user feedback on what weights/features they need

---

## Implementation Plan for Rulebooks (Future)

### When: Post-v1.0.0 (M10 or v1.1.0)

### What to Build:

**1. Data Model (1 day)**
```rust
pub struct Rulebook {
    pub name: String,
    pub entry_points: HashMap<String, f32>, // promptsection -> weight
    pub initial_context: HashMap<String, Value>,
    pub required_interfaces: Vec<String>,
    pub batch_config: BatchConfig,
}
```

**2. Renderer Integration (2 days)**
- Weighted selection algorithm
- Batch loop with variety
- Context pre-initialization
- Interface checking

**3. Authoring Tool (1-2 days)**
- RulebookEditor.vue component
- Add/remove entry points
- Weight sliders
- Initial context editor

**4. Validator (1 day)**
- Validate entry points exist
- Validate weights sum correctly
- Check required interfaces available
- Verify initial context types

**5. CLI (1 day)**
- `rpg render --rulebook adventure_daily`
- Batch flag: `--count 100`
- Output format options

**Total Estimate:** 6-7 days (1 week)

---

## Action Items

### Immediate (Today)

1. **Document the decision** ✅ (this document)
2. **Update DEVELOPMENT_PLAN.md** to clarify Rulebooks deferred
3. **Update M7-M9 scope** to exclude Rulebooks
4. **Update RFC 0001** with implementation note

### Before v1.0.0 Release (M8-M9)

1. **Mark in documentation:** "Rulebooks specified, implementation in v1.1.0"
2. **Update examples** to show workaround (manual batch)
3. **Add to roadmap** showing v1.1.0 plan

### After v1.0.0 (v1.1.0 Development)

1. Create **M10: Rulebook Support** milestone
2. Implement data model, renderer, authoring tool, validator
3. Write migration guide
4. Release v1.1.0 with Rulebooks

---

## Questions for You

**Decision Point:**

Which option do you prefer?

**Option 1:** v1.0.0 without Rulebooks (fastest, ships in 2-3 weeks)
**Option 2:** v1.0.0 with Rulebooks (complete spec, ships in 4-5 weeks)  
**Option 3:** v1.0.0 core + v1.1.0 Rulebooks (phased, v1.0 in 2-3 weeks, v1.1 in 6-7 weeks)

**My Recommendation:** Option 3

---

## Summary

**Status:**
- ✅ Rulebooks are **fully specified** in RFC 0001 and source-of-truth
- ✅ Examples exist in sanity-checks and documentation
- ❌ **Not implemented** in M1-M7 reference implementation
- ⏳ **Deferred** to post-v1.0.0

**Recommendation:**
- Ship v1.0.0 **without** Rulebooks (mark as "coming in v1.1.0")
- Add Rulebooks in **v1.1.0** (~1 week of work)
- Users can manually batch generate in v1.0.0

**Your Call:**
You're the architect - what do you want to do? 😊

---

**This ensures:**
- Spec stays complete (Rulebooks documented)
- Implementation stays focused (core first)
- Users get value sooner (v1.0.0 faster)
- Complexity added later (when core proven)

**What do you think?** 🤔

