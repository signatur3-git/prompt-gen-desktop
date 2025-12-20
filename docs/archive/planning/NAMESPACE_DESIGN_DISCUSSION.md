# Namespace Design: Dots are Convention, Not Hierarchy! 🔍

**Date:** 2025-12-17  
**Issue:** User expected Java-style hierarchical packages  
**Resolution:** ✅ Fixed validation to allow dots; clarified mental model

---

## TL;DR

**✅ FIXED:** AddNamespaceDialog now allows dots (restart dev server to see changes)

**Mental Model Clarification:**

**Spec Says:** `featured.common` is a **namespace identifier with dot convention**
- Dots suggest semantic grouping (like `org.apache.*` in Java)
- But technically implemented as flat HashMap keys
- No enforced parent/child relationships in the data structure

**You're Right About Java:**
- `org.apache.commons` and `org.apache.kafka` DO share semantic meaning
- The `org.apache` prefix signals "Apache Foundation projects"
- This is **organizational hierarchy** (mental model) ✅
- But Java's file system structure is **technical implementation detail**

**RPG Design:**
- **Same mental model:** `featured.common` suggests "common utilities in featured library"
- **Different technical implementation:** Flat keys in HashMap, not nested objects
- **Why:** Distribution is package-level, not namespace-level; simpler implementation

---

## What the Spec Actually Says

### From RFC 0001 and source-of-truth:

> **Namespace** – Logical grouping containing datatypes, promptsections, separatorsets, rulebooks, pools, morphers, and contextinterfaces.

**Key Quote from component-overview.md:**
> "Namespace: A logical grouping (e.g., `featured.common`) within a package."

**From models.rs comment:**
```rust
/// Namespace identifier (e.g., "featured.common")
pub id: String,
```

**Not:**
```rust
/// Namespace path with hierarchy
pub path: Vec<String>,  // ["featured", "common"]
```

---

## The Design Rationale

### Why Dots in Names?

**Convention, not structure:**
- Like DNS: `example.com` is ONE name, not a hierarchy
- Like reverse domains: `com.example.myapp`
- Suggests organization without enforcing it

**Examples from spec:**
- `featured.common` - Shared library namespace
- `story.fantasy` - Fantasy story elements
- `ontologies.en.articles` - English article rules
- `ontologies.de.nouns` - German noun data

**Pattern:** `category.subcategory` or `package.module`

### Why NOT Hierarchical?

**1. Packages are the Unit of Distribution:**
```yaml
# Package A
namespaces:
  featured.common:      # Namespace 1
    datatypes: ...
  featured.advanced:    # Namespace 2 (separate, not child!)
    datatypes: ...
```

**Not:**
```yaml
# NOT THIS:
namespaces:
  featured:              # Parent namespace
    common:              # Child namespace
      datatypes: ...
    advanced:            # Another child
      datatypes: ...
```

**2. References Use Full Namespace:**
```yaml
template: "{featured.common:animal}"  # Full namespace name
```

**Not:**
```yaml
template: "{featured/common:animal}"  # NO hierarchy traversal
```

**3. Dependencies are Package-Level:**
```yaml
dependencies:
  - package_id: "featured-library"  # Import whole package
    version: "1.0.0"
```

**Not:**
```yaml
dependencies:
  - namespace: "featured.common"  # Can't import sub-namespaces
```

---

## Current Implementation is Correct

### Data Model (models.rs):

```rust
pub struct Package {
    pub namespaces: HashMap<String, Namespace>,  // ✅ Flat map
}

pub struct Namespace {
    pub id: String,  // ✅ Just a name, e.g., "featured.common"
    pub datatypes: HashMap<String, Datatype>,
    // ...
}
```

**This matches the spec!**

### Package YAML Structure:

```yaml
id: "featured-library"
version: "1.0.0"
namespaces:
  featured.common:       # Key is full namespace name
    datatypes:
      animal:
        values: [cat, dog]
  featured.advanced:     # Another flat key
    datatypes:
      creature:
        values: [dragon, griffin]
```

