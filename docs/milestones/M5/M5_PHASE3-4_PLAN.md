# M5 Phase 3+4: Separator Sets & Min/Max Multiplicity - Plan

**Date Started:** 2025-12-17  
**Status:** 🔄 **READY TO START**  
**Goal:** Handle multiple selections with proper formatting

---

## Why Combine Phase 3 & 4?

These features are tightly coupled:
- Min/Max multiplicity creates lists: `{colors?min=2,max=3}` → ["red", "blue", "green"]
- Separator sets format lists: `sep=comma_and` → "red, blue and green"
- Better to implement together than separately

**Combined Time Estimate:** 5-6 hours (vs 7 hours separately)

---

## Phase 3+4 Goals

### 1. Min/Max Multiplicity ✅
- Parse `?min=X,max=Y` in template syntax
- Select multiple values from same datatype
- Support `unique=true` for no duplicates
- Default separator: space `" "`

### 2. Separator Sets ✅
- Implement SeparatorSet data model (already exists in M2!)
- Format lists with primary/secondary separators
- Support `&sep=namespace:separatorset` syntax
- Common patterns: `comma_and`, `semicolon_or`, `newline`

### 3. Integration ✅
- Update template parser to handle parameters
- Update Reference model with min/max/sep/unique fields
- Update selector to return multiple values
- Update renderer to format with separators

---

## Current State

### Already Implemented (M2):
```rust
// In core/models.rs
pub struct SeparatorSet {
    pub name: String,
    pub primary: String,    // ", "
    pub secondary: String,  // " and "
    pub tertiary: String,   // "; "
}

pub struct Reference {
    pub target: String,
    pub filter: Option<String>,
    // Need to add:
    // pub min: Option<usize>,
    // pub max: Option<usize>,
    // pub separator: Option<String>,
    // pub unique: Option<bool>,
}
```

### What Needs Implementation:
1. Template parser: Parse `{ref?min=2,max=4&sep=comma_and&unique=true}`
2. Reference model: Add min/max/sep/unique fields
3. Selector: Return Vec<SelectedValue> instead of single value
4. Renderer: Format multiple values with separator set

---

## Implementation Plan

### Step 1: Update Template Parser ✅
**Goal:** Parse parameter syntax in references

**Current:** `{reference}` or `{reference#{filter}}`  
**Target:** `{reference?min=2,max=3&sep=comma_and&unique=true#{filter}}`

**Approach:**
```rust
// In template_parser.rs
pub struct TemplateToken {
    Reference {
        name: String,
        filter: Option<String>,
        min: Option<usize>,      // NEW
        max: Option<usize>,      // NEW
        separator: Option<String>, // NEW
        unique: Option<bool>,    // NEW
    }
}

// Parse: {name?params&params#filter}
// 1. Split on '#' → ["name?params&params", "filter"]
// 2. Split [0] on '?' → ["name", "params&params"]
// 3. Parse params: "min=2,max=3&sep=comma_and&unique=true"
```

### Step 2: Update Reference Model ✅
**Goal:** Store parameters in Reference

```rust
// In core/models.rs
pub struct Reference {
    pub target: String,
    pub filter: Option<String>,
    pub min: usize,           // Default: 1
    pub max: usize,           // Default: 1
    pub separator: Option<String>,  // Default: None (use space)
    pub unique: bool,         // Default: false
}
```

### Step 3: Implement SeparatorSet Formatter ✅
**Goal:** Format lists with separators

```rust
// In renderer/separator.rs (NEW FILE)
impl SeparatorSet {
    pub fn format(&self, items: &[String]) -> String {
        match items.len() {
            0 => String::new(),
            1 => items[0].clone(),
            2 => format!("{}{}{}", items[0], self.secondary, items[1]),
            _ => {
                // "a, b, c and d"
                let mut result = items[0..items.len()-1].join(&self.primary);
                result.push_str(&self.secondary);
                result.push_str(&items[items.len()-1]);
                result
            }
        }
    }
}
```

