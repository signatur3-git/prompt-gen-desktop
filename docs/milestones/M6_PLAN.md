# M6 Plan: Package Validation & CLI Tool

**Created:** 2025-12-17  
**Status:** 🔄 **IN PROGRESS**  
**Goal:** Comprehensive package validation and CLI tool for developer experience

---

## Overview

**Milestone 6** focuses on tooling to make package authoring smooth and error-free. We'll build a comprehensive validator that catches common mistakes and a CLI tool for common operations.

---

## Goals

1. **Comprehensive Package Validator**
   - Schema validation (structure, types)
   - Semantic validation (references exist, logic sound)
   - Best practices (naming, organization)
   - Helpful error messages

2. **CLI Tool**
   - `validate` - Check package for errors
   - `render` - Render a prompt section with seed
   - `test` - Run test scenarios
   - `info` - Show package information

3. **Developer Experience**
   - Clear, actionable error messages
   - Quick feedback loop
   - Documentation for common issues

---

## Current State

**What We Have:**
- ✅ Basic YAML parsing (serde-based)
- ✅ Data model validation (Rust types)
- ✅ Desktop app GUI
- ⚠️ Very limited validation (only schema)

**What's Missing:**
- ❌ Semantic validation (references, rules logic)
- ❌ Best practices checks
- ❌ CLI tool
- ❌ Helpful error messages
- ❌ Validation guide documentation

---

## Implementation Approach

### Phase 1: Enhanced Validation (Week 1)

**Goal:** Comprehensive package validation in Rust

**Tasks:**
1. Create validation module with error types
2. Implement schema validation
3. Implement semantic validation
4. Implement best practices checks
5. Add helpful error messages
6. Unit tests for all validation rules

**Validations to Implement:**

#### Schema Validation
- ✅ Package structure (id, version, namespaces)
- ✅ Required fields present
- ✅ Types correct (string, number, etc.)
- ⏳ **Enhanced:** More detailed checks

#### Semantic Validation
- [ ] All references resolve to existing datatypes/promptsections
- [ ] No circular references in nested templates
- [ ] Tag filters reference valid tags
- [ ] Separator sets referenced actually exist
- [ ] Rules reference existing tags/context keys
- [ ] Min <= Max for multiplicity
- [ ] Unique constraint makes sense (enough values)

