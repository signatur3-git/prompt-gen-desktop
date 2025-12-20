# M8.5 APPROACH CORRECTION - Start with Desired Prompts

**Date:** 2025-12-17  
**Critical Issue:** We're designing tools instead of outcomes  
**Status:** ⚠️ **APPROACH REVERSED**

---

## The Problem

**What we've been doing:**
- Designing datatypes (colors, creatures, materials)
- Organizing namespaces (visual, subject, action)
- Planning 100+ datatypes with 2,500+ values
- Tool-first, output-second

**Why this is wrong:**
- We don't know if these tools produce good prompts
- We're guessing at what's needed
- The tool dictates the outcome, not the author
- We might build 2,500 values that produce repetitive garbage

**Your insight:** "The tool will dictate the outcome, not the author/developer"

**100% CORRECT!** ✅

---

## The Correct Approach

### Step 1: Define DESIRED Prompts

**Start with what we WANT to generate.** Examples:

#### Category: Fantasy Scenes

**Desired outputs:**
1. "An ancient dragon perched atop a crumbling tower at sunset, dramatic golden light breaking through storm clouds"
2. "A mystical forest glade where ethereal wisps of light dance between towering silver birches under a full moon"
3. "A battle-worn knight kneeling before a sacred altar in a forgotten temple, rays of divine light streaming through stained glass"
4. "A floating island suspended in swirling clouds, waterfalls cascading into the void below, aurora dancing overhead"
5. "A shadowy figure cloaked in darkness standing at a crossroads where ancient paths meet, spectral lanterns flickering"

#### Category: Character Portraits

**Desired outputs:**
1. "Close-up portrait of an elderly wizard with piercing blue eyes, weathered skin, and flowing white beard, soft candlelight, mysterious atmosphere"
2. "Young elven archer with emerald eyes and braided silver hair, confident stance, forest backdrop, golden hour lighting"
3. "Scarred warrior in battle-worn armor, determined expression, dramatic side lighting highlighting facial features"
4. "Ethereal fairy with translucent wings and glowing aura, delicate features, surrounded by floating magical particles"
5. "Hooded assassin shrouded in shadow, only glowing amber eyes visible, misty background"

#### Category: Landscapes

**Desired outputs:**
1. "Vast desert expanse with towering sand dunes under a blazing sun, heat shimmer visible, distant oasis"
2. "Frozen tundra stretching to the horizon, northern lights painting the sky in brilliant greens and purples"
3. "Tropical jungle with massive ancient trees, vines hanging thick, shafts of sunlight piercing the canopy, mist rising"
4. "Volcanic landscape with rivers of molten lava, dark ash clouds, glowing fissures, hellish red illumination"
5. "Peaceful meadow filled with wildflowers, gentle rolling hills, soft afternoon light, butterflies dancing"

#### Category: Abstract/Conceptual

**Desired outputs:**
1. "The concept of time visualized as a spiral of clocks and hourglasses dissolving into cosmic dust"
2. "Dreams manifesting as colorful geometric shapes floating in an ethereal void, soft gradients blending"
3. "The feeling of nostalgia represented by faded sepia-toned memories overlapping in misty layers"
4. "Chaos and order colliding - one half fractured sharp edges, other half smooth flowing curves"
5. "The passage of seasons shown in a single tree transforming through spring bloom, summer green, autumn gold, winter bare"

#### Category: Architectural

**Desired outputs:**
1. "Gothic cathedral interior with soaring vaulted ceilings, light streaming through rose windows, dust motes floating"
2. "Futuristic cityscape at night, neon lights reflecting in rain-slicked streets, holographic advertisements flickering"
3. "Ancient ruins overgrown with vines and moss, broken columns reaching toward the sky, nature reclaiming civilization"
4. "Cozy cottage with warm light glowing from windows, smoke rising from chimney, snow falling gently"
5. "Impossible architecture defying gravity, staircases leading nowhere, Escher-like contradictions"

---

## Step 2: Reverse-Engineer Requirements

