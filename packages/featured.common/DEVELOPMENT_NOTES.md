# Prompt-Gen Common Package - Development Notes

**Project:** M8.5 - Common Package Development  
**Started:** 2025-12-17  
**Goal:** Build comprehensive `prompt-gen.common` package to validate v1.0.0 specification

---

## Design Phase - Day 1 (REVISED - Significantly Expanded)

### Package Structure - EXPANDED SCOPE

**Package ID:** `prompt-gen.common`  
**Version:** 1.0.0  
**Namespaces:** 10+ sub-namespaces (EXPANDED from 5)

**Foundation Layer:**
1. **common.base** - Core linguistic primitives
2. **common.grammar** - Advanced grammar patterns

**Visual Layer:**
3. **common.visual.color** - Color theory and palettes
4. **common.visual.light** - Lighting and atmosphere
5. **common.visual.material** - Materials and textures
6. **common.visual.form** - Shapes and geometry

**Subject Layer:**
7. **common.subject.living** - People, creatures, plants
8. **common.subject.object** - Inanimate objects and structures
9. **common.subject.environment** - Settings and locations

**Action Layer:**
10. **common.action.motion** - Movement and dynamics
11. **common.action.state** - States and conditions
12. **common.action.interaction** - Relationships between subjects

**Artistic Layer:**
13. **common.style.artistic** - Art movements and techniques
14. **common.style.mood** - Emotional and atmospheric
15. **common.style.era** - Historical and cultural periods

**Technical Layer:**
16. **common.technical.quality** - Quality and detail parameters
17. **common.technical.composition** - Camera and framing
18. **common.technical.effects** - Special effects and post-processing

**TOTAL: 17 namespaces** (instead of 5)

### Why This Scope?

**Original problem:** 30 datatypes, ~350 values - too small to truly validate:
- Emergent generation at scale
- Tag coordination complexity
- Namespace organization
- Whether single YAML scales
- Performance issues
- Missing features

**Revised target:**
- **100+ datatypes** (3x larger)
- **1,500-2,000 values** (5-6x larger)  
- **60+ promptsections** (hierarchical composition)
- **200+ tags** (complex coordination)
- **10-12 level nesting** (test depth limits)
- **Estimated YAML: 100-150 KB** (test file size viability)

**This will REALLY test the spec!**

---

## Namespace 1: common.base (Foundation)

### Purpose
Provide reusable building blocks for all other namespaces. These are the "primitives" that everything else builds on.

### Datatypes Planned

#### 1. articles
**Purpose:** Placeholder for context-selected articles

**Note:** Articles are NOT selected directly from this datatype. Instead:
1. Words (nouns, adjectives) have `article` tag: `{article: a}` or `{article: an}`
2. Rules extract the article from the selected word: `set: article from: ref:noun.tags.article`
3. Template references the context: `{article}` → `target: context:article`

**This datatype exists ONLY for:**
- Documentation (what articles exist)
- Potential future use in edge cases
- NOT for normal selection!

**Values:**
- a (tags: {type: indefinite, phonetic_context: consonant_start})
- an (tags: {type: indefinite, phonetic_context: vowel_start})
- the (tags: {type: definite, phonetic_context: any})

**How it actually works:**
```yaml
# In datatypes - words HAVE article tags
colors:
  values:
    - text: red
      tags: {article: a, warm: true}
    - text: orange  
      tags: {article: an, warm: true}

# In rules - extract article from selected word
rules:
  - set: article
    from: first_selected([ref:adjective, ref:color, ref:noun]).tags.article

# In template - reference context article
template: "{article} {color} {noun}"
references:
  article: {target: context:article, min: 1, max: 1}
  color: {target: common.visual.color:colors, min: 0, max: 1}
  noun: {target: common.subject.living:creatures}
```

**Result:** "an orange dragon" (article comes from "orange")

#### 2. conjunctions
**Values:**
- and (tags: {type: additive, separator_compatible: true})
- or (tags: {type: alternative, separator_compatible: true})
- with (tags: {type: accompaniment})
- featuring (tags: {type: highlight})
- plus (tags: {type: additive, informal: true})

**Purpose:** Connect elements naturally

#### 3. prepositions
**Values:**
- in (tags: {spatial: container, temporal: true})
- on (tags: {spatial: surface})
- at (tags: {spatial: location, temporal: true})
- near (tags: {spatial: proximity})
- under (tags: {spatial: below})
- above (tags: {spatial: above})
- beside (tags: {spatial: adjacent})
- behind (tags: {spatial: rear})
- through (tags: {spatial: passage})

**Purpose:** Spatial and temporal relationships

#### 4. intensifiers
**Values:**
- very (tags: {strength: high, formality: neutral})
- extremely (tags: {strength: very_high, formality: formal})
- slightly (tags: {strength: low, formality: neutral})
- somewhat (tags: {strength: medium_low, formality: formal})
- incredibly (tags: {strength: very_high, formality: informal})
- barely (tags: {strength: minimal, formality: neutral})
- fairly (tags: {strength: medium, formality: neutral})

**Purpose:** Modify intensity of descriptions

#### 5. quantifiers
**Values:**
- one (tags: {numeric: 1, plurality: singular})
- two (tags: {numeric: 2, plurality: plural})
- a few (tags: {numeric_range: [3,5], plurality: plural, vague: true})
- several (tags: {numeric_range: [5,8], plurality: plural, vague: true})
- many (tags: {numeric_range: [10,50], plurality: plural, vague: true})
- countless (tags: {numeric: infinite, plurality: plural, vague: true})

