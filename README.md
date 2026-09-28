# YAI SDK

Mutation receipts expose only Case identity/version and selected transition
identity facts. They do not expose persisted `CaseState` or `Transition` layouts.
Removing accidentally serialized private fields is a producer defect correction,
not a promise of payload-byte compatibility; the supported `WorkCommit` fields
remain stable. Canonical state and persistence schemas remain Core-private.

MIT Rust-first supported local client boundary. YAI Core and Studio are separate
proprietary products; this repository contains no Case engine, persistence,
scheduler, server, or semantic admission implementation.

`Cargo.toml` is the canonical independent SemVer authority (0.1.0). Future release
tags use `vMAJOR.MINOR.PATCH`; no tag or release is implied by a build. Wire
`yai.client.v1`, Application `yai.studio.application.v1`, capability catalog
`yai.application_capability_catalog.v1` are independent contract identities.
The legacy Application identity is retained without changing its semantics.

Before 1.0, breaking public APIs require a minor increment; patches preserve the
declared contract. Core, Studio and SDK release independently. Supporting a new
wire or projection schema requires explicit compatibility negotiation, not merely
a product version change.

On connection the same-user Unix client verifies discovery permissions, exact
live process and home identity, peer credentials and handshake identities.
Compatibility checks protocol/Application/catalog identity, not matching commits
or product patch versions. Core product versions are reported independently.

Use `HostClient::connect(home, ClientKind::Studio)` then `call`, `status` or
`subscribe`. Capability discovery is `application.capabilities`; only advertised
operations are available. `OperationRequest.input` and result data are bounded JSON
projections using the operation's published contract, not private engine records.
Common typed projections live in `projections`. Error/refusal result states remain
distinct from transport failure. `call_typed` marks lost transport responses as
indeterminate; it never automatically replays mutations. Observe durable operation
identities through the advertised execution operations before retrying.

Subscriptions carry Case invalidation facts, not replacement Case state. Reconnect
and instance changes require resynchronizing affected views. Closing a connection
does not stop Core or cancel admitted work. `start(home, installed_yai, &["host",
"serve"])` may launch an installed Core; it never requires Core source or embeds a
server. Shutdown is an explicit supported host operation.

Linux same-user local transport is qualified; no remote/cloud transport is claimed.
`cargo test` is standalone conformance software evidence. Private Core must run
its own real server tests against these wire/client owners. The SDK's mock server
tests do not establish Case semantics, authority or runtime qualification.

See [qualified scope](docs/qualification.md) for independent clone, real Core
and native Studio evidence and the limits of those claims.

Source SDK is MIT; dependencies retain their own licenses. This repository does
not qualify a binary/customer package for distribution. Such artifacts require
an exact dependency closure, notices and the `distribution.legal.v1` gate.
