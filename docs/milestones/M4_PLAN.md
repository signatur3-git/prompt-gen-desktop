# M4: Context & Coordination - Implementation Plan

**Timeline:** Week 7-8 (Jan 2026)  
**Status:** ⏳ **JUST STARTED!**  
**Goal:** Make "A red ball" work (with computed article from Rules)

---

## Objective

Implement the context system and coordination mechanisms (Rules, Decisions, Tag Filtering) to enable proper article selection, pluralization, and other coordination patterns identified in M1.

**Success Criteria:**
- Can render "A red ball" / "An orange ball" with correct articles
- Rules execute during enrichment phase
- Context store works (scoped key-value)
- Tag filtering works during selection
- All M1 example prompts work (at least the first 3)

---

## What M4 Adds to the Pipeline

### Current (M3):
```
Phase 1: Selection
  - Parse template → {color} {object}
  - Select values → "red" "ball"

Phase 2: Enrichment (STUB)
  - Pass through unchanged

Phase 3: Rendering
  - Substitute → "red ball"
```

### After M4:
```
Phase 1: Selection
  - Parse template → {article} {color} {object}
  - Select values → {color: "red", object: "ball"}
  - Tag filtering can constrain selection

Phase 2: Enrichment (RULES & DECISIONS)
  - Execute Rules in order
  - Rules can read selected values' tags
  - Rules can write to context
  - Decisions can perform complex logic
  - Example: compute_article reads color.tags.article → writes "a" to context

Phase 3: Rendering
  - Read from context OR selected values
  - Substitute → "a red ball"
```

---

## Architecture Overview

### New Components

**1. Context Store**
```rust
pub struct Context {
    scopes: HashMap<String, HashMap<String, ContextValue>>,
}

pub enum ContextValue {
    Text(String),
    Number(i32),
    Boolean(bool),
    List(Vec<String>),
}
```

**2. Rules Processor**
```rust
pub struct RulesProcessor<'a> {
    context: &'a mut Context,
    selected: &'a HashMap<String, SelectedValue>,
}

impl RulesProcessor {
    pub fn execute_rule(&mut self, rule: &Rule) -> Result<()>;
}
```

**3. Tag Filter (during selection)**
```rust
// In selector.rs
pub fn select_with_filter(
    &mut self, 
    reference: &str, 
    filter: Option<&str>
) -> Result<SelectedValue>
```

**4. Decisions Processor (if needed)**
```rust
pub struct DecisionsProcessor<'a> {
    context: &'a mut Context,
}
```

---

## Implementation Tasks

### Phase 1: Context Store (2-3 hours)

**Goal:** Implement scoped key-value context store

**Tasks:**
- [ ] Create `context.rs` module
- [ ] Implement `Context` struct with scope management
- [ ] Support get/set operations
- [ ] Support different value types (Text, Number, Boolean, List)
- [ ] Unit tests (15+)

**Files:**
- `src-tauri/src/context/mod.rs`
- `src-tauri/src/context/context.rs`
- `src-tauri/src/context/value.rs`

**Success:** Can store/retrieve values in different scopes

---

### Phase 2: Rules Data Model (1 hour)

**Goal:** Ensure Rule data structures are ready

**Tasks:**
- [ ] Verify `Rule` struct in `core/models.rs`
- [ ] Add any missing fields (phase, logic, conditions)
- [ ] Update serialization/deserialization
- [ ] Create test fixtures

**Files:**
- `src-tauri/src/core/models.rs` (may already have Rule struct from M2)

**Success:** Can parse Rules from YAML

---

### Phase 3: Rules Processor (4-5 hours)

**Goal:** Execute Rules during Phase 2 (Enrichment)

**Tasks:**
- [ ] Create `rules/mod.rs` module
- [ ] Implement `RulesProcessor` struct
- [ ] Support rule logic types:
  - `set` - Set context value from constant or reference
  - `compute` - Call built-in functions (e.g., compute_article)
  - `copy` - Copy from one context key to another
- [ ] Implement `compute_article` helper
- [ ] Execute all rules for a namespace in order
- [ ] Unit tests (20+)

