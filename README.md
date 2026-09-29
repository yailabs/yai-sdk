<!-- docs:metadata
title: YAI SDK
id: yai-sdk
document: product
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# YAI SDK

**The public client library and contract for governed YAI operations.**

YAI SDK lets a client discover capabilities, inspect authorized Cases and submit
supported actions to a running YAI Core. Core admits the operation; the SDK does
not implement Case semantics, policy, persistence or scheduling. Studio consumes
this same boundary. Provider and YVEX operations go through YAI, never around it.

The current functional client is **Rust on Linux, same-user local Unix transport**.
The TypeScript package exports contract types only, not a runtime client.
Internet transports and a provider-plugin API are not supported.

## Add and connect

The independent product line is `0.1.0`. A published source revision is not a
crates.io/npm release; pin an exact reviewed Git revision in your application:

```toml
[dependencies]
yai-sdk = { git = "https://github.com/yailabs/yai-sdk", rev = "YOUR_REVIEWED_FULL_COMMIT" }
```

With an installed compatible Core already running against your explicitly chosen
profile, the SDK performs authenticated discovery and compatibility negotiation:

```rust,no_run
use yai_sdk::{client::{BoundLocalTransport, Client}, workflows::EmptyInput, ClientKind};

# fn main() -> Result<(), Box<dyn std::error::Error>> {
let home = std::env::var("YAI_HOME")?;
let transport = BoundLocalTransport::connect(home, ClientKind::External)?;
let client = Client::discover(transport, "my-client:discovery")?;
let response = client.cases().list("my-client:list", &EmptyInput {})?;
println!("{:?}: {:?}", response.state, response.data);
# Ok(())
# }
```

Use the executable [Case inspection example](examples/cases.rs):

```sh
cargo run --locked --example cases -- "$YAI_HOME"
```

This is a real read-only client, not a fixture and not implicit identity enrollment.
The [workflow guide](docs/guides/README.md) explains setup, actions and observation.

## Capabilities, results and observations

Typed APIs group supported operations into Cases, Identity, Materials, Knowledge,
Memory, Authority, Work, Handoff, Resources, Compute and Conversation.
They preserve the operation's contract identities and refuse a
missing or incompatible catalog entry before dispatch. Discovery never grants
authority: Core rechecks current access for every operation.

`Response<T>` retains both a typed projection and the exact received envelope.
Success, partial, unauthorized, stale, unavailable and unsupported postures must
not be collapsed into a boolean. Transport failures are separate. A lost response
after dispatch is **indeterminate**, not permission to retry a mutation.

Use the low-level operation API for remaining published operations. It remains a
supported escape hatch, not a private engine entry point. Case events invalidate
views; they do not replace state. Reconnect or Host replacement requires fresh
discovery and resynchronization while preserving client-local edits.

## Developer documentation

[Documentation home](docs/README.md) routes product, architecture, contracts,
guides, conformance, qualification, project control and release policy.

- [Public contract](docs/contracts/README.md): framing, identities and failure semantics.
- [API reference](docs/reference/README.md): typed workflows and complete operation inventory.
- [Development](docs/development/README.md): standalone checks and public fixtures.
- [Qualification](docs/qualification.md): actual evidence, not inferred support.

## Versions and licensing

Core, Studio and SDK use independent SemVer. `Cargo.toml` owns this repository's
version; `yai.client.v1`, Application and capability schema identities evolve
independently. Compatibility never requires matching Git commits. See
[release policy](docs/releases/README.md).

SDK first-party source is [MIT](LICENSE); Core and Studio remain proprietary.
Dependencies retain their own terms. Passing source checks is not customer-package
distribution readiness: [legal qualification](docs/distribution.md) remains
artifact-specific and fail-closed. [Provenance](PROVENANCE.md) records extraction.
