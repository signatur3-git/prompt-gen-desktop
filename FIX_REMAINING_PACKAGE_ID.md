# Desktop Fix - Manual Instructions

**Issue:** The main script fixed `Dependency` struct but missed other references

---

## Errors Found

### Error 1 & 2: VersionMismatchData (lines 283, 291)

**File:** `src/parser/dependency_resolver.rs`

The `VersionMismatchData` struct still has `package_id` field but code tries to access `package`.

**Fix Option 1 - Change field access:**
```rust
// Line 283 - Change from:
writeln!(f, "Version mismatch for dependency: {}", data.package)?;

// To:
writeln!(f, "Version mismatch for dependency: {}", data.package_id)?;

// Line 291 - Change from:
data.package, data.required, data.found)

// To:
data.package_id, data.required, data.found)
```

**Fix Option 2 - Change struct definition (better - consistent with DEC-0012):**

Find the `VersionMismatchData` struct definition and change:
```rust
// Change from:
pub package_id: String,

// To:
pub package: String,
```

Then the error lines 283 and 291 are already correct!

---

### Error 3: Test code (line 346)

**File:** `src/parser/dependency_resolver.rs` (in test module)

**Fix:**
```rust
// Line 346 - Change from:
package_id: "test.package".to_string(),

// To:
package: "test.package".to_string(),
```

---

## Quick Fix (Run this script)

```powershell
cd D:\workspaces\prompt-gen-desktop
..\prompt-gen-spec\scripts\fix-desktop-dec-0012-additional.ps1
```

This script will:
1. Find and fix VersionMismatchData struct
2. Fix all `.package_id` field access
3. Fix test code
4. Run cargo check to verify

---

## Or Manual Fix

If you prefer to fix manually:

1. **Find VersionMismatchData struct:**
   ```powershell
   cd src-tauri\src
   Get-ChildItem -Recurse -Filter *.rs | Select-String -Pattern "struct VersionMismatchData"
   ```

2. **Change the field:**
   ```rust
   pub struct VersionMismatchData {
       pub package: String,  // Changed from package_id
       pub required: String,
       pub found: String,
       pub path: Option<String>,
       pub details: Option<String>,
   }
   ```

3. **Fix test at line 346:**
   ```rust
   Dependency {
       package: "test.package".to_string(),  // Changed from package_id
       version: "1.0.0".to_string(),
       path: None,
   }
   ```

4. **Verify:**
   ```powershell
   cd src-tauri
   cargo check
   ```

---

## Why This Happened

The main script only replaced field access patterns like `.package_id`, but:

1. **VersionMismatchData** is a separate struct (not Dependency)
2. **Test code** uses struct initialization syntax `package_id: value`
3. The regex in the main script didn't catch these patterns

---

## Status

✅ Main Dependency struct - Fixed  
❌ VersionMismatchData struct - Needs fix  
❌ Test code - Needs fix  

**Run the additional script or apply manual fixes above.**