**Files:**
- `src-tauri/src/rules/mod.rs`
- `src-tauri/src/rules/processor.rs`
- `src-tauri/src/rules/helpers.rs` (compute_article, etc.)

**Success:** Can execute the article computation rule

---

### Phase 4: Integrate Rules into Renderer (2 hours)

**Goal:** Phase 2 executes Rules instead of being a stub

**Tasks:**
- [ ] Update `renderer.rs` Phase 2
- [ ] Create Context at start of render
- [ ] Pass Context through phases
- [ ] Execute Rules after selection, before rendering
- [ ] Update Phase 3 to read from context for computed values
- [ ] Integration tests

**Files:**
- `src-tauri/src/renderer/renderer.rs` (update phase_2_enrichment)

**Success:** Render "a red ball" with computed article

---

### Phase 5: Tag Filtering (3-4 hours)

**Goal:** Allow constraining selection based on tags

**Tasks:**
- [ ] Update Reference struct to support filter expressions
- [ ] Parse filter syntax: `{datatype#mood:peaceful}`
- [ ] Implement tag matching in selector
- [ ] Support operators: `=`, `!=`, `exists`, `!exists`
- [ ] Unit tests (15+)

**Files:**
- `src-tauri/src/renderer/selector.rs` (add filtering)
- `src-tauri/src/renderer/template_parser.rs` (parse filter syntax)

**Success:** Can select "only peaceful colors" via tag filter

---

### Phase 6: Update Test Packages (1-2 hours)

**Goal:** Create test packages that use M4 features

**Tasks:**
- [ ] Update `minimal.yaml` to use Rules for article
- [ ] Create `article-test.yaml` with various article examples
- [ ] Create `tag-filter-test.yaml` with mood-based filtering
- [ ] Add documentation to each test package

**Files:**
- `reference-impl/rpg-desktop/test-packages/minimal.yaml` (update)
- `reference-impl/rpg-desktop/test-packages/article-test.yaml` (new)
- `reference-impl/rpg-desktop/test-packages/tag-filter-test.yaml` (new)

**Success:** All test packages load and render correctly

---

### Phase 7: UI Updates (1-2 hours)

**Goal:** Show context values in UI

**Tasks:**
- [ ] Update `RenderResult` to include context values
- [ ] Update `LivePreview.vue` to display context
- [ ] Show which rules executed
- [ ] Better visualization of the enrichment phase

**Files:**
- `src-tauri/src/renderer/renderer.rs` (update RenderResult)
- `src/components/LivePreview.vue` (add context display)

**Success:** Can see computed article value in UI

---

### Phase 8: Decisions (Optional - may defer to M5)

**Goal:** Support complex coordination logic

**Tasks:**
- [ ] Evaluate if needed for M4 goals
- [ ] If needed: Implement Decisions processor
- [ ] If not needed: Document deferral decision

**Files:**
- TBD based on evaluation

**Decision Point:** After Phase 4, evaluate if Decisions are needed for M4 scope

---

## Test Plan

### Unit Tests

**Context Store:**
- Get/set in different scopes
- Scope isolation
- Value type conversions
- Edge cases (missing keys, etc.)

**Rules Processor:**
- Execute simple set rules
- Execute compute_article
- Rules access selected values' tags
- Rules write to context
- Rule ordering matters

**Tag Filtering:**
- Filter by exact match
- Filter by exists/not exists
- Multiple filters
- No matches returns error

### Integration Tests

**End-to-End Scenarios:**
1. Render "a red ball" (article from rule)
2. Render "an orange ball" (different article)
3. Render with tag filter (mood:peaceful)
4. Multiple rules executing in sequence
5. Context values used in rendering

### Manual Testing

**In the app:**
1. Load minimal.yaml (with rules)
2. Render and verify article is correct
3. Check context display shows article value
4. Load tag-filter-test.yaml
5. Render with filters
6. Verify only matching values selected

---

## Minimal.yaml Updates

### Current (M3):
```yaml
prompt_sections:
  basic:
    template: "{color} {object}"
    references:
      color: { target: test:colors }
      object: { target: test:objects }
```