**NOT hierarchical:**
```yaml
# This would be WRONG:
namespaces:
  featured:
    common:
      datatypes: ...
    advanced:
      datatypes: ...
```

---

## Why This Design Makes Sense

### 1. **Simplicity**
- No complex path resolution
- No parent/child relationship rules
- No inheritance confusion

### 2. **Clarity**
- Each namespace is independent
- Full name always explicit in references
- No ambiguity about scope

### 3. **Flexibility**
- Authors choose their own naming conventions
- `featured.common` and `story.common` don't conflict
- Easy to reorganize without breaking references

### 4. **Package-Oriented**
- Packages are the distribution unit
- Namespaces are just organizational labels
- Dependencies work at package level

---

## Comparison to Java Packages

### Important Distinction: Semantic vs Technical Hierarchy

**You're right!** Java packages DO have hierarchy, but in **two different ways:**

### 1. Semantic/Organizational Hierarchy (Mental Model) ✅

**Java:**
```java
org.apache.commons.lang     // Commons library, language utilities
org.apache.commons.io       // Commons library, I/O utilities  
org.apache.kafka.clients    // Kafka library, client code
```

**What this means:**
- `org.apache.*` = "Apache Foundation projects" (organizational grouping)
- `org.apache.commons.*` = "Apache Commons library" (shared purpose)
- The prefix **signals semantic relationship** ✅

**RPG (Same Concept!):**
```yaml
featured.common      # Featured library, common utilities
featured.advanced    # Featured library, advanced features
ontologies.en        # Ontologies library, English language
ontologies.de        # Ontologies library, German language
```

**What this means:**
- `featured.*` = "Featured library components" (organizational grouping)
- `ontologies.*` = "Language ontology data" (shared purpose)
- The prefix **signals semantic relationship** ✅

**This IS hierarchy in the mental model!** You're absolutely right.

### 2. Technical Implementation (Data Structure)

**Java:**
```
Technically: File system directories (can traverse up/down)
src/
  org/
    apache/
      commons/
        lang/
          StringUtils.java
```

**RPG:**
```
Technically: Flat HashMap (no traversal, just lookup)
namespaces: {
  "featured.common": {...},
  "featured.advanced": {...},
  "ontologies.en": {...}
}
```

**The Difference:**
- Java: Can do `import org.apache.commons.*` (wildcard parent import)
- RPG: Must reference full name `featured.common:animal` (no wildcards)

**Why RPG is flatter:**
- Packages are distribution unit (you import whole packages, not namespaces)
- No need for directory structure (YAML keys, not files)
- Simpler implementation (HashMap vs tree structure)

---

## What You Were Worried About (Valid Concern!)

### The Question:
> "With `org.apache`, the projects under that namespace have something in common functionally speaking."

**Answer:** YES! Absolutely! This is the **semantic hierarchy** and RPG has it too!

### The Mental Model SHOULD Be:

**Organizational Grouping:**
- `featured.common` → "Common utilities from the Featured library"
- `featured.advanced` → "Advanced features from the Featured library"
- Both share the `featured.` prefix because they're related!

**This is good design!** It helps users understand:
- Where components come from
- What they're related to
- How to organize their own namespaces

### What You Should Do:

**✅ Use dots to show semantic grouping:**
```yaml
myproject.core      # Your project's core functionality
myproject.utils     # Your project's utilities
myproject.fantasy   # Your project's fantasy-specific content
```

**✅ Follow conventions like Java reverse-domain:**
```yaml
com.example.myapp.creatures
com.example.myapp.locations
com.example.myapp.items
```

**This creates the mental model of hierarchy without requiring technical hierarchy!**

---

## Comparison to Java Packages (Updated)

### Java Packages (Semantic + Technical Hierarchy):
```java
package com.example.myapp.models;  // Hierarchy

import com.example.myapp.utils.*;   // Can import parent
import com.example.other.Helper;    // Different tree
```

