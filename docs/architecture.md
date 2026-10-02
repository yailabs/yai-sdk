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

Accepted decision: YAI Core owns governed Case semantics and its resident server
lifecycle; YVEX owns model/runtime truth; the SDK owns separate public client
projections, compatibility and conformance; Studio owns presentation and local
UI state. This repository is independently MIT licensed; no private repository
history or semantic implementation is copied.

The Cargo workspace contains the existing `yai-sdk` client and a separate
`yvex-sdk` crate. Neither depends on the other. The former speaks the YAI local
Application contract. The latter exposes YVEX's versioned,
read-only, identity-pinned OpenSSH management contract. Its canonical producer
is YVEX's public [remote-management contract](https://github.com/yailabs/yvex/blob/main/docs/contracts/remote-management.md),
not a second SDK-owned model/runtime registry. It does not expose the private
YVEX Unix wire or wrap human CLI output. Adding a Rust type does not establish
live YVEX service availability.

The `yvex-sdk::openai` module separately interprets YVEX's public profile-v3
catalog and exact-request preflight reports. It takes a caller-owned HTTP fetch
callback, so YAI retains endpoint locality, credentials, transport policy and
Case admission; only the YVEX-specific payload contract is projected here.
This module neither sends generation nor creates a model-session authority.

`yvex-sdk::finite` is the separate semantic-neutral finite-decision client.
With the optional `finite-decision-native` feature, Rust bindings are generated
from verified installed public YVEX headers and call
`yvex_finite_producer_execute_local` in the public C client archive. YVEX owns
private transport, model input construction and execution. SDK owns byte bounds,
copied projections and exact population/generation/result checks. Without the
native client, it refuses explicitly; it does not implement a substitute scorer.
The [ABI projection](../crates/yvex-sdk/contract/finite.json) follows YVEX's declaration
signatures and schema-v1 records, not producer Git equality. Native compilation
and synthetic ABI conformance are not real-model or YAI Fast Search qualification.
See [consumer setup and current packaging boundary](guides/yvex.md#finite-decision-public-c-client).

Control flows from clients into the owning service; facts flow back from YVEX
through the provider adapter to YAI where a Case result is involved. Studio
may read YVEX operator facts directly, but any Case-affecting action remains
on the governed YAI Application path. YAI CLI and SDK are sibling clients of
YAI owners; YVEX CLI and SDK are sibling projections of YVEX owners.

The Rust client and public TypeScript DTOs describe the same Application JSON
projection surface. TypeScript declarations are projected contracts, not private
CaseState representations. Core's current-admission operation dispatcher remains
the sole authority. Capability metadata grants no permission.

`contract/workflows.json` owns the shared result envelope, invalidation event,
common value projections and typed workflow descriptors. Deterministic generation
produces Rust and TypeScript; `--check` rejects edits to either projection.
`contract/operations.json` binds the released 111-operation inventory. Core
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

When a client selects an exact installed Core through `YAI_EXECUTABLE`, SDK
telemetry also compares that executable's device/inode with the fenced running
Host process. `matches_running`, `different_from_running`, `installed_missing`
and `unknown` are local installation observations, not protocol compatibility
or a reason to replay work. The existing process-link observation separately
reports an executable replaced on disk. Clients can distinguish a stale Host
from a compatible binary that is merely installed; replacing or restarting
the Host remains an explicit lifecycle operation after active work is checked.

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

## Native peer credentials

The local Unix transport admits native peer PID/UID facts. Linux uses
`SO_PEERCRED`; macOS uses `getpeereid` plus `SOL_LOCAL/LOCAL_PEERPID`.
Returned structure length and positive PID are checked before publication.
Unsupported Unix platforms refuse; a failed query never substitutes discovery
file values or caller identity. Same-user and discovery process checks remain
mandatory.

This does not port `LocalProcessIdentity`: its v1 schema, start ticks, canonical
identity and live-process observation remain Linux-defined. Capture on macOS
explicitly returns `process_identity_unsupported_platform`. A conforming native
Host/start identity requires coordinated Core/SDK work; peer queries and a
compilable desktop shell alone do not establish authenticated macOS Host use.
