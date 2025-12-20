# M5: Advanced Features - Implementation Plan

**Timeline:** Week 9-10 (December 2025)  
**Status:** ⏳ **READY TO START**  
**Depends On:** M4 (Context & Coordination) ✅ COMPLETE

---

## Objective

Implement advanced templating features to enable complex, realistic prompts as identified in M1 analysis.

**Success Criteria:**
- ✅ Nested promptsections work (template references template)
- ✅ Complex tag expressions (AND/OR/NOT)
- ✅ Pools for aggregation and drawing
- ✅ Separator sets (comma_and, semicolon_or, etc.)
- ✅ Min/max multiplicity (`{ref?min=0,max=3}`)
- ✅ All 6 M1 example prompts work

---

## Current Capabilities (M4)

### What Works Now
```yaml
# Simple reference
{color}

# Tag filtering (single condition)
{animal#{tags.can_fly}}

# Context reference
{article}

# Rules coordination
rules:
  - set: article
    from: "ref:color.tags.article"
```

### What Doesn't Work Yet
```yaml
# Nested promptsections ❌
{scene:intro}

# Complex tag expressions ❌
{creature#{tags.can_fly && tags.nocturnal}}
{weapon#{tags.type == "melee" || tags.magical}}

# Pools ❌
{context.pool.characters}

# Separator sets ❌
{colors?sep=comma_and}

# Min/max multiplicity ❌
{adjectives?min=0,max=3}
```

---

## M5 Phases

### Phase 1: Nested PromptSections (Priority: HIGH) ✅ COMPLETE
**Goal:** Templates can reference other templates

**Status:** ✅ **COMPLETE** - 2025-12-17  
**User Verified:** "Greetings, Alice! Take care. Charlie arrives at the tavern."

**Implementation:**
- ✅ Added recursion depth tracking (MAX_RECURSION_DEPTH = 10)
- ✅ New error type: `MaxRecursionDepth(usize, String)`
- ✅ Modified `render()` to use `render_with_depth()`
- ✅ Updated `phase_1_selection()` to detect and render nested promptsections
- ✅ Helper method `is_promptsection_reference()` distinguishes types

**Test Package:** `test-packages/nested-test.yaml`
- Simple nesting: `greeting` → datatypes only
- 1-level nesting: `full_greeting` → uses `greeting`
- 2-level nesting: `scene_description` → uses `full_greeting` + `action_phrase`

**Challenges Addressed:**
- ✅ Infinite recursion detection (depth limit)
- ✅ Scope management for nested contexts (each render has own context)
- ✅ Reference resolution order (works correctly)

**Time Actual:** 1 hour


---

### Phase 2: Complex Tag Expressions (Priority: HIGH) ✅ COMPLETE
**Goal:** Support AND/OR/NOT in tag filters

**Status:** ✅ **COMPLETE** - 2025-12-17  
**User Verified:** "Looks good. No values that are out of place and no errors."

**Implementation:**
- ✅ Created full recursive descent parser (`tag_expression.rs`, 450+ lines)
- ✅ Tokenizer handles operators, strings, numbers, booleans
- ✅ Proper operator precedence (OR < AND < NOT)
- ✅ AND (`&&`), OR (`||`), NOT (`!`), comparisons (`==`, `!=`)
- ✅ Updated selector to use expression parser
- ✅ 9 unit tests included

**Test Package:** `test-packages/complex-tags-test.yaml`
- AND: `tags.can_fly && tags.nocturnal` → owl, bat
- OR: `tags.type == "melee" || tags.magical` → sword, staff, wand
- NOT: `!tags.domesticated` → wolf, bear, rabbit
- Complex: `!tags.dangerous && tags.domesticated` → hamster
- Multiple: `tags.can_fly && !tags.nocturnal` → eagle

**Time Actual:** 2 hours

**Current:**
```yaml
# Only single condition
{animal#{tags.can_fly}}
```

**Target:**
```yaml
# Complex expressions
{creature#{tags.can_fly && tags.nocturnal}}
{weapon#{tags.type == "melee" || tags.magical}}
{animal#{tags.dangerous && !tags.domesticated}}
```

