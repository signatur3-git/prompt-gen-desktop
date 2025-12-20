# Validation Guide: Package Validation Rules

**Version:** 0.1.0  
**Updated:** 2025-12-17

---

## Overview

The RPG validator checks packages for errors and best practices. This guide explains all validation rules, why they exist, and how to fix common issues.

---

## Validation Levels

### Errors (Must Fix)

Errors prevent the package from working correctly. You **must** fix all errors before the package can be used.

**Exit Code:** Validation fails with exit code 1

### Warnings (Should Fix)

Warnings indicate potential issues or best practice violations. The package will work, but you should review and fix warnings.

**Exit Code:** Validation passes with exit code 0 (but warnings are shown with `--warnings` flag)

---

## Schema Validation

### Package Structure

**Rule:** Package must have valid structure

**What it checks:**
- Package has `id` and `version` fields
- At least one namespace is defined
- Version follows semver format (X.Y.Z)

**Example Error:**
```
Validation error: Package must contain at least one namespace
```

**How to fix:**
```yaml
# Add required fields
id: my.package
version: 1.0.0

namespaces:
  my_namespace:
    id: my_namespace
    # ... namespace content
```

---

## Semantic Validation

### 1. Reference Not Found

**Rule:** All references must point to existing datatypes or promptsections

**Why:** Prevents runtime errors when rendering

**Example Error:**
```
Reference not found: 'missing_type' in test:prompt.color
  Suggestion: Did you mean 'test:colors' (datatype)?
```

**How to fix:**

Option 1 - Fix the typo:
```yaml
# Before (wrong)
references:
  color:
    target: missing_type

# After (correct)
references:
  color:
    target: colors
```

Option 2 - Create the missing datatype:
```yaml
datatypes:
  missing_type:
    name: missing_type
    values:
      - text: "red"
```

**Tip:** The validator suggests similar names when available!

---

### 2. Circular Reference

**Rule:** Nested promptsections cannot create circular dependencies

**Why:** Prevents infinite loops during rendering

**Example Error:**
```
Circular reference detected: test:section_a → test:section_b → test:section_c → test:section_a
```

**How to fix:**

Break the cycle by removing one reference:

```yaml
# Before (circular)
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
  template: "{section_a}"  # ← This creates the cycle
  references:
    section_a:
      target: section_a

# After (fixed)
section_c:
  template: "Done!"  # Break the cycle
```

**Tip:** The error shows the full chain, making it easy to identify where to break the cycle.

---

### 3. Invalid Tag Filter

**Rule:** Tag filter expressions must have valid syntax

**Why:** Invalid filters cause rendering to fail

**Example Error:**
```
Invalid tag filter: 'tags.can_fly &&' - Unexpected end of expression
```

**How to fix:**

Fix the syntax error:

```yaml
# Before (invalid)
references:
  creature:
    target: creatures
    filter: "tags.can_fly &&"  # ← Incomplete expression

# After (correct)
references:
  creature:
    target: creatures
    filter: "tags.can_fly && tags.nocturnal"  # Complete expression
```

**Valid operators:**
- `&&` - AND
- `||` - OR
- `!` - NOT
- `==` - Equals
- `!=` - Not equals

**Valid expressions:**
```yaml
filter: "tags.can_fly"                          # Simple
filter: "tags.can_fly && tags.nocturnal"        # AND
filter: "tags.type == 'bird' || tags.type == 'bat'"  # OR
filter: "!tags.dangerous"                       # NOT
filter: "tags.can_fly && !tags.domesticated"    # Complex
```

---

### 4. Separator Not Found

**Rule:** Referenced separator sets must exist

**Why:** Prevents runtime errors when formatting lists

**Example Error:**
```
Separator set not found: 'comma_and' referenced in test:prompt.colors
```

**How to fix:**

Option 1 - Create the separator set:
```yaml
separator_sets:
  comma_and:
    name: comma_and
    primary: ", "
    secondary: " and "
```

Option 2 - Fix the reference:
```yaml
# Use an existing separator or remove the reference
references:
  colors:
    separator: null  # No separator
```

---

### 5. Min > Max Invalid

**Rule:** In references, min must be <= max

**Why:** Logically impossible to satisfy

**Example Error:**
```
Min must be <= Max: min=5, max=2 in test:prompt.colors
```

**How to fix:**

Swap or correct the values:

```yaml
# Before (invalid)
references:
  colors:
    min: 5
    max: 2  # ← Less than min!

# After (correct)
references:
  colors:
    min: 2
    max: 5
```

---

### 6. Unique Constraint Infeasible

**Rule:** When `unique: true`, max cannot exceed available values

**Why:** Can't select 5 unique items from 2 values

**Example Error:**
```
Unique constraint infeasible: requested 5 unique values but only 2 values available in test:colors
```

**How to fix:**

Option 1 - Reduce max:
```yaml
# Before (infeasible)
datatypes:
  colors:
    values:
      - text: "red"
      - text: "blue"  # Only 2 values

references:
  colors:
    target: colors
    max: 5      # ← Can't get 5 unique from 2!
    unique: true

# After (fixed)
references:
  colors:
    target: colors
    max: 2      # Match available count
    unique: true
```

Option 2 - Add more values:
```yaml
datatypes:
  colors:
    values:
      - text: "red"
      - text: "blue"
      - text: "green"
      - text: "yellow"
      - text: "purple"  # Now have 5 values
```

Option 3 - Remove unique constraint:
```yaml
references:
  colors:
    max: 5
    unique: false  # Allow duplicates
```

---

## Best Practices Validation

### 1. Invalid Naming

