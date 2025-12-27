# Navigation Alignment - Testing Checklist

## Status: ✅ Ready for Testing

All navigation changes have been implemented and the build is successful. Use this checklist to verify the changes work as expected.

---

## Pre-Testing Verification

- [x] `MainNavigation.vue` recreated from scratch
- [x] `GeneratePage.vue` updated to use MainNavigation
- [x] `PackageEditor.vue` updated with Tools dropdown
- [x] Build successful (`npm run build`)
- [x] No empty files found
- [x] No compilation errors
- [ ] Dev server running (`npm run dev`)
- [ ] Manual testing

---

## Testing Scenarios

### 1. Home Page Navigation (/​)

**Initial Load:**
- [ ] App loads to Home page by default
- [ ] Home page displays:
  - [ ] App title "Random Prompt Generator"
  - [ ] Version number
  - [ ] 4 navigation cards (Generate, Edit, Library, Marketplace)
  - [ ] Quick Actions section (New Package, Open Package)
- [ ] No top navigation bar on Home page (intentional)

**Navigation Cards:**
- [ ] Click "Generate" card → Navigate to /generate
- [ ] Click "Edit" card → Navigate to /edit
- [ ] Click "Library" card → Navigate to /library
- [ ] Click "Marketplace" card → Navigate to /marketplace

**Quick Actions:**
- [ ] Click "New Package" → Opens NewPackageDialog
- [ ] Click "Open Package" → Opens file picker

---

### 2. Top Navigation Bar (All Pages Except Home)

**Visual Check:**
- [ ] Navigation bar appears at top of Generate, Edit, Library, Marketplace pages
- [ ] Navigation links display with icons:
  - [ ] 🏠 Home
  - [ ] ⚡ Generate
  - [ ] ✏️ Edit
  - [ ] 📚 Library
  - [ ] 📦 Marketplace
- [ ] Active page is highlighted (different background)
- [ ] Hover effects work (background changes on hover)

**Navigation Functionality:**
- [ ] From Generate: Click each nav link, verify navigation works
- [ ] From Edit: Click each nav link, verify navigation works
- [ ] From Library: Click each nav link, verify navigation works
- [ ] From Marketplace: Click each nav link, verify navigation works
- [ ] Active page always highlighted correctly

---

### 3. Generate Page (/generate)

**Layout:**
- [ ] MainNavigation at top
- [ ] No "Back to Editor" button (removed)
- [ ] Page content displays correctly
- [ ] Can select rulebook and generate prompts

**Navigation:**
- [ ] All nav links work
- [ ] "Generate" is highlighted as active

---

### 4. Edit Page (/edit) - Tools Menu

**Layout:**
- [ ] MainNavigation at top
- [ ] Package info displays when package loaded:
  - [ ] Package name
  - [ ] Version number
- [ ] "🔧 Tools ▾" button in top-right
- [ ] Editor content displays normally

**Tools Menu:**
- [ ] Click "🔧 Tools ▾" → Dropdown opens
- [ ] Dropdown contains:
  - [ ] 📄 New Package
  - [ ] 📂 Open Package
  - [ ] 💾 Save Package
  - [ ] ───────── (divider)
  - [ ] 📤 Export Package

**Tools Menu Actions:**
- [ ] Click "New Package" → Opens NewPackageDialog, dropdown closes
- [ ] Click "Open Package" → Opens file picker, dropdown closes
- [ ] Click "Save Package" → Saves current package (if changes exist)
- [ ] "Save Package" disabled when no changes
- [ ] Click "Export Package" → Opens save dialog
- [ ] "Export Package" disabled when no package loaded
- [ ] Click outside dropdown → Dropdown closes

**Editor Functions:**
- [ ] Can create new package
- [ ] Can load existing package
- [ ] Package info updates in nav bar
- [ ] Can edit components
- [ ] Can save changes
- [ ] Validation panel works

---

### 5. Library Page (/library)

**Layout:**
- [ ] MainNavigation at top with action buttons:
  - [ ] 🔄 Refresh
  - [ ] 📥 Import Package
- [ ] Library browser displays packages
- [ ] Can load, delete, export packages

**Navigation:**
- [ ] All nav links work
- [ ] "Library" is highlighted as active

---

