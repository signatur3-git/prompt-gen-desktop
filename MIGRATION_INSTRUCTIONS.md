# M10 Execution Guide: Desktop App Extraction

**Milestone:** M10 - Desktop App Extraction & CI/CD  
**Duration:** Week 18-19 (Estimated)  
**Status:** Ready to Execute  
**Date:** 2025-12-20

---

## Overview

M10 extracts the reference implementation from `reference-impl/rpg-desktop/` into a standalone GitHub repository with CI/CD, pre-built binaries, and npm packages. This prepares for the **external v1.0.0 public release**.

**Key Goal:** Transform internal development setup → production-ready standalone application

---

## Prerequisites ✅

Before starting, verify:
- ✅ M9 complete (all features working)
- ✅ All tests passing (Rust, Vue, Tauri)
- ✅ No critical bugs
- ✅ Documentation complete
- ✅ Internal v1.1.0 tagged

**Status:** All prerequisites met! Ready to proceed.

---

## Phase 1: Repository Setup (Manual)

### Step 1.1: Create GitHub Repository

**Action:** Manually create new repository on GitHub

1. Go to GitHub.com
2. Click "New Repository"
3. Name: `prompt-gen-desktop`
4. Description: "Desktop authoring tool for Random Prompt Generator packages"
5. **Keep it Private** initially (public after testing)
6. **Do NOT** initialize with README, .gitignore, or license
7. Create repository

**Result:** Empty repository URL: `https://github.com/[your-org]/prompt-gen-desktop.git`

---

### Step 1.2: Prepare Local Repository

**Action:** Create new local git repository

```bash
# Create new directory for standalone repo
cd D:\workspaces
mkdir prompt-gen-desktop
cd prompt-gen-desktop

# Initialize git
git init
git branch -M main

# Set git identity (use your GitHub identity)
git config user.name "Signatur3"
git config user.email "hobbykuenstler@gmail.com"

# Add remote (replace [your-org] with your GitHub username)
git remote add origin https://github.com/signatur3-git/prompt-gen-desktop.git
```

**Checkpoint:** ✅ Empty git repository created and connected to GitHub

---

### Step 1.3: Copy Application Files

**Action:** Copy files from `prompt-gen-spec/reference-impl/rpg-desktop/` to `prompt-gen-desktop/`

**Files to Copy:**

```
Source: D:\workspaces\prompt-gen-spec\reference-impl\rpg-desktop\
Target: D:\workspaces\prompt-gen-desktop\

Copy these directories/files:
├── src/                    # Vue frontend
├── src-tauri/              # Rust backend
├── public/                 # Static assets
├── tests/                  # Test files (if exist)
├── package.json
├── package-lock.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── index.html
├── .gitignore              # Create new one (see template below)
├── README.md               # Create new one (see template below)
├── LICENSE                 # Copy from parent
└── Cargo.toml              # In src-tauri/
```

**Command:**
```powershell
# Copy all files
Copy-Item -Path "D:\workspaces\prompt-gen-spec\reference-impl\rpg-desktop\*" -Destination "D:\workspaces\prompt-gen-desktop\" -Recurse -Exclude node_modules,target,dist

# Verify
cd D:\workspaces\prompt-gen-desktop
ls
```

**Checkpoint:** ✅ All application files copied

---

### Step 1.4: Create .gitignore

**Action:** Create `.gitignore` file in `prompt-gen-desktop/`

**Content:**
```gitignore
# Dependencies
node_modules/
/target

# Build outputs
/dist
/dist-ssr
*.local

# Tauri build
/src-tauri/target
/src-tauri/Cargo.lock

# IDE
.vscode/*
!.vscode/extensions.json
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Temp
*.tmp
*.temp
```

**Checkpoint:** ✅ .gitignore created

---

### Step 1.5: Create README.md

**Action:** Create `README.md` for the standalone repository

