<!-- docs:metadata
title: Governed multi-provider runtime contracts
id: yai-sdk.guides.provider-runtime
document: guide
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Governed multi-provider runtime contracts

[Guides](README.md) · [Public contracts](../contracts/README.md)

SDK 0.4 projects YAI-owned provider semantics; it does not implement inference,
calculate Authority, manage models, or orchestrate Case Work. The independent
YVEX client remains unchanged. TypeScript remains a types-only contract package.

## Published inputs and observations

| Public operation / typed Rust method | Contract |
| --- | --- |
| `provider.protocols` / `provider().protocols` | Consumer protocol inventory, explicitly not server qualification |
| `provider.register` / `provider().register` | Exact target, optional identity-bound HTTP profile |
| `provider.probe` / `provider().probe` | Immutable explicit synthetic submission, optional streaming and common workload |
| `provider.probe.get` / `provider().probe_run` | Retained original run, no automatic retry |
| `provider.credential.rotate` / `provider().rotate_credential` | Non-secret revision; invalidates previous credential qualification |
| `provider.credential.store` / `provider().store_credential` | One protected native replacement, no secret in public observations |
| `provider.credential.status` / `provider().credential_status` | Exact storage/recovery posture without rewrite |
| `provider.case.setup` / `provider().setup_case` | Separate model executor, admitted model view and bounded profile; explicit partial steps |
| `conversation.readiness` / `conversation().readiness` | Current automatic admission and requestable capability facts, without SEND |
| `provider.lab.compare` / `provider().compare_runs` | Retained probes OR disclosure-qualified canonical executions; never redispatches |
| `product.distribution.verify` / `product().verify_distribution` | Publisher signature/component verification, not entitlement or Case authority |

The canonical [operation inventory](../../contract/operations.json) has 138
operations; [workflow types](../../contract/workflows.json) generate both Rust and
TypeScript projections. Older operations and explicit bounded Work remain supported.
SDK 0.2 deliberately changed Rust source types for automatic observation limits
to optional, so consumers must handle an explicitly admitted text profile.
SDK 0.3 adds the optional approved public DER TLS trust anchor to the profile;
Rust struct-literal consumers must account for the additional field. Old JSON
profiles without that field remain unchanged.

SDK 0.4 adds `provider().store_credential` and `provider().credential_status`,
with contracts `yai.provider_credential_store_input.v1`,
`yai.provider_credential_status_input.v1` and `yai.provider_credential_storage.v1`.
Store input is an intentionally sensitive, same-user native transport request:
`target_ref`, unique `revision_label`, and bounded `secret`. Never log/serialize it
to a journal, fixture, model context or diagnostic file. Rust Debug is redacted;
owned input and wire buffers are zeroized on drop. This does not erase every caller,
JSON or JavaScript string copy, or protect an attacker-controlled process memory.

Core records only a non-secret write identity and revisions. All same-Tenant
targets sharing the credential reference are invalidated atomically before one
native write. The OS-protected envelope must match the current write identity;
old or unbound native material is not silently qualified. A failed/uncertain write
is observed with `credential_status`, never automatically retried. Reusing an
already prepared revision label refuses without another native write.

Status exposes `write_ref`, exact revisions, `stored/not_stored/unbound/unavailable`,
reason, `credential_available` and `case_authority=false`, never secret bytes.
`stored` is local storage evidence, not authentication success, provider qualification,
commercial entitlement or Case authority. Locked/unsupported vaults are explicitly
unavailable. These operations require current Tenant Owner authority; a caller
must negotiate their descriptors because an older installed Host lacks them.

Five public protocols are represented: `open_ai_chat`, `anthropic_messages`,
`google_generate_content`, `cohere_chat`, `bedrock_converse`. Brands sharing a
protocol do not create SDK implementations or model allowlists. The Core inventory
reports implemented capabilities; each actual target must separately qualify.

