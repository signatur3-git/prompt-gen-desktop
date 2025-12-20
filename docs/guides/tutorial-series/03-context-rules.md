# Tutorial 3: Context Rules - Element Coordination

**Level:** Intermediate  
**Time:** 45 minutes  
**Prerequisites:** [Tutorial 1](./01-basic-package.md), [Tutorial 2](./02-tag-filtering.md)

---

## What You'll Learn

By the end of this tutorial, you'll be able to:

- ✅ Understand the context system
- ✅ Create rules to coordinate elements
- ✅ Solve the article problem ("a" vs "an")
- ✅ Use context values in templates
- ✅ Chain rules together

---

## The Problem: "A Eagle" 

Consider this template:

```yaml
template: "A {creature} soaring"
```

**Possible outputs:**
- "A swan soaring" ✅ Correct
- "A eagle soaring" ❌ **Wrong!** Should be "**an** eagle"
- "A owl soaring" ❌ **Wrong!** Should be "**an** owl"

**The problem:** We need "a" or "an" depending on the creature selected, but we don't know which creature will be chosen until runtime.

---

## The Solution: Context & Rules

The **context** is a key-value store that gets populated during rendering. **Rules** read from references and write to the context.

**How it works:**
1. Template references select values
2. Rules examine those values and set context
3. Template uses context values

---

## Step 1: Tag Values with Articles

First, add article information to your creatures:

```yaml
datatypes:
  creatures:
    name: creatures
    values:
      - text: swan
        weight: 1
        tags:
          article: a
      
      - text: eagle
        weight: 1
        tags:
          article: an
      
      - text: owl
        weight: 1
        tags:
          article: an
      
      - text: deer
        weight: 1
        tags:
          article: a
    extends: null
    override_tags: {}
```

**What we added:**
- Each value has an `article` tag
- "swan" and "deer" use "a"
- "eagle" and "owl" use "an"

---

## Step 2: Create a Rule

Add a rule that reads the creature's article tag and sets it in context:

```yaml
namespaces:
  tutorial:
    id: tutorial
    datatypes:
      # ...creatures as above
    
    rules:
      - set: article
        from: ref:creature.tags.article
    
    prompt_sections:
      # ...we'll add this next
```

**Understanding the rule:**
- `set: article` - Write to context key "article"
- `from: ref:creature.tags.article` - Read from creature's article tag
- `ref:creature` - References the value selected for the "creature" reference

---

## Step 3: Use Context in Template

Now update your promptsection to use the context:

```yaml
prompt_sections:
  flying_scene:
    name: flying_scene
    template: "{article} {creature} soaring through the sky"
    references:
      article:
        target: context:article
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      
      creature:
        target: tutorial:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Key points:**
- `{article}` references `context:article`
- `context:` is a special namespace for context values
- The rule will set `article` before the template renders

---

## How It Works: Execution Flow

```
1. Engine selects value for {creature} reference
   Example: "eagle"

2. Rules execute (Enrichment Phase)
   Rule reads: ref:creature.tags.article = "an"
   Rule writes: context.article = "an"

3. Template renders (Rendering Phase)
   {article} → reads context.article → "an"
   {creature} → "eagle"
   
4. Final output: "an eagle soaring through the sky" ✅
```

---

## Complete Example

```yaml
id: tutorial.articles
version: 1.0.0
metadata:
  name: Article Coordination
  description: Demonstrates context rules for article selection
  authors: ["Tutorial"]
  bypass_filters: false

namespaces:
  tutorial:
    id: tutorial
    datatypes:
      creatures:
        name: creatures
        values:
          - text: swan
            weight: 1
            tags:
              article: a
              can_fly: true
          
          - text: eagle
            weight: 1
            tags:
              article: an
              can_fly: true
          
          - text: owl
            weight: 1
            tags:
              article: an
              can_fly: true
          
          - text: deer
            weight: 1
            tags:
              article: a
              can_fly: false
        extends: null
        override_tags: {}
    
    rules:
      - set: article
        from: ref:creature.tags.article
    
    prompt_sections:
      simple_scene:
        name: simple_scene
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
            target: tutorial:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
      
      flying_scene:
        name: flying_scene
        template: "{article} {creature#{tags.can_fly}} soaring in the sky"
        references:
          article:
            target: context:article
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          creature:
            target: tutorial:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
    
    separator_sets: {}
    decisions: []

dependencies: []
```

**Test outputs:**
- `simple_scene`: "a swan", "an eagle", "an owl", "a deer" ✅
- `flying_scene`: "a swan soaring...", "an eagle soaring...", "an owl soaring..." ✅

---

## Advanced: Multiple Rules

You can have multiple rules working together:

```yaml
datatypes:
  adjectives:
    values:
      - text: brave
        tags:
          article: a
      - text: ancient
        tags:
          article: an
  
  creatures:
    values:
      - text: knight
        tags:
          article: a
      - text: eagle
        tags:
          article: an

