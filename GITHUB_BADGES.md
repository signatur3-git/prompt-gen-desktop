# GitHub Actions Badges

Add these badges to your README.md to show build status:

## For README.md

Add these near the top of your README.md:

```markdown
# Random Prompt Generator - Desktop Application

[![Build](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml/badge.svg)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml)
[![Release](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/release.yml/badge.svg)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Desktop authoring tool for creating and editing Random Prompt Generator (RPG) packages.
```

## Alternative Badge Styles

**Flat Square:**
```markdown
[![Build](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml/badge.svg?style=flat-square)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml)
```

**With Branch:**
```markdown
[![Build](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml)
```

## Additional Useful Badges

**Version:**
```markdown
![Version](https://img.shields.io/github/v/release/signatur3-git/prompt-gen-desktop)
```

**Downloads:**
```markdown
![Downloads](https://img.shields.io/github/downloads/signatur3-git/prompt-gen-desktop/total)
```

**Last Commit:**
```markdown
![Last Commit](https://img.shields.io/github/last-commit/signatur3-git/prompt-gen-desktop)
```

**Issues:**
```markdown
![Issues](https://img.shields.io/github/issues/signatur3-git/prompt-gen-desktop)
```

## Full Example

```markdown
# Random Prompt Generator - Desktop Application

[![Build](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml/badge.svg)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/build.yml)
[![Release](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/release.yml/badge.svg)](https://github.com/signatur3-git/prompt-gen-desktop/actions/workflows/release.yml)
![Version](https://img.shields.io/github/v/release/signatur3-git/prompt-gen-desktop)
![Downloads](https://img.shields.io/github/downloads/signatur3-git/prompt-gen-desktop/total)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Desktop authoring tool for creating and editing Random Prompt Generator (RPG) packages.
```

Note: These badges will only work correctly after:
1. You push the workflows to GitHub
2. At least one workflow run completes
3. You create your first release (for the version badge)

