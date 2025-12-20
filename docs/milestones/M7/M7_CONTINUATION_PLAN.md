# M7 Continuation Plan + Testing Strategy

**Date:** 2025-12-17  
**Status:** Ready to Continue Phase 3

---

## M7 Current Status

### ✅ Complete (50%):
- **Phase 1:** Package Management (100%)
- **Phase 2:** Component Editors (100%)
  - All 5 editors built and working
  - Bonus: Add Namespace feature

### ⏳ Remaining (50%):
- **Phase 3:** Live Preview & Validation (0%)
- **Phase 4:** UX Polish (0%)

**Let's continue with Phase 3!** 🚀

---

## Phase 3: Live Preview & Validation

### Goals:
1. Real-time validation feedback
2. Enhanced live preview
3. Debug view for context inspection
4. Better error messages

### Tasks (Priority Order):

#### 1. Validation Integration (Highest Priority)
**Connect M6 validator to authoring tool:**
- ✅ Validator already exists (M6 complete)
- ⏳ Call validator on package changes
- ⏳ Display errors in ValidationPanel
- ⏳ Highlight errors in editors
- ⏳ Click error → jump to component

**Implementation:**
```vue
// PackageEditor.vue already has:
const validationErrors = ref([])

async function validatePackage(pkg) {
  const result = await invoke('validate_package', { package: pkg })
  validationErrors.value = result.errors || []
}

// Need to add:
- Error highlighting in component tree
- Click-to-jump functionality
- Real-time validation on edit
```

**Estimated:** 2-3 hours

#### 2. Enhanced Live Preview
**Already exists, needs improvements:**
- ✅ Current: Renders selected promptsection
- ⏳ Add: Preview all promptsections in package
- ⏳ Add: Auto-refresh on changes
- ⏳ Add: Show which components are referenced

**Estimated:** 1-2 hours

#### 3. Debug View
**New component for troubleshooting:**
- Show context values during render
- Show selection choices made
- Show rule executions
- Step-through rendering

**Estimated:** 2-3 hours

#### 4. Testing Panel
**Batch testing tools:**
- ✅ Already have batch rendering
- ⏳ Add: Export to file
- ⏳ Add: Seed range testing
- ⏳ Add: Compare outputs

**Estimated:** 1-2 hours

**Total Phase 3:** 6-10 hours (~1-2 days)

#### 5. Integration Tests (Deferred but Planned)
**Test workflows with mocked Tauri:**
- ⏳ Load package → Edit → Save workflow
- ⏳ Create package → Add namespace → Save
- ⏳ Validation errors → Jump to component
- ⏳ Mock Tauri invoke calls

**Note:** Tracked for future implementation, not blocking Phase 3 completion.

**Estimated:** 3-4 hours when implemented

---

## Phase 4: UX Polish

### Goals:
1. Keyboard shortcuts
2. Better navigation
3. Performance optimization
4. Visual improvements

### Tasks:

#### 1. Keyboard Shortcuts
- `Ctrl+S` - Save
- `Ctrl+O` - Open
- `Ctrl+N` - New
- `Ctrl+R` - Render
- `F5` - Refresh preview

**Estimated:** 1 hour

#### 2. Navigation Improvements
- Breadcrumb trail
- Recent files
- Search components
- Favorites

**Estimated:** 2 hours

#### 3. Performance
- Lazy load large packages
- Virtual scrolling for long lists
- Debounce validation
- Cache rendered previews

**Estimated:** 2 hours

#### 4. Visual Polish
- Loading states
- Animations
- Better icons
- Consistent spacing

**Estimated:** 2 hours

**Total Phase 4:** 7 hours (~1 day)

---

## Testing Strategy for Tauri/Vue

### Question: Can we add frontend tests?

**Answer: YES! Multiple approaches available:**

### 1. Unit Tests (Vue Components in Isolation)

**Tool:** Vitest + Vue Test Utils

**What it tests:**
- Component logic
- Data transformations
- Computed properties
- Event emissions

**Does NOT need Tauri context!**

**Setup:**
```bash
npm install -D vitest @vue/test-utils happy-dom
```

**Add to package.json:**
```json
{
  "scripts": {
    "test": "vitest",
    "test:ui": "vitest --ui",
    "test:coverage": "vitest --coverage"
  }
}
```

