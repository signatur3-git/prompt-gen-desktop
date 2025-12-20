# Debugging "Always Red Apple" Issue

**Problem:** User reports always getting "red apple" regardless of seed changes

**Expected:** Different seeds should produce different outputs

---

## Investigation Steps

### 1. Verified RNG Works
- Ran unit test `test_determinism` ✅ PASSES
- Ran unit test `test_render_different_seeds` ✅ PASSES
- The underlying RNG implementation is correct

### 2. Added Debug Logging

Added extensive logging to trace the issue:

**In `render_prompt` command:**
- Logs the seed value received from UI
- Logs the final output

**In `selector.select`:**
- Logs how many values are available
- Logs which value was selected

**In `weighted_choice`:**
- Logs the random value generated
- Logs the target after multiplying by total weight
- Logs which index was selected

---

## What to Look For

**When you click Render again, check the terminal output for:**

```
render_prompt called with seed: <number>
Selecting from test:colors: 3 values available
weighted_choice: random=<0.0-1.0>, total=3.0, target=<0.0-3.0>
weighted_choice: selected index <0-2>
Selected: <color> from test:colors
Selecting from test:objects: 2 values available
weighted_choice: random=<0.0-1.0>, total=2.0, target=<0.0-2.0>
weighted_choice: selected index <0-1>
Selected: <object> from test:objects
render_prompt output: <color> <object>
```

---

## Possible Causes

### Hypothesis 1: Seed Not Changing
If you see the same seed value in logs, the UI isn't updating the seed properly.

**Look for:** `render_prompt called with seed: 42` every time (same number)

### Hypothesis 2: RNG State Issue
If seed changes but selections don't, there's a problem with RNG state.

**Look for:** Different seeds but same `random=` values

### Hypothesis 3: Weighted Choice Bug
If random values change but always select same index.

**Look for:** Different `random=` but always `selected index 0`

---

## Next Steps

1. **Click Render with different seeds** (42, 99, 123, etc.)
2. **Watch the terminal** where `npm run tauri:dev` is running
3. **Copy the log output** and share it
4. **We'll identify the issue** from the logs

---

## Quick Test

Try these specific seeds and note outputs:

| Seed | Expected | Actual |
|------|----------|--------|
| 42   | ???      | red apple |
| 99   | ???      | red apple |
| 123  | ???      | red apple |
| 999  | ???      | red apple |

If they're all "red apple", we have a real problem!

---

## Status

✅ Debug logging added
✅ App recompiling
⏳ Waiting for test results

**Action:** Click Render with different seeds and check terminal output!

