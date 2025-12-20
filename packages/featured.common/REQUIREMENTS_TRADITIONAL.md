# Requirements Extraction - Traditional Foundation (Categories 1-7)

**Date:** 2025-12-17  
**Source:** 140 desired prompts from `DESIRED_PROMPTS.md`  
**Phase:** Extract minimal requirements for traditional categories

---

## Methodology

For each traditional category (1-7), analyze prompts to identify:
1. **Core subjects** - What entities appear?
2. **Modifiers** - What describes them?
3. **Context** - Time, weather, lighting, location
4. **Relationships** - Spatial, temporal, compositional
5. **Coordination needs** - What must align?

---

## Category 1: Fantasy Scenes (10 prompts analyzed)

### Example Prompts
1. "An ancient dragon perched atop a crumbling tower at sunset..."
2. "A mystical forest glade where ethereal wisps of light dance..."
3. "A battle-worn knight kneeling before a sacred altar..."

### Required Datatypes (Minimal)

**Creatures (mythical):**
- dragon, phoenix, unicorn, griffin (core 4)
- Tags: {article, can_fly, size, magical_level, alignment}

**Locations (fantasy):**
- tower, temple, forest glade, floating island, crossroads, cave, ruins, garden, fortress (core 9)
- Tags: {article, condition, magical, indoor/outdoor}

**Ages/Conditions:**
- ancient, crumbling, battle-worn, forgotten, sacred, mystical, enchanted (core 7)
- Tags: {applies_to: [creature/location/object], intensity}

**Times of Day:**
- sunset, dawn, midnight, twilight, golden hour (core 5)
- Tags: {light_color, light_intensity, mood}

**Weather/Atmosphere:**
- storm clouds, mist, fog, clear, swirling clouds (core 5)
- Tags: {visibility, drama_level, compatible_times}

**Lighting Qualities:**
- dramatic, ethereal, divine, soft, harsh, golden (core 6)
- Tags: {intensity, color, mood, source_type}

**Magical Phenomena:**
- wisps of light, aurora, spectral lanterns, glowing crystals (core 4)
- Tags: {color, intensity, movement}

### Required PromptSections

**fantasy_scene_core:**
```yaml
template: "{age} {creature} {spatial_action} {location_state} {time_context}, {lighting_detail}"
```

**Components needed:**
- age (ancient, mystical, etc.)
- creature (dragon, etc.)
- spatial_action (perched atop, standing before, etc.)
- location (tower, temple, etc.)
- location_state (crumbling, sacred, etc.)
- time (sunset, etc.)
- lighting (dramatic golden light, etc.)

### Required Rules

**Article coordination:**
```yaml
rules:
  - set: article
    from: first_selected([ref:age, ref:creature]).tags.article
```

**Time → Lighting color:**
```yaml
# sunset → golden/orange
# dawn → pink/soft
# midnight → cool/blue
```

**Weather → Atmosphere:**
```yaml
# storm → dramatic
# mist → mysterious
```

---

## Category 2: Character Portraits (10 prompts analyzed)

### Example Prompts
1. "Close-up portrait of an elderly wizard with piercing blue eyes..."
2. "Young elven archer with bright emerald eyes and braided silver hair..."

### Required Datatypes (Minimal)

**Character Archetypes:**
- wizard, warrior, archer, fairy, assassin, queen, druid, mage, pirate (core 9)
- Tags: {article, age_range, gender, profession_type}

**Ages (people):**
- elderly, young, middle-aged, teenage, ancient (core 5)
- Tags: {numeric_range, applies_to: people}

**Facial Features:**
- eyes, skin, beard, hair, face (core 5)
- Tags: {body_part_type, requires_age_compat}

**Feature Qualities:**
- piercing, weathered, flowing, braided, scarred, delicate (core 6)
- Tags: {applies_to: [eyes/skin/hair/etc], intensity}

**Colors (for features):**
- blue, emerald, silver, white, amber, gray, brown, auburn (core 8)
- Tags: {article, warm/cool, vibrant/muted}

**Lighting (portrait):**
- soft candlelight, warm golden hour, dramatic side lighting, diffused (core 4)
- Tags: {direction, quality, mood}

**Atmosphere (portrait):**
- mysterious, confident, wise, ethereal, determined (core 5)
- Tags: {emotion, intensity}

### Required PromptSections

**character_portrait:**
```yaml
template: "{framing} portrait of {age} {archetype} with {features}"
```

**features (list):**
```yaml
template: "{feature_list?min=2,max=3,sep=comma_and}"
```

**feature_item:**
```yaml
template: "{quality} {color} {body_part}"
```

### Required Rules

**Article from archetype:**
```yaml
rules:
  - set: article
    from: ref:archetype.tags.article
```

**Feature → Quality compatibility:**
```yaml
# eyes can be: piercing, bright, glowing
# skin can be: weathered, smooth, scarred
# hair can be: flowing, braided, wild
```

---

## Category 3: Natural Landscapes (10 prompts analyzed)

### Example Prompts
1. "Vast desert with towering sand dunes under blazing sun..."
2. "Frozen tundra with northern lights overhead..."

### Required Datatypes (Minimal)

**Biomes:**
- desert, tundra, jungle, volcanic, meadow, mountains, coast, forest, valley, savanna (core 10)
- Tags: {article, climate, vegetation_level, elevation}

