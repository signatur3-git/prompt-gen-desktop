# Template Syntax Reference

**Version:** 1.0.0  
**Last Updated:** 2025-12-17

This document describes the complete template syntax for RPG v1.0.0, including examples and best practices.

**For learning:** See [Tutorial 1: Basic Package](../guides/tutorial-series/01-basic-package.md)  
**For tag filtering:** See [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md)  
**For lists:** See [Tutorial 4: Lists and Separators](../guides/tutorial-series/04-lists-separators.md)

---

## Overview

Templates are strings with **placeholders** that get replaced during rendering. A template like:

```yaml
template: "A {adjective} {creature}"
```

Becomes output like: `"A brave knight"` or `"A clever wizard"`

---

## Basic Placeholder Syntax

### Simple Placeholder

```yaml
template: "{reference_name}"
```

The `reference_name` must match a key in the `references` section:

```yaml
template: "{creature}"
references:
  creature:
    target: fantasy:creatures
    # ... other properties
```

**Rules:**
- Placeholder names are case-sensitive
- Must be valid identifiers (letters, numbers, underscores)
- Cannot start with a number

---

## Reference Definition

Every placeholder needs a corresponding reference definition:

```yaml
references:
  reference_name:
    target: namespace:datatype    # or context:keyname
    min: 1                          # minimum values to select
    max: 1                          # maximum values to select
    separator: null                 # separatorset name for lists
    unique: false                   # prevent duplicates
    filter: null                    # tag filter expression
```

### Target

**Format:** `namespace:component` or `context:keyname`

**Examples:**
```yaml
# Datatype reference
target: fantasy:creatures

# Another promptsection (nesting)
target: fantasy:greeting

# Context value
target: context:article
```

### Min / Max

**Select multiple values from a datatype:**

```yaml
# Exactly 1 value
min: 1
max: 1

# 2-4 values (random count)
min: 2
max: 4

# 0-3 values (optional)
min: 0
max: 3

# Exactly 3 values
min: 3
max: 3
```

**When `min > 1` or `max > 1`:** Multiple values selected and joined with separator

**Learn more:** [Tutorial 4](../guides/tutorial-series/04-lists-separators.md)

### Separator

**Name of a SeparatorSet** to use for formatting lists:

```yaml
separator: comma_and
# Result: "red, blue and green"

separator: comma_or
# Result: "red, blue or green"

separator: slash
# Result: "red/blue/green"
```

**When null:** Multiple values joined with space (no formatting)

### Unique

**Prevent selecting the same value twice:**

```yaml
unique: true
# Result: "red, blue, green" (no duplicates)

unique: false
# Result: "red, red, blue" (duplicates allowed)
```

**Use `unique: true` when:**
- Listing items in inventory
- Selecting palette colors
- Physical items that can't duplicate

**Use `unique: false` when:**
- Rolling dice (can roll same number)
- Random words (can repeat)
- Probability simulations

### Filter

**Tag filtering expression** to constrain selection:

```yaml
# Only flying creatures
filter: "tags.can_fly"

# Flying AND magical
filter: "tags.can_fly && tags.is_magical"

# Can fly OR swim
filter: "tags.can_fly || tags.can_swim"

# NOT dangerous
filter: "!tags.is_dangerous"
```

**Operators:**
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT
- `==` - Equality (for strings/numbers)
- `!=` - Inequality
- `>`, `<`, `>=`, `<=` - Numeric comparison

**Learn more:** [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md)

---

## Inline Syntax (Advanced)

### Inline Tag Filtering

Instead of using the `filter` property, you can filter inline in the template:

```yaml
# Inline filter
template: "{creature#{tags.can_fly}}"

# Equivalent to:
template: "{creature}"
references:
  creature:
    target: fantasy:creatures
    filter: "tags.can_fly"
```

**Syntax:** `{reference#{expression}}`

