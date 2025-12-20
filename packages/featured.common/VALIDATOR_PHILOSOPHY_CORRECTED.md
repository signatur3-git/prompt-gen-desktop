# Validator Philosophy: Library Packages & Usage Detection

**Date:** 2025-12-18  
**Issue:** Current validator warnings about "unused" datatypes  
**Root Question:** Should packages be allowed to have unused datatypes?  
**Answer:** **YES** - For library packages!

---

## The Core Insight

**User's correct observation:**
> "Shouldn't the validator be satisfied if the exact namespaced datatype is found at least once anywhere? Why does it need to be found several times?"

**YES!** The validator should only check:
- ✅ Is datatype used? (at least once, anywhere)
- ❌ NOT: How many times is it used?

**And furthermore:**
> "Do we want to allow authors to provide packages that are pure collections of datatypes without any other entity types for mix and matching scenarios?"

**ABSOLUTELY YES!** This is a valid and important use case.

---

## Package Types & Validation Philosophy

### Use Case 1: Library Packages

**Pure datatype collections for reuse:**

```yaml
# colors-library.yaml
metadata:
  id: colors-library
  version: 1.0.0
  type: library  # ← Indicates this is a library

namespaces:
  colors.warm:
    datatypes:
      reds: [crimson, scarlet, ruby, ...]
      oranges: [tangerine, amber, copper, ...]
      yellows: [gold, lemon, sunflower, ...]
  
  colors.cool:
    datatypes:
      blues: [azure, navy, sky, ...]
      greens: [emerald, jade, forest, ...]
      purples: [violet, lavender, plum, ...]

# NO PROMPT SECTIONS - That's OK!
# NO RULES - That's OK!
# These are meant to be imported by other packages
```

**Validation should:**
- ✅ Check syntax is valid
- ✅ Check datatype structure is correct
- ✅ Check tags are well-formed
- ❌ **Don't warn about "unused" datatypes** - They're exports!

### Use Case 2: Application Packages

**Self-contained packages that render:**

```yaml
# my-generator.yaml
metadata:
  id: my-generator
  version: 1.0.0
  type: application  # ← Indicates this should be complete

namespaces:
  my.scenes:
    datatypes:
      creatures: [dragon, phoenix]
      locations: [castle, forest]
    
    prompt_sections:
      scene:
        references:
          creature: my.scenes:creatures  # ✅ Used
          # location not used ❌
```

**Validation should:**
- ✅ Check syntax is valid
- ✅ Check everything used correctly
- ⚠️ **Warn about unused datatypes** - Might be cleanup opportunity

### Use Case 3: Composite Packages

**Imports from libraries and adds content:**

```yaml
# my-fantasy.yaml
metadata:
  id: my-fantasy
  version: 1.0.0
  type: application
  dependencies:
    - id: colors-library
      version: ^1.0.0

namespaces:
  my.scenes:
    prompt_sections:
      colored_dragon:
        references:
          color: colors-library:colors.warm:reds  # ← Import from library
          creature: my.datatypes:creatures
```

**Validation of my-fantasy:**
- ✅ Check own datatypes used
- ✅ Check imports resolve (colors-library available)
- ⚠️ Warn about OWN unused datatypes
- ❌ **Don't validate colors-library** (that's validated separately)

**Validation of colors-library:**
- ✅ Check it's well-formed
- ❌ Don't warn about "unused" (it's type: library)

---

## Proposed Package Metadata Extension

### Add Type Field

```yaml
metadata:
  id: package-id
  version: 1.0.0
  type: library | application  # NEW FIELD
```

**Types:**

**`library`** - Collection of reusable components
- May have only datatypes
- May have only rules
- May have utility promptsections
- **Unused warnings suppressed** - Meant for import
- Examples: colors-library, common-rules, base-articles

**`application`** - Self-contained or executable package
- Has promptsections and/or rulebooks
- Intended to generate output
- **Unused warnings enabled** - Helps find dead code
- Examples: fantasy-generator, character-maker

