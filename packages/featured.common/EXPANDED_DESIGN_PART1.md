# Prompt-Gen Common Package - REVISED EXPANDED DESIGN

**Revision Date:** 2025-12-17 (Evening)  
**Reason:** Original scope too small to validate emergent generation at scale

---

## Why Expand?

**Original Design Problems:**
- 30 datatypes, ~350 values → Too small!
- Won't discover file size issues (20-30 KB is trivial)
- Won't test tag coordination at real scale
- Won't reveal missing features
- Won't prove/disprove emergent generation
- 5 namespaces → Won't test organization at scale

**Revised Goals:**
- **100+ datatypes** → Real complexity
- **1,500-2,000 values** → Combinatorial explosion
- **60+ promptsections** → Deep hierarchies
- **17 namespaces** → Test organization
- **100-150 KB YAML** → Test file size limits
- **10-12 level nesting** → Test depth limits

**This will ACTUALLY validate the spec or break it!**

---

## Revised Namespace Structure (17 Namespaces)

### FOUNDATION LAYER (2 namespaces)

#### 1. common.base
**Purpose:** Core linguistic primitives

**Datatypes (8):**
- articles (3 values) - a, an, the
- determiners (15 values) - this, that, these, those, some, any, all, both, either, neither, each, every, few, many, several
- conjunctions (12 values) - and, or, but, nor, for, yet, so, with, featuring, plus, versus, without
- prepositions_spatial (20 values) - in, on, at, near, under, above, beside, behind, through, across, along, around, between, among, within, outside, inside, beyond, toward, away from
- prepositions_temporal (10 values) - during, before, after, until, since, while, throughout, by, at, within
- intensifiers (15 values) - very, extremely, slightly, somewhat, incredibly, barely, fairly, highly, utterly, completely, partially, moderately, exceptionally, remarkably, quite
- quantifiers (18 values) - one, two, three, few, several, many, countless, pair, trio, dozen, handful, multitude, abundance, scattering, collection, group, cluster, swarm
- comparatives (12 values) - more, less, better, worse, larger, smaller, brighter, darker, taller, shorter, faster, slower

**Total values: ~113**

#### 2. common.grammar
**Purpose:** Advanced grammatical constructs

**Datatypes (6):**
- possessives (8 values) - my, your, his, her, its, our, their, whose
- pronouns (15 values) - I, you, he, she, it, we, they, who, what, which, this, that, these, those, one
- verb_forms (20 values) - standing, sitting, running, walking, flying, swimming, resting, working, playing, creating, destroying, building, dancing, singing, fighting, sleeping, eating, drinking, thinking, dreaming
- adjective_order (7 values) - opinion, size, age, shape, color, origin, material (for ordering)
- sentence_connectors (15 values) - however, therefore, meanwhile, furthermore, moreover, nevertheless, consequently, subsequently, accordingly, likewise, otherwise, hence, thus, whereas, although
- emphasis_markers (10 values) - indeed, certainly, absolutely, definitely, truly, really, actually, particularly, especially, notably

**Total values: ~75**

**Foundation Layer Total: ~188 values**

---

### VISUAL LAYER (4 namespaces)

#### 3. common.visual.color
**Purpose:** Comprehensive color theory

**Datatypes (10):**
- colors_primary (12 values) - red, blue, yellow, green, orange, purple, black, white, gray, brown, pink, beige
- colors_advanced (25 values) - crimson, scarlet, vermillion, azure, cobalt, navy, emerald, jade, chartreuse, amber, ochre, sienna, magenta, lavender, indigo, teal, turquoise, coral, salmon, ivory, ebony, charcoal, pearl, silver, gold
- color_modifiers (20 values) - bright, dark, light, deep, pale, vivid, muted, dull, rich, faded, saturated, desaturated, warm, cool, vibrant, subdued, neon, pastel, metallic, iridescent
- color_patterns (15 values) - solid, gradient, striped, spotted, mottled, checkered, marbled, rainbow, ombre, tie-dye, camouflage, zebra-striped, leopard-print, polka-dotted, geometric
- color_harmonies (8 values) - monochromatic, analogous, complementary, triadic, split-complementary, tetradic, warm palette, cool palette
- transparency (8 values) - opaque, translucent, transparent, semi-transparent, frosted, crystal-clear, hazy, misty
- luminosity (12 values) - glowing, radiant, luminous, brilliant, shimmering, sparkling, gleaming, dim, shadowy, dark, bright, dazzling
- color_temperature (6 values) - very warm, warm, neutral, cool, very cool, icy
- color_intensity (6 values) - intense, moderate, subtle, bold, soft, dramatic
- finish (8 values) - matte, glossy, satin, metallic, pearlescent, opalescent, reflective, mirror-like