**Implementation:**
```rust
// In selector.rs
pub struct TagExpression {
    expr: Expr, // Use a simple expression parser
}

impl TagExpression {
    pub fn parse(filter: &str) -> Result<Self>;
    pub fn evaluate(&self, tags: &HashMap<String, TagValue>) -> Result<bool>;
}

enum Expr {
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Comparison(String, Operator, TagValue), // tags.type == "melee"
    BoolCheck(String), // tags.can_fly
}
```

**Test Cases:**
1. `{animal#{tags.can_fly && tags.can_swim}}` → swan, duck
2. `{weapon#{tags.type == "melee" || tags.magical}}` → sword, staff
3. `{creature#{!tags.domesticated}}` → wolf, dragon

**Time Estimate:** 6 hours

---

### Phase 3: Separator Sets (Priority: MEDIUM)
**Goal:** Proper list formatting

**Implementation:**
```rust
pub struct SeparatorSet {
    pub primary: String,    // ", "
    pub secondary: String,  // " and "
    pub tertiary: String,   // "; "
}

impl SeparatorSet {
    pub fn format(&self, items: &[String]) -> String {
        match items.len() {
            0 => String::new(),
            1 => items[0].clone(),
            2 => format!("{}{}{}", items[0], self.secondary, items[1]),
            _ => {
                let mut result = items[0..items.len()-1].join(&self.primary);
                result.push_str(&self.secondary);
                result.push_str(&items[items.len()-1]);
                result
            }
        }
    }
}
```

**Test Cases:**
```yaml
separatorsets:
  comma_and:
    primary: ", "
    secondary: " and "
  
# Usage:
template: "{colors?min=3,max=3&sep=comma_and}"

# Results:
# ["red", "blue", "green"] → "red, blue and green"
# ["red", "blue"] → "red and blue"
# ["red"] → "red"
```

**Time Estimate:** 3 hours

---

### Phase 4: Min/Max Multiplicity (Priority: HIGH)
**Goal:** Variable repetition of references

**Current:**
```yaml
# Only single selection
{color}  # → "red"
```

**Target:**
```yaml
# Multiple selections
{colors?min=0,max=3}  # → "red blue orange"
{adjectives?min=1,max=2}  # → "beautiful mysterious"
```

**Implementation:**
```rust
pub struct Reference {
    pub target: String,
    pub min: usize,  // Default: 1
    pub max: usize,  // Default: 1
    pub separator: Option<String>,  // Default: " "
    pub unique: bool,  // Default: false
}

impl Renderer {
    fn resolve_reference_with_multiplicity(
        &mut self,
        reference: &Reference
    ) -> Result<String> {
        let count = self.rng.gen_range(reference.min..=reference.max);
        let mut results = Vec::new();
        
        for _ in 0..count {
            let value = self.select_value(&reference.target)?;
            if reference.unique && results.contains(&value) {
                continue; // Skip duplicates
            }
            results.push(value);
        }
        
        Ok(results.join(reference.separator.as_deref().unwrap_or(" ")))
    }
}
```

**Test Cases:**
```yaml
# Variable adjectives
template: "{adjectives?min=0,max=3} {noun}"
# → "beautiful mysterious ancient castle"
# → "old tower"
# → "fortress"

# Unique items
template: "{colors?min=3,max=3&unique=true}"
# → "red blue orange" (never repeats)
```

**Time Estimate:** 4 hours

---

### Phase 5: Pools (Priority: MEDIUM)
**Goal:** Aggregate and draw from collections

**Use Cases:**
- Build a party of characters
- Collect items to describe
- Maintain state across sections

**Implementation:**
```rust
// In context.rs
impl Context {
    pub fn pool_add(&mut self, pool_key: &str, value: String) {
        // Add to list
    }
    
    pub fn pool_draw(
        &mut self, 
        pool_key: &str, 
        count: usize,
        rng: &mut Rng
    ) -> Result<Vec<String>> {
        // Random draw from pool
    }
    
    pub fn pool_remove(&mut self, pool_key: &str, value: &str) {
        // Remove specific value
    }
}
```