### Alternative: Auto-Detection

```yaml
# If metadata.type not specified, infer from contents:

# Has prompt sections or rulebooks → application
# Has only datatypes/rules → library
```

**Problem:** Ambiguous for packages with both
**Better:** Explicit declaration

---

## Validation Rules by Package Type

### Library Package Validation

```rust
fn validate_library_package(package: &Package) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();
    
    // 1. Check syntax and structure
    report.check_yaml_syntax(package)?;
    report.check_namespaces_valid(package)?;
    report.check_datatypes_well_formed(package)?;
    
    // 2. Check references within the package resolve
    // (If it has promptsections, they should be valid)
    for namespace in &package.namespaces {
        for section in &namespace.prompt_sections {
            report.check_references_resolve(section, package)?;
        }
    }
    
    // 3. DON'T check for unused datatypes
    // Library datatypes are exports - meant to be unused in THIS package
    
    Ok(report)
}
```

### Application Package Validation

```rust
fn validate_application_package(package: &Package) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();
    
    // 1. All library checks
    report.merge(validate_library_package(package)?);
    
    // 2. Check for unused datatypes (potential dead code)
    let used_datatypes = find_all_used_datatypes(package);
    
    for datatype in all_datatypes(package) {
        if !used_datatypes.contains(&datatype.id) {
            report.add_warning(Warning::UnusedDatatype {
                datatype: datatype.id,
                message: "Datatype defined but never used in any promptsection",
                suggestion: "Remove if not needed, or change package type to 'library'"
            });
        }
    }
    
    // 3. Check dependencies are available (if any)
    for dep in &package.metadata.dependencies {
        report.check_dependency_available(dep)?;
    }
    
    Ok(report)
}
```

---

## Finding Used Datatypes (Corrected Logic)

**Simple approach - no graph traversal needed:**

```rust
fn find_all_used_datatypes(package: &Package) -> HashSet<String> {
    let mut used = HashSet::new();
    
    // Iterate ALL promptsections in ALL namespaces
    for namespace in &package.namespaces {
        for promptsection in &namespace.prompt_sections {
            // Check each reference
            for reference in &promptsection.references {
                // If it's a datatype reference (not context: or section)
                if is_datatype_reference(&reference.target) {
                    used.insert(reference.target.clone());
                }
            }
        }
    }
    
    // Also check rules (if they select from datatypes)
    for namespace in &package.namespaces {
        for rule in &namespace.rules {
            for logic in &rule.logic {
                if let Some(from_ref) = parse_ref_from_expression(&logic.from) {
                    if is_datatype_reference(&from_ref) {
                        used.insert(from_ref);
                    }
                }
            }
        }
    }
    
    used
}
```

