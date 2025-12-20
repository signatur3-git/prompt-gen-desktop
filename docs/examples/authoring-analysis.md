# Authoring Analysis: Package Design for Text-to-Image Prompts

**Created:** 2025-12-16  
**Purpose:** Reverse-engineer package designs from real-world prompts  
**Status:** IN PROGRESS - M1 Active Analysis

---

## Progress Tracker

| Prompt | Analyzed | Rules | Decisions | Verdict | Key Insights |
|--------|----------|-------|-----------|---------|--------------|
| **S1: Basic Object** | ✅ | ✅ | ✅ | Rules win | Trivial case, Rules perfect |
| **S2: Optional Adjective** | ✅ | ✅ | ✅ | Rules + helper | Need `first_selected()` primitive |
| **S3: Plural Form** | ✅ | ✅ | ✅ | Decisions win | Multiple outputs, Decisions excel |
| M1: Atmospheric | ⏳ | - | - | - | - |
| M2: Character | ⏳ | - | - | - | - |
| M3: Multi-object | ⏳ | - | - | - | - |

**Status:** 3/15 analyzed (20%) - Clear hybrid pattern emerging!

---

## S1 Analysis: Basic Object

### Prompt: "A red ball"

**Complexity Level:** Simple

#### Entities Required

- [x] Colors (red, blue, green, etc.)
- [x] Objects (ball, car, house, etc.)
- [x] Article (a/an) - computed, not a datatype

#### Datatype Designs

**Datatype: `colors`**
```yaml
name: colors
values:
  - text: "red"
    tags:
      article: "a"
      phonetic: "consonant"
  - text: "blue"
    tags:
      article: "a"
      phonetic: "consonant"
  - text: "orange"
    tags:
      article: "an"
      phonetic: "vowel"
  - text: "yellow"
    tags:
      article: "a"
      phonetic: "consonant"
  - text: "emerald"
    tags:
      article: "an"
      phonetic: "vowel"
```

**Datatype: `objects`**
```yaml
name: objects
values:
  - text: "ball"
    tags:
      article: "a"
      phonetic: "consonant"
      plural: "balls"
  - text: "apple"
    tags:
      article: "an"
      phonetic: "vowel"
      plural: "apples"
  - text: "house"
    tags:
      article: "a"
      phonetic: "consonant"
      plural: "houses"
  - text: "umbrella"
    tags:
      article: "an"
      phonetic: "vowel"
      plural: "umbrellas"
```

#### Template Structure

```yaml
name: basic_object
template: "{article} {color} {object}"
references:
  color: colors
  object: objects
  article: computed  # Determined in enrichment phase
```

#### Coordination Requirements

**What needs to coordinate:**
1. Article selection - must match first actual word (color in this case)

**Dependencies:**
- Article depends on color's phonetic properties
- Color is always selected (not optional)
- Object is always selected (not optional)

**Key Insight:** Since color is always present and comes first, article should match color's article tag.

#### Solution A: Simple Rules

```yaml
rules:
  - name: compute_article
    phase: enrichment
    logic:
      - set: article
        from: ref:color.tags.article
        scope: prompt
```

**Explanation:**
- Since color is guaranteed to be first word, just use its article tag
- No need for complex logic or conditionals
- Very straightforward

**Pros:**
- ✅ Extremely simple (one line!)
- ✅ Easy to read and understand
- ✅ Feels declarative
- ✅ Non-programmer could understand this
- ✅ No helper functions needed

**Cons:**
- ❌ None for this simple case!

**Complexity Score:** 1/5 (trivial)

**Template vs Script Feel:**
- ✅ Feels declarative
- ✅ Non-programmer could understand
- ✅ More config than code

#### Solution B: Decisions Framework

```json
{
  "namespace": "example",
  "name": "select_article_for_word",
  "inputs": {
    "word": "object"
  },
  "outputs": {
    "article": "string"
  },
  "processor": {
    "type": "expression",
    "formula": "word.tags.article"
  }
}
```

**Template usage:**
```yaml
name: basic_object
template: "{article} {color} {object}"
references:
  color: colors
  object: objects
  article: decision.call(example:select_article_for_word, word=ref:color)
```

**Explanation:**
- Create a reusable Decision for article selection
- Pass the color to it
- Decision extracts the article tag
- Returns the article

**Pros:**
- ✅ Reusable across packages
- ✅ Clear inputs/outputs
- ✅ Could be in a shared library (featured.common)

**Cons:**
- ❌ Overkill for this simple case
- ❌ More ceremony (JSON definition)
- ❌ Harder to understand for beginners
- ❌ Reference syntax in template is more complex

**Complexity Score:** 3/5 (simple logic, but framework overhead)

**Template vs Script Feel:**
- ⚠️ Feels more "framework-y"
- ❌ Requires understanding Decisions concept
- ⚠️ Borderline between config and code

#### Comparison

| Aspect | Rules | Decisions | Winner |
|--------|-------|-----------|--------|
| Simplicity | One line! | JSON + template ref | **Rules** |
| Readability | Crystal clear | Need to understand framework | **Rules** |
| Maintainability | Inline, simple | Separate definition | Tie |
| Extensibility | Limited to this case | Reusable | **Decisions** |
| Feels like templates? | Yes! | Less so | **Rules** |
| Learning curve | Minimal | Need to learn framework | **Rules** |

**Verdict:** For this simple case, **Rules win easily**.

The Decisions framework shows potential for reusability (could share `select_article_for_word` across packages), but the overhead isn't justified for such a trivial case.

**Template vs Script Feel:**
- **Rules:** ✅ Feels like templates/config
- **Decisions:** ⚠️ Feels like a framework

**Key Insight:** For simple cases where you just need to extract a tag value, Rules are perfect.

---

## S2 Analysis: Optional Adjective

### Prompt: "An elegant swan" OR "A swan"

**Complexity Level:** Simple (but introduces coordination challenge!)

#### Entities Required

- [x] Adjectives (elegant, graceful, beautiful)
- [x] Animals (swan, eagle, deer)
- [x] Article (a/an) - computed

#### Datatype Designs

**Datatype: `adjectives`**
```yaml
name: adjectives
values:
  - text: "elegant"
    tags:
      article: "an"
      phonetic: "vowel"
      tone: "refined"
  - text: "graceful"
    tags:
      article: "a"
      phonetic: "consonant"
      tone: "refined"
  - text: "beautiful"
    tags:
      article: "a"
      phonetic: "consonant"
      tone: "aesthetic"
  - text: "majestic"
    tags:
      article: "a"
      phonetic: "consonant"
      tone: "powerful"
  - text: "ancient"
    tags:
      article: "an"
      phonetic: "vowel"
      tone: "timeless"
```

**Datatype: `animals`**
```yaml
name: animals
values:
  - text: "swan"
    tags:
      article: "a"
      phonetic: "consonant"
      habitat: "water"
      plural: "swans"
  - text: "eagle"
    tags:
      article: "an"
      phonetic: "vowel"
      habitat: "air"
      plural: "eagles"
  - text: "owl"
    tags:
      article: "an"
      phonetic: "vowel"
      habitat: "air"
      plural: "owls"
  - text: "deer"
    tags:
      article: "a"
      phonetic: "consonant"
      habitat: "forest"
      plural: "deer"
```

#### Template Structure

```yaml
name: optional_adjective_animal
template: "{article} {adjective} {animal}"
references:
  adjective: adjectives?min=0,max=1
  animal: animals
  article: computed
```

#### Coordination Requirements

**What needs to coordinate:**
1. Article selection - must match whichever word comes first
   - If adjective selected (50% chance) → use adjective.tags.article
   - If adjective NOT selected → use animal.tags.article

**Dependencies:**
- Article depends on knowing whether adjective was selected
- Can't decide article at template start
- Classic "deferred decision" problem