#### Best Practices
- [ ] Naming conventions (lowercase, alphanumeric, hyphens)
- [ ] No duplicate IDs within namespace
- [ ] Reasonable weights (sum doesn't overflow)
- [ ] Template references are clear
- [ ] No unused datatypes/promptsections

**Deliverable:** `src-tauri/src/validator/` module

---

### Phase 2: CLI Tool (Week 2)

**Goal:** Command-line tool for package operations

**Tasks:**
1. Set up CLI framework (clap)
2. Implement `validate` command
3. Implement `render` command
4. Implement `info` command
5. Implement `test` command (if time allows)
6. Documentation

**Commands:**

#### `rpg-cli validate <package.yaml>`
```
Validating package: test-packages/lists-test.yaml
✓ Schema validation passed
✓ Semantic validation passed
⚠ Best practices: 2 warnings
  - Warning: Datatype 'unused_colors' is defined but never referenced
  - Warning: PromptSection 'deprecated' uses old filter syntax

Summary: Valid package with 2 warnings
```

#### `rpg-cli render <package.yaml> <section> [--seed=42]`
```
Loading package: test-packages/lists-test.yaml
Rendering: test:flying_creatures_with_article
Seed: 42

Output:
  an owl, eagle and swan

Selected values:
  article: an
  creatures: [owl, eagle, swan]

Render time: 2ms
```

#### `rpg-cli info <package.yaml>`
```
Package: test.lists v1.0.0
Description: Lists and Separator Sets Test

Namespaces: 1
  - test (5 datatypes, 6 prompt sections, 2 separator sets, 1 rule)

Dependencies: 0
```

#### `rpg-cli test <package.yaml>` (Optional)
```
Running tests for: test-packages/lists-test.yaml

test:simple_list ............... PASS (10/10 renders successful)
test:natural_list .............. PASS (10/10 renders successful)
test:unique_colors ............. PASS (10/10 renders successful, all unique)
test:optional_adjectives ....... PASS (min=0 working, outputs varied)
test:flying_creatures .......... PASS (article computation correct)

Summary: 5/5 tests passed
```

**Deliverable:** `src-tauri/src/cli.rs` or separate CLI binary

---

## Technical Decisions

### TD-001: CLI Architecture

**Options:**
1. Integrate into existing Tauri app (add CLI mode)
2. Separate CLI binary that uses core library
3. Node.js CLI wrapper around Tauri commands

**Decision:** Option 2 - Separate Rust CLI binary
- Shares core validation/rendering code with desktop app
- Can be distributed standalone
- Better performance than Node.js wrapper
- Clean separation of concerns

**Implementation:**
```toml
# Cargo.toml
[[bin]]
name = "rpg-desktop"
path = "src/main.rs"

[[bin]]
name = "rpg-cli"
path = "src/cli.rs"
```

### TD-002: Error Message Format

**Decision:** Structured errors with context
```rust
pub enum ValidationError {
    ReferenceNotFound {
        reference: String,
        defined_in: String,
        line: Option<usize>,
    },
    CircularReference {
        chain: Vec<String>,
    },
    // ...
}

impl ValidationError {
    pub fn to_user_message(&self) -> String {
        // Human-friendly messages with suggestions
    }
}
```

### TD-003: Validation Levels

**Decision:** Three levels
1. **ERROR** - Must fix (package won't work)
2. **WARNING** - Should fix (best practices)
3. **INFO** - Nice to know (suggestions)

---

## Test Strategy

### Validation Tests

**Good Packages (Should Pass):**
- minimal.yaml
- article-test.yaml
- tag-filter-test.yaml
- nested-test.yaml
- complex-tags-test.yaml
- lists-test.yaml

**Bad Packages (Should Fail):**
- missing-reference.yaml - References non-existent datatype
- circular-refs.yaml - A → B → C → A nesting
- invalid-filter.yaml - Malformed tag expression
- min-max-reversed.yaml - min > max
- missing-separator.yaml - References non-existent separator set
- duplicate-ids.yaml - Same ID used twice in namespace

**Create:** `test-packages/invalid/` directory with error cases

### CLI Tests

- [ ] Validate command works on good/bad packages
- [ ] Render command produces correct output
- [ ] Info command shows package details
- [ ] Error messages are clear and helpful
- [ ] Exit codes correct (0 = success, 1 = error)

---

## Success Criteria

- [ ] Validator catches all common errors
- [ ] Error messages are clear and actionable
- [ ] CLI commands work as expected
- [ ] Can validate/render any package from CLI
- [ ] Documentation complete
- [ ] All tests passing
- [ ] Developer experience is smooth

---

## Estimated Time

**Phase 1 (Validation):** 8-12 hours
- Module setup: 2h
- Schema validation: 2h
- Semantic validation: 4h
- Best practices: 2h
- Error messages: 2h

**Phase 2 (CLI):** 8-10 hours
- CLI setup: 2h
- Validate command: 2h
- Render command: 2h
- Info command: 1h
- Test command: 2h (optional)
- Documentation: 1h

**Total:** 16-22 hours (2-3 weeks part-time)

---

## Dependencies

**Rust Crates:**
- `clap` - CLI argument parsing
- `colored` - Terminal colors for output
- `indicatif` - Progress bars/spinners
- `thiserror` - Error handling

**No new external dependencies for validation** - uses existing serde/core types

---

## Documentation Deliverables

1. **Validation Guide** (`docs/tools/validation-guide.md`)
   - All validation rules explained
   - Examples of common errors
   - How to fix them

2. **CLI Guide** (`docs/tools/cli-guide.md`)
   - Installation
   - All commands with examples
   - Common workflows

3. **Error Reference** (`docs/tools/error-reference.md`)
   - All error codes
   - Causes and solutions

---

## Next Steps

**Immediate:**
1. Create validation module structure
2. Define ValidationError types
3. Implement first validation rules
4. Write tests for validation

**This Week:**
- Complete Phase 1 (validation)
- Start Phase 2 (CLI basics)

**Next Week:**
- Complete Phase 2 (CLI)
- Documentation
- User testing

---

## Questions to Resolve

1. **Q:** Should CLI be in same binary as desktop app?
   **A:** No, separate binary sharing core library (TD-001)

2. **Q:** What validation rules are most important?
   **A:** References exist, no circular deps, filters valid (prioritize)

3. **Q:** How detailed should error messages be?
   **A:** Very detailed - include line numbers, suggestions, examples

4. **Q:** Should we validate tag expression syntax?
   **A:** Yes! Parse and validate tag expressions (use M5 parser)

---

**Status:** Ready to start implementation! 🚀

**Start with:** Creating the validator module structure and defining error types.

