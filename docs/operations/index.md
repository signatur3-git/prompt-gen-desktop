# Operations Guide

This guide captures repository workflows and contributor expectations that sit outside the specification itself.

## Local preview and builds

Use the npm scripts to install dependencies and launch a VitePress preview:

```bash
npm install
npm run dev
```

The preview runs with hot module replacement for Markdown edits. To validate the production build, run:

```bash
npm run build
npm run preview
```

The static site output is emitted to `docs/.vitepress/dist/`. Serving the production build locally mirrors the assets that GitHub Pages would publish.

## Formatting

The repository relies on Prettier for consistent formatting of Markdown and configuration files. Before sending changes, normalize the files with:

```bash
npm run lint
```

The command prints the files it reformats. Commit the resulting changes to keep diffs minimal and predictable.

## Contributing changes

1. Create a topic branch from the latest `main` branch.
2. Update or add documentation within `docs/`.
3. Run the formatting command above to keep Markdown tidy.
4. Open a pull request summarizing the change and reference relevant RFC sections for reviewers.

## Keeping branches up to date

Rebase frequently so topic branches incorporate the latest changes without merge commits:

```bash
git fetch origin
git rebase origin/main
```

Resolve conflicts using your editor, stage the fixes with `git add`, and run `git rebase --continue`. Once the branch is rebased, rerun the formatting command to catch any whitespace adjustments introduced during conflict resolution.

## Deterministic documentation preview

When evaluating rendering behaviour or sanity checks that depend on seeded randomness, document the seed value in the Markdown examples so reviewers can reproduce the same output locally. This applies to both CLI-driven workflows and GUI mockups rendered via the VitePress site.
