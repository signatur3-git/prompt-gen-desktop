# 🎯 Next Steps - Release v1.0.1-rc

## ✅ What's Been Done

1. **Version bumped to 1.0.1** in all files (package.json, Cargo.toml, tauri.conf.json)
2. **Automatic version syncing** implemented via `useVersion()` composable
3. **v1.0.1-rc tag retagged** to point to the correct commit with all latest changes
4. **Changes pushed to GitHub** (commit `b423a24`)

---

## 🚀 Your Action Items

### Step 1: Delete Old GitHub Release

1. Go to: https://github.com/signatur3-git/prompt-gen-desktop/releases
2. Find the old **v1.0.1-rc** release
3. Click **Delete** to remove it
4. This frees up the tag for the new build

### Step 2: Check GitHub Actions

1. Go to: https://github.com/signatur3-git/prompt-gen-desktop/actions
2. Look for a workflow run triggered by the `v1.0.1-rc` tag push
3. **If you see a run:** Wait for it to complete (~15-30 minutes)
4. **If no run started:** You may need to manually create a release

### Step 3: Manual Release (If Needed)

If the workflow didn't trigger automatically:

1. Go to: https://github.com/signatur3-git/prompt-gen-desktop/releases/new
2. **Choose tag:** Select `v1.0.1-rc` from dropdown
3. **Release title:** `v1.0.1-rc - Package Library & Marketplace Integration`
4. **Description:** 
   ```markdown
   ## Features
   - Complete package library management system
   - Marketplace OAuth integration with deep linking
   - Browse and install packages from marketplace
   - Generate page with multi-package rulebook selection
   - Dark theme support
   - Automatic version syncing from package.json
   - Import/export packages from files and marketplace

   ## Bug Fixes
   - Fixed separator set tertiary operator handling
   - Resolved OAuth callback issues in production
   - Fixed token exchange endpoint URLs
   - Improved error handling and logging

   ## Installation
   Download the appropriate binary for your platform below.
   ```
5. **Check:** "This is a pre-release" (since it's an RC)
6. **Click:** "Publish release"
7. This will trigger the build workflow

### Step 4: Wait for Build to Complete

The GitHub Actions workflow will:
- ✅ Build for Windows (x64)
- ✅ Build for macOS (x64 and ARM64)
- ✅ Build for Linux (x64)
- ✅ Run all tests
- ✅ Upload artifacts to the release

**Estimated time:** 15-30 minutes

### Step 5: Download and Test

Once builds complete:

1. **Download** the Windows binaries
2. **Clear cache** (important!):
   ```powershell
   # Close ALL app instances first!
   Remove-Item -Recurse -Force $env:LOCALAPPDATA\com.rpg.desktop\EBWebView
   ```
3. **Run** the downloaded binary
4. **Verify:**
   - App header shows: **"App v1.0.1"** ✅ (not v1.0.0)
   - All features work correctly
   - Package library functions properly
   - Marketplace connection works

---

## 🔍 Verification

In the app, check:
- [ ] Header shows "App v1.0.1" when no package is loaded
- [ ] Library page shows installed packages
- [ ] Marketplace connection works
- [ ] Can browse and install packages
- [ ] Generate page allows multi-package rulebook selection
- [ ] Dark theme applies correctly

---

## 📝 Notes

### Version Display
- **Frontend header:** Shows version from `package.json` automatically
- **No more hardcoded versions** in Vue components!
- Future updates only need to change `package.json` (for UI display)

### Files That Still Need Manual Updates
- `package.json` - Single source of truth for frontend
- `Cargo.toml` - Required for Rust compilation
- `tauri.conf.json` - Required for installers and app metadata

### Why 3 Files?
These are different build systems that can't dynamically share version at compile time:
- **npm/Vite** (frontend) reads from package.json
- **Cargo** (Rust backend) reads from Cargo.toml
- **Tauri bundler** (installers) reads from tauri.conf.json

But at least the **UI never needs manual updates** anymore! 🎉

---

## ❓ Troubleshooting

### "App still shows v1.0.0"
**Solution:** Clear the WebView cache (see Step 5 above)

### "Release workflow didn't trigger"
**Solution:** Manually create the release (see Step 3 above)

### "Build failed in GitHub Actions"
**Solution:** Check the logs at https://github.com/signatur3-git/prompt-gen-desktop/actions
Common issues:
- Linting errors (run `npm run lint` locally first)
- Test failures (run `npm test` locally first)
- Rust compilation errors (run `cd src-tauri && cargo build` locally first)

---

## 🎉 Once Complete

You'll have a proper v1.0.1-rc release with:
- ✅ Correct version displayed in app
- ✅ All latest features included
- ✅ Automatic version syncing for future releases
- ✅ Clean release process going forward

---

**Created:** 2025-12-24  
**Status:** Waiting for release workflow to complete

