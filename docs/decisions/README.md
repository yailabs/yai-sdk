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
