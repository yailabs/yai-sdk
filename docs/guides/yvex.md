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