**Scale Modifiers:**
- vast, towering, massive, endless, gentle, jagged (core 6)
- Tags: {size_category, applies_to}

**Terrain Features:**
- sand dunes, ice formations, ancient trees, lava rivers, wildflowers, peaks, cliffs (core 7)
- Tags: {article, biome_compatible, natural_formation}

**Sky Conditions:**
- blazing sun, pale winter sky, thick canopy, ash clouds, clear afternoon (core 5)
- Tags: {weather_type, time_compatible, light_level}

**Natural Phenomena:**
- northern lights, heat shimmer, mist rising, spray creating rainbows, butterflies (core 5)
- Tags: {biome_compatible, time_compatible, visibility_effect}

**Colors (landscape):**
- golden, green, purple, orange, crimson, pink, white (core 7)
- Tags: {article, natural_occurrence, time_association}

### Required PromptSections

**landscape_scene:**
```yaml
template: "{scale} {biome} with {terrain_features} under {sky}, {phenomenon}"
```

### Required Rules

**Biome → Compatible features:**
```yaml
# desert → sand dunes, oasis
# tundra → ice formations, aurora
# jungle → ancient trees, canopy
```

**Time → Sky color:**
```yaml
# sunrise → pink/gold
# noon → blazing/bright
# sunset → orange/crimson
```

---

## Summary: Minimal Foundation Requirements

### Total Datatypes Needed (Traditional): ~30

**Subjects (15):**
1. creatures_mythical (10 values)
2. creatures_real (15 values) 
3. character_archetypes (10 values)
4. locations_fantasy (12 values)
5. locations_natural (10 values)
6. structures (12 values)
7. terrain_features (10 values)
8. body_parts (8 values)
9. objects (12 values)
10. natural_phenomena (8 values)
11. magical_phenomena (6 values)
12. vegetation (8 values)
13. weather_elements (8 values)
14. celestial (6 values)
15. abstract_concepts (8 values)

**Modifiers (10):**
16. ages (8 values)
17. sizes (8 values)
18. conditions (10 values)
19. qualities (12 values)
20. colors (15 values)
21. feature_qualities (10 values)
22. scale_modifiers (8 values)
23. intensity_levels (6 values)
24. textures (8 values)
25. materials (8 values)

**Context (5):**
26. times_of_day (8 values)
27. lighting_qualities (10 values)
28. lighting_sources (8 values)
29. moods (10 values)
30. atmospheres (8 values)

**Total estimated values: ~280-300**

### Core Tag Dimensions

**Essential tags all values need:**
1. `article` - a/an/the (phonetic-based)
2. `type` - what kind of thing it is
3. `category` - broader grouping

**Coordination tags:**
4. `can_fly`, `can_swim`, `can_run` - capability
5. `time_compatible` - what times work
6. `weather_compatible` - what weather works
7. `biome_compatible` - what environments work
8. `applies_to` - what it can modify
9. `age_range` - for characters
10. `size_category` - small/medium/large/huge
11. `condition_type` - pristine/worn/ruined
12. `magical_level` - mundane/magical/divine

**Visual tags:**
13. `color_category` - warm/cool
14. `vibrant` - high/medium/low
15. `light_level` - bright/medium/dim
16. `visibility` - clear/obscured/hidden

### Spatial Relationships Needed

**Prepositions (15 values):**
- atop, before, beside, within, beneath, above, around, through, across, along, between, among, near, under, over

**Spatial Verbs (12 values):**
- perched, standing, kneeling, floating, hovering, resting, lying, sitting, flying, swimming, walking, running

**Compositional Terms (8 values):**
- in foreground, in background, in center, to left, to right, in distance, close-up, wide shot

---

## Hierarchical Structure (Traditional)

### Top-Level Sections

**1. fantasy_scene**
```yaml
template: "{positioned_subject} {location_context}, {atmospheric_detail}"

positioned_subject:
  template: "{age} {creature} {spatial_verb} {location_state}"
  
location_context:
  template: "{preposition} {location} {time}"
  
atmospheric_detail:
  template: "{lighting} {weather_phenomenon}"
```

**2. character_portrait**
```yaml
template: "{framing} portrait of {age} {archetype} with {features}, {lighting}, {mood}"

features:
  template: "{feature_list?min=2,max=3,sep=comma_and}"
  
feature_list:
  template: "{quality} {color} {body_part}"
```

**3. landscape**
```yaml
template: "{scale} {biome} with {terrain_feature} under {sky}, {phenomenon}"
```

### Depth Analysis

**Maximum nesting: 4-5 levels**
- Level 1: Top section (fantasy_scene)
- Level 2: Sub-sections (positioned_subject, location_context)
- Level 3: Components (age, creature, location)
- Level 4: Datatypes (values with tags)
- Level 5: Context references (article from tags)

**Within 10-level limit** ✅

---

## Next Steps

**Phase 1 (Today):**
1. Implement these 30 datatypes with ~300 values
2. Add article tags to all values
3. Create 3 top-level promptsections (fantasy, portrait, landscape)
4. Add basic rules (article coordination, time→lighting)

**Phase 2 (Tomorrow):**
1. Test generation (50 prompts from each category)
2. Evaluate quality and variety
3. Identify gaps
4. Add creative categories on top

**File to create:**
- `prompt-gen-common.yaml` - Starting with traditional foundation

---

**Status:** Requirements extracted, ready to implement! ✅