**For each desired prompt, ask:**

### Example: "Ancient dragon perched atop crumbling tower at sunset"

**What do we actually need?**

**Subject elements:**
- dragon (mythical creature)
- tower (structure)
- Relationship: dragon + tower + "perched atop"

**State/condition:**
- ancient (age descriptor)
- crumbling (decay state)

**Lighting/time:**
- sunset (specific time)
- golden light (quality derived from sunset)
- dramatic (mood modifier for light)

**Weather/atmosphere:**
- storm clouds (weather condition)
- breaking through (light-weather interaction)

**What this reveals we need:**
1. Creatures with age descriptors
2. Structures with decay states
3. Spatial relationships (perched atop, standing before, etc.)
4. Times of day with implied lighting qualities
5. Weather conditions
6. Light-weather interactions
7. Mood modifiers

---

### Example: "Mystical forest glade, ethereal wisps, silver birches, full moon"

**What do we actually need?**

**Location:**
- forest glade (specific environment type)
- mystical (mood/magical quality)

**Phenomena:**
- wisps of light (magical effect)
- ethereal (quality of wisps)
- dancing (movement type)

**Vegetation:**
- birches (specific tree type)
- towering (size modifier)
- silver (color descriptor)

**Sky:**
- full moon (celestial object + phase)

**Spatial:**
- between (relationship)
- under (relationship)

**What this reveals we need:**
1. Specific environment types (glade, clearing, grove)
2. Mood/magical qualities that apply to locations
3. Magical phenomena (wisps, particles, auras)
4. Specific plant types with descriptors
5. Celestial objects with states
6. Spatial prepositions for relationships

---

### Example: "Close-up portrait, elderly wizard, piercing eyes, weathered skin"

**What do we actually need?**

**Framing:**
- close-up (camera distance)
- portrait (composition type)

**Subject:**
- wizard (character archetype)
- elderly (age)

**Features:**
- eyes (body part)
  - piercing (quality)
  - blue (color)
- skin (body part)
  - weathered (texture/age)
- beard (body part)
  - flowing (movement quality)
  - white (color)

**Lighting:**
- soft (quality)
- candlelight (source)

**Mood:**
- mysterious (atmosphere)

**What this reveals we need:**
1. Camera framing terms
2. Composition types
3. Character archetypes with age ranges
4. Body parts
5. Feature qualities (piercing, weathered, flowing)
6. Colors for features
7. Lighting sources with implied qualities
8. Atmospheric moods

---

## Step 3: Pattern Recognition

**Looking across all examples, what patterns emerge?**

### Pattern 1: Hierarchical Composition

Most prompts follow structure:
```
[Subject] [doing/being] [something] [in location] [at time] [with lighting] [with mood]
```

**But components can be:**
- Simple: "dragon" 
- Modified: "ancient dragon"
- Positioned: "ancient dragon perched atop tower"
- Contextualized: "ancient dragon perched atop crumbling tower at sunset"

### Pattern 2: Layered Details

Details stack in layers:
1. **Core subject** - dragon, wizard, forest
2. **Primary modifiers** - ancient, elderly, mystical
3. **Spatial context** - perched atop, in glade, at crossroads
4. **Temporal context** - sunset, full moon, golden hour
5. **Atmospheric context** - storm clouds, misty, ethereal
6. **Mood/quality** - dramatic, mysterious, peaceful

### Pattern 3: Cross-Domain Coordination

Elements must coordinate across domains:
- Time → Lighting (sunset → golden light)
- Weather → Atmosphere (storm → dramatic)
- Age → Texture (ancient → weathered)
- Magic → Visual effects (mystical → wisps, glow)

### Pattern 4: Specificity Gradients

Prompts can be more or less specific:
- Generic: "forest"
- Specific: "forest glade"
- Very specific: "mystical forest glade with silver birches"

System must support all levels.

---

## Step 4: Identify Required Datatypes (From Needs)

**Based on desired prompts, we ACTUALLY need:**

