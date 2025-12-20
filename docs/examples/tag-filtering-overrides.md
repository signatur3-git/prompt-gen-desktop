# Tag Filtering Override Examples

**Purpose:** Demonstrate how tag filtering supports both realistic and creative/absurdist packages  
**Date:** 2025-12-16

---

## The Design Principle

**Tag filters are opt-in per reference, not enforced globally.**

- **With filter:** `activities#{expression}` → Filtering applied
- **Without filter:** `activities` → All values available
- **Author chooses** when to enforce constraints

---

## Example 1: Realistic Package (Filters ON)

```yaml
namespace: realistic.nature
metadata:
  description: "Physically plausible nature scenes"

datatypes:
  - name: animals
    values:
      - text: "swan"
        tags:
          can_swim: true
          can_fly: true
          can_run: false
      - text: "deer"
        tags:
          can_swim: false
          can_fly: false
          can_run: true
      - text: "eagle"
        tags:
          can_swim: false
          can_fly: true
          can_run: false

  - name: activities
    values:
      - text: "swimming"
        tags:
          requires_can_swim: true
      - text: "running"
        tags:
          requires_can_run: true
      - text: "flying"
        tags:
          requires_can_fly: true

promptsections:
  - name: realistic_scene
    template: "{article} {animal} {activity}"
    references:
      animal: animals
      
      # FILTER APPLIED - Only compatible activities
      activity: activities#{
        (tags.requires_can_swim implies ref:animal.tags.can_swim) and
        (tags.requires_can_fly implies ref:animal.tags.can_fly) and
        (tags.requires_can_run implies ref:animal.tags.can_run)
      }
      
      article: computed

# Results:
# ✅ "A swan swimming" (swan can swim)
# ✅ "A deer running" (deer can run)
# ✅ "An eagle flying" (eagle can fly)
# ❌ "A swan running" (FILTERED OUT - swan cannot run)
# ❌ "An eagle swimming" (FILTERED OUT - eagle cannot swim)
```

---

## Example 2: Absurdist Package (Filters OFF)

```yaml
namespace: absurd.world
metadata:
  description: "Where physical laws are suggestions"

imports:
  # Reuse datatypes from realistic package!
  - namespace: realistic.nature

promptsections:
  - name: absurd_scene
    template: "{article} {animal} {activity}"
    references:
      # Same datatypes, NO FILTERS = anything goes
      animal: realistic.nature:animals
      activity: realistic.nature:activities  # No filter!
      
      article: computed

# Results:
# ✅ "A swan swimming" (still possible)
# ✅ "A swan running" (NOW ALLOWED - no filter!)
# ✅ "An eagle swimming" (NOW ALLOWED - absurd but intentional!)
# ✅ "A deer flying" (NOW ALLOWED - surreal!)
```

**Key Point:** Same datatypes, different constraints. No redefinition needed!

---

## Example 3: Mixed Package (Selective Filtering)

```yaml
namespace: fantasy.realm
metadata:
  description: "Realistic animals, magical activities"

imports:
  - namespace: realistic.nature

datatypes:
  - name: magical_activities
    values:
      - text: "teleporting"
        tags:
          magical: true
      - text: "shapeshifting"
        tags:
          magical: true
      - text: "swimming"
        tags:
          magical: false
          requires_can_swim: true

promptsections:
  - name: fantasy_scene
    template: "{article} {animal} {activity}"
    references:
      animal: realistic.nature:animals
      
      # Realistic animals, but magical activities have no constraints
      activity: magical_activities#{
        tags.magical or 
        (tags.requires_can_swim implies ref:animal.tags.can_swim)
      }
      
      article: computed

# Results:
# ✅ "A swan swimming" (realistic, follows rules)
# ✅ "A swan teleporting" (magical, no constraints needed)
# ✅ "A deer shapeshifting" (magical, allowed)
# ❌ "A deer swimming" (FILTERED - not magical, deer can't swim)
```

---

## Example 4: Package Override Extension

```yaml
namespace: crazy.mutations
metadata:
  description: "Mutated animals with new abilities"

imports:
  - namespace: realistic.nature

datatypes:
  - name: mutant_animals
    extends: realistic.nature:animals
    override_tags:
      # Override to give all animals all abilities
      can_swim: true
      can_fly: true
      can_run: true
    values:
      # Inherits swan, deer, eagle but with new capabilities

promptsections:
  - name: mutant_scene
    template: "{article} {animal} {activity}"
    references:
      animal: mutant_animals
      
      # Can now apply realistic filters because all animals have all abilities
      activity: realistic.nature:activities#{
        (tags.requires_can_swim implies ref:animal.tags.can_swim) and
        (tags.requires_can_fly implies ref:animal.tags.can_fly) and
        (tags.requires_can_run implies ref:animal.tags.can_run)
      }
      
      article: computed

# Results:
# ✅ "A swan swimming" (had this before)
# ✅ "A swan running" (NOW ALLOWED - mutant swan can run!)
# ✅ "A deer flying" (NOW ALLOWED - mutant deer can fly!)
# All combinations valid because mutant animals have all abilities
```

