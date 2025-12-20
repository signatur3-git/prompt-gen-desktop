# Error Reference: Complete Validation Error Catalog

**Version:** 0.1.0  
**Updated:** 2025-12-17

---

## Overview

This document catalogs all validation errors and warnings with examples, causes, and solutions.

---

## Errors

### E001: ReferenceNotFound

**Message:**
```
Reference not found: '{reference}' in {location}
```

**Cause:** A promptsection references a datatype or promptsection that doesn't exist.

**Example:**
```yaml
prompt_sections:
  my_prompt:
    template: "{color}"
    references:
      color:
        target: missing_datatype  # ← Does not exist
```

**Error:**
```
Reference not found: 'missing_datatype' in test:my_prompt.color
```

**Solution 1 - Create the datatype:**
```yaml
datatypes:
  missing_datatype:
    name: missing_datatype
    values:
      - text: "red"
```

**Solution 2 - Fix the reference:**
```yaml
references:
  color:
    target: colors  # Use existing datatype
```

**With Suggestion:**
```
Reference not found: 'colour' in test:my_prompt.color
  Suggestion: Did you mean 'test:colors' (datatype)?
```

---

### E002: CircularReference

**Message:**
```
Circular reference detected: {chain}
```

**Cause:** Nested promptsections create a loop (A references B, B references C, C references A).

**Example:**
```yaml
prompt_sections:
  section_a:
    template: "{section_b}"
    references:
      section_b:
        target: section_b
  
  section_b:
    template: "{section_c}"
    references:
      section_c:
        target: section_c
  
  section_c:
    template: "{section_a}"  # ← Completes the cycle
    references:
      section_a:
        target: section_a
```

**Error:**
```
Circular reference detected: test:section_a → test:section_b → test:section_c → test:section_a
```

**Solution - Break the cycle:**
```yaml
section_c:
  template: "Final text"  # Don't reference section_a
```

**Tips:**
- Maximum nesting depth is 10 levels
- Cycles are detected at any depth
- The error shows the full chain to help identify where to break it

---

### E003: InvalidTagFilter

**Message:**
```
Invalid tag filter: {expression} - {reason}
```

**Cause:** Tag filter expression has syntax errors.

**Example:**
```yaml
references:
  creature:
    target: creatures
    filter: "tags.can_fly &&"  # ← Incomplete
```

**Error:**
```
Invalid tag filter: 'tags.can_fly &&' - Unexpected end of expression
```

**Common Syntax Errors:**

**Missing operand:**
```
tags.can_fly &&          ✗ No right operand
tags.can_fly && true     ✓ Complete
```

**Invalid operator:**
```
tags.can_fly & tags.nocturnal   ✗ Use &&
tags.can_fly && tags.nocturnal  ✓ Correct
```

**Unbalanced parentheses:**
```
(tags.can_fly && tags.nocturnal  ✗ Missing )
(tags.can_fly && tags.nocturnal) ✓ Balanced
```

**Invalid tag name:**
```
tags.can fly             ✗ Space in name
tags.can_fly             ✓ Use underscore
```

**Solution - Fix the syntax:**
```yaml
filter: "tags.can_fly && tags.nocturnal"  # Complete expression
```

**Valid Operators:**
- `&&` - Logical AND
- `||` - Logical OR
- `!` - Logical NOT
- `==` - Equals
- `!=` - Not equals
- `()` - Grouping

---

### E004: SeparatorNotFound

**Message:**
```
Separator set not found: '{separator}' referenced in {location}
```

**Cause:** Reference uses a separator set that doesn't exist.

**Example:**
```yaml
references:
  colors:
    target: colors
    separator: comma_and  # ← Does not exist

separator_sets: {}  # Empty!
```

**Error:**
```
Separator set not found: 'comma_and' referenced in test:prompt.colors
```

**Solution - Create the separator set:**
```yaml
separator_sets:
  comma_and:
    name: comma_and
    primary: ", "
    secondary: " and "
```

---

### E005: MinMaxInvalid

**Message:**
```
Min must be <= Max: min={min}, max={max} in {location}
```

**Cause:** Reference has min > max, which is logically impossible.

**Example:**
```yaml
references:
  colors:
    min: 5
    max: 2  # ← Less than min!
```

**Error:**
```
Min must be <= Max: min=5, max=2 in test:prompt.colors
```

**Solution - Swap or fix values:**
```yaml
references:
  colors:
    min: 2
    max: 5
```

**Edge Cases:**
- `min: 0, max: 0` ✓ Valid (optional reference)
- `min: 1, max: 1` ✓ Valid (exactly one)
- `min: 3, max: 3` ✓ Valid (exactly three)

---

### E006: UniqueConstraintInfeasible

**Message:**
```
Unique constraint infeasible: requested {requested} unique values but only {available} values available in {datatype}
```

**Cause:** Reference requests more unique values than the datatype contains.

**Example:**
```yaml
datatypes:
  colors:
    values:
      - text: "red"
      - text: "blue"  # Only 2 values

references:
  colors:
    target: colors
    max: 5       # ← Want 5 unique...
    unique: true # ← ...but only 2 available!
```

**Error:**
```
Unique constraint infeasible: requested 5 unique values but only 2 values available in test:colors
```

**Solution 1 - Reduce max:**
```yaml
references:
  colors:
    max: 2       # Match available count
    unique: true
```

**Solution 2 - Add more values:**
```yaml
datatypes:
  colors:
    values:
      - text: "red"
      - text: "blue"
      - text: "green"
      - text: "yellow"
      - text: "purple"  # Now 5 values
```