**Challenge:** Template begins with `{article}` but we don't know which word is first until AFTER we've selected the adjective (which might be null).

#### Solution A: Simple Rules

**Approach 1: Using helper function**
```yaml
rules:
  - name: compute_article
    phase: enrichment
    logic:
      - set: article
        from: first_selected([ref:adjective, ref:animal]).tags.article
        scope: prompt
```

**Helper needed:** `first_selected([ref1, ref2, ...])` returns first non-null reference

**Explanation:**
- Check adjective first
- If it's null (not selected), fall back to animal
- Extract article tag from whichever is first

**Pros:**
- ✅ Very concise (one line!)
- ✅ Reads naturally: "get article from first selected word"
- ✅ Easy to understand the intent
- ✅ Feels declarative

**Cons:**
- ❌ Need to define `first_selected()` helper
- ❓ **Is this an engine primitive or a pattern?** ← Critical question!
- ❌ Assumes refs can be null

**Complexity Score:** 2/5 (simple once you have the helper)

**Approach 2: Explicit conditional**
```yaml
rules:
  - name: compute_article
    phase: enrichment
    logic:
      - set: article
        from: |
          if ref:adjective != null 
            then ref:adjective.tags.article 
            else ref:animal.tags.article
        scope: prompt
```

**Explanation:**
- Explicit if/then/else
- Check if adjective exists
- Use its article if yes, animal's if no

**Pros:**
- ✅ No helper functions needed
- ✅ Very explicit logic
- ✅ Self-documenting

**Cons:**
- ❌ More verbose
- ❌ Conditional in expression (adds complexity)
- ⚠️ Starting to feel less "template-like"
- ⚠️ Need to support conditionals in expressions

**Complexity Score:** 3/5

#### Solution B: Decisions Framework

```json
{
  "namespace": "featured.common",
  "name": "select_article_optional",
  "inputs": {
    "primary": "object|null",
    "fallback": "object"
  },
  "outputs": {
    "article": "string"
  },
  "processor": {
    "type": "expression",
    "formula": "primary != null ? primary.tags.article : fallback.tags.article"
  }
}
```

**Template usage:**
```yaml
name: optional_adjective_animal
template: "{article} {adjective} {animal}"
references:
  adjective: adjectives?min=0,max=1
  animal: animals
  article: decision.call(featured.common:select_article_optional, 
                         primary=ref:adjective, 
                         fallback=ref:animal)
```

**Explanation:**
- Generic Decision for "pick article from optional element"
- Takes primary (might be null) and fallback (guaranteed)
- Returns appropriate article
- Reusable pattern!

**Pros:**
- ✅ Very reusable across packages
- ✅ Could be in featured.common library
- ✅ Clear naming (select_article_optional)
- ✅ Explicit inputs/outputs
- ✅ Logic encapsulated and testable
- ✅ Other packages can use this without reimplementing

**Cons:**
- ❌ More ceremony for this case
- ❌ Template reference syntax is complex
- ❌ Need to understand Decisions framework
- ⚠️ Feels less like templates

**Complexity Score:** 4/5 (simple logic, but framework overhead)

#### Comparison

| Aspect | Rules (Helper) | Rules (Conditional) | Decisions | Winner |
|--------|----------------|---------------------|-----------|--------|
| Simplicity | Very concise | Verbose | JSON definition | **Rules (Helper)** |
| Readability | Natural | Explicit | Need framework knowledge | **Rules (Helper)** |
| Maintainability | Depends on helper | Self-contained | Separate, testable | **Decisions** |
| Reusability | None | None | High! | **Decisions** |
| Feels like templates? | Yes | Getting complex | No | **Rules (Helper)** |
| No dependencies | No (needs helper) | Yes | No (needs framework) | **Rules (Conditional)** |

**Verdict:** **Depends on what we provide as engine primitives!**

- If we provide `first_selected()` as an **engine primitive**, then **Rules (Helper)** wins
- If we want maximum reusability, **Decisions** shows its value
- **Rules (Conditional)** is middle ground but verbose and feels less template-like

**Template vs Script Feel:**
- **Rules (Helper):** ✅ Template-like IF helper exists
- **Rules (Conditional):** ⚠️ Starting to feel programmatic
- **Decisions:** ❌ Feels like a framework/library pattern

**Key Insights:**

1. **Critical Question Emerges:** Should `first_selected()` be an **engine primitive**?
   - **YES:** Makes Rules very powerful for common cases
   - **NO:** Forces either verbose conditionals or Decisions framework

2. **Decisions Framework Value:** Shows real value for **reusable patterns**
   - `select_article_optional` could be used in hundreds of packages
   - Worth the framework overhead if shared widely
   - Package authors don't reinvent the wheel

3. **This is the crossroads:** S1 was trivial, S2 reveals the design tension
   - Simple extraction (S1) → Rules perfect
   - Optional element coordination (S2) → Need either helpers OR framework

**Recommendation Emerging:**
- Provide `first_selected()` (and similar helpers) as **engine primitives**
- Use Rules for common coordination patterns
- Reserve Decisions for:
  - Complex/reusable logic
  - Shared library patterns (featured.common)
  - External integration (LLM calls, APIs)
- **Hybrid approach emerging!**

---

## Patterns Discovered So Far

### Pattern 1: Direct Tag Extraction
- **Seen in:** S1
- **Challenge:** Get a tag value from a selected reference
- **Best solved with:** Rules - `ref:name.tags.tagname`
- **Engine primitive needed:** None (just reference syntax)

### Pattern 2: Optional Element Coordination
- **Seen in:** S2
- **Challenge:** Pick value from first non-null reference
- **Best solved with:** Rules IF `first_selected()` exists
- **Engine primitive needed:** `first_selected([ref1, ref2, ...])`
- **Alternative:** Decisions framework for reusable patterns

---

## Engine Primitives Discovered

### Must Have
- [x] `ref:name` - Reference a selected value
- [x] `ref:name.tags.tagname` - Extract tag value
- [ ] `ref:name != null` - Check if reference exists (for conditionals)

### Should Have (Makes Rules powerful)
- [ ] `first_selected([ref1, ref2, ...])` - Return first non-null reference
  - Used in: S2 article coordination
  - Alternative: Conditional expressions or Decisions
  - Impact: Makes common pattern trivial

### Conditional Support
- [ ] `if condition then value1 else value2` - Ternary conditional
  - Used in: S2 (alternative approach)
  - Makes Rules more flexible
  - Risk: Could become too script-like

---

## Next Prompts to Analyze

### Up Next: S3 - Plural Form
**Prompt:** "Three cats sleeping" / "A cat sleeping"

**Expected Challenges:**
- Count-based coordination (1 vs multiple)
- Pluralization of nouns
- Verb agreement (sleeps vs sleep)
- Article vs number word ("a cat" vs "three cats")

**Will test:**
- Count access in Rules
- Multiple coordinated changes
- Whether Rules can handle it or need Decisions

### Then: M1-M3 (Medium Complexity)
Will reveal if patterns scale to more complex scenarios.

---

## Interim Recommendations (After 2 Prompts)

### Emerging Hybrid Approach

**Use Rules when:**
- ✅ Simple tag extraction
- ✅ Straightforward coordination
- ✅ Common patterns with helper functions
- ✅ Package-specific logic

**Use Decisions when:**
- ✅ Reusable patterns across packages
- ✅ Complex logic that benefits from encapsulation
- ✅ External integrations (LLMs, APIs)
- ✅ Shared library functions (featured.common)

**Provide as Engine Primitives:**
- `first_selected()` - Critical for optional elements
- Maybe conditional expressions (if/then/else)
- Tag extraction (ref:name.tags.tagname)
- Null checking (ref:name != null)

**Status:** Need more data! S3 will test count-based coordination.

---

## S3 Analysis: Plural Form

### Prompt: "Three cats sleeping" OR "A cat sleeping"

**Complexity Level:** Simple (introduces count-based coordination)

#### Entities Required

- [x] Animals (with singular/plural forms)
- [x] Verbs/Activities (with singular/plural agreement)
- [x] Count (1-5, determines everything)
- [x] Article or number word (computed based on count)

#### Datatype Designs

**Datatype: `animals`**
```yaml
name: animals
values:
  - text: "cat"
    tags:
      article: "a"
      plural: "cats"
      type: "mammal"
  - text: "dog"
    tags:
      article: "a"
      plural: "dogs"
      type: "mammal"
  - text: "bird"
    tags:
      article: "a"
      plural: "birds"
      type: "avian"
  - text: "fish"
    tags:
      article: "a"
      plural: "fish"      # Irregular!
      type: "aquatic"
  - text: "sheep"
    tags:
      article: "a"
      plural: "sheep"     # Irregular!
      type: "mammal"
```

**Datatype: `activities`**
```yaml
name: activities
values:
  - text: "sleeping"
    tags:
      singular: "sleeps"
      plural: "sleep"
      continuous: "sleeping"  # Gerund form (same for all)
  - text: "running"
    tags:
      singular: "runs"
      plural: "run"
      continuous: "running"
  - text: "eating"
    tags:
      singular: "eats"
      plural: "eat"
      continuous: "eating"
```

**Datatype: `number_words`**
```yaml
name: number_words
values:
  - text: "one"
    tags:
      number: 1
  - text: "two"
    tags:
      number: 2
  - text: "three"
    tags:
      number: 3
  - text: "four"
    tags:
      number: 4
  - text: "five"
    tags:
      number: 5
```

#### Template Structure

**Option A: Using count directly**
```yaml
name: animals_with_activity
template: "{article_or_count} {animal} {activity}"
references:
  animal: animals?count=random(1,5)   # Store count
  activity: activities
  article_or_count: computed
```

**Option B: Using number_words datatype**
```yaml
name: animals_with_activity  
template: "{article_or_count} {animal} {activity}"
references:
  count_word: number_words            # "three", "one", etc.
  animal: animals?count=ref:count_word.tags.number
  activity: activities
  article_or_count: computed
```

#### Coordination Requirements

**What needs to coordinate:**
1. **Count determines everything:**
   - If count == 1: "a cat sleeps" (article + singular noun + singular verb)
   - If count > 1: "three cats sleep" (number + plural noun + plural verb)

**Dependencies:**
- Animal form (singular/plural) depends on count
- Verb form (singular/plural) depends on count
- Article vs number word depends on count

**Challenge:** Multiple coordinated changes based on a single variable (count).

#### Solution A: Simple Rules

```yaml
rules:
  - name: setup_count
    phase: enrichment
    logic:
      - set: count
        from: random.int(1, 5)
        scope: prompt
      
  - name: compute_forms
    phase: enrichment
    logic:
      # Article or number word
      - set: article_or_count
        from: |
          if context.get('count') == 1
            then ref:animal.tags.article
            else string(context.get('count'))
        scope: prompt
      
      # Animal form (singular or plural)
      - set: animal
        from: |
          if context.get('count') == 1
            then ref:animal.text
            else ref:animal.tags.plural
        scope: prompt
      
      # Verb form
      - set: activity
        from: |
          if context.get('count') == 1
            then ref:activity.tags.singular
            else ref:activity.tags.plural
        scope: prompt
```

**Explanation:**
- Generate random count (1-5)
- Use conditionals to select appropriate forms
- Three separate rules, all checking the same count

**Pros:**
- ✅ Logic is explicit
- ✅ Each transformation clear
- ✅ No external dependencies

**Cons:**
- ❌ Very verbose (3 conditional expressions)
- ❌ Repetitive (checking count three times)
- ❌ Feels programmatic, not template-like
- ❌ Would get worse with more coordination needs

**Complexity Score:** 4/5 (logic is simple but verbose)

**Alternative with helpers:**
```yaml
rules:
  - name: setup_count
    phase: enrichment
    logic:
      - set: count
        from: random.int(1, 5)
        scope: prompt
      
  - name: compute_forms
    phase: enrichment
    logic:
      - set: article_or_count
        from: pluralize_article(context.get('count'), ref:animal.tags.article)
        scope: prompt
      
      - set: animal
        from: pluralize(ref:animal.text, ref:animal.tags.plural, context.get('count'))
        scope: prompt
      
      - set: activity
        from: conjugate(ref:activity, context.get('count'))
        scope: prompt
```

**Helpers needed:**
- `pluralize_article(count, article)` - Returns article or number as string
- `pluralize(singular, plural, count)` - Returns appropriate form
- `conjugate(verb, count)` - Returns appropriate verb form

**Pros:**
- ✅ Much more concise
- ✅ Reads like template directives
- ✅ Helpers could be engine primitives

**Cons:**
- ❌ Need to define/provide multiple helpers
- ❌ Each use case needs different helper
- ⚠️ Getting close to "helper function explosion"

**Complexity Score:** 3/5 (cleaner but many helpers)

#### Solution B: Decisions Framework

```json
{
  "namespace": "featured.common",
  "name": "pluralize_based_on_count",
  "inputs": {
    "count": "integer",
    "item": "object"
  },
  "outputs": {
    "article_or_count": "string",
    "item_form": "string",
    "verb_form": "string"
  },
  "processor": {
    "type": "rule_set",
    "rules": [
      {
        "condition": "count == 1",
        "output": {
          "article_or_count": "item.tags.article",
          "item_form": "item.text",
          "verb_form": "verb.tags.singular"
        }
      },
      {
        "condition": "count > 1",
        "output": {
          "article_or_count": "string(count)",
          "item_form": "item.tags.plural",
          "verb_form": "verb.tags.plural"
        }
      }
    ]
  }
}
```

**Template usage:**
```yaml
name: animals_with_activity
template: "{article_or_count} {animal} {activity}"
references:
  animal_ref: animals
  activity_ref: activities
  
  # Call Decision to handle all coordination
  result: decision.call(featured.common:pluralize_based_on_count,
                       count=random.int(1,5),
                       item=ref:animal_ref,
                       verb=ref:activity_ref)
  
  # Unpack results
  article_or_count: result.article_or_count
  animal: result.item_form
  activity: result.verb_form
```

**Explanation:**
- One Decision handles all count-based coordination
- Returns all three needed values
- Encapsulates the pluralization logic
- Reusable across any count-based scenario

**Pros:**
- ✅ All coordination logic in one place
- ✅ Highly reusable (any noun + verb scenario)
- ✅ Testable independently
- ✅ Could be in featured.common library
- ✅ Handles irregular plurals if tags are set
- ✅ Scales to more complex scenarios

**Cons:**
- ❌ Complex template reference syntax
- ❌ Need to "unpack" results
- ❌ Feels like calling a function, not templates
- ❌ Steeper learning curve

**Complexity Score:** 4/5 (encapsulates complexity but framework overhead)

#### Comparison

| Aspect | Rules (Conditionals) | Rules (Helpers) | Decisions | Winner |
|--------|---------------------|-----------------|-----------|--------|
| Simplicity | Verbose | Concise | Framework setup | **Rules (Helpers)** |
| Readability | Clear but repetitive | Natural | Need framework | **Rules (Helpers)** |
| Maintainability | Hard to change | Depends on helpers | Centralized | **Decisions** |
| Reusability | None | Helper reuse only | High! | **Decisions** |
| Scalability | Gets worse | Helper explosion? | Scales well | **Decisions** |
| Feels like templates? | No (too verbose) | Yes | No (function calls) | **Rules (Helpers)** |

**Verdict:** **This reveals a critical tension!**