**Example test:**
```javascript
// DatatypeEditor.spec.js
import { mount } from '@vue/test-utils'
import { describe, it, expect } from 'vitest'
import DatatypeEditor from './DatatypeEditor.vue'

describe('DatatypeEditor', () => {
  it('adds a new value', () => {
    const wrapper = mount(DatatypeEditor, {
      props: {
        datatypeName: 'colors',
        data: { values: [] }
      }
    })
    
    wrapper.vm.addValue()
    
    expect(wrapper.vm.values).toHaveLength(1)
    expect(wrapper.emitted('update')).toBeTruthy()
  })
  
  it('validates weight range', () => {
    const wrapper = mount(DatatypeEditor, {
      props: {
        datatypeName: 'test',
        data: { values: [{ text: 'red', weight: 50 }] }
      }
    })
    
    const slider = wrapper.find('input[type="range"]')
    expect(slider.attributes('min')).toBe('1')
    expect(slider.attributes('max')).toBe('100')
  })
})
```

**Pros:**
- Fast (no Tauri overhead)
- Easy to write
- Good for logic testing
- CI-friendly

**Cons:**
- Can't test Tauri integration
- Can't test file dialogs
- Can't test backend calls

---

### 2. Integration Tests (With Mock Tauri)

**Tool:** Vitest + Mock Tauri APIs

**What it tests:**
- Component + Tauri integration
- Package loading/saving flow
- Validation calls
- Full workflows

**Setup:**
```javascript
// __mocks__/@tauri-apps/api/core.js
export const invoke = vi.fn()

// Test file
import { invoke } from '@tauri-apps/api/core'

vi.mock('@tauri-apps/api/core')

describe('PackageEditor integration', () => {
  it('loads package', async () => {
    invoke.mockResolvedValue({
      id: 'test',
      version: '1.0.0',
      // ...
    })
    
    await wrapper.vm.loadPackage()
    
    expect(invoke).toHaveBeenCalledWith('load_package', { path: 'test.yaml' })
    expect(wrapper.vm.currentPackage.value.id).toBe('test')
  })
})
```

**Pros:**
- Tests integration logic
- No real Tauri needed
- Still fast
- CI-friendly

**Cons:**
- Mocks need maintenance
- Not testing real Tauri behavior

---

### 3. E2E Tests (Full Tauri App)

**Tool:** WebdriverIO + Tauri Driver

**What it tests:**
- Full application
- Real Tauri context
- File system operations
- Actual user workflows

**Setup:**
```bash
npm install -D @wdio/cli tauri-driver
```

**Example:**
```javascript
// e2e/package-management.spec.js
describe('Package Management', () => {
  it('creates and saves package', async () => {
    await browser.click('#new-package-btn')
    await browser.setValue('#package-id', 'test.package')
    await browser.setValue('#package-name', 'Test Package')
    await browser.click('#create-btn')
    
    await browser.click('#save-package-btn')
    // File dialog appears (Tauri native)
    
    const packageExists = await browser.execute(() => {
      // Check file system
    })
    expect(packageExists).toBe(true)
  })
})
```

**Pros:**
- Tests real app
- Tests Tauri APIs
- Catches integration bugs
- Confidence boost

**Cons:**
- Slow (full app startup)
- Complex setup
- Flaky without care
- Hard to run in CI

---

## Recommended Testing Strategy

### Phase 1: Unit Tests (Start Now!)

**Focus:**
- Component logic
- Data transformations
- Event handling

**Components to test first:**
1. **DatatypeEditor** - Add/remove values, weight validation
2. **PromptSectionEditor** - Reference management, template parsing
3. **AddNamespaceDialog** - Validation logic
4. **ComponentTree** - Tree state, expand/collapse

**Why start here:**
- Fast feedback
- Easy to write
- High value (catches logic bugs)
- Doesn't need Tauri

**Setup:** ~30 minutes  
**First tests:** ~2 hours  
**Total:** ~2.5 hours for good coverage

**Status:** ⏳ Planned for Phase 3

---

### Phase 2: Integration Tests (Planned, Not in M7 Scope)

**Focus:**
- PackageEditor workflows
- Validation integration
- Mock Tauri calls

**Key workflows to test:**
1. Load package → Edit → Save
2. Create package → Add namespace → Save
3. Validation errors → Jump to component

**Setup:** ~1 hour (reuse Vitest)  
**Tests:** ~3 hours  
**Total:** ~4 hours

**Status:** 📋 **Documented for post-M7 implementation**
- Not blocking v1.0.0 release
- Will be implemented in v1.1.0 or maintenance phase
- Tracked in DEVELOPMENT_PLAN.md as future task

---

### Phase 3: E2E Tests (Separate Project)

**Focus:**
- Package import/export
- File system operations
- Native dialogs

**Status:** 🔮 **Future separate project**
- Will be standalone testing suite
- Not part of current reference implementation
- Defer to after v1.0.0 release

**Note:** User will start this as a new project later.

---

## Implementation Plan

### Today/Tomorrow: Phase 3 (Live Preview & Validation)

