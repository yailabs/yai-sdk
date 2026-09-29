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
