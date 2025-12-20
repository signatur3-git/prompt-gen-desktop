# Decision Framework: Complete Solution Summary

## What We've Built

A comprehensive specification for **author-defined natural language decisions** in the Random Prompt Generator, replacing hardcoded logic with a generic, composable Decision framework.

## The Core Innovation

**Problem**: Previous approach tried to be generic while hardcoding patterns like "requests" and "contributions" through ContextInterfaces. This created a contradiction.

**Solution**: The engine provides **only generic mechanisms**:
- Context storage (get, set, has, delete, list)
- Expression evaluation
- Decision invocation
- Deterministic rendering order

Authors define **ALL semantic logic** through Decision components with four processor types:

1. **Expression**: Simple formulas (`word[0] in ['a','e','i','o','u']`)
2. **Rule Set**: Ordered if-then-else logic
3. **Script**: Embedded code (JS/Python/Lua) in sandbox
4. **External**: LLM/API calls for complex natural language processing

## Documents Created

### 1. `source-of-truth/context-interactions.md` (Rewritten)
- Generic context operations (no semantic methods)
- Complete Decision component specification
- Four processor types with examples
- Patterns for context coordination
- Migration guide from old approach

### 2. `source-of-truth/decision-patterns-guide.md` (New)
**Purpose**: Show how to solve real challenges using Decisions

**Contents**:
- Complete rendering lifecycle explanation with timing
- 10 core patterns with code examples
- Challenge-solution matrix
- How to mix Decisions across prompts/packages
- Advanced patterns (caching, middleware, composition)
- Debugging strategies

**Key Insight**: Rendering is left-to-right, depth-first. Order matters. Decisions execute at invocation point and can read/write context for coordination.

### 3. `source-of-truth/article-coordination-problem.md` (New)
**Purpose**: Solve the deferred decision-making problem

**The Problem**: With `"A {adjective?min=0,max=1} {noun}"`, you need the article BEFORE the word renders, but must choose it AFTER you know which word renders (adjective might skip due to min=0).

**5 Solutions**:
1. **Render-first, articlize-after**: Build phrase in context, prepend article
2. **PromptSection wrapper**: `featured.common:articled_noun_phrase` abstraction
3. **First-word signaling**: Words claim article responsibility via context flags
4. **Two-pass rendering**: Engine feature (breaks generic principle)
5. **Macro expansion**: Authoring tool feature (good long-term)

**Recommendation**: Ship Solution 2 in `featured.common`, document Solution 1 for custom cases, plan Solution 5 for M4.

### 4. `source-of-truth/phonetic-article-selection.md` (New)
**Purpose**: Address the phonetics vs. spelling challenge

**The Problem**: "An hour" (silent h), "a university" (sounds like "yoo")—article selection is phonetic, not orthographic.

**4 Accuracy Levels**:
1. **Simple letter-based**: `word[0] in ['a','e','i','o','u']` (~70% accurate)
2. **Exception lists**: Hardcode "hour", "university" (~85% accurate)
3. **Phonetic dictionary**: CMU dict lookup (~95% accurate)
4. **LLM-based**: GPT-4 with caching (~99% accurate)

**Recommended Approach**: Hybrid—try datatype tags first, then phonetic dict, then LLM fallback. Let authors choose accuracy level with `mode` parameter.

**Key Innovation**: Shows how Decision framework scales from simple rules to LLM integration without engine changes.

### 5. `docs/project-management/m1-source-of-truth-mapping.md` (Updated)
- Documented the Decision framework major update
- Added alignment table entry for Decisions
- Listed new source-of-truth documents
- Added open tasks for follow-up work

## Key Design Principles Demonstrated

### 1. Progressive Complexity
Start with simple stateless Decisions, add context coordination as needed, integrate LLMs for edge cases.

### 2. Explicit Over Implicit
All coordination visible in templates through `decision.call()` and context operations. No magic.

### 3. Composition Over Configuration
Complex behaviors emerge from combining simple Decisions. Build libraries, not monoliths.

### 4. Multiple Accuracy Levels
Authors choose speed vs. accuracy: simple rules for prototypes, LLMs for production.

### 5. Mix and Match
Same Decision works across any prompt, any package. Just a function call.

