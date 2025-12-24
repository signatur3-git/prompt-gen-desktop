# 📝 Quick Reference: Releasing a New Version

## 🌿 Feature Branch Workflow

### Working on Features (Lightweight CI)

```bash
# Create feature branch
git checkout -b feature/my-feature

# Make changes and commit
git add .
git commit -m "feat: add my feature"

# Push to GitHub (triggers quick checks only: lint + tests)
git push origin feature/my-feature

# When ready, merge locally (fast-forward only)
git checkout main
git merge --ff-only feature/my-feature
git push origin main  # Triggers full build on all platforms

# Clean up feature branch
git branch -d feature/my-feature
git push origin --delete feature/my-feature
```

**Note:** Feature branches only run lint + tests (< 5 min). Main branch runs full multi-platform builds (~ 20-30 min).

---

## 🚀 Release Checklist

### 1. Update Version (3 files)

```bash
# Edit these files to update version number:
# 1. package.json          → "version": "X.Y.Z"
# 2. src-tauri/Cargo.toml  → version = "X.Y.Z"
# 3. src-tauri/tauri.conf.json → "version": "X.Y.Z"
```

### 2. Commit and Tag

```bash
cd D:\workspaces\prompt-gen-desktop

# Stage and commit
git add -A
git commit -m "chore: bump version to X.Y.Z"

# Create annotated tag
git tag -a vX.Y.Z -m "Release vX.Y.Z - Brief description"

# Push everything
git push origin main --tags
```

### 3. Create GitHub Release

1. Go to: https://github.com/signatur3-git/prompt-gen-desktop/releases/new
2. Select tag: `vX.Y.Z`
3. Add release notes
4. Publish (triggers build workflow)

### 4. Test Downloaded Binaries

```powershell
# Clear cache first!
Remove-Item -Recurse -Force $env:LOCALAPPDATA\com.rpg.desktop\EBWebView

# Run binary and verify version in app header
```

---

## 🎯 Version Numbering

Follow semantic versioning:
- **Major (X)**: Breaking changes
- **Minor (Y)**: New features, backwards compatible  
- **Patch (Z)**: Bug fixes only

Examples:
- `1.0.0` → First stable release
- `1.0.1` → Bug fixes
- `1.1.0` → New features
- `2.0.0` → Breaking changes

---

## ✅ Pre-Release Testing

Before creating a release:

```bash
# 1. Lint check
npm run lint

# 2. Run tests
npm run test:run

# 3. Build locally
npm run tauri:build

# 4. Test local build
.\src-tauri\target\release\rpg-desktop.exe
```

---

## 📋 What Gets Auto-Synced

✅ **Frontend UI version** - Reads from package.json via `useVersion()` composable  
❌ **Cargo/Tauri versions** - Must be manually updated (no way to share at compile time)

This is why we update 3 files, but at least the UI is automatic! 🎉

---

## 🔗 Useful Links

- **Releases:** https://github.com/signatur3-git/prompt-gen-desktop/releases
- **Actions:** https://github.com/signatur3-git/prompt-gen-desktop/actions
- **Package Manager:** https://prompt-gen-marketplace-production.up.railway.app

---

*Keep this document handy for quick releases!*

