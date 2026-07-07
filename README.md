# Loo-Cast

The first-party Vapor content workspace. It owns three sibling artifacts:

- Spacetime Engine;
- Loo-Cast Game;
- the Loo-Cast Packagepack that composes the engine and game.

The Packagepack references its constituents through Vapor identities. It is not
nested inside either constituent and does not turn Vapor composition into Cargo
dependencies.