### Updated (M4):
```yaml
prompt_sections:
  basic:
    template: "{article} {color} {object}"
    references:
      color: { target: test:colors }
      object: { target: test:objects }
      article: { target: context:article }  # Read from context

rules:
  - name: compute_article
    phase: enrichment
    logic:
      type: compute
      function: article
      args:
        - ref: color.tags.article  # Read tag from selected color
      output: context:article      # Write to context
```

---

## Success Criteria (Detailed)

### Must Have (M4 Scope)
- ✅ Context store working (get/set in scopes)
- ✅ Rules execute during Phase 2
- ✅ Article computation works ("a" vs "an")
- ✅ Can read selected values' tags in rules
- ✅ Can write to context from rules
- ✅ Phase 3 can read from context
- ✅ Tag filtering works (basic `#tag:value`)
- ✅ Test packages demonstrate features
- ✅ UI shows context values

### Nice to Have (May defer)
- ⏳ Decisions framework (evaluate after Phase 4)
- ⏳ Complex tag filter expressions
- ⏳ First_selected() helper
- ⏳ All 6 M1 prompts working (target: first 3)

### Out of Scope (M5+)
- ❌ Parameters (`?min=1,max=3`)
- ❌ Separators (`&sep=comma_and`)
- ❌ Nested promptsections
- ❌ Pools

---

## Timeline Estimate

**Total: 14-19 hours (2-3 days focused work)**

| Phase | Task | Hours |
|-------|------|-------|
| 1 | Context Store | 2-3 |
| 2 | Rules Data Model | 1 |
| 3 | Rules Processor | 4-5 |
| 4 | Integrate into Renderer | 2 |
| 5 | Tag Filtering | 3-4 |
| 6 | Test Packages | 1-2 |
| 7 | UI Updates | 1-2 |
| 8 | Decisions (Optional) | TBD |

**Conservative: 3 days**  
**Optimistic: 2 days**

---

## Risk Assessment

### Low Risk
- ✅ Context store (straightforward HashMap)
- ✅ Rules data model (already partially defined)
- ✅ UI updates (similar to M3)

### Medium Risk
- ⚠️ Rules execution logic (new complexity)
- ⚠️ Tag filtering syntax (parsing complexity)
- ⚠️ Integration with existing pipeline

### Mitigation Strategies
1. **Start with Context Store** (low risk, foundational)
2. **Keep Rules simple** (only what's needed for article)
3. **Tag filtering MVP** (exact match only initially)
4. **Test incrementally** (after each phase)
5. **Defer Decisions** (if not needed for M4 goals)

---

## Dependencies

### M3 Complete ✅
- Three-phase pipeline working
- Template parser ready
- Selector ready to add filtering
- RenderResult can be extended

### External
- No new crates needed (use existing serde, etc.)
- Context store uses standard HashMap
- Rules logic is custom (no external framework)

---

## Documentation Updates

### After M4 Completion
- [ ] Update DEVELOPMENT_PLAN.md (M4 → COMPLETE, M5 → READY)
- [ ] Update README.md (4/7 milestones, 57.1%)
- [ ] Update docs/milestones/index.md (add M4 section)
- [ ] Update reference-impl/COMPLIANCE.md (context, rules, tag filtering)
- [ ] Create M4_COMPLETE.md
- [ ] Document any new decisions in DECISIONS.md

**See:** [DOCUMENTATION_UPDATE_CHECKLIST.md](../../../DOCUMENTATION_UPDATE_CHECKLIST.md)

---

## Next Steps

**Ready to start! Order of implementation:**

1. ✅ **M4_PLAN.md created** ← We are here!
2. ⏳ Create context module and implement Context store
3. ⏳ Verify/update Rule data structures
4. ⏳ Implement Rules processor
5. ⏳ Integrate into renderer Phase 2
6. ⏳ Add tag filtering to selector
7. ⏳ Update test packages
8. ⏳ Update UI
9. ⏳ Test and verify
10. ⏳ Complete M4!

**Let's build the context store first!** 🚀

---

**Status:** M4 Plan Complete - Ready to implement! ⏳

