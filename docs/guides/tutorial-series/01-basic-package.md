# Tutorial 1: Creating Your First Package

**Level:** Beginner  
**Time:** 30 minutes  
**Prerequisites:** Text editor, basic YAML knowledge

---

## What You'll Learn

By the end of this tutorial, you'll be able to:

- ✅ Understand package structure
- ✅ Create a simple package from scratch
- ✅ Define datatypes with values
- ✅ Create promptsections with templates
- ✅ Use references to connect templates to data
- ✅ Test your package (manually)

---

## The Goal

We'll create a package that generates simple character descriptions like:

- "A brave knight"
- "A clever wizard"
- "A mysterious rogue"

---

## Step 1: Create the Package File

Create a new file called `character-descriptions.yaml`:

```yaml
id: tutorial.characters
version: 1.0.0
metadata:
  name: Character Descriptions
  description: Simple fantasy character descriptions
  authors: ["Your Name"]
  bypass_filters: false

namespaces: {}
dependencies: []
```

**What's this?**
- `id`: Unique identifier for your package (use reverse domain notation)
- `version`: Semantic version (major.minor.patch)
- `metadata`: Information about your package
- `namespaces`: Where all your content will go (we'll add this next)
- `dependencies`: Other packages this one needs (none for now)

---

## Step 2: Add a Namespace

Namespaces organize your content. Let's add one called `fantasy`:

```yaml
id: tutorial.characters
version: 1.0.0
metadata:
  name: Character Descriptions
  description: Simple fantasy character descriptions
  authors: ["Your Name"]
  bypass_filters: false

namespaces:
  fantasy:
    id: fantasy
    datatypes: {}
    prompt_sections: {}
    separator_sets: {}
    rules: []
    decisions: []

dependencies: []
```

**What's a namespace?**
- A container for related components
- Prevents naming conflicts (like folders)
- You can have multiple namespaces in one package

---

## Step 3: Create Your First Datatype

Datatypes are lists of values. Let's create one for adjectives:

```yaml
namespaces:
  fantasy:
    id: fantasy
    datatypes:
      adjectives:
        name: adjectives
        values:
          - text: brave
            weight: 1
            tags: {}
          - text: clever
            weight: 1
            tags: {}
          - text: mysterious
            weight: 1
            tags: {}
          - text: wise
            weight: 1
            tags: {}
          - text: cunning
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
    # ...rest stays the same
```

**Understanding datatypes:**
- `name`: The datatype identifier
- `values`: Array of possible values
  - `text`: The actual text to use
  - `weight`: Probability (1 = equal chance)
  - `tags`: Properties for filtering (we'll use this in Tutorial 2)
- `extends`: Inherit from another datatype (null = none)
- `override_tags`: Modify inherited tags (we'll skip this for now)

---

## Step 4: Add Another Datatype

Now add character classes:

```yaml
datatypes:
  adjectives:
    # ...as above
  
  classes:
    name: classes
    values:
      - text: knight
        weight: 1
        tags: {}
      - text: wizard
        weight: 1
        tags: {}
      - text: rogue
        weight: 1
        tags: {}
      - text: archer
        weight: 1
        tags: {}
      - text: cleric
        weight: 1
        tags: {}
    extends: null
    override_tags: {}
```

**You now have two datatypes:**
1. `adjectives` - descriptive words
2. `classes` - character types

---

## Step 5: Create a PromptSection

PromptSections are templates that combine datatypes:

```yaml
prompt_sections:
  simple_description:
    name: simple_description
    template: "A {adjective} {class}"
    references:
      adjective:
        target: fantasy:adjectives
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      class:
        target: fantasy:classes
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```

**Understanding promptsections:**
- `name`: The promptsection identifier
- `template`: Pattern with `{placeholders}`
- `references`: Define what each placeholder pulls from
  - `target`: Which datatype to use (format: `namespace:datatype`)
  - `min`/`max`: How many values to select (1 = one value)
  - `separator`: How to join multiple values (null = just one value)
  - `unique`: Prevent duplicates (false = can repeat)
  - `filter`: Tag filter expression (null = no filtering)

**The template `"A {adjective} {class}"`:**
- Will replace `{adjective}` with a random adjective
- Will replace `{class}` with a random class
- Result: "A brave knight", "A clever wizard", etc.

---

## Step 6: Complete Package

Here's your complete package:

```yaml
id: tutorial.characters
version: 1.0.0
metadata:
  name: Character Descriptions
  description: Simple fantasy character descriptions
  authors: ["Your Name"]
  bypass_filters: false

namespaces:
  fantasy:
    id: fantasy
    datatypes:
      adjectives:
        name: adjectives
        values:
          - text: brave
            weight: 1
            tags: {}
          - text: clever
            weight: 1
            tags: {}
          - text: mysterious
            weight: 1
            tags: {}
          - text: wise
            weight: 1
            tags: {}
          - text: cunning
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
      
      classes:
        name: classes
        values:
          - text: knight
            weight: 1
            tags: {}
          - text: wizard
            weight: 1
            tags: {}
          - text: rogue
            weight: 1
            tags: {}
          - text: archer
            weight: 1
            tags: {}
          - text: cleric
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
    
    prompt_sections:
      simple_description:
        name: simple_description
        template: "A {adjective} {class}"
        references:
          adjective:
            target: fantasy:adjectives
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          class:
            target: fantasy:classes
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

---

## Step 7: Test Your Package (Manual)

Since the CLI isn't available yet, test manually:

**Check structure:**
1. ✅ Does it have an `id` and `version`?
2. ✅ Does it have a `fantasy` namespace?
3. ✅ Does `fantasy` have two datatypes?
4. ✅ Does `fantasy` have one promptsection?
5. ✅ Do references point to existing datatypes?

**Verify references:**
- `adjective` references `fantasy:adjectives` ✅
- `class` references `fantasy:classes` ✅

**Check template:**
- Template uses `{adjective}` and `{class}` ✅
- Both are defined in references ✅

**If all checks pass, your package is valid!** 🎉

---

## Understanding What You Built

### Data Flow

```
1. Template: "A {adjective} {class}"
                     ↓              ↓
2. References:   adjective      class
                     ↓              ↓
3. Datatypes:  fantasy:adjectives  fantasy:classes
                     ↓              ↓
4. Values:    [brave, clever...]  [knight, wizard...]
                     ↓              ↓
5. Selection:      "clever"        "rogue"
                     ↓              ↓
6. Result:    "A clever rogue"
```

### Key Concepts

**Separation of Data and Templates:**
- Datatypes = reusable data
- PromptSections = reusable templates
- References = connect templates to data

**This design allows:**
- Multiple templates using same datatypes
- Extending datatypes without changing templates
- Sharing packages and mixing data from different sources

---

## Exercises

Try these modifications to practice:

### Exercise 1: Add More Adjectives

Add 3 more adjectives to the `adjectives` datatype:
- powerful
- ancient
- fierce

<details>
<summary>Solution</summary>

```yaml
adjectives:
  name: adjectives
  values:
    # ...existing values
    - text: powerful
      weight: 1
      tags: {}
    - text: ancient
      weight: 1
      tags: {}
    - text: fierce
      weight: 1
      tags: {}
```
</details>

### Exercise 2: Create a New PromptSection

Create a new promptsection called `formal_introduction` with template:
```
"Behold, {adjective} {class} of the realm!"
```

<details>
<summary>Solution</summary>

```yaml
prompt_sections:
  simple_description:
    # ...existing promptsection
  
  formal_introduction:
    name: formal_introduction
    template: "Behold, {adjective} {class} of the realm!"
    references:
      adjective:
        target: fantasy:adjectives
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      class:
        target: fantasy:classes
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```
</details>

### Exercise 3: Add Weapons Datatype

Create a new datatype called `weapons` with values:
- sword, bow, staff, dagger, axe

Then create a promptsection `character_with_weapon`:
```
"A {adjective} {class} wielding a {weapon}"
```

<details>
<summary>Solution</summary>

```yaml
datatypes:
  # ...existing datatypes
  weapons:
    name: weapons
    values:
      - text: sword
        weight: 1
        tags: {}
      - text: bow
        weight: 1
        tags: {}
      - text: staff
        weight: 1
        tags: {}
      - text: dagger
        weight: 1
        tags: {}
      - text: axe
        weight: 1
        tags: {}
    extends: null
    override_tags: {}

prompt_sections:
  # ...existing promptsections
  character_with_weapon:
    name: character_with_weapon
    template: "A {adjective} {class} wielding a {weapon}"
    references:
      adjective:
        target: fantasy:adjectives
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      class:
        target: fantasy:classes
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
      weapon:
        target: fantasy:weapons
        min: 1
        max: 1
        separator: null
        unique: false
        filter: null
```
</details>

---

## Common Mistakes

### Mistake 1: Wrong Reference Target

❌ **Wrong:**
```yaml
references:
  adjective:
    target: adjectives  # Missing namespace!
```

✅ **Correct:**
```yaml
references:
  adjective:
    target: fantasy:adjectives  # namespace:datatype
```

### Mistake 2: Reference Name Doesn't Match Template

❌ **Wrong:**
```yaml
template: "A {adj} {class}"
references:
  adjective:  # Doesn't match {adj}!
    target: fantasy:adjectives
```

✅ **Correct:**
```yaml
template: "A {adjective} {class}"
references:
  adjective:  # Matches {adjective}
    target: fantasy:adjectives
```

### Mistake 3: Missing Namespace ID

❌ **Wrong:**
```yaml
namespaces:
  fantasy:
    datatypes: {}  # Missing id!
```

✅ **Correct:**
```yaml
namespaces:
  fantasy:
    id: fantasy  # Must match the key
    datatypes: {}
```

---

## What's Next?

**You now know:**
- ✅ How to structure a package
- ✅ How to create datatypes
- ✅ How to create promptsections
- ✅ How references connect templates to data

**Next tutorials:**
- **[Tutorial 2: Tag Filtering](./02-tag-filtering.md)** - Select values by properties
- **[Tutorial 3: Context Rules](./03-context-rules.md)** - Coordinate between elements
- **[Tutorial 4: Lists and Separators](./04-lists-separators.md)** - Generate multiple values

**Want more?**
- Read the [Getting Started Guide](../getting-started.md) for concepts
- Check [Architecture Overview](../../architecture/overview.md) for technical details
- See [Examples](../../examples/text-to-image-prompts.md) for real-world packages

---

## Summary

In this tutorial, you:

1. ✅ Created a package from scratch
2. ✅ Defined metadata and namespaces
3. ✅ Created two datatypes (adjectives, classes)
4. ✅ Built a promptsection with a template
5. ✅ Connected template to data with references
6. ✅ Validated your package manually

**You're ready to create your own packages!** 🎉

**Next:** Learn how to use tags to filter values in [Tutorial 2: Tag Filtering](./02-tag-filtering.md)!

