# Domain docs

## Before exploring

- Read `CONTEXT.md` at the repository root when it exists.
- Read ADRs under `docs/adr/` that touch the area being explored when they exist.
- Proceed silently when these files do not exist; do not require them to be created upfront.

## Layout

This is a single-context repository:

- `CONTEXT.md` contains the domain glossary and context.
- `docs/adr/` contains repository-wide architectural decisions.

## Vocabulary and ADRs

Use terms as defined in `CONTEXT.md` in issue titles, proposals, tests, and refactors. If a needed term is missing, treat that as a signal for domain modeling rather than silently inventing competing terminology.

If a proposal conflicts with an existing ADR, call out the conflict explicitly instead of silently overriding it.
