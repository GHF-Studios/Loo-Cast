# Contributing

## How to Contribute

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/my-feature`
3. Make your changes
4. Add tests
5. Run `cargo xtask setup_sdk` once per clone
6. Run `cargo xtask audit`
7. Commit: `git commit -m "feat: add my feature"`
8. Push: `git push origin feature/my-feature`
9. Create a pull request

## Code Style

- Follow Rust standard style. The pre-commit hook runs `cargo fmt`.
- No clippy warnings. `cargo xtask audit` runs clippy with `-D warnings`.
- Add tests for new features
- Document public APIs

## Local Hooks

- `pre-commit` formats the alpha workspace.
- `pre-push` runs `cargo xtask audit`.
- If hooks are missing, run `cargo xtask setup_sdk`.

## Questions?

Open an issue or start a discussion!