**Content:**
```markdown
# Random Prompt Generator - Desktop Application

Desktop authoring tool for creating and editing Random Prompt Generator (RPG) packages.

## Features

- 📝 **Visual Package Editor** - Create packages without writing YAML
- 🎲 **Live Preview** - See rendered prompts in real-time
- ✅ **Validation** - Real-time error detection with jump-to-error
- 📚 **Rulebooks** - Create prompt templates with weighted entry points
- 🔗 **Dependencies** - Reuse packages with semver version management
- 💾 **YAML Export** - Clean, readable package format

## Installation

### Pre-built Binaries (Recommended)

Download the latest release for your platform:
- [Windows (x64)](https://github.com/[your-org]/prompt-gen-desktop/releases)
- [macOS (Intel/Apple Silicon)](https://github.com/[your-org]/prompt-gen-desktop/releases)
- [Linux (AppImage)](https://github.com/[your-org]/prompt-gen-desktop/releases)

### Build from Source

**Prerequisites:**
- Node.js 18+ and npm
- Rust 1.70+
- Platform-specific dependencies (see [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites))

**Build:**
```bash
# Install dependencies
npm install

# Development mode
npm run tauri:dev

# Production build
npm run tauri:build
```

## Usage

1. **Create Package** - File → New Package
2. **Add Entities** - Datatypes, Prompt Sections, Separators, Rules
3. **Preview** - See rendered output in Live Preview panel
4. **Validate** - Check for errors in Validation panel
5. **Save** - File → Save Package

## Documentation

- [Getting Started Guide](https://github.com/[your-org]/prompt-gen-spec/blob/main/docs/guides/getting-started.md)
- [Tutorial Series](https://github.com/[your-org]/prompt-gen-spec/blob/main/docs/guides/tutorial-series/)
- [Full Documentation](https://github.com/[your-org]/prompt-gen-spec/tree/main/docs)

## Related Projects

- [RPG Specification](https://github.com/[your-org]/prompt-gen-spec) - Format specification
- [RPG CLI](https://github.com/[your-org]/prompt-gen-cli) - Command-line tools (coming soon)

## Version

**Current:** v1.0.0 (External)  
**Internal:** v1.1.0 (Includes Dependencies & Rulebooks)

## License

MIT License - See LICENSE file

## Contributing

Contributions welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Support

- [Issue Tracker](https://github.com/[your-org]/prompt-gen-desktop/issues)
- [Discussions](https://github.com/[your-org]/prompt-gen-desktop/discussions)
- [Specification Repository](https://github.com/[your-org]/prompt-gen-spec)
```

**Checkpoint:** ✅ README.md created

---

### Step 1.6: Create CHANGELOG.md

**Action:** Create `CHANGELOG.md` for version tracking

**Content:**
```markdown
# Changelog

All notable changes to the Random Prompt Generator Desktop Application will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2025-12-XX (Upcoming)

### Added
- Visual package editor with 5 component editors
- Real-time validation with error/warning display
- Live preview with batch generation
- Rulebook support with weighted entry points
- Dependency management with semver
- Cross-package references (all entity types)
- Jump-to-error navigation
- YAML package import/export
- Deterministic rendering with seeded RNG
- Context system with rules
- Tag filtering with complex expressions
- Nested templates (up to 10 levels)
- Min/max multiplicity with unique constraint
- Natural list formatting with separator sets

### Changed
- N/A (Initial release)

### Deprecated
- N/A

### Removed
- N/A

### Fixed
- N/A

### Security
- N/A

## Version History

- **v1.0.0** - First public release (External)
  - Feature set matches Internal v1.1.0
  - Includes Rulebooks and Dependencies
  - Production-ready with full documentation

[1.0.0]: https://github.com/[your-org]/prompt-gen-desktop/releases/tag/v1.0.0
```

**Checkpoint:** ✅ CHANGELOG.md created

---

### Step 1.7: Initial Commit

**Action:** Commit all files to git

```bash
cd D:\workspaces\prompt-gen-desktop

# Add all files
git add .

# Commit
git commit -m "Initial commit: RPG Desktop v1.0.0

- Complete Vue + Tauri desktop application
- Visual package editor with 5 component editors
- Real-time validation and live preview
- Rulebook and dependency support
- All M1-M9 features implemented
- Ready for CI/CD setup

Internal v1.1.0 codebase prepared for external v1.0.0 release."

# Push to GitHub
git push -u origin main
```

**Checkpoint:** ✅ Code pushed to GitHub

---

