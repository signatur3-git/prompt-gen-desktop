# HOW TO REPRODUCE THE ORIGINAL BUG

**Purpose:** Reproduce the "validation error doesn't disappear" bug  
**Status:** Ready to test

---

## Quick Fix Applied First

✅ **Unused references now show ERRORS** (not silent)

**What this means:**
- Define reference without using it → ❌ ERROR
- Use `{ref}` without defining → ❌ ERROR
- Both directions checked → Authors get immediate feedback

---

## Reproduction Steps

### Scenario 1: Add Reference, Then Template (Should Work)

**This tests if validation re-runs when you edit:**

1. **Start the app:**
   ```bash
   npm run tauri:dev
   ```

2. **Open console** (F12) and **clear it**

3. **Load or create a package**

4. **Edit a promptsection**

5. **Add a reference:**
   - Name: `test_ref`
   - Target: `test:some_datatype` (doesn't matter if it exists)
   - Leave template EMPTY

6. **You should see ERROR:**
   ```
   ❌ Reference not found: unused: test_ref
   💡 Reference 'test_ref' is defined but not used in template
   ```

7. **Now edit the template to add:** `{test_ref}`

8. **WATCH THE CONSOLE for:**
   ```
   🔔 PromptSectionEditor emitting update
   📝 PromptSection update received
   ⏰ Validation scheduled
   ⏱️  Debounce complete
   🔍 Validation triggered
   ✅ Validation result: {...}
   ```

9. **CHECK: Did the "unused" error disappear?**
   - ✅ **YES** → Validation is working!
   - ❌ **NO** → **THIS IS THE BUG!** Copy console logs!

---

### Scenario 2: Add Template, Then Reference (Should Work)

**This is the reverse scenario:**

1. **Clear console**

2. **Edit promptsection**

3. **Edit template to add:** `{new_ref}`

4. **You should see ERROR:**
   ```
   ❌ Reference not found: 'new_ref'
   💡 Add reference definition for 'new_ref'
   ```

5. **Now add the reference:**
   - Name: `new_ref`
   - Target: `test:datatype`

6. **WATCH THE CONSOLE** (same logs as above)

7. **CHECK: Did the error disappear?**
   - ✅ **YES** → Validation is working!
   - ❌ **NO** → **THIS IS THE BUG!** Copy console logs!

---

### Scenario 3: Fix Invalid Target (Should Work)

**This tests if validation re-runs when you fix a reference target:**

1. **Clear console**

2. **Edit promptsection with:**
   - Template: `{creature}`
   - Reference: `creature` with target `invalid`

3. **You should see ERROR:**
   ```
   ❌ Reference not found: 'invalid' in test:prompt_name
   ```

4. **Change reference target to:** `test:animals` (or any valid datatype)

5. **WATCH THE CONSOLE**

6. **CHECK: Did the error disappear?**
   - ✅ **YES** → Validation is working!
   - ❌ **NO** → **THIS IS THE BUG!** Copy console logs!

---

## What to Look For

### If Validation IS Working:

**Console shows:**
```
🔔 PromptSectionEditor emitting update: {...}
   ↓
📝 PromptSection update received: {...}
   ↓
🔄 Scheduling validation
   ↓
⏰ Validation scheduled (300ms debounce)
   ↓
⏱️  Debounce complete, running validation now
   ↓
🔍 Validation triggered for package: your-package
   ↓
✅ Validation result: { is_valid: true, errors: [], ... }
```

**UI:** Error disappears from ValidationPanel

---

### If Bug Exists:

**Possibility 1: Missing logs**
- Some emoji missing → Chain is broken
- **COPY which logs appear and which don't!**

**Possibility 2: All logs appear, but error stays**
- All emoji logs present
- Validation result shows `errors: []`
- But ValidationPanel still visible
- **This is a Vue reactivity bug!**

**Possibility 3: All logs appear, validation returns same error**
- All emoji logs present
- Validation result shows `errors: [...]` (not empty)
- **The validator is finding a DIFFERENT error!**
- **EXPAND the validation result object and copy the error message**

---

## What to Copy and Paste

**If bug occurs, copy:**

1. **Exact steps you did** (which scenario)

2. **ALL console output** from when you made the fix

3. **The validation result object** (expand it to see full errors array)

4. **Screenshot** of the ValidationPanel showing the error

---

## Expected Outcome

**After this test, ONE of three things:**

1. ✅ **Everything works** → Original bug is already fixed!

2. ❌ **Logs missing** → I know where the chain breaks, can fix immediately

3. ❌ **All logs present but error stays** → Either validator bug or reactivity bug, console logs will tell me which

---

## Changes Made

**File:** `src-tauri/src/validator/mod.rs`

**Change:** Added unused reference check as ERROR (not warning)

**Benefit:**
- Authors get immediate feedback
- Catches typos in reference names
- Catches incomplete work (defined but forgot to use)
- Both directions now checked (template ↔ references)

**Compile status:** ✅ Passes

---

## Ready to Test!

**Restart dev server:**
```bash
npm run tauri:dev
```

**Follow Scenario 1, 2, or 3 above**

**Copy console logs if error doesn't disappear!**

---

**This will finally let us see the ACTUAL bug!** 🔍

