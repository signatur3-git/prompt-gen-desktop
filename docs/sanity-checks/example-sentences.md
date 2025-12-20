# Sanity Check Example Sentences

The following sample outputs ensure the prompt generator can reliably render natural language while honoring scoped context,
contract requirements, and deterministic replay characteristics. Each example is grouped by the underlying sanity check
scenario and includes expected ontology or data hooks.

## Reference Template and Example Sentences

The sanity checks below are all driven by the same `creature_intro`
prompt-section template. The template combines helper functions, typed
slots, and ontology-backed data lookups to surface deterministic sentences
for auditing.

```handlebars
{{#prompt_section "creature_intro"}}
  {{#with (coalesce (context "language.subject.current")
                    (context "language.subject.current.de")) as |subject|}}
    {{set "language.modifier.stack" (modifiers subject)}}
    {{article subject}} {{subject.surface}}
    {{#if (context "flavor.mode" == "daily_quest")}}
      {{context "quests.daily"}}
    {{/if}}
  {{/with}}
{{/prompt_section}}
```

- `coalesce` selects the first populated subject slot so the template works
  for both English and German ontologies.
- `context` reads pull hydrated data objects from the scoped context
  ledger.
- `set` writes new values (e.g., modifier stacks) so downstream sections
  can reuse the same derived data.
- `article` is the helper under audit; it inspects the active noun record
  and modifier list to select the correct determiner.

The renderer executes this section inside sanity check rulebooks that
provide English and German ontologies. The resulting sentences are listed
below.

### Example Sentences by Scenario

### SC-001 — Article Agreement with Optional Modifiers
- "an elusive owl glides silently"
- "a bold otter casually surveys the river"

  These sentences deliberately pair consonant- and vowel-led nouns with
  optional adjectives so the article helper must inspect both the noun
  metadata and the active modifier stack before selecting `a` or `an`.

### SC-003 — Uniqueness Guarantees in Selections
- "Gather three distinct relics: the ember key, the silver dial, and the moonstone shard."
- "Prepare a feast with unique ingredients: sunroot, frostberry, and whisperleaf."

### SC-004 — Multilingual Ontology Hand-off
- "Bienvenue, brave ranger, your English dossier follows."
- "Vous affrontez the crimson wyrm under a moonlit sky."

### SC-005 — Temporal Seeds and System Randomness
- "Daily quest (2024-02-19, seed 8472): escort the archivist through the auric dunes."
- "Replay seed 112358 (2024-02-19): recover the lost hymn from the echoing vaults."

## Namespaced Configuration Requirements

```yaml
packages:
  prompt_sections:
    creature_intro:
      namespace: prompt_sections.creature_intro
      requires:
        - contracts.language.articles
      renderer: renderers.template.v1
    daily_quest:
      namespace: prompt_sections.daily_quest
      requires:
        - contracts.random.seeded
        - contracts.language.number
      renderer: renderers.template.v1

ontologies:
  en:
    articles:
      namespace: ontologies.en.articles
      exports:
        - contracts.language.articles
    nouns:
      namespace: ontologies.en.nouns
      exports:
        - contracts.language.subjects
  de:
    nouns:
      namespace: ontologies.de.nouns
      exports:
        - contracts.language.subjects.de
      datatypes:
        - datatype: language.noun.german
          fields:
            - name: surface
              type: string
            - name: article_token
              type: enum
              values: [der, die, das]
            - name: case
              type: enum
              values: [nominative, accusative, dative, genitive]
            - name: number
              type: enum
              values: [singular, plural]
            - name: modifiers
              type: list
              items: language.modifier
    articles:
      namespace: ontologies.de.articles
      exports:
        - contracts.language.articles.de
      consumes:
        - contracts.language.subjects.de

rulebooks:
  adventure_daily:
    namespace: rulebooks.adventure.daily
    entrypoint: prompt_sections.daily_quest
    depends_on:
      - ontologies.en.articles
      - ontologies.de.articles
      - prompt_sections.creature_intro
      - prompt_sections.daily_quest
    context_contracts:
      - contracts.random.seeded
      - contracts.language.articles
      - contracts.language.articles.de
```

## Article Agreement Context Wiring

The article helper is driven entirely by the contracts enumerated in the
`requires` stanza above. The runtime sets up the following context slots
before template evaluation:

