# Validation Timing & Order Dependency Analysis

**Date:** 2025-12-17  
**Issue:** User question about validation timing and order dependency  
**Status:** ✅ Current behavior documented, enhancement recommended

---

## User's Question

> "Let's say the user adds a ref and gets the validation error, then changes the template to include the datatype, the validation is still against the saved template state, right? Can an author resolve this, or will the validator complain about the missing reference if the template modification is made first?"

---

## Current Behavior Analysis

### Validation Triggers on In-Memory State ✅

**Good news:** Validation runs on **current in-memory state**, NOT saved state!

**Code:**
```javascript
// PackageEditor.vue
watch(currentPackage, async (newPkg) => {
  if (newPkg) {
    await validatePackage(newPkg)  // Uses in-memory currentPackage
  }
}, { deep: true })
```

**This means:**
- ✅ User edits template → Validation sees new template
- ✅ User adds reference → Validation sees new reference
- ✅ User doesn't need to save to see validation results
- ✅ Real-time feedback as they type

---

## Order Dependency Issue

### Scenario 1: Add Reference First (No Problem)

**Steps:**
1. User adds reference definition: `creature: test:animals`
2. **Watcher triggers** → `validatePackage()` called
3. Validator checks: Reference `creature` exists ✅
4. Validator checks: Is `creature` used in template? No.
5. **Result:** No error (unused reference is just a warning at package level)

### Scenario 2: Edit Template First (Error!)

**Steps:**
1. User types `{creature}` in template
2. **Watcher triggers** → `validatePackage()` called
3. Validator checks: Template uses `{creature}`
4. Validator checks: Is reference `creature` defined? No!
5. **Result:** ❌ **ERROR** - "Reference 'creature' not found in promptsection"

### The Problem

**Order matters:**
- ✅ Reference → Template = No error
- ❌ Template → Reference = Error (until reference added)

**User friction:**
- User types `{creature}` in template
- **Immediate red error** appears
- User has to:
  1. See error
  2. Scroll down to references section  
  3. Click "Add Reference"
  4. Type "creature"
  5. **Error finally disappears**

**This is annoying but WORKABLE!** The error is helpful, it tells them what to do.

---

## Current Validator Behavior

### What Gets Checked (Errors)

**From `validator/mod.rs`:**

1. ✅ **Reference in template but not defined** → ERROR
   ```rust
   ValidationError::ReferenceNotFound {
       reference: "creature",
       defined_in: "my_promptsection",
       suggestion: Some("Add reference definition")
   }
   ```

2. ✅ **Reference target doesn't exist** → ERROR
   ```rust
   ValidationError::ReferenceNotFound {
       reference: "test:nonexistent",
       defined_in: "my_promptsection",
       suggestion: Some("Check datatype name")
   }
   ```

3. ✅ **Circular references** → ERROR
4. ✅ **Min > Max** → ERROR
5. ✅ **Unique constraint infeasible** → ERROR

### What Gets Checked (Warnings)

**Package-level unused components:**
1. ✅ **Unused datatype** (never referenced anywhere) → WARNING
2. ✅ **Unused promptsection** (never referenced) → WARNING
3. ✅ **Unused separator set** (never referenced) → WARNING

**NOT checked:**
- ❌ **Unused reference within promptsection** (defined but not in template) → No warning

---

## The UX Issue

### Current Experience:

**Author wants to add a new creature reference:**

**Option A: Template first (friction)**
1. Type `{creature}` in template
2. **Red error appears immediately** ❌
3. Scroll to references
4. Add reference
5. Error disappears ✅

**Option B: Reference first (smooth)**
1. Add reference `creature`
2. No error
3. Type `{creature}` in template
4. Still no error ✅

**Problem:** Authors naturally want to write templates first, then define what the references mean. But that triggers errors.

---

## Potential Enhancements

### Enhancement 1: Add "Unused Reference" Warning

**Add to validator:**
```rust
pub enum ValidationWarning {
    // ...existing warnings...
    
    UnusedReference {
        reference: String,
        promptsection: String,
    },
}
```

**Check in validator:**
```rust
fn check_unused_references(package: &Package, result: &mut ValidationResult) {
    for (ns_id, namespace) in &package.namespaces {
        for (ps_name, promptsection) in &namespace.prompt_sections {
            // Find references in template
            let used_refs = extract_references_from_template(&promptsection.template);
            
            // Check for defined but unused
            for ref_name in promptsection.references.keys() {
                if !used_refs.contains(ref_name) {
                    result.add_warning(ValidationWarning::UnusedReference {
                        reference: ref_name.clone(),
                        promptsection: format!("{}:{}", ns_id, ps_name),
                    });
                }
            }
        }
    }
}
```

**Result:**
- Reference defined but not in template → **Warning** (yellow)
- Reference in template but not defined → **Error** (red)

**Benefits:**
- Catches leftover references from refactoring
- Helps authors clean up their promptsections
- Non-blocking (just a warning)

---

### Enhancement 2: Auto-create Reference Stub

**When user types `{new_ref}` in template:**
1. Parser detects new reference
2. Automatically add stub to references:
   ```javascript
   references: {
       new_ref: {
           target: '',  // Empty, user needs to fill
           min: 1,
           max: 1,
           separator: null,
           unique: false,
           filter: null
       }
   }
   ```
3. Show **warning** instead of **error**:
   ```
   ⚠️ Reference 'new_ref' has no target defined
   💡 Click to configure reference
   ```

**Benefits:**
- No error when typing template first
- Reference section auto-populated
- User just needs to fill in target
- Smoother workflow

**Downside:**
- More magic, less explicit
- Could confuse users if they don't notice auto-creation

---

### Enhancement 3: Smart Error Messages

**Current error:**
```
❌ Reference not found: 'creature' in test:my_prompt
📍 my_prompt
💡 Check that the reference exists
```

**Enhanced error:**
```
❌ Reference 'creature' used in template but not defined
📍 test:my_prompt
💡 Add reference: Click "+ Add Reference" and enter "creature"
```

**Even better with action button:**
```
❌ Reference 'creature' used in template but not defined
📍 test:my_prompt
💡 [Add Reference "creature" Now]  ← Clickable button
```

**Benefits:**
- Tells user exactly what to do
- One-click fix
- Reduces friction

---

## Recommendation

### For v1.0.0 (Current Behavior is OK!)

**Current behavior is WORKABLE:**
- ✅ Validation runs on in-memory state (real-time)
- ✅ Errors are clear and helpful
- ✅ Authors can resolve by adding reference
- ✅ Order dependency exists but is manageable

**Small enhancement: Better error messages**

Change error message to be more explicit:
```
❌ Reference 'creature' is used in template but not defined in references
📍 test:my_prompt  
💡 Solution: Add reference definition for "creature" in the references section below
```

**Cost:** 5 minutes to update error message  
**Benefit:** Clearer guidance for users

---

### For v1.1.0 (Enhancement)

**Add "Unused Reference" warning:**
- Catches defined references not in template
- Helps cleanup during refactoring
- Symmetric with "undefined reference" error

**Cost:** 1-2 hours  
**Benefit:** Better author experience, catches mistakes

---

### For v1.2.0 (Polish)

**Add auto-create stub + action button:**
- Type `{ref}` → auto-creates stub
- Click error → jumps to reference and focuses target field
- One-click "Add Reference" button

**Cost:** 4-6 hours  
**Benefit:** Smooth workflow, less friction

---

## Answer to User's Questions

### Q1: "Validation is still against the saved template state, right?"

**Answer:** ❌ **NO!** Validation runs against **in-memory state**.

**Evidence:**
```javascript
watch(currentPackage, async (newPkg) => {
  if (newPkg) {
    await validatePackage(newPkg)  // currentPackage.value = in-memory
  }
}, { deep: true })
```

**This means:**
- User edits template → Validation sees it immediately
- User adds reference → Validation sees it immediately
- No need to save to see validation results
- Real-time feedback ✅

---

### Q2: "Can an author resolve this?"

**Answer:** ✅ **YES!** Authors can resolve in any order:

**Resolution Path 1: Template first**
1. Type `{creature}` in template → ❌ Error appears
2. Add reference `creature` → ✅ Error disappears

**Resolution Path 2: Reference first**
1. Add reference `creature` → No error
2. Type `{creature}` in template → No error

**Both paths work!** It's just that Path 1 shows an error temporarily.

---

### Q3: "Will the validator complain about the missing reference if the template modification is made first?"

**Answer:** ✅ **YES**, but this is GOOD!

**Scenario:**
1. Template has: `{animal}`
2. User changes to: `{creature}`  
3. References still has: `animal: test:animals`
4. **Validator correctly errors**: "Reference 'creature' not found"
5. User adds: `creature: test:animals`
6. **Error disappears**

**This is correct behavior because:**
- Template uses `{creature}` but it's not defined
- Without this error, render would fail
- Error guides user to fix the issue
- Real-time feedback prevents broken templates

---

## Summary

### Current Behavior (v1.0.0)

✅ **Validation is real-time** (in-memory state, not saved)  
✅ **Order dependency exists** (template→ref shows error, ref→template doesn't)  
✅ **Errors are helpful** (tells you what's wrong)  
✅ **Authors can resolve** (add the missing reference)  
⚠️ **Minor friction** (error appears while typing template)

**Verdict:** Working correctly! Minor friction is acceptable.

---

### Quick Win Enhancement

**Update error message to be more explicit:**

```javascript
// validation.rs
ValidationError::ReferenceNotFound { reference, defined_in, .. } => ErrorInfo {
    message: format!(
        "Reference '{}' is used in the template but not defined in references section",
        reference
    ),
    location: Some(defined_in),
    suggestion: Some(format!(
        "Add a reference definition for '{}' in the references section",
        reference
    )),
}
```

**Cost:** 5 minutes  
**Benefit:** Clearer guidance

---

### Future Enhancements (v1.1.0+)

1. **Unused reference warning** (catches defined but not used) - 1-2 hours
2. **Auto-create stub** (when typing {ref} in template) - 2-3 hours
3. **Action button** (one-click "Add Reference") - 2-3 hours

**Total for all:** 5-8 hours of work

---

## Files to Update (If Adding Enhancement)

### Quick Win (Better Error Message)

**File:** `src-tauri/src/commands/validation.rs`

**Change:**
```rust
ValidationError::ReferenceNotFound { reference, defined_in, suggestion } => ErrorInfo {
    message: format!(
        "Reference '{}' is used in template but not defined in references",
        reference
    ),
    location: Some(defined_in),
    suggestion: Some(format!(
        "Add reference definition: Click '+ Add Reference' and enter '{}'",
        reference
    )),
}
```

**Test:** Type `{invalid}` in template → Should see clearer message

---

## Conclusion

**Current behavior is CORRECT and WORKABLE!**

The order dependency is a minor UX friction but:
- Real-time validation works ✅
- Authors can resolve errors ✅
- Errors guide users to fix ✅
- No broken states possible ✅

**Small improvement:** Better error message (5 min)  
**Future enhancement:** Unused reference warning (1-2 hours)

**For v1.0.0:** Ship as-is with improved error message.  
**For v1.1.0:** Add unused reference warnings.

---

**User can proceed with confidence!** The validation is working correctly. 🎉

