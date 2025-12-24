# Package Library & Generation System

**Date:** 2025-12-24  
**Status:** 🚧 In Progress  
**Feature:** Multi-package storage, library browser, and dedicated Generate page

## Overview

Transform the desktop app from a single-package editor into a full package management system with:
1. **Package Library** - Persistent storage of installed, created, and downloaded packages
2. **Library Browser** - View and manage all available packages
3. **Generate Page** - Select rulebooks across multiple packages to generate prompts
4. **Marketplace Integration** - Installed packages persist and are available across sessions

## Current State

### ✅ What Works
- Loading a single package with dependency resolution
- Marketplace connection and package browsing
- OAuth authentication flow
- Package downloading from marketplace
- Basic package editor
- Live preview for loaded package

### ❌ What's Missing
- **No persistent storage** - Downloaded packages aren't saved
- **No package library** - Can't view all installed packages
- **No multi-package context** - Can only work with one package at a time
- **No Generate page** - Can't select rulebooks from multiple packages
- **Installation doesn't persist** - Marketplace installs don't add to library

## Architecture Design

### 1. Package Storage Layer

**Location:** `%APPDATA%/com.rpg.desktop/packages/`

**Structure:**
```
packages/
├── installed/              # Marketplace packages
│   ├── namespace.package-name@1.0.0.yaml
│   ├── namespace.package-name@1.1.0.yaml
│   └── metadata.json       # Installation metadata
├── local/                  # User-created packages
│   ├── my-custom-package.yaml
│   └── metadata.json
└── library.json            # Package library index
```

**library.json Schema:**
```json
{
  "version": "1.0.0",
  "packages": [
    {
      "id": "namespace.package-name",
      "name": "Package Name",
      "version": "1.0.0",
      "source": "marketplace" | "local" | "imported",
      "path": "installed/namespace.package-name@1.0.0.yaml",
      "installedAt": "2025-12-24T10:00:00Z",
      "lastUsed": "2025-12-24T11:00:00Z",
      "metadata": {
        "authors": ["Author Name"],
        "description": "...",
        "tags": ["tag1", "tag2"]
      }
    }
  ]
}
```

### 2. Rust Backend Commands

**New Commands:**
```rust
// Package library management
list_packages() -> Vec<PackageLibraryEntry>
install_package_to_library(yaml_content: String, source: PackageSource) -> PackageLibraryEntry
remove_package_from_library(package_id: String, version: String) -> ()
load_package_from_library(package_id: String, version: String) -> Package
load_all_packages() -> HashMap<String, Package>  // For Generate page

// Library operations
get_library_path() -> String
refresh_library() -> Vec<PackageLibraryEntry>
export_package(package_id: String, dest_path: String) -> ()
```

### 3. Frontend Components

**New Pages:**
- `src/pages/LibraryPage.vue` - Browse and manage package library
- `src/pages/GeneratePage.vue` - Multi-package prompt generation

**New Components:**
- `src/components/PackageLibraryBrowser.vue` - Library UI
- `src/components/RulebookSelector.vue` - Select rulebooks across packages
- `src/components/PackageCard.vue` - Display package info in library

**Updated Components:**
- `MarketplacePage.vue` - Install to library instead of temp location
- `PackageEditor.vue` - Save to library, load from library

### 4. Router Updates

```typescript
{
  path: '/library',
  name: 'Library',
  component: LibraryPage
},
{
  path: '/generate',
  name: 'Generate',
  component: GeneratePage
}
```

## Implementation Plan

### Phase 1: Storage Backend (Rust)
- [ ] Create `src-tauri/src/storage/` module
- [ ] Implement `PackageLibrary` struct
- [ ] Add `library.json` read/write
- [ ] Implement package installation to library
- [ ] Add Tauri commands for library operations
- [ ] Write tests for storage layer

### Phase 2: Library Browser (Frontend)
- [ ] Create `LibraryPage.vue`
- [ ] Create `PackageLibraryBrowser.vue` component
- [ ] Implement package listing
- [ ] Add search/filter functionality
- [ ] Add package actions (load, delete, export)
- [ ] Update navigation to include Library

### Phase 3: Marketplace Integration
- [ ] Update `MarketplacePage.vue` to install to library
- [ ] Show installation status
- [ ] Handle version conflicts
- [ ] Add "installed" badges in marketplace browser

### Phase 4: Generate Page
- [ ] Create `GeneratePage.vue`
- [ ] Create `RulebookSelector.vue`
- [ ] Load all packages in library
- [ ] Display rulebooks from all packages
- [ ] Implement multi-package context
- [ ] Add generation UI (seed, batch, etc.)
- [ ] Show generated prompts

### Phase 5: Editor Integration
- [ ] Add "Save to Library" action
- [ ] Add "Load from Library" action
- [ ] Update package loading to use library
- [ ] Add library status indicators

## User Flows

### Flow 1: Install from Marketplace
1. User opens Marketplace
2. Browses packages
3. Clicks "Install"
4. Package downloaded and saved to `packages/installed/`
5. Added to library.json
6. Success notification: "Package installed to library"
7. User can navigate to Library to see it

### Flow 2: Generate Prompts
1. User clicks "Generate" in navigation
2. Sees list of all installed packages
3. Expands packages to see rulebooks
4. Selects rulebook(s) from one or more packages
5. Sets generation options (seed, context)
6. Clicks "Generate"
7. Sees generated prompts
8. Can copy, save, or regenerate

### Flow 3: Create and Save
1. User creates package in Editor
2. Clicks "Save to Library"
3. Package saved to `packages/local/`
4. Added to library.json
5. Now available in Library and Generate pages

## Technical Considerations

### Dependency Resolution
- When loading packages from library, resolve deps from library first
- Fall back to search paths if not in library
- Update `DependencyResolver` to check library

### Version Management
- Support multiple versions of same package
- Library browser shows all versions
- Generate page uses latest version by default
- Allow version selection

### Performance
- Cache loaded packages in memory
- Lazy-load package details in library browser
- Index library.json for fast lookups

### Data Migration
- Detect existing packages in old locations
- Offer to import to library
- Preserve user's work

## Files to Create

**Rust:**
- `src-tauri/src/storage/mod.rs`
- `src-tauri/src/storage/package_library.rs`
- `src-tauri/src/storage/library_entry.rs`
- `src-tauri/src/commands/library.rs`

**Frontend:**
- `src/pages/LibraryPage.vue`
- `src/pages/GeneratePage.vue`
- `src/components/PackageLibraryBrowser.vue`
- `src/components/PackageCard.vue`
- `src/components/RulebookSelector.vue`
- `src/services/package-library.service.ts`

**Documentation:**
- Update `README.md` with new features
- Add user guide for library and generation

## Success Criteria

✅ Users can install packages from marketplace and they persist  
✅ Library page shows all installed/created packages  
✅ Generate page allows selecting rulebooks from any package  
✅ Prompt generation works with multi-package context  
✅ Package editor can save to and load from library  
✅ All existing functionality continues to work  

## Next Steps

1. Start with Phase 1: Implement Rust storage backend
2. Create basic library.json structure
3. Add Tauri commands for library operations
4. Build Library page UI
5. Integrate with existing flows

---

**Ready to begin implementation!** 🚀

