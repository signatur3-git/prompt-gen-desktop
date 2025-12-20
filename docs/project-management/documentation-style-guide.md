title: RPG Documentation Style Guide (M0 Approved)
---

# RPG Documentation Style Guide

This guide establishes the formatting, citation, and change-control conventions for the RPG v1.0.0.rc1 documentation program. It meets Milestone M0 acceptance criteria and will be refined as additional deliverables are authored.

> **Governance approval:** Ratified alongside the M0 plan on 2024-04-12. Future revisions should note review checkpoints in document metadata once owner assignments are finalized.

## 1. Structure & Formatting

- **Front Matter:** Every Markdown document must include YAML front matter with a `title` field. Additional metadata (status, owner, last-reviewed) will be added in later milestones.
- **Headings:** Use sentence case for all headings (`## Authoring tool overview`). Limit heading depth to four levels to preserve navigability in the documentation site.
- **Lists & Tables:** Prefer tables for traceability matrices and checklists. Use ordered lists for procedures and unordered lists for conceptual highlights.
- **Callouts:** Use blockquotes (`>`) to highlight governance decisions or mandatory warnings. Reserve bold text for compliance keywords such as **MUST**, **SHOULD**, and **MAY**.
- **Code & Commands:** Wrap CLI commands in fenced code blocks with language hints (e.g., <code>```bash</code>) and reference expected outputs inline or in adjacent tables.

## 2. Terminology & Voice

- Align terminology with the approved glossary (Milestone M1 deliverable). Until the glossary is finalized, reference RFC 0001 and `source-of-truth` documents for authoritative phrasing.
- Write in the active voice with clear actor identification (e.g., "The Authoring Tool validates package namespaces").
- Introduce abbreviations once per document and add them to the glossary if they persist across sections.

## 3. Citations & Cross-References

- **Internal Links:** Use relative links to cross-reference other documentation files. Include descriptive link text instead of bare URLs.
- **External References:** Provide the full title of external standards or tools and link to their canonical source. Note access requirements if the source is restricted.
- **Traceability:** When documenting requirements derived from RFC 0001 or `source-of-truth` materials, cite the specific clause or section in parentheses (e.g., "(RFC 0001 §3.2)").

## 4. Change Control & Review

- All documentation changes must be tracked via pull requests referencing the relevant milestone and RFP section.
- Each document must list a **Document Custodian** in its metadata once owner assignments are finalized (tracked in the project plan).
- Require at least one reviewer outside the originating workstream for changes affecting compliance or implementation guidance.
- Record accepted changes in the change log (to be created in Milestone M5) and update review status within two business days of merge.

## 5. Accessibility & Localization

- Ensure images include descriptive alt text and diagrams have accompanying textual explanations.
- Provide tables in Markdown format rather than screenshots to support screen readers.
- Use inclusive language and avoid idioms that may not translate cleanly for non-native English readers.

## 6. Future Enhancements

The following items are earmarked for later milestones:

- Expand this guide with template snippets for compliance checklists (M3 dependency).
- Add localization guidelines once translation requirements are confirmed by governance.
- Introduce automated linting rules via documentation tooling (pending evaluation in M2).

root@50b9aa70c633:/workspace/prompt-gen-spec#
