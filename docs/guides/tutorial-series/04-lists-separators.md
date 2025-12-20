# Tutorial 4: Lists and Separators - Natural Formatting

**Level:** Advanced  
**Time:** 45 minutes  
**Prerequisites:** [Tutorial 1](./01-basic-package.md), [Tutorial 2](./02-tag-filtering.md)

---

## What You'll Learn

By the end of this tutorial, you'll be able to:

- ✅ Generate multiple values from a single reference
- ✅ Create separator sets for natural formatting
- ✅ Use min/max parameters to control quantity
- ✅ Ensure unique values with the unique flag
- ✅ Build complex list-based prompts

---

## The Problem: Ugly Lists

Suppose you want to list multiple colors:

```yaml
template: "{color} {color} {color}"
```

**Output:** "red blue green" ❌

**Problems:**
- Hard-coded quantity (always 3)
- No natural separators (no commas)
- Might repeat ("red red blue")

**What we want:** "red, blue and green" ✅

---

## The Solution: Min/Max + Separators

Use the `min`, `max`, `separator`, and `unique` parameters on references:

```yaml
references:
  colors:
    target: tutorial:colors
    min: 2
    max: 4
    separator: comma_and
    unique: true
    filter: null
```

---

## Step 1: Create a Separator Set

Separator sets define how to format lists:

```yaml
separator_sets:
  comma_and:
    name: comma_and
    two: "{0} and {1}"
    start: "{0}, "
    middle: "{0}, "
    end: "{0} and {1}"
```

**What this means:**
- `two`: Format when exactly 2 items → "red and blue"
- `start`: Format first item in longer list → "red, "
- `middle`: Format middle items → "blue, "
- `end`: Format last two items → "green and yellow"

**Example with 4 items:**
```
["red", "blue", "green", "yellow"]
         ↓
"red, blue, green and yellow"
 ↑start ↑middle ↑end
```

---

## Step 2: Use Min/Max in Template

Update your promptsection to use the separator:

```yaml
prompt_sections:
  color_list:
    name: color_list
    template: "Colors: {colors}"
    references:
      colors:
        target: tutorial:colors
        min: 2
        max: 4
        separator: comma_and
        unique: true
        filter: null
```

**Parameters explained:**
- `min: 2` - Select at least 2 colors
- `max: 4` - Select at most 4 colors
- `separator: comma_and` - Use the comma_and separator set
- `unique: true` - No duplicates allowed
- `filter: null` - No tag filtering

**Possible outputs:**
- "Colors: red and blue" (2 items)
- "Colors: red, blue and green" (3 items)
- "Colors: red, blue, green and yellow" (4 items)

---

## Complete Example

```yaml
id: tutorial.lists
version: 1.0.0
metadata:
  name: Lists and Separators
  description: Demonstrates natural list formatting
  authors: ["Tutorial"]
  bypass_filters: false

namespaces:
  tutorial:
    id: tutorial
    datatypes:
      colors:
        name: colors
        values:
          - text: red
            weight: 1
            tags: {}
          - text: blue
            weight: 1
            tags: {}
          - text: green
            weight: 1
            tags: {}
          - text: yellow
            weight: 1
            tags: {}
          - text: purple
            weight: 1
            tags: {}
          - text: orange
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
      
      adjectives:
        name: adjectives
        values:
          - text: vibrant
            weight: 1
            tags: {}
          - text: dull
            weight: 1
            tags: {}
          - text: bright
            weight: 1
            tags: {}
          - text: dark
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
    
    separator_sets:
      comma_and:
        name: comma_and
        two: "{0} and {1}"
        start: "{0}, "
        middle: "{0}, "
        end: "{0} and {1}"
      
      comma_or:
        name: comma_or
        two: "{0} or {1}"
        start: "{0}, "
        middle: "{0}, "
        end: "{0} or {1}"
      
      slash:
        name: slash
        two: "{0}/{1}"
        start: "{0}/"
        middle: "{0}/"
        end: "{0}/{1}"
    
    prompt_sections:
      color_list:
        name: color_list
        template: "Available colors: {colors}"
        references:
          colors:
            target: tutorial:colors
            min: 2
            max: 4
            separator: comma_and
            unique: true
            filter: null
      
      color_choice:
        name: color_choice
        template: "Choose {colors}"
        references:
          colors:
            target: tutorial:colors
            min: 2
            max: 3
            separator: comma_or
            unique: true
            filter: null
      
      color_palette:
        name: color_palette
        template: "Palette: {colors}"
        references:
          colors:
            target: tutorial:colors
            min: 3
            max: 5
            separator: slash
            unique: true
            filter: null
    
    rules: []
    decisions: []

dependencies: []
```

