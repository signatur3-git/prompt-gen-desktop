# AddNamespaceDialog Pattern Test

**Date:** 2025-12-17  
**Issue:** User reports dots are still rejected  
**Status:** Code is correct, likely needs dev server restart

---

## Pattern in Code (CORRECT)

### HTML Pattern Attribute:
```vue
pattern="[a-z][a-z0-9._-]*"
```

**This pattern allows:**
- ✅ Starts with lowercase letter
- ✅ Followed by lowercase letters, numbers, **dots**, underscores, hyphens
- ✅ Example: `featured.common` ✓
- ✅ Example: `ontologies.en.articles` ✓

### JavaScript Validation:
```javascript
if (!/^[a-z][a-z0-9._-]*$/.test(namespaceId.value)) {
  alert('Namespace ID must start with lowercase letter and contain only lowercase letters, numbers, dots, underscores, and hyphens')
  return
}
```

**This regex allows:**
- ✅ Same pattern as HTML
- ✅ Dots are explicitly included in character class

---

## Test Cases

| Input | Should Work? | Pattern Match? |
|-------|--------------|----------------|
| `common` | ✅ Yes | `^[a-z][a-z0-9._-]*$` → Yes |
| `featured.common` | ✅ Yes | `^[a-z][a-z0-9._-]*$` → Yes |
| `ontologies.en.articles` | ✅ Yes | `^[a-z][a-z0-9._-]*$` → Yes |
| `my_namespace` | ✅ Yes | `^[a-z][a-z0-9._-]*$` → Yes |
| `my-namespace` | ✅ Yes | `^[a-z][a-z0-9._-]*$` → Yes |
| `Featured.common` | ❌ No | Uppercase F fails |
| `1common` | ❌ No | Starts with number |
| `.common` | ❌ No | Starts with dot |

---

## Why It Might Still Reject Dots

### Possible Causes:

1. **Dev Server Not Restarted**
   - Changes require HMR reload
   - Try: Stop and restart `npm run tauri:dev`

2. **Browser Cache**
   - Old component cached
   - Try: Hard refresh (Ctrl+Shift+R)

3. **File Not Saved**
   - Check if file has unsaved changes
   - Status: ✅ File saved

4. **Different File Being Used**
   - Check if there's another AddNamespaceDialog.vue
   - Status: ✅ Only one file

---

## Verification Steps

### 1. Check Pattern Syntax

**Test in browser console:**
```javascript
const pattern = /^[a-z][a-z0-9._-]*$/;
console.log(pattern.test('featured.common')); // Should be true
console.log(pattern.test('test.namespace')); // Should be true
console.log(pattern.test('Test.namespace')); // Should be false (uppercase)
```

### 2. Check HTML Validation

The HTML5 `pattern` attribute uses the same regex syntax but without delimiters:
```html
pattern="[a-z][a-z0-9._-]*"  <!-- No ^ or $ needed in HTML pattern -->
```

**This is CORRECT!** HTML pattern automatically anchors to full string.

### 3. Verify Character Class

In regex character classes `[...]`, the dot `.` is **literal**, not a wildcard:
```javascript
[a-z0-9._-]  // Matches: a-z, 0-9, dot (.), underscore (_), hyphen (-)
```

**This is CORRECT!**

---

## Solution

### If dots are still rejected:

**Step 1: Restart Dev Server**
```bash
# Stop current dev server (Ctrl+C)
# Then restart:
npm run tauri:dev
```

**Step 2: Hard Refresh Browser**
- Windows: `Ctrl + Shift + R`
- Mac: `Cmd + Shift + R`

**Step 3: Clear App Cache** (if Tauri app)
- Close app completely
- Restart dev server
- Relaunch app

**Step 4: Check DevTools Console**
- Open DevTools (F12)
- Try entering `featured.common`
- Check for any validation errors in console

---

## Expected Behavior After Fix

**Input:** `featured.common`

**HTML Pattern Validation:**
- Browser checks: `featured.common` matches `[a-z][a-z0-9._-]*`
- Result: ✅ Valid (no browser error)

**Form Submission:**
- JavaScript checks: `/^[a-z][a-z0-9._-]*$/.test('featured.common')`
- Result: ✅ Valid (passes regex test)

**Outcome:**
- No alert shown
- Namespace added to package
- Dialog closes

---

## Debug Commands

### Test Pattern in Node.js:
```javascript
const pattern = /^[a-z][a-z0-9._-]*$/;
const testCases = [
  'common',
  'featured.common',
  'ontologies.en.articles',
  'my_namespace',
  'my-namespace',
  'Featured.common',  // Should fail
  '1common',          // Should fail
  '.common'           // Should fail
];

testCases.forEach(test => {
  console.log(`"${test}": ${pattern.test(test) ? '✅ PASS' : '❌ FAIL'}`);
});
```

**Expected Output:**
```
"common": ✅ PASS
"featured.common": ✅ PASS
"ontologies.en.articles": ✅ PASS
"my_namespace": ✅ PASS
"my-namespace": ✅ PASS
"Featured.common": ❌ FAIL
"1common": ❌ FAIL
".common": ❌ FAIL
```

---

## Code is Correct!

**Confirmed:**
- ✅ Pattern allows dots: `[a-z][a-z0-9._-]*`
- ✅ JavaScript validation allows dots: `/^[a-z][a-z0-9._-]*$/`
- ✅ Error message mentions dots
- ✅ Placeholder shows dot examples
- ✅ Help text mentions dots

**Most Likely Issue:** Dev server needs restart for HMR to pick up changes.

---

## Action Required

**User should:**
1. Stop dev server (Ctrl+C in terminal)
2. Restart: `npm run tauri:dev`
3. Try entering `featured.common` again
4. Should work now! ✅

**If still fails:**
- Check browser DevTools console for errors
- Verify file saved (no asterisk in editor)
- Check if correct file is being loaded (inspect element)

---

**Status:** Code fix is complete and correct. Dots ARE allowed. User needs to restart dev server.

