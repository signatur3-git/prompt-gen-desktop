# Tag Filtering Implementation Status

**Date:** 2025-12-18  
**Issue:** User still getting "scarred gray beard" despite `applies_to` tags  
**Question:** Is tag filtering implemented?

---

## Answer: PARTIALLY IMPLEMENTED ⚠️

### What IS Implemented ✅

**Simple tag filtering works:**
```yaml
# Filter by tag existence
filter: "tags.can_fly"  # ✅ WORKS - Select only values with can_fly tag

# Filter by tag value
filter: "tags.type == melee"  # ✅ WORKS - Select only melee types
filter: "tags.mood != dark"   # ✅ WORKS - Exclude dark moods

# Complex expressions
filter: "tags.can_fly && tags.size == large"  # ✅ WORKS - Both conditions
filter: "tags.can_fly || tags.can_swim"       # ✅ WORKS - Either condition
filter: "!tags.dangerous"                     # ✅ WORKS - NOT dangerous
```

**Implemented in:** `src/renderer/tag_expression.rs`

**How it works:**
- Filters based on the VALUE's own tags
- Supports: `&&` (AND), `||` (OR), `!` (NOT)
- Supports: `==`, `!=` comparisons
- Evaluates against the value being selected

---

### What is NOT Implemented ❌

**Cross-reference filtering DOES NOT work:**
```yaml
# These would require reading OTHER selected values:
filter: "ref:body_part.text in tags.applies_to"  # ❌ NOT IMPLEMENTED
filter: "tags.for_{ref:body_part.tags.type}"     # ❌ NOT IMPLEMENTED
filter: "compatible(ref:quality, ref:color)"     # ❌ NOT IMPLEMENTED
```

**Why it doesn't work:**
- Current implementation evaluates filters DURING selection
- At selection time, we don't have access to other references yet
- References are selected independently, then combined

