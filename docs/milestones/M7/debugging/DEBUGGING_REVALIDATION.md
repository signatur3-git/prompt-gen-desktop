# Debugging Guide: Why Validation Doesn't Re-Run

**Issue:** Validation errors don't disappear when you fix the issue  
**Tests:** All 33 tests pass, components work in isolation  
**Conclusion:** The bug is in the real application's event chain

---

## Step-by-Step Debugging

### Step 1: Verify Console Logs

Open DevTools (F12) and **clear the console**. Then:

1. **Edit a promptsection** (change template or reference)
2. **Watch for these logs in ORDER:**

```
Expected sequence:
🔔 PromptSectionEditor emitting update: {...}
📝 PromptSection update received: {...}
🔄 Scheduling validation after promptsection update
⏰ Validation scheduled (300ms debounce)
   Clearing previous timeout (if typing fast)
⏱️  Debounce complete, running validation now
🔍 Validation triggered for package: your-package-id
✅ Validation result: {...}
```

**If ANY of these are missing, tell me WHICH ONE!**

---

### Step 2: Check Validation Result

When you see `✅ Validation result:`, **expand the object** and check:

```javascript
{
  is_valid: true,    // ← Should be true when error is fixed
  errors: [],        // ← Should be EMPTY when fixed
  warnings: []
}
```

**If `errors` is NOT empty, the backend validator is still finding errors!**

---

### Step 3: Verify ValidationPanel Reactivity

After you see `✅ Validation result: { errors: [] }`:

1. **Check if ValidationPanel is still visible**
2. **Check console for:** `validationErrors.value =` assignment

**The panel should disappear if `errors` is empty!**

---

## Common Issues & Solutions

### Issue 1: No logs at all

**Problem:** PromptSectionEditor not emitting updates  
**Solution:** Check if you're editing the right component

### Issue 2: Missing "PromptSection update received"

**Problem:** Event not wired up  
**Solution:** Check if `@update` handler exists on PromptSectionEditor

### Issue 3: Missing "Validation scheduled"

**Problem:** `onPromptSectionUpdate` not calling `scheduleValidation()`  
**Solution:** This is a bug in PackageEditor.vue

### Issue 4: Missing "Debounce complete"

**Problem:** Timeout being cleared but not rescheduled  
**Solution:** This is a bug in the debounce logic

### Issue 5: Missing "Validation triggered"

**Problem:** `validatePackage()` not being called  
**Solution:** Bug in setTimeout callback

### Issue 6: Validation result has errors

**Problem:** Backend validator still finding issues  
**Solution:** 
- Check what the error is
- Verify your fix actually addresses it
- Restart dev server to get latest Rust code

### Issue 7: Panel still visible despite `errors: []`

**Problem:** Vue reactivity not working  
**Solution:** This would be VERY surprising given tests pass

---

## Quick Test

**Run this in the browser console while app is open:**

```javascript
// Check current state
console.log('Current errors:', window.__vue_app__?.config?.globalProperties?.$root?.validationErrors)

// Try to force clear
if (window.__vue_app__) {
  // This is a hack to test reactivity
  window.__vue_app__.config.globalProperties.$root.validationErrors = []
}
```

**If the panel disappears:** Reactivity works, issue is validation not running  
**If panel stays:** Vue reactivity is broken (unlikely given tests pass)

---

## The Most Likely Culprit

Based on your symptoms, I suspect **Issue #6**: The backend validator IS running, but it's STILL finding errors!

**Why:**
- You see the old error
- You make a fix
- Validation runs
- But backend still returns the same error

**Possible reasons:**
1. **Rust code not recompiled** - Old validator logic running
2. **Your fix doesn't actually fix the error** - Validator is right
3. **Different error appears** - You fixed one, but another appeared

---

## Action Items

**Right now, please:**

1. **Open DevTools Console** (F12)
2. **Clear console** (click trash icon)
3. **Make an edit** (e.g., fill in a reference target)
4. **Watch the console** for the emoji logs
5. **Copy and paste ALL the console output** to me

**I need to see:**
- Which logs appear
- Which logs are missing
- What the validation result actually says

**Then I can tell you exactly what's broken!**

---

## My Hypothesis

I bet you'll see ALL the logs, and the validation result will say:

```javascript
✅ Validation result: {
  is_valid: false,
  errors: [
    { message: "Some error you didn't actually fix", ... }
  ]
}
```

**Meaning:** The system IS working, but either:
- The Rust validator has a bug and still finds errors
- Your fix didn't actually address the error
- A different error appeared

**Let's find out!** 🔍

---

## If I'm Wrong

If you see **MISSING logs**, then I know exactly where the chain breaks and can fix it.

**Copy the console output and paste it to me!** That's all I need to solve this.

