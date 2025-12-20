# CLI Guide: Random Prompt Generator CLI Tool

**Version:** 0.1.0  
**Updated:** 2025-12-17

---

## Overview

The `rpg-cli` tool provides command-line access to package validation, rendering, and inspection. It's designed for developers, CI/CD pipelines, and quick testing.

---

## Installation

### Building from Source

```bash
cd reference-impl/rpg-desktop/src-tauri
cargo build --release --bin rpg-cli
```

The binary will be at: `target/release/rpg-cli.exe` (Windows) or `target/release/rpg-cli` (Unix)

### Adding to PATH

**Windows:**
```powershell
# Add to your PATH
$env:Path += ";D:\path\to\reference-impl\rpg-desktop\src-tauri\target\release"
```

**Linux/Mac:**
```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/reference-impl/rpg-desktop/src-tauri/target/release"
```

---

## Commands

### `rpg-cli validate`

Validates a package file for errors and warnings.

**Usage:**
```bash
rpg-cli validate <FILE> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to package file (YAML or JSON)

**Options:**
- `-w, --warnings` - Show warnings in addition to errors
- `-v, --verbose` - Show detailed validation progress
- `-h, --help` - Print help

**Examples:**

Basic validation:
```bash
rpg-cli validate my-package.yaml
```

With warnings:
```bash
rpg-cli validate my-package.yaml --warnings
```

Verbose output:
```bash
rpg-cli validate my-package.yaml --verbose --warnings
```

**Exit Codes:**
- `0` - Package is valid
- `1` - Package has errors

**Output Example (Success):**
```
============================================================
Validating: my-package.yaml
============================================================

✓ VALIDATION PASSED

────────────────────────────────────────────────────────────
Result: VALID
Warnings: 0
────────────────────────────────────────────────────────────
```

**Output Example (Errors):**
```
✗ VALIDATION FAILED

Errors (2)

  1. Reference not found: 'missing_type' in test:prompt.ref1
  
  2. Min must be <= Max: min=5, max=2 in test:prompt.colors

────────────────────────────────────────────────────────────
Result: INVALID
Errors: 2
────────────────────────────────────────────────────────────
```

---

### `rpg-cli info`

Displays package information and structure.

**Usage:**
```bash
rpg-cli info <FILE>
```

**Arguments:**
- `<FILE>` - Path to package file (YAML or JSON)

**Examples:**

Show package info:
```bash
rpg-cli info my-package.yaml
```

**Output Example:**
```
============================================================
Package Information: my-package.yaml
============================================================

Package: my.package v1.0.0
Description: A sample package for testing
Authors: John Doe, Jane Smith

Namespaces: 2

  └─ test
     ├─ 5 datatype(s)
     ├─ 3 promptsection(s)
     ├─ 2 separator set(s)
     └─ 1 rule(s)

  └─ common
     ├─ 10 datatype(s)
     ├─ 5 promptsection(s)
     ├─ 3 separator set(s)
     └─ 0 rule(s)

Dependencies: 1
  └─ base.colors (v1.0.0)

────────────────────────────────────────────────────────────
```

---

### `rpg-cli render`

Renders a prompt section with a given seed.

**Usage:**
```bash
rpg-cli render <FILE> <SECTION> [OPTIONS]
```

**Arguments:**
- `<FILE>` - Path to package file (YAML or JSON)
- `<SECTION>` - Prompt section to render (format: `namespace:section`)

**Options:**
- `-s, --seed <SEED>` - Seed for deterministic rendering (default: 42)
- `-c, --count <COUNT>` - Number of prompts to generate (default: 1)
- `-h, --help` - Print help

**Examples:**

Single render with default seed:
```bash
rpg-cli render my-package.yaml test:scene
```

Custom seed:
```bash
rpg-cli render my-package.yaml test:scene --seed 12345
```

Generate 10 variations:
```bash
rpg-cli render my-package.yaml test:scene --seed 100 --count 10
```

**Output Example (Single):**
```
============================================================
Rendering: test:scene
============================================================

→ Loading package from my-package.yaml
✓ Package loaded

Seed: 42

  A mysterious forest with ancient trees under moonlight

────────────────────────────────────────────────────────────
Render time: 0.00ms
```

**Output Example (Batch):**
```
#1 ──────────────────────────────────────────── (Seed: 100)

  A mysterious forest with ancient trees under moonlight

