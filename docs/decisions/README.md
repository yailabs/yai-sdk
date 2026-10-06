<!-- docs:metadata
title: SDK Decisions
id: yai-sdk.decisions
document: reference
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK Decisions

**Durable client boundaries, separate from delivery chronology.**

[Documentation](../README.md)

## SDK-D01 — Project typed workflows over admitted operations

Accepted for YAI.SDK.PLATFORM.0. Keep the qualified low-level client and transport.
Add capability-aware typed domain accessors, with caller-supplied correlation and
domain-owned submission identities. A received refusal remains a result; transport
loss after dispatch remains indeterminate. No automatic mutation replay.

A language-neutral workflow descriptor generates shared request/result records and
operation bindings. Core retains semantics and current admission. Public DTOs are
projections, never a reason to publish persisted state. Closed privacy records keep
their refusal tests. TypeScript remains contracts-only until a transport is earned.

This avoids both a client-side second engine and a public API consisting only of
untyped operation strings. It does not introduce provider plugins, direct YVEX
control, internet transport or a second server lifecycle.

## SDK-D02 — YVEX owns its independent public client

Accepted for `YVEX.PLATFORM.CONTROL.PLANE.CONVERGENCE.0`. Canonical Rust transport,
contracts and generated TypeScript projections belong to the public YVEX
repository. An independent YVEX client must not require YAI or Studio merely to
operate model, build, runtime or Session lifecycle.

Retain the historical `yvex-sdk` 0.1 package as a thin reexport of canonical 0.2,
and `@yai/sdk/yvex` as a type reexport of `@yvex/sdk`. Preserve compatibility entry
points and examples without retaining a duplicate implementation. Explicit exact
pins and producer/client conformance govern consumption. YAI Application contracts,
Case authority and the independently versioned finite-computation boundary remain
unchanged. Publication and real runtime evidence are separate from this decision.