**Total values: ~120**

#### 4. common.visual.light
**Purpose:** Lighting and atmospheric effects

**Datatypes (12):**
- light_sources (20 values) - sun, moon, stars, candle, torch, lantern, fire, lightning, aurora, bioluminescence, neon, LED, spotlight, floodlight, streetlight, chandelier, fairy lights, glow, phosphorescence, laser
- light_quality (18 values) - soft, harsh, diffuse, direct, scattered, focused, ambient, dramatic, natural, artificial, warm, cool, neutral, golden, silver, colored, white, filtered
- light_direction (12 values) - front-lit, backlit, side-lit, top-lit, bottom-lit, rim-lit, silhouetted, spot-lit, evenly-lit, cross-lit, under-lit, over-lit
- light_intensity (10 values) - bright, dim, moderate, brilliant, blinding, subtle, faint, strong, weak, overwhelming
- time_of_day (16 values) - pre-dawn, dawn, sunrise, early morning, mid-morning, late morning, noon, early afternoon, mid-afternoon, late afternoon, golden hour, sunset, dusk, twilight, night, midnight
- weather_lighting (15 values) - sunny, overcast, cloudy, partly cloudy, stormy, foggy, misty, hazy, clear, bright, gloomy, dramatic storm light, soft rain light, snow light, dusty
- shadows (12 values) - hard shadows, soft shadows, long shadows, short shadows, no shadows, deep shadows, subtle shadows, cast shadows, self-shadows, dramatic shadows, dappled shadows, multiple shadows
- atmospheric_effects (18 values) - volumetric rays, god rays, lens flare, bloom, haze, fog, mist, dust particles, smoke, steam, light beams, caustics, refraction, diffraction, scattering, aurora effect, starlight, moonbeams
- light_moods (15 values) - romantic, mysterious, ominous, cheerful, somber, dramatic, peaceful, energetic, moody, ethereal, magical, sinister, warm, cold, neutral
- contrast (8 values) - high contrast, low contrast, medium contrast, dramatic contrast, subtle contrast, balanced, stark, gentle
- color_cast (12 values) - warm cast, cool cast, neutral cast, golden cast, blue cast, green cast, purple cast, orange cast, red cast, sepia tone, monochrome, natural
- exposure (6 values) - overexposed, underexposed, properly exposed, high-key, low-key, balanced

**Total values: ~162**

#### 5. common.visual.material
**Purpose:** Materials, textures, and surfaces

**Datatypes (15):**
- materials_natural (20 values) - wood, stone, rock, sand, earth, clay, mud, ice, snow, water, glass (natural), crystal, amber, pearl, coral, bone, horn, shell, leather, fur
- materials_metal (15 values) - iron, steel, bronze, copper, brass, silver, gold, platinum, aluminum, titanium, chrome, pewter, tin, lead, zinc
- materials_synthetic (18 values) - plastic, rubber, vinyl, nylon, polyester, acrylic, fiberglass, carbon fiber, silicone, foam, resin, epoxy, latex, PVC, Teflon, kevlar, lycra, velcro
- materials_fabric (20 values) - cotton, silk, wool, linen, velvet, satin, denim, canvas, burlap, felt, tweed, corduroy, fleece, chiffon, organza, taffeta, brocade, damask, muslin, cheesecloth
- materials_organic (15 values) - flesh, skin, muscle, plant matter, leaves, bark, grass, moss, lichen, fungus, scales, feathers, hair, petals, vines
- textures_tactile (25 values) - smooth, rough, soft, hard, bumpy, ridged, grooved, pitted, porous, dense, fluffy, fuzzy, silky, velvety, scratchy, slippery, sticky, slimy, dry, wet, grainy, powdery, crystalline, fibrous, woven
- textures_visual (20 values) - glossy, matte, shiny, dull, reflective, translucent, opaque, patterned, plain, textured, smooth, rough-looking, weathered, pristine, worn, aged, new, polished, tarnished, rustic
- surface_qualities (18 values) - cracked, chipped, peeling, flaking, rusted, corroded, polished, burnished, etched, engraved, embossed, carved, molded, cast, forged, hammered, brushed, sanded
- patterns (20 values) - striped, spotted, swirled, marbled, veined, grained, knurled, dimpled, honeycomb, lattice, mesh, grid, tessellated, fractal, organic, geometric, random, ordered, symmetrical, asymmetrical
- finish_quality (12 values) - mirror-finish, high-gloss, semi-gloss, satin, eggshell, matte, flat, textured, hammered, brushed, distressed, antiqued
- transparency_levels (8 values) - completely opaque, mostly opaque, semi-opaque, translucent, semi-transparent, mostly transparent, crystal clear, perfectly clear
- reflectivity (10 values) - highly reflective, moderately reflective, slightly reflective, non-reflective, mirror-like, diffusely reflective, specularly reflective, retroreflective, partially reflective, selectively reflective
- temperature_feel (8 values) - ice-cold, cold, cool, neutral, warm, hot, burning, freezing
- weight_appearance (8 values) - heavy-looking, substantial, solid, light-looking, delicate, airy, massive, insubstantial
- age_appearance (12 values) - brand new, new, recent, modern, aged, old, ancient, timeworn, weathered, pristine, well-worn, decayed

