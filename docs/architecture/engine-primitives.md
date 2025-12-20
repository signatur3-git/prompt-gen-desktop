---
title: Engine Primitives Reference
---

# Engine Primitives Reference

**Version:** 1.0.0  
**Last Updated:** 2025-12-17

This document defines the primitives that RPG rendering engines provide. These primitives enable authors to create sophisticated prompts without domain-specific features.

**For learning:** See [Tutorial Series](../guides/tutorial-series/)  
**For overview:** See [Architecture Overview](./overview.md)

---

## Design Philosophy

**Provide mechanisms, not semantics:**
- Engine primitives are **generic** and **language-agnostic**
- Domain-specific logic belongs in package rules and datatypes
- Keep the primitive set **minimal but sufficient**

**Evidence:** M1 analysis of 6 real-world prompts validated this primitive set handles coordination scenarios when combined with Rules and Tag Filtering.

---

## Core Primitives

### Reference Syntax

**Access selected values and their properties:**

```yaml
ref:name                  # Access selected value
ref:name.text            # Value's text property
ref:name.tags.tagname    # Access specific tag value
ref:namespace:name       # Fully qualified reference
```

**Behavior:**
- Returns the selected value from the datatype or promptsection
- If value not selected or null, returns null
- Tag access on null value returns null (safe navigation)

**Example:**
```yaml
template: "{article} {color} {object}"
rules:
  - set: article
    from: ref:color.tags.article  # Extract "a" or "an" from color tags
```

---

### Context Operations

**Scoped key-value storage for coordination:**

#### context.get(key, scope?)

```yaml
context.get('article')              # Get from .prompt scope
context.get('theme', '.global')     # Get from .global scope
context.get('count', '.section.hero')  # Get from custom scope
```

**Behavior:**
- Retrieves value from specified scope
- Falls back hierarchically: `.section.X` → `.prompt` → `.global`
- Returns null if key not found in any scope
- **Deterministic:** Always returns same value for same key/scope

#### context.set(key, value, scope?)

```yaml
context.set('article', 'a')                    # Set in .prompt scope
context.set('theme', 'dark', '.global')        # Set in .global scope
context.set('hp', 100, '.section.hero')        # Set in custom scope
```

**Behavior:**
- Stores value in specified scope
- Overwrites existing value if present
- **Deterministic:** No side effects beyond storage

#### context.has(key, scope?)

```yaml
context.has('article')              # Check .prompt scope
context.has('theme', '.global')     # Check .global scope
```

**Behavior:**
- Returns true if key exists in scope (or fallback)
- Returns false otherwise
- **Deterministic:** Consistent with get() behavior

**Scopes:**
- `.global` - Shared across entire batch
- `.prompt` - Per-prompt (default)
- `.section.<id>` - Per promptsection instance
- Custom scopes allowed

---

### Random Operations

**Deterministic randomness with seeding:**

#### random.int(min, max)

```yaml
random.int(1, 6)        # Random integer 1-6 (dice roll)
random.int(0, 100)      # Random integer 0-100 (percentage)
```

**Behavior:**
- Returns random integer in range [min, max] (inclusive)
- **Deterministic:** Same seed produces same sequence
- Uses seeded RNG initialized per prompt/batch
- **MUST NOT use system random** (breaks determinism)

**Implementation Note:**
Engines MUST use a seeded PRNG (e.g., Xorshift, PCG) initialized with the prompt's seed value.

---

## Tag Filtering (Critical Addition - v1.0.0-rc1)

**Selection-time filtering for physical plausibility and compatibility:**

### Basic Syntax

```yaml
datatype#tag:value              # Static tag filter
datatype#{expression}           # Dynamic tag filter with expressions
```

### Filter Expressions

**Available in filter context:**
- `tags.*` - Current value's tags being evaluated
- `ref:*` - Previously selected references  
- Boolean operators: `and`, `or`, `not`, `implies`
- Comparison operators: `==`, `!=`, `<`, `>`, `<=`, `>=`
- List operators: `in`, `contains`

