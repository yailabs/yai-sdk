# SDK ownership

This public MIT repository owns supported client contracts and transport, not
YAI semantics, authority, persistence, scheduler or server implementation. Private
Core and Studio repositories consume compatible contracts, never shared HEADs.

Preserve shared work; stage only owned files and inspect live diffs before commit.
`Cargo.toml` owns independent product SemVer. Wire and projection schema identities
change only for their own contract changes. `package.json` and lockfiles project
the product version and are checked mechanically.

Use `cargo test --locked`, `npm ci && npm run check`, and `python3 tools/check.py`.
Conformance fixtures prove client contracts, not private Core semantics. Core and
Studio must additionally qualify real integration. Missing evidence is not PASS.
No credentials, generated packages, build products or private source in Git.
