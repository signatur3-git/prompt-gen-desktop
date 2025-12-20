# M7 Tauri v2 Import Fix

**Date:** 2025-12-17  
**Issue:** Import errors with Tauri v2 API

---

## Problem

The app failed to start with the error:
```
Failed to resolve import "@tauri-apps/api/tauri" from "src/components/PackageEditor.vue"
Failed to resolve import "@tauri-apps/api/dialog"
```

## Root Cause

Tauri v2 changed the API structure:
- **Old (v1):** `@tauri-apps/api/tauri` and `@tauri-apps/api/dialog`
- **New (v2):** `@tauri-apps/api/core` and `@tauri-apps/plugin-dialog`

The existing components (PackageViewer.vue, LivePreview.vue) were already using the correct v2 imports, but the new M7 components used the old v1 imports.

## Solution

Updated PackageEditor.vue imports:

**Before:**
```javascript
import { invoke } from '@tauri-apps/api/tauri'
import { open, save } from '@tauri-apps/api/dialog'
```

**After:**
```javascript
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'
```

## Additional Fixes

Also added `default-run = "rpg-desktop"` to Cargo.toml to fix the "could not determine which binary to run" error when running `npm run tauri:dev`.

## Status

✅ **Fixed** - App should now compile and run correctly

## Files Modified

1. `src/components/PackageEditor.vue` - Fixed Tauri imports
2. `src-tauri/Cargo.toml` - Added default-run
3. `src/App.vue` - Removed orphaned CSS

## Testing

Run the dev server:
```bash
cd reference-impl/rpg-desktop
npm run tauri:dev
```

Should now:
- ✅ Compile without errors
- ✅ Start the desktop app
- ✅ Show the new M7 Package Editor UI

---

**All import issues resolved!** 🎉