`ProviderHttpProfile` carries authentication *kind*, paths, API version, output
token field, expected response alias and optional expected reported deployment
fingerprint. It never carries a secret. Remote inference opt-in defaults off.
For a private HTTPS service, `tls_trust_anchor_der` is the base64-encoded public
certificate independently approved for that target. It is not a private key,
TOFU discovery, an alternate hostname or an option to bypass verification.
Absent an anchor the normal WebPKI roots apply; an explicit anchor restricts trust
to that approved root while preserving hostname, TLS and DNS/locality checks.
`catalog_path` omitted means exact-model/catalog-less operation, not an invented
catalog. Producer-reported metadata is not independently verified model weights.

## Automatic execution and compatibility

Existing `execution: "automatic"` requires the admitted native Work profile.
New callers can explicitly use `execution: "automatic_v2"`: Core selects the
current admitted Work OR text-only answer profile. It never substitutes text for
a failed native Work profile. Older omitted execution fields remain legacy.

`ProviderCaseSetupInput` requires exactly one of `conversation_limits` or
`answer_limits`. Neither a profile nor Skill assignment grants Resource authority.
The executor is distinct from the human author and must not have a Principal link.
Multiple eligible automatic executors are an explicit ambiguity.

Readiness exposes exact binding/qualification/suitability/selection facts, limits,
credential/opt-in blockers and available requestable capabilities. A listed tool
still requires the current Decision/Review/effect admission. Setup steps retain
generation, changed disposition and Transition/binding references without dumping
private state or requiring clients to interpret an internal persistence object.

## Laboratory and recovery

Provide either `runs` (target + original synthetic submission) or `executions`
(Case + author + original Conversation submission), 2–16 distinct references.
Mixed evidence classes are refused. Case comparisons use current disclosure and
retained W/ContextFrame/input identities; no private journal is required.

Synthetic common workloads retain suite version, objective and an exact-text
oracle. The existing synthetic response budget remains 96 tokens. Oracle failure
is a workload outcome, never proof that a different model is superior. Arbitrary
complex Case tasks use ordinary bounded Conversation, not this synthetic probe.

Reported token usage, conservative wire estimates, measured caller duration and
context preparation remain separate. Unknown prefill/decode/compute/memory facts
are null, not zero. Different execution baselines are not a controlled comparison
unless an external evaluation oracle establishes their comparability.

Interrupted submissions remain observable, never retried. Losing a response is
indeterminate; changing provider does not make redispatch safe. Progressive public
fragments are provisional and do not create a canonical answer or Receipt.

## Studio handoff

The executable [public Rust example](../../examples/provider_runtime.rs) connects
to an already running supported Host and performs only negotiated reads:

```sh
cargo run --locked --example provider_runtime -- /absolute/yai/home protocols
cargo run --locked --example provider_runtime -- /absolute/yai/home readiness case:exact participant:author
cargo run --locked --example provider_runtime -- /absolute/yai/home compare comparison.json
```

The Home selects the existing local transport, not a private database read or a
client submission journal. An unsupported operation or semantic refusal remains
explicit; the example does not launch a Host, qualify a provider or retry a SEND.
Operator product checks use the official `yai ...` commands in Core's runbook.

Consume SDK 0.3's typed contracts and negotiated operation descriptors. Inspect
protocol support and current targets separately. Show configured versus observed
identity and unknown runtime/artifact facts. Use readiness rather than SEND as a
compatibility probe, and display partial setup and exact current blockers.

For native credential entry use the protected native facility and its Tenant-scoped
reference, then rotate the non-secret revision through the public owner. Never put
credentials in SDK profiles, laboratory evidence, UI metadata or reports.

Qualify native consumer interaction independently: text profile with v2 opt-in;
native Work unchanged; unavailable/ambiguous executor; stale generation; missing
credential; refused disclosure; interrupted probe/stream; same-submission recovery;
explicit null producer facts; no implicit Review approval or effect replay.
Core fixture evidence does not qualify Studio or live commercial services.