**Examples:**
```yaml
# Flying creatures
"{creatures#{tags.can_fly}}"

# Large and dangerous
"{creatures#{tags.size == 'large' && tags.is_dangerous}}"

# Can fly or swim
"{creatures#{tags.can_fly || tags.can_swim}}"
```

### Inline Parameters

You can also specify parameters inline:

```yaml
# With parameters
template: "{colors?min=2,max=4,sep=comma_and,unique=true}"

# Equivalent to:
references:
  colors:
    target: fantasy:colors
    min: 2
    max: 4
    separator: comma_and
    unique: true
```

**Parameter syntax:** `{reference?param=value,param=value}`

**Available parameters:**
- `min=N` - Minimum count
- `max=N` - Maximum count
- `sep=separatorset_name` - Separator set
- `unique=true` or `unique=false` - Uniqueness

**When to use inline:**
- Quick prototyping
- One-off special cases
- Simple filters

**When to use properties:**
- Production packages
- Reusable patterns
- Complex configurations

---

## Complete Examples

### Example 1: Simple Reference

```yaml
prompt_sections:
  greeting:
    template: "Hello, {name}!"
    references:
      name:
        target: game:names
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Output:** "Hello, Alice!" or "Hello, Bob!"

---

### Example 2: Multiple Values with Separator

```yaml
prompt_sections:
  color_list:
    template: "Available colors: {colors}"
    references:
      colors:
        target: game:colors
        min: 2
        max: 4
        separator: comma_and
        unique: true
        filter: null
```

**Outputs:**
- "Available colors: red and blue" (2 items)
- "Available colors: red, blue and green" (3 items)
- "Available colors: red, blue, green and yellow" (4 items)

---

### Example 3: Tag Filtering

```yaml
prompt_sections:
  flying_scene:
    template: "{creature} soaring through the sky"
    references:
      creature:
        target: game:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: "tags.can_fly"
```

**Outputs:**
- "swan soaring through the sky" ✅
- "eagle soaring through the sky" ✅
- Never "deer soaring..." (can't fly) ❌

---

### Example 4: Context Reference

```yaml
rules:
  - set: article
    from: ref:creature.tags.article

prompt_sections:
  description:
    template: "{article} {creature}"
    references:
      article:
        target: context:article
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      creature:
        target: game:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Outputs:**
- "a swan" ✅ (correct article)
- "an eagle" ✅ (correct article)

**Learn more:** [Tutorial 3: Context Rules](../guides/tutorial-series/03-context-rules.md)

---

### Example 5: Nested Template

```yaml
prompt_sections:
  greeting:
    template: "Greetings, {name}!"
    references:
      name:
        target: game:names
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
  
  full_scene:
    template: "{greeting} {action}"
    references:
      greeting:
        target: game:greeting     # References another promptsection
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      action:
        target: game:actions
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Output:** "Greetings, Alice! Take care."

---

### Example 6: Complex Inline

```yaml
template: "{adjectives?min=1,max=3,sep=comma_and} {creature#{tags.can_fly && tags.is_magical}}"
```

**Translation:**
- Select 1-3 adjectives
- Format with comma_and separator
- Select creature that can fly AND is magical

**Output:** "ancient, powerful dragon" or "mystical fairy"

---

## Tag Filter Expression Syntax

### Boolean Tags

```yaml
tags:
  can_fly: true
  is_magical: false

# Usage:
filter: "tags.can_fly"
filter: "!tags.is_magical"
```

### String Tags

```yaml
tags:
  size: large
  color: red

# Usage (requires quotes):
filter: "tags.size == 'large'"
filter: "tags.color != 'blue'"
```

### Numeric Tags

```yaml
tags:
  level: 5
  hitpoints: 100

# Usage:
filter: "tags.level > 3"
filter: "tags.hitpoints <= 50"
```

### Complex Expressions

```yaml
# AND
filter: "tags.can_fly && tags.is_magical"

# OR
filter: "tags.can_fly || tags.can_swim"

