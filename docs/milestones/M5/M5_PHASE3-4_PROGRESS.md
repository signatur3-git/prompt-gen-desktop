# M5 Phase 3+4 Implementation Progress

**Date Started:** 2025-12-17  
**Status:** 🔄 IN PROGRESS  
**Estimated Time:** 5-6 hours

---

## Progress Tracker

### Step 1: Update Template Parser ⏳ IN PROGRESS
**Goal:** Parse `{ref?min=2,max=4&sep=comma_and&unique=true#{filter}}`

- [ ] Update TemplateToken struct with new fields
- [ ] Implement parameter parser
- [ ] Handle min/max/sep/unique parameters
- [ ] Maintain backward compatibility
- [ ] Unit tests

**Status:** Starting now...

### Step 2: Update Reference Model 🔴 NOT STARTED
**Goal:** Add min/max/sep/unique fields to Reference in YAML

- [ ] Update Reference struct in core/models.rs
- [ ] Add serde defaults
- [ ] Update existing references to work with new fields

### Step 3: Implement Separator Formatter 🔴 NOT STARTED
**Goal:** Format lists with SeparatorSet

- [ ] Create renderer/separator.rs
- [ ] Implement format() method
- [ ] Handle 0, 1, 2, 3+ items correctly

### Step 4: Update Selector 🔴 NOT STARTED
**Goal:** Support selecting multiple values

- [ ] Add select_multiple() method
- [ ] Handle unique constraint
- [ ] Max attempts protection

### Step 5: Update Renderer 🔴 NOT STARTED
**Goal:** Handle multiple selections and formatting

- [ ] Change SelectedValue to Vec<SelectedValue>
- [ ] Update phase_1_selection for min/max
- [ ] Update phase_3_rendering for separator formatting
- [ ] Find separator sets in package

### Step 6: Create Test Package 🔴 NOT STARTED
**Goal:** lists-test.yaml with all test cases

- [ ] Simple list (space-separated)
- [ ] Natural list (comma_and)
- [ ] Unique constraint
- [ ] Optional items (min=0)
- [ ] SANITY CHECK: M4+M5 integration

### Step 7: Test & Debug 🔴 NOT STARTED
**Goal:** Verify all scenarios work

- [ ] Load package
- [ ] Test each prompt section
- [ ] Verify with different seeds
- [ ] User verification

---

## Current Task

**Working on:** Step 1 - Template Parser

**Next file to edit:** `src-tauri/src/renderer/template_parser.rs`

---

**Last Updated:** 2025-12-17 (starting)

