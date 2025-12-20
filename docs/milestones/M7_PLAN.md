# M7 Plan: Web Authoring Tool

**Created:** 2025-12-17  
**Status:** 🔄 **IN PROGRESS**  
**Goal:** Visual package authoring without touching YAML

---

## Overview

M7 is the **final core milestone** before documentation and publication! After M7, we'll have:
- ✅ Complete rendering engine (M3-M5)
- ✅ Context & coordination (M4)
- ✅ Validation & CLI (M6)
- 🎯 **Visual authoring tool** (M7)

Then just M8 (docs) and M9 (publication)!

---

## Goals

### Primary Goal
**Enable authors to create packages visually without editing YAML**

### Secondary Goals
- Visual editors for all components (Datatypes, PromptSections, Rules, etc.)
- Live preview with real-time rendering
- Integration with M6 validator
- Better than current desktop viewer (which is read-only)

---

## Scope

### What's IN Scope for M7

**Component Editors:**
- ✅ Datatype editor (add/edit/delete values, set weights, tags)
- ✅ PromptSection editor (template, references with parameters)
- ✅ SeparatorSet editor (primary, secondary, tertiary)
- ✅ Rules editor (conditions, actions)
- ⚠️ Decision editor (if needed - currently deferred)

**Package Management:**
- ✅ Create new package
- ✅ Load existing package
- ✅ Save package to YAML
- ✅ Package metadata editor

**Live Features:**
- ✅ Real-time rendering preview
- ✅ Validation as you type
- ✅ Error highlighting
- ✅ Debug view (show context, selections)

**UI/UX:**
- ✅ Clean, intuitive interface
- ✅ Drag-and-drop where appropriate
- ✅ Keyboard shortcuts
- ✅ Undo/redo

### What's OUT of Scope

- ❌ Multi-user collaboration
- ❌ Cloud storage/sync
- ❌ Marketplace integration
- ❌ Package dependencies browser
- ❌ AI-assisted authoring
- ❌ Mobile version

These are post-v1.0 features.

---

## Architecture Decision

### Option 1: Extend Current Desktop App (Tauri + Vue)
**Pros:**
- Already have the foundation
- Reuse existing renderer
- Consistent with M3-M6 work

**Cons:**
- Desktop only (not web accessible)
- Distribution requires installers

### Option 2: Pure Web App (Vue + WASM)
**Pros:**
- Accessible via browser
- No installation needed
- Easy to share

**Cons:**
- Need to compile Rust to WASM
- More complex build

### **Decision: Extend Desktop App (Option 1)**

**Rationale:**
- Leverage existing Tauri infrastructure
- Reuse all M3-M6 Rust code directly
- Can add web export later if needed
- Faster development (1-2 weeks vs 3-4)
- Better performance (native vs WASM)

**Trade-off:** Desktop-only for v1.0, but can be shared as executable

---

## Technical Approach

### Current State (Desktop Viewer)
```
✅ Package loading
✅ Read-only display
✅ Live preview with rendering
✅ Batch generation
✅ Copy to clipboard
```

### What We Need to Add
```
🎯 Editable components
🎯 Create/Save functionality  
🎯 Validation integration
🎯 Component wizards
🎯 Better UX for authoring
```

### Architecture
```
┌─────────────────────────────────────────┐
│           Vue.js Frontend               │
│  ┌─────────────────────────────────┐   │
│  │  Package Manager                │   │
│  │  - New/Load/Save                │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  Component Editors              │   │
│  │  - Datatype Editor              │   │
│  │  - PromptSection Editor         │   │
│  │  - SeparatorSet Editor          │   │
│  │  - Rules Editor                 │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  Live Preview                   │   │
│  │  - Real-time rendering          │   │
│  │  - Validation errors            │   │
│  │  - Debug view                   │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
              ↕ Tauri Commands
┌─────────────────────────────────────────┐
│           Rust Backend                  │
│  ┌─────────────────────────────────┐   │
│  │  File Operations                │   │
│  │  - Read/Write YAML              │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  Validator (M6)                 │   │
│  │  - Real-time validation         │   │
│  └─────────────────────────────────┘   │
│  ┌─────────────────────────────────┐   │
│  │  Renderer (M3-M5)               │   │
│  │  - Live preview                 │   │
│  └─────────────────────────────────┘   │
└─────────────────────────────────────────┘
```

---

## Implementation Plan

### Phase 1: Package Management (3-4 days)
**Goal:** Create, load, save packages

**Tasks:**
1. New package wizard
   - Package metadata form
   - Create default namespace
   - Initialize with template