### Examples

**Physical plausibility:**
```yaml
activity: activities#{tags.requires_can_swim and ref:subject.tags.can_swim}
# Only selects activities the subject can perform
```

**Temporal coherence:**
```yaml
atmosphere: atmospheres#{ref:time.text in tags.time_compat}
# Only selects atmospheres compatible with time of day
```

**Item compatibility:**
```yaml
possession: possessions#{ref:character.text in tags.owners}
# Only selects possessions appropriate for character type
```

**Behavior:**
- Filter applied **during selection** (not after)
- Only values matching filter are eligible for selection
- If filtered list is empty, rendering fails with error
- **No filter = no filtering** (all values eligible)

**This is ESSENTIAL** - cannot be achieved with post-selection validation or Decisions alone.

---

## Helper Functions

**Convenience functions for common patterns:**

### first_selected([ref1, ref2, ...])

```yaml
first_selected([ref:adjective, ref:noun])
# Returns first non-null reference
```

**Behavior:**
- Evaluates references in order
- Returns first reference that is not null
- Returns null if all references are null
- **Critical for optional element coordination** (M1 Pattern 2)

**Example:**
```yaml
template: "{article} {adjective} {noun}"
references:
  adjective: adjectives?min=0,max=1  # Optional
  noun: nouns
rules:
  - set: article
    from: first_selected([ref:adjective, ref:noun]).tags.article
# Uses adjective's article if present, otherwise noun's
```

---

## Conditional Expressions

**Simple branching logic:**

### if/then/else

```yaml
if condition then value1 else value2
```

**Example:**
```yaml
if context.get('count') == 1 
  then ref:noun.text 
  else ref:noun.tags.plural
```

**Behavior:**
- Evaluates condition (must be boolean)
- Returns value1 if true, value2 if false
- Both branches are expressions (can be nested)
- **Deterministic:** No side effects

---

## Type Conversions

**Basic type operations:**

### string(value)

```yaml
string(42)           # "42"
string(3.14)         # "3.14"
string(true)         # "true"
```

**Behavior:**
- Converts value to string representation
- **Deterministic:** Same input always produces same output

### int(value)

```yaml
int("42")            # 42
int(3.14)            # 3 (truncates)
```

**Behavior:**
- Converts to integer
- Truncates decimals (does not round)
- Returns null if conversion fails

---

## List Operations

**For working with tag lists:**

### in operator

```yaml
"dawn" in tags.time_compat          # Check if value in list
ref:time.text in tags.time_compat   # Dynamic check
```

**Behavior:**
- Returns true if value is in list
- Returns false otherwise
- Case-sensitive string comparison

### contains operator

```yaml
tags.time_compat contains "dawn"    # Check if list contains value
```

**Behavior:**
- Same as `in` but reversed operands
- Syntactic sugar for readability

---

## Template Reference Reuse

**Use same value multiple times:**

```yaml
template: "A {color} {vehicle1} and a {color} {vehicle2}"
```

**Behavior:**
- Variable selected once in Selection phase
- Can be referenced multiple times in template
- All references use the same selected value
- **Automatic** - no special syntax needed

**M1 Discovery:** This is a natural template feature, not a special primitive!

---

## What is NOT a Primitive

**Domain-specific helpers belong in Decisions:**

❌ `pluralize(singular, plural, count)` - Use Decision  
❌ `conjugate(verb, subject)` - Use Decision  
❌ `articlize(word)` - Use tags + Rules  
❌ `capitalize(text)` - Use Morpher  
❌ `context.request()` - Pattern, not primitive  
❌ `context.contribute()` - Pattern, not primitive  

**Rationale:**
- Domain-specific = leads to "helper explosion"
- Different languages need different helpers
- Keeps engine generic and maintainable
- Authors can define these as Decisions in featured.common

---

## Primitive Usage Statistics (M1 Evidence)

