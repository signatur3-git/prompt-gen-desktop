# Rules Processor Fix: Article Selection Bug

## Issue
When using the `peaceful_scene` prompt section with "orange" color, the rendered output was "a orange landscape" instead of "an orange landscape".

## Root Cause
The rules processor was not properly implementing the documented specification behavior for context contributions.

According to the spec (from `source-of-truth/context-interactions.md`):
> "If multiple match, **the first successful one sets the value; others skip if already set**."

And from `docs/architecture/context-interactions.md`:
> "Additional contributions skip once the key is populated, preserving deterministic behaviour."

The implementation was evaluating the expression and setting the context value without first checking if the value already existed. This meant that multiple rules could overwrite each other's contributions, violating the "first contribution wins" principle.

## Fix
Modified `execute_step()` in `src-tauri/src/rules/processor.rs` to:
1. Build the context key first
2. Check if the key already exists using `context.has()`
3. Skip the rule step if value already exists
4. Only evaluate the expression and set the value if it doesn't exist

This ensures the "first contribution wins" behavior documented in the spec.

## Test Case
The `tag-filter-test.yaml` package has two rules:
1. `compute_article_from_color` - tries to set article from color tags
2. `compute_article_from_animal` - tries to set article from animal tags

For the `peaceful_scene` template which uses `{color}`:
- The first rule successfully sets `article` to "an" for "orange"
- The second rule skips because `article` is already set
- Result: "an orange landscape" ✓

## Compliance
This fix brings the implementation into compliance with the documented specification. No conceptual changes were made - only bug fixes to match the already-documented behavior.

## Related Files
- `src-tauri/src/rules/processor.rs` - Fixed execute_step()
- `test-packages/tag-filter-test.yaml` - Test package that exposed the bug
- `source-of-truth/context-interactions.md` - Spec documenting the expected behavior
- `docs/architecture/context-interactions.md` - Additional spec documentation

