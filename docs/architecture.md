<!-- docs:metadata
title: Client Architecture
id: yai-sdk.architecture
document: architecture
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Supported client boundary

[Documentation](README.md)

Accepted decision: Core owns semantics and resident server lifecycle; SDK owns
public projections, compatibility, wire framing and client behavior; Studio owns
presentation, interaction and local UI state. This repository is independently
MIT licensed; no private repository history or semantic implementation is copied.

The Rust client and public TypeScript DTOs describe the same Application JSON
projection surface. TypeScript declarations are projected contracts, not private
CaseState representations. Core's current-admission operation dispatcher remains
the sole authority. Capability metadata grants no permission.

`contract/workflows.json` owns the shared result envelope, invalidation event,
common value projections and typed workflow descriptors. Deterministic generation
produces Rust and TypeScript; `--check` rejects edits to either projection.
`contract/operations.json` binds the released 98-operation inventory. Core
conformance compares that inventory with the real catalog, while ordinary clients
may use a compatible subset. This is contract ownership, not copied admission.

The typed `Client<T>` sits above `ClientTransport`, fences operation schema and
response identity, and preserves raw received outcomes. `BoundLocalTransport`
authenticates each connection and refuses a replaced Host before dispatch;
callers explicitly rediscover and resynchronize. There is no automatic mutation
replay and no provider bypass.

Mutation receipts contain public Case identity/version and selected transition
identity facts, not canonical CaseState or persisted Transition layouts. Provider
probe projections retain observable request, timing, evidence and qualification,
not carrier tokens, process ownership or storage seals. Core constructs these
typed projections at the Application producer; the SDK neither scrubs arbitrary
private JSON nor reconstructs semantic state. Removing formerly accidental private
fields fixes producer exposure and does not promise payload-byte compatibility.

`yai.client.v1` is bounded newline JSON on a same-user Linux Unix socket. Discovery
is an explicit supported local endpoint record, not permission to inspect stores.
Server peer credentials and authenticated process/start identity prevent stale PID
reuse. Handshake advertises independent Core SemVer, wire protocol, Application
protocol and capability schema; Git identity has no compatibility meaning.
After checking the private discovery file's ownership and YAI_HOME identity, a
dead process identity is reported as stale before comparing protocol versions.
A live incompatible Host still refuses before application dispatch. This lets
supported startup reclaim stale metadata without treating it as a live protocol
mismatch.

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