From analysis of 6 real-world prompts:

| Primitive | Usage % | Critical? |
|-----------|---------|-----------|
| `ref:name.tags.tag` | 100% | ✅ Yes |
| Tag filtering `#{}` | 17% | ✅ Yes (essential for plausibility) |
| `first_selected()` | 17% | ✅ Yes (pattern 2) |
| `context.get/set` | 17% | ✅ Yes (pattern 3) |
| `random.int()` | 17% | ✅ Yes (pattern 3) |
| Conditionals | 17% | ⚠️ Sometimes |
| Template reuse | 17% | ✅ Automatic |

**Conclusion:** All primitives validated against real scenarios. Minimal set covers 100% of coordination needs when combined with Rules, Tag Filtering, and Decisions.

---

## Implementation Requirements

### MUST Provide

Conforming engines MUST implement:
- ✅ Reference syntax (`ref:name`, `ref:name.tags.tag`)
- ✅ Context operations (`get`, `set`, `has`)
- ✅ Tag filtering syntax (`datatype#{expression}`)
- ✅ `first_selected()` helper
- ✅ `random.int()` with seeded PRNG
- ✅ Conditional expressions (`if/then/else`)
- ✅ Type conversions (`string()`, `int()`)
- ✅ List operators (`in`, `contains`)
- ✅ Template reference reuse

### MUST NOT Provide

Engines MUST NOT implement:
- ❌ Domain-specific helpers (pluralize, etc.)
- ❌ Non-deterministic operations (system random, time)
- ❌ External I/O during rendering
- ❌ Language-specific linguistic operations

### Determinism Requirements

All primitives MUST be:
- **Deterministic:** Same input always produces same output
- **Pure:** No side effects except `context.set()`
- **Seeded:** Random operations use prompt seed
- **Reproducible:** Same seed produces identical results across runs

---

## Validation

Validators MUST enforce:
- Referenced tags exist in datatype
- Referenced refs are previously selected (no forward references in filters)
- Filter expressions evaluate to boolean
- Scope names are valid
- Type conversions are safe

Validators SHOULD warn:
- Unused context keys
- Unreachable branches
- Complex nested conditionals (suggest Decision)

---

## Examples from M1 Analysis

### S1: Direct Tag Extraction (Pattern 1)

```yaml
rules:
  - set: article
    from: ref:color.tags.article
```

Uses: `ref:name.tags.tag`

### S2: Optional Element (Pattern 2)

```yaml
rules:
  - set: article
    from: first_selected([ref:adjective, ref:noun]).tags.article
```

Uses: `first_selected()`, `ref:name.tags.tag`

### S3: Count-Based Pluralization (Pattern 3)

```yaml
rules:
  - set: count
    from: random.int(1, 5)
  - set: noun_form
    from: if context.get('count') == 1 then ref:noun.text else ref:noun.tags.plural
```

Uses: `random.int()`, `context.get/set()`, `if/then/else`, `ref:name.tags.tag`

### M1: Selection-Time Filtering (Pattern 4)

```yaml
activity: activities#{tags.requires_can_swim and ref:subject.tags.can_swim}
```

Uses: Tag filtering `#{}`, `ref:name.tags.tag`, boolean operators

### M3: Context Reuse (Pattern 6)

```yaml
template: "A {color} {vehicle1} and a {color} {vehicle2}"
```

Uses: Template reference reuse (automatic)

---

## See Also

- [Template Syntax](./template-syntax.md) - Complete grammar
- [Context Interactions](./context-interactions.md) - Context system details
- [Tag Filtering Overrides](../examples/tag-filtering-overrides.md) - Creative package examples
- [Authoring Analysis](../examples/authoring-analysis.md) - Real-world usage patterns

---

**Last Updated:** 2025-12-16 (M1 Completion)  
**Validated Against:** 6 real-world text-to-image prompt scenarios  
**Coverage:** 100% of coordination patterns with Rules + Tag Filtering + Decisions