**Example:**
```yaml
rules:
  - name: build_party
    phase: enrichment
    logic:
      - pool_add: "party"
        from: "ref:character"
        count: 3

prompt_sections:
  party_intro:
    template: "Your party: {context.pool.party?sep=comma_and}"
```

**Time Estimate:** 5 hours

---

### Phase 6: Integration & Testing (Priority: HIGH)
**Goal:** Make all M1 example prompts work

**Test with M1 Examples:**
1. **Simple Portrait**
   ```
   A [mood] [species] [character_class] with [physical_trait]
   ```

2. **Complex Scene**
   ```
   [time_of_day] in [location]. [weather]. 
   [character_intro]. [action]. [mood_descriptor]
   ```

3. **Item Description**
   ```
   A [material] [item_type] with [decoration]. 
   [magical_property]. [history_hint]
   ```

**Create comprehensive test package:**
```yaml
id: test.m5-advanced
version: 1.0.0

# Tests:
# - Nested promptsections
# - Complex tag filters
# - Min/max multiplicity
# - Separator sets
# - Pools
```

**Time Estimate:** 6 hours

---

### Phase 7: UI Updates (Priority: LOW)
**Goal:** Show advanced features in UI

**Updates:**
- Display nested structure
- Show pool contents
- Visualize tag expressions
- Preview multiplicity ranges

**Time Estimate:** 2 hours

---

## Total Effort Estimate

| Phase | Effort | Priority |
|-------|--------|----------|
| 1. Nested PromptSections | 4 hours | HIGH |
| 2. Complex Tag Expressions | 6 hours | HIGH |
| 3. Separator Sets | 3 hours | MEDIUM |
| 4. Min/Max Multiplicity | 4 hours | HIGH |
| 5. Pools | 5 hours | MEDIUM |
| 6. Integration & Testing | 6 hours | HIGH |
| 7. UI Updates | 2 hours | LOW |
| **TOTAL** | **30 hours** | — |

**Estimated:** 30-40 hours (1.5-2 weeks)

---

## Success Criteria Checklist

### Core Features
- [ ] Nested promptsections work
- [ ] Can reference templates from templates
- [ ] Infinite recursion detected and prevented
- [ ] Complex tag expressions parse correctly
- [ ] AND/OR/NOT operations work
- [ ] Min/max multiplicity works
- [ ] Separator sets format lists properly
- [ ] Pools can aggregate and draw values

### M1 Examples
- [ ] Example 1: Simple Portrait renders
- [ ] Example 2: Complex Scene renders
- [ ] Example 3: Item Description renders
- [ ] Example 4: Environmental Description renders
- [ ] Example 5: Action Sequence renders
- [ ] Example 6: Dialogue Snippet renders

### Quality
- [ ] All unit tests pass (target: 100+)
- [ ] All test packages work
- [ ] No regressions in M3/M4 features
- [ ] Code coverage >80%
- [ ] Documentation complete

---

## Risk Assessment

### High Risk
- **Nested sections complexity:** Could cause subtle bugs
  - *Mitigation:* Extensive testing, recursion limits
  
- **Tag expression parsing:** Could be fragile
  - *Mitigation:* Use well-tested parser library or keep simple

### Medium Risk
- **Pools state management:** Complex scope interactions
  - *Mitigation:* Clear scope rules, good tests

### Low Risk
- **Separator sets:** Straightforward implementation
- **Min/max multiplicity:** Extension of existing selection

---

## Dependencies

### M4 Complete ✅
- Context system working
- Rules engine working
- Tag filtering (simple) working
- 3-phase pipeline stable

### External
- None (all features can be implemented with existing tech stack)

---

## Post-M5 Outlook

After M5, we'll have:
- Complete template engine
- All M1 features working
- Rich prompt generation
- Solid foundation for M6 (validation)

**Remaining for v1.0:**
- M6: Package Validation & CLI
- M7: Web Authoring Tool

---

**Status:** Ready to begin! 🚀

**Next Action:** Start Phase 1 (Nested PromptSections)

