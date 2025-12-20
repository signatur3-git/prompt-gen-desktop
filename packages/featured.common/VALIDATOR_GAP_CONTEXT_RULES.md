# Validator Gap: Context Reference Validation

**Date:** 2025-12-18  
**Discovered by:** User testing  
**Issue:** Validator doesn't check if context references have corresponding rules  
**Severity:** Medium - Should be validation error, not runtime error  
**Type:** Validation gap (missing check)

---

## Issue Description

**What happened:**
- User tried to render `character_in_action` prompt section
- Got **runtime error:** "Reference not selected: article"
- Validator had reported package as **VALID** with no errors

**What SHOULD have happened:**
- Validator should have reported **validation error:** "Context reference 'article' has no corresponding rule"
- Error should appear during package validation, not at runtime

**Root cause:**
- `character_in_action` references `context:article`
- No rule existed in the `common.styles` namespace to set `article`
- **Validator doesn't check if context references have corresponding rules**

---

## Why This Is A Validation Gap (Not Just Runtime)

**The issue is NOT:**
- Missing rule at runtime ❌
- Template syntax problem ❌
- Reference resolution problem ❌

**The issue IS:**
- **Validator should detect this statically** ✅
- **Missing validation check** ✅
- **Causes runtime error when it should be validation error** ✅

**Comparison:**

| Check | Currently | Should Be |
|-------|-----------|-----------|
| Reference exists in template | ✅ Validates | ✅ Validates |
| Target datatype exists | ✅ Validates | ✅ Validates |
| Context reference has rule | ❌ **Missing!** | ✅ **Should validate** |

**Result:** Package passes validation but fails at runtime - this is the gap!

---

## Example

### Broken Configuration (validator says OK, runtime fails):

```yaml
common.styles:
  id: common.styles
  
  # NO RULES HERE - validator doesn't complain!
  
  prompt_sections:
    character_in_action:
      template: "{article} {age} {archetype} {action}"
      references:
        article:
          target: context:article  # ← References context
        age:
          target: common.characters:character_ages
        # ...
```

**Validator:** ✅ VALID  
**Runtime:** ❌ "Reference not selected: article"

### Fixed Configuration:

```yaml
common.styles:
  id: common.styles
  
  rules:  # ← Added rules section
    - name: compute_action_character_article
      phase: enrichment
      logic:
        - set: article
          from: "ref:age.tags.article"
          scope: prompt
  
  prompt_sections:
    character_in_action:
      template: "{article} {age} {archetype} {action}"
      references:
        article:
          target: context:article  # Now has rule to set it
        # ...
```

**Validator:** ✅ VALID  
**Runtime:** ✅ Works!

---

## Why Validator Doesn't Catch This

### Current Validation Logic:

The validator checks:

1. **Check references exist in template** - ✅ Does this
   ```rust
   // Validator checks template syntax is valid
   Template::parse(&promptsection.template)?;
   ```

2. **Check target datatypes exist** - ✅ Does this  
   ```rust
   // Validator checks reference targets resolve
   if !reference.target.starts_with("context:") {
       validate_datatype_exists(package, reference.target)?;
   }
   ```

3. **Check template syntax valid** - ✅ Does this
   ```rust
   // Validator parses template to find issues
   ```

4. **Check context references have rules** - ❌ **DOESN'T DO THIS**
   ```rust
   // THIS CHECK IS MISSING!
   // Should be:
   if reference.target.starts_with("context:") {
       // Extract context key from "context:article"
       let context_key = reference.target.strip_prefix("context:")?;
       
       // Check if ANY rule in this namespace sets this key
       let has_rule = namespace.rules.iter().any(|rule| {
           rule.logic.iter().any(|logic| logic.set == context_key)
       });
       
       if !has_rule {
           return Err(ValidationError::MissingContextRule {
               context_key,
               promptsection: promptsection.name,
               namespace: namespace.id,
           });
       }
   }
   ```

### What's Missing:

**The validator SKIPS context references:**

```rust
// In current validator (src-tauri/src/validator/mod.rs)
if reference.target.starts_with("context:") {
    continue;  // ← SKIPS validation!
}
```

**Why it skips:** Assumes context references will be populated by rules, so doesn't validate them

**The problem:** Doesn't check if rules ACTUALLY exist to populate them!

**Result:** Package validates even though it will fail at runtime

---

## This Should Be A Validation Error

**Expected validator output:**

```
❌ Validation Error

Context reference 'article' in prompt section 'character_in_action' 
has no corresponding rule to set it.

📍 Location: common.styles:character_in_action
💡 Suggestion: Add a rule in the common.styles namespace:

   rules:
     - name: compute_article_for_action
       phase: enrichment
       logic:
         - set: article
           from: "ref:age.tags.article"
           scope: prompt
```

**Instead, current output:**
```
✅ VALID

(No errors, but will fail at runtime!)
```

---

## Impact

### Current State:
- **Package authors** can create packages that pass validation but fail at runtime
- **Errors appear late** - during testing/rendering, not authoring
- **Poor developer experience** - "Why did validation say it was OK?"
- **Confusing** - Error message doesn't explain it's a missing rule