## Real-World Example: Article Selection

### Simple Case (Known Word)
```
{decision.call(featured.common:select_article_best, word='elephant', mode='simple')} elephant
→ "an elephant"
```

### Coordination Problem (Optional Adjective)
```
{featured.common:articled_noun_phrase(
  adjective=vocab:adjectives,
  noun=vocab:nouns,
  article_mode='best'
)}
→ "an elegant swan" (with adjective)
→ "a swan" (without adjective)
```

### With Phonetic Accuracy
The `select_article_best` Decision:
1. Checks datatype tags first (author-controlled)
2. Falls back to phonetic dictionary (~95% accurate)
3. Uses LLM as final fallback (~99% accurate)
4. Caches results for determinism and performance

## Implementation Impact

### Simpler Engine
- Fewer hardcoded behaviors
- No special-case logic for articles, gender, etc.
- Just: context storage + expression eval + Decision invocation

### More Author Power
- Define any logic as Decisions
- Integrate external services (LLMs, databases, APIs)
- Create reusable Decision libraries
- Control accuracy/performance trade-offs

### Better Testability
- Each Decision is isolated and testable
- Deterministic with seeded randomness
- Mock external services for testing

### Clearer Coordination
- Context mutations visible in templates
- Decision calls explicit, not implicit
- Rendering lifecycle documented and predictable

## Migration Path

For existing designs using "ContextInterfaces":

1. **Convert interfaces to Decisions**: Each contribution rule becomes a Decision
2. **Make coordination explicit**: Replace `context.request()` with `context.set()`
3. **Move logic to processors**: Transfer conditions/actions to rule processors
4. **Update templates**: Change implicit contributions to explicit `decision.call()`
5. **Document conventions**: Explain context key naming schemes to users

## Next Steps (From M1 Mapping Document)

### Immediate (M2-M3)
- Update component overview ER diagram with Decision entity
- Add Decision editing/testing to Authoring Tool spec
- Create validation rules for Package Validator
- Update glossary with Decision terminology

### Near-Term (M4)
- Document sandboxing requirements for Script/External processors
- Create example packages for each processor type
- Design macro expansion system for Authoring Tool
- Build reference Decision library for `featured.common`

### Long-Term (M5-M6)
- RFP examples showcasing Decision flexibility
- Performance benchmarking for Decision types
- Security audit for sandboxing
- Marketplace integration (Decision discovery)

## Success Metrics

The Decision framework succeeds if:

1. ✅ **Engine stays generic**: No semantic knowledge hardcoded
2. ✅ **Authors have full control**: Can implement any logic type
3. ✅ **Solutions are composable**: Decisions mix and match freely
4. ✅ **Progression is smooth**: Simple → complex without rewrites
5. ✅ **Coordination is explicit**: All interactions visible in templates

## Questions Answered

### "How does article selection work with optional elements?"
See `article-coordination-problem.md`. Use render-first/articlize-after pattern or PromptSection wrappers.

### "Isn't article choice phonetics-based, not letter-based?"
Yes! See `phonetic-article-selection.md`. Provides 4 accuracy levels from simple rules to LLM-based.

### "Can authors mix Decisions across different prompts?"
Absolutely. See `decision-patterns-guide.md` Section "Mixing and Matching Across Prompts". Same Decision works anywhere.

### "How does rendering lifecycle affect Decision usage?"
Critical. See lifecycle section in `decision-patterns-guide.md`. Left-to-right, depth-first. Context accumulates. Order matters.

### "Can Decisions call other Decisions?"
Yes. Cascading pattern in `decision-patterns-guide.md` Pattern 4. Build complex logic from simple primitives.

### "How do I handle non-deterministic external calls?"
Cache results in context (.global scope) for determinism. See `phonetic-article-selection.md` LLM caching example.

## Conclusion

The Decision framework provides a **truly generic engine** with **maximum author flexibility**. From simple article selection to complex LLM-based natural language processing, everything is author-defined through composable Decision components.

The specification is now ready for implementation, with clear patterns, progressive complexity, and real-world solutions to hard problems like optional elements and phonetic analysis.

**The engine just invokes Decisions. Authors define the semantics.**