**Total values: ~244**

#### 6. common.visual.form
**Purpose:** Shapes, geometry, and spatial forms

**Datatypes (8):**
- shapes_2d (20 values) - circle, square, rectangle, triangle, oval, ellipse, pentagon, hexagon, octagon, star, heart, crescent, diamond, trapezoid, parallelogram, rhombus, spiral, wave, zigzag, blob
- shapes_3d (25 values) - sphere, cube, cylinder, cone, pyramid, prism, torus, dodecahedron, icosahedron, tetrahedron, octahedron, ellipsoid, paraboloid, hyperboloid, helix, spiral, coil, disc, ring, arch, dome, spire, column, pillar, obelisk
- geometric_style (12 values) - geometric, organic, angular, curved, faceted, smooth, crystalline, amorphous, structured, freeform, symmetrical, asymmetrical
- proportions (15 values) - tiny, small, medium, large, huge, gigantic, colossal, miniature, life-sized, oversized, undersized, elongated, compressed, stretched, squat
- complexity (10 values) - simple, complex, intricate, elaborate, minimalist, ornate, detailed, plain, busy, sparse
- symmetry (8 values) - perfectly symmetrical, mostly symmetrical, slightly asymmetrical, asymmetrical, radially symmetric, bilaterally symmetric, fractally symmetric, chaotic
- dimension_emphasis (6 values) - flat, two-dimensional, three-dimensional, volumetric, dimensional, spatial
- scale_modifiers (10 values) - microscopic, minuscule, compact, full-sized, expansive, monumental, towering, vast, infinite, boundless

**Total values: ~106**

**Visual Layer Total: ~632 values**

---

### SUBJECT LAYER (3 namespaces)

#### 7. common.subject.living
**Purpose:** Living entities - people, creatures, plants

