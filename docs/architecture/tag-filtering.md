# Tag Filtering Guide

**Version:** 1.0.0  
**Status:** ✅ Implemented and Working  
**Last Updated:** 2025-12-17

**For learning:** See [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md)  
**For overview:** See [Architecture Overview](./overview.md#tag-filtering-m4-m5-)

---

## Overview

Tag filtering allows you to constrain value selection based on tags. This enables realistic constraints (e.g., only flying animals) or creative freedom (no filters = anything goes).

**Key use case:** Physical plausibility - ensure "deer" doesn't appear in "flying through the sky" prompts.

---

## Syntax

### Basic Tag Filter
```yaml
{reference#{filter_expression}}
```

### Examples

**Filter by boolean tag:**
```yaml
{animal#{tags.can_fly}}
```
- Selects only values where `can_fly` is `true`
- Filters out values where `can_fly` is `false` or missing

**Filter results:**
- ✅ eagle (can_fly: true)
- ✅ swan (can_fly: true)
- ✅ duck (can_fly: true)
- ❌ deer (can_fly: false)
- ❌ rabbit (can_fly: false)

---

## How It Works

### 1. Define Tags in Datatypes

```yaml
datatypes:
  animals:
    name: animals
    values:
      - text: "eagle"
        tags:
          can_fly: true
          can_swim: false
          can_run: false
      
      - text: "deer"
        tags:
          can_fly: false
          can_swim: false
          can_run: true
```

### 2. Use Filters in Templates

```yaml
prompt_sections:
  flying_scene:
    name: flying_scene
    template: "{article} {animal#{tags.can_fly}} flies overhead"
    references:
      article:
        target: "context:article"
      animal:
        target: "test:animals"
```

### 3. Rendering Process

**Phase 1: Selection with Filtering**
1. Parse template: Extract `animal` reference with filter `tags.can_fly`
2. Get all animals from datatype
3. Apply filter: Keep only animals where `can_fly == true`
4. Select randomly from filtered list

**Result:** Only flying animals appear in output

---

## Supported Tag Types

### Boolean Tags
```yaml
tags:
  can_fly: true
  is_aquatic: false
```

**Filter:** `{animal#{tags.can_fly}}`
- `true` → Included ✅
- `false` → Excluded ❌
- Missing → Excluded ❌

### String Tags
```yaml
tags:
  mood: "peaceful"
```

**Filter:** `{color#{tags.mood}}`
- Non-empty string → Included ✅
- Empty string → Excluded ❌
- Missing → Excluded ❌

### Number Tags
```yaml
tags:
  size: 5
  weight: 0
```

**Filter:** `{object#{tags.size}}`
- Non-zero → Included ✅
- Zero → Excluded ❌
- Missing → Excluded ❌

---

## Complete Examples

### Example 1: Movement Types

```yaml
# Define animals with movement capabilities
datatypes:
  animals:
    values:
      - text: "eagle"
        tags:
          can_fly: true
          can_swim: false
          can_run: false
      - text: "swan"
        tags:
          can_fly: true
          can_swim: true
          can_run: false
      - text: "deer"
        tags:
          can_fly: false
          can_swim: false
          can_run: true

# Use filters in templates
prompt_sections:
  flying:
    template: "{animal#{tags.can_fly}} soars"
    # Results: eagle, swan (never deer)
  
  swimming:
    template: "{animal#{tags.can_swim}} splashes"
    # Results: swan only
  
  running:
    template: "{animal#{tags.can_run}} sprints"
    # Results: deer only
```

### Example 2: Optional Filters

```yaml
# Realistic package - with filter
realistic_scene:
  template: "{animal#{tags.can_fly}} flies"
  # Only flying animals

# Absurdist package - no filter
absurd_scene:
  template: "{animal} flies"
  # Any animal (deer can fly in absurd world!)
```

**Key Point:** Filters are opt-in per reference, not enforced globally!

---

## Current Limitations (M4)

### ✅ Supported
- Simple tag existence checks: `tags.can_fly`
- Boolean evaluation
- String/number truthiness
- Nested braces in expressions

### 🔜 Future (M5+)
- Comparison operators: `tags.mood == "peaceful"`
- Logical operators: `tags.can_fly and tags.can_swim`
- Negation: `!tags.can_fly`
- Complex expressions: `(tags.requires_swim implies ref:water.present)`

---

## Testing

### Load Test Package
1. Start app: `npm run tauri:dev`
2. Import: `test-packages/tag-filter-test.yaml`
3. Render different scenes:
   - `flying_scene` → eagle, swan, duck only
   - `swimming_scene` → swan, duck only
   - `running_scene` → deer, rabbit only

### Verify Filtering
- Click "Render" multiple times with different seeds
- **Should NEVER see:**
  - Deer in flying_scene ❌
  - Eagle in running_scene ❌
  - Rabbit in swimming_scene ❌

---

## Best Practices

### 1. Use Descriptive Tag Names
```yaml
# Good
tags:
  can_fly: true
  is_nocturnal: true

# Avoid
tags:
  f: true
  n: true
```

### 2. Document Your Tags
```yaml
# Add comments explaining tag purpose
animals:
  values:
    - text: "swan"
      tags:
        can_fly: true      # Capable of flight
        can_swim: true     # Capable of swimming
        can_run: false     # Cannot run effectively
```

### 3. Make Filters Optional
```yaml
# Default: No filter (anything goes)
normal_scene:
  template: "{animal} appears"

# Constrained: With filter (realistic)
realistic_scene:
  template: "{animal#{tags.can_fly}} flies"
```

### 4. Handle Empty Results
If a filter excludes all values, the renderer will return an error:
```
Error: Datatype has no values after filtering
```

**Solution:** Ensure at least one value matches your filter!

---

## Architecture Notes

### Filter Evaluation Order
1. **Template Parse** → Extract filter expression
2. **Phase 1: Selection** → Apply filter before selection
3. **Weighted Random** → Select from filtered list

### Performance
- Filtering happens once per selection
- O(n) where n = number of values in datatype
- Negligible impact for typical package sizes

---

## Migration from No Filters

### Before (M3):
```yaml
template: "{animal} flies"
# Problem: Deer can fly!
```

### After (M4):
```yaml
template: "{animal#{tags.can_fly}} flies"
# Solution: Only flying animals!
```

**Backward Compatible:** Templates without filters still work!

---

## Cross-References

**For learning:**
- [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md) - Complete walkthrough with exercises
- [Getting Started](../guides/getting-started.md) - Tag filtering basics

**For reference:**
- [Components](./components.md) - Datatype tag structure
- [Template Syntax](./template-syntax.md) - Filter expression syntax
- [Overview](./overview.md) - System design

**Implementation:**
- [Tag Filter Test Package](../../reference-impl/rpg-desktop/test-packages/tag-filter-test.yaml)
- [M4 Completion](../milestones/M4_COMPLETE.md)

---

**Status:** Production Ready ✅  
**Tests:** 67 passing  
**Manual Testing:** Verified with flying birds & running rabbits! 🦅🐇

