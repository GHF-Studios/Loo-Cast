# Contributing

Current audience: internal/solo workflow for active alpha development.

## Flow

1. Start from a named issue.
2. Update local `develop` and create a topic branch from it.
3. Run `cargo xtask setup_sdk` once per clone (or when hooks are missing).
4. Make the scoped change.
5. Add/update tests when needed.
6. Run `cargo xtask audit`.
7. Push the topic branch.
8. Open a PR into `develop`.
9. Link related issue(s) in the PR sidebar (`Development`).
10. Close linked issue(s) before merge.

## Code Style

- Follow Rust standard style. The pre-commit hook runs `cargo fmt`.
- No clippy warnings in workspace code. `cargo xtask audit` runs clippy with `--no-deps` and `-D warnings`.
- Add tests when behavior changes.
- Update docs when behavior/workflow changes.

## Local Hooks

- `pre-commit` formats the alpha workspace.
- `pre-push` runs `cargo xtask audit`.
- If hooks are missing, run `cargo xtask setup_sdk`.
- To remove managed hooks installed by `setup_sdk`, run `cargo xtask clean_sdk`.

## Questions?

Open an issue or start a discussion!
