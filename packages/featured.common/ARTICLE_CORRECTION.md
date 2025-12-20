# CRITICAL CORRECTION: Article Selection Design

**Date:** 2025-12-17  
**Issue:** Expanded design had articles BACKWARDS  
**Status:** ⚠️ **CORRECTED**

---

## The Mistake I Made

**WRONG approach (in expanded design):**
```yaml
articles:
  - a (tags: {vowel_follows: false})
  - an (tags: {vowel_follows: true})
```

**Problem:** The article doesn't know what word follows it! This is impossible.

---

## The Correct Approach (From M4 Implementation)

**Articles come FROM words, not TO words:**

```yaml
# Words declare their article need based on phonetic start
colors:
  values:
    - text: red
      tags: {article: a}  # consonant sound
    - text: orange
      tags: {article: an}  # vowel sound
    - text: azure
      tags: {article: an}  # vowel sound

creatures:
  values:
    - text: swan
      tags: {article: a}  # consonant sound
    - text: eagle
      tags: {article: an}  # vowel sound
    - text: owl
      tags: {article: an}  # vowel sound
    - text: unicorn
      tags: {article: a}  # sounds like "yoo-ni-corn" (consonant)
```

---

## How It Works (3-Step Process)

### Step 1: Words Tag Themselves

Each word declares what article it needs based on its **phonetic start** (not spelling):

```yaml
adjectives:
  - text: ancient
    tags: {article: an}  # starts with vowel sound
  - text: brave
    tags: {article: a}   # starts with consonant sound
  - text: honest
    tags: {article: an}  # 'h' is silent, vowel sound
  - text: unique
    tags: {article: a}   # sounds like "yoo-neek" (consonant)
```

### Step 2: Rules Extract Article

Rules extract the article from the selected word:

```yaml
rules:
  - set: article
    from: first_selected([ref:adjective, ref:noun]).tags.article
```

**What this does:**
- If adjective selected → use adjective's article
- If no adjective → use noun's article
- Example: "ancient dragon" → article from "ancient" = "an"
- Example: "dragon" (no adjective) → article from "dragon" = "a"

### Step 3: Template Uses Context

```yaml
template: "{article} {adjective} {noun}"
references:
  article:
    target: context:article  # From rules!
  adjective:
    target: colors
    min: 0
    max: 1
  noun:
    target: creatures
```

---

## Why This Works

### Phonetic Awareness at Source

**Each word knows its own phonetics:**
- "hour" → `{article: an}` (silent 'h')
- "heir" → `{article: an}` (silent 'h')
- "honor" → `{article: an}` (silent 'h')
- "unicorn" → `{article: a}` (sounds like consonant)
- "university" → `{article: a}` (sounds like consonant)

**Pronunciation variants handled:**
- "herb" (US) → `{article: an}` (silent 'h')
- "herb" (UK) → `{article: a}` (pronounced 'h')
- Package author decides based on target audience

### First Contribution Wins

When optional adjectives exist:

```yaml
# With adjective
"ancient" (article: an) + "dragon" (article: a)
→ first_selected finds "ancient"
→ article = "an"
→ Output: "an ancient dragon" ✅

# Without adjective  
null + "dragon" (article: a)
→ first_selected skips null, finds "dragon"
→ article = "a"
→ Output: "a dragon" ✅
```

---

## Examples from Real Implementation

### Example 1: Simple

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
    template: "{article} {creature}"
    references:
      article: {target: context:article}
      creature: {target: test:creatures}
```

**Output:**
- "an owl" ✅
- "a deer" ✅

### Example 2: With Optional Adjective

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
      article: {target: context:article}
      adjective: {target: test:adjectives, min: 0, max: 1}
      creature: {target: test:creatures}
```

**Outputs:**
- With adjective: "an ancient dragon" ✅ (article from "ancient")
- Without adjective: "a dragon" ✅ (article from "dragon")

### Example 3: Edge Cases

```yaml
words:
  - text: hour
    tags: {article: an}  # silent 'h', vowel sound
  - text: honor
    tags: {article: an}  # silent 'h', vowel sound
  - text: house
    tags: {article: a}   # pronounced 'h', consonant sound
  - text: unicorn
    tags: {article: a}   # "yoo" sound, consonant
  - text: umbrella
    tags: {article: an}  # "um" sound, vowel
  - text: one
    tags: {article: a}   # "wun" sound, consonant
  - text: 8-bit
    tags: {article: an}  # "eight" sound, vowel
```

---

## For Expanded Package Design

### Every Word Needs Article Tag

**In common.visual.color:**
```yaml
colors_primary:
  - text: red
    tags: {article: a, warm: true}
  - text: orange
    tags: {article: an, warm: true}
  - text: azure
    tags: {article: an, cool: true}
  - text: emerald
    tags: {article: an, cool: true}
  - text: ivory
    tags: {article: an, warm: true, light: true}
```

**In common.subject.living:**
```yaml
creatures_mythical:
  - text: dragon
    tags: {article: a, can_fly: true}
  - text: eagle
    tags: {article: an, can_fly: true}
  - text: owl
    tags: {article: an, can_fly: true}
  - text: unicorn
    tags: {article: a, magical: true}  # "yoo-ni-corn"
  - text: ogre
    tags: {article: an}  # "oh-ger"
```

**In common.subject.object:**
```yaml
objects_tools:
  - text: sword
    tags: {article: a, weapon: true}
  - text: axe
    tags: {article: an, weapon: true}  # "ax" sound
  - text: umbrella
    tags: {article: an}
  - text: one-handed sword
    tags: {article: a}  # "wun" sound
```

---

## What About the "articles" Datatype?

**Purpose:** Documentation ONLY, not for selection

**Why it exists:**
- Documents what articles are available
- Provides reference for package authors
- Potential edge cases (direct article specification)

**It should be:**
```yaml
articles:
  values:
    - text: a
      tags: {type: indefinite, phonetic_context: consonant_start}
    - text: an
      tags: {type: indefinite, phonetic_context: vowel_start}
    - text: the
      tags: {type: definite, phonetic_context: any}
```

**But it's NOT used in normal templates!**

Articles come from context which comes from word tags.

---

## Summary

**CORRECT:**
1. ✅ Words tag themselves with needed article: `{article: a}` or `{article: an}`
2. ✅ Rules extract article: `set: article from: ref:word.tags.article`
3. ✅ Template uses context: `{article}` → `target: context:article`

**INCORRECT:**
1. ❌ Articles datatype with selection
2. ❌ Articles deciding what follows them
3. ❌ Filtering articles by "vowel_follows" tag

---

## Action Items for Expanded Package

**For ALL datatypes with selectable values:**
1. Add `article` tag to EVERY value
2. Determine phonetically (not by spelling)
3. Consider pronunciation variants (US vs UK)

**Estimated tags to add:**
- ~2,500-3,000 values × 1 article tag each
- = ~2,500-3,000 article tags to define

**This is feasible because:**
- Most are obvious (red=a, orange=an)
- Pattern recognition helps (most words starting with vowels need "an")
- Edge cases are documented (hour, unicorn, etc.)

---

**Status:** Design corrected, ready to implement correctly! ✅