#2 ──────────────────────────────────────────── (Seed: 101)

  A bustling marketplace in a medieval town at sunset

#3 ──────────────────────────────────────────── (Seed: 102)

  A serene lake surrounded by mountains at dawn

────────────────────────────────────────────────────────────
Total: 3 prompts rendered
────────────────────────────────────────────────────────────
```

---

## Common Workflows

### Pre-Commit Validation

Validate all packages before committing:

```bash
# Validate all packages in a directory
for file in packages/*.yaml; do
    rpg-cli validate "$file" || exit 1
done
```

### CI/CD Integration

GitHub Actions example:

```yaml
name: Validate Packages

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Build CLI
        run: |
          cd reference-impl/rpg-desktop/src-tauri
          cargo build --release --bin rpg-cli
      - name: Validate packages
        run: |
          for file in packages/*.yaml; do
            ./target/release/rpg-cli validate "$file" --warnings || exit 1
          done
```

### Testing Package Changes

Quick test after editing:

```bash
# Validate
rpg-cli validate my-package.yaml --warnings

# Check what changed
rpg-cli info my-package.yaml

# Test a few renders
rpg-cli render my-package.yaml test:scene --count 5
```

### Generating Dataset

Generate many prompts for AI training:

```bash
# Generate 1000 variations with sequential seeds
rpg-cli render my-package.yaml test:scene --seed 1 --count 1000 > dataset.txt
```

---

## Tips & Tricks

### Deterministic Testing

Use specific seeds for reproducible results:

```bash
# Always same output with same seed
rpg-cli render pkg.yaml test:scene --seed 42
```

### Quick Package Stats

```bash
# Count components
rpg-cli info my-package.yaml | grep -E "datatype|promptsection|separator|rule"
```

### Validation in Scripts

```bash
# Check exit code
if rpg-cli validate pkg.yaml; then
    echo "Package is valid!"
else
    echo "Validation failed!"
fi
```

### Colored Output

The CLI uses colors by default. To disable (for logs):

```bash
# Redirect to file (colors stripped automatically in most terminals)
rpg-cli validate pkg.yaml > validation.log 2>&1
```

---

## Troubleshooting

### "Package not found"

**Problem:** File path is wrong or file doesn't exist

**Solution:** Use absolute paths or verify file exists
```bash
rpg-cli validate ./path/to/package.yaml
```

### "Failed to parse YAML"

**Problem:** YAML syntax error in package

**Solution:** Use a YAML validator or check line numbers in error
```bash
# The error will show which line has the issue
rpg-cli validate broken.yaml
```

### "Reference not found"

**Problem:** Template references a non-existent datatype/promptsection

**Solution:** Check the error message for suggestions
```
Reference not found: 'colours' in test:prompt.color
  Suggestion: Did you mean 'test:colors' (datatype)?
```

### "Circular reference detected"

**Problem:** Nested promptsections create a loop (A→B→C→A)

**Solution:** Check the reported chain and break the cycle
```
Circular reference detected: test:a → test:b → test:c → test:a
```

---

## Advanced Usage

### Batch Processing

Process multiple packages:

```bash
# Validate all YAML files
find packages/ -name "*.yaml" -exec rpg-cli validate {} \;

# Generate samples from all packages
for pkg in packages/*.yaml; do
    echo "=== $pkg ==="
    rpg-cli render "$pkg" test:main --count 3
done
```

### Automation Scripts

Create wrapper scripts:

```bash
#!/bin/bash
# validate-all.sh

FAILED=0
for pkg in packages/*.yaml; do
    echo "Validating $pkg..."
    if ! rpg-cli validate "$pkg" --warnings; then
        FAILED=$((FAILED + 1))
    fi
done

echo ""
if [ $FAILED -eq 0 ]; then
    echo "✓ All packages valid!"
    exit 0
else
    echo "✗ $FAILED package(s) failed validation"
    exit 1
fi
```

---

## See Also

- [Validation Guide](./validation-guide.md) - Understanding validation rules
- [Error Reference](./error-reference.md) - All error types explained
- [Package Format](../architecture/components.md) - Package structure reference

---

**Questions or issues?** Check the [main documentation](../../README.md) or file an issue.