**Datatypes (20):**
- people_archetypes (30 values) - warrior, wizard, rogue, priest, merchant, farmer, artisan, scholar, noble, servant, child, elder, youth, hero, villain, sage, fool, king, queen, peasant, knight, archer, blacksmith, healer, bard, thief, assassin, guard, traveler, hermit
- people_professions (40 values) - [extensive list of professions]
- people_characteristics (25 values) - tall, short, muscular, slender, young, old, beautiful, plain, scarred, tattooed, bearded, bald, long-haired, elegant, rough, refined, wild, civilized, mysterious, ordinary, exotic, local, foreign, noble-looking, common-looking
- creatures_mythical (35 values) - dragon, phoenix, unicorn, griffin, chimera, hydra, basilisk, cockatrice, manticore, sphinx, pegasus, centaur, minotaur, medusa, kraken, leviathan, behemoth, gargoyle, golem, demon, angel, fairy, sprite, pixie, gnome, dwarf, elf, orc, troll, goblin, imp, wraith, specter, banshee, revenant
- creatures_real (40 values) - [extensive list of animals]
- creatures_fantasy (25 values) - elemental, shapeshifter, werewolf, vampire, lich, necromancer, summoned being, construct, animated statue, living shadow, dream being, nightmare creature, void entity, chaos spawn, crystalline being, plant creature, fungal entity, slime, ooze, gelatinous form, insect swarm, hive mind, symbiote, parasite, amalgamation
- plants_common (20 values) - tree, flower, grass, bush, vine, fern, moss, mushroom, cactus, bamboo, palm, oak, pine, willow, rose, lily, orchid, sunflower, daisy, tulip
- plants_exotic (20 values) - carnivorous plant, giant flower, tree of life, world tree, magical herb, glowing fungus, crystal plant, metal tree, flesh plant, singing reed, weeping willow, man-eating plant, healing herb, poison ivy, enchanted rose, moonflower, sunstar, dreamleaf, soulroot, heartwood
- being_states (15 values) - living, dead, undead, spectral, ethereal, corporeal, physical, spiritual, ascended, fallen, corrupted, purified, transformed, evolved, devolved
- being_sizes (12 values) - tiny, small, medium, large, huge, gargantuan, colossal, titanic, microscopic, human-sized, giant, miniature
- being_numbers (10 values) - solitary, pair, trio, small group, group, crowd, horde, swarm, legion, multitude
- age_stages (12 values) - infant, child, adolescent, young adult, adult, middle-aged, elderly, ancient, ageless, newborn, mature, venerable
- creature_features (30 values) - winged, horned, tailed, scaled, furred, feathered, clawed, fanged, tentacled, multi-headed, multi-limbed, eyeless, many-eyed, armored, spiked, venomous, bioluminescent, transparent, crystalline, ethereal, shadowy, flaming, icy, rocky, wooden, metallic, fleshy, skeletal, spectral, amorphous
- intelligence_levels (8 values) - bestial, animal-like, primitive, sentient, intelligent, wise, genius, god-like
- temperaments (20 values) - calm, aggressive, friendly, hostile, curious, indifferent, playful, serious, wild, tame, noble, savage, gentle, fierce, timid, bold, protective, predatory, docile, feral
- creature_abilities (25 values) - can fly, can swim, can burrow, can climb, can run, can teleport, can shapeshift, can breathe fire, can freeze, can electrify, can poison, can heal, can curse, can bless, can speak, can think, can cast magic, can resist magic, can become invisible, can phase, can regenerate, can split, can merge, can evolve, can adapt
- diet_types (8 values) - carnivore, herbivore, omnivore, scavenger, parasite, filter-feeder, photosynthetic, energy-based
- habitat_types (15 values) - terrestrial, aquatic, aerial, subterranean, arboreal, desert-dwelling, arctic, tropical, temperate, volcanic, ethereal plane, shadow realm, dream world, void, between dimensions
- social_structures (10 values) - solitary, pair-bonded, family group, pack, herd, flock, colony, hive, civilization, tribe
- reproduction_types (6 values) - egg-laying, live birth, budding, splitting, spores, magical creation

**Total values: ~426**

#### 8. common.subject.object
**Purpose:** Inanimate objects and structures

**Datatypes (15):**
- objects_tools (30 values) - sword, axe, hammer, staff, wand, bow, shield, armor, helm, gauntlet, boots, cloak, ring, amulet, crown, scepter, orb, chalice, key, lock, chain, rope, torch, lantern, compass, map, book, scroll, potion, crystal
- objects_furniture (25 values) - chair, table, bed, throne, bench, stool, cabinet, chest, wardrobe, mirror, desk, shelf, pedestal, altar, statue, fountain, column, arch, door, window, gate, wall, floor, ceiling, roof
- objects_containers (20 values) - box, crate, barrel, jar, vase, urn, pot, pan, bowl, cup, plate, basket, bag, sack, pouch, bottle, flask, canteen, chest, trunk
- objects_decorative (20 values) - painting, sculpture, tapestry, rug, curtain, banner, flag, emblem, crest, seal, insignia, decoration, ornament, trinket, bauble, gem, jewel, treasure, relic, artifact
- structures_buildings (30 values) - castle, fortress, tower, keep, palace, temple, cathedral, church, monastery, shrine, mansion, house, cottage, hut, shed, barn, mill, tavern, inn, shop, market, arena, coliseum, amphitheater, stadium, bridge, aqueduct, wall, gate, archway
- structures_ruins (15 values) - ancient ruins, crumbling wall, fallen tower, abandoned castle, destroyed temple, buried city, sunken structure, overgrown ruin, skeletal remains, weathered monument, broken statue, collapsed bridge, shattered gate, eroded fortress, forgotten shrine
- objects_magical (25 values) - magic staff, enchanted sword, cursed ring, holy relic, demonic artifact, ancient tome, spell scroll, magic crystal, power stone, soul gem, life orb, death's scythe, time piece, dimensional key, reality anchor, void shard, chaos seed, order seal, creation spark, destruction core, binding chain, freedom wing, illusion veil, truth lens, fate thread
- objects_technology (20 values) - machine, device, mechanism, clockwork, gear, lever, pulley, wheel, engine, reactor, generator, transmitter, receiver, sensor, actuator, processor, interface, terminal, console, control panel
- objects_natural (20 values) - stone, rock, boulder, pebble, crystal, mineral, ore, metal vein, fossil, shell, bone, tooth, claw, horn, feather, scale, egg, nest, hive, web
- object_states (15 values) - intact, broken, damaged, pristine, worn, ancient, new, restored, ruined, functional, non-functional, activated, dormant, powered, unpowered
- object_materials (12 values) - wooden, stone, metal, crystal, glass, ceramic, bone, leather, fabric, paper, wax, ice
- object_sizes (10 values) - tiny, small, medium, large, huge, massive, miniature, life-sized, oversized, monumental
- object_conditions (15 values) - clean, dirty, polished, tarnished, rusty, pristine, weathered, aged, new, old, maintained, neglected, restored, decayed, rotting
- object_qualities (15 values) - ordinary, common, rare, unique, magical, mundane, sacred, profane, blessed, cursed, enchanted, mundane, legendary, mythical, divine
- object_functions (12 values) - weapon, tool, container, decoration, furniture, structure, machine, artifact, treasure, waste, resource, mystery

