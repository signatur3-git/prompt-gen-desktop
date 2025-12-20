# Tutorial 2: Tag Filtering - Smart Value Selection

**Level:** Intermediate  
**Time:** 45 minutes  
**Prerequisites:** [Tutorial 1: Basic Package](./01-basic-package.md)

---

## What You'll Learn

By the end of this tutorial, you'll be able to:

- ✅ Add tags to datatype values
- ✅ Use tag filtering to select specific values
- ✅ Create logical expressions (AND, OR, NOT)
- ✅ Build context-aware prompts
- ✅ Solve the "flying swan" problem

---

## The Problem: Unrealistic Combinations

Consider this package:

```yaml
datatypes:
  creatures:
    values:
      - text: swan
      - text: deer
      - text: eagle
  
  actions:
    values:
      - text: flying
      - text: running
      - text: swimming

prompt_sections:
  scene:
    template: "{creature} {action}"
```

**Possible outputs:**
- "swan flying" ✅ Good
- "eagle flying" ✅ Good
- "deer flying" ❌ **Problem!** Deer can't fly!
- "swan running" ❌ **Problem!** Swans don't run!

**We need a way to ensure physical plausibility.**

---

## The Solution: Tags

Tags are properties you add to values to describe them. Then you can filter based on those properties.

---

## Step 1: Add Tags to Values

Let's update our creatures with ability tags:

```yaml
datatypes:
  creatures:
    name: creatures
    values:
      - text: swan
        weight: 1
        tags:
          can_fly: true
          can_run: false
          can_swim: true
      
      - text: deer
        weight: 1
        tags:
          can_fly: false
          can_run: true
          can_swim: false
      
      - text: eagle
        weight: 1
        tags:
          can_fly: true
          can_run: false
          can_swim: false
    extends: null
    override_tags: {}
```

**What we added:**
- `tags`: Key-value pairs describing each value
- `can_fly`, `can_run`, `can_swim`: Boolean properties

---

## Step 2: Create Action-Specific Datatypes

Now create datatypes for different action types:

```yaml
datatypes:
  flying_creatures:
    name: flying_creatures
    values: []  # Empty - we'll filter from creatures
    extends: creatures
    override_tags: {}
```

Wait! This won't work. We need a different approach.

**Better solution:** Use tag filtering in the template directly.

---

## Step 3: Use Inline Tag Filtering

Update your promptsection to use tag filters:

