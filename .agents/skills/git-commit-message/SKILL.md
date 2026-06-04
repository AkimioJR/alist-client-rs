---
name: git-commit-message
description: "Generate or review English Git commit messages for this repository using the local convention with prefixes feat, perfect, fix, refactor, style, docs, test, update, and chore. Use when Codex needs to draft, improve, validate, or explain commit messages, including scoped forms like feat(models): or fix(api):."
---

# Git Commit Message

## Goal

Create concise, consistent commit messages for this Rust AList client repository. Prefer the local format while borrowing the common `type(scope): subject` convention.

## Format

Use one of these forms:

```text
type: description
type(scope): description
```

Rules:

- Use exactly one allowed `type`.
- Use `scope` only when it clarifies the changed area.
- Write `description` in English to match this repository's commit history.
- Keep the first line short and specific; aim for 50 characters or fewer when practical.
- Do not end the first line with punctuation.
- Prefer an action/result phrase, not a vague label.

Good examples:

```text
feat(models): add archive extraction request models
perfect(api): include context in JSON parse errors
fix(archive): encode archive listing paths correctly
refactor(client): split filesystem request building
docs: update AList API notes
test(upload): cover multipart upload failure handling
```

Avoid:

```text
update
fix bug
feat: change code
style: big change.
```

## Allowed Types

- `feat`: Add a new user-facing capability, API method, data model, or behavior.
- `perfect`: Improve an existing capability without changing its core purpose, such as better error handling, validation, ergonomics, performance, or edge-case support.
- `fix`: Correct a bug, incorrect behavior, failing request, wrong serialization, panic, or test failure.
- `refactor`: Restructure code without intended behavior changes.
- `style`: Change formatting, imports, lint-only cleanup, or naming that does not affect behavior.
- `docs`: Change documentation, examples, comments, README-like material, or API notes.
- `test`: Add, update, or fix tests without changing production behavior.
- `update`: Update generated API artifacts, dependencies, versions, lockfiles, or synchronized upstream definitions.
- `chore`: Change repository maintenance, CI, build configuration, scripts, or other auxiliary tooling.

If both `feat` and `perfect` seem plausible, choose `feat` only when the public surface or supported behavior is newly added. Use `perfect` for polishing something that already exists.

## Scope Guidance

Prefer scopes that match the repository structure:

- `client`: shared client behavior in `src/client.rs` or cross-cutting client modules.
- `auth`, `public`, `fs`, `upload`, `archive`: feature-specific client or model modules.
- `models`: shared model declarations or broad model changes.
- `error`: error types and conversions.
- `tests`: integration or unit tests.
- `ci`: GitHub Actions and automation.
- `deps`: dependency or lockfile changes.

Use no scope for tiny or broad changes where a scope would be artificial.

## Workflow

1. Inspect the staged diff first when available with `git diff --cached --stat` and `git diff --cached`.
2. If nothing is staged, inspect the working-tree diff with `git diff --stat` and `git diff`.
3. Identify the user-visible intent, not every edited file.
4. Choose the most specific allowed type and optional scope.
5. Draft one first-line commit message.
6. If the change has multiple independent intents, suggest splitting commits instead of forcing one broad message.

## Validation Checklist

Before returning a commit message, verify:

- The first token matches `feat`, `perfect`, `fix`, `refactor`, `style`, `docs`, `test`, `update`, or `chore`.
- The syntax is `type: description` or `type(scope): description`.
- The description says what changed, not merely where files changed.
- The message does not mention unrelated edits.
- The scope, if present, is lowercase and meaningful for this repository.