**Total values: ~279**

#### 9. common.subject.environment
**Purpose:** Settings, locations, and environments

**Datatypes (15):**
- biomes (20 values) - forest, jungle, desert, tundra, grassland, savanna, swamp, marsh, bog, fen, mountain, valley, canyon, cave, underground, ocean, sea, lake, river, stream
- landscapes (25 values) - plain, plateau, mesa, butte, hill, mountain range, volcanic field, crater, caldera, geyser field, ice field, glacier, ice shelf, dune field, oasis, badlands, wasteland, meadow, field, grove, clearing, thicket, woodland, moorland, heath
- water_features (20 values) - ocean, sea, lake, pond, pool, puddle, river, stream, creek, brook, waterfall, cascade, rapids, whirlpool, maelstrom, spring, geyser, hot spring, lagoon, bay
- sky_features (15 values) - clear sky, cloudy sky, stormy sky, starry sky, aurora, rainbow, meteor shower, eclipse, blood moon, harvest moon, nebula, cosmic vista, dimensional rift, void tear, heaven's gate
- terrain_features (20 values) - peak, summit, ridge, cliff, precipice, escarpment, gorge, ravine, chasm, pit, crater, cave mouth, grotto, tunnel, cavern, chamber, passage, shaft, mine, quarry
- vegetation_density (8 values) - barren, sparse, scattered, moderate, dense, thick, overgrown, impenetrable
- climate_zones (12 values) - tropical, subtropical, temperate, subarctic, arctic, alpine, mediterranean, continental, oceanic, arid, semi-arid, humid
- seasons_detail (8 values) - early spring, late spring, early summer, late summer, early autumn, late autumn, early winter, late winter
- weather_conditions (25 values) - sunny, cloudy, overcast, rainy, stormy, snowy, sleety, hailing, foggy, misty, hazy, clear, windy, calm, humid, dry, hot, cold, freezing, scorching, temperate, mild, severe, extreme, pleasant
- time_periods (12 values) - dawn, morning, midday, afternoon, evening, dusk, night, midnight, witching hour, eternal day, eternal night, timeless
- magical_environments (20 values) - enchanted forest, cursed land, blessed ground, sacred grove, profane site, magical nexus, ley line crossing, dimensional portal, elemental plane, shadow realm, ethereal plane, astral plane, dream dimension, nightmare realm, void space, chaos realm, order domain, creation field, destruction zone, liminal space
- architectural_styles (18 values) - classical, gothic, baroque, renaissance, medieval, ancient, modern, futuristic, organic, geometric, crystalline, ethereal, shadowy, metallic, wooden, stone, magical, technological
- scale_environments (10 values) - microscopic, miniature, small-scale, human-scale, large-scale, vast, enormous, continental, planetary, cosmic
- environment_moods (15 values) - peaceful, hostile, neutral, inviting, foreboding, mysterious, cheerful, gloomy, serene, chaotic, orderly, wild, tamed, natural, artificial
- special_features (20 values) - ancient monument, mysterious structure, magical artifact site, battle scar, natural wonder, geological oddity, elemental anomaly, dimensional instability, temporal distortion, spatial anomaly, gravity well, magnetic field, radiation zone, dead zone, fertile crescent, oasis, landmark, waypoint, boundary, threshold

**Total values: ~273**

**Subject Layer Total: ~978 values**

---

**(Continued in next file - this is getting very large!)**

**Current Total So Far:**
- Foundation: ~188 values
- Visual: ~632 values  
- Subject: ~978 values
- **Subtotal: ~1,798 values**

**Still to design:**
- Action Layer (3 namespaces)
- Artistic Layer (3 namespaces)
- Technical Layer (3 namespaces)

**Projected Final Total: 2,500-3,000 values across 100+ datatypes in 17 namespaces**

---

**THIS is the scale needed to truly validate emergent generation!**

