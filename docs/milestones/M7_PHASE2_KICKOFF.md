# M7 Phase 2: Component Editors - START! 🚀

**Date:** 2025-12-17  
**Status:** 🎯 **STARTING NOW**  
**Goal:** Visual editing for all components

---

## Phase 2 Overview

**What we're building:** Visual editors so users can create and edit:
- Datatypes (values, weights, tags)
- PromptSections (templates, references)
- SeparatorSets (primary, secondary, tertiary)
- Rules (conditions, actions)

**Timeline:** 5-6 days estimated (but we're crushing estimates!)

---

## Build Order

### 1. Datatype Editor (First - Tonight/Tomorrow)
**Why first:** Most fundamental, users need this most  
**Complexity:** Medium  
**Features:**
- List of values with add/remove
- Weight sliders (1-100)
- Tag editor (key-value pairs)
- Preview what gets selected

### 2. PromptSection Editor (Next - Day 2-3)
**Why second:** Core authoring experience  
**Complexity:** High (Monaco editor integration)  
**Features:**
- Template text editor with syntax highlighting
- Reference configuration panel
- Min/max/separator/unique parameters
- Tag filter expression builder
- Live preview of rendered output

### 3. SeparatorSet Editor (Day 3)
**Why third:** Simple, quick win  
**Complexity:** Low  
**Features:**
- Primary separator input
- Secondary separator input
- Tertiary separator (optional)
- Preview formatting

### 4. Rules Editor (Day 4)
**Why last:** Most complex, least common  
**Complexity:** High  
**Features:**
- Condition builder (when/logic)
- Action editor (set context values)
- Test against selections

---

## Starting with Datatype Editor

### What It Needs

**UI Layout:**
```
┌─────────────────────────────────────┐
│ Datatype: colors                    │
├─────────────────────────────────────┤
│ Values:                             │
│ ┌─────────────────────────────────┐ │
│ │ ☰ red         [▓▓▓▓▓░] 50  [×] │ │
│ │ ☰ blue        [▓▓▓░░░] 30  [×] │ │
│ │ ☰ green       [▓▓░░░░] 20  [×] │ │
│ │ [+ Add Value]                   │ │
│ └─────────────────────────────────┘ │
│                                     │
│ Selected Value: red                 │
│ ┌─────────────────────────────────┐ │
│ │ Tags:                           │ │
│ │ article: "a"                    │ │
│ │ type: "color"                   │ │
│ │ [+ Add Tag]                     │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

**Data Structure:**
```typescript
interface Datatype {
  name: string
  values: DatatypeValue[]
  extends: string | null
  override_tags: Record<string, any>
}

interface DatatypeValue {
  text: string
  weight: number
  tags: Record<string, any>
}
```

**Features:**
1. ✅ Add/remove values
2. ✅ Edit value text
3. ✅ Adjust weights with slider
4. ✅ Add/edit/remove tags
5. ✅ Drag to reorder values
6. ✅ Preview weighted selection

---

## Implementation Plan

### Step 1: Basic Structure (30 min)
- Create DatatypeEditor.vue component
- Props: datatypeId, namespaceId, data
- Emit: update event
- Layout: values list + selected value panel

### Step 2: Value List (30 min)
- Display all values
- Add new value button
- Remove value button
- Edit value text inline

### Step 3: Weight Sliders (20 min)
- Add weight slider per value
- Update weight on slide
- Show percentage next to slider
- Normalize weights to 100% total

### Step 4: Tag Editor (30 min)
- Show tags for selected value
- Add new tag (key-value)
- Edit tag values
- Remove tags
- Support different types (string, number, boolean)

### Step 5: Integration (20 min)
- Wire up to PackageEditor
- Handle component selection
- Save changes back to package
- Show in component tree

**Total Estimate:** 2-2.5 hours for complete Datatype Editor

---

## Success Criteria

**Datatype Editor Done When:**
- [x] Can add new datatypes
- [x] Can edit existing datatypes
- [x] Can add/remove values
- [x] Can adjust weights
- [x] Can add/edit/remove tags
- [x] Changes save back to package
- [x] Preview shows what gets selected
- [x] UI is intuitive (no docs needed)

---

## Let's Build!

**Starting:** DatatypeEditor.vue

**Goal:** Create a beautiful, intuitive editor for datatypes in ~2 hours!

Ready? Let's go! 🚀

