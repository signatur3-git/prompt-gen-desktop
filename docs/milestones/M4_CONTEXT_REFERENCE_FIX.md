# M4 Bug Fix - Context Reference Selection

**Issue:** "Render error: Selection error: Datatype not found: context:article"

**Root Cause:** Phase 1 (Selection) was trying to select from "context:article" as if it were a datatype, but context references should be skipped during selection because they're populated by Rules during Phase 2 (Enrichment).

---

## The Fix

### 1. Updated Phase 1 Selection ✅

**File:** `renderer/renderer.rs`

**Before:**
```rust
// Select from the target datatype
let value = selector.select(&reference.target)?;
selected.insert(ref_name, value);
```

**After:**
```rust
// Skip context references - they'll be populated by Rules during Phase 2
if reference.target.starts_with("context:") {
    continue;
}

// Select from the target datatype
let value = selector.select(&reference.target)?;
selected.insert(ref_name, value);
```

**Why:** Context references don't need selection - they're computed values that will be written to context by Rules.

---

### 2. Updated minimal.yaml ✅

**File:** `test-packages/minimal.yaml`

**Before:**
```yaml
article:
  target: computed  # ❌ Unclear what this means
```

**After:**
```yaml
article:
  target: context:article  # ✅ Clear: read from context.article
```

**Why:** Makes it explicit that this reference reads from context, not from a datatype.

---

## How It Works Now

### The Flow:

**Template:**
```yaml
template: "{article} {color} {object}"
references:
  article:
    target: context:article  # Special: read from context
  color:
    target: test:colors      # Normal: select from datatype
  object:
    target: test:objects     # Normal: select from datatype
```

**Phase 1 (Selection):**
```
- Parse template → {article} {color} {object}
- Check each reference:
  - article: target = "context:article" → SKIP (will be in context)
  - color: target = "test:colors" → SELECT "orange"
  - object: target = "test:objects" → SELECT "ball"
- Result: selected = { color: "orange", object: "ball" }
```

**Phase 2 (Enrichment):**
```
- Execute Rule: compute_article
  - Read: color.tags.article = "an"
  - Write: context.article = "an"
- Result: context = { article: "an" }
```

**Phase 3 (Rendering):**
```
- {article} → not in selected, check context → "an" ✅
- {color} → in selected → "orange" ✅
- {object} → in selected → "ball" ✅
- Result: "an orange ball" ✅
```

---

## Convention Established

### Reference Target Formats:

**1. Datatype Selection:**
```yaml
target: namespace:datatype
target: test:colors
```
→ Select value from datatype during Phase 1

**2. Context Reference:**
```yaml
target: context:key
target: context:article
```
→ Skip Phase 1, read from context during Phase 3 (populated by Rules in Phase 2)

**3. PromptSection (future):**
```yaml
target: namespace:promptsection
target: compositions:scene
```
→ Nested rendering (M5 feature)

---

## Testing

**Now you can:**
1. Reload minimal.yaml
2. Select "with_article" promptsection
3. Render with different seeds
4. Should see:
   - "a red ball"
   - "a blue ball"
   - "an orange ball" ✅

**The fix is deployed!** The app should auto-reload with the changes.

---

## Summary

**Problem:** Trying to select from "context:article" as if it were a datatype  
**Solution:** Skip context: references during Phase 1 selection  
**Convention:** target format indicates behavior:
- `namespace:datatype` → select
- `context:key` → read from context

**Status:** ✅ FIXED! Article computation should now work!

---

**Try rendering again!** 🚀

