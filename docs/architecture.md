# Supported client boundary

Accepted decision: Core owns semantics and resident server lifecycle; SDK owns
public projections, compatibility, wire framing and client behavior; Studio owns
presentation, interaction and local UI state. This repository is independently
MIT licensed; no private repository history or semantic implementation is copied.

The Rust client and public TypeScript DTOs describe the same Application JSON
projection surface. TypeScript declarations are projected contracts, not private
CaseState representations. Core's current-admission operation dispatcher remains
the sole authority. Capability metadata grants no permission.

`yai.client.v1` is bounded newline JSON on a same-user Linux Unix socket. Discovery
is an explicit supported local endpoint record, not permission to inspect stores.
Server peer credentials and authenticated process/start identity prevent stale PID
reuse. Handshake advertises independent Core SemVer, wire protocol, Application
protocol and capability schema; Git identity has no compatibility meaning.

Client attach and subscription do not own Core lifetime. Explicit launcher support
starts an installed executable with `host serve`, never links Core into Studio.
An admitted mutation may survive disconnect: no automatic replay occurs; typed
transport failures after dispatch are indeterminate. Consumers observe exact
durable submission identity through supported operations before retry.

Case updates invalidate views and carry exact Case/generation/cursor identity.
Reconnect or host-instance change requires resynchronization and must preserve
client-local dirty buffers. SDK does not reconstruct missing Case state.

The finite MVP is local Linux transport. Internet transports, authorization across
OS users, semantic policy and frontend design are not implemented here.