- **Rules (Conditionals):** Too verbose, doesn't scale
- **Rules (Helpers):** Better but risk "helper explosion"
- **Decisions:** Best for reusability, but feels less like templates

**Template vs Script Feel:**
- **Rules (Conditionals):** ❌ Feels like programming
- **Rules (Helpers):** ✅ Feels like templates IF helpers exist
- **Decisions:** ⚠️ Feels like calling library functions

**Key Insights:**

1. **Count-based coordination is common** in text-to-image prompts
   - "Three knights with their swords" 
   - "A wizard with his staff"
   - This pattern will appear frequently

2. **Multiple coordinated changes** from one variable (count)
   - Rules struggle without many helpers
   - Decisions excel at "bundle of coordinated outputs"

3. **Helper function explosion risk:**
   - `pluralize()`, `conjugate()`, `pluralize_article()`
   - Each linguistic pattern needs a helper
   - Is this sustainable?

4. **Decisions show value for complex coordination:**
   - One Decision can return multiple outputs
   - All logic centralized
   - Highly reusable

**Recommendation After S3:**

**Hybrid approach is necessary:**

1. **Provide basic engine primitives:**
   - `first_selected()` - For optional elements (S2)
   - `if/then/else` - For simple conditionals
   - Tag extraction - `ref:name.tags.tagname`
   - Maybe `pluralize()` if it's truly universal

2. **Use Rules for simple cases:**
   - S1: Direct tag extraction
   - S2: Optional elements with `first_selected()`

3. **Use Decisions for complex coordination:**
   - S3: Multiple outputs based on count
   - Reusable patterns (featured.common library)
   - When you need to return multiple coordinated values

4. **Don't try to do everything with Rules:**
   - Leads to verbose conditionals OR
   - Leads to helper explosion

**Pattern Emerging:**
- **Simple extraction:** Rules
- **Simple coordination:** Rules + basic helpers
- **Complex coordination:** Decisions framework
- **Reusable patterns:** Decisions in featured.common

---

## Patterns Discovered (Updated After S3)

### Pattern 1: Direct Tag Extraction
- **Seen in:** S1
- **Challenge:** Get a tag value from a selected reference
- **Best solved with:** Rules - `ref:name.tags.tagname`
- **Engine primitive needed:** None (just reference syntax)

### Pattern 2: Optional Element Coordination
- **Seen in:** S2
- **Challenge:** Pick value from first non-null reference
- **Best solved with:** Rules + `first_selected()`
- **Engine primitive needed:** `first_selected([ref1, ref2, ...])`
- **Alternative:** Decisions for reusable version

### Pattern 3: Count-Based Pluralization ⭐ NEW
- **Seen in:** S3
- **Challenge:** Multiple coordinated changes based on count
  - Article/number word
  - Singular/plural noun
  - Singular/plural verb
- **Best solved with:** **Decisions Framework**
- **Why:** Multiple outputs, complex logic, highly reusable
- **Rules struggle:** Would need many conditionals or many helpers

---

## Engine Primitives Discovered (Updated)

### Must Have
- [x] `ref:name` - Reference a selected value
- [x] `ref:name.tags.tagname` - Extract tag value
- [x] `ref:name != null` - Check if reference exists
- [x] `context.get(key)` - Retrieve from context
- [x] `context.set(key, value)` - Store in context
- [x] `random.int(min, max)` - Random integer (for counts)

### Should Have (Makes Rules Usable)
- [ ] `first_selected([ref1, ref2, ...])` - First non-null reference (S2)
- [ ] `if condition then value1 else value2` - Conditional expressions (S2, S3)
- [ ] `string(number)` - Convert number to string (S3)

### Nice to Have (But maybe not?)
- [ ] `pluralize(singular, plural, count)` - Pluralization helper
- [ ] `conjugate(verb, count)` - Verb conjugation
- [ ] Risk: Too many domain-specific helpers

---

## Updated Recommendations (After 3 Simple Prompts)

### Use Rules When:
- ✅ Direct tag extraction (S1)
- ✅ Simple optional element coordination (S2 with helper)
- ✅ Single output computation
- ✅ Package-specific, one-off logic

### Use Decisions When:
- ✅ **Multiple coordinated outputs** (S3) ⭐ Key insight!
- ✅ Complex reusable patterns
- ✅ Cross-package sharing (featured.common)
- ✅ Logic too complex for simple conditionals
- ✅ External integrations (LLMs, APIs)

### Provide as Engine Primitives:
- ✅ `first_selected()` - Critical for S2 pattern
- ✅ Conditional expressions - Needed for flexibility
- ✅ Basic type conversions
- ❌ NOT domain-specific helpers (pluralize, etc.) - Use Decisions for these

### Emerging Architecture:

```
Simple Cases (S1):
  Rules → Direct tag extraction

Medium Cases (S2):
  Rules + Basic Engine Primitives (first_selected)
  
Complex Cases (S3):
  Decisions Framework (multiple outputs, reusable)
  
Featured.Common Library:
  - select_article_optional (S2)
  - pluralize_based_on_count (S3)
  - More patterns as discovered
```

---

## Next: Medium Complexity (M1-M3)

**Up Next: M1 - Atmospheric Scene**
```
"An elegant swan swimming in a misty lake at dawn"
```

**Expected to test:**
- Multiple optional adjectives
- Thematic consistency (mood/tone)
- Tag-based filtering
- Whether Decisions scale to multi-element coordination

**Will reveal:**
- If Rules + primitives can handle mid-complexity
- Where Decisions become necessary
- How reusable patterns compose

---

## M1 Analysis: Atmospheric Scene

### Prompt: "An elegant swan swimming in a misty lake at dawn"

**Complexity Level:** Medium

#### Entities Required

- [x] Subjects (swan, deer, eagle)
- [x] Subject adjectives (elegant, graceful, majestic)
- [x] Activities (swimming, gliding, soaring)
- [x] Atmospheric adjectives (misty, foggy, hazy)
- [x] Locations (lake, forest, mountain)
- [x] Times of day (dawn, dusk, midnight)
- [x] Article (computed)

#### Datatype Designs

**Datatype: `subjects`**
```yaml
name: subjects
values:
  - text: "swan"
    tags:
      article: "a"
      habitat: "water"
      can_swim: true
      can_fly: true
      tone: "elegant"
  - text: "deer"
    tags:
      article: "a"
      habitat: "forest"
      can_run: true
      tone: "graceful"
  - text: "eagle"
    tags:
      article: "an"
      habitat: "air"
      can_fly: true
      can_soar: true
      tone: "majestic"
  - text: "fox"
    tags:
      article: "a"
      habitat: "forest"
      can_run: true
      tone: "cunning"
```

**Datatype: `subject_adjectives`**
```yaml
name: subject_adjectives
values:
  - text: "elegant"
    tags:
      article: "an"
      tone: "refined"
      mood: "peaceful"
  - text: "graceful"
    tags:
      article: "a"
      tone: "refined"
      mood: "peaceful"
  - text: "majestic"
    tags:
      article: "a"
      tone: "powerful"
      mood: "awe-inspiring"
  - text: "wild"
    tags:
      article: "a"
      tone: "untamed"
      mood: "energetic"
```

**Datatype: `activities`**
```yaml
name: activities
values:
  - text: "swimming"
    tags:
      requires_water: true
      requires_can_swim: true
      mood: "peaceful"
  - text: "gliding"
    tags:
      requires_can_fly: true
      mood: "peaceful"
  - text: "soaring"
    tags:
      requires_can_soar: true
      mood: "powerful"
  - text: "running"
    tags:
      requires_can_run: true
      mood: "energetic"
```