### Core Subjects (Must-Have)
- Characters (archetypes: wizard, warrior, fairy, etc.)
- Creatures (mythical + real: dragon, horse, bird, etc.)
- Vegetation (trees, flowers, plants with specific types)
- Structures (towers, temples, cottages, etc.)
- Locations (glades, deserts, mountains, etc.)

### Modifiers (Must-Have)
- Age descriptors (ancient, elderly, young, new)
- Size modifiers (towering, massive, tiny, vast)
- Condition states (crumbling, weathered, pristine, battle-worn)
- Quality descriptors (ethereal, mystical, sacred, profane)
- Colors (for features, objects, lighting)

### Spatial/Temporal (Must-Have)
- Spatial relationships (atop, before, between, within)
- Times of day (sunset, dawn, midnight, golden hour)
- Celestial states (full moon, crescent moon, eclipse)
- Seasons (with visual implications)

### Lighting/Atmosphere (Must-Have)
- Light sources (sun, moon, candle, divine light)
- Light qualities (soft, harsh, golden, dramatic)
- Weather (storm, fog, clear, rain)
- Atmospheric effects (mist, dust motes, particles)

### Compositional (Must-Have)
- Camera angles (close-up, wide shot, bird's eye)
- Composition rules (centered, rule of thirds)
- Focus types (portrait, landscape, detail)

### Effects (Nice-to-Have)
- Magical phenomena (wisps, auras, glows)
- Motion descriptors (flowing, cascading, dancing)
- Textures (smooth, rough, weathered)
- Materials (stone, wood, metal)

---

## Step 5: Test-Driven Design

**For EACH desired prompt category:**

### 1. Fantasy Scenes - Example Breakdown

**Prompt:** "Ancient dragon perched atop crumbling tower at sunset, dramatic golden light breaking through storm clouds"

**Required hierarchy:**
```yaml
prompt_sections:
  fantasy_scene:
    template: "{subject_positioned} {time_lighting}, {atmospheric_detail}"
    
  subject_positioned:
    template: "{age} {creature} {spatial_verb} {location_state}"
    # "ancient dragon perched atop crumbling tower"
    
  time_lighting:
    template: "{time}, {light_quality} {light_color} light {light_interaction} {weather}"
    # "at sunset, dramatic golden light breaking through storm clouds"
    
  atmospheric_detail:
    # Optional additional atmosphere
```

**Required datatypes:**
```yaml
ages: [ancient, young, aged, new, timeless]
creatures: [dragon, phoenix, griffin, ...]
spatial_verbs: [perched atop, standing before, floating above, ...]
locations: [tower, temple, mountain, ...]
location_states: [crumbling, pristine, ancient, ruined, ...]
times: [sunset, dawn, midnight, noon, ...]
light_qualities: [dramatic, soft, harsh, gentle, ...]
light_colors: [golden, silver, crimson, azure, ...]
light_interactions: [breaking through, streaming through, filtering through, ...]
weather: [storm clouds, clear sky, mist, fog, ...]
```

**Tag coordination needed:**
- time → light_color (sunset implies golden/orange)
- weather → light_quality (storm implies dramatic)
- location → location_state (tower can be crumbling)

### 2. Character Portraits - Example Breakdown

**Prompt:** "Close-up portrait of elderly wizard with piercing blue eyes, weathered skin, flowing white beard"

**Required hierarchy:**
```yaml
prompt_sections:
  character_portrait:
    template: "{framing} portrait of {age} {archetype} with {features}"
    
  features:
    template: "{feature_list?min=2,max=4,sep=comma_and}"
    
  feature_list:
    # Each feature: "{quality} {color} {body_part}"
    # piercing blue eyes
    # weathered skin
    # flowing white beard
```

**Required datatypes:**
```yaml
framing: [close-up, wide, medium, ...]
ages: [elderly, young, middle-aged, ...]
archetypes: [wizard, warrior, rogue, ...]
feature_qualities: [piercing, weathered, flowing, sharp, ...]
colors: [blue, white, gray, emerald, ...]
body_parts: [eyes, skin, beard, hair, ...]
```

**Tag coordination needed:**
- archetype → typical_features (wizard → beard, staff, robes)
- age → skin_quality (elderly → weathered)
- body_part → compatible_qualities (eyes → piercing, skin → weathered)

---

## Step 6: Emergence Test

**Question:** Can we get variety from limited definitions?

**Example test:**
```
Given:
- 10 creatures
- 10 age descriptors
- 15 spatial verbs
- 10 locations
- 10 location states
- 10 times
- 10 light qualities
- 10 weather conditions

Combinations: 10 × 10 × 15 × 10 × 10 × 10 × 10 × 10 = 1.5 billion prompts

But with tag filtering:
- Only 20-30% compatible = ~300-450 million realistic prompts
```

**Still massive variety!**

**But quality test:**
- Do they SOUND good?
- Do they make sense?
- Are they interesting?
- Are they repeatable?

**This requires generating 100 and manually reviewing!**

---

## Revised M8.5 Plan

### Phase 1: Prompt Design (Days 1-2)

**Goal:** Design 50-100 desired prompts across 5-10 categories

**Categories:**
1. Fantasy scenes (10-15 prompts)
2. Character portraits (10-15 prompts)
3. Landscapes (10-15 prompts)
4. Architecture (10-15 prompts)
5. Abstract/conceptual (10-15 prompts)
6. Action scenes (10-15 prompts)
7. Atmospheric moods (10-15 prompts)

**Deliverable:** `DESIRED_PROMPTS.md` with all target outputs

### Phase 2: Requirements Extraction (Day 3)

**Goal:** Reverse-engineer what datatypes/tags are needed

**Process:**
1. Break down each prompt into components
2. Identify required datatypes
3. Identify coordination patterns
4. Identify hierarchical structures

**Deliverable:** `REQUIREMENTS_FROM_PROMPTS.md`

### Phase 3: Minimal Viable Design (Days 4-5)

**Goal:** Design ONLY what's needed for desired prompts

**Not:** "Let's add 100 datatypes"  
**But:** "These 30 datatypes enable our 50 prompts"

**Deliverable:** `MINIMAL_DESIGN.md` - only necessary components

### Phase 4: Implementation (Days 6-10)

**Goal:** Build the minimal viable package

**Process:**
1. Implement datatypes (with article tags!)
2. Implement promptsections
3. Implement rules
4. Test frequently

### Phase 5: Generation & Evaluation (Days 11-14)

**Goal:** Generate 100 prompts, evaluate quality

**Metrics:**
- How many are good? (target: >70%)
- How many are interesting? (target: >50%)
- How much repetition? (target: <30% similar)
- Do they match desired style? (target: >80%)

### Phase 6: Iteration (Days 15-17)

**Goal:** Fix what's wrong

**Based on evaluation:**
- Missing datatypes? Add them.
- Too repetitive? Add variety.
- Poor coordination? Add tags/rules.
- Not emergent? Restructure hierarchy.

---

## What This Approach Fixes

**Before (tool-first):**
- ❌ Design 100 datatypes
- ❌ Hope they produce good prompts
- ❌ Tool dictates outcome

**After (outcome-first):**
- ✅ Design desired prompts
- ✅ Reverse-engineer requirements
- ✅ Build minimal necessary tools
- ✅ Author/developer dictates outcome

---

## Immediate Next Steps

**RIGHT NOW:**
1. Stop designing datatypes
2. Start writing desired prompts (50-100 examples)
3. Group by category
4. Identify what makes each one good

**TONIGHT/TOMORROW:**
1. Finish desired prompts collection
2. Begin reverse-engineering requirements
3. Identify minimal datatypes needed

**THIS WEEKEND:**
1. Design minimal viable package (based on actual needs)
2. Start implementation
3. Test early and often

---

**Status:** Approach corrected - starting with outcomes, not tools! ✅

**This is the right way to validate emergent generation.**