**Example outputs:**

`color_list`:
- "Available colors: red and blue"
- "Available colors: red, blue and green"
- "Available colors: red, blue, green and yellow"

`color_choice`:
- "Choose red or blue"
- "Choose red, blue or green"

`color_palette`:
- "Palette: red/blue/green"
- "Palette: red/blue/green/yellow/purple"

---

## Separator Patterns

### Pattern: Comma + And

**Use:** Natural English lists

```yaml
comma_and:
  two: "{0} and {1}"
  start: "{0}, "
  middle: "{0}, "
  end: "{0} and {1}"
```

**Examples:**
- 2 items: "red and blue"
- 3 items: "red, blue and green"
- 4 items: "red, blue, green and yellow"

### Pattern: Comma + Or

**Use:** Choices, alternatives

```yaml
comma_or:
  two: "{0} or {1}"
  start: "{0}, "
  middle: "{0}, "
  end: "{0} or {1}"
```

**Examples:**
- 2 items: "red or blue"
- 3 items: "red, blue or green"

### Pattern: Slash

**Use:** Compact notation, code-style

```yaml
slash:
  two: "{0}/{1}"
  start: "{0}/"
  middle: "{0}/"
  end: "{0}/{1}"
```

**Examples:**
- 2 items: "red/blue"
- 3 items: "red/blue/green"

### Pattern: Newline

**Use:** Vertical lists

```yaml
newline:
  two: "{0}\n{1}"
  start: "{0}\n"
  middle: "{0}\n"
  end: "{0}\n{1}"
```

**Examples:**
- 2 items:
  ```
  red
  blue
  ```
- 3 items:
  ```
  red
  blue
  green
  ```

### Pattern: Numbered

**Use:** Ordered lists

```yaml
numbered:
  two: "1. {0}\n2. {1}"
  start: "1. {0}\n"
  middle: "2. {0}\n"  # Note: This is simplified, real impl would need counter
  end: "2. {0}\n3. {1}"
```

**Note:** Current spec doesn't support auto-incrementing counters. Use fixed numbers or wait for future enhancement.

---

## The Unique Flag

### unique: true

Prevents selecting the same value multiple times:

```yaml
references:
  colors:
    target: tutorial:colors
    min: 3
    max: 3
    unique: true
```

