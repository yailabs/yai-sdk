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
identity-pinned OpenSSH management contracts. Version 1 remains read-only;
version 2 adds explicitly granted product lifecycle operations and durable Job
observation. Its canonical producer
is YVEX's public [remote-management contract](https://github.com/yailabs/yvex/blob/main/docs/contracts/remote-management.md),
not a second SDK-owned model/runtime registry. It does not expose the private
YVEX Unix wire or wrap human CLI output. Adding a Rust type does not establish
live YVEX service availability.

`yvex-sdk::finite::remote` separately consumes the public
`yvex.finite.request.v1` / `yvex.finite.response.v1` computation protocol over
restricted SSH, with explicit finite-decision enrollment rather than management
permission. It reuses the semantic-neutral finite input/result projection and
the approved `SshConnection` trust substrate. Exact configured source, model,
binding, tokenizer, program and input-policy identities must match alongside
correlation, alias, generation and ordered candidate IDs. There is no native
library requirement for this remote client, no private socket forwarding and no
lifecycle mutation. Bounded concurrent pipe I/O prevents transport deadlock;
timeout/cancellation/reply loss conservatively preserve unknown computational
outcome. Correlation is not deduplication; no call is retried automatically.
This client is not a YAI realization-selection or qualification owner.

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
may operate YVEX lifecycle and explicitly separate diagnostic generation through
public management; any Case-affecting action remains
on the governed YAI Application path. YAI CLI and SDK are sibling clients of
YAI owners; YVEX CLI and SDK are sibling projections of YVEX owners.

The Rust client and public TypeScript DTOs describe the same Application JSON
projection surface. TypeScript declarations are projected contracts, not private
CaseState representations. Core's current-admission operation dispatcher remains
the sole authority. Capability metadata grants no permission.

`contract/workflows.json` owns the shared result envelope, invalidation event,
common value projections and typed workflow descriptors. Deterministic generation
produces Rust and TypeScript; `--check` rejects edits to either projection.
`contract/operations.json` binds the released 115-operation inventory. Core
conformance compares that inventory with the real catalog, while ordinary clients
may use a compatible subset. This is contract ownership, not copied admission.

Product Access projects commercial, explicit local-development and retained
unconfigured pre-release sources without manufacturing an account or entitlement.
Core alone enforces the default-off compiled development capability plus durable
profile opt-in; SDK reports the serving instance, not its own build or settings.
Commercial profile display is independent of licence validity. The closed safe
profile DTO preserves the authenticated account reference and typed absence until
the commercial service publishes a versioned native-safe profile boundary.
Name/email/verification are never inferred from Principal, Tenant or local labels.

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

The YAI Application MVP uses local Linux transport. The independent YVEX domain
also supports restricted identity-pinned SSH management and finite computation;
deployed listener/model qualification is separate. Internet YAI Application
transport, semantic policy and frontend design are not implemented here.

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

## Native YVEX network connections

The public YVEX product client retains its 36 operations across three transports:
explicitly pinned HTTPS, same-user public Unix companion, and advanced restricted
SSH. The transport does not promote management into YAI inference, provider trust,
Case authority, or a computational Host instance. DNS-SD returns untrusted hints;
a probe observes a certificate, explicit trust retains its exact DER SHA-256,
and the producer separately approves a product-management grant. TLS signatures
are verified and redirects, ambient proxies and cookies are not used.

`management::connections::ConnectionManager` is the shared native client owner.
Safe profiles live under the absolute XDG config root (or HOME/.config), in
`yvex/management-connections.v1`; secrets live only in the platform keyring under
`org.yailabs.yvex.management.v1`. The `native-credentials` feature selects Secret
Service on Linux and Keychain on macOS. No plaintext fallback exists. The current
safe registry lock/local companion implementation is qualified on Linux; Windows
registry locking remains explicitly unsupported. A native caller may supply a
qualified store through `CredentialStore`; injected stores are not a production
security claim. Profiles are immutable Host/client identity bindings. Forgetting
a local profile does not revoke the producer grant.

The Unix companion uses the producer's documented protected socket and OS peer
UID. It checks exact service identity before dispatch through a producer-owned
request precondition, avoiding stale-server mutation after restart. Detecting a
missing companion never starts a Host and never falls back from a failed remote
connection. Core's commercial credentials and YAI authority are not involved.

The SDK connection example and Studio consume the same native owner. This proves
reusable client/profile semantics; it does not claim that the separately owned
YAI CLI has integrated these new network APIs. The producer YVEX CLI owns local
service approval/revocation. Private commercial profile or credential code is
not imported.