## Phase 2: Verify Build Locally

**Before setting up CI/CD, verify everything builds on your machine**

### Step 2.1: Clean Build Test

```bash
cd D:\workspaces\prompt-gen-desktop

# Clean everything
Remove-Item node_modules -Recurse -Force -ErrorAction SilentlyContinue
Remove-Item src-tauri\target -Recurse -Force -ErrorAction SilentlyContinue

# Fresh install
npm install

# Build (note: use tauri:build, not tauri build)
npm run tauri:build
```

**Expected Output:**
- Build completes successfully
- Executable in `src-tauri/target/release/`
- Installer in `src-tauri/target/release/bundle/`

**Checkpoint:** ✅ Local build successful

---

### Step 2.2: Test Executable

```bash
# Run the built executable
.\src-tauri\target\release\rpg-desktop.exe

# Test:
# 1. Open application
# 2. Create new package
# 3. Add some datatypes
# 4. Preview rendering
# 5. Validate package
# 6. Save package
# 7. Load saved package
```

**Checkpoint:** ✅ Executable works correctly

---

### Step 2.3: Test CLI Binary

The build also includes a CLI tool for validation and rendering from command line.

**CLI Location:**
```
D:\workspaces\prompt-gen-desktop\src-tauri\target\release\rpg-cli.exe
```

**Navigate to CLI:**
```powershell
cd D:\workspaces\prompt-gen-desktop\src-tauri\target\release
```

**Test 1 - Validate Package:**
```powershell
.\rpg-cli.exe validate D:\workspaces\prompt-gen-spec\packages\featured.common\featured.common.yaml
```

**Expected:** `✓ VALIDATION PASSED`

**Test 2 - Package Info:**
```powershell
.\rpg-cli.exe info D:\workspaces\prompt-gen-spec\packages\featured.common\featured.common.yaml
```

**Expected:** Shows package structure with 7 namespaces

**Test 3 - Render Prompts:**
```powershell
.\rpg-cli.exe render D:\workspaces\prompt-gen-spec\packages\featured.common\featured.common.yaml "common.fantasy:fantasy_scene" --seed 12345 --count 3
```

**Expected:** Renders 3 fantasy scene prompts with deterministic output

**Test 4 - Test Determinism:**
```powershell
# Run twice with same seed - should produce identical output
.\rpg-cli.exe render D:\workspaces\prompt-gen-spec\packages\featured.common\featured.common.yaml "common.fantasy:fantasy_scene" --seed 12345 --count 1
.\rpg-cli.exe render D:\workspaces\prompt-gen-spec\packages\featured.common\featured.common.yaml "common.fantasy:fantasy_scene" --seed 12345 --count 1
```

**Expected:** Both outputs are identical

**Available Prompt Sections:**
- `common.fantasy:fantasy_scene` - Fantasy creatures and locations
- `common.characters:character_portrait` - Character descriptions
- `common.nature:landscape_scene` - Landscape scenes
- `common.lighting:lighting_scene` - Lighting and atmosphere

**CLI Commands:**
- `validate <file> [--warnings] [--verbose]` - Validate package
- `info <file>` - Show package information
- `render <file> <section> [--seed N] [--count N]` - Render prompts

**Checkpoint:** ✅ CLI works correctly

---

## Phase 3: CI/CD Setup (GitHub Actions)

### Step 3.1: Create .github/workflows Directory

```bash
cd D:\workspaces\prompt-gen-desktop
mkdir .github\workflows
```

---

### Step 3.2: Create Build Workflow

**File:** `.github/workflows/build.yml`

**Content:** (See attached complete workflow)

**Key Features:**
- Builds on Windows, macOS, Linux
- Runs tests
- Creates installers
- Uploads artifacts
- Caches dependencies

---

### Step 3.3: Create Release Workflow

**File:** `.github/workflows/release.yml`

**Key Features:**
- Triggers on version tags (v*.*.*)
- Builds all platforms
- Creates GitHub release
- Uploads binaries as release assets
- Publishes npm package (if configured)

---

### Step 3.4: Commit Workflows

