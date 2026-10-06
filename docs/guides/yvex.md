<!-- docs:metadata
title: Inspect a YVEX device
id: yai-sdk.guides.yvex
document: guide
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Inspect a YVEX device

[Guides](README.md) · [Architecture](../architecture.md)

The `yvex-sdk` Rust crate is a separate client domain in this public SDK
repository. It represents facts that originate in YVEX, not YAI Case truth.
Management v1 retains the enrolled, read-only `device.describe` and
`host.status` operations. The separately granted remote finite-decision client
is described below. Neither discovers an untrusted machine, enrolls a peer,
starts YVEX, loads a model, invokes chat generation or inspects a Case.

The same crate also has an `openai` module for the public
`yvex.openai.compat.v3` model-catalog and exact-request capacity preflight
projection. An integrator supplies one already-qualified HTTP fetch function;
the module validates the exact YVEX fields, deployment identity and token
accounting, and passes the complete request bytes unchanged to preflight.
This is a producer extension, not an alternative YAI admission chain or a
promise that an engine can subsequently execute the request.

An operator must independently approve the exact Ed25519 host key and enroll
the client key on the YVEX machine. OpenSSH verifies that key against a private
known-hosts file. The SDK then checks the returned device and peer identities
and request correlation. A reachable address or self-reported JSON identity
is not a trust anchor. The [YVEX contract](https://github.com/yailabs/yvex/blob/main/docs/contracts/remote-management.md)
owns the listener, forced-command and enrollment requirements.

```toml
[dependencies]
yvex-sdk = { git = "https://github.com/yailabs/yai-sdk", rev = "PINNED_REVIEWED_REV" }
```

Construct `SshConnection` with absolute paths to the pinned known-hosts file
and enrolled private client key, the exact address/port/user, and the two
approved `ssh-ed25519:sha256:<64-hex>` identities. `ManagementClient::new`
validates this configuration; `device_describe()` and `host_status()` perform
one bounded read each. The executable [example](../../crates/yvex-sdk/examples/management.rs)
accepts these exact values:

When the host key comes from an authorized YAI `machine.get` projection,
`PinnedHostFile::new(address, port, host_public_key)` may materialize one
short-lived, private OpenSSH trust file. Keep that value alive for the read and
pass `pin.path()` into `SshConnection`; do not use `ssh-keyscan` as approval or
persist the enrolled private client key in a Case. The SDK's Rust
`MachineAssetView` is a typed YAI client projection of the approved pin, not
YVEX runtime state. A revoked registration cannot be used as current trust.

```sh
cargo run --locked -p yvex-sdk --example management -- \
  /absolute/pinned_known_hosts /absolute/enrolled_client_key \
  machine.example 2222 yvex ssh-ed25519:sha256:DEVICE_HEX \
  ssh-ed25519:sha256:PEER_HEX host.status
```

`HostState::Stopped` means the pinned management listener answered but the
YVEX inference host socket was absent. `HostState::Unavailable` means the
listener answered but could not read the current local Host. Neither means
that YAI may dispatch Case work. SSH failure is a transport/trust failure,
not `Stopped`; the client intentionally does not infer a more specific cause
from localized OpenSSH stderr. Model residency and generation are not in this
contract. A Case-affecting action belongs to YAI Application, whose provider
adapter may consume YVEX computation without delegating Case authority.

The standalone SDK tests qualify typed parsing and identity refusal. The
isolated YVEX SSH fixture also invokes this SDK example against the real
forced-command service, including wrong-device refusal and stopped/running
Host observations. Those tests do not claim a deployed remote listener or
real-model inference.

## Finite decision public C client

`yvex-sdk::finite::Request` contains an exact model alias/resident generation,
bounded question/context and ordered opaque candidate IDs/text. It carries no
Case authority, templates, marker/token IDs or model-family selectors. Call
`finite::execute_local(absolute_socket, &request)` once; the public YVEX C client
opens the existing host transport. This SDK never encodes the private wire,
retries automatically or switches generation/model. A caller must independently
qualify the producer and admit disclosure before calling it.

The default crate builds without YVEX and returns `NativeClientNotInstalled`.
To build an actual native consumer, explicitly enable `finite-decision-native`
and supply `YVEX_CLIENT_PREFIX`: a reviewed prefix with
`include/yvex/{core,finite_decision_producer}.h` and `lib/libyvex.a` from one
compatible producer installation. Optional `YVEX_CLIENT_LINK_LIBS` lists the
installation's additional linker library names, comma-separated. Native builds
need Python 3, Clang/libclang and a C compiler; bindings are generated at build
time, never hand-maintained ABI structs. `YVEX_SDK_PYTHON` may select the build
Python. This is not a Python inference runtime.

At producer reference `67a7905`, ordinary YVEX `make install` publishes only the
executable product, not this native client prefix. This SDK does not silently
copy a sibling checkout, manufacture a library or claim a supported producer
installer. Real client installation/runtime qualification therefore remains an
explicit producer packaging dependency until its owner supplies the prefix.
The [public contract](https://github.com/yailabs/yvex/blob/67a7905ea9deb98b0704629a1f979634e19007fb/docs/contracts/c-api.md)
and [declaration projection](../../crates/yvex-sdk/contract/finite.json) own compatibility;
source revisions identify evidence rather than requiring matching repository HEADs.

The [finite example](../../crates/yvex-sdk/examples/finite.rs) takes a socket and
structured request file. Relative candidate probabilities are not calibrated
confidence. The copied result includes zero-generation counters and exact
computational identities; it is not a semantic decision or proof of model quality.

`make check-finite-native YVEX_CLIENT_INCLUDE_DIR=/reviewed/public/include`
compiles an SDK-only synthetic C peer in a disposable prefix and tests bindings,
result/refusal handling and declaration-drift rejection. It never loads a model,
starts a YVEX host or qualifies System-1/Fast Search. Ordinary standalone checks
continue without external headers or private repositories.

## Remote finite decision

The [producer contract](https://github.com/yailabs/yvex/blob/803dd98d4c54d7a26cb3c08def6b350be51b7179/docs/contracts/finite-decision-remote.md)
and its linked request/response schemas own `finite.decision.execute`.
`finite::remote::RemoteClient` uses an approved `SshConnection` on a dedicated
restricted listener enrolled with `--scope finite-decision`. Management-only
keys do not acquire compute permission. This path does not require
`finite-decision-native`, installed headers/archive or a local YVEX socket.
`execute_local` remains exclusively local.

Construct `remote::ProducerIdentity` from independently admitted exact source,
logical-model, binding, tokenizer, physical-program and input-policy identities.
Neither an alias nor generation alone is sufficient. The caller owns producer
qualification, disclosure and YAI Participant policy. Passing these expected
identities to the SDK is not itself a qualification fact.

```rust,ignore
use yvex_sdk::finite::remote::{Invocation, RemoteClient};
let client = RemoteClient::new(approved_connection, admitted_identity)?
    .with_timeout(std::time::Duration::from_secs(5))?;
let invocation = Invocation::new(bounded_finite_request)?;
// Retain invocation.request_id before dispatch. This is correlation, NOT replay safety.
let observation = client.execute(&invocation)?;
```

`execute_cancellable(&invocation, &AtomicBool)` permits explicit local transport
cancellation. `NotDispatched` is reserved for pre-dispatch client rejection or an
authenticated explicit producer refusal. Once SSH starts, transport/pin/auth
failure, timeout, cancellation, malformed/foreign response and native producer
error are conservatively `OutcomeUnavailable`. This does not certify that the
model never ran or that its lease retired immediately. Only an accepted exact
success is `Completed`. No durable receipt/cache exists; reusing a request ID
may execute again. Caller policy must decide any new independent invocation.

Returned observations preserve correlation, peer identities and the existing
`ResultObservation`, including producer-authored time/resource counters and
uncalibrated raw/relative scores. No question/context or untrusted native error
text is rendered in ordinary error diagnostics. Structured reason/name/owner
fields remain explicitly inspectable, not automatically logged.

The [remote example](../../crates/yvex-sdk/examples/finite_remote.rs) accepts:

```sh
cargo run --locked -p yvex-sdk --example finite_remote -- \
  /absolute/pins /absolute/enrolled_key DGX_ADDRESS FINITE_PORT YVEX_USER \
  ssh-ed25519:sha256:DEVICE_HEX ssh-ed25519:sha256:PEER_HEX \
  /absolute/admitted-producer-identity.json /absolute/bounded-request.json 5000
```

`make check-finite-remote` uses disposable keys and an isolated real OpenSSH
listener with a forced synthetic public-protocol peer. It proves client transport,
one dispatch per invocation, mismatched identities/refusal/loss and untrusted
key/pin controls; it does not run YVEX or a model. The optional cross-repository
parity lane checks both published schema layouts and refuses finite-schema drift
separately from management-operation drift. Exon→DGX with a resident finite engine
requires the actual approved listener, enrollment and exact model evidence.


## Product management v2

`yvex_sdk::management` is the separate product-management client. An existing
v1 `management` key remains read-only; the producer must explicitly enroll the
peer with `product-management`. The same independently approved `SshConnection`
mechanics apply. This permission does not grant finite computation or YAI Case
authority. Studio does not invoke a remote shell command or parse CLI output.

The reviewed [management projection](../../crates/yvex-sdk/contract/management.json)
projects producer operations into Rust and `@yai/sdk/yvex` TypeScript types through
`tools/yvex_management.py`. These types describe Models, exact Sources, registry
observations, acquired representations, Builds, Packages, Hosts, Engines,
Sessions, Jobs and events. There are no speculative Training types or operations.
Training can later add a capability and exact lineage into these existing owners.

The client exposes domain facades (`models`, `acquisitions`, `builds`, `packages`,
`hosts`, `engines`, `sessions`, `generations`, `jobs`, `observe`, `registries`,
`sources`, `profiles`). Reads return validated typed observations. Operational methods prepare
an invocation; its identity is available before dispatch:

```rust,ignore
use yvex_sdk::management::{Client, EngineLoadInput};
let client = Client::new(approved_connection)?;
let request = client.engines().prepare_load(&EngineLoadInput {
    profile: exact_profile,
    host_instance: observed_host_instance,
    context_capacity: None,
})?;
let request_id = request.request_id().to_owned(); // retain before dispatch
let admitted_job = client.execute(&request)?;
```

The authenticated response preserves the approved device, enrolled peer and
request identity. Runtime writes additionally fence the exact observed Host
instance; destructive Session operations and generation fence Session identity.
An engine alias or generation number alone is insufficient across Host restart.
Source resume/cancellation fence their exact acquisition operation and generation.

A response timeout, malformed/foreign response or transport cancellation after
SSH starts is `outcome_unavailable`. It does not prove that work stopped. The SDK
never retries. Reobserve `jobs().get` using the retained request ID. An absent
receipt does not authorize resubmission. The producer owns retention and refuses
new admission when retention is full rather than silently reusing identities.
Client disconnect does not cancel a durable Job; cancellation is a separate
explicit producer operation. The finite-computation client retains its own
contract, which does not have these durable receipts.

`job.list` returns bounded summaries without prompts or result bodies. `job.get`
returns the exact receipt, including current result/progress where the owner
provides it. Generation results retain public typed output channels, completion
and metrics. A YVEX Session is computational state, not a YAI Conversation.
Direct management generation never commits a Case Turn or grants Case authority.
Registry authentication is observed through redacted host-owned account facts;
acquisition accepts only an opaque credential reference provisioned by that Host.
Requests accept no raw registry tokens. Profiles retain exact package identity;
removal fences the expected package under the producer registry transaction.
Source cleanup uses acquisition identity/generation and explicit confirmation. No training capability is inferred.

The executable `management_product` example takes approved SSH configuration,
a retained request JSON file and an optional client timeout. This is a public
protocol client, not a production credential installer. Replies are bounded to
1 MiB and requests to 128 KiB. Malformed typed payloads are rejected before
reaching consumer presentation. Capability discovery determines support; a
runtime name or product version is not qualification evidence.

`make check-management-remote` runs isolated real SSH with a synthetic protocol
peer. It qualifies transport/identity/refusal and exact read-after-loss behavior,
not YVEX model execution. Producer lifecycle qualification and native Studio
acceptance remain independently required.