**Possible:** "red, blue and green"  
**Impossible:** "red, red and blue" (can't repeat)

### unique: false

Allows duplicates:

```yaml
references:
  colors:
    target: tutorial:colors
    min: 3
    max: 3
    unique: false
```

**Possible:** "red, red and blue" (duplicates allowed)  
**Also possible:** "red, blue and green"

**When to use:**
- `unique: true` - When duplicates don't make sense (colors in a palette)
- `unique: false` - When repetition is ok (dice rolls, random words)

---

## Min/Max Behavior

### Fixed Quantity

```yaml
min: 3
max: 3
```

**Result:** Always exactly 3 items

### Variable Quantity

```yaml
min: 2
max: 5
```

**Result:** Randomly chooses a number between 2-5, then selects that many items

**Distribution:** Uniform (each quantity equally likely)

### Optional Items

```yaml
min: 0
max: 3
```

**Result:** 0-3 items (can be empty!)

**Use case:** Optional modifiers, sometimes-present elements

---

## Advanced: Combining Lists and Filters

You can combine `min/max/unique/separator` with tag filtering:

```yaml
prompt_sections:
  magical_items:
    name: magical_items
    template: "You find {items}"
    references:
      items:
        target: tutorial:treasures
        min: 2
        max: 4
        separator: comma_and
        unique: true
        filter: "tags.is_magical"  # Only magical items
```

**Or inline:**

```yaml
template: "You find {items#{tags.is_magical}?min=2,max=4,sep=comma_and,unique=true}"
```

**Inline parameter syntax:**
- `min=2` - Set min
- `max=4` - Set max
- `sep=comma_and` - Set separator
- `unique=true` - Set unique flag

**Note:** Use inline for quick prototypes, use reference properties for production packages.

---

## Real-World Example: Character Inventory

```yaml
datatypes:
  weapons:
    values:
      - text: sword
        tags: {type: melee, rarity: common}
      - text: bow
        tags: {type: ranged, rarity: common}
      - text: staff
        tags: {type: magic, rarity: uncommon}
      - text: legendary blade
        tags: {type: melee, rarity: legendary}
  
  potions:
    values:
      - text: health potion
        tags: {effect: healing}
      - text: mana potion
        tags: {effect: magic}
      - text: strength potion
        tags: {effect: buff}

separator_sets:
  inventory:
    two: "{0} and {1}"
    start: "- {0}\n"
    middle: "- {0}\n"
    end: "- {0}\n- {1}"

prompt_sections:
  starting_inventory:
    template: "Your starting equipment:\n{weapons}\n\nPotions:\n{potions}"
    references:
      weapons:
        target: game:weapons
        min: 1
        max: 2
        separator: inventory
        unique: true
        filter: "tags.rarity == 'common'"
      potions:
        target: game:potions
        min: 2
        max: 3
        separator: inventory
        unique: true
        filter: null
```

**Example output:**
```
Your starting equipment:
- sword
- bow

Potions:
- health potion
- mana potion
- strength potion
```

---

## Best Practices

### 1. Set Reasonable Min/Max

✅ **Good:**
```yaml
colors:
  min: 2  # At least 2 for a "palette"
  max: 5  # Not too many to overwhelm
```

❌ **Bad:**
```yaml
colors:
  min: 10  # Datatype only has 6 values!
  max: 20
```

**Rule:** `max` should not exceed available unique values (when `unique: true`)

### 2. Match Separator to Context

✅ **Good:**
```yaml
# Formal description
separator: comma_and
"Colors: red, blue and green"

# Casual list
separator: slash
"Palette: red/blue/green"
```

❌ **Bad:**
```yaml
# Too formal for code
separator: comma_and
"RGB: red, blue and green"  # Use slash instead
```

### 3. Use Unique for Physical Items

✅ **Good:**
```yaml
# Can't have same item twice
inventory_items:
  unique: true
```

❌ **Bad:**
```yaml
# Dice rolls should allow duplicates
dice_rolls:
  unique: true  # Wrong! Can roll same number twice
```

### 4. Consider Empty Lists

If `min: 0`, handle the empty case:

```yaml
# With min: 0
template: "Modifiers{modifiers?min=0,max=3,sep=comma_and}"
# Could output: "Modifiers" (empty)
# Better: "Modifiers: {modifiers}" or check for empty in description
```

---

## Common Mistakes

### Mistake 1: Separator Not Defined

❌ **Wrong:**
```yaml
references:
  colors:
    separator: comma_and  # Separator set doesn't exist!

separator_sets: {}  # Empty!
```

✅ **Correct:**
```yaml
references:
  colors:
    separator: comma_and

separator_sets:
  comma_and:  # Define the separator!
    two: "{0} and {1}"
    # ...
```

### Mistake 2: Max Exceeds Unique Values

❌ **Wrong:**
```yaml
datatypes:
  colors:
    values:
      - text: red
      - text: blue
      # Only 2 values

references:
  colors:
    min: 3  # Can't select 3 unique from 2!
    max: 5
    unique: true
```

✅ **Correct:**
```yaml
# Either reduce max
min: 1
max: 2

# Or add more values
# Or set unique: false
```

### Mistake 3: Wrong Placeholder Format

❌ **Wrong:**
```yaml
comma_and:
  two: "0 and 1"  # Missing curly braces!
  start: "0, "
  middle: "0, "
  end: "0 and 1"
```

✅ **Correct:**
```yaml
comma_and:
  two: "{0} and {1}"  # Use {0} and {1}
  start: "{0}, "
  middle: "{0}, "
  end: "{0} and {1}"
```

---

## Exercises

### Exercise 1: Create a Semicolon Separator

Create a separator that uses semicolons instead of commas:

<details>
<summary>Solution</summary>

```yaml
separator_sets:
  semicolon_and:
    name: semicolon_and
    two: "{0} and {1}"
    start: "{0}; "
    middle: "{0}; "
    end: "{0} and {1}"
```

**Output:** "red; blue and green"
</details>

### Exercise 2: Variable Adjective Count

Create a promptsection that generates 1-3 adjectives before a noun:

<details>
<summary>Solution</summary>

```yaml
prompt_sections:
  described_noun:
    template: "{adjectives} {noun}"
    references:
      adjectives:
        target: tutorial:adjectives
        min: 1
        max: 3
        separator: comma_and
        unique: true
        filter: null
      noun:
        target: tutorial:nouns
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Outputs:**
- "brave knight"
- "brave, clever knight"
- "brave, clever, mysterious knight"
</details>

### Exercise 3: Filtered List

Create a list of 2-4 flying creatures:

<details>
<summary>Solution</summary>

```yaml
prompt_sections:
  flying_group:
    template: "A flock of {creatures#{tags.can_fly}}"
    references:
      creatures:
        target: tutorial:creatures
        min: 2
        max: 4
        separator: comma_and
        unique: true
        filter: null  # Filter is inline in template
```

**Or with filter property:**

```yaml
prompt_sections:
  flying_group:
    template: "A flock of {creatures}"
    references:
      creatures:
        target: tutorial:creatures
        min: 2
        max: 4
        separator: comma_and
        unique: true
        filter: "tags.can_fly"
```
</details>

---

## What's Next?

**You now know:**
- ✅ How to generate multiple values (min/max)
- ✅ How to create separator sets (natural formatting)
- ✅ How to prevent duplicates (unique flag)
- ✅ How to combine lists with filters
- ✅ Common separator patterns

**Further reading:**
- [Separator Sets](../../architecture/components.md#separator-sets) - Technical details
- [Template Syntax](../../architecture/template-syntax.md) - Complete parameter reference
- [Examples](../../examples/text-to-image-prompts.md) - Real-world usage

**You've completed the tutorial series!** 🎉

**Next steps:**
- Build your own packages
- Study [real examples](../../examples/)
- Read the [full specification](../../architecture/overview.md)
- Wait for the desktop authoring tool (Q1 2026)

---

## Summary

Lists and separators enable **natural language generation**:

**Problem:** Multiple values need natural formatting  
**Solution:** Min/max parameters + separator sets

**Key concepts:**
- `min`/`max`: Control quantity (1-N values)
- `unique`: Prevent/allow duplicates
- `separator`: Name of separator set to use
- Separator set: Defines formatting for 2, start, middle, end

**Separator patterns:**
- Comma + And: "red, blue and green"
- Comma + Or: "red, blue or green"
- Slash: "red/blue/green"
- Newline: Vertical lists
- Custom: Any format you need

**Remember:**
- Create separator sets in the namespace
- Reference by name in template/reference
- Use `{0}` and `{1}` placeholders in separators
- Set realistic min/max for your datatype size
- Use `unique: true` when duplicates don't make sense

**You can now generate beautifully formatted lists!** ✨

**Congratulations on completing the tutorial series!** 🎊