```bash
git add .github/
git commit -m "ci: add GitHub Actions workflows

- Build workflow for PR/push validation
- Release workflow for tagged versions
- Multi-platform builds (Windows, macOS, Linux)
- Automated testing and artifact upload"

git push
```

**Checkpoint:** ✅ CI/CD workflows configured

---

## Phase 4: Test CI/CD

### Step 4.1: Trigger Build

```bash
# Make a small change to trigger CI
echo "# Test" >> README.md
git add README.md
git commit -m "test: trigger CI build"
git push
```

**Check:**
1. Go to GitHub → Actions tab
2. Watch build progress
3. Verify all platforms build successfully
4. Download artifacts to test

**Checkpoint:** ✅ CI builds working

---

### Step 4.2: Create Test Release

```bash
# Tag a pre-release version
git tag v1.0.0-rc1
git push origin v1.0.0-rc1
```

**Check:**
1. GitHub Actions → Release workflow runs
2. GitHub → Releases → v1.0.0-rc1 appears
3. All platform binaries attached
4. Download and test each platform

**Checkpoint:** ✅ Release process working

---

## Phase 5: Documentation Updates

### Step 5.1: Update Main Spec Repository

**File:** `prompt-gen-spec/README.md`

Add link to new repository:
```markdown
## Related Repositories

- **[prompt-gen-desktop](https://github.com/[your-org]/prompt-gen-desktop)** - Desktop authoring tool (v1.0.0)
- **prompt-gen-cli** - Command-line tools (coming in M11)
- **prompt-gen-web** - Web application (coming in M12)
```

**File:** `prompt-gen-spec/DEVELOPMENT_PLAN.md`

Update M10 status:
```markdown
### Milestone 10: Desktop App Extraction & CI/CD
**Status:** ✅ COMPLETE
**Completed:** 2025-12-XX
```

---

### Step 5.2: Update Project Status Files

Update these files in `prompt-gen-spec`:
- QUICK_STATUS.md
- PROJECT_STATUS_SUMMARY.md
- INDEX.md
- reference-impl/STATUS.md

---

## Phase 6: npm Package (Optional for v1.0.0)

**Note:** CLI tools are planned for M11. For v1.0.0, focus on desktop app.

If you want to publish CLI separately:

```bash
# In prompt-gen-desktop
npm publish --access public
```

---

## Phase 7: Final Release

### Step 7.1: Tag v1.0.0

```bash
cd D:\workspaces\prompt-gen-desktop

# Create final release tag
git tag -a v1.0.0 -m "Release v1.0.0 - First Public Release

Features:
- Visual package editor
- Real-time validation
- Live preview
- Rulebook support
- Dependency management
- Cross-package references
- Jump-to-error navigation

Documentation: https://github.com/[your-org]/prompt-gen-spec
"

git push origin v1.0.0
```

**Checkpoint:** ✅ v1.0.0 released

---

### Step 7.2: Make Repository Public

1. Go to GitHub repository settings
2. Scroll to "Danger Zone"
3. Click "Change visibility"
4. Make public
5. Confirm

**Checkpoint:** ✅ Repository public

---

### Step 7.3: Announcement

Create announcement in:
- GitHub Discussions
- README badges
- Social media (optional)

---

## Reporting Back: Checkpoints

After completing each phase, report back with:

✅ **Phase 1 Complete:**
- Repository created: [URL]
- Initial commit pushed
- README and files in place

✅ **Phase 2 Complete:**
- Local build successful
- Executable tested and working
- Ready for CI/CD

✅ **Phase 3 Complete:**
- Workflows committed
- CI/CD configured

✅ **Phase 4 Complete:**
- CI builds working
- Test release successful
- Binaries downloaded and tested

✅ **Phase 5 Complete:**
- Documentation updated
- Cross-references working

✅ **Phase 6 Complete (Optional):**
- npm package published

✅ **Phase 7 Complete:**
- v1.0.0 tagged and released
- Repository public
- Announcement made

---

## Troubleshooting

### Rulebooks Not Showing After Loading Package

**Symptom:** After loading a YAML package file, rulebooks are not displayed in the UI even though they exist in the file.

**Possible Causes:**

