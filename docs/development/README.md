<!-- docs:metadata
title: SDK Development and QA
id: yai-sdk.development
document: guide
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK Development and QA

**Standalone public checks first; real producers provide separate evidence.**

[Documentation](../README.md)

Clone this repository alone. Rust, Python 3 and Node/npm are development tools;
private Core source and a Studio checkout are not build prerequisites.

```sh
make check
cargo test --locked --all-targets
cargo doc --no-deps
```

Change public workflow facts in `contract/workflows.json`, then run
`python3 tools/workflows.py` and review both generated projections. The checked
operation inventory binds the admitted contract IDs; adding a method does not
make an unsupported Core capable of executing it.

Conformance tests use explicit synthetic servers/fixtures. They test compatibility,
framing, correlation, refusals, disclosure-safe projection shape and lost-response
behavior, not Core semantics. Real Core and Studio integration must additionally
exercise admitted operations through the same public transport.

For intentional four-repository work, build the current YAI CLI, run Studio's
`npm run test:kernel` to compile its consumer disposition module, then compose
the owner-published inventories explicitly:

```sh
make check-platform-parity \
  YAI_BIN=/absolute/path/to/yai/target/debug/yai \
  STUDIO_ROOT=/absolute/path/to/studio \
  YVEX_ROOT=/absolute/path/to/yvex
```

The checker compares YAI's live Application catalog with the SDK operation
projection, Studio's YAI operation dispositions, YVEX's canonical remote
operation registry with the separate YVEX SDK manifest, and Studio's native
YVEX dispositions. Controlled owner deltas must fail. This is contract and
consumer-disposition parity, not installed compatibility or real-model evidence.
No sibling checkout is an implicit package dependency; release consumers pin
reviewed SDK revisions, while local source overrides are explicit and temporary.

Keep temporary profiles, logs, installed dependencies and build products out of
Git. Never use an operator Case for destructive test setup. Publication follows
focused review and qualification; the legal gate remains independent.