rules:
  # Rule 1: Set article from adjective OR creature
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article

prompt_sections:
  description:
    template: "{article} {adjective} {creature}"
    references:
      article:
        target: context:article
      adjective:
        target: tutorial:adjectives
      creature:
        target: tutorial:creatures
```

**What `first_selected()` does:**
- Checks if `adjective` was selected (has value)
- If yes, uses its article tag
- If no, uses creature's article tag
- Ensures the article matches the first word

**Examples:**
- "adjective + creature" → "a brave knight" (article from "brave")
- "adjective + creature" → "an ancient eagle" (article from "ancient")

---

## Rule Syntax Reference

### Basic Rule

```yaml
- set: context_key
  from: expression
```

### Reading from References

```yaml
# Read entire reference value
from: ref:creature

# Read a tag from reference
from: ref:creature.tags.article

# Read nested tag
from: ref:creature.tags.stats.level
```

### Functions

**`first_selected([...])`**
```yaml
# Use first non-null value
from: first_selected([ref:adjective, ref:creature]).tags.article
```

**Future functions** (not yet implemented):
- `if(condition, true_value, false_value)`
- `concat(value1, value2)`
- `pluralize(value, count)`

---

## Pattern: Adjective Priority

When you have adjective + noun, the article should match the adjective (if present):

```yaml
rules:
  - set: article
    from: first_selected([ref:adjective, ref:noun]).tags.article

prompt_sections:
  description:
    template: "{article} {adjective} {noun}"
```

**Why this works:**
- If adjective selected: Uses adjective's article
- If no adjective: Falls back to noun's article
- Always grammatically correct!

**Examples:**
- "ancient ruins" → "an ancient ruins" (from "ancient")
- "brave warrior" → "a brave warrior" (from "brave")
- No adjective: "a warrior" (from "warrior")

---

## Pattern: Pluralization (Conceptual)

While pluralization helpers aren't in v1.0.0, you can prepare for them:

```yaml
datatypes:
  creatures:
    values:
      - text: swan
        tags:
          plural: swans
          article: a
      - text: wolf
        tags:
          plural: wolves
          article: a

# Future (not yet implemented):
rules:
  - set: creature_text
    from: if(ref:count > 1, ref:creature.tags.plural, ref:creature)
  - set: article
    from: if(ref:count > 1, '', ref:creature.tags.article)
```

**For now:** Use separate datatypes for singular and plural:

```yaml
datatypes:
  creature_singular:
    values:
      - text: swan
        tags: {article: a}
      - text: wolf
        tags: {article: a}
  
  creature_plural:
    values:
      - text: swans
        tags: {article: ''}  # No article for plural
      - text: wolves
        tags: {article: ''}

prompt_sections:
  singular:
    template: "{article} {creature}"
    references:
      article: {target: context:article}
      creature: {target: tutorial:creature_singular}
  
  plural:
    template: "{creature}"  # No article needed
    references:
      creature: {target: tutorial:creature_plural}
```

---

## Best Practices

### 1. Set Rules Before They're Used

✅ **Good:**
```yaml
rules:
  - set: article
    from: ref:creature.tags.article

prompt_sections:
  scene:
    template: "{article} {creature}"  # Article already set by rule
```

❌ **Bad:**
```yaml
prompt_sections:
  scene:
    template: "{article} {creature}"  # Tries to use article before it's set!

rules:
  - set: article
    from: ref:creature.tags.article
```

**Note:** Rules are processed in order, before template rendering.

### 2. Use Meaningful Context Keys

✅ **Good:**
```yaml
rules:
  - set: article
  - set: creature_count
  - set: primary_color
```

❌ **Bad:**
```yaml
rules:
  - set: a
  - set: cc
  - set: pc
```

### 3. Tag Consistently

Ensure ALL values have the required tags:

✅ **Good:**
```yaml
values:
  - text: swan
    tags: {article: a}
  - text: eagle
    tags: {article: an}
  - text: owl
    tags: {article: an}
```

❌ **Bad:**
```yaml
values:
  - text: swan
    tags: {article: a}
  - text: eagle
    # Missing article tag!
  - text: owl
    tags: {article: an}
```

### 4. Document Your Rules

Add comments explaining what rules do:

```yaml
rules:
  # Set article based on first word (adjective or creature)
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article
  
  # Set creature's habitat for environment coordination
  - set: habitat
    from: ref:creature.tags.habitat
```

---

## Common Mistakes

### Mistake 1: Wrong Reference Name

❌ **Wrong:**
```yaml
rules:
  - set: article
    from: ref:animal.tags.article  # Reference is called "creature" not "animal"!

references:
  creature:
    target: tutorial:creatures
```

✅ **Correct:**
```yaml
rules:
  - set: article
    from: ref:creature.tags.article  # Matches reference name

references:
  creature:
    target: tutorial:creatures
