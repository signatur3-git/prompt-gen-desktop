# Getting Started with Random Prompt Generator

**Welcome!** This guide will help you get up and running with the Random Prompt Generator (RPG) specification and reference implementation.

---

## What is RPG?

The Random Prompt Generator is a **template-based system** for creating varied, contextually-aware text prompts. It's particularly useful for:

- **Text-to-image AI prompts** (Midjourney, DALL-E, Stable Diffusion)
- **Creative writing prompts**
- **Game content generation** (NPC descriptions, quest text, item names)
- **Procedural storytelling**

### Key Features

✨ **Template-Based** - Define patterns, not individual outputs  
🎲 **Deterministic** - Same seed = same output (reproducible)  
🔗 **Context-Aware** - Elements coordinate (articles, pluralization, agreement)  
🏷️ **Tag Filtering** - "flying creatures" or "nocturnal animals"  
📦 **Modular** - Reusable packages with namespaces  
🎨 **Visual Authoring** - No coding required (desktop app included)

---

## Quick Start (3 Options)

### Option 1: Use the Desktop Authoring Tool (Coming Soon!)

> **📅 Status:** Desktop app will be available in standalone repository at v1.0.0 (Q1 2026)

**Best for:** Visual creators who want to create packages without writing YAML

**What it will offer:**
1. Visual package creation wizard
2. Component editors for all types (datatypes, promptsections, rules)
3. Real-time preview with live rendering
4. Validation with helpful error messages
5. Export to YAML

**Time:** 15 minutes to first package (when available)

**For now:** Use Option 2 (YAML) or wait for official release

---

### Option 2: Write Packages in YAML (Available Now)

**Best for:** Developers comfortable with text editors

