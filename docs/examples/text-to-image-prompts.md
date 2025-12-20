# Text-to-Image Prompt Examples: Design Validation Scenarios

**Created:** 2025-12-16  
**Purpose:** Real-world scenarios to validate context interaction design  
**Status:** Draft - Ready for Analysis

---

## Overview

This document contains progressively complex text-to-image prompts that serve as **design validation scenarios**. For each prompt, we'll reverse-engineer what a package author would need to define, helping us determine:

1. What should be **engine primitives** vs. **author-defined logic**
2. Whether **Rules** are sufficient or if we need the full **Decisions framework**
3. Where the system should feel like **templates** vs. requiring **programmatic control**

---

## Simple Prompts (Level 1)

### S1: Basic Object
```
A red ball
```

**Challenges:**
- Article selection (A vs An)
- Single adjective + noun combination

**What author needs to define:**
- Datatype: colors (with article tags: a/an)
- Datatype: objects (with article tags)
- PromptSection: main template
- Logic: Select article based on first word

**Coordination needed:**
- Article determined by which word is first (adjective if present, else noun)

---

### S2: Optional Adjective
```
An elegant swan
A swan
```

**Challenges:**
- Adjective is optional (50% chance)
- Article must match whichever word comes first
- "Elegant" requires "An", "Swan" requires "A"

**What author needs to define:**
- Datatype: adjectives (with article + phonetic tags)
- Datatype: animals (with article tags)
- PromptSection: template with min=0,max=1 adjective
- Logic: Defer article selection until we know which word is first

**Coordination needed:**
- Can't decide article at start of template
- Need to know what word actually renders first
- Classic "deferred decision" problem

---

### S3: Plural Form
```
Three cats sleeping
A cat sleeping
```

**Challenges:**
- Random count (1 to 5)
- Noun must pluralize if count > 1
- Verb must agree ("cat sleeps" vs "cats sleep")

**What author needs to define:**
- Datatype: animals (with plural forms: cat/cats)
- Datatype: verbs (with singular/plural forms: sleeps/sleep)
- Logic: Select forms based on count
- Count variable stored in context

**Coordination needed:**
- Count determines noun form
- Count determines verb form
- Article vs number word ("a cat" vs "three cats")

---

## Medium Complexity (Level 2)

### M1: Atmospheric Scene
```
An elegant swan swimming in a misty lake at dawn
```

**Challenges:**
- Multiple optional adjectives
- Prepositional phrases
- Atmospheric modifiers
- Consistent tone/mood

**What author needs to define:**
- Datatype: subjects (swan, deer, eagle) with tags
- Datatype: subject-adjectives (elegant, graceful, majestic)
- Datatype: activities (swimming, gliding, soaring)
- Datatype: atmospheric-adjectives (misty, foggy, hazy)
- Datatype: locations (lake, forest, mountain)
- Datatype: times (dawn, dusk, midnight)
- Logic: Ensure atmospheric adjectives match time tags
- Logic: Ensure activities match subject capabilities
- PromptSection: Compose full scene

