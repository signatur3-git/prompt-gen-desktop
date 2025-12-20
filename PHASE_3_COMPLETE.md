# CI/CD Phase Complete - Known Issues

## Status: ✅ Phase 3 Complete - CI/CD Working

All 4 matrix builds are now GREEN and producing artifacts for:
- Windows x64 (.exe, .msi, NSIS installer)
- Linux x64 (AppImage, .deb)
- macOS Apple Silicon (.app, .dmg)
- macOS Intel (.app, .dmg)

---

## Issues Resolved (11 total)

1. ✅ **Library Target** - Added `[lib]` section to Cargo.toml
2. ✅ **Test Packages** - Copied correct test packages from reference
3. ✅ **Linux Dependencies** - Added GTK/WebKit system packages
4. ✅ **Cargo.lock** - Committed to repository for reproducible builds
5. ✅ **Empty Workflows** - Removed pr-check.yml and release.yml
6. ✅ **Package Conflict** - Removed conflicting libappindicator3-dev
7. ✅ **Icon Format** - Created proper RGBA PNG icon
8. ✅ **Doctest** - Fixed example code in tag_expression.rs
9. ✅ **Clippy Warnings** - Fixed 19 clippy warnings (module inception, dead code, etc.)
10. ✅ **Dead Code** - Added #[allow(dead_code)] to API methods
11. ✅ **Tauri CLI** - Used official tauri-action instead of manual installation

---

## Known Issues (Runtime)

### Issue: Rulebooks Not Showing After Loading Package

**Status:** Not a bug - expected behavior for packages without rulebooks

**Details:**
- The application correctly loads and displays rulebooks when they exist
- Many older package files (pre-M9) don't contain rulebooks
- Rulebooks are an M9 feature, so only packages created with M9+ will have them

**How to Verify Rulebooks Work:**
1. Load `test-packages-for-comparison/rulebook-test.yaml` (included with installation)
2. This test file contains 3 rulebooks:
   - `basic_scenes` - Weighted entry points
   - `varied_scenes` - With batch variety
   - `themed_scenes` - With context defaults
3. Rulebooks should appear in the UI

**Expected Structure:**
Rulebooks must be in YAML as:
```yaml
namespaces:
  namespace_id:
    rulebooks:           # HashMap/object at namespace level
      rulebook_id:       # Key-value pairs
        name: "Rulebook Name"
        description: "Description"
        entry_points:
          - prompt_section: "namespace:section_name"
            weight: 1.0
        batch_variety: false
        context_defaults: {}
```

**Common Mistakes:**
- ❌ Rulebooks at package root level (should be in namespace)
- ❌ Rulebooks as an array (should be HashMap/object)
- ❌ Loading pre-M9 packages (they don't have rulebooks)

**Resolution:**
- User needs to either:
  1. Use a package with rulebooks (M9+)
  2. Add rulebooks to their package using the editor
  3. Verify their YAML structure matches the expected format

---

## Next Steps

### Phase 4: Test CI/CD ✅ COMPLETE
- ✅ All builds passing
- ✅ Artifacts downloadable
- ✅ Windows executable tested

### Phase 5: Documentation Updates (Next)
- Update main spec repository links
- Update project status files
- Add badges to README

### Phase 6: Optional npm Package
- CLI tools planned for M11
- Can skip for v1.0.0

### Phase 7: Final Release (v1.0.0)
- Tag v1.0.0
- Make repository public
- Create announcement

---

## Build Artifacts Available

All artifacts can be downloaded from GitHub Actions:
- **Windows:** rpg-desktop.exe, rpg-cli.exe, .msi installer, NSIS installer
- **Linux:** rpg-desktop, rpg-cli, .AppImage, .deb package
- **macOS (Apple Silicon):** rpg-desktop.app, .dmg
- **macOS (Intel):** rpg-desktop.app, .dmg

---

## CI/CD Performance

**First Build:** ~60-70 minutes (compiling everything)
**Subsequent Builds:** ~20-30 minutes (with caching)

**Current Workflow:**
- ✅ Runs tests (130 tests passing)
- ✅ Runs clippy with `-D warnings`
- ✅ Builds for 4 platforms in parallel
- ✅ Uses official tauri-action
- ✅ Caches dependencies for speed

---

## Lessons Learned

1. **Always run `cargo clippy -- -D warnings` locally** before pushing
   - This matches CI exactly and catches warnings early
   
2. **Use official actions when available**
   - tauri-action handles CLI installation and optimization automatically
   
3. **Batch commits**
   - Each push triggers full CI build on all platforms
   - Costs compute time and delays feedback
   
4. **Test packages structure is critical**
   - YAML structure must exactly match Rust models
   - Small differences cause silent failures

5. **Documentation is essential**
   - User-facing issues need troubleshooting guides
   - Expected behavior vs bugs must be clarified

---

## Ready for Next Phase ✅

Phase 3 (CI/CD Setup) is **COMPLETE** and working correctly. All green builds, all artifacts available, ready to proceed to Phase 5 (Documentation Updates).

