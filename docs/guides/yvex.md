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
Its current supported remote scope is deliberately small: the enrolled,
read-only `device.describe` and `host.status` operations from YVEX management
v1. It does not discover an untrusted machine for you, enroll a peer, start
YVEX, load a model, invoke generation or inspect a Case.

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