# NOT
filter: "!tags.is_dangerous"

# Combination
filter: "(tags.can_fly || tags.can_swim) && !tags.is_dangerous"

# With comparisons
filter: "tags.level > 5 && tags.size == 'large'"
```

---

## Best Practices

### 1. Use Descriptive Reference Names

✅ **Good:**
```yaml
template: "{article} {adjective} {creature} {action}"
```

❌ **Bad:**
```yaml
template: "{a} {b} {c} {d}"
```

### 2. Keep Templates Simple

✅ **Good:**
```yaml
template: "{greeting} {main_description}"
references:
  greeting: {target: game:greetings, ...}
  main_description: {target: game:descriptions, ...}
```

❌ **Too complex:**
```yaml
template: "{a?min=1,max=3,sep=s1} {b#{tags.x && tags.y}} {c?unique=true} {d#{tags.z || !tags.w}}"
```

**Better:** Break into smaller promptsections

### 3. Use Inline Filters Sparingly

✅ **Good for quick prototypes:**
```yaml
template: "{creature#{tags.can_fly}}"
```

✅ **Better for production:**
```yaml
template: "{creature}"
references:
  creature:
    target: game:creatures
    filter: "tags.can_fly"
```

### 4. Document Expected Outputs

```yaml
# Example outputs:
# - "a brave knight"
# - "an ancient dragon"
# - "a clever wizard"
template: "{article} {adjective} {class}"
```

### 5. Avoid Deep Nesting

✅ **Good:** 2-3 levels deep
```yaml
scene → greeting → name
```

❌ **Bad:** 10+ levels deep (performance impact)

**Recursion limit:** 10 levels (enforced by engine)

---

## Common Mistakes

### Mistake 1: Reference Name Mismatch

❌ **Wrong:**
```yaml
template: "A {adj} {noun}"
references:
  adjective: {target: game:adjectives, ...}  # Doesn't match "adj"!
```

✅ **Correct:**
```yaml
template: "A {adjective} {noun}"
references:
  adjective: {target: game:adjectives, ...}  # Matches!
```

### Mistake 2: Missing Quotes in String Comparison

❌ **Wrong:**
```yaml
filter: "tags.size == large"  # large not quoted!
```

✅ **Correct:**
```yaml
filter: "tags.size == 'large'"  # Quoted!
```

### Mistake 3: Using Both Filter Property and Inline

❌ **Wrong:**
```yaml
template: "{creature#{tags.can_fly}}"
references:
  creature:
    filter: "tags.is_magical"  # Both specified - confusing!
```

✅ **Correct (choose one):**
```yaml
# Option 1: Inline only
template: "{creature#{tags.can_fly && tags.is_magical}}"
references:
  creature:
    filter: null

# Option 2: Property only
template: "{creature}"
references:
  creature:
    filter: "tags.can_fly && tags.is_magical"
```

---

## Validation

**The validator checks:**
- ✅ All template placeholders have matching references
- ✅ All reference targets exist
- ✅ Tag filter expressions are syntactically valid
- ✅ Min ≤ max for all references
- ✅ Separator sets exist when referenced
- ⚠️ Unused references (warning, not error)

---

## Cross-References

**For learning:**
- [Tutorial 1: Basic Package](../guides/tutorial-series/01-basic-package.md) - Create your first template
- [Tutorial 2: Tag Filtering](../guides/tutorial-series/02-tag-filtering.md) - Filter by properties
- [Tutorial 3: Context Rules](../guides/tutorial-series/03-context-rules.md) - Use context references
- [Tutorial 4: Lists and Separators](../guides/tutorial-series/04-lists-separators.md) - Multi-value templates

**For reference:**
- [Components](./components.md) - Data model details
- [Context Interactions](./context-interactions.md) - Rules and context
- [Tag Filtering](./tag-filtering.md) - Filter expression details

---

**This syntax reference reflects the v1.0.0 implementation from M2-M5.**