**Purpose:** Quantity specification

### PromptSections Planned

#### basic_noun_phrase
```yaml
template: "{article} {adjective} {noun}"
references:
  article: {target: context:article}
  adjective: {target: common.visual:adjectives, min: 0, max: 1}
  noun: {target: common.composition:subjects}
```

#### compound_phrase
```yaml
template: "{phrase_1} {conjunction} {phrase_2}"
references:
  phrase_1: {target: common.base:basic_noun_phrase}
  conjunction: {target: common.base:conjunctions}
  phrase_2: {target: common.base:basic_noun_phrase}
```

#### spatial_relationship
```yaml
template: "{preposition} {location}"
references:
  preposition: {target: common.base:prepositions, filter: "tags.spatial"}
  location: {target: common.visual:environments}
```

---

## Namespace 2: common.visual (Visual Elements)

### Purpose
Visual descriptors with coordinated properties - colors, textures, materials, lighting, weather, seasons.

### Datatypes Planned

#### 1. colors
**Examples:**
- red (tags: {warm: true, vibrant: high, rgb: [255,0,0], complements: blue})
- blue (tags: {cool: true, vibrant: high, rgb: [0,0,255], complements: orange})
- emerald (tags: {cool: true, vibrant: high, green: true, precious: true})
- crimson (tags: {warm: true, vibrant: high, red: true, dramatic: true})
- ivory (tags: {warm: true, vibrant: low, light: true, elegant: true})
- charcoal (tags: {cool: true, vibrant: low, dark: true, neutral: true})

**~15 values** with rich color theory tags

#### 2. adjectives_visual
**Examples:**
- bright (tags: {luminosity: high, mood: cheerful})
- dark (tags: {luminosity: low, mood: mysterious})
- vivid (tags: {saturation: high, intensity: high})
- muted (tags: {saturation: low, intensity: low})
- shimmering (tags: {effect: reflective, dynamic: true})
- glowing (tags: {effect: luminous, magical: true})

**~20 values**

#### 3. textures
**Examples:**
- smooth (tags: {tactile: soft, reflective: true})
- rough (tags: {tactile: hard, natural: true})
- silky (tags: {tactile: soft, elegant: true, fabric: true})
- weathered (tags: {age: old, natural: true, rustic: true})
- polished (tags: {finish: refined, reflective: true})
- coarse (tags: {tactile: hard, natural: true})

**~15 values**

#### 4. materials
**Examples:**
- wood (tags: {organic: true, warm: true, natural: true})
- metal (tags: {inorganic: true, cool: true, industrial: true})
- glass (tags: {transparent: true, fragile: true, modern: true})
- fabric (tags: {soft: true, flexible: true, textile: true})
- stone (tags: {solid: true, natural: true, permanent: true})
- crystal (tags: {transparent: true, precious: true, magical: true})

**~12 values**

#### 5. lighting
**Examples:**
- soft morning light (tags: {time: morning, quality: diffuse, mood: gentle, temperature: cool})
- harsh midday sun (tags: {time: noon, quality: direct, mood: stark, temperature: warm})
- golden hour (tags: {time: sunset, quality: warm, mood: romantic, temperature: very_warm})
- moonlight (tags: {time: night, quality: dim, mood: mysterious, temperature: cool})
- dramatic backlight (tags: {direction: back, quality: high_contrast, mood: dramatic})

**~12 values** with time/mood coordination

#### 6. weather
**Examples:**
- clear skies (tags: {time_compatible: [noon, afternoon], visibility: high, mood: bright})
- misty (tags: {time_compatible: [morning, evening], visibility: low, mood: mysterious})
- stormy (tags: {time_compatible: [afternoon, evening], visibility: medium, mood: dramatic})
- foggy (tags: {time_compatible: [morning, night], visibility: very_low, mood: eerie})

**~10 values** with time coordination

#### 7. seasons
**Examples:**
- spring (tags: {temperature: mild, vegetation: blooming, mood: fresh})
- summer (tags: {temperature: hot, vegetation: lush, mood: vibrant})
- autumn (tags: {temperature: cool, vegetation: changing, mood: melancholic})
- winter (tags: {temperature: cold, vegetation: dormant, mood: stark})

**4 values** with rich associations

### PromptSections Planned

#### visual_detail
```yaml
template: "{color} {material} with {texture} texture"
```

#### atmosphere
```yaml
template: "{lighting} with {weather} weather"
# With rule to coordinate time_of_day
```

#### environment_mood
```yaml
template: "{season} {atmosphere}"
```

---

## Next Steps

**Today (Day 1):**
1. ✅ Create project structure
2. ✅ Design common.base datatypes (complete above)
3. ✅ Design common.visual datatypes (complete above)
4. 🔄 Design common.composition datatypes (next)
5. ⏳ Design common.style datatypes
6. ⏳ Design common.technical datatypes

**Tomorrow (Day 2):**
- Start implementing YAML
- Begin with common.base namespace
- Add rules for article coordination

---

## Design Decisions

### Decision 1: Tag Richness
**Approach:** Rich tags from the start, even if not all used immediately
**Rationale:** Better to have them and not need them than discover we need them later

### Decision 2: Coordination Strategy
**Approach:** Time-based coordination (lighting ↔ weather, season ↔ atmosphere)
**Rationale:** Natural dependencies that test tag filtering at scale

### Decision 3: Value Count
**Approach:** 10-20 values per datatype (not 3-5)
**Rationale:** Need enough variety for emergent combinations

---

**Status:** Design phase in progress  
**Progress:** 2/5 namespaces designed (40%)

