<!-- docs:metadata
title: Connect and Use YAI
id: yai-sdk.guides
document: guide
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Connect and Use YAI

**Use an installed Core, never its source tree.**

[Documentation](../README.md) · [Contract](../contracts/README.md)

For YVEX-owned device/Host facts, use the separate [YVEX client guide](yvex.md).
It does not confer access to YAI Cases.

## Inspect an authorized Case

Select an explicit `YAI_HOME` containing your enrolled identity. Start or attach
to an installed compatible Core using its supported `yai host` commands. Do not
point an example at another user's profile or copy private store files.

```sh
cargo run --locked --example cases -- "$YAI_HOME"
```

The example negotiates, discovers capabilities, lists currently visible Cases and
opens each through Core admission. An empty list is not missing storage; it may
reflect the caller's visibility. No example silently creates authority.

## Submit an action deliberately

After discovering the catalog, use `client.cases().create(correlation, &input)`
with a typed `CaseCreateInput` containing the exact Tenant and new Case reference.
Core may refuse. Check `Response.state` before consuming `data`, and retain the
returned Case generation. Authority methods similarly require explicit Review,
Participant and rationale inputs. No helper infers a participant or approval.

If communication becomes indeterminate, do not wrap the call in a retry loop.
Observe the domain identity using the supported read before deciding what to do.

The executable [create example](../../examples/create_case.rs) deliberately creates
one named Case, assigns an operator role, links the authenticated principal and
opens the Case. Use an isolated evaluation profile and explicit identities:

```sh
cargo run --locked --example create_case -- "$YAI_HOME" tenant:example case:example participant:example
cargo run --locked --example recall -- "$YAI_HOME" case:example 'current task'
cargo run --locked --example conformance -- "$YAI_HOME"
```

The [Recall example](../../examples/recall.rs) obtains a fresh authorized generation
and requests bounded Recall v2 at that generation. It prints the complete received
evidence, including missingness and limitations. Recall remains Core-derived
evidence, not SDK authority or an implicit model invocation. The
[conformance example](../../examples/conformance.rs) checks all published operation
contract identities on a real Host without requiring its source checkout.

## Consume finite navigation

Use `client.provider().register_finite(correlation, &FiniteProviderRegisterInput)`
for an explicitly approved public computational target. The input carries public
lineage and native SSH credential **references**, not secrets or a model activation.
Core separately owns trust, exact current-W qualification through
`client.provider().qualify_finite` and Participant cognitive binding.

`client.cognitive().fast_search(correlation, &FastSearchInput)` consumes current W
already obtained from Core. Observe response state before data. `scored` carries
the exact navigation/request, signed fixed-point raw scores, scale, ranking,
binding/qualification and safe computation references. Those scores are
uncalibrated; never label them confidence. `deterministic_fallback` carries a precise
reason and may retain completed computational evidence without a qualified ranking.
`outcome_unavailable` does not mean no compute or permit retry. A stale/unauthorized
Core result does not authorize using an older frontier.

These are YAI semantic workflows, not calls clients should replace with direct
YVEX scoring. Studio must reconcile the three new operation dispositions. Current
real-model Fast Search qualification remains external to SDK conformance.

## Observe and resynchronize (Host)

### Ordinary Conversation context

`client.conversation().send_with_search(correlation, &TaskConversationInput)`
uses the existing `conversation.send` operation. `memory_search_mode` is
`standard` or `fast`; Core decides whether a meaningful, admitted finite
selection exists. This input variant preserves `send_text` and its historical
input bytes. Clients must not run a separate search and construct their own W.

Use `get_context(correlation, &ConversationContextGetInput { include_context:
true, ... })` to observe the exact submitted identity through `execution.get`.
This is a current-disclosure forensic read, not refresh, redispatch or admission.
`prepared_context.invocations` retains exact lineage and optional
`ContextPreparationObservation`: selected/omitted evidence identities and
reasons, family byte contributions, semantic bounds, deterministic/System-1
selection, raw-score ranking and bounded computational evidence. Full W/frame
and provider-input facts remain separately represented; actual producer token
counts must not be confused with conservative semantic-unit estimates.

An older compatible Core can omit `preparation`. `unavailable_reason` means
derived backing could not be observed; it is not zero input, success or a retry
grant. `finite_dispatch = outcome_unavailable` requires retaining the exact
attempt identity, never an automatic resend. SDK contracts establish no context
sufficiency, authority, calibrated confidence or model-quality claim.

For a configured provider, [models](../../examples/models.rs) uses only the
YAI registered-target operation:

```sh
cargo run --locked --example models -- "$YAI_HOME" tenant:example provider-target:example
```

A returned model name is catalog evidence, not current inference availability or
permission. The SDK neither contacts the provider directly nor loads its engines.

The low-level `HostClient::subscribe` provides typed `HostEvent` values. Use a
separate connection for subscription and operations. Case updates invalidate the
named views; fetch a fresh authorized projection. A new Host instance requires a
new bound client and catalog. Closing the subscriber does not stop Core.

The existing [structured client](../../examples/client.rs) can inspect status or
subscribe from a separate process. Its generic JSON operations are a diagnostic
surface; ordinary code should prefer the typed workflow methods where present.
