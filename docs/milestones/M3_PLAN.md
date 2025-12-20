# M3: Basic Rendering - Implementation Plan

**Timeline:** Week 5-6 (Dec 16 - Dec 30, 2025)  
**Status:** ⏳ In Progress - Just Started!  
**Goal:** Render "A red ball" from minimal.yaml

---

## Objective

Implement a basic three-phase rendering pipeline that can:
1. Parse template syntax (`{reference}`)
2. Select values from datatypes (with seed)
3. Substitute values into templates
4. Display rendered prompts in UI

**Success Criteria:**
- Can render simple templates from minimal.yaml
- Rendering is deterministic (same seed = same output)
- Live preview works in UI
- No context operations yet (M4)

---

## Architecture Overview

### Three-Phase Pipeline

```
1. SELECTION PHASE
   Input: PromptSection, Package, Seed
   Process: Parse template, select values from datatypes
   Output: Map of {reference_name → selected_value}

2. ENRICHMENT PHASE (Stub for M3)
   Input: Selected values
   Process: (M4 will add Rules/Decisions here)
   Output: Same map (no-op in M3)

3. RENDERING PHASE
   Input: Template + Selected values
   Process: Substitute {reference} with values
   Output: Final rendered string
```

### Components to Build

**Rust Backend:**
1. `seeded_random.rs` - Deterministic RNG
2. `template_parser.rs` - Parse `{reference}` syntax
3. `selector.rs` - Select values from datatypes
4. `renderer.rs` - Three-phase pipeline
5. `commands/render.rs` - Tauri command

**Vue Frontend:**
1. `LivePreview.vue` - Preview component with seed input

---

## Implementation Tasks

### Phase 1: Seeded Random (30 min)
- [ ] Create `SeededRandom` struct
- [ ] Implement seeded PRNG (Xorshift or use `rand` crate)
- [ ] Unit tests for determinism

### Phase 2: Template Parser (2-3 hours)
- [ ] Parse `{reference}` syntax
- [ ] Extract reference names from template
- [ ] Handle escaped braces
- [ ] Parse simple references (no params yet)
- [ ] Unit tests

### Phase 3: Value Selector (2-3 hours)
- [ ] Select random value from datatype
- [ ] Respect weights (if defined)
- [ ] Return selected value with tags
- [ ] Unit tests

### Phase 4: Renderer (3-4 hours)
- [ ] Implement three-phase pipeline
- [ ] Selection: Parse + select all references
- [ ] Enrichment: Stub (no-op for M3)
- [ ] Rendering: Substitute values into template
- [ ] Unit tests for full pipeline

### Phase 5: Tauri Integration (1-2 hours)
- [ ] Create `render_prompt` command
- [ ] Wire up to renderer
- [ ] Error handling
- [ ] Test from Vue

### Phase 6: Vue UI (2-3 hours)
- [ ] Create LivePreview component
- [ ] Seed input field
- [ ] Render button
- [ ] Display output
- [ ] Error display

### Phase 7: Testing & Polish (2-3 hours)
- [ ] Integration tests
- [ ] Test with minimal.yaml
- [ ] Verify determinism
- [ ] Polish UI
- [ ] Update documentation

**Total Estimated Effort:** 12-18 hours (1.5-2 days focused work)

---

## Data Structures

### Template AST
```rust
pub enum TemplateToken {
    Text(String),           // Static text
    Reference(String),      // {reference_name}
}

pub struct Template {
    pub tokens: Vec<TemplateToken>,
}
```

### Selection Result
```rust
pub struct SelectedValue {
    pub text: String,
    pub tags: HashMap<String, serde_json::Value>,
}

pub struct SelectionContext {
    pub values: HashMap<String, SelectedValue>,
}
```

### Render Result
```rust
pub struct RenderResult {
    pub output: String,
    pub seed: u64,
}
```

---

## Scope Limitations for M3

### IN SCOPE ✅
- Simple `{reference}` syntax
- Random selection from datatypes
- Deterministic seeding
- Basic template substitution
- Live preview UI

### OUT OF SCOPE (M4+) ❌
- **Parameters:** `{ref?min=1,max=3}` → M5
- **Tag filtering:** `{ref#filter}` → M4
- **Context operations:** `context.get()` → M4
- **Rules execution:** `compute_article` → M4
- **Decisions execution:** `pluralize` → M4
- **Nested promptsections:** `{namespace:section}` → M5
- **Separators:** `&sep=comma_and` → M5

**Keep it SIMPLE for M3!**

---

## Test Plan

### Unit Tests
1. `SeededRandom` - same seed produces same sequence
2. `TemplateParser` - correctly parses various templates
3. `Selector` - selects values, respects weights
4. `Renderer` - full pipeline works

### Integration Tests
1. Render "A {color} {object}" → "A red ball"
2. Different seeds → different outputs
3. Same seed → same output (determinism)
4. Multiple renders → consistent

### Manual Tests
1. Load minimal.yaml
2. Click render with seed 42
3. See "A [color] [object]" output
4. Click again → same output
5. Change seed → different output

---

## Minimal.yaml Reminder

```yaml
namespaces:
  test:
    datatypes:
      colors:
        values:
          - text: red
          - text: blue
          - text: orange
      objects:
        values:
          - text: ball
          - text: apple
    prompt_sections:
      basic:
        template: "{article} {color} {object}"
        references:
          color: { target: test:colors }
          object: { target: test:objects }
```

**For M3:** Ignore `{article}` (context operation for M4)  
**Simplified template:** Just render `{color} {object}` → "red ball"

---

## File Structure

```
src-tauri/src/
├── renderer/              ← NEW MODULE
│   ├── mod.rs
│   ├── seeded_random.rs
│   ├── template_parser.rs
│   ├── selector.rs
│   └── renderer.rs
├── commands/
│   ├── mod.rs
│   ├── package.rs         ← Existing
│   └── render.rs          ← NEW
└── main.rs                ← Update to register render commands

src/components/
├── PackageViewer.vue      ← Existing
└── LivePreview.vue        ← NEW
```

---

## Success Criteria (Revisited)

From DEVELOPMENT_PLAN.md:

- [ ] Can render "Hello {name}" deterministically with seed
- [ ] Can handle simple references: `{color} {object}`
- [ ] Same seed produces same output
- [ ] Different seeds produce different outputs
- [ ] Live preview works in UI
- [ ] All PoC-level examples work (without Rules/Decisions)

---

## Next Steps

1. Create renderer module structure
2. Implement SeededRandom
3. Implement TemplateParser
4. Implement Selector
5. Implement Renderer pipeline
6. Create Tauri command
7. Build Vue LivePreview component
8. Test with minimal.yaml
9. Update documentation

**Let's start with SeededRandom!**

---

**M3 Status:** Ready to begin implementation! 🚀  
**First Task:** Create renderer module and SeededRandom  
**Estimated First Session:** 4-6 hours for core rendering pipeline