**Datatype: `atmospheric_adjectives`**
```yaml
name: atmospheric_adjectives
values:
  - text: "misty"
    tags:
      article: "a"
      time_compat: ["dawn", "dusk"]
      mood: "mysterious"
  - text: "foggy"
    tags:
      article: "a"
      time_compat: ["dawn", "morning"]
      mood: "mysterious"
  - text: "sun-drenched"
    tags:
      article: "a"
      time_compat: ["noon", "afternoon"]
      mood: "bright"
  - text: "moonlit"
    tags:
      article: "a"
      time_compat: ["night", "midnight"]
      mood: "mysterious"
```

**Datatype: `locations`**
```yaml
name: locations
values:
  - text: "lake"
    tags:
      article: "a"
      habitat_type: "water"
  - text: "forest"
    tags:
      article: "a"
      habitat_type: "forest"
  - text: "mountain"
    tags:
      article: "a"
      habitat_type: "air"
  - text: "meadow"
    tags:
      article: "a"
      habitat_type: "forest"
```

**Datatype: `times`**
```yaml
name: times
values:
  - text: "dawn"
    tags:
      mood: "peaceful"
  - text: "dusk"
    tags:
      mood: "peaceful"
  - text: "midnight"
    tags:
      mood: "mysterious"
  - text: "noon"
    tags:
      mood: "bright"
```

#### Template Structure

```yaml
name: atmospheric_scene
template: "{article} {subject_adj} {subject} {activity} in {atmos_adj} {location} at {time}"
references:
  subject_adj: subject_adjectives?min=0,max=1
  subject: subjects
  activity: activities
  atmos_adj: atmospheric_adjectives
  location: locations
  time: times
  article: computed
```

#### Coordination Requirements

**What needs to coordinate:**

1. **Article selection** - First non-null word (subject_adj OR subject)
2. **Activity compatibility** - Must match subject's capabilities
   - Swan can swim → "swimming" OK
   - Deer can't swim → "swimming" NOT OK
3. **Thematic consistency** - Tone/mood should align
   - "elegant" + "peaceful" activity = good
   - "wild" + "peaceful" activity = mismatch
4. **Atmospheric coherence** - Atmospheric adj compatible with time
   - "misty" + "dawn" = good
   - "sun-drenched" + "midnight" = impossible
5. **Habitat matching** - Location matches subject's habitat
   - Swan + lake = good
   - Eagle + lake = questionable

**Dependencies:**
- Article → first word (like S2)
- Activity → subject's capability tags
- Atmospheric adjective → time compatibility
- Optional: mood consistency across elements

#### Solution A: Simple Rules

**Approach: Tag filtering + validation**

```yaml
rules:
  # Article (same as S2)
  - name: compute_article
    phase: enrichment
    logic:
      - set: article
        from: first_selected([ref:subject_adj, ref:subject]).tags.article
        scope: prompt

  # Validate activity matches subject
  - name: validate_activity
    phase: enrichment
    logic:
      - set: valid_activity
        from: |
          activity_reqs = ref:activity.tags
          subject_caps = ref:subject.tags
          
          # Check each requirement
          if activity_reqs.requires_can_swim and !subject_caps.can_swim
            then false
          else if activity_reqs.requires_can_fly and !subject_caps.can_fly
            then false
          else if activity_reqs.requires_can_run and !subject_caps.can_run
            then false
          else
            true
        scope: prompt
      
      # Re-select if invalid (but how?)
      # This is a problem! Can't easily re-select in Rules

  # Validate atmospheric/time compatibility
  - name: validate_atmosphere
    phase: enrichment  
    logic:
      - set: valid_atmosphere
        from: ref:time.text in ref:atmos_adj.tags.time_compat
        scope: prompt
```

**Problems with Rules:**

1. **Can't easily re-select invalid combinations**
   - Can detect incompatibility
   - But can't trigger re-selection
   - Would need to fail and retry entire prompt?

2. **Tag filtering needs to happen BEFORE selection**
   - Want to filter activities by subject capabilities
   - But Rules run AFTER selection (enrichment phase)
   - Need selection-time filtering!

3. **Multiple validation rules**
   - Activity vs subject
   - Atmosphere vs time
   - Optional: mood consistency
   - Gets verbose quickly

**Pros:**
- ✅ Article computation same as S2 (works)
- ✅ Validation logic is explicit

**Cons:**
- ❌ Can't re-select on validation failure
- ❌ Need filtering DURING selection, not after
- ❌ Multiple complex conditionals
- ❌ Doesn't feel template-like anymore