**Rule:** Names should be lowercase alphanumeric with hyphens/underscores

**Why:** Consistency and avoid parsing issues

**Example Error:**
```
Invalid naming: 'MyDatatype' - Datatype names should be lowercase alphanumeric with hyphens or underscores
```

**How to fix:**

```yaml
# Before (invalid)
datatypes:
  MyDatatype:      # ← Uppercase
    name: MyDatatype

# After (correct)
datatypes:
  my_datatype:     # ← Lowercase with underscore
    name: my_datatype
```

**Valid names:**
- `my_datatype` ✓
- `my-datatype` ✓
- `datatype123` ✓
- `my_type_v2` ✓

**Invalid names:**
- `MyDatatype` ✗ (uppercase)
- `my datatype` ✗ (space)
- `my.datatype` ✗ (dot)
- `123datatype` ✗ (starts with number)

---

### 2. Unused Datatype (Warning)

**Rule:** Datatypes should be referenced by at least one promptsection

**Why:** Unused components clutter the package

**Example Warning:**
```
Unused datatype: 'old_colors' is defined but never referenced
```

**How to fix:**

Option 1 - Use it:
```yaml
prompt_sections:
  my_prompt:
    template: "{color}"
    references:
      color:
        target: old_colors  # Now it's used!
```

Option 2 - Remove it:
```yaml
# Delete the unused datatype
datatypes:
  # old_colors: removed
```

---

### 3. Unused Separator Set (Warning)

**Rule:** Separator sets should be referenced

**Why:** Unused components clutter the package

**Example Warning:**
```
Unused separator set: 'comma_or' is defined but never referenced
```

**How to fix:**

Same as unused datatypes - either use it or remove it.

---

### 4. Large Weight Sum (Warning)

**Rule:** Sum of weights should be reasonable (< 1000)

**Why:** Very large sums may cause precision issues

**Example Warning:**
```
Large weight sum in datatype 'test:colors': 5000.00 (consider normalizing)
```

**How to fix:**

Normalize the weights:

```yaml
# Before (large sum)
datatypes:
  colors:
    values:
      - text: "red"
        weight: 2000
      - text: "blue"
        weight: 3000  # Sum = 5000

# After (normalized)
datatypes:
  colors:
    values:
      - text: "red"
        weight: 2     # Same ratio
      - text: "blue"
        weight: 3     # Sum = 5
```

**Note:** The actual probabilities are the same (40% vs 60%), just scaled down.

---

## Quick Reference

### Error Categories

| Error Type | Severity | Can Render? |
|------------|----------|-------------|
| ReferenceNotFound | ERROR | No |
| CircularReference | ERROR | No |
| InvalidTagFilter | ERROR | No |
| SeparatorNotFound | ERROR | No |
| MinMaxInvalid | ERROR | No |
| UniqueConstraintInfeasible | ERROR | No |
| InvalidNaming | ERROR | No |
| UnusedDatatype | WARNING | Yes |
| UnusedSeparatorSet | WARNING | Yes |
| LargeWeightSum | WARNING | Yes |

---

## Validation Workflow

### 1. Run Validation

```bash
rpg-cli validate my-package.yaml --warnings
```

### 2. Fix Errors First

Focus on errors before warnings:
- ReferenceNotFound - Fix typos or create missing components
- CircularReference - Break the cycle
- MinMaxInvalid - Swap values
- InvalidTagFilter - Fix syntax

### 3. Review Warnings

Check warnings for code quality:
- Remove unused components
- Normalize large weights
- Fix naming conventions

### 4. Re-validate

```bash
rpg-cli validate my-package.yaml --warnings
```

### 5. Test Rendering

```bash
rpg-cli render my-package.yaml test:section --count 10
```

---

## Common Patterns

### Pattern: Typo in Reference

**Symptom:** "Reference not found" with suggestion

**Fix:** Follow the suggestion
```
Reference not found: 'colour' in test:prompt.main
  Suggestion: Did you mean 'test:colors' (datatype)?

✓ Fix: Change 'colour' to 'colors'
```

### Pattern: Forgot to Create Datatype

**Symptom:** "Reference not found" with no suggestion

**Fix:** Create the datatype
```yaml
datatypes:
  new_type:
    name: new_type
    values:
      - text: "value1"
```

### Pattern: Nested Templates Too Deep

**Symptom:** "Circular reference" or recursion error

**Fix:** Simplify nesting or break cycle
```
Max nesting depth: 10 levels
If you hit this, restructure your templates
```

### Pattern: Filter Syntax Error

**Symptom:** "Invalid tag filter"

**Fix:** Check operator spelling and completeness
```
tags.can_fly &&        ✗ Incomplete
tags.can_fly && true   ✓ Complete
```

---

## Advanced Validation

### Custom Validation Scripts

Validate multiple packages:

```bash
#!/bin/bash
# validate-all.sh

for pkg in packages/*.yaml; do
    echo "Validating: $pkg"
    rpg-cli validate "$pkg" --warnings || exit 1
done

echo "All packages valid!"
```

### Pre-commit Hook

```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running package validation..."
for pkg in $(git diff --cached --name-only | grep '\.yaml$'); do
    if ! rpg-cli validate "$pkg"; then
        echo "Validation failed for $pkg"
        exit 1
    fi
done
```

---

## See Also

- [CLI Guide](./cli-guide.md) - Using the validation command
- [Error Reference](./error-reference.md) - Complete error catalog
- [Package Format](../architecture/components.md) - Package structure reference

---

**Questions?** The validator provides helpful error messages. Read them carefully - they often include suggestions!

