# VitePress Configuration for Restructured Documentation

**Created:** 2025-12-16  
**Purpose:** Guide for updating VitePress navigation after doc restructuring

---

## What Changed

The VitePress config has been updated to match the new RFC-ready documentation structure.

### Old Structure
```
- Overview
- Architecture
- Sanity Checks
- RFC
```

### New Structure
```
- Home
- Specification (Core, Extensions, Conformance)
- Guides (Implementors, Authors, Tutorials)
- Reference (Examples, Patterns, API)
- Glossary
```

---

## New Navigation

### Top-Level Nav

**Specification** - Dropdown with:
- Core Spec → `/spec/core/00-introduction`
- Extensions → `/spec/extensions/`
- Conformance → `/spec/core/10-conformance`

**Guides** - Dropdown with:
- Implementors → `/guides/implementors/00-overview`
- Package Authors → `/guides/authors/00-getting-started`
- Tutorials → `/guides/tutorials/01-hello-world`

**Reference** - Dropdown with:
- Examples → `/reference/examples/`
- Patterns → `/reference/patterns/`
- API Reference → `/reference/api/`

### Sidebars

Each major section has its own sidebar:

**`/spec/core/`** - 12 numbered sections (00-11)
- 0. Introduction
- 1. Architecture
- 2. Package Format
- 3. Data Model
- 4. Template Language
- 5. Rendering Engine
- 6. Context System
- 7. Coordination (from M1)
- 8. Validation
- 9. Security
- 10. Conformance
- 11. Package Registry

**`/spec/extensions/`** - Optional features
- Morphers
- Pools
- LLM Integration
- Internationalization
- Custom Processors

**`/guides/implementors/`** - Per-component guides
- 0. Overview
- 1. Rendering Engine
- 2. Package Validator
- 3. Authoring Tool
- 4. Package Marketplace
- 5. Reusable Libraries

**`/guides/authors/`** - Package authoring
- 0. Getting Started
- 1. Creating Datatypes
- 2. Writing Templates
- 3. Coordination Patterns
- 4. Testing Packages
- 5. Publishing
- 6. Best Practices

**`/guides/tutorials/`** - Step-by-step
- 1. Hello World
- 2. Simple Prompts
- 3. Optional Elements
- 4. Pluralization
- 5. Complex Scenes
- 6. Character Generator

**`/reference/examples/`** - Complete packages
**`/reference/patterns/`** - Design patterns
**`/reference/api/`** - API documentation

---

## Migration Checklist

When you restructure the documentation (M1 bonus task):

### Phase 1: Create Placeholder Files

Create stub files for all navigation entries so VitePress doesn't throw errors:

```bash
# Create directory structure
mkdir -p docs/spec/{core,extensions,conformance}
mkdir -p docs/guides/{implementors,authors,tutorials}
mkdir -p docs/reference/{examples,patterns,api}

# Create stub files for spec/core
for i in {00..11}; do
  touch "docs/spec/core/$i-placeholder.md"
done

# Create stub files for guides
touch docs/guides/implementors/{00..05}-placeholder.md
touch docs/guides/authors/{00..06}-placeholder.md
touch docs/guides/tutorials/{01..06}-placeholder.md

# Create index files
touch docs/spec/extensions/index.md
touch docs/reference/examples/index.md
touch docs/reference/patterns/index.md
touch docs/reference/api/index.md
```

### Phase 2: Move Existing Content

Map old files to new locations:

```bash
# Example moves (adjust as needed)
mv docs/rfc/0001-*.md docs/spec/core/00-introduction.md
mv docs/architecture/overview.md docs/spec/core/01-architecture.md
mv docs/architecture/components.md docs/spec/core/03-data-model.md
mv docs/architecture/template-syntax.md docs/spec/core/04-template-language.md
mv docs/architecture/context-interactions.md docs/spec/core/06-context-system.md
```

### Phase 3: Update Cross-References

Search for old links and update:

```bash
# Find all markdown files with old links
grep -r "](/architecture/" docs/
grep -r "](/rfc/" docs/
grep -r "](/sanity-checks/" docs/

# Update to new paths
# /architecture/components -> /spec/core/03-data-model
# /rfc/0001-* -> /spec/core/00-introduction
# etc.
```

### Phase 4: Create Stub Content

For placeholder files, use this template:

```markdown
---
title: [Section Name]
status: Draft
---

# N. Section Name

> **Status:** This section is being migrated and will be updated during M1-M8.

## Coming Soon

This section will contain:
- [Topic 1]
- [Topic 2]
- [Topic 3]

## Related Sections

- [Link to related section]

---

**Last Updated:** [Date]
```