2. Save functionality
   - Serialize to YAML
   - File picker integration
   - Validation before save
3. Load improvements
   - Better error handling
   - Recent files list
4. Package metadata editor
   - Edit name, version, authors, description
   - Dependency management UI

**Deliverables:**
- `PackageEditor.vue` - Main editor container
- `NewPackageDialog.vue` - Creation wizard
- `PackageMetadataEditor.vue` - Metadata form
- Tauri commands: `create_package`, `save_package`

**Success Criteria:**
- Can create new package with wizard
- Can save package to YAML file
- Can reload and edit saved package
- Validation prevents invalid saves

---

### Phase 2: Component Editors (5-6 days)
**Goal:** Visual editing for all components

**Tasks:**
1. Datatype Editor
   - List of values with add/remove
   - Weight sliders
   - Tag editor (key-value pairs)
   - Extends support
2. PromptSection Editor
   - Template text editor with syntax highlighting
   - Reference configuration
   - Min/max/separator/unique parameters
   - Tag filter expression builder
3. SeparatorSet Editor
   - Primary/secondary/tertiary inputs
   - Preview with examples
4. Rules Editor
   - Condition builder
   - Action editor
   - Test against selections

**Deliverables:**
- `DatatypeEditor.vue` - Edit datatypes
- `PromptSectionEditor.vue` - Edit promptsections
- `SeparatorSetEditor.vue` - Edit separators
- `RulesEditor.vue` - Edit rules
- `TagEditor.vue` - Reusable tag editing component
- `ReferenceEditor.vue` - Edit reference parameters

**Success Criteria:**
- All component types editable
- Changes reflected immediately in preview
- Validation shows errors inline
- Intuitive UX (no need for docs to figure out)

---

### Phase 3: Live Preview & Validation (2-3 days)
**Goal:** Real-time feedback while authoring

**Tasks:**
1. Live Preview enhancements
   - Auto-update on changes
   - Show validation errors
   - Debug view (context values, selections)
   - Multiple seed testing
2. Validation integration
   - Run M6 validator on changes
   - Highlight errors in editors
   - Show warnings
   - Suggest fixes
3. Testing tools
   - Render multiple times with different seeds
   - Export rendered prompts
   - Test specific promptsections

**Deliverables:**
- Enhanced `LivePreview.vue`
- `ValidationPanel.vue` - Show errors/warnings
- `DebugView.vue` - Internal state inspection
- `TestingPanel.vue` - Batch testing

**Success Criteria:**
- Errors shown immediately when typing
- Preview updates in real-time
- Debug view helps diagnose issues
- Can test package thoroughly before saving

---

### Phase 4: UX Polish (2-3 days)
**Goal:** Professional, polished experience

**Tasks:**
1. Navigation & Layout
   - Sidebar with component tree
   - Tabs for multiple open components
   - Breadcrumbs for navigation
2. Keyboard shortcuts
   - Ctrl+S to save
   - Ctrl+N for new component
   - Ctrl+Z/Y for undo/redo
3. Visual improvements
   - Icons for component types
   - Color coding
   - Tooltips
   - Loading states
4. Onboarding
   - Welcome screen
   - Sample packages to explore
   - Quick tips

**Deliverables:**
- `ComponentTree.vue` - Navigate package structure
- `ShortcutHandler.vue` - Keyboard shortcuts
- `WelcomeScreen.vue` - First-run experience
- Updated styling and icons

**Success Criteria:**
- Feels professional and polished
- Easy to navigate large packages
- Keyboard shortcuts speed up workflow
- New users can figure it out quickly

---

## Timeline

### Week 1 (Days 1-7)
- **Days 1-4:** Phase 1 - Package Management
- **Days 5-7:** Phase 2 Start - Datatype Editor

### Week 2 (Days 8-14)
- **Days 8-10:** Phase 2 Continued - PromptSection & Other Editors
- **Days 11-13:** Phase 3 - Live Preview & Validation
- **Day 14:** Phase 4 - UX Polish

**Total Estimated:** 12-14 days (2 weeks)

---

## Success Criteria

### Must Have (Launch Blockers)
- [x] Can create new package from scratch
- [x] Can edit all component types visually
- [x] Can save package to valid YAML
- [x] Live preview updates in real-time
- [x] Validation shows errors clearly
- [x] Can test package thoroughly

### Should Have (High Priority)
- [x] Keyboard shortcuts work
- [x] Clean, intuitive UI
- [x] Debug view available
- [x] Undo/redo for major operations

### Nice to Have (If Time Permits)
- [ ] Drag-and-drop reordering
- [ ] Component templates/snippets
- [ ] Export rendered prompts to file
- [ ] Dark mode