```

### Mistake 2: Forgetting `context:` Prefix

❌ **Wrong:**
```yaml
references:
  article:
    target: article  # Missing context: prefix!
```

✅ **Correct:**
```yaml
references:
  article:
    target: context:article  # context: prefix required
```

### Mistake 3: Undefined Tag

❌ **Wrong:**
```yaml
rules:
  - set: article
    from: ref:creature.tags.article  # Tag might not exist!

datatypes:
  creatures:
    values:
      - text: swan
        tags: {}  # No article tag defined!
```

✅ **Correct:**
```yaml
datatypes:
  creatures:
    values:
      - text: swan
        tags:
          article: a  # Always define tags used in rules
```

---

## Exercises

### Exercise 1: Add Color Coordination

Add a `color` tag to creatures and create a rule that sets it:

```yaml
- text: swan
  tags:
    article: a
    color: white
- text: eagle
  tags:
    article: an
    color: brown
```

Create a rule and template that uses the color.

<details>
<summary>Solution</summary>

```yaml
rules:
  - set: article
    from: ref:creature.tags.article
  - set: color
    from: ref:creature.tags.color

prompt_sections:
  colored_scene:
    template: "{article} {color} {creature}"
    references:
      article:
        target: context:article
      color:
        target: context:color
      creature:
        target: tutorial:creatures
```

**Outputs:** "a white swan", "an brown eagle"

**Note:** "an brown" is grammatically wrong! This shows why `first_selected()` is important when you have adjectives.
</details>

### Exercise 2: Habitat Coordination

Create a system where creature habitat determines the location:

```yaml
creatures:
  - text: fish
    tags:
      habitat: water
  - text: bird
    tags:
      habitat: sky

locations:
  - text: lake
    tags:
      type: water
  - text: clouds
    tags:
      type: sky

rules:
  - set: habitat
    from: ref:creature.tags.habitat

template: "{creature} in the {location#{tags.type == context.habitat}}"
```

Wait! Inline filters can't access context yet. Use this approach instead:

<details>
<summary>Solution</summary>

Create separate promptsections:

```yaml
rules:
  - set: habitat
    from: ref:creature.tags.habitat

prompt_sections:
  aquatic_scene:
    template: "{creature#{tags.habitat == 'water'}} swimming in the {location#{tags.type == 'water'}}"
  
  aerial_scene:
    template: "{creature#{tags.habitat == 'sky'}} flying in the {location#{tags.type == 'sky'}}"
```

This ensures matching habitats without relying on context in filters.
</details>

---

## Real-World Use Case: Complex Descriptions

```yaml
datatypes:
  adjectives:
    values:
      - text: ancient
        tags: {article: an, intensity: high}
      - text: brave
        tags: {article: a, intensity: medium}
  
  creatures:
    values:
      - text: dragon
        tags: {article: a, power: high}
      - text: knight
        tags: {article: a, power: medium}
  
  actions:
    values:
      - text: soaring
        tags: {requires_flight: true}
      - text: standing
        tags: {requires_flight: false}

rules:
  # Set article from first word
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article

prompt_sections:
  complex_scene:
    template: "{article} {adjective} {creature} {action}"
    references:
      article: {target: context:article}
      adjective: {target: tutorial:adjectives}
      creature: {target: tutorial:creatures}
      action: {target: tutorial:actions}
```

**Outputs:**
- "an ancient dragon soaring" ✅ (article from "ancient")
- "a brave knight standing" ✅ (article from "brave")

---

## What's Next?

**You now know:**
- ✅ How context works (key-value store)
- ✅ How to create rules (set context from references)
- ✅ How to use context in templates (context: prefix)
- ✅ Article coordination pattern
- ✅ Rule execution flow

**Next tutorial:**
- **[Tutorial 4: Lists and Separators](./04-lists-separators.md)** - Generate multiple values with natural formatting

**Further reading:**
- [Context Interactions](../../architecture/context-interactions.md) - Technical details
- [Rules Engine](../../architecture/components.md#rules) - Complete rule syntax

---

## Summary

Context rules solve **coordination problems**:

**Problem:** Template elements need to match/coordinate  
**Solution:** Rules read from selections and write to context

**Key concepts:**
- **Context:** Key-value store for coordination
- **Rules:** `set` context from reference tags
- **Execution:** Rules run before template rendering
- **Reference:** Use `ref:name` to read selected values
- **Usage:** Reference context with `context:key`

**Common patterns:**
- Article selection: `first_selected([ref:adj, ref:noun]).tags.article`
- Property coordination: `ref:creature.tags.habitat`
- Multi-step coordination: Multiple rules setting different context keys

**Remember:**
- Rules execute in order
- All referenced values must have required tags
- Context keys can be any name
- Use `first_selected()` for adjective + noun patterns

**You can now create grammatically correct, coordinated prompts!** 📝

**Next:** Learn how to generate lists with [Tutorial 4: Lists and Separators](./04-lists-separators.md)!