### Phase 5: Test VitePress

```bash
cd docs
npm install
npx vitepress dev
```

Visit:
- http://localhost:5173/ - Home
- http://localhost:5173/spec/core/00-introduction - Spec
- http://localhost:5173/guides/implementors/00-overview - Guides
- http://localhost:5173/reference/examples/ - Reference

Verify:
- All nav links work
- Sidebars appear correctly
- No 404 errors
- Search works

---

## Features Added

### Search
```typescript
search: {
  provider: 'local'
}
```
Built-in local search across all documentation.

### Edit Links
```typescript
editLink: {
  pattern: 'https://github.com/yourusername/prompt-gen-spec/edit/main/docs/:path',
  text: 'Edit this page on GitHub'
}
```
Update the URL to match your repository.

### Footer
```typescript
footer: {
  message: 'Released under the MIT License.',
  copyright: 'Random Prompt Generator Specification v1.0.0-rc1'
}
```

### Social Links
```typescript
socialLinks: [
  {icon: 'github', link: 'https://github.com/yourusername/prompt-gen-spec'}
]
```

---

## Customization

### Update GitHub URLs

In `.vitepress/config.ts`, replace:
```typescript
pattern: 'https://github.com/yourusername/prompt-gen-spec/edit/main/docs/:path'
// and
link: 'https://github.com/yourusername/prompt-gen-spec'
```

With your actual repository URL.

### Add More Navigation

To add new sections:

1. **Add to nav array:**
```typescript
{
  text: 'New Section',
  items: [
    {text: 'Subsection', link: '/new/subsection'}
  ]
}
```

2. **Add sidebar:**
```typescript
'/new/': [
  {
    text: 'New Section',
    items: [
      {text: 'Page 1', link: '/new/page1'},
      {text: 'Page 2', link: '/new/page2'}
    ]
  }
]
```

3. **Create files:**
```bash
mkdir -p docs/new
touch docs/new/page1.md docs/new/page2.md
```

### Customize Theme

VitePress supports custom themes. See [VitePress docs](https://vitepress.dev/guide/custom-theme).

---

## Progressive Enhancement

You don't have to create all files at once. The migration can happen gradually:

### Phase 1 (M1 Bonus)
- ✅ Update config.ts (DONE)
- Create stub files for all nav entries
- Move core spec files (rfc/ → spec/core/)
- Test that navigation works

### Phase 2 (M2-M4)
- Write implementation guides as we build
- Add examples from test packages
- Document patterns from M1 analysis

### Phase 3 (M5-M7)
- Fill remaining spec sections
- Complete all tutorials
- Add API documentation

### Phase 4 (M8)
- Polish all content
- Ensure completeness
- Final review

---

## Benefits of New Config

### Better Organization
- Clear separation: Spec vs Guides vs Reference
- Hierarchical navigation
- Easy to find what you need

### Better UX
- Search across all docs
- Edit links for contributions
- Clear current section in sidebar
- Breadcrumbs and nav hints

### Better for Implementors
- "I want to build X" → Go to Guides → Implementors → Component
- Progressive disclosure: Tutorial → Guide → Spec
- Component isolation: Can read just what you need

### Better for Authors
- "How do I solve X?" → Go to Reference → Patterns → Pattern
- Step-by-step tutorials
- Best practices documented

### RFC-Ready
- Numbered spec sections (like IETF RFCs)
- Clear normative vs informative
- Professional presentation
- Ready for external review

---

## Troubleshooting

### "Page not found" errors

Create stub files for all nav entries:
```markdown
# [Page Title]

> This page is under construction.

Check back soon!
```

### Sidebar not showing

Ensure the sidebar key matches the route:
- `/spec/core/` sidebar shows on `/spec/core/*` pages
- `/guides/authors/` sidebar shows on `/guides/authors/*` pages

### Links broken after move

Update all internal links to new paths. Use search:
```bash
grep -r "](/old/path/" docs/
```

### VitePress won't start

Check for syntax errors in `config.ts`:
```bash
npx vitepress dev
# Look for error messages
```

---

## Next Steps

1. **Now (M1 Bonus):** Create stub files, move core content
2. **M2-M7:** Fill in content as we develop
3. **M8:** Polish and finalize

---

## Resources

- [VitePress Documentation](https://vitepress.dev/)
- [VitePress Theme Config](https://vitepress.dev/reference/default-theme-config)
- [Mermaid Plugin](https://github.com/emersonbottero/vitepress-plugin-mermaid)

---

**The config is ready! Now create the directory structure and stub files.** 🚀