### User Impact:
- Spent time debugging runtime error instead of fixing validation error
- Had to understand namespace-level rules to figure out the issue
- Could have been caught immediately during package editing

### Expected Flow (With Fix):

**During authoring:**
```
User adds context reference → Validator runs → Error appears
↓
"Missing rule for context:article in common.styles namespace"
↓
User adds rule → Validator runs → Error clears → Save enabled
```

**Current broken flow:**
```
User adds context reference → Validator runs → No error
↓
User saves package → Loads in app → Tries to render
↓
Runtime error! "Reference not selected: article"
↓
User debugs, discovers missing rule, adds it manually
```

**Difference:** Error feedback 10+ minutes later, not immediately!

---

## Recommended Fix

### Phase 1: Add Validation Check (HIGH PRIORITY)

**Implementation:**
1. Add `ValidationError::MissingContextRule` variant
2. In validator, check each `context:` reference
3. Verify a rule in the same namespace sets that context value
4. Report error if no rule found

**Effort:** ~1 hour  
**Priority:** HIGH - Prevents runtime errors

**Example error message:**
```
❌ Error: Context reference 'article' in prompt section 'character_in_action' 
   has no corresponding rule to set it.
   
   📍 common.styles:character_in_action
   💡 Add a rule in the common.styles namespace that sets 'article' in context
```

### Phase 2: Suggest Fix (MEDIUM PRIORITY)

**Enhancement:** Auto-suggest rule to add

```
💡 Suggestion: Add this rule to common.styles:

   rules:
     - name: compute_article_for_character_in_action
       phase: enrichment
       logic:
         - set: article
           from: "ref:age.tags.article"  # Assuming 'age' reference exists
           scope: prompt
```

**Effort:** ~2 hours  
**Priority:** MEDIUM - Nice to have

---

## Workaround (Temporary)

**For package authors:**
1. Manually check all `context:` references
2. Verify each has a rule in the same namespace
3. Test rendering before considering package complete

**For reference implementation:**
- Fix will be added in next validation improvement phase
- Not blocking v1.0.0 but should be in v1.1.0

---

## Similar Issues to Check

### Other Validation Gaps:

1. **Circular rule dependencies** - Rules that depend on each other
2. **Rule references non-existent refs** - Rule tries to read `ref:xyz` that doesn't exist
3. **Scope mismatches** - Rule sets `scope: prompt` but reference expects different scope
4. **Phase ordering** - Rules in wrong phase for when reference is needed
5. **Nested promptsection references** - Validator doesn't trace through nested sections ⚠️ **DISCOVERED**

**Recommendation:** Audit validator for all context-related checks

### Newly Discovered: Nested Reference Detection

**Issue:** Validator reports datatypes as "unused" when referenced through nested promptsections

**Example:**
```yaml
# Direct reference - detected ✅
fantasy_scene:
  references:
    creature:
      target: common.fantasy:creatures  # ✅ Validator sees this

# Nested reference - NOT detected ❌
ultimate_scene:
  references:
    styled_scene:
      target: common.styles:styled_fantasy  # This nests to fantasy_scene
      # ❌ Validator doesn't trace through to see creatures is used
```

**Impact:** Low - False warnings, but package still validates
**Priority:** Low - Nice to have for v1.1.0

---

## Resolution

**Fixed in package:** ✅ Added rule to `common.styles` namespace
**Fixed in validator:** ⏳ To be implemented
**Root cause identified:** 🔴 **CRITICAL BLOCKER** - Rules namespace-scoped (should be cross-package)

**Timeline:**
- Package workaround: Immediate (done) ✅
- Validator fix: v1.1.0 or v1.0.1 patch
- **Root cause fix: v1.0.0 (BLOCKER)** - See `BLOCKER_RULE_SCOPING.md`

**Related Issues:**
- **BLOCKER:** Rules should be package-wide and cross-package importable
- User requirement: "Other package authors must be able to import and use the rules common in the base package(s)"
- Current behavior forces rule duplication across namespaces
- See `BLOCKER_RULE_SCOPING.md` for complete analysis and implementation plan

---

## Testing Notes

**To reproduce:**
1. Create prompt section with `context:article` reference
2. Don't add rule to set article in that namespace
3. Validator says VALID
4. Try to render → runtime error

**To verify fix (when implemented):**
1. Same setup as above
2. Validator should report ERROR
3. Cannot render until rule added
4. Developer catches issue during authoring

---

## Documentation Impact

**Should update:**
- `docs/architecture/validation.md` - Document this check
- `docs/guides/tutorial-series/tutorial-3-context-rules.md` - Warn about namespace scope
- Reference implementation README - List known validation gaps

---

**Status:** Issue found ✅, Package fixed ✅, Validator enhancement needed ⏳  
**Classification:** **Validation gap** - Should be validation error, not runtime error  
**Priority for v1.0.0:** Not blocking (runtime error is clear, but suboptimal UX)  
**Priority for v1.1.0:** HIGH (better developer experience - catch at validation time)

**Key Point:** The error IS detectable statically - validator just doesn't check for it yet!