**Structure:**
```
com/
  example/
    myapp/
      models/
        User.java
      utils/
        Helper.java
```

### RPG Namespaces (Flat with Convention):
```yaml
namespace: com.example.myapp.models  # Just a name

references:
  helper: com.example.myapp.utils:helper_function  # Full name required
  other: com.example.other:something              # No hierarchy
```

**Structure:**
```yaml
namespaces:
  com.example.myapp.models:    # Flat key
    datatypes: ...
  com.example.myapp.utils:     # Another flat key
    datatypes: ...
  com.example.other:           # Unrelated flat key
    datatypes: ...
```

**No directory tree, no parent/child!**

---

## Examples from Spec

### Example 1: Multiple Namespaces in One Package

```yaml
id: "featured-library"
namespaces:
  featured.common:      # Shared utilities
    datatypes:
      animal: [cat, dog]
  featured.fantasy:     # Fantasy-specific
    datatypes:
      creature: [dragon, unicorn]
  featured.scifi:       # SciFi-specific
    datatypes:
      technology: [laser, robot]
```

**These are 3 independent namespaces:**
- NOT a hierarchy (`featured` → `common`, `fantasy`, `scifi`)
- Just naming convention (all start with `featured.`)
- Each can be referenced independently

### Example 2: Cross-Package References

**Package A:**
```yaml
id: "ontologies-english"
namespaces:
  ontologies.en.articles:
    rules:
      - when: ref:noun.tags.article
        set: context.prompt.article
        value: ref:noun.tags.article
```

**Package B:**
```yaml
id: "story-templates"
dependencies:
  - package_id: "ontologies-english"
    version: "1.0.0"
namespaces:
  story.fantasy:
    prompt_sections:
      intro:
        template: "{context.prompt.article} {creature}"
        references:
          creature: ontologies.en.articles:noun  # Full namespace reference
```

**Note:** Can't do `ontologies:en:articles:noun` - that's not how it works!

---

## Why AddNamespaceDialog Constraints are Correct

### Current Validation:
```javascript
pattern="[a-z][a-z0-9_-]*"
```

**This is TOO RESTRICTIVE!** Should allow dots!

### Should Be:
```javascript
pattern="[a-z][a-z0-9._-]*"  // Add dot to allowed chars
```

**Why:**
- Spec examples use dots: `featured.common`, `ontologies.en.articles`
- Dots are part of naming convention
- Still validates as flat string (no hierarchy implied)

---

## The Fix Needed

### Current AddNamespaceDialog (WRONG):

```vue
<input
  id="namespace-id"
  v-model="namespaceId"
  type="text"
  placeholder="e.g., common, creatures, locations"
  pattern="[a-z][a-z0-9_-]*"  <!-- ❌ No dots! -->
  required
/>
<small>Lowercase alphanumeric with underscores/hyphens</small>
```

### Should Be (CORRECT):

```vue
<input
  id="namespace-id"
  v-model="namespaceId"
  type="text"
  placeholder="e.g., common, featured.common, ontologies.en.articles"
  pattern="[a-z][a-z0-9._-]*"  <!-- ✅ Dots allowed! -->
  required
/>
<small>Lowercase alphanumeric with dots, underscores, hyphens (e.g., "featured.common")</small>
```

**Why this works:**
- Still validates as flat string
- Allows naming convention from spec
- No hierarchy enforcement needed

---

## Mental Model

### ❌ Don't Think:
```
Hierarchical tree like Java:
  featured/
    common/
    advanced/
  story/
    fantasy/
```

### ✅ Think Instead:
```
Flat list with naming conventions:
  - "featured.common"      (shared utilities)
  - "featured.advanced"    (advanced features)
  - "story.fantasy"        (fantasy stories)
  - "myapp.custom"         (your custom stuff)
```

**Like email domains:**
- `gmail.com` and `mail.google.com` are different names
- No parent/child relationship
- Just naming conventions