**Coordination needed:**
- Tone consistency (elegant + graceful, not elegant + chaotic)
- Physical plausibility (swan swims, not flies)
- Atmospheric coherence (misty + dawn works, scorching + midnight doesn't)

---

### M2: Character with Possessions
```
A wizard with his ornate staff standing beside an ancient tree
```

**Challenges:**
- Gender agreement (his/her/their)
- Article for both character and possession
- Possession adjective optional
- Spatial relationship

**What author needs to define:**
- Datatype: characters (wizard, warrior, archer) with gender tags
- Datatype: possessions (staff, sword, bow) with compatible-owner tags
- Datatype: possession-adjectives (ornate, weathered, gleaming)
- Datatype: environment-objects (tree, rock, fountain) with age/quality tags
- Datatype: spatial-relations (beside, near, under)
- Logic: Select possessive pronoun based on character gender
- Logic: Match possessions to character type
- Logic: Handle articles for multiple noun phrases

**Coordination needed:**
- Gender propagates from character to pronoun
- Possessions must be appropriate for character type
- Multiple article decisions (A wizard... an ancient tree)
- Possessive agreement (his/her/their)

---

### M3: Multiple Objects with Shared Property
```
A red car and a red bicycle parked on a sunny street
```

**Challenges:**
- Two objects share same adjective (both red)
- Conjunction (and)
- Shared location/context
- Articles for each object

**What author needs to define:**
- Datatype: colors (red, blue, green)
- Datatype: vehicles (car, bicycle, motorcycle)
- Datatype: weather-adjectives (sunny, rainy, foggy)
- Datatype: locations (street, driveway, parking lot)
- PromptSection: Compose multi-object scene
- Logic: Apply same color to multiple objects
- Logic: Use separator (and) correctly

**Coordination needed:**
- Store selected color, reuse for both objects
- Handle "a red car and a red bicycle" not "a red car and bicycle"
- Each object needs its own article
- Color determined once, applied twice

---

## Complex Scenes (Level 3)

### C1: Medieval Battle Scene
```
Three medieval knights with ornate armor standing beside their horses in a torch-lit castle courtyard at midnight
```

**Challenges:**
- Plural count (three knights)
- Plural possessive (their horses)
- Multiple coordinated elements
- Temporal and spatial context
- Adjective distribution

**What author needs to define:**
- Datatype: character-types (knight, archer, soldier) with plural forms
- Datatype: armor-adjectives (ornate, battered, gleaming)
- Datatype: poses (standing, kneeling, marching)
- Datatype: possessions (horses, weapons, banners)
- Datatype: lighting (torch-lit, moonlit, shadowy)
- Datatype: locations (courtyard, hall, rampart)
- Datatype: times (midnight, dawn, dusk)
- Logic: Pluralize nouns based on count
- Logic: Select plural possessive (their) when count > 1
- Logic: Ensure lighting matches time (torch-lit works for midnight)
- Logic: Ensure possession count matches character count

**Coordination needed:**
- Count propagates: three knights → their horses (plural possessive)
- Lighting appropriate for time of day
- Possessions scaled to character count
- Adjectives distributed correctly (ornate armor, not ornate knights)
- Spatial coherence (in a courtyard, not floating)

---

### C2: Character Interaction Scene
```
A tall warrior and a small mage examining an ancient artifact in their dimly lit study
```

**Challenges:**
- Two characters with different traits
- Shared action (examining)
- Shared possession (their artifact)
- Shared location (their study)
- Plural context despite different individuals
- Multiple adjectives on different entities

**What author needs to define:**
- Datatype: character-types (warrior, mage, rogue)
- Datatype: physical-traits (tall, short, muscular)
- Datatype: magical-traits (powerful, apprentice, wise)
- Datatype: shared-actions (examining, discussing, creating)
- Datatype: mystical-objects (artifact, tome, crystal)
- Datatype: object-adjectives (ancient, mysterious, glowing)
- Datatype: room-types (study, laboratory, chamber)
- Datatype: lighting (dimly lit, brightly lit, dark)
- Logic: Match trait adjectives to character types (tall for warrior, not mage)
- Logic: Use plural possessive for shared items (their)
- Logic: Verb agreement (two subjects = plural verb)

**Coordination needed:**
- Different adjectives for different characters
- Shared possessive despite different genders possible
- Plural verb form for compound subject
- Articles for each character (A warrior... a mage)
- Single article for shared possession (an artifact)

---

### C3: Environmental Storytelling
```
A weathered wooden sign creaking in the wind beside a moss-covered fountain in an abandoned village square under storm clouds
```

**Challenges:**
- Long chain of descriptive elements
- Atmospheric consistency
- Spatial relationships (beside, in, under)
- State implications (weathered → abandoned theme)
- No characters, pure environment

**What author needs to define:**
- Datatype: objects (sign, fountain, statue, bench)
- Datatype: object-materials (wooden, stone, metal)
- Datatype: wear-states (weathered, pristine, ruined)
- Datatype: object-actions (creaking, dripping, swaying)
- Datatype: environmental-effects (wind, rain, sunlight)
- Datatype: growth (moss-covered, ivy-draped, bare)
- Datatype: location-types (village square, town plaza, market)
- Datatype: location-states (abandoned, bustling, quiet)
- Datatype: sky-conditions (storm clouds, clear sky, sunset)
- Datatype: spatial-prepositions (beside, near, under, in front of)
- Logic: Theme consistency (weathered + abandoned + storm = coherent)
- Logic: Physical plausibility (sign can creak, fountain can't)
- Logic: Spatial chain (beside → in → under)

**Coordination needed:**
- Thematic coherence across all elements
- Action-object compatibility
- Environmental effect pairing (wind makes things creak)
- Spatial preposition chain
- State agreement (everything suggests abandonment)

---

### C4: Dynamic Action Scene
```
Five armored soldiers charging across a burning battlefield while dodging arrows under a smoke-filled sky
```

**Challenges:**
- Plural subjects with count
- Multiple simultaneous actions (charging, dodging)
- Active environmental hazards (burning, arrows)
- Environmental state (smoke-filled)
- Gerund forms (burning, smoke-filled)
- All elements suggest chaos/action

**What author needs to define:**
- Datatype: character-types (soldier, warrior, defender)
- Datatype: armor-types (armored, unarmored, lightly-armored)
- Datatype: group-actions (charging, retreating, advancing)
- Datatype: evasive-actions (dodging, blocking, parrying)
- Datatype: projectiles (arrows, spears, bolts)
- Datatype: battlefield-states (burning, frozen, muddy)
- Datatype: environmental-descriptors (smoke-filled, ash-covered, blood-stained)
- Logic: Plural verb forms (soldiers charge, not charges)
- Logic: Ensure actions compatible (can charge while dodging)
- Logic: Theme consistency (all elements suggest battle)
- Logic: Gerund/participle forms (-ing)

**Coordination needed:**
- Count → plural forms throughout
- Multiple concurrent actions that make sense together
- Environmental state matches action intensity
- Hazards appropriate to scene type
- Energetic/chaotic tone across all elements

---

## Extreme Complexity (Level 4)

### E1: Cinematic Scene with Foreground/Background
```
In the foreground, a hooded figure with a glowing amulet stands on ancient steps, while in the background, a massive cathedral looms beneath a blood-red moon
```

**Challenges:**
- Spatial layers (foreground/background)
- Scale contrast (figure vs cathedral)
- Multiple articles for multiple entities
- Lighting sources (glowing amulet, blood-red moon)
- Atmospheric unity despite spatial separation
- Compositional framing

**What author needs to define:**
- Datatype: character-descriptors (hooded, cloaked, masked)
- Datatype: characters (figure, person, stranger)
- Datatype: magical-objects (amulet, orb, gem)
- Datatype: object-effects (glowing, pulsing, sparking)
- Datatype: foreground-locations (steps, bridge, platform)
- Datatype: location-adjectives (ancient, crumbling, marble)
- Datatype: background-structures (cathedral, tower, fortress)
- Datatype: background-descriptors (massive, towering, imposing)
- Datatype: celestial-objects (moon, sun, stars)
- Datatype: celestial-colors (blood-red, silver, golden)
- Datatype: spatial-framing (in foreground, in background, distant)
- Logic: Coordinate lighting (amulet glow + moon light)
- Logic: Scale relationships (figure small, cathedral massive)
- Logic: Compositional balance

**Coordination needed:**
- Foreground/background marked explicitly
- Scale descriptors appropriate to layer
- Lighting sources don't conflict
- Atmospheric mood unified (mysterious/ominous)
- Multiple independent clauses with proper conjunction

---

### E2: Temporal/Causal Narrative
```
After the storm, a rainbow appears over a valley where a farmer surveys his flooded fields, his face showing relief
```

**Challenges:**
- Temporal sequence (after the storm)
- Causal relationship (storm → flooding → relief)
- Nested spatial context (over valley, in valley)
- Character emotion matched to situation
- Possessive references (his fields, his face)
- Gerund clause (showing relief)

**What author needs to define:**
- Datatype: weather-events (storm, blizzard, drought)
- Datatype: post-event-phenomena (rainbow, fog, clearing)
- Datatype: landscapes (valley, hillside, plain)
- Datatype: character-types (farmer, traveler, merchant)
- Datatype: actions (surveys, examines, walks through)
- Datatype: possessions (fields, crops, livestock)
- Datatype: possession-states (flooded, dry, destroyed)
- Datatype: emotions (relief, despair, hope)
- Logic: Causal chains (storm causes flooding)
- Logic: Emotion appropriate to situation
- Logic: Temporal markers (after, during, before)
- Logic: Gender agreement for possessives

**Coordination needed:**
- Event sequence makes logical sense
- Post-event phenomena match event type
- Character emotion matches situation outcome
- Possessives track to character
- Nested spatial prepositions (over valley where...)

---

## Analysis Questions

For each scenario, consider:

1. **Can this be solved with simple Rules in enrichment phase?**
   - If yes, how complex are the rules?
   - Do they feel like templates or like programming?

2. **Does this require the full Decisions framework?**
   - Which processor type would you use? (Expression, Rule Set, Script, External)
   - Is the complexity justified?

3. **What engine primitives are truly needed?**
   - Random selection? (already planned)
   - Context get/set? (already planned)
   - List operations? (first, filter, map)
   - String operations? (concat, replace)
   - Conditional logic? (if/then/else)
   - What else?

4. **Where does it stop feeling like templates?**
   - At what point would you want to write actual code?
   - Is that a sign we need the feature, or that the approach is wrong?

5. **Could this be simplified with better datatype design?**
   - Should tags carry more semantic information?
   - Should relationships be first-class? (character → weapon compatibility)

---

## Next Steps

1. **Review these scenarios** - Add more if needed
2. **Reverse-engineer packages** - For each scenario, design the full package
3. **Test both approaches** - Try solving with Rules-only vs Decisions framework
4. **Document patterns** - What patterns emerge? What's reusable?
5. **Make the call** - Rules, Decisions, or hybrid?

See `authoring-analysis.md` for reverse-engineered package designs.

---

**Ready to analyze? Start with Simple scenarios and work up to Complex! 📝**