**Solution 3 - Allow duplicates:**
```yaml
references:
  colors:
    max: 5
    unique: false  # Duplicates OK
```

---

### E007: InvalidNaming

**Message:**
```
Invalid naming: '{name}' - {reason}
```

**Cause:** Component name doesn't follow naming conventions.

**Example:**
```yaml
datatypes:
  My-Datatype:  # ← Uppercase
    name: My-Datatype
```

**Error:**
```
Invalid naming: 'My-Datatype' - Datatype names should be lowercase alphanumeric with hyphens or underscores
```

**Naming Rules:**
- Must start with lowercase letter
- Can contain: `a-z`, `0-9`, `-`, `_`
- Cannot contain: uppercase, spaces, dots, special chars

**Valid:**
- `colors` ✓
- `my_colors` ✓
- `my-colors` ✓
- `colors_v2` ✓

**Invalid:**
- `Colors` ✗ Uppercase
- `my colors` ✗ Space
- `my.colors` ✗ Dot
- `2colors` ✗ Starts with number

**Solution - Use lowercase:**
```yaml
datatypes:
  my_datatype:  # ← Lowercase with underscore
    name: my_datatype
```

---

## Warnings

### W001: UnusedDatatype

**Message:**
```
Unused datatype: '{namespace}:{datatype}' is defined but never referenced
```

**Cause:** Datatype exists but no promptsection references it.

**Example:**
```yaml
datatypes:
  old_colors:  # ← Defined
    name: old_colors
    values:
      - text: "red"

prompt_sections:
  my_prompt:
    template: "{color}"
    references:
      color:
        target: colors  # ← References 'colors', not 'old_colors'
```

**Warning:**
```
Unused datatype: 'test:old_colors' is defined but never referenced
```

**Impact:** None - package still works, just has extra data

**Solution 1 - Use it:**
```yaml
references:
  old_color:
    target: old_colors
```

**Solution 2 - Remove it:**
```yaml
# Delete the old_colors datatype
```

---

### W002: UnusedPromptSection

**Message:**
```
Unused promptsection: '{namespace}:{promptsection}' is defined but never referenced
```

**Cause:** PromptSection exists but is never referenced by other promptsections.

**Note:** This is often intentional - the promptsection is an entry point for rendering, not referenced by others.

**Example:**
```yaml
prompt_sections:
  helper:  # ← Not referenced
    template: "{adjective} {noun}"
  
  main:
    template: "A {color} ball"
```

**Warning:**
```
Unused promptsection: 'test:helper' is defined but never referenced
```

**When to Ignore:**
- Entry point promptsections (meant to be called directly)
- Testing/debug promptsections

**When to Fix:**
- Actually unused helper templates

---

### W003: UnusedSeparatorSet

**Message:**
```
Unused separator set: '{namespace}:{separator}' is defined but never referenced
```

**Cause:** Separator set exists but no reference uses it.

**Example:**
```yaml
separator_sets:
  comma_or:  # ← Defined but not used
    primary: ", "
    secondary: " or "

references:
  colors:
    separator: comma_and  # Uses different separator
```

**Warning:**
```
Unused separator set: 'test:comma_or' is defined but never referenced
```

**Solution - Use it or remove it**

---

### W004: LargeWeightSum

**Message:**
```
Large weight sum in datatype '{datatype}': {sum} (consider normalizing)
```

**Cause:** Sum of weights in a datatype is very large (>1000).

**Example:**
```yaml
datatypes:
  colors:
    values:
      - text: "red"
        weight: 2000
      - text: "blue"
        weight: 3000  # Sum = 5000
```

**Warning:**
```
Large weight sum in datatype 'test:colors': 5000.00 (consider normalizing)
```

**Impact:** Potential floating-point precision issues

**Solution - Normalize:**
```yaml
datatypes:
  colors:
    values:
      - text: "red"
        weight: 2   # Same 40% probability
      - text: "blue"
        weight: 3   # Same 60% probability
```

**Note:** The ratios remain the same (2:3 vs 2000:3000)

---

## Error Code Summary

| Code | Name | Severity | Blocks Rendering |
|------|------|----------|------------------|
| E001 | ReferenceNotFound | Error | Yes |
| E002 | CircularReference | Error | Yes |
| E003 | InvalidTagFilter | Error | Yes |
| E004 | SeparatorNotFound | Error | Yes |
| E005 | MinMaxInvalid | Error | Yes |
| E006 | UniqueConstraintInfeasible | Error | Yes |
| E007 | InvalidNaming | Error | Yes |
| W001 | UnusedDatatype | Warning | No |
| W002 | UnusedPromptSection | Warning | No |
| W003 | UnusedSeparatorSet | Warning | No |
| W004 | LargeWeightSum | Warning | No |

---

## Quick Troubleshooting

### "My package won't validate"

1. Run with `--verbose` to see details
2. Fix errors from top to bottom
3. Each error includes a fix suggestion
4. Re-run after each fix

### "I fixed the error but still get warning"

Warnings don't block rendering. You can:
- Fix them for cleaner code
- Ignore them if intentional
- Use `--warnings` flag to see them

### "Error message is unclear"

The validator tries to provide:
- Exact location (namespace:section.reference)
- Suggestions for typos
- Clear fix instructions

If still unclear, check this reference guide!

---

## See Also

- [Validation Guide](./validation-guide.md) - Detailed validation rules
- [CLI Guide](./cli-guide.md) - Using the validation command
- [Package Format](../architecture/components.md) - Package structure

---

**Pro Tip:** Most errors include suggestions or fix hints. Read the error message carefully!