1. **Learn** the package format (see [Package Structure](#your-first-package))
2. **Create** a YAML file following the schema
3. **Study** existing examples in `docs/examples/`
4. **Validate** your package structure manually

**Time:** 30 minutes to first package

**Note:** CLI validator will be available at v1.0.0. For now, check syntax against examples.

---

### Option 3: Read the Spec and Implement (Available Now)

**Best for:** Developers building their own implementation

1. **Read** the [Specification](../architecture/overview.md)
2. **Study** the [Architecture](../architecture/components.md)
3. **Review** examples in `docs/examples/`
4. **Implement** in your language of choice

**Time:** 1-2 weeks for basic implementation

---

## Installation

> **⚠️ Note:** The reference implementation is currently under development and not yet published as a standalone repository. Until v1.0.0, the desktop authoring tool and CLI are available only for developers working on the specification itself.

### For Users (After v1.0.0)

**The desktop app and CLI will be available as:**
- Pre-built binaries for Windows, macOS, and Linux
- npm package: `npm install -g @rpg/cli`
- Standalone repository: `github.com/your-org/rpg-implementation`

**Expected release:** Q1 2026

### For Contributors (Now)

If you're contributing to the specification and have access to the reference implementation:

**Prerequisites:**
- **Node.js** 18+
- **Rust** 1.70+
- **Git**

**Setup:**
```bash
# The reference-impl directory is gitignored in the spec repo
# Ask maintainers for access or wait for standalone release
cd reference-impl/rpg-desktop
npm install
npm run tauri:dev
```

### For Implementers (Now)

If you want to build your own implementation:

1. **Clone the specification:**
```bash
git clone https://github.com/your-org/prompt-gen-spec.git
cd prompt-gen-spec
```

2. **Read the spec:**
   - [Architecture Overview](../architecture/overview.md)
   - [Component Details](../architecture/components.md)
   - [Template Syntax](../architecture/template-syntax.md)

3. **Study the examples:**
   - See `docs/examples/` for working package examples
   - Read source-of-truth documents in `source-of-truth/`

4. **Implement in your language:**
   - Follow the spec (language-agnostic)
   - Reference the documented design decisions
   - Test against example packages

---

## Your First Package

Let's create a simple package that generates color + object combinations like "red ball" or "blue apple".

### Creating the Package (YAML)

Create a file called `my-first-package.yaml`:

```yaml
id: my.first
version: 1.0.0
metadata:
  name: My First Package
  description: Simple color + object combinations
  authors: ["Your Name"]
  bypass_filters: false

namespaces:
  test:
    id: test
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
        extends: null
        override_tags: {}
      
      objects:
        name: objects
        values:
          - text: ball
            weight: 1
            tags: {}
          - text: apple
            weight: 1
            tags: {}
          - text: car
            weight: 1
            tags: {}
        extends: null
        override_tags: {}
    
    prompt_sections:
      simple:
        name: simple
        template: "{color} {object}"
        references:
          color:
            target: test:colors
            min: 1
            max: 1
            separator: null
            unique: false
            filter: null
          object:
            target: test:objects
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

### Understanding the Structure

**Package metadata:**
- `id`: Unique identifier (reverse domain notation)
- `version`: Semantic version
- `metadata`: Name, description, authors

**Namespace `test`:**
- Contains all our components
- Organizes datatypes and promptsections

**Datatypes:**
- `colors`: List of color values (red, blue, green)
- `objects`: List of object values (ball, apple, car)
- Each value has `weight: 1` (equal probability)

**PromptSection `simple`:**
- Template: `{color} {object}` - placeholders for values
- References define what each placeholder pulls from
- `test:colors` and `test:objects` reference our datatypes

### Testing the Package

**Manual validation:**
- Check YAML syntax with any YAML validator
- Verify all references point to existing datatypes
- Ensure namespace IDs match

**Expected outputs** (conceptually):
- Seed 42: "red apple"
- Seed 123: "blue ball"
- Seed 456: "green car"

> **Note:** The CLI validator and renderer will be available at v1.0.0. For now, verify your package structure matches the example.

### Using the Desktop App (When Available)

Once the desktop authoring tool is released, you'll be able to:

1. **Click "Create New Package"**
2. **Fill in metadata:** ID, name, version
3. **Add namespace:** `test`
4. **Add datatypes:** Use visual editor to add `colors` and `objects`
5. **Add promptsection:** `simple` with template `{color} {object}`
6. **Preview live:** See results with different seeds
7. **Export:** Save as YAML file

**This workflow will be much faster than hand-writing YAML!**

---

## Core Concepts

### Packages

A **package** is a collection of reusable content organized into namespaces. Think of it like an npm package or Python module.

**Structure:**
- **Metadata:** ID, version, name, description, authors
- **Namespaces:** Organize components (like folders)
- **Dependencies:** Reference other packages

### Namespaces

A **namespace** groups related components together to avoid naming conflicts.

**Contains:**
- **Datatypes:** Lists of values (like "colors" or "animals")
- **PromptSections:** Templates that combine values
- **SeparatorSets:** How to format lists ("red, blue and green")
- **Rules:** Context coordination (articles, pluralization)
- **Decisions:** Advanced context operations (future)

### Datatypes

A **datatype** is a list of weighted values with optional tags.

**Example:**
```yaml
creatures:
  name: creatures
  values:
    - text: swan
      weight: 1
      tags:
        can_fly: true
        can_swim: true
        article: an
    - text: deer
      weight: 1
      tags:
        can_fly: false
        can_swim: false
        article: a
```

**Uses:**
- Basic lists (colors, numbers, adjectives)
- Tagged lists (filter by properties)
- Weighted lists (some values more common)
- Extensible lists (build on other datatypes)

### PromptSections

A **promptsection** is a template that references datatypes.

**Example:**
```yaml
peaceful_scene:
  name: peaceful_scene
  template: "{article} {adjective} {creature} {action} in {location}"
  references:
    article:
      target: context:article  # From context (set by rules)
    adjective:
      target: test:adjectives
    creature:
      target: test:creatures
    action:
      target: test:actions
    location:
      target: test:locations
```

**Features:**
- **References:** `{name}` pulls from datatypes
- **Min/Max:** `{colors?min=2,max=4}` select multiple
- **Separators:** `{colors?sep=comma_and}` format lists
- **Unique:** `{colors?unique=true}` no duplicates
- **Tag Filters:** `{creatures#{tags.can_fly}}` filter by tags
- **Nested:** References to other promptsections

### Rules

**Rules** coordinate between template elements using the context.

**Example:**
```yaml
rules:
  - set: article
    from: first_selected([ref:adjective, ref:creature]).tags.article
```

**Common Uses:**
- **Article selection:** "a swan" vs "an eagle"
- **Pluralization:** Adjust text based on quantity
- **Gender agreement:** Match adjectives to nouns
- **Custom coordination:** Any context-based logic

### Tag Filtering

**Tag filtering** selects values based on properties.

**Syntax:** `{datatype#{expression}}`

**Examples:**
```yaml
# Only flying creatures
template: "{creatures#{tags.can_fly}}"

# Nocturnal AND can_fly
template: "{creatures#{tags.nocturnal && tags.can_fly}}"

# Can fly OR can swim
template: "{creatures#{tags.can_fly || tags.can_swim}}"

# NOT aquatic
template: "{creatures#{!tags.aquatic}}"
```

**Operators:** `&&` (AND), `||` (OR), `!` (NOT)

---

## Common Patterns

### Pattern 1: Lists with Natural Formatting

**Goal:** Generate "red, blue and green" (not "red blue green")

**Solution:**

1. **Define separator set:**
```yaml
separator_sets:
  comma_and:
    name: comma_and
    two: "{0} and {1}"
    start: "{0}, "
    middle: "{0}, "
    end: "{0} and {1}"
```

2. **Use in template:**
```yaml
template: "{colors?min=2,max=4,sep=comma_and}"
```

---

### Pattern 2: Article Coordination

**Goal:** "a swan" not "an swan", "an eagle" not "a eagle"

**Solution:**

1. **Tag values with articles:**
```yaml
creatures:
  values:
    - text: swan
      tags:
        article: a
    - text: eagle
      tags:
        article: an
```

2. **Add rule to set context:**
```yaml
rules:
  - set: article
    from: ref:creature.tags.article
```

3. **Use in template:**
```yaml
template: "{article} {creature}"
references:
  article:
    target: context:article
  creature:
    target: test:creatures
```

---

### Pattern 3: Filtered Selection

**Goal:** Only select flying creatures

**Solution:**

```yaml
template: "{creature} flying through the sky"
references:
  creature:
    target: test:creatures
    filter: "tags.can_fly"  # Only creatures that can fly
```

Or inline:

```yaml
template: "{creatures#{tags.can_fly}} flying"
```

---

### Pattern 4: Nested Templates

**Goal:** Compose complex prompts from smaller pieces

**Solution:**

```yaml
prompt_sections:
  greeting:
    template: "Greetings, {name}!"
    references:
      name:
        target: test:names
  
  full_scene:
    template: "{greeting} {scene_description}"
    references:
      greeting:
        target: test:greeting  # Reference to another promptsection
      scene_description:
        target: test:scene
```

---

## Best Practices

### 1. Use Meaningful IDs

✅ **Good:** `fantasy.creatures`, `scifi.weapons`, `modern.vehicles`  
❌ **Bad:** `pkg1`, `data`, `stuff`

### 2. Tag for Filtering

✅ **Good:** Tag values with properties that matter for coordination  
❌ **Bad:** Tag everything with every possible property

**Example:**
```yaml
# Good - tags that affect template logic
creatures:
  values:
    - text: swan
      tags:
        article: a
        can_fly: true
        can_swim: true

# Overkill - irrelevant tags
creatures:
  values:
    - text: swan
      tags:
        color: white
        wingspan_cm: 240
        scientific_name: Cygnus olor
        diet: herbivore
        # ... 20 more tags
```

### 3. Keep Templates Simple

✅ **Good:** Declarative, template-like  
❌ **Bad:** Complex nested logic

**Good:**
```yaml
template: "{article} {adjective} {creature} {action}"
```

**Bad (avoid):**
```yaml
# Don't try to implement conditionals in templates
# Use rules or separate promptsections instead
template: "{if_flying#{flying_action}else#{ground_action}}"
```

### 4. Use Namespaces

✅ **Good:** Organize by theme or purpose  
❌ **Bad:** Everything in one namespace

**Good:**
```yaml
namespaces:
  fantasy:
    # Fantasy-specific content
  scifi:
    # Sci-fi specific content
  common:
    # Shared content
```

### 5. Document Your Packages

Add descriptions and comments:

```yaml
metadata:
  name: Fantasy Creatures
  description: |
    A comprehensive package of fantasy creatures with
    proper article coordination and tag filtering support.
  authors: ["Your Name"]
```

---

## Next Steps

### Learn More

- **[Architecture Overview](../architecture/overview.md)** - How the system works
- **[Template Syntax](../architecture/template-syntax.md)** - Complete syntax reference
- **[Context Interactions](../architecture/context-interactions.md)** - Rules and coordination
- **[Tag Filtering](../architecture/tag-filtering.md)** - Advanced filtering

### Tutorials

- **[Tutorial 1: Basic Packages](./tutorial-series/01-basic-package.md)** - Create your first package
- **[Tutorial 2: Tag Filtering](./tutorial-series/02-tag-filtering.md)** - Use tags effectively
- **[Tutorial 3: Context Rules](./tutorial-series/03-context-rules.md)** - Coordinate elements
- **[Tutorial 4: Nested Templates](./tutorial-series/04-nested-templates.md)** - Compose complexity

### Examples

- **[Text-to-Image Prompts](../examples/text-to-image-prompts.md)** - Real-world examples
- **[Test Packages](../../reference-impl/rpg-desktop/test-packages/)** - Working examples

### Tools

- **[CLI Guide](../tools/cli-guide.md)** - Command-line interface
- **[Validation Guide](../tools/validation-guide.md)** - Package validation
- **[Error Reference](../tools/error-reference.md)** - Common errors

---

## Getting Help

### Common Issues

**"Reference not found"**
- Check spelling of datatype names
- Verify namespace prefix (e.g., `test:colors` not just `colors`)
- Ensure datatype exists in the namespace

**"Circular reference detected"**
- Check for promptsection A → B → A loops
- Use validator to see the full chain

**"Invalid tag expression"**
- Check syntax: `#{tags.property}` not `#{property}`
- Verify operators: `&&`, `||`, `!`
- Use parentheses for complex expressions

### Resources

- **[GitHub Issues](https://github.com/your-org/prompt-gen-spec/issues)** - Bug reports and questions
- **[Discussions](https://github.com/your-org/prompt-gen-spec/discussions)** - Community help
- **[Specification](../architecture/overview.md)** - Complete technical reference

---

## Contributing

Want to improve RPG? We welcome:

- **Bug reports** - Found an issue? Report it!
- **Package examples** - Share interesting packages
- **Documentation improvements** - Clarify confusing sections
- **Feature requests** - Suggest enhancements

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for details.

---

## FAQ

**Q: When will the desktop authoring tool be available?**  
A: The desktop app is part of the v1.0.0 release, planned for Q1 2026. It's currently in the reference implementation phase (not yet published). Until then, create packages by writing YAML following the examples in this guide.

**Q: Do I need to know programming to use RPG?**  
A: Eventually, no! The desktop authoring tool will make it accessible without programming. Currently, you need to write YAML, which requires basic text editing skills but not programming knowledge.

**Q: Can I use RPG commercially?**  
A: Yes! The spec and reference implementation are open source. Check the [LICENSE](../../LICENSE) for details.

**Q: What's the difference between datatypes and promptsections?**  
A: Datatypes are lists of values. PromptSections are templates that combine those values.

**Q: Can I extend packages from others?**  
A: Yes! Use the `dependencies` field and reference components with `namespace:component` syntax. This will be fully supported at v1.0.0.

**Q: Is output always the same for a given seed?**  
A: Yes! That's the deterministic guarantee. Same seed + same package = same output.

**Q: Can I use this for game development?**  
A: Absolutely! RPG is perfect for procedural content generation in games.

**Q: What's the performance like?**  
A: Fast! Rust backend renders thousands of prompts per second. Exact speed depends on complexity.

**Q: Can I implement this in Python/JavaScript/etc.?**  
A: Yes! The spec is language-agnostic. The Rust implementation is a reference, not a requirement.

**Q: Where can I find working examples?**  
A: Check `docs/examples/` in the spec repository for various example packages showing different features.

---

## Summary

You now know:

✅ What RPG is and what it's used for  
✅ Three ways to get started (visual, YAML, or implement)  
✅ Core concepts (packages, namespaces, datatypes, promptsections)  
✅ Common patterns (lists, articles, filtering, nesting)  
✅ Best practices (meaningful IDs, tagging, simple templates)  
✅ Where to get help and learn more

**Ready to create?** Start with Option 1 (desktop app) or jump to [Tutorial 1](./tutorial-series/01-basic-package.md)!

**Have questions?** Check the [FAQ](#faq) or ask in [Discussions](https://github.com/your-org/prompt-gen-spec/discussions)!

**Happy generating!** 🎲✨