---

## When Hierarchy DOES Exist

### At Package Level:

```yaml
# Package dependencies form a graph (not tree):
packages:
  my-app:
    depends on: [featured-library, ontologies-english]
  featured-library:
    depends on: [ontologies-english]
  ontologies-english:
    depends on: []
```

**This is a dependency graph, not namespace hierarchy!**

### At Component Level:

```yaml
# PromptSections can nest:
prompt_sections:
  scene:
    template: "{location_description} {character_action}"
    references:
      location_description: featured.common:describe_location  # Nested!
      character_action: story.fantasy:character_action
```

**This is composition, not namespace hierarchy!**

---

## Summary

### What You Expected (Java-style):
```
Semantic hierarchy (mental model):
  org.apache
    ├── commons (related by organization)
    └── kafka (related by organization)
  
Technical hierarchy (file structure):
  org/
    apache/
      commons/
      kafka/
```

### What RPG Actually Has:
```
Semantic hierarchy (mental model): ✅ YES!
  featured
    ├── common (related by purpose)
    └── advanced (related by purpose)
  
Technical hierarchy (data structure): ❌ NO!
  Flat HashMap:
    - "featured.common"
    - "featured.advanced"
```

### Your Concern Was Valid:

**You said:** "With `org.apache`, the projects under that namespace have something in common functionally speaking."

**Answer:** YES! And RPG has this too!
- `featured.*` namespaces ARE semantically related
- `ontologies.*` namespaces ARE semantically related  
- The dots **DO signal meaningful grouping**
- This is **intentional and good design**

### The Difference Is Only Technical:

**Java:**
- Semantic hierarchy: ✅ (org.apache.* are related)
- Technical hierarchy: ✅ (file system directories, can traverse)

**RPG:**
- Semantic hierarchy: ✅ (featured.* are related)
- Technical hierarchy: ❌ (HashMap keys, no traversal)

**Why the difference:**
- Java distributes by JAR (can contain multiple packages, wildcards work)
- RPG distributes by Package (whole packages at once, no wildcards needed)
- Simpler implementation without loss of semantic meaning

### Why It's Better This Way:
1. **Simpler** - No hierarchy rules to learn
2. **Clearer** - Full names always explicit
3. **Flexible** - Easy to reorganize
4. **Package-oriented** - Distribution unit is clear

### What Needs Fixing:
1. ✅ **AddNamespaceDialog** - Allow dots in namespace IDs
2. ✅ **Documentation** - Clarify naming convention vs hierarchy
3. ✅ **Examples** - Show `featured.common` style names

---

## Recommendation

**Keep the flat namespace design!** It's correct per spec and has good reasons.

**But fix:**
1. AddNamespaceDialog validation (allow dots)
2. Placeholder text (show `featured.common` examples)
3. Help text (explain convention vs hierarchy)

**Don't change:**
- Data model (HashMap is correct)
- Reference syntax (full namespace name required)
- Package structure (flat namespaces map)

---

## The Pitch (for Flat Namespaces)

**Why flat namespaces with dot convention > hierarchical:**

1. **Distribution is Package-Based**
   - You publish/install packages, not namespaces
   - Hierarchy would add complexity without value

2. **References Must Be Explicit**
   - `{featured.common:animal}` is clear
   - No ambiguity about which namespace

3. **No "Parent" Namespace Concept**
   - What would `featured` contain without `.common`?
   - Avoids empty parent namespaces

4. **Simpler Mental Model**
   - Namespace = label for a set of components
   - Dots = naming convention (like DNS)
   - No inheritance or scope rules

5. **Proven Pattern**
   - DNS doesn't have hierarchy (example.com vs subdomain.example.com are both flat records)
   - Python packages use dots but are flat in many systems
   - Reverse domain notation (com.example.app) is convention, not structure

**It works!** The spec is smart here. 😊

---

**Next Action:** Fix AddNamespaceDialog to allow dots! 🔧

