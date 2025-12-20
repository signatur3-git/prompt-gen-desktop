---
layout: home
title: Random Prompt Generator Specification

hero:
  name: Random Prompt Generator
  text: v1.0.0 Specification
  tagline: A deterministic, template-based system for creating varied, contextually-aware text prompts
  actions:
    - theme: brand
      text: For Package Authors
      link: /guides/getting-started
    - theme: alt
      text: For Implementers
      link: /architecture/overview

features:
  - icon: 🎲
    title: Deterministic
    details: Same seed always produces same output. Perfect for reproducible results and testing.
  
  - icon: 📝
    title: Template-Based
    details: Define patterns, not individual outputs. Create infinite variations from reusable templates.
  
  - icon: 🔗
    title: Context-Aware
    details: Elements coordinate automatically. Proper articles ("a" vs "an"), pluralization, and more.
  
  - icon: 🏷️
    title: Tag Filtering
    details: Select by properties. Only "flying creatures" or "nocturnal animals" - physically plausible outputs.
  
  - icon: 📦
    title: Modular Packages
    details: Reusable content with namespaces. Share and compose packages like npm modules.
  
  - icon: 🎨
    title: Visual Authoring
    details: Desktop app for package creation. No YAML required (but supported for advanced users).
---

## Choose Your Path

<div class="vp-link-grid" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 1.5rem; margin: 2rem 0;">

<div style="padding: 1.5rem; border: 1px solid var(--vp-c-divider); border-radius: 8px;">

### 📝 **I want to create packages**
*Package authors who want to generate prompts*

**Start here:**
1. [Getting Started Guide](/guides/getting-started) - Understand the system (20 min)
2. [Tutorial 1: Basic Package](/guides/tutorial-series/01-basic-package) - Create your first package (30 min)
3. [Tutorial 2: Tag Filtering](/guides/tutorial-series/02-tag-filtering) - Add physical plausibility (45 min)
4. [Tutorial 3: Context Rules](/guides/tutorial-series/03-context-rules) - Coordinate elements (45 min)

**You'll learn:**
- How to create packages in YAML
- Tag filtering for realistic outputs
- Article coordination ("a" vs "an")
- Natural list formatting

**Perfect for:** Text-to-image AI users, game designers, creative writers

</div>

<div style="padding: 1.5rem; border: 1px solid var(--vp-c-divider); border-radius: 8px;">

### 🔧 **I want to implement RPG**
*Developers building RPG in their language*

**Start here:**
1. [Architecture Overview](/architecture/overview) - System design (30 min)
2. [Components & Data Model](/architecture/components) - Core structures (1 hour)
3. [Template Syntax](/architecture/template-syntax) - Parser requirements (1 hour)
4. [Context System](/architecture/context-interactions) - Coordination logic (1 hour)

**You'll learn:**
- Three-phase rendering pipeline
- Data model specifications
- Template parsing requirements
- Context and rules engine

**Perfect for:** Language library developers, tool creators, integration developers

</div>

<div style="padding: 1.5rem; border: 1px solid var(--vp-c-divider); border-radius: 8px;">

### 📖 **I want to understand RPG**
*Researchers, evaluators, or curious minds*

**Start here:**
1. [Getting Started Guide](/guides/getting-started) - What is RPG? (20 min)
2. [Examples: Text-to-Image](/examples/text-to-image-prompts) - Real-world use cases (15 min)
3. [Architecture Overview](/architecture/overview) - How it works (30 min)
4. [Milestone Progress](/milestones/index) - Development history (15 min)

**You'll learn:**
- Core concepts and terminology
- Real-world applications
- Design decisions and rationale
- Current project status

**Perfect for:** Evaluators, students, technical writers, project managers

</div>

</div>

## Current Status

**Version:** 1.0.0  
**Progress:** 8/9 milestones complete (88.9%)  
**Status:** Documentation complete, ready for v1.0.0 release

**Latest Updates:**
- ✅ Complete user documentation (getting-started + 4 tutorials)
- ✅ Consolidated architecture specifications  
- ✅ Working reference implementation (Tauri + Vue + Rust)
- ✅ Comprehensive validation and CLI tools

See [Milestone Progress](/milestones/index) for detailed development history.

## Use Cases

**Perfect for:**
- 🎨 Text-to-image AI prompts (Midjourney, DALL-E, Stable Diffusion)
- ✍️ Creative writing prompts
- 🎮 Game content generation (NPCs, quests, items)
- 📚 Procedural storytelling
- 🎲 Tabletop RPG content

## Key Features Showcase

### Templates with Coordination
```yaml
template: "{article} {adjective} {creature} {action}"
```
Produces: "an ancient dragon soaring" (not "a ancient dragon")

### Tag Filtering
```yaml
template: "{creature#{tags.can_fly}} flying through the sky"
```
Only selects creatures that can actually fly. No "deer flying" output!

### Natural List Formatting
```yaml
{colors?min=2,max=4,sep=comma_and}
```
Produces: "red, blue and green" (not "red blue green")

## Quick Reference

**For Package Authors:**
- [Getting Started](/guides/getting-started)
- [Tutorial Series](/guides/tutorial-series/01-basic-package)
- [Template Syntax Reference](/architecture/template-syntax)
- [Examples](/examples/text-to-image-prompts)

**For Implementers:**
- [Architecture Overview](/architecture/overview)
- [Components & Data Model](/architecture/components)
- [Context System](/architecture/context-interactions)
- [Tag Filtering](/architecture/tag-filtering)

**For Everyone:**
- [Glossary](/glossary) - Terminology reference
- [Milestones](/milestones/index) - Development progress

## Project Info

**Repository:** Prompt Generator Specification  
**License:** MIT  
**Implementation:** Tauri + Vue 3 + Rust (reference)  
**Target Release:** Q1 2026

---

<div style="text-align: center; padding: 2rem 0;">
  <strong>Not sure where to start?</strong><br>
  <a href="/guides/getting-started">Getting Started Guide</a> is a good choice for everyone!
</div>

Issues and pull requests are welcome. Please cite relevant sections of the RFC when proposing changes so reviewers can evaluate impacts across the ecosystem.