```yaml
prompt_sections:
  flying_scene:
    name: flying_scene
    template: "{creature#{tags.can_fly}} flying through the sky"
    references:
      creature:
        target: tutorial:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Tag filter syntax:** `{reference#{expression}}`

**What happens:**
1. Engine looks at `tutorial:creatures`
2. Filters to only values where `tags.can_fly` is true
3. Selects from: swan, eagle (deer excluded!)
4. Result: Always realistic! ✅

---

## Step 4: Multiple Filter Expressions

Create different scenes for different abilities:

```yaml
prompt_sections:
  flying_scene:
    name: flying_scene
    template: "{creature#{tags.can_fly}} flying through the sky"
    references:
      creature:
        target: tutorial:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
  
  running_scene:
    name: running_scene
    template: "{creature#{tags.can_run}} running across the field"
    references:
      creature:
        target: tutorial:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
  
  swimming_scene:
    name: swimming_scene
    template: "{creature#{tags.can_swim}} swimming in the lake"
    references:
      creature:
        target: tutorial:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Outputs:**
- `flying_scene`: "swan flying..." or "eagle flying..." (never deer)
- `running_scene`: "deer running..." (only deer)
- `swimming_scene`: "swan swimming..." (only swan)

---

## Advanced: Logical Operators

### AND Operator (`&&`)

Select creatures that can fly AND swim:

```yaml
template: "{creature#{tags.can_fly && tags.can_swim}} soaring over the water"
```

**Result:** Only swan (can both fly and swim)

### OR Operator (`||`)

Select creatures that can fly OR run:

```yaml
template: "{creature#{tags.can_fly || tags.can_run}} moving swiftly"
```

**Result:** swan, deer, or eagle (all can do at least one)

### NOT Operator (`!`)

Select creatures that CANNOT fly:

```yaml
template: "{creature#{!tags.can_fly}} walking on the ground"
```

**Result:** Only deer

### Complex Expressions

Combine operators with parentheses:

```yaml
template: "{creature#{(tags.can_fly || tags.can_swim) && !tags.can_run}}"
```

**Translation:** Can fly OR swim, but NOT run  
**Result:** swan or eagle (both can fly/swim but not run)

---

## Complete Example: Fantasy Creatures

Let's build a more complex example:

```yaml
id: tutorial.fantasy
version: 1.0.0
metadata:
  name: Fantasy Creatures with Tags
  description: Demonstrates tag filtering for realistic outputs
  authors: ["Tutorial"]
  bypass_filters: false

namespaces:
  fantasy:
    id: fantasy
    datatypes:
      creatures:
        name: creatures
        values:
          - text: dragon
            weight: 1
            tags:
              can_fly: true
              is_magical: true
              is_dangerous: true
              size: large
          
          - text: fairy
            weight: 1
            tags:
              can_fly: true
              is_magical: true
              is_dangerous: false
              size: tiny
          
          - text: wolf
            weight: 1
            tags:
              can_fly: false
              is_magical: false
              is_dangerous: true
              size: medium
          
          - text: unicorn
            weight: 1
            tags:
              can_fly: false
              is_magical: true
              is_dangerous: false
              size: large
        extends: null
        override_tags: {}
      
      locations:
        name: locations
        values:
          - text: enchanted forest
            weight: 1
            tags:
              is_magical: true
              is_dangerous: false
          
          - text: dark cavern
            weight: 1
            tags:
              is_magical: false
              is_dangerous: true
          
          - text: crystal palace
            weight: 1
            tags:
              is_magical: true
              is_dangerous: false
          
          - text: mountain peak
            weight: 1
            tags:
              is_magical: false
              is_dangerous: false
        extends: null
        override_tags: {}
    
    prompt_sections:
      aerial_scene:
        name: aerial_scene
        template: "A {creature#{tags.can_fly}} soaring above the {location}"
        references:
          creature:
            target: fantasy:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          location:
            target: fantasy:locations
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
      
      magical_scene:
        name: magical_scene
        template: "A {creature#{tags.is_magical}} in the {location#{tags.is_magical}}"
        references:
          creature:
            target: fantasy:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          location:
            target: fantasy:locations
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
      
      dangerous_encounter:
        name: dangerous_encounter
        template: "A {creature#{tags.is_dangerous}} lurking in the {location#{tags.is_dangerous}}"
        references:
          creature:
            target: fantasy:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          location:
            target: fantasy:locations
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
      
      peaceful_scene:
        name: peaceful_scene
        template: "A {creature#{!tags.is_dangerous}} resting in the {location#{!tags.is_dangerous}}"
        references:
          creature:
            target: fantasy:creatures
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          location:
            target: fantasy:locations
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
    
    separator_sets: {}
    rules: []
    decisions: []

dependencies: []
```

**Example outputs:**

`aerial_scene`:
- "A dragon soaring above the enchanted forest"
- "A fairy soaring above the mountain peak"
- Never wolf or unicorn (can't fly)

`magical_scene`:
- "A dragon in the enchanted forest" (both magical)
- "A fairy in the crystal palace" (both magical)
- Never wolf (not magical)

`dangerous_encounter`:
- "A dragon lurking in the dark cavern" (both dangerous)
- "A wolf lurking in the dark cavern" (both dangerous)
- Never fairy or unicorn (not dangerous)

`peaceful_scene`:
- "A fairy resting in the enchanted forest" (both peaceful)
- "A unicorn resting in the crystal palace" (both peaceful)
- Never dragon or wolf (dangerous)

---

## Tag Types

### Boolean Tags

```yaml
tags:
  can_fly: true
  is_magical: false
```

**Usage:** `{ref#{tags.can_fly}}`

### String Tags

```yaml
tags:
  size: large
  color: red
  alignment: good
```

**Usage:** `{ref#{tags.size == 'large'}}`

**Note:** String comparisons use single quotes

### Numeric Tags

```yaml
tags:
  level: 5
  hitpoints: 100
```

**Usage:** `{ref#{tags.level > 3}}`

**Operators:** `>`, `<`, `>=`, `<=`, `==`, `!=`

---

## Best Practices

### 1. Use Meaningful Tag Names

✅ **Good:**
```yaml
tags:
  can_fly: true
  is_nocturnal: true
  habitat: forest
```

❌ **Bad:**
```yaml
tags:
  f: true
  n: true
  h: forest
```

### 2. Tag for Behavior, Not Everything

✅ **Good:** Tag properties that affect template logic
```yaml
tags:
  can_fly: true      # Affects which actions are valid
  is_magical: true   # Affects location compatibility
```

❌ **Overkill:** Tag irrelevant details
```yaml
tags:
  scientific_name: "Cygnus olor"
  average_lifespan: 20
  diet: "herbivore"
  # ... only tag what you'll actually filter on
```

### 3. Keep Expressions Simple

✅ **Good:**
```yaml
template: "{creature#{tags.can_fly}}"
template: "{creature#{tags.can_fly && tags.is_magical}}"
```

❌ **Too Complex:**
```yaml
template: "{creature#{(tags.can_fly || (tags.can_swim && !tags.is_aquatic)) && (tags.size == 'large' || tags.level > 5)}}"
# Hard to understand and maintain
```

### 4. Document Your Tags

Add comments in your package:

```yaml
creatures:
  # Tags:
  # - can_fly: boolean - Can the creature fly?
  # - is_magical: boolean - Is the creature magical?
  # - is_dangerous: boolean - Is the creature dangerous?
  # - size: string - tiny, small, medium, large, huge
  values:
    - text: dragon
      tags:
        can_fly: true
        is_magical: true
        is_dangerous: true
        size: large
```

---

## Common Mistakes

### Mistake 1: Forgetting Quotes for Strings

❌ **Wrong:**
```yaml
template: "{creature#{tags.size == large}}"  # large not quoted!
```

✅ **Correct:**
```yaml
template: "{creature#{tags.size == 'large'}}"
```

### Mistake 2: Using Filter Property AND Inline Filter

❌ **Wrong:**
```yaml
template: "{creature#{tags.can_fly}}"
references:
  creature:
    target: fantasy:creatures
    filter: "tags.is_magical"  # Both inline AND property - don't do this
```

✅ **Correct:**
```yaml
# Use inline tag filter
template: "{creature#{tags.can_fly && tags.is_magical}}"
references:
  creature:
    target: fantasy:creatures
    filter: null  # Use null when using inline filters
```

**OR**

```yaml
# Use filter property (without inline)
template: "{creature}"
references:
  creature:
    target: fantasy:creatures
    filter: "tags.can_fly && tags.is_magical"
```

### Mistake 3: Undefined Tags

❌ **Wrong:**
```yaml
values:
  - text: dragon
    tags:
      can_fly: true
      # is_magical not defined!

template: "{creature#{tags.is_magical}}"  # Some values don't have this tag
```

✅ **Correct:**
```yaml
values:
  - text: dragon
    tags:
      can_fly: true
      is_magical: true  # Define for ALL values
```

---

## Exercises

### Exercise 1: Add Habitat Tags

Add a `habitat` tag to creatures (values: forest, mountain, water, sky):

```yaml
- text: dragon
  tags:
    habitat: mountain
- text: fairy
  tags:
    habitat: forest
```

Then create a promptsection that only selects forest creatures.

<details>
<summary>Solution</summary>

```yaml
prompt_sections:
  forest_scene:
    name: forest_scene
    template: "{creature#{tags.habitat == 'forest'}} dwelling in the woods"
    references:
      creature:
        target: fantasy:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```
</details>

### Exercise 2: Flying OR Magical

Create a promptsection that selects creatures that can either fly OR are magical:

<details>
<summary>Solution</summary>

```yaml
prompt_sections:
  special_scene:
    name: special_scene
    template: "{creature#{tags.can_fly || tags.is_magical}} with extraordinary abilities"
    references:
      creature:
        target: fantasy:creatures
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```
</details>

### Exercise 3: Large Non-Dangerous

Create a filter for creatures that are large but NOT dangerous:

<details>
<summary>Solution</summary>

```yaml
template: "{creature#{tags.size == 'large' && !tags.is_dangerous}}"
# Result: Only unicorn (large but not dangerous)
```
</details>

---

## Real-World Use Case: Text-to-Image Prompts

Tag filtering is essential for generating physically plausible image prompts:

```yaml
animals:
  values:
    - text: eagle
      tags:
        can_fly: true
        habitat: sky
        time_active: diurnal
    
    - text: owl
      tags:
        can_fly: true
        habitat: forest
        time_active: nocturnal
    
    - text: bear
      tags:
        can_fly: false
        habitat: forest
        time_active: diurnal

prompt_sections:
  night_sky_scene:
    template: "{animal#{tags.can_fly && tags.time_active == 'nocturnal'}} flying under the moonlight"
    # Result: Only owl (can fly AND nocturnal)
  
  daytime_flight:
    template: "{animal#{tags.can_fly && tags.time_active == 'diurnal'}} soaring in the sunlight"
    # Result: Only eagle (can fly AND diurnal)
```

**Without tag filtering:** Could generate "bear flying under moonlight" ❌  
**With tag filtering:** Only physically plausible combinations ✅

---

## What's Next?

**You now know:**
- ✅ How to add tags to values
- ✅ How to use inline tag filtering
- ✅ Logical operators (AND, OR, NOT)
- ✅ When and why to use tags
- ✅ Best practices for tagging

**Next tutorials:**
- **[Tutorial 3: Context Rules](./03-context-rules.md)** - Coordinate between elements (articles, pluralization)
- **[Tutorial 4: Lists and Separators](./04-lists-separators.md)** - Generate multiple values with natural formatting

**Further reading:**
- [Tag Filtering Architecture](../../architecture/tag-filtering.md) - Technical details
- [Context Interactions](../../architecture/context-interactions.md) - How rules use tags

---

## Summary

Tag filtering solves the **physical plausibility problem**:

**Problem:** Random combinations can be unrealistic  
**Solution:** Tag values with properties, filter by those properties

**Key syntax:**
- Inline filter: `{ref#{expression}}`
- Boolean: `{ref#{tags.can_fly}}`
- String: `{ref#{tags.size == 'large'}}`
- Numeric: `{ref#{tags.level > 5}}`
- Logical: `&&` (AND), `||` (OR), `!` (NOT)

**Remember:**
- Tag for behavior that affects templates
- Keep expressions simple
- Document your tags
- Define tags consistently across all values

**You can now create contextually-aware, physically plausible prompts!** 🎯

**Next:** Learn how to coordinate elements with [Tutorial 3: Context Rules](./03-context-rules.md)!

