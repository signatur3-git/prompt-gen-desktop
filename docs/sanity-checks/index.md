# Sanity Check Scenarios

These scenarios exercise edge cases and integration pathways for the Random Prompt Generator (RPG) specification. They help implementation teams validate deterministic behaviour, context coordination, and package interoperability before hardening production code.

Each scenario outlines:

- **Context** – Why the scenario matters for end users.
- **Artifacts** – Suggested package assets (YAML source of truth, deterministic JSON mirrors) and supporting scripts.
- **Expected Behaviours** – Deterministic outcomes and compliance hooks the platform must satisfy.
- **Open Questions** – Follow-up design work or specification gaps revealed by the scenario.

## SC-001: Article Agreement with Optional Modifiers

**Context**

Authors avoid hard-coded heuristics for article selection while supporting optional modifiers (adjectives, adverbs). The rendering engine should respect ContextInterfaces that derive articles from datatype tags and scoped context.

**Artifacts**

- Namespace `featured.common` exporting:
  - Datatypes `noun`, `adjective`, `adverb` with tags (`article`, `starts_with_vowel`).
  - ContextInterface `language.article_interface` implementing request/contribution flow.
  - Separatorset `comma_and` for enumerations.
- PromptSection `creature.intro`:
  ```yaml
  template: |
    {context.request(featured.common:article_requested.<prompt>)}
    {featured.common:adjective?min=0,max=2&sep=featured.common:comma_and}
    {featured.common:noun}
    {if context.has(featured.common:adverb_choice.<prompt>) ? {featured.common:adverb} : }
  ```
- Validator fixture ensuring YAML and JSON remain in sync and that `min <= max`.

**Expected Behaviours**

- Renderer selects `a` vs `an` solely via ContextInterface contributions informed by datatype tags.
- Optional adjectives/adverbs update context before article resolution, yielding natural phrasing.
- Validator flags missing interface declarations or tags.

**Open Questions**

- Should interfaces expose phonetic fallbacks (e.g., manual overrides for acronym handling)?
- How are locale-specific article sets registered for discovery in the marketplace?

## SC-002: Marketplace Contract Wiring

**Context**

Marketplace consumers need to inspect which packages satisfy required interfaces and morphers before installation.

**Artifacts**

- Package manifest declaring:
  ```yaml
  id: story.fantasy
  version: 0.1.0
  requires:
    - featured.common >=1.0.0
    - contracts.language.articles
  provides:
    - contracts.language.tone
  ```
- Rulebook `adventure.start` referencing promptsections that depend on `contracts.language.articles`.
- Marketplace mock (CLI or API) that prints dependency graphs with capability icons.

**Expected Behaviours**

- Package validator confirms required interfaces have providers before publish.
- Marketplace UI/API lists which dependency delivers each capability and surfaces validator signatures.
- CLI workflows (`rpg publish`, `rpg install`) respect semantic version ranges.

**Open Questions**

- Should capability identifiers follow URI form (`rpg://language/articles`)?
- How are optional capabilities communicated to installers?

## SC-003: Uniqueness Guarantees in Repetitions

**Context**

Promptsections often require unique draws from finite datasets (e.g., "list three distinct animals"). Deterministic uniqueness must hold even across nested sections.

**Artifacts**

- Datatype `featured.common:animal` with exactly three entries and `unique_pool: true` metadata.
- PromptSection `featured.common:animal_trio`:
  ```yaml
  template: |
    {featured.common:animal?min=3,max=3&sep=featured.common:comma_and&unique=true}
  ```
- Validator rule checking that requested `unique=true` counts do not exceed dataset cardinality.

**Expected Behaviours**

- Renderer tracks scope-specific selections and prevents duplicates.
- Validator surfaces deterministic errors if the dataset cannot satisfy uniqueness requirements.
- Marketplace badges signal packages that rely on uniqueness so engines can expose configuration toggles.

**Open Questions**

- Do uniqueness scopes reset per promptsection invocation or per rulebook entry? (Current guidance: per scope.)
- How should validators expose suggested dataset sizes when uniqueness fails?

## SC-004: Multilingual Ontology Hand-off

**Context**

Packages may mix namespaces across languages. Prompt authors combine English nouns with French morphology to localise content.

**Artifacts**

- Namespaces `en.core` and `fr.morphology`.
- ContextInterface `fr.morphology:gender_interface` with contributions for grammatical gender.
- PromptSection that switches scopes mid-template to request gender from the French interface while using English datatypes.

**Expected Behaviours**

- Renderer handles scope transitions deterministically and applies morphers based on returned gender.
- Validator ensures multilingual packages declare compatible interfaces and morphers.
- Marketplace highlights localisation coverage (languages, morphers, interfaces) for discovery.

**Open Questions**

- Should packages specify fallback locales for missing translations?
- How are morphology packs versioned relative to base datasets?

## SC-005: Temporal Seeds and System Randomness

**Context**

Only minimal random sources (seeded numbers, UUIDs, timestamps) are provided by the system. Authors must document seed derivation to guarantee reproducibility.

**Artifacts**

- Rulebook `daily.quest` deriving seeds from date + user ID.
- CLI example invoking the renderer with explicit `--seed`, `--uuid-sequence`, and `--timestamp` overrides.

**Expected Behaviours**

- Renderer exposes CLI/SDK parameters to set seeds and system randomness.
- Validator ensures rulebooks document seed derivation and that packages avoid unseeded randomness.
- Marketplace metadata records whether packages rely on external time inputs.

**Open Questions**

- How should production systems log seed provenance for audit trails?
- What privacy considerations apply when logging deterministic UUID sequences?

## Additional Candidates

- **SC-006: Contextual Pronoun Resolution** – Validate pronoun interfaces driven by gender and plurality tags.
- **SC-007: Conditional Morphers for Pluralisation** – Confirm morphers adjust determiners when counts vary.
- **SC-008: Nested PromptSections with Shared Context** – Ensure deterministic context merging during recursion.

These scenarios provide a backlog for regression suites and interoperability tests as v1.0.0 stabilises.