1. **Package doesn't contain rulebooks** - Many older package files (pre-M9) don't have rulebooks yet
2. **Wrong structure** - Rulebooks must be under `namespaces.<namespace_id>.rulebooks` as a HashMap/object
3. **Array vs HashMap** - Rulebooks must be a HashMap (object with keys), not an array

**How to Check:**

Open your YAML file and verify the structure matches this:
```yaml
namespaces:
  your_namespace:
    # ... other content ...
    rulebooks:          # Must be an object, not array
      rulebook_id:      # The key is the rulebook ID
        name: "My Rulebook"
        description: "Description"
        entry_points:
          - prompt_section: "namespace:section_name"
            weight: 1.0
        batch_variety: false
        context_defaults: {}
```

**Wrong structure (array):**
```yaml
rulebooks:           # ❌ At package level
  - name: "My Rulebook"
    # ...
```

**Correct structure (HashMap in namespace):**
```yaml
namespaces:
  main:
    rulebooks:       # ✅ Inside namespace
      my_rulebook:   # ✅ Key-value pairs
        name: "My Rulebook"
        # ...
```

**To test:**
- Load `test-packages-for-comparison/rulebook-test.yaml` from your installation
- This file has correct rulebook structure
- If this shows rulebooks but your file doesn't, compare the structures

**Fix:**
1. Open your YAML file in a text editor
2. Ensure rulebooks are inside each namespace object (not at package root)
3. Ensure rulebooks is an object with named keys, not an array
4. Save and reload in the application

### Open Package Button Does Nothing

**Symptom:** Clicking "Open Package" in production build doesn't show file dialog

**Cause:** Missing Tauri v2 capabilities file for dialog plugin permissions

**Fix:** Create `src-tauri/capabilities/default.json`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default permissions for RPG Desktop",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:window:default",
    "core:window:allow-close",
    "core:window:allow-minimize",
    "core:window:allow-maximize",
    "dialog:allow-open",
    "dialog:allow-save",
    "dialog:allow-message",
    "dialog:allow-ask",
    "dialog:allow-confirm"
  ]
}
```

Then rebuild: `npm run tauri:build`

---

### Build Fails on GitHub Actions

**Check:**
- Rust version in workflow matches local
- Node version in workflow matches local
- Platform-specific dependencies installed
- Secrets configured (code signing certificates)

### Tests Fail in CI

**Check:**
- All test files copied
- Test data/fixtures included
- Environment variables set

### Artifacts Missing

**Check:**
- Artifact upload paths correct
- Build completed successfully
- Artifact retention settings

---

## Estimated Timeline

- **Phase 1 (Setup):** 2-3 hours
- **Phase 2 (Verify):** 1 hour
- **Phase 3 (CI/CD):** 3-4 hours
- **Phase 4 (Test):** 2-3 hours
- **Phase 5 (Docs):** 1-2 hours
- **Phase 6 (npm):** 1 hour (optional)
- **Phase 7 (Release):** 1 hour

**Total:** 11-16 hours over 2-3 days

---

## Success Criteria

- ✅ Standalone repository created and public
- ✅ CI/CD builds all platforms automatically
- ✅ Pre-built binaries available for download
- ✅ All tests passing in CI
- ✅ v1.0.0 tagged and released
- ✅ Documentation updated and linked
- ✅ README reflects new repository structure

---

## Next Steps After M10

**M11: Web Application** (Week 20-22)
- Browser-based package editor
- Cloud storage integration
- Feature parity with desktop

**M12: Marketplace** (Week 23-25)
- Package discovery and distribution
- Cryptographic signing
- Namespace ownership

---

## Questions to Report Back

1. ✅ **Phase 1:** Repository created? Initial push successful?
2. ✅ **Phase 2:** Local build works? Executable runs correctly?
3. ✅ **Phase 3:** Workflows committed? Any errors?
4. ✅ **Phase 4:** CI builds passing? Artifacts downloadable?
5. ✅ **Phase 5:** Documentation updated? Links working?
6. ✅ **Phase 6:** npm package published (if doing)?
7. ✅ **Phase 7:** Release live? Repository public?

**Report back after each phase and I'll help with any issues!**

---

**Status:** Ready to Execute  
**Start:** Begin with Phase 1 when ready  
**Support:** Report back with progress and any blockers