**Key points:**
- ✅ Check ALL namespaces (not just one)
- ✅ Direct references are enough (don't need transitive)
- ✅ Used once = used (don't count multiple uses)

**Why transitive isn't needed:**

```yaml
styled_fantasy:
  references:
    base_scene: fantasy_scene  # This IS used

fantasy_scene:
  references:
    creature: creatures  # This IS used

# Both are used! Simple check finds both.
# No need to trace through styled_fantasy → fantasy_scene → creatures
```

---

## Rulebook-Level Validation (Future)

**This is where strict validation belongs!**

```yaml
# rulebook.yaml
metadata:
  id: my-rulebook
  dependencies:
    - colors-library  # Library package
    - my-scenes       # Application package

entries:
  - section: my-scenes:colored_dragon
    weight: 1.0
```

**Rulebook validation:**

```rust
fn validate_rulebook(rulebook: &Rulebook, context: &LoadedPackages) -> Result<()> {
    // 1. All dependencies available?
    for dep in &rulebook.dependencies {
        if !context.has_package(dep) {
            return Err("Missing dependency: {}", dep);
        }
    }
    
    // 2. All entry sections exist?
    for entry in &rulebook.entries {
        if !can_resolve_promptsection(entry.section, context) {
            return Err("Entry section not found: {}", entry.section);
        }
    }
    
    // 3. Can we transitively resolve ALL references?
    for entry in &rulebook.entries {
        let all_refs = collect_transitive_references(entry.section, context);
        
        for ref in all_refs {
            if !can_resolve(ref, context) {
                return Err("Cannot resolve reference: {} needed by {}", 
                    ref, entry.section);
            }
        }
    }
    
    // 4. All context contracts satisfied?
    check_context_contracts(rulebook, context)?;
    
    Ok(())
}
```

**This is where we check:**
- ✅ Everything needed for rendering is available
- ✅ Transitive references all resolve
- ✅ Library datatypes are imported correctly
- ✅ Can actually execute

---

## Applying This To Our Package

### Current Situation

```yaml
# prompt-gen-common.yaml
metadata:
  id: prompt-gen.common
  version: 0.1.0
  # type: ??? (Not specified)

# Has:
# - 37 datatypes
# - 17 prompt sections
# - 6 rules
```

**What type is it?**

**Mixed!** It has:
- Datatypes meant for export (library aspect)
- Promptsections for rendering (application aspect)

**Best classification:** `application` (since it CAN render)

**But:** Many datatypes might be intentionally for export too

### Recommendations

**Option 1: Mark as application**
```yaml
metadata:
  type: application
```
- Validator warns about unused datatypes
- We fix warnings OR mark specific datatypes as `shared: true`

**Option 2: Mark as library**
```yaml
metadata:
  type: library
```
- No unused warnings
- Everything available for import
- Promptsections are example/reference implementations

**Option 3: Add per-datatype export markers**
```yaml
datatypes:
  creatures:
    shared: true  # ← Exported, don't warn if unused
    values: [...]
  
  internal_helper:
    shared: false  # ← Internal only, warn if unused
    values: [...]
```

### My Recommendation

**For prompt-gen-common specifically:**

Mark as `library` since:
- It's meant to be a base package
- Other packages should import from it
- Datatypes are intentionally reusable
- Having unused datatypes is expected and fine

```yaml
metadata:
  id: prompt-gen.common
  version: 0.1.0
  type: library  # Base package for reuse
```

---

## Summary of Corrections

### What I Got Wrong

1. ❌ Thought validator needed to count usage multiple times
2. ❌ Over-complicated with graph traversal
3. ❌ Didn't consider library package use case

### What's Actually Right

1. ✅ Validator just checks: "used at least once?"
2. ✅ Simple iteration through all references is enough
3. ✅ Library packages SHOULD have "unused" datatypes (they're exports)
4. ✅ Strict validation belongs at rulebook level, not package level

### Key Design Principles

**Package Validation (Lenient):**
- Check correctness, not completeness
- Allow library packages with only datatypes
- Suppress "unused" warnings for libraries

**Rulebook Validation (Strict):**
- Check everything needed for rendering is available
- Check transitive dependencies resolve
- This is where "unused" errors matter

---

## Action Items

### For v1.0.0

1. **Add `metadata.type` field to package schema**
   - `library` | `application`
   - Optional, auto-detect if not specified

2. **Update validator to respect package type**
   - Library: Don't warn about unused
   - Application: Warn about unused (helpful cleanup)

3. **Document library package pattern**
   - Show examples of pure datatype collections
   - Explain when to use `type: library`

### For v1.1.0 (With Rulebooks)

4. **Implement rulebook validation**
   - Check all dependencies available
   - Check transitive references resolve
   - Strict validation for executability

5. **Consider per-datatype `shared` flag**
   - Allow fine-grained control
   - `shared: true` = don't warn if unused
   - Useful for mixed packages

---

**User was right to question my analysis!** The validation philosophy should be:
- **Packages:** Lenient, allow libraries
- **Rulebooks:** Strict, ensure executability

This is much cleaner than complex graph traversal for usage counting.