### Step 4: Update Selector for Multiple Values ✅
**Goal:** Select N values instead of 1

```rust
// In selector.rs
impl Selector {
    pub fn select_multiple(
        &mut self,
        reference: &str,
        count: usize,
        filter: Option<&str>,
        unique: bool
    ) -> Result<Vec<SelectedValue>> {
        let mut results = Vec::new();
        let mut attempts = 0;
        const MAX_ATTEMPTS: usize = 100;
        
        while results.len() < count && attempts < MAX_ATTEMPTS {
            let value = self.select_with_filter(reference, filter)?;
            
            if unique {
                if !results.iter().any(|v| v.text == value.text) {
                    results.push(value);
                }
            } else {
                results.push(value);
            }
            
            attempts += 1;
        }
        
        Ok(results)
    }
}
```

### Step 5: Update Renderer Phase 1 & 3 ✅
**Goal:** Handle multiple selections and formatting

```rust
// Phase 1: Selection
fn phase_1_selection(...) -> Result<HashMap<String, Vec<SelectedValue>>> {
    // Select min to max values per reference
    let count = rng.gen_range(reference.min..=reference.max);
    let values = selector.select_multiple(
        &reference.target,
        count,
        reference.filter.as_deref(),
        reference.unique
    )?;
    selected.insert(ref_name, values);
}

// Phase 3: Rendering
fn phase_3_rendering(...) -> Result<String> {
    for token in parsed.tokens {
        match token {
            TemplateToken::Reference { name, separator, .. } => {
                let values = selected.get(&name)?;
                
                // Format with separator
                let text = if let Some(sep_ref) = separator {
                    let sep_set = find_separator_set(sep_ref)?;
                    sep_set.format(&values.iter().map(|v| v.text.clone()).collect::<Vec<_>>())
                } else {
                    // Default: space-separated
                    values.iter().map(|v| &v.text).collect::<Vec<_>>().join(" ")
                };
                
                output.push_str(&text);
            }
        }
    }
}
```

---

## Test Package

```yaml
id: test.lists
version: 1.0.0

namespaces:
  test:
    # Datatypes
    datatypes:
      colors:
        values:
          - text: "red"
          - text: "blue"
          - text: "green"
          - text: "yellow"
          - text: "purple"
      
      adjectives:
        values:
          - text: "beautiful"
          - text: "mysterious"
          - text: "ancient"
          - text: "magical"
    
    # Separator sets
    separator_sets:
      comma_and:
        primary: ", "
        secondary: " and "
        tertiary: "; "
      
      comma_or:
        primary: ", "
        secondary: " or "
        tertiary: "; "
    
    # Prompt sections
    prompt_sections:
      # Min/max without separator (space-separated)
      simple_list:
        template: "{colors?min=2,max=4}"
        references:
          colors:
            target: test:colors
        # → "red blue green yellow"
      
      # With separator set
      natural_list:
        template: "{colors?min=2,max=3&sep=comma_and}"
        references:
          colors:
            target: test:colors
        # → "red, blue and green"
      
      # With unique constraint
      unique_colors:
        template: "{colors?min=3,max=3&sep=comma_and&unique=true}"
        references:
          colors:
            target: test:colors
        # → "red, blue and green" (no duplicates)
      
      # Optional items (min=0)
      optional_adjectives:
        template: "A {adjectives?min=0,max=2&sep=comma_and} castle"
        references:
          adjectives:
            target: test:adjectives
        # → "A castle" (0 adjectives)
        # → "A beautiful castle" (1 adjective)
        # → "A beautiful and mysterious castle" (2 adjectives)
      
      # SANITY CHECK: M4 + M5 integration
      # Combines tag filtering (M4) + article computation (M4) + separator sets (M5)
      flying_creatures_with_article:
        template: "{article} {creatures?min=2,max=3&sep=comma_and#{tags.can_fly}}"
        references:
          article:
            target: context:article
          creatures:
            target: test:creatures
    
    # M4: Rules for article computation
    rules:
      - name: compute_article_from_creatures
        phase: enrichment
        logic:
          - set: article
            from: "ref:creatures.tags.article"
            scope: prompt

# Additional datatypes for sanity check
datatypes:
  creatures:
    name: creatures
    values:
      - text: "owl"
        tags:
          can_fly: true
          article: "an"
        weight: 1
      
      - text: "eagle"
        tags:
          can_fly: true
          article: "an"
        weight: 1
      
      - text: "swan"
        tags:
          can_fly: true
          article: "a"
        weight: 1
      
      - text: "bat"
        tags:
          can_fly: true
          article: "a"
        weight: 1
      
      - text: "cat"
        tags:
          can_fly: false
          article: "a"
        weight: 1
```