**Complexity Score:** 5/5 (too complex, doesn't solve the problem)

**Alternative: Tag Filtering in Template**

```yaml
name: atmospheric_scene
template: "{article} {subject_adj} {subject} {activity} in {atmos_adj} {location} at {time}"
references:
  subject_adj: subject_adjectives?min=0,max=1
  subject: subjects
  
  # Filter activities based on subject's capabilities
  activity: activities#requires_can_swim:true
    if ref:subject.tags.can_swim
    else activities#requires_can_fly:true
    if ref:subject.tags.can_fly
    else activities#requires_can_run:true
  
  # Filter atmospheric adjectives by time
  atmos_adj: atmospheric_adjectives#{time_compat contains ref:time.text}
  
  location: locations#{habitat_type == ref:subject.tags.habitat}
  time: times
  article: computed
```

**This requires:**
- Tag filtering syntax: `datatype#tag:value`
- Dynamic filtering: `datatype#{expression}`
- Conditional reference selection

**Pros:**
- ✅ Filtering at selection time (correct!)
- ✅ Ensures compatibility automatically
- ✅ More declarative

**Cons:**
- ❌ Very complex template syntax
- ❌ Requires sophisticated filtering engine
- ❌ Conditionals in template (not enrichment)
- ⚠️ Is this still "template-like"?

**Complexity Score:** 4/5 (cleaner but complex syntax)

#### Solution B: Decisions Framework

**Approach: Decisions handle validation and selection**

```json
{
  "namespace": "featured.common",
  "name": "select_compatible_activity",
  "inputs": {
    "subject": "object",
    "activities": "list"
  },
  "outputs": {
    "activity": "object"
  },
  "processor": {
    "type": "script",
    "language": "javascript",
    "code": `
      // Filter activities by subject capabilities
      const compatible = inputs.activities.filter(act => {
        if (act.tags.requires_can_swim && !inputs.subject.tags.can_swim) return false;
        if (act.tags.requires_can_fly && !inputs.subject.tags.can_fly) return false;
        if (act.tags.requires_can_run && !inputs.subject.tags.can_run) return false;
        return true;
      });
      
      // Select random from compatible
      return {
        activity: compatible[Math.floor(Math.random() * compatible.length)]
      };
    `
  }
}
```

```json
{
  "namespace": "featured.common",
  "name": "select_compatible_atmosphere",
  "inputs": {
    "time": "object",
    "atmospheres": "list"
  },
  "outputs": {
    "atmosphere": "object"
  },
  "processor": {
    "type": "expression",
    "formula": "atmospheres.filter(a => a.tags.time_compat.includes(time.text)).random()"
  }
}
```

**Template usage:**
```yaml
name: atmospheric_scene
template: "{article} {subject_adj} {subject} {activity} in {atmos_adj} {location} at {time}"
references:
  subject_adj: subject_adjectives?min=0,max=1
  subject: subjects
  time: times
  
  # Use Decisions for filtered selection
  activity: decision.call(featured.common:select_compatible_activity,
                         subject=ref:subject,
                         activities=datatype:activities)
  
  atmos_adj: decision.call(featured.common:select_compatible_atmosphere,
                          time=ref:time,
                          atmospheres=datatype:atmospheric_adjectives)
  
  location: locations#{habitat_type == ref:subject.tags.habitat}
  
  article: decision.call(featured.common:select_article_optional,
                         primary=ref:subject_adj,
                         fallback=ref:subject)
```

**Explanation:**
- Decisions handle filtered selection
- Can access full datatype list and filter programmatically
- Returns compatible random selection
- Reusable across packages

**Pros:**
- ✅ Solves the selection-time filtering problem
- ✅ Logic encapsulated and testable
- ✅ Highly reusable (any subject+activity scenario)
- ✅ Can handle complex filtering logic
- ✅ Ensures physical plausibility automatically

**Cons:**
- ❌ Need to pass full datatype lists to Decisions
- ❌ Template syntax getting complex
- ⚠️ Requires script processor (more powerful than expressions)

**Complexity Score:** 4/5 (complex but handles the problem)

#### Comparison

| Aspect | Rules (Validation) | Rules (Tag Filter) | Decisions | Winner |
|--------|-------------------|-------------------|-----------|--------|
| Solves problem | No (can't re-select) | Maybe (complex syntax) | Yes | **Decisions** |
| Simplicity | Complex validation | Complex syntax | Framework + script | **Rules (Tag Filter)** |
| Reusability | None | None | High | **Decisions** |
| Template feel | No (too verbose) | Questionable | No (function calls) | **Rules (Tag Filter)** |
| Correctness | Can detect, can't fix | Works if syntax exists | Works | **Decisions** |

**Verdict:** **This scenario reveals a critical gap!**

**Neither approach is fully satisfactory:**

1. **Rules struggle with selection-time filtering**
   - Can validate after selection
   - Can't easily re-select or filter before selection
   - Need complex conditional references

2. **Tag filtering syntax needed**
   - `datatype#tag:value` - Basic filtering
   - `datatype#{expression}` - Dynamic filtering
   - But this adds complexity to template syntax

3. **Decisions can solve it**
   - Can filter datatypes programmatically
   - Returns compatible selection
   - But requires passing full datatype lists

**Key Insights:**

1. **Selection-time filtering is a real need**
   - Physical plausibility (swan swims, not runs)
   - Temporal coherence (misty at dawn, not noon)
   - Thematic consistency (peaceful mood throughout)

2. **Tag filtering should be a core feature**
   - Not just in Rules/Decisions
   - Built into template reference syntax
   - Example: `activities#requires_can_swim:true`

3. **Decisions are necessary for complex filtering**
   - When filter criteria depend on other selections
   - When logic is too complex for tag matching
   - Example: "activity compatible with subject capabilities"

**Recommendation for M1:**

**Add tag filtering to template syntax:**
```yaml
# Basic tag filtering
activity: activities#mood:peaceful

# Dynamic filtering (expression-based)
activity: activities#{tags.mood == ref:subject_adj.tags.mood}

# Compatibility filtering (this is what we need)
activity: activities#{compatible_with(ref:subject)}
```

**For complex compatibility, use Decisions:**
- `select_compatible_activity(subject, activities)`
- `select_compatible_atmosphere(time, atmospheres)`
- Reusable patterns in featured.common

**This is a hybrid solution at a deeper level:**
- Simple tag filtering: Built into syntax
- Complex filtering: Decisions framework

---

## M2 Analysis: Character with Possessions

### Prompt: "A wizard with his ornate staff standing beside an ancient tree"

**Complexity Level:** Medium (gender agreement pattern)

#### Entities Required

- [x] Characters (wizard, warrior, archer)
- [x] Possessions (staff, sword, bow)
- [x] Possession adjectives (ornate, weathered, gleaming)
- [x] Environment objects (tree, rock, fountain)
- [x] Environment adjectives (ancient, moss-covered, crumbling)
- [x] Spatial relations (beside, near, under)
- [x] Poses (standing, kneeling, sitting)
- [x] Articles (multiple: character, possession, environment)
- [x] Possessive pronoun (his/her/their)

#### Datatype Designs

**Datatype: `characters`**
```yaml
name: characters
values:
  - text: "wizard"
    tags:
      article: "a"
      gender: "masculine"
      possessive: "his"
      typical_items: ["staff", "wand", "tome"]
  - text: "warrior"
    tags:
      article: "a"
      gender: "masculine"
      possessive: "his"
      typical_items: ["sword", "shield", "armor"]
  - text: "archer"
    tags:
      article: "an"
      gender: "feminine"
      possessive: "her"
      typical_items: ["bow", "quiver", "arrows"]
  - text: "rogue"
    tags:
      article: "a"
      gender: "neutral"
      possessive: "their"
      typical_items: ["dagger", "lockpicks", "cloak"]
```

**Datatype: `possessions`**
```yaml
name: possessions
values:
  - text: "staff"
    tags:
      article: "a"
      item_type: "weapon"
      owners: ["wizard", "mage"]
  - text: "sword"
    tags:
      article: "a"
      item_type: "weapon"
      owners: ["warrior", "knight"]
  - text: "bow"
    tags:
      article: "a"
      item_type: "weapon"
      owners: ["archer", "ranger"]
  - text: "tome"
    tags:
      article: "a"
      item_type: "book"
      owners: ["wizard", "scholar"]
```

**Datatype: `possession_adjectives`**
```yaml
name: possession_adjectives
values:
  - text: "ornate"
    tags:
      article: "an"
      quality: "decorative"
  - text: "weathered"
    tags:
      article: "a"
      quality: "worn"
  - text: "gleaming"
    tags:
      article: "a"
      quality: "pristine"
  - text: "ancient"
    tags:
      article: "an"
      quality: "old"
```

**Datatype: `environment_objects`**
```yaml
name: environment_objects
values:
  - text: "tree"
    tags:
      article: "a"
      type: "natural"
  - text: "rock"
    tags:
      article: "a"
      type: "natural"
  - text: "fountain"
    tags:
      article: "a"
      type: "constructed"
  - text: "statue"
    tags:
      article: "a"
      type: "constructed"
```

**Datatype: `spatial_relations`**
```yaml
name: spatial_relations
values:
  - text: "beside"
  - text: "near"
  - text: "under"
  - text: "in front of"
```

**Datatype: `poses`**
```yaml
name: poses
values:
  - text: "standing"
  - text: "kneeling"
  - text: "sitting"
  - text: "leaning"
```

#### Template Structure

```yaml
name: character_with_possession
template: "{char_article} {character} with {possessive} {poss_adj} {possession} {pose} {relation} {env_adj} {env_object}"
references:
  character: characters
  possession: possessions
  poss_adj: possession_adjectives?min=0,max=1
  env_adj: environment_adjectives?min=0,max=1
  env_object: environment_objects
  pose: poses
  relation: spatial_relations
  
  # Computed values
  char_article: computed
  possessive: computed
  poss_article: computed  # Not shown in template but needed
  env_article: computed   # Not shown in template but needed
```

**Note:** Template as shown doesn't include all articles. Better version:

```yaml
template: "{char_article} {character} with {possessive} {poss_article} {poss_adj} {possession} {pose} {relation} {env_article} {env_adj} {env_object}"
```

Actually, possessive pronoun implies possession ownership, so no article needed there. Original template is correct!

#### Coordination Requirements

**What needs to coordinate:**

1. **Gender agreement**
   - Character gender → possessive pronoun
   - "wizard" (masculine) → "his"
   - "archer" (feminine) → "her"
   - "rogue" (neutral) → "their"

2. **Multiple articles**
   - Character article: "A wizard" / "An archer"
   - Environment article (with optional adj): "an ancient tree" / "a tree"

3. **Possession compatibility (optional)**
   - Wizard should have staff/wand/tome (not sword)
   - Warrior should have sword/shield (not bow)
   - Could be strict or flexible

4. **Optional adjectives**
   - Possession adjective (min=0)
   - Environment adjective (min=0)
   - Each affects its own article

**Dependencies:**
- Possessive pronoun ← character gender (simple lookup)
- Character article ← character
- Environment article ← first_selected([env_adj, env_object])
- Possession compatibility ← character type (if enforced)

#### Solution A: Simple Rules

```yaml
rules:
  # Character article (trivial)
  - name: compute_char_article
    phase: enrichment
    logic:
      - set: char_article
        from: ref:character.tags.article
        scope: prompt
  
  # Possessive pronoun (trivial - just extract tag)
  - name: compute_possessive
    phase: enrichment
    logic:
      - set: possessive
        from: ref:character.tags.possessive
        scope: prompt
  
  # Environment article (same pattern as S2)
  - name: compute_env_article
    phase: enrichment
    logic:
      - set: env_article
        from: first_selected([ref:env_adj, ref:env_object]).tags.article
        scope: prompt
```

**For possession compatibility (optional enforcement):**
```yaml
  # Validate possession matches character
  - name: validate_possession
    phase: enrichment
    logic:
      - set: possession_valid
        from: ref:character.text in ref:possession.tags.owners
        scope: prompt
      
      # If invalid, what do we do? Same problem as M1!
```

**Pros:**
- ✅ Gender agreement is trivial (just tag extraction)
- ✅ Articles same as S1 and S2 (we know how to do this)
- ✅ Simple, clear logic

**Cons:**
- ❌ Can't enforce possession compatibility (same issue as M1)
- ⚠️ But maybe possession compatibility is optional?

**Complexity Score:** 2/5 (easy if we don't enforce compatibility)

**If enforcing compatibility, use filtering:**
```yaml
template: "{char_article} {character} with {possessive} {poss_article} {poss_adj} {possession} {pose} {relation} {env_article} {env_adj} {env_object}"
references:
  character: characters
  
  # Filter possessions by character compatibility
  possession: possessions#{ref:character.text in tags.owners}
  
  # Rest is simple
  poss_adj: possession_adjectives?min=0,max=1
  env_adj: environment_adjectives?min=0,max=1
  env_object: environment_objects
  pose: poses
  relation: spatial_relations
```

**Complexity Score:** 3/5 (with tag filtering syntax)

#### Solution B: Decisions Framework

```json
{
  "namespace": "featured.common",
  "name": "select_compatible_possession",
  "inputs": {
    "character": "object",
    "possessions": "list"
  },
  "outputs": {
    "possession": "object"
  },
  "processor": {
    "type": "expression",
    "formula": "possessions.filter(p => p.tags.owners.includes(character.text)).random()"
  }
}
```

**Template usage:**
```yaml
references:
  character: characters
  possession: decision.call(featured.common:select_compatible_possession,
                           character=ref:character,
                           possessions=datatype:possessions)
  # ... rest
```

**Pros:**
- ✅ Enforces possession compatibility
- ✅ Reusable across packages

**Cons:**
- ❌ Overkill if compatibility is optional

**Complexity Score:** 3/5

#### Comparison

| Aspect | Rules (No Compat) | Rules (Tag Filter) | Decisions | Winner |
|--------|-------------------|-------------------|-----------|--------|
| Gender agreement | Trivial | Trivial | Trivial | **All** |
| Articles | Easy (S2 pattern) | Easy | Easy | **All** |
| Possession compat | No enforcement | Tag filtering | Full filtering | **Decisions** |
| Simplicity | Very simple | Needs tag syntax | Framework | **Rules (No Compat)** |
| Correctness | Maybe OK | Depends on syntax | Always correct | **Decisions** |

**Verdict:** **Depends on strictness requirements!**

**Key Insights:**

1. **Gender agreement is trivial**
   - Just tag extraction: `ref:character.tags.possessive`
   - Same pattern as S1 article extraction
   - Rules handle perfectly

2. **Multiple articles = multiple S2 patterns**
   - Character article: direct extraction
   - Environment article: `first_selected()` pattern
   - No new complexity

3. **Possession compatibility is the question**
   - **Strict:** Need filtering (tag syntax OR Decisions)
   - **Loose:** Don't enforce, accept any combination
   - Design choice!

4. **This is easier than expected**
   - Feared gender agreement would be complex
   - Actually just tag extraction
   - Main challenge is possession filtering (if wanted)

**Recommendation for M2:**

**Use Rules for core coordination:**
- Gender → possessive: `ref:character.tags.possessive`
- Articles: Same patterns as S1/S2

**Use tag filtering OR Decisions for compatibility:**
- **If strict:** `possessions#{ref:character.text in tags.owners}`
- **If loose:** Just select randomly, let implausible combos happen

**Pattern: This is mostly S1 + S2 combined!**
- Multiple instances of simple patterns
- No new coordination complexity
- Main question is filtering (M1 issue)

---

## M3 Analysis: Multiple Objects with Shared Property

### Prompt: "A red car and a red bicycle parked on a sunny street"

**Complexity Level:** Medium (context reuse pattern)

#### Entities Required

- [x] Colors (red, blue, green)
- [x] Vehicles (car, bicycle, motorcycle)
- [x] Weather adjectives (sunny, rainy, foggy)
- [x] Locations (street, driveway, parking lot)
- [x] Articles (one per vehicle)
- [x] Shared color (selected once, used twice)

#### Datatype Designs

**Datatype: `colors`**
```yaml
name: colors
values:
  - text: "red"
    tags:
      article: "a"
  - text: "blue"
    tags:
      article: "a"
  - text: "green"
    tags:
      article: "a"
  - text: "silver"
    tags:
      article: "a"
```

**Datatype: `vehicles`**
```yaml
name: vehicles
values:
  - text: "car"
    tags:
      article: "a"
      type: "wheeled"
  - text: "bicycle"
    tags:
      article: "a"
      type: "wheeled"
  - text: "motorcycle"
    tags:
      article: "a"
      type: "wheeled"
  - text: "truck"
    tags:
      article: "a"
      type: "wheeled"
```

**Datatype: `weather_adjectives`**
```yaml
name: weather_adjectives
values:
  - text: "sunny"
    tags:
      article: "a"
  - text: "rainy"
    tags:
      article: "a"
  - text: "foggy"
    tags:
      article: "a"
```

**Datatype: `locations`**
```yaml
name: locations
values:
  - text: "street"
    tags:
      article: "a"
  - text: "driveway"
    tags:
      article: "a"
  - text: "parking lot"
    tags:
      article: "a"
```

#### Template Structure

**Challenge:** Need TWO vehicles, SAME color.

**Option A: Two separate references**
```yaml
name: two_vehicles
template: "{article1} {color} {vehicle1} and {article2} {color} {vehicle2} parked on {weather_adj} {location}"
references:
  color: colors                    # Selected ONCE
  vehicle1: vehicles
  vehicle2: vehicles               # Different vehicle
  weather_adj: weather_adjectives
  location: locations
  article1: computed
  article2: computed
```

**Problem:** How do we ensure vehicle1 != vehicle2?
- Need unique constraint
- Or accept duplicates ("a red car and a red car")

**Option B: Use min/max for vehicles**
```yaml
name: two_vehicles
template: "{vehicle_list} parked on {weather_adj} {location}"
references:
  color: colors
  vehicles: vehicles?min=2,max=2&sep=and_separator&unique=true
  weather_adj: weather_adjectives
  location: locations
```

**Custom rendering:** Each vehicle gets "{article} {color} {vehicle}"

**Problem:** How to apply color to each vehicle in the list?
- Need to repeat color for each item
- Standard separator doesn't support this

#### Coordination Requirements

**What needs to coordinate:**

1. **Shared color property**
   - Select color once
   - Apply to both vehicles
   - "a red car and a red bicycle" (not "a red car and a blue bicycle")

2. **Individual articles**
   - Each vehicle gets its own article
   - "A car and a bicycle" (both get "a")
   - Even though color is shared

3. **Unique vehicles (optional)**
   - Don't want "a red car and a red car"
   - Should select two different vehicles

4. **List formatting**
   - Use "and" conjunction
   - Each item: "{article} {color} {vehicle}"

**Dependencies:**
- Color selected once, stored in context
- Each vehicle gets color from context
- Each vehicle needs its own article

#### Solution A: Simple Rules

```yaml
rules:
  # Select shared color
  - name: select_shared_color
    phase: enrichment
    logic:
      - set: shared_color
        from: ref:color.text
        scope: prompt
  
  # Vehicle 1 article
  - name: compute_article1
    phase: enrichment
    logic:
      - set: article1
        from: ref:color.tags.article   # Based on color, not vehicle
        scope: prompt
  
  # Vehicle 2 article (same logic)
  - name: compute_article2
    phase: enrichment
    logic:
      - set: article2
        from: ref:color.tags.article
        scope: prompt
```

**Template with explicit structure:**
```yaml
template: "{article1} {color} {vehicle1} and {article2} {color} {vehicle2} parked on {weather_adj} {location}"
references:
  color: colors
  vehicle1: vehicles
  vehicle2: vehicles    # Could be same as vehicle1
```

**For unique vehicles:**
```yaml
references:
  color: colors
  vehicle1: vehicles
  vehicle2: vehicles#{text != ref:vehicle1.text}  # Tag filtering to exclude vehicle1
```

**Pros:**
- ✅ Very straightforward
- ✅ Color selected once, referenced twice
- ✅ Articles computed simply
- ✅ Explicit template structure

**Cons:**
- ❌ Hardcoded to exactly 2 vehicles
- ❌ Unique constraint needs tag filtering
- ⚠️ Doesn't scale to 3+ vehicles

**Complexity Score:** 2/5 (simple for fixed count)

**Alternative: Using context directly in template**
```yaml
template: "A {color} {vehicle1} and a {color} {vehicle2} parked on {weather_adj} {location}"
references:
  color: colors    # Selected once
  vehicle1: vehicles
  vehicle2: vehicles#{text != ref:vehicle1.text}
  weather_adj: weather_adjectives
  location: locations
```

**Even simpler!** Just reference `{color}` twice in template.

**Pros:**
- ✅ Extremely simple
- ✅ Color naturally reused
- ✅ Articles hardcoded (they're always "a" for these vehicles)

**Cons:**
- ❌ Hardcoded articles in template
- ❌ Won't work if color is "orange" (needs "an")

**Complexity Score:** 1/5 (simplest, but limited)

#### Solution B: Decisions Framework

**For variable count with shared property:**

```json
{
  "namespace": "featured.common",
  "name": "apply_shared_property",
  "inputs": {
    "property": "object",
    "items": "list",
    "count": "integer"
  },
  "outputs": {
    "formatted_list": "string"
  },
  "processor": {
    "type": "script",
    "code": `
      const selected = inputs.items.slice(0, inputs.count);
      const formatted = selected.map(item => {
        const article = inputs.property.tags.article;
        return \`\${article} \${inputs.property.text} \${item.text}\`;
      });
      return {
        formatted_list: formatted.join(' and ')
      };
    `
  }
}
```

**Template usage:**
```yaml
template: "{vehicle_list} parked on {weather_adj} {location}"
references:
  color: colors
  vehicles: decision.call(featured.common:apply_shared_property,
                         property=ref:color,
                         items=datatype:vehicles,
                         count=2)
  weather_adj: weather_adjectives
  location: locations
```

**Pros:**
- ✅ Scales to any count
- ✅ Handles articles correctly
- ✅ Ensures uniqueness
- ✅ Reusable for "three red balls", "two blue houses", etc.

**Cons:**
- ❌ Overkill for fixed simple case
- ⚠️ Hides formatting in script

**Complexity Score:** 3/5

#### Comparison

| Aspect | Rules (Explicit) | Rules (Template Reuse) | Decisions | Winner |
|--------|------------------|----------------------|-----------|--------|
| Simplicity | Simple | Simplest | Framework | **Rules (Template)** |
| Correct articles | Yes | Hardcoded | Yes | **Rules/Decisions** |
| Scalability | Fixed count | Fixed count | Variable | **Decisions** |
| Readability | Clear | Very clear | Hidden logic | **Rules** |

**Verdict:** **Rules win for simple fixed-count cases!**

**Key Insights:**

1. **Context reuse is trivial**
   - Just reference `{color}` multiple times in template
   - No special logic needed
   - Natural template behavior!

2. **Shared property pattern is common**
   - "Two red cars"
   - "Three blue houses"
   - "Five wooden chairs"
   - But can be handled simply for fixed counts

3. **Fixed count vs variable count**
   - **Fixed (2 objects):** Template reuse works great
   - **Variable (2-5 objects):** Need Decisions or complex lists

4. **This is simpler than expected**
   - Feared "apply same property to multiple items" would be hard
   - Actually just reference the same variable multiple times
   - Template language naturally supports this!

**Recommendation for M3:**

**For fixed count (like this example):**
```yaml
template: "A {color} {vehicle1} and a {color} {vehicle2} parked on..."
references:
  color: colors
  vehicle1: vehicles
  vehicle2: vehicles#{text != ref:vehicle1.text}
```

**For variable count:**
Use Decisions to format list with shared property.

**Pattern: Context reuse is a core template feature!**
- No special coordination needed
- Just reference the same value multiple times
- Rules/Decisions not needed for this

---

## Patterns Discovered (Updated After M1-M3)

### Pattern 1: Direct Tag Extraction
- **Seen in:** S1, M2
- **Challenge:** Get a tag value from a selected reference
- **Best solved with:** Rules - `ref:name.tags.tagname`
- **Engine primitive needed:** None (just reference syntax)

### Pattern 2: Optional Element Coordination
- **Seen in:** S2, M2
- **Challenge:** Pick value from first non-null reference
- **Best solved with:** Rules + `first_selected()`
- **Engine primitive needed:** `first_selected([ref1, ref2, ...])`

### Pattern 3: Count-Based Pluralization
- **Seen in:** S3
- **Challenge:** Multiple coordinated changes based on count
- **Best solved with:** Decisions Framework
- **Why:** Multiple outputs, complex logic, highly reusable

### Pattern 4: Selection-Time Filtering ⭐ NEW (Critical!)
- **Seen in:** M1
- **Challenge:** Filter datatypes based on compatibility/plausibility
  - Activity must match subject capabilities
  - Atmosphere must match time of day
- **Best solved with:** Tag filtering syntax OR Decisions
- **Engine feature needed:** `datatype#{filter_expression}`
- **Why critical:** Physical plausibility and thematic coherence

### Pattern 5: Gender Agreement (Simpler than expected!)
- **Seen in:** M2
- **Challenge:** Select pronoun based on gender
- **Best solved with:** Rules - just tag extraction!
- **Pattern:** Same as Pattern 1
- **Insight:** Not a new pattern, just tag lookup

### Pattern 6: Context Reuse (Trivial!)
- **Seen in:** M3
- **Challenge:** Apply same property to multiple items
- **Best solved with:** Template reference reuse
- **No special logic needed:** Just use `{color}` multiple times
- **Insight:** Templates naturally support this!

---

## Analysis Template (Copy for Remaining Prompts)

### Prompt: [PROMPT TEXT]

**Complexity Level:** Simple | Medium | Complex | Extreme

#### Entities Required
- [ ] 
- [ ] 

#### Datatype Designs
```yaml
# Design datatypes here
```

#### Template Structure
```yaml
# Design template here
```

#### Coordination Requirements
- 

#### Solution A: Simple Rules
```yaml
# Rules here
```

**Complexity Score:** ?/5

#### Solution B: Decisions Framework
```json
// Decisions here
```

**Complexity Score:** ?/5

#### Comparison
**Verdict:** [Which is better?]

**Key Insights:** [What did we learn?]

---

**Status:** M1 in progress - S1 and S2 analyzed, S3 next!