**Current selection order:**
1. Select `quality` (doesn't know what `body_part` will be)
2. Select `color` (doesn't know what `body_part` will be)  
3. Select `body_part` (doesn't know what `quality` or `color` were)
4. Combine them in template

**To make cross-reference filtering work, would need:**
- Select references in dependency order
- Pass previously selected values to filter evaluator
- Re-architect selection phase to support this

---

## Why You're Still Getting "Scarred Gray Beard"

### The Tags ARE There:
```yaml
qualities:
  - text: scarred
    tags: {applies_to: [skin, face]}  # ✅ Tag exists

body_parts:
  - text: beard
    tags: {article: a}  # beard exists
```

### But NO Filter is Applied:
```yaml
feature_description:
  references:
    quality:
      target: common.characters:qualities
      # NO FILTER HERE - so ALL qualities can be selected
    body_part:
      target: common.characters:body_parts
```

### Even If We Added Filter, It Wouldn't Work:
```yaml
quality:
  filter: "ref:body_part.text in tags.applies_to"  # ❌ Syntax not supported
```

**Because:** Filter can only check the quality's own tags, not coordinate with body_part

---

## What Would Need to Be Implemented

### Option 1: Cross-Reference Filtering (Complex)

**Spec says this should work:**
```yaml
quality:
  target: common.characters:qualities
  filter: "ref:body_part.text in tags.applies_to"
```

**Implementation requirements:**
1. **Parse `ref:` syntax** in tag expressions ⏱️ 30 min
2. **Track selected values** during Phase 1 ⏱️ 1 hour
3. **Evaluate filters with context** of other selections ⏱️ 2 hours
4. **Re-order selection** to respect dependencies ⏱️ 2 hours
5. **Handle circular dependencies** (quality needs body_part, body_part needs quality?) ⏱️ 1 hour

**Total effort:** ~6-7 hours of implementation

**Complexity:** ⭐⭐⭐⭐ HIGH

---

### Option 2: Post-Selection Filtering (Easier)

**Alternative approach:**
- Select all references independently (current approach)
- After all selected, validate combinations
- Re-roll if invalid combination
- Limit retries to prevent infinite loops

**Implementation:**
```rust
// After Phase 1 selection:
let max_retries = 10;
for attempt in 0..max_retries {
    let selected = select_all_references();
    if validate_combination(selected, rules) {
        break; // Good combination!
    }
    // Bad combination, try again with different seed
}
```

**Total effort:** ~3-4 hours

**Complexity:** ⭐⭐⭐ MODERATE

**Downside:** May require multiple selection attempts, less efficient

---

### Option 3: Separate Datatype Per Body Part (Workaround)

**Current problem:**
- One `qualities` datatype for ALL body parts
- 10 qualities × 8 body parts = 80 combos, only ~30 make sense

**Workaround solution:**
```yaml
datatypes:
  eye_qualities:
    values:
      - text: piercing
      - text: intense
      - text: gentle
      - text: mysterious

  hair_qualities:
    values:
      - text: flowing
      - text: braided
      - text: wild

  skin_qualities:
    values:
      - text: weathered
      - text: scarred
      - text: delicate

# Use different qualities based on body part
references:
  eye_quality:
    target: common.characters:eye_qualities
  hair_quality:
    target: common.characters:hair_qualities
```

**Total effort:** ~2 hours (refactor package)

**Complexity:** ⭐⭐ LOW

**Downside:** 
- Less DRY (repeat quality definitions)
- More datatypes to maintain
- Loses flexibility of tags

---

## Current Status

### In Reference Implementation:

**Tag Filtering:**
- ✅ Simple tag checks: `tags.can_fly`
- ✅ Tag comparisons: `tags.type == melee`
- ✅ Complex expressions: `tags.A && tags.B`
- ❌ Cross-reference: `ref:other.text in tags.applies_to`
- ❌ Dynamic tag lookup: `tags.for_{ref:other.tags.type}`

**What This Means:**
- Can filter values based on their OWN tags
- Cannot filter based on OTHER selected values
- Character portrait combinations will remain unfiltered

---

## Recommendation

### Short Term (Now):

**Accept the limitation:**
- Document that cross-reference filtering not yet implemented
- Note that 34% of combos are good (66% problematic)
- Users can re-roll to get better combinations
- Not blocking for M8.5 or v1.0.0

### Medium Term (Post-v1.0.0):

**Implement Option 2 (Post-Selection Filtering):**
- Easier than full cross-reference support
- Solves 80% of the problem
- ~3-4 hours implementation
- Could be v1.1.0 feature

### Long Term (v1.1.0 or v2.0.0):

**Implement Option 1 (Full Cross-Reference Filtering):**
- Matches spec completely
- Most powerful and flexible
- ~6-7 hours implementation
- Would solve 95%+ of combinations

---

## Documentation Update

### TAG_FILTERING_ANALYSIS.md Should Note:

**Current implementation gap:**
- Spec describes cross-reference filtering: ✅ Documented
- Reference impl supports simple filtering: ✅ Implemented  
- Reference impl does NOT support cross-ref: ❌ Not implemented yet

**This means:**
- Our analysis of "45 minutes to 81% success" assumed feature was implemented
- Actually would require 6-7 hours to implement the feature first
- Then 45 minutes to configure it in the package
- Total: **~7 hours** not 45 minutes

---

## Summary

**Q:** Is `applies_to` implemented?

**A:** The TAGS are there, but cross-reference FILTERING is NOT implemented.

**What works:**
- Tags on values: ✅ Working
- Simple tag filters: ✅ Working (`tags.can_fly`)
- Complex tag filters: ✅ Working (`tags.A && tags.B`)

**What doesn't work:**
- Cross-reference filters: ❌ Not implemented
- Coordinated selection: ❌ Not implemented
- Filter syntax like `ref:body_part.text in tags.applies_to`: ❌ Not supported

**Why you see "scarred gray beard":**
- No filter applied to quality selection
- All 10 qualities can be selected with any of 8 body parts
- 80 total combinations, ~30 make sense
- Random selection gives ~34% good combos

**To fix:**
- Need to implement cross-reference filtering (6-7 hours)
- OR use post-selection validation (3-4 hours)  
- OR refactor package with separate datatypes per body part (2 hours)

**For M8.5 testing:** This is a known limitation, continue testing other features.

**For v1.0.0 release:** 🔴 **CRITICAL BLOCKER** - Must implement cross-reference filtering before shipping.

**See:** `BLOCKER_CROSS_REF_FILTERING.md` in spec root for implementation plan.

---

**Status:** Tag filtering is PARTIALLY IMPLEMENTED - simple filters work, cross-reference coordination does not. ⚠️

**v1.0.0 Impact:** 🔴 **BLOCKS RELEASE** - Spec documents feature that doesn't work = unacceptable for v1.0.0.

