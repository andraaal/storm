# Issue tracker: GitHub

Issues and specs for this repository live as GitHub Issues. Use the `gh` CLI for issue operations; infer the repository from the GitHub remote.

## Conventions

- Create an issue with `gh issue create --title "..." --body "..."`.
- Read an issue with `gh issue view <number> --comments`.
- List issues with `gh issue list` and filter by state or label as needed.
- Comment with `gh issue comment <number> --body "..."`.
- Apply or remove labels with `gh issue edit <number> --add-label "..."` or `--remove-label "..."`.
- Close an issue with `gh issue close <number> --comment "..."`.

## Pull requests as a triage surface

PRs are not a request surface for triage.

## When a skill says to publish to the issue tracker

Create a GitHub issue in this repository.
