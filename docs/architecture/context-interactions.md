# Context Interactions

**Version:** 1.0.0  
**Last Updated:** 2025-12-17

This document describes how the context system enables coordination between template elements in RPG v1.0.0.

**For learning:** See [Tutorial 3: Context Rules](../guides/tutorial-series/03-context-rules.md)  
**For overview:** See [Architecture Overview](./overview.md#context-system-m4-)

---

## What is Context?

The **context** is a simple key-value store that enables template elements to coordinate with each other. It solves coordination problems like:

- **Article selection:** "a" vs "an" based on the following word
- **Property sharing:** All elements share the same color or habitat
- **Conditional logic:** Behavior changes based on selected values

---

## How Context Works

### The Three-Phase Pipeline

Context is populated and used across the three rendering phases:

```
Phase 1: Selection
├─> Template: "{article} {creature}"
├─> Select creature: "swan"
└─> (no context changes yet)

Phase 2: Enrichment (Rules Execute)
├─> Rule: set article from creature.tags.article
├─> Context now has: article = "a"
└─> (context populated)

Phase 3: Rendering
├─> Replace {article} with context.article → "a"
├─> Replace {creature} with "swan"
└─> Output: "a swan" ✅
```

**Key insight:** Rules run AFTER selection but BEFORE rendering.

**Learn more:** [Tutorial 3](../guides/tutorial-series/03-context-rules.md)

---

## Rules System

**Rules** are the primary way to populate context in v1.0.0.

### Basic Rule Structure

```yaml
rules:
  - set: context_key
    from: expression
```

**Fields:**
- `set` - Context key to write (string)
- `from` - Expression to evaluate (string)

### Rule Execution

**Order:** Rules execute in the order they appear in the `rules` array

**Timing:** During Enrichment phase (after Selection, before Rendering)

**Scope:** Rules can read from selected references and write to context

---

## Expression Syntax

### Reading from References

**Access selected value:**
```yaml
from: ref:creature
# Returns the full selected value object
```

**Access tag from selected value:**
```yaml
from: ref:creature.tags.article
# Returns "a" or "an" depending on creature
```

**Nested tag access:**
```yaml
from: ref:creature.tags.stats.level
# Access nested tag structure
```

### The `first_selected()` Function

**Purpose:** Get the first non-null value from a list of references

**Syntax:**
```yaml
from: first_selected([ref:adjective, ref:noun]).tags.article
```

**How it works:**
1. Check if `adjective` was selected (has value)
2. If yes, return its `article` tag
3. If no (null), check `noun`
4. Return the first non-null value's tag

**Use case:** Article coordination with optional adjectives

**Example:**
```yaml
# With adjective selected
adjective = "ancient" (tags.article = "an")
noun = "knight" (tags.article = "a")
first_selected([ref:adjective, ref:noun]).tags.article
→ "an" (from adjective)

# Without adjective (null)
adjective = null
noun = "knight" (tags.article = "a")  
first_selected([ref:adjective, ref:noun]).tags.article
→ "a" (from noun)
```

---

## Common Patterns

### Pattern 1: Article Coordination

**Problem:** Need "a swan" but "an eagle"

**Solution:**
```yaml
datatypes:
  creatures:
    values:
      - text: swan
        tags:
          article: a
      - text: eagle
        tags:
          article: an

rules:
  - set: article
    from: ref:creature.tags.article

prompt_sections:
  description:
    template: "{article} {creature}"
    references:
      article:
        target: context:article
      creature:
        target: fantasy:creatures
```

**Output:** "a swan" or "an eagle" ✅

**Learn more:** [Tutorial 3: Article Coordination](../guides/tutorial-series/03-context-rules.md#step-3-use-context-in-template)

---

### Pattern 2: Adjective Priority

**Problem:** With optional adjective, article should match adjective (if present) or noun

**Solution:**
```yaml
rules:
  - set: article
    from: first_selected([ref:adjective, ref:noun]).tags.article

template: "{article} {adjective} {noun}"
```

**Examples:**
- "ancient ruins" → "an ancient ruins" (article from "ancient")
- "brave warrior" → "a brave warrior" (article from "brave")  
- No adjective: "a warrior" (article from "warrior")

---

### Pattern 3: Property Sharing

**Problem:** Want all elements to share the same property (color, habitat, etc.)

**Solution:**
```yaml
rules:
  - set: primary_color
    from: ref:creature.tags.color

prompt_sections:
  scene:
    template: "{creature} in a {primary_color} forest"
    references:
      creature:
        target: game:creatures
      primary_color:
        target: context:primary_color
```

**Output:** Creature's color is reused for the forest ✅

---

### Pattern 4: Conditional Coordination

**Problem:** Behavior changes based on what was selected

**Solution:**
```yaml
rules:
  - set: habitat
    from: ref:creature.tags.habitat
  - set: location_type
    from: ref:habitat  # Reuse context value set by previous rule

# Then use habitat in tag filters
template: "{creature} in the {location#{tags.type == context.habitat}}"
```

**Note:** In v1.0.0, inline filters cannot directly access context. Use separate rules and tag matching instead.

---

## Context Reference Format

### In Templates

To use a context value in a template:

```yaml
template: "{context_key}"
references:
  context_key:
    target: context:keyname  # Note: context: prefix
    min: 1
    max: 1
    separator: null
    unique: false
    filter: null
```

**Key points:**
- Use `context:` prefix for target
- The `keyname` must match the `set` value from a rule
- Context values are populated during Enrichment phase

---

## Context Scope (v1.0.0)

### Simple Scope Model

In v1.0.0, context has a **simple flat scope**:

- All context keys are available to all templates in the same render
- No hierarchical scopes (`.global`, `.prompt`, `.section`)
- Context is cleared between renders (deterministic)

**Future versions** may add scoped context for more complex scenarios.

---

## Rules Best Practices

### 1. Order Matters

✅ **Good:** Rules in logical order
```yaml
rules:
  - set: article        # Set article first
    from: ref:creature.tags.article
  - set: color          # Then set color
    from: ref:creature.tags.color
```

❌ **Bad:** Rules depend on later rules
```yaml
rules:
  - set: description    # Uses 'article' before it's set!
    from: concat(context.article, " creature")
  - set: article
    from: ref:creature.tags.article
```

### 2. Use Meaningful Names

✅ **Good:**
```yaml
- set: article
- set: creature_habitat
- set: primary_color
```

❌ **Bad:**
```yaml
- set: a
- set: h
- set: c
```

### 3. Tag Consistently

Ensure all values have tags used by rules:

✅ **Good:** All creatures have article tag
```yaml
values:
  - text: swan
    tags: {article: a}
  - text: eagle
    tags: {article: an}
```

❌ **Bad:** Missing tags
```yaml
values:
  - text: swan
    tags: {article: a}
  - text: eagle
    tags: {}  # Missing article!
```

### 4. Document Rules

Add comments explaining what rules do:

```yaml
rules:
  # Set article based on first word (adjective or creature)
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article
  
  # Share creature's habitat with environment
  - set: habitat
    from: ref:creature.tags.habitat
```

---

## Validation

The validator checks:

✅ Rules reference existing context keys  
✅ Rules read from valid references  
✅ Tag paths exist in datatype values  
✅ Templates reference context keys that will be set  

⚠️ Warnings for:
- Context keys set but never used
- References in rules that might be null

---

## Advanced: Multiple Rules

You can chain rules to build complex coordination:

```yaml
rules:
  # Step 1: Extract properties
  - set: creature_habitat
    from: ref:creature.tags.habitat
  
  - set: creature_color
    from: ref:creature.tags.color
  
  # Step 2: Use properties for coordination
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article
  
  # Step 3: Conditional logic (stored for later use)
  - set: is_aquatic
    from: ref:creature.tags.habitat == 'water'
```

**Note:** Context values from earlier rules can be used in templates but not in later rule expressions in v1.0.0.

---

## Features Not in v1.0.0

### ContextInterface (Future)

Formal contracts for context keys with:
- Type declarations
- Default values
- Contribution patterns
- Validation rules

**Status:** Simplified in v1.0.0 - context is simple key-value

### Scoped Context (Future)

Hierarchical scopes (`.global`, `.prompt`, `.section`) for:
- Cross-prompt state sharing
- Section-local state
- Nested scope fallback

**Status:** v1.0.0 uses flat scope (simpler)

### Context Conditionals in Templates (Future)

Direct context access in templates:
```yaml
template: "{if context.has('mood') ? context.get('mood') : 'neutral'}"
```

**Status:** v1.0.0 requires rules to set context, then reference with `context:key`

---

## Examples

### Example 1: Simple Article

```yaml
datatypes:
  creatures:
    values:
      - text: owl
        tags: {article: an}
      - text: deer
        tags: {article: a}

rules:
  - set: article
    from: ref:creature.tags.article

prompt_sections:
  scene:
    template: "{article} {creature} in the forest"
    references:
      article: {target: context:article, min: 1, max: 1, ...}
      creature: {target: game:creatures, min: 1, max: 1, ...}
```

**Outputs:**
- "an owl in the forest"
- "a deer in the forest"

---

### Example 2: Adjective Priority

```yaml
datatypes:
  adjectives:
    values:
      - text: ancient
        tags: {article: an}
      - text: brave
        tags: {article: a}
  
  creatures:
    values:
      - text: dragon
        tags: {article: a}

rules:
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article

prompt_sections:
  description:
    template: "{article} {adjective} {creature}"
    references:
      article: {target: context:article, ...}
      adjective: {target: game:adjectives, min: 0, max: 1, ...}
      creature: {target: game:creatures, ...}
```

**Outputs:**
- With adjective: "an ancient dragon" (from "ancient")
- Without adjective: "a dragon" (from "dragon")

---

### Example 3: Multi-Property Coordination

```yaml
datatypes:
  creatures:
    values:
      - text: fish
        tags:
          article: a
          habitat: water
          color: silver
      - text: bird
        tags:
          article: a
          habitat: sky
          color: blue

rules:
  - set: article
    from: ref:creature.tags.article
  - set: habitat
    from: ref:creature.tags.habitat
  - set: color
    from: ref:creature.tags.color

prompt_sections:
  scene:
    template: "{article} {color} {creature} in the {habitat}"
    references:
      article: {target: context:article, ...}
      color: {target: context:color, ...}
      creature: {target: game:creatures, ...}
      habitat: {target: context:habitat, ...}
```

**Outputs:**
- "a silver fish in the water"
- "a blue bird in the sky"

All properties coordinated! ✅

---

## Debugging Tips

### Check Rule Execution

**Problem:** Context value not what you expect

**Debug:**
1. Check rule order - does rule execute before use?
2. Check reference names - does `ref:creature` match template?
3. Check tag existence - do all values have the tag?
4. Check tag path - is it `tags.article` or `tags.stats.article`?

### Common Errors

**Error:** "Context key 'article' not found"
- **Cause:** No rule sets this key
- **Fix:** Add rule: `set: article from: ...`

**Error:** "Reference 'creature' not found in rule"
- **Cause:** Rule references `ref:creature` but template uses different name
- **Fix:** Match reference names

**Error:** "Tag 'article' not found on value"
- **Cause:** Some datatype values missing the tag
- **Fix:** Add tag to all values

---

## Cross-References

**For learning:**
- [Tutorial 3: Context Rules](../guides/tutorial-series/03-context-rules.md) - Complete walkthrough
- [Getting Started](../guides/getting-started.md) - Context basics

**For reference:**
- [Components](./components.md) - Rules data model
- [Template Syntax](./template-syntax.md) - Context references
- [Overview](./overview.md) - System design

---

**This context system reflects the v1.0.0 implementation from M4.**

