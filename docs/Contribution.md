# Contributing

Welcome — thanks for helping improve the project! This is a short guide for contributors.

Getting started

- Read the `README.md` for project purpose and quick start.
- Check `documents/` for design notes before proposing large changes.

Issues & PRs

- Open an issue for bugs, feature requests, or design discussions. Provide reproduction steps and logs where possible.
- For code changes, create a branch named `feat/<short-desc>` or `fix/<short-desc>` and submit a PR targeting `main` (or the current default branch).

Style & formatting

- Follow Rust idioms and use `rustfmt` for formatting: `cargo fmt`.
- Run lints: `cargo clippy` and address warnings when reasonable.
- Keep commits small and focused, with clear messages.

Testing & CI

- Add or update unit/integration tests when changing behavior.
- Validate the build locally (`./build.ps1` or `./build.sh`) and run any applicable test tasks.

Documentation

- Update docs in `docs/` and `README.md` when interfaces or workflows change.
- Add TODOs and link to `documents/` for deep-dive notes.

Contribution etiquette

- Be respectful and descriptive in discussions.
- If your change is large, open an RFC-style issue first for design feedback.

TODO:
- Add a checklist for PR review and release procedures.
- Document branching and versioning policy.

Thanks — every contribution helps. See `docs/Building.md` and `docs/Architecture.md` for further context.