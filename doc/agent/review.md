# Review and Contribution Guide

This file captures review focus, commit and PR expectations, and general
change-discipline rules.

## Review Focus

- Focus on memory issues.
- When reviewing a removed function call, check that required behavior was not
  dropped accidentally during a refactor.
- When reviewing a removed function call, check whether the callee became unused
  and should also be removed.
- When reviewing commits that refactor, move, or rewrite code, point out dropped
  comments or docstrings that still apply.

## Change Discipline

- Keep diffs small.
- Do not do unprompted refactorings or core reorganizations.
- Avoid mixing formatting and logic changes.
- Preserve comments and docstrings when porting, refactoring, or rewriting code
  if they still apply.
- Never commit changes unless the user explicitly asks for a commit.

## Commits

When the user explicitly asks for a commit:

- Write a subject of 50 characters or fewer.
- Add a blank line after the subject.
- Include an explanatory body as described in `CONTRIBUTING.md`.
- Keep commits atomic.
- Ensure all commits individually pass linters.

## Pull Requests

PR descriptions should:

- Outline the change.
- List verification commands or screenshots.
- Flag hardware requirements.
- Mark drafts with `[WIP]` until they are ready.
- Wait to squash until reviews conclude.