| Context Slot | Source Namespace | Purpose |
| --- | --- | --- |
| `language.article.rules` | `ontologies.en.articles` / `ontologies.de.articles` | Lookup table keyed by phonetic descriptors (English) or grammatical gender (German). |
| `language.modifier.stack` | `prompt_sections.creature_intro` | Ordered list of active adjectives whose leading phoneme may override default article assumptions. |
| `language.subject.current` | `ontologies.en.nouns` | Emits the noun payload containing both the surface form and phonetic hints. |
| `language.subject.current.de` | `ontologies.de.nouns` | Emits the German noun payload, including the pre-tagged article token and case metadata. |

When `article(subject)` is called inside the template, the helper performs
these deterministic steps:

1. **Subject resolution** — Reads `language.subject.current` (English) or
   `language.subject.current.de` (German) to obtain the noun plus its
   phonetic descriptor or pre-tagged article token.
2. **Modifier folding** — Folds in any front-loaded modifiers from
   `language.modifier.stack`, re-evaluating the phonetic descriptor if the
   modifier changes the audible onset (e.g., "hourly" still resolves to a
   vowel sound). For German nouns the modifier stack is preserved but the
   article token is not recomputed because it is delivered with the noun
   payload.
3. **Rule evaluation** — Queries `language.article.rules` with the resolved
   descriptor (English) or the noun's `article_token` and case (German).
   German ontologies act as interfaces: they encode gender (`der`, `die`,
   `das`) and grammatical case, so the helper merely maps that structured
   data to the correct surface determiner.

Because the contracts populate these slots prior to rendering, the helper
does not perform any probabilistic work at runtime; the outcome is fully
determined by the noun payload and modifier context captured when the
section is hydrated.

### Context Interactions by Component

| Component | Read Slots | Write Slots | Notes |
| --- | --- | --- | --- |
| `prompt_sections.creature_intro` | `language.subject.current`, `language.subject.current.de`, `quests.daily` | `language.modifier.stack`, `language.article.trace` | Template orchestrates helper calls, writes modifier stack, and records trace handles for audits. |
| `ontologies.*.nouns` | — | `language.subject.current*` | Ontologies behave like interfaces: they define the available fields and guarantee that nouns expose the metadata (`surface`, `phonetics`, `article_token`). |
| `ontologies.*.articles` | `language.subject.current*`, `language.modifier.stack` | `language.article.rules`, `language.article.ledger` | Articles ontologies translate ontology-specific noun metadata into a normalized rule set; the ledger is appended for deterministic replay. |
| `datatypes.language.noun.german` | — | — | Datatype schema validates that German nouns carry an `article_token`, ensuring the helper never guesses the article. |
| `rulebooks.adventure.daily` | All contract slots declared in the rulebook | Seeds the context namespace with deterministic values before rendering. |

The context store therefore serves as the shared medium: prompt sections can
write derived data, ontologies populate initial subject records, and helpers
read the normalized view without knowing which ontology supplied the data.

### German Article Selection Walkthrough

1. The rulebook selects `ontologies.de.nouns` as the source for the current
   subject, yielding a record such as `{ surface: "Eule", article_token:
   "die", case: "nominative" }`.
2. The noun record is stored in `language.subject.current.de` and the German
   article ontology injects deterministic mappings into
   `language.article.rules` (e.g., `die+nominative → die`).
3. The template writes any modifiers (e.g., `list("nachtaktive")`) into
   `language.modifier.stack` but does not alter the `article_token`.
4. `article(subject)` receives the noun record, detects the German datatype,
   and asks `language.article.rules` for the surface determiner. The helper
   therefore replays the ontology-provided token (`die nachtaktive Eule`) with
   no heuristics.

The same flow applies to English nouns, except the rule evaluation step uses
phonetic descriptors rather than predefined article tokens. In both cases the
context ledger captures every read and write, enabling the sanity checks to
replay decisions deterministically.

## Detailed Rendering Flow

1. **Namespace Resolution** — The renderer loads all declared namespaces and
   validates that every required contract is satisfied. Article rulebooks
   and random seed providers must be present or the render aborts.
2. **Context Priming** — Seed values, ontology selections, and section-level
   modifiers are written into the context slots listed above. The priming
   step hashes the seed plus namespace IDs to guarantee reproducibility.
3. **Template Expansion** — Prompt sections consume the primed context,
   resolving helper calls such as `article(...)`, deterministic list
   combinators, and bilingual fragments. Each helper logs its inputs and
   outputs for later replay validation.
4. **Post-processing** — The renderer normalizes whitespace, persists the
   context ledger (including article decisions), and emits the final
   sentences for sanity check comparison. During audits, the ledger can be
   replayed to confirm that the stored noun and modifier data would yield
   the same article choice.

These detailed steps show exactly how the prompt generator binds article
logic to context and ensure that the example sentences render
deterministically.