---

## Example 5: "Crazy World" Use Case

**Question:** Can someone create a package where swans roll like bowling balls without redefining everything?

**Answer:** Yes! Multiple approaches:

### Approach A: No Filters (Simplest)

```yaml
namespace: bowling.world
imports:
  - namespace: realistic.nature

datatypes:
  - name: bowling_activities
    values:
      - text: "rolling like a bowling ball"
      - text: "spinning like a top"
      - text: "bouncing like a rubber ball"

promptsections:
  - name: bowling_scene
    template: "{article} {animal} {activity}"
    references:
      animal: realistic.nature:animals  # Reuse, no filter
      activity: bowling_activities  # No filter
      article: computed

# Results:
# ✅ "A swan rolling like a bowling ball"
# ✅ "A deer spinning like a top"
# ✅ "An eagle bouncing like a rubber ball"
```

### Approach B: Add New Tags

```yaml
namespace: bowling.world
imports:
  - namespace: realistic.nature

datatypes:
  - name: bowling_animals
    extends: realistic.nature:animals
    add_tags:
      can_roll: true  # Add new capability
    values:
      # Inherits all animals, adds rolling capability

  - name: activities
    values:
      - text: "rolling"
        tags:
          requires_can_roll: true
      - text: "swimming"
        tags:
          requires_can_swim: true

promptsections:
  - name: scene
    template: "{article} {animal} {activity}"
    references:
      animal: bowling_animals
      
      # Filter now passes for rolling (all have can_roll: true)
      activity: activities#{
        (tags.requires_can_roll implies ref:animal.tags.can_roll) and
        (tags.requires_can_swim implies ref:animal.tags.can_swim)
      }

# Results:
# ✅ "A swan rolling" (now has can_roll tag)
# ✅ "A swan swimming" (still has can_swim)
# ❌ "A deer swimming" (still filtered - deer can't swim)
```

### Approach C: Override Mode

```yaml
namespace: bowling.world
metadata:
  bypass_filters: true  # Global override for entire package

imports:
  - namespace: realistic.nature

promptsections:
  - name: scene
    template: "{article} {animal} {activity}"
    references:
      animal: realistic.nature:animals
      
      # Filter specified but bypassed due to package metadata
      activity: realistic.nature:activities#{
        tags.requires_can_swim and ref:animal.tags.can_swim
      }

# Results: All combinations allowed despite filter
# (bypass_filters: true disables all filtering)
```

---

## Design Benefits

### 1. Reusability
✅ Import `realistic.nature` datatypes  
✅ Use with or without constraints  
✅ No need to copy/paste definitions

### 2. Flexibility
✅ Realistic packages can enforce plausibility  
✅ Absurdist packages can ignore constraints  
✅ Mixed packages can apply selectively

### 3. Composition
✅ Build on existing packages  
✅ Override just what you need  
✅ Extend with new capabilities

### 4. Backward Compatibility
✅ No filter = works as before  
✅ Adding filters is optional  
✅ Existing packages unaffected

---

## Validation Rules

**When filter is present:**
- Expression must be boolean
- Referenced tags must exist
- Referenced refs must be selected
- Empty result = error (no compatible items)

**When filter is absent:**
- All values available
- No validation errors
- Author's intentional choice

**Package-level bypass:**
- `bypass_filters: true` disables all filters
- Useful for absurdist/creative packages
- Can still specify filters (for documentation) but they're ignored

---

## Best Practices

### For Realistic Packages
```yaml
# Always filter for plausibility
activity: activities#{compatible_with(ref:subject)}
```

### For Absurdist Packages
```yaml
# Skip filters, embrace chaos
activity: activities  # No filter
```

### For Reusable Libraries
```yaml
# Provide well-tagged datatypes
# Let consumers decide whether to filter
datatypes:
  - name: animals
    values:
      - text: "swan"
        tags:
          can_swim: true
          can_fly: true
          can_run: false
          # Tags describe capabilities
          # Consumers choose whether to enforce
```

### For Package Authors
```yaml
# Document your intent
metadata:
  description: "Realistic nature scenes with plausibility checks"
  # OR
  description: "Absurdist scenarios where anything goes"
```

---

## Summary

**Q:** Can authors reuse entities but override constraints?  
**A:** ✅ Yes! Multiple ways:

1. **No filter** = No constraints (simplest)
2. **Extend with overrides** = Modify tags
3. **Package-level bypass** = Disable all filters
4. **Selective filtering** = Apply only where wanted

**The key:** Filters are **opt-in tools**, not mandatory restrictions. Authors choose when to enforce constraints, enabling both realistic and creative packages without redefinition.

---

**Bottom line:** You can absolutely create a "Crazy World" package that lets swans roll like bowling balls by simply importing realistic datatypes and not applying filters. No redefinition needed! 🎳🦢