**Priority 1: Validation Integration** (2-3 hours)
```
1. Wire up validator calls on change
2. Display errors in ValidationPanel
3. Add click-to-jump functionality
4. Test with invalid packages
```

**Priority 2: Enhanced Preview** (1-2 hours)
```
1. Add "Preview All" mode
2. Auto-refresh on changes
3. Show referenced components
```

**Priority 3: Debug View** (2-3 hours)
```
1. Create DebugView component
2. Show context values
3. Show selection log
4. Show rule executions
```

**Total:** 5-8 hours

---

### Next Day: Phase 4 (UX Polish)

**Priority 1: Keyboard Shortcuts** (1 hour)
```
1. Add global keyboard handler
2. Implement save/open/new/render
```

**Priority 2: Navigation** (2 hours)
```
1. Add breadcrumbs
2. Recent files list
```

**Priority 3: Performance** (2 hours)
```
1. Debounce validation
2. Virtual scrolling for large lists
```

**Priority 4: Polish** (2 hours)
```
1. Loading states
2. Better icons
3. Animations
```

**Total:** 7 hours

---

### In Parallel: Unit Tests (Optional for M7)

**While building Phase 3/4, can add tests:**

**Day 1:** Setup + DatatypeEditor tests (2 hours)
**Day 2:** PromptSectionEditor + SeparatorSetEditor tests (2 hours)
**Day 3:** Component test coverage (2 hours)

**Total:** 6 hours (can overlap with feature work)

**Status:** Optional for M7 - can defer to post-release maintenance

---

### Post-M7: Integration Tests (v1.1.0 or Maintenance)

**After v1.0.0 release, add integration tests:**

**Week 1:** Setup mocking infrastructure (2 hours)
**Week 2:** Write workflow tests (4 hours)
**Week 3:** Test coverage and CI integration (2 hours)

**Total:** 8 hours

**Tracked in:** DEVELOPMENT_PLAN.md as post-v1.0.0 task

---

## Timeline Summary

| Day | M7 Phase | Testing | Total Hours |
|-----|----------|---------|-------------|
| 1 | Phase 3 (validation, preview) | Optional: Setup + Datatype tests | 5-10h |
| 2 | Phase 3 (debug view, testing panel) | Optional: Component tests | 4-8h |
| 3 | Phase 4 (shortcuts, navigation) | Optional: Test coverage | 5-9h |
| 4 | Phase 4 (performance, polish) | Optional: Cleanup | 5-7h |

**Total M7 Completion (without tests):** 2-3 days (19-34 hours)  
**Total M7 Completion (with unit tests):** 3-4 days (25-40 hours)  
**At your pace:** Probably 1-2 days! ⚡

**Integration tests:** Deferred to post-v1.0.0 (tracked in planning docs)

---

## Answer to Your Questions

### 1. Can we continue with M7?

**YES! Let's do Phase 3!**

**Starting with:**
- Validation integration (highest value)
- Enhanced preview
- Debug view

**Estimated:** 1-2 days at your pace

---

### 2. Is it possible to add frontend tests?

**YES! Absolutely!**

**Best approach:**
1. **Unit tests with Vitest** - No Tauri context needed
   - Test component logic
   - Fast, easy, high value
   - Start here!

2. **Integration tests with mocked Tauri** - If needed
   - Test workflows
   - Still no real Tauri
   - Good for PackageEditor

3. **E2E tests with Tauri Driver** - Optional
   - Full app testing
   - Defer to later
   - Complex but comprehensive

**Recommendation:** Start with unit tests while building Phase 3!

**Not problematic at all!** Tauri context only needed for E2E, not unit/integration.

---

## Next Steps

**Your choice:**

### Option A: Phase 3 Only
- Focus on validation + preview features
- Skip tests for now
- Fastest to M7 completion
- **Timeline:** 1-2 days

### Option B: Phase 3 + Tests
- Build features AND tests
- Better long-term quality
- Slightly slower but safer
- **Timeline:** 2-3 days

### Option C: Phase 3 + Phase 4 + Tests
- Complete M7 fully
- Add comprehensive tests
- Production-ready
- **Timeline:** 3-4 days

**My Recommendation:** Option B (Phase 3 + Tests)
- Good balance
- Tests pay off quickly
- Still fast pace
- Quality foundation for v1.0

---

## Ready to Start!

**Let me know and I'll:**
1. ✅ Set up Vitest testing
2. ✅ Build Phase 3 features (validation, preview, debug)
3. ✅ Write tests as we go
4. ✅ Update all documentation

**Or just pick what you want to focus on!** 😊

---

**M7 is 50% done, let's finish it!** 🚀

