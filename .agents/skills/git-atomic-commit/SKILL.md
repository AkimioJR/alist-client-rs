---
name: git-atomic-commit
description: "Plan and create atomic Git commits for larger coding tasks. Use when Codex implements a multi-step change, prepares commits, splits a large diff, reviews commit boundaries, or must ensure each commit is coherent, independently buildable, and does not depend on code introduced only in a later commit."
---

# Git Atomic Commit

## Goal

Split larger work into small, reviewable Git commits where each commit has one intent and can build on its own. Never bundle unrelated changes just because they were edited in one working session.

Use this skill to decide commit boundaries and ordering. When a boundary needs a final commit message, invoke or consult `$git-commit-message` instead of duplicating message rules here.

## Atomic Commit Rules

- Put one logical intent in each commit.
- Keep dependency order correct: if commit A calls a function, type, module, feature flag, or test helper, commit A must also introduce it or depend on an earlier commit that does.
- Keep each commit buildable whenever practical. For this Rust repository, prefer running `cargo check` after each planned commit boundary and `cargo test` for behavior changes.
- Avoid "fix the previous commit" commits while still preparing local history; fold follow-up fixes into the commit that introduced the issue.
- Separate mechanical changes from behavior changes when possible.
- Separate generated or upstream API updates from handwritten code when possible.
- Separate tests from implementation only when the implementation commit is still meaningful and buildable without the new tests.

## Boundary Examples

Prefer boundaries like:

- sync upstream API definitions;
- add request/response models for one feature area;
- implement one client feature after its models exist;
- add tests for the smallest behavior slice they validate;
- perform one mechanical refactor before behavior changes that depend on it;
- fix one bug without bundling unrelated cleanup.

Avoid boundaries like:

- One commit that mixes API sync, model additions, client implementation, tests, formatting, and unrelated cleanup.
- A commit that updates call sites to use `build_archive_request` while adding `build_archive_request` only in the next commit.
- A commit that fails to compile because a new trait method, enum variant, module export, or dependency is missing until later.

## Planning Workflow

When work is large enough to require multiple steps:

1. Inspect the target change and identify independent intents.
2. Draft a commit sequence before editing too much. Each item should describe one buildable state.
3. Order commits by dependencies:
   - data models and shared helpers before call sites;
   - dependency or generated API updates before code using them;
   - refactors before behavior changes that rely on the refactor;
   - tests after or with the implementation they validate.
4. Implement and verify one boundary at a time when practical.
5. Before committing, inspect `git diff` or `git diff --cached` and move unrelated hunks out of the staged set.
6. After each commit or staged boundary, run the lightest useful verification:
   - `cargo fmt --check` for formatting-sensitive changes;
   - `cargo check` for compile safety;
   - `cargo test` when behavior or tests changed.

## Splitting an Existing Large Diff

If a large working-tree diff already exists:

1. Use `git diff --stat` to see changed areas.
2. Use `git diff` to classify hunks by intent.
3. Stage selectively with `git add -p` or file-level staging only when every hunk in the file belongs to the same commit.
4. Verify the staged state with `git diff --cached`.
5. Check dependency completeness before committing:
   - all referenced functions/types/modules are present in the staged or previous history;
   - all exports needed by staged call sites are staged;
   - `Cargo.toml` and `Cargo.lock` are staged together when dependency resolution changes;
   - tests do not rely on behavior omitted from the same or earlier commits.
6. Commit the staged atomic change, then repeat for the remaining diff.

## Rust Repository Guidance

Use repository structure to choose boundaries:

- `alist-api/`: upstream API or generated/reference material. Prefer a separate commit boundary for this area.
- `src/models/**`: request/response/data types. Usually comes before client methods using those types.
- `src/client/**`: API methods and request logic. Keep each feature area (`auth`, `public`, `fs`, `upload`, `archive`) coherent.
- `src/error.rs`: error model or conversion changes. Place before code that returns the new errors.
- `src/tests.rs`: tests. Pair with the smallest implementation slice they validate.
- `.github/**`: CI changes. Keep separate from product code unless required for the same build fix.

## Readiness Checklist

Before creating or recommending commits, verify:

- Each commit has one primary reason to exist.
- Each commit can be reviewed without reading a later commit.
- Earlier commits do not call code introduced later.
- Public API additions include required model exports in the same or earlier commit.
- Dependency and lockfile changes are not split incorrectly.
- Verification has been run per boundary, or skipped with a clear reason.
- Commit message wording is delegated to `$git-commit-message`.
