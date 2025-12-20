# M8.5 Day 2 Plan - Expand and Validate

**Date:** 2025-12-18 (Tomorrow)  
**Starting Point:** Working package with 4 namespaces, 14 datatypes, 130 values  
**Goal:** Expand to 300 values, add landscapes, test at scale

---

## Morning Session (3-4 hours)

### Task 1: Add Landscapes Namespace (1 hour)
**File:** `prompt-gen-common.yaml`

Add `common.nature` namespace with:
```yaml
common.nature:
  id: common.nature
  
  datatypes:
    biomes:
      name: biomes
      values: [10 biomes with tags]
    
    terrain_features:
      name: terrain_features
      values: [10 features with tags]
    
    natural_phenomena:
      name: natural_phenomena  
      values: [8 phenomena with tags]
  
  rules:
    - name: compute_landscape_article
      phase: enrichment
      logic:
        - set: article
          from: "ref:biome.tags.article"
          scope: prompt
  
  prompt_sections:
    landscape_scene:
      name: landscape_scene
      template: "{article} {scale} {biome} with {terrain_feature} under {sky}, {phenomenon}"
```

**Validation:** Run `validate.ps1`, should still pass

### Task 2: Expand Existing Datatypes (1 hour)

**Add to common.fantasy:**
- 5 more creatures (total 15)
- 5 more locations (total 17)

**Add to common.visual (new namespace):**
- colors (12 values)
- scales (8 values)
- textures (8 values)

**Target:** ~220 values total

**Validation:** Run `validate.ps1` after each addition

### Task 3: Generate Test Prompts (1 hour)

**Using desktop app:**
1. Generate 50 fantasy_scene prompts (batch render, different seeds)
2. Generate 50 character_portrait prompts
3. Generate 50 landscape_scene prompts (new!)
4. Save outputs to `test_outputs/`

**Create:** `TEST_RESULTS.md` with:
- All 150 generated prompts
- Quality assessment (good/ok/bad counts)
- Variety assessment (repetition check)
- Article coordination check (all correct?)

---

## Afternoon Session (3-4 hours)

### Task 4: Evaluate Quality (1 hour)

**Review test outputs:**
- Count "good" prompts (natural, interesting, coherent)
- Count "ok" prompts (usable but repetitive)
- Count "bad" prompts (broken, nonsensical)
- Target: 70%+ good, <10% bad

**Document findings:**
- What works well?
- What needs improvement?
- Are there missing datatypes?
- Is variety sufficient?

### Task 5: Add One Creative Category (2 hours)

**Choose:** Game art styles (most demanded)

**Add to common.styles (new namespace):**
```yaml
common.styles:
  id: common.styles
  
  datatypes:
    rendering_styles:
      - low-poly
      - pixel-art
      - voxel
      - vector
      - flat-shaded
      (10 values total)
    
    color_limits:
      - 4-color palette
      - 16-bit
      - monochrome
      (6 values total)
  
  prompt_sections:
    styled_scene:
      template: "{base_scene}, {style}, {color_limit}"
      references:
        base_scene: common.fantasy:fantasy_scene
        style: common.styles:rendering_styles
        color_limit: common.styles:color_limits
```

**Test:** Generate 30 styled prompts
**Example output:** "An ancient dragon perched atop a tower at sunset, low-poly, 16-bit color palette"

### Task 6: Document Findings (30 minutes)

**Create:** `M8.5_DAY2_SUMMARY.md`
- What was added (namespaces, datatypes, values)
- Statistics (total counts)
- Test results (quality metrics)
- Lessons learned
- What's next for Day 3

---

## Evening Session (Optional - 1-2 hours)

### Task 7: Add More Creative Styles (if time permits)

**Either:**
1. Expand game art styles (add more rendering techniques)
2. Add propaganda/period styles
3. Add technical/instructional formats

**Or:** Start on comprehensive tag filtering examples

---

## Success Criteria for Day 2

**Must Have:**
- ✅ 300+ values across all datatypes
- ✅ Landscapes namespace working
- ✅ 150 test prompts generated
- ✅ Quality evaluation documented
- ✅ At least 70% good prompts

**Should Have:**
- ✅ One creative style category added
- ✅ Styled prompts working
- ✅ Cross-category combinations tested

**Could Have:**
- ⏳ Second creative category
- ⏳ Tag filtering examples
- ⏳ Advanced coordination patterns

---

## File Checklist

**To Create:**
- [ ] `test_outputs/fantasy_scenes.txt` (50 prompts)
- [ ] `test_outputs/character_portraits.txt` (50 prompts)
- [ ] `test_outputs/landscapes.txt` (50 prompts)
- [ ] `test_outputs/styled_prompts.txt` (30 prompts)
- [ ] `TEST_RESULTS.md` (quality analysis)
- [ ] `M8.5_DAY2_SUMMARY.md` (day summary)

**To Update:**
- [ ] `prompt-gen-common.yaml` (add namespaces/datatypes)
- [ ] `IMPLEMENTATION_PROGRESS.md` (track progress)
- [ ] `DAY1_PROGRESS.md` (mark Day 1 complete, link to Day 2)

---

## Quick Commands

**Validate package:**
```powershell
cd D:\workspaces\prompt-gen-spec\prompt-gen-common
.\validate.ps1
```

**Generate prompts (desktop app):**
1. Load `prompt-gen-common.yaml`
2. Select prompt section
3. Set batch count to 50
4. Click "Generate Batch"
5. Copy output to test_outputs/

**Check file size:**
```powershell
(Get-Item "prompt-gen-common.yaml").Length / 1KB
# Should be ~30-35 KB at 300 values
```

---

## Timeline Estimate

**Morning:** 3-4 hours
- Landscapes: 1 hour
- Expand datatypes: 1 hour
- Generate tests: 1 hour
- Break: 30 min

**Afternoon:** 3-4 hours
- Evaluate: 1 hour
- Creative styles: 2 hours
- Document: 30 min
- Break: 30 min

**Total:** 6-8 hours of actual work

---

**End of Day 2 Target:**
- ✅ 300+ values
- ✅ 6-8 namespaces
- ✅ 150+ test prompts
- ✅ 1 creative category
- ✅ Quality validated at scale
- ✅ Ready for Day 3 (more creative categories)

**Let's make Day 2 as successful as Day 1!** 🚀

