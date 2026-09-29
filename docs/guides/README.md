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

## Observe and resynchronize

The low-level `HostClient::subscribe` provides typed `HostEvent` values. Use a
separate connection for subscription and operations. Case updates invalidate the
named views; fetch a fresh authorized projection. A new Host instance requires a
new bound client and catalog. Closing the subscriber does not stop Core.

The existing [structured client](../../examples/client.rs) can inspect status or
subscribe from a separate process. Its generic JSON operations are a diagnostic
surface; ordinary code should prefer the typed workflow methods where present.
