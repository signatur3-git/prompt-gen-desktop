# M8 Phase 3: Spec Consolidation - Task List

**Date:** 2025-12-17  
**Status:** 🔄 **IN PROGRESS**

---

## Objective

Review and update all architecture documentation to:
1. Match M1-M7 implementation reality
2. Align with tutorial examples
3. Remove/defer features not in v1.0.0
4. Ensure consistency across all docs

---

## Architecture Files to Review

### File 1: docs/architecture/overview.md ⏳

**Current state:** References advanced features not in v1.0.0

**Issues found:**
- ❌ Mentions "Rulebooks" (deferred to v1.1.0+)
- ❌ Mentions "Pools" (not implemented)
- ❌ Mentions "Morphers" (not implemented)
- ❌ Mentions "ContextInterface" as complex contract (simplified in v1.0.0)
- ❌ Mentions "Marketplace" component (future)
- ⚠️ May not reflect actual M2-M7 implementation

**Actions needed:**
- [ ] Update component list to v1.0.0 scope
- [ ] Remove references to unimplemented features
- [ ] Add references to tutorials
- [ ] Align with DEVELOPMENT_PLAN.md milestones
- [ ] Match implementation in reference-impl

---

### File 2: docs/architecture/components.md ⏳

**Status:** Not yet reviewed

**Actions needed:**
- [ ] Review component descriptions
- [ ] Ensure matches data models from M2
- [ ] Check references to separator sets (M5)
- [ ] Verify tag filtering coverage (M4)
- [ ] Update examples to match tutorials

---

### File 3: docs/architecture/context-interactions.md ⏳

**Status:** Not yet reviewed

**Actions needed:**
- [ ] Verify context system matches M4 implementation
- [ ] Check rule syntax matches validator
- [ ] Ensure examples work with current spec
- [ ] Reference Tutorial 3 (Context Rules)
- [ ] Document actual `first_selected()` function

---

### File 4: docs/architecture/tag-filtering.md ⏳

**Status:** Not yet reviewed

**Actions needed:**
- [ ] Verify tag expression syntax (M4/M5)
- [ ] Check operators (&&, ||, !)
- [ ] Ensure examples match Tutorial 2
- [ ] Document string/numeric comparisons
- [ ] Add cross-references to tutorials

---

### File 5: docs/architecture/template-syntax.md ⏳

**Status:** Not yet reviewed

**Actions needed:**
- [ ] Document min/max/sep/unique parameters (M5)
- [ ] Show inline tag filter syntax
- [ ] Reference Tutorial 4 (Lists)
- [ ] Update examples to working YAML
- [ ] Document all parameter combinations

---

### File 6: docs/architecture/engine-primitives.md ⏳

**Status:** Not yet reviewed

**Actions needed:**
- [ ] Review against M3 rendering engine
- [ ] Document three-phase rendering
- [ ] Explain seeded RNG behavior
- [ ] Reference implementation details
- [ ] Remove unimplemented primitives

---

## Source-of-Truth Files to Review

### source-of-truth/component-overview.md ⏳

**Actions needed:**
- [ ] Check against M2 data models
- [ ] Verify ER relationships
- [ ] Update with M4 context system
- [ ] Align with tutorials

### source-of-truth/template-engine.md ⏳

**Actions needed:**
- [ ] Match M3 three-phase rendering
- [ ] Document actual parser implementation
- [ ] Reference Tutorial examples
- [ ] Update technical details

### source-of-truth/context-interactions.md ⏳

**Actions needed:**
- [ ] Match M4 context implementation
- [ ] Document rule execution order
- [ ] Show actual syntax
- [ ] Reference Tutorial 3

---

## Examples to Update

### docs/examples/text-to-image-prompts.md ⏳

**Actions needed:**
- [ ] Ensure all examples are valid YAML
- [ ] Reference tutorials for techniques used
- [ ] Add expected outputs
- [ ] Match current spec version

### docs/examples/authoring-analysis.md ⏳

**Actions needed:**
- [ ] Review relevance to v1.0.0
- [ ] Update patterns to match tutorials
- [ ] Remove references to unimplemented features

---

## Cross-Reference Updates

### Getting-Started Guide

**Actions needed:**
- [ ] Verify all architecture links work
- [ ] Ensure examples match architecture docs
- [ ] Check tutorial references are correct

### Tutorial Series

**Actions needed:**
- [ ] Verify architecture doc links
- [ ] Ensure examples align with spec
- [ ] Check all code is valid per spec

---

## Consistency Checks

### Version Alignment

**Check:**
- [ ] All docs reference v1.0.0 scope
- [ ] Features marked as v1.1.0+ where appropriate
- [ ] No promises of unavailable features
- [ ] Timeline expectations clear (Q1 2026)

### Terminology Consistency

**Verify same terms used:**
- [ ] "Datatype" (not "ValueSet" or "List")
- [ ] "PromptSection" (not "Template" alone)
- [ ] "Reference" (not "Binding" or "Link")
- [ ] "Separator Set" (not "Formatter")
- [ ] "Tag Filtering" (not "Tag Selection")
- [ ] "Context" (not "State" or "Store")
- [ ] "Rule" (not "Hook" or "Processor")

### Example Validity

**Ensure all YAML examples:**
- [ ] Have correct structure
- [ ] Use valid namespace:component format
- [ ] Include all required fields
- [ ] Don't reference unimplemented features
- [ ] Match tutorial examples

---

## Priority Order

### High Priority (Must Do)

1. **overview.md** - First point of contact
2. **components.md** - Core reference
3. **template-syntax.md** - Most used by authors
4. **context-interactions.md** - Complex topic needs clarity

### Medium Priority (Should Do)

5. **tag-filtering.md** - Well covered in Tutorial 2
6. **engine-primitives.md** - Technical but important
7. **source-of-truth/** files - Reference for implementers

### Low Priority (Nice to Have)

8. **examples/** - Can be enhanced over time
9. Additional diagrams
10. Video content (post v1.0.0)

---

## Approach

### For Each File

1. **Read current content**
2. **Identify issues:**
   - Unimplemented features mentioned
   - Outdated syntax
   - Missing cross-references
   - Inconsistent terminology
3. **Update content:**
   - Remove/defer unimplemented features
   - Add tutorial cross-references
   - Fix examples
   - Align with implementation
4. **Verify:**
   - Check against DEVELOPMENT_PLAN.md
   - Match tutorials
   - Validate examples

---

## Success Criteria

### Phase 3 Complete When:

- [ ] All architecture docs reviewed
- [ ] Unimplemented features removed/deferred
- [ ] Examples match v1.0.0 spec
- [ ] Cross-references to tutorials added
- [ ] Terminology consistent
- [ ] All YAML examples valid
- [ ] Links verified

---

## Next Steps

**Start with:** overview.md (highest priority, first contact point)

**Then:** components.md, template-syntax.md, context-interactions.md

**Finally:** Remaining files and examples

---

## Status Tracking

**Files Reviewed:** 0/6 architecture + 0/3 source-of-truth  
**Examples Updated:** 0/2  
**Cross-References Added:** 0  
**Terminology Fixes:** 0

---

**Ready to begin Phase 3!** 🚀

