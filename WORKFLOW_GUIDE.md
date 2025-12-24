# 🔄 Development Workflow Guide

## Overview

This repository uses a **feature branch workflow** with **fast-forward merges only** to maintain linear history.

---

## 🌿 Branch Strategy

### Main Branch
- **`main`** - Production-ready code
- Always stable and deployable
- Only accepts fast-forward merges
- Triggers full CI/CD (all platforms)

### Feature Branches
- **`feature/*`** - New features
- **`fix/*`** - Bug fixes
- **`chore/*`** - Maintenance tasks

---

## 💻 Day-to-Day Workflow

### 1. Start New Feature

```bash
# Make sure main is up to date
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/marketplace-sync
```

### 2. Work on Feature

```bash
# Make changes
# ... edit files ...

# Commit frequently
git add .
git commit -m "feat: add marketplace sync"

# Push to GitHub (triggers lightweight CI)
git push origin feature/marketplace-sync
```

**What runs on feature branches:**
- ✅ Frontend lint + tests
- ✅ Rust clippy + tests
- ❌ No multi-platform builds
- ⏱️ ~5 minutes

### 3. Merge to Main (Option A: Local Fast-Forward)

```bash
# Ensure feature branch is up to date
git checkout main
git pull origin main
git checkout feature/marketplace-sync
git rebase main  # If main moved forward

# Fast-forward merge
git checkout main
git merge --ff-only feature/marketplace-sync

# If ff-only fails, you need to rebase first
# Push to trigger full build
git push origin main

# Clean up
git branch -d feature/marketplace-sync
git push origin --delete feature/marketplace-sync
```

### 3. Merge to Main (Option B: Pull Request)

```bash
# Push feature branch
git push origin feature/marketplace-sync

# On GitHub:
# 1. Create Pull Request from feature/marketplace-sync → main
# 2. Review changes
# 3. Click "Merge" (configured for fast-forward)
# 4. Delete branch on GitHub
```

**What runs on main branch:**
- ✅ Frontend lint + tests
- ✅ Rust clippy + tests
- ✅ Multi-platform builds (Windows, Linux, macOS)
- ⏱️ ~20-30 minutes

---

## 🚀 Release Workflow

### Creating a Release

```bash
# Ensure you're on latest main
git checkout main
git pull origin main

# Update version in 3 files
# - package.json
# - src-tauri/Cargo.toml
# - src-tauri/tauri.conf.json

# Commit version bump
git add -A
git commit -m "chore: bump version to 1.0.1"

# Create tag
git tag -a v1.0.1 -m "Release v1.0.1 - Bug fixes"

# Push everything
git push origin main --tags
```

On GitHub:
1. Go to Releases → New Release
2. Select tag `v1.0.1`
3. Add release notes
4. Publish (triggers release builds)

---

## 🔍 CI/CD Costs Summary

| Workflow | Trigger | Platforms | Duration | Use Case |
|----------|---------|-----------|----------|----------|
| **Feature CI** | Push to `feature/*` | Ubuntu only | ~5 min | Development |
| **Build** | Push to `main` | 4 platforms | ~25 min | Integration |
| **Release** | GitHub Release | 4 platforms | ~30 min | Distribution |

---

## 🛡️ Best Practices

### ✅ DO
- Create feature branches for all changes
- Use descriptive branch names (`feature/oauth-flow`, `fix/clipboard-bug`)
- Commit frequently with clear messages
- Rebase on main before merging
- Use `--ff-only` to enforce linear history
- Test locally before pushing

### ❌ DON'T
- Commit directly to main
- Create merge commits (use rebase instead)
- Push untested code
- Leave stale feature branches

---

## 🔧 Git Configuration

Enforce fast-forward only merges locally:

```bash
# Set default merge strategy
git config merge.ff only

# Now `git merge` will fail if fast-forward isn't possible
```

---

## 📋 Common Commands

```bash
# Check current branch
git branch

# See what changed
git status
git diff

# Sync with remote
git fetch origin
git pull origin main

# Rebase feature on latest main
git checkout feature/my-feature
git rebase main

# Abort rebase if needed
git rebase --abort

# Force push after rebase (only on feature branches!)
git push origin feature/my-feature --force-with-lease

# Delete local branch
git branch -d feature/my-feature

# Delete remote branch
git push origin --delete feature/my-feature
```

---

## 🆘 Troubleshooting

### "Fatal: Not possible to fast-forward"

Your branch has diverged from main. Rebase first:

```bash
git checkout feature/my-feature
git fetch origin
git rebase origin/main
# Resolve conflicts if any
git push origin feature/my-feature --force-with-lease
```

### "Failed to push some refs"

Someone else pushed to main. Pull and try again:

```bash
git pull origin main --rebase
git push origin main
```

### Feature Branch CI Failed

Fix issues locally, then push:

```bash
# Run checks locally first
npm run lint
npm run test:run
cd src-tauri && cargo clippy && cargo test

# Push fix
git add .
git commit -m "fix: resolve CI issues"
git push origin feature/my-feature
```

---

## 🔗 Links

- **Actions:** https://github.com/signatur3-git/prompt-gen-desktop/actions
- **Releases:** https://github.com/signatur3-git/prompt-gen-desktop/releases
- **Pull Requests:** https://github.com/signatur3-git/prompt-gen-desktop/pulls

---

*Keep development fast and CI costs low!* ⚡