---

## Expected Behavior

### Example 1: Simple List (space-separated)
```yaml
{colors?min=2,max=3}
```
**Possible outputs:**
- "red blue" (2 items)
- "red blue green" (3 items)

### Example 2: Natural List (comma_and)
```yaml
{colors?min=2,max=3&sep=comma_and}
```
**Possible outputs:**
- "red and blue" (2 items)
- "red, blue and green" (3 items)

### Example 3: Unique Items
```yaml
{colors?min=3,max=3&unique=true}
```
**Possible outputs:**
- "red, blue and green" ✅ (all different)
- Never: "red, red and blue" ❌ (has duplicate)

### Example 4: Optional Items
```yaml
{adjectives?min=0,max=2}
```
**Possible outputs:**
- "" (0 items)
- "beautiful" (1 item)
- "beautiful mysterious" (2 items)

### Example 5: SANITY CHECK - M4 + M5 Integration
```yaml
{article} {creatures?min=2,max=3&sep=comma_and#{tags.can_fly}}
```
**What this tests:**
- Tag filtering (M4): Only selects flying creatures
- Article computation (M4): Computes article from first selected creature
- Min/Max multiplicity (M5): Selects 2-3 creatures
- Separator sets (M5): Formats with "and"

**Possible outputs:**
- "an owl, eagle and swan" ✅ (article matches first creature)
- "a swan and bat" ✅ (article matches first creature)
- "an eagle and owl" ✅ (article matches first creature)
- Never: "a owl" ❌ (wrong article)
- Never: "swan cat" ❌ (cat can't fly - filtered out)

---

## Challenges

### 1. Infinite Loop with Unique + Small Dataset
**Problem:** If `min=5` but datatype only has 3 values and `unique=true`

**Solution:** 
- Max attempts limit (100)
- Error if can't satisfy uniqueness requirement
- Document this edge case

### 2. Complex Template Token Structure
**Problem:** TemplateToken getting complex with many fields

**Solution:**
- Keep parameters in separate struct
- Use builder pattern for clarity

### 3. Separator Set Not Found
**Problem:** User specifies non-existent separator

**Solution:**
- Return helpful error
- Fall back to space separator?
- Or fail fast to catch typos

---

## Success Criteria

- [ ] Can parse `{ref?min=2,max=4&sep=comma_and&unique=true}`
- [ ] Can select 0 to N values from datatype
- [ ] Separator sets format correctly:
  - [ ] 0 items: ""
  - [ ] 1 item: "red"
  - [ ] 2 items: "red and blue"
  - [ ] 3+ items: "red, blue and green"
- [ ] Unique constraint prevents duplicates
- [ ] Space separator used by default
- [ ] Custom separator sets work
- [ ] Min=0 allows optional items
- [ ] Error handling for edge cases

---

## Implementation Order

1. ✅ Update template parser (most complex)
2. ✅ Update Reference model
3. ✅ Implement separator formatter
4. ✅ Update selector for multiple values
5. ✅ Update renderer phases 1 & 3
6. ✅ Create test package
7. ✅ Test all scenarios
8. ✅ User verification

**Estimated Time:** 5-6 hours

---

**Status:** Ready to implement! 🚀

**Next:** Start with template parser - that's the foundation for everything else.