---

## User Stories

### Story 1: Create First Package
**As a** new author  
**I want to** create a package from scratch  
**So that** I can generate prompts for my project

**Acceptance:**
- Click "New Package" opens wizard
- Fill in metadata (name, version, authors)
- Creates default namespace
- Ready to add datatypes

### Story 2: Add Datatype
**As an** author  
**I want to** create datatypes with values  
**So that** I can define selectable options

**Acceptance:**
- Click "Add Datatype" creates new one
- Can add values with weights
- Can add tags to values
- Can preview selections

### Story 3: Create Template
**As an** author  
**I want to** create promptsections with templates  
**So that** I can compose prompts

**Acceptance:**
- Type template with {references}
- Configure each reference (min/max/filter/etc.)
- See live preview update
- Validation shows any errors

### Story 4: Test & Save
**As an** author  
**I want to** test my package thoroughly  
**So that** I'm confident it works

**Acceptance:**
- Render multiple times with different seeds
- See debug information
- Fix any validation errors
- Save to YAML file

---

## Technical Decisions

### TD-004: State Management
**Decision:** Use Vue 3 Composition API with reactive refs  
**Rationale:** Simple enough for this app, no need for Vuex/Pinia  
**Alternative:** Pinia (overkill for this scope)

### TD-005: Form Validation
**Decision:** VeeValidate for form validation  
**Rationale:** Industry standard, good Vue 3 support  
**Alternative:** Custom validation (reinventing wheel)

### TD-006: Code Editor
**Decision:** Monaco Editor for template editing  
**Rationale:** Same as VS Code, syntax highlighting, autocomplete  
**Alternative:** CodeMirror (also good, but Monaco has better TS support)

### TD-007: File Operations
**Decision:** Tauri file system API  
**Rationale:** Already using Tauri, native performance  
**Alternative:** Browser file APIs (limited capabilities)

---

## Risks & Mitigation

### Risk 1: Complexity
**Risk:** Component editors too complex to use  
**Mitigation:** User testing early, simplify UX, add tooltips/help

### Risk 2: Performance
**Risk:** Live preview slow with large packages  
**Mitigation:** Debounce updates, lazy rendering, optimization

### Risk 3: Validation Errors
**Risk:** Cryptic validation errors confuse users  
**Mitigation:** Use M6's helpful error messages, inline help

### Risk 4: Data Loss
**Risk:** Unsaved changes lost on crash  
**Mitigation:** Auto-save to temp file, recovery on restart

---

## Dependencies

### From Previous Milestones
- ✅ M3-M5: Renderer working
- ✅ M6: Validator with helpful errors
- ✅ M2: YAML serialization

### External Libraries
- Vue 3 (already using)
- VeeValidate (form validation)
- Monaco Editor (code editing)
- Tauri (already using)

---

## Testing Strategy

### During Development
- Component tests with Vue Test Utils
- Manual testing of each editor
- Test with M1-M6 packages

### Before Release
- Create package from scratch (full workflow)
- Edit existing packages
- Test all validation errors
- Try to break it (edge cases)
- Performance testing with large packages

---

## Documentation

### For M7
- User guide for authoring tool
- Screenshots/video walkthrough
- Keyboard shortcuts reference
- Tips & best practices

### Update Existing
- README.md - Add authoring tool section
- DEVELOPMENT_PLAN.md - Mark M7 complete
- COMPLIANCE.md - Update status

---

## Post-M7 (Out of Scope, Future)

### Web Version
- Compile Rust to WASM
- Host as web app
- Same UI, browser-based

### Advanced Features
- Package marketplace browser
- Dependency management
- Version control integration
- Collaboration features
- AI-assisted authoring

### Mobile
- Touch-optimized UI
- Mobile app (Tauri supports mobile)

---

## Next Steps

### Immediate (Start Phase 1)
1. Create `PackageEditor.vue` shell
2. Implement new package wizard
3. Add save functionality
4. Test create → edit → save workflow

### This Week
- Complete Phase 1 (Package Management)
- Start Phase 2 (Datatype Editor)

---

## M7 Milestones

- [ ] **Phase 1 Complete:** Can create and save packages
- [ ] **Phase 2 Complete:** All component editors working
- [ ] **Phase 3 Complete:** Live preview and validation integrated
- [ ] **Phase 4 Complete:** UX polished and ready
- [ ] **User Verification:** Author can create package without docs
- [ ] **M7 Complete:** Ready for M8 (documentation)

---

**Let's build the authoring tool and complete the core implementation!** 🚀

**Ready to start Phase 1: Package Management?** 💪