### 6. Marketplace Page (/marketplace)

**Layout:**
- [ ] MainNavigation at top
- [ ] Marketplace settings/browser displays
- [ ] Can browse and install packages

**Navigation:**
- [ ] All nav links work
- [ ] "Marketplace" is highlighted as active

---

## Theme Testing

### Light Theme
- [ ] Navigation bar background correct
- [ ] Text colors readable
- [ ] Active link highlighted clearly
- [ ] Hover effects visible
- [ ] Tools dropdown styled correctly

### Dark Theme
- [ ] Switch to dark theme (OS setting or manual)
- [ ] Navigation bar background correct
- [ ] Text colors readable
- [ ] Active link highlighted clearly
- [ ] Hover effects visible
- [ ] Tools dropdown styled correctly

---

## Cross-Page Workflow Testing

### Scenario 1: Create Package from Home
1. [ ] Start at Home
2. [ ] Click "New Package" quick action
3. [ ] Create package
4. [ ] Should navigate to Edit page
5. [ ] Package info shows in nav bar

### Scenario 2: Edit → Generate Workflow
1. [ ] Load package in Edit page
2. [ ] Click "Generate" in nav bar
3. [ ] Package available in Generate page
4. [ ] Generate prompts
5. [ ] Click "Edit" to return

### Scenario 3: Marketplace → Library → Edit
1. [ ] Go to Marketplace
2. [ ] Install a package
3. [ ] Click "Library" in nav bar
4. [ ] Find installed package
5. [ ] Load package
6. [ ] Should open in Edit page

---

## Edge Cases

### No Package Loaded
- [ ] Edit page shows welcome message
- [ ] Tools menu "Export Package" is disabled
- [ ] Can still access all navigation

### Unsaved Changes
- [ ] Make changes in editor
- [ ] "Save Package" in Tools menu enabled
- [ ] Navigate away → Browser warns about unsaved changes (if implemented)

### Navigation State
- [ ] Refresh page → Active page remains highlighted
- [ ] Browser back/forward → Navigation updates correctly

---

## Keyboard & Accessibility

- [ ] Tab through navigation links
- [ ] Focus visible on nav links
- [ ] Enter/Space activates links
- [ ] Tab into Tools dropdown
- [ ] Escape closes Tools dropdown
- [ ] Screen reader announces navigation (if tested)

---

## Performance

- [ ] Navigation is responsive (no lag)
- [ ] Page transitions smooth
- [ ] Tools dropdown opens/closes quickly
- [ ] No console errors in browser dev tools
- [ ] No Vue warnings in console

---

## Known Issues / Limitations

### IDE Warning (Non-Critical)
```
Selector active is never used
```
- This is a false positive - Vue Router adds the `.active` class dynamically
- Does not affect functionality

---

## Regression Testing

Verify existing features still work:

### Package Editor
- [ ] Component tree sidebar
- [ ] Datatype editor
- [ ] SeparatorSet editor
- [ ] PromptSection editor
- [ ] Rules editor
- [ ] Rulebook editor
- [ ] Live preview panel
- [ ] Validation panel

### Generate Page
- [ ] Package selection
- [ ] Rulebook selection
- [ ] Context editing
- [ ] Prompt generation
- [ ] Output display
- [ ] Copy to clipboard

### Library
- [ ] Package listing
- [ ] Load package
- [ ] Delete package
- [ ] Export package
- [ ] Import package

### Marketplace
- [ ] Package browsing
- [ ] Package details
- [ ] Install package
- [ ] Settings/configuration

---

## Sign-Off

**Tester:** _________________  
**Date:** _________________  
**Build Version:** _________________  

**Result:**
- [ ] ✅ All tests passed
- [ ] ⚠️ Minor issues found (list below)
- [ ] ❌ Major issues found (list below)

**Issues Found:**

1. _______________________________________________
2. _______________________________________________
3. _______________________________________________

**Notes:**

_______________________________________________
_______________________________________________
_______________________________________________

---

## Next Steps After Testing

If all tests pass:
1. Update user documentation
2. Create release notes
3. Tag release version
4. Deploy to production

If issues found:
1. Document issues in GitHub/issue tracker
2. Prioritize fixes
3. Re-test after fixes
4. Repeat until all critical issues resolved

