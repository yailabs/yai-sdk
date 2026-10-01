<!-- docs:metadata
title: Public Client Contract
id: yai-sdk.contracts
document: reference
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Public Client Contract

**Identity-bound communication; Core admits meaning.**

[Documentation](../README.md) · [Reference](../reference/README.md)

## Transport and compatibility

`yai.client.v1` uses bounded newline-delimited JSON over a same-user Linux Unix
socket. The frame bound is 8 MiB including its delimiter. The Rust
[wire owner](../../src/wire.rs) defines client/server envelopes. Discovery is a
supported endpoint record, not a license to inspect persistence. Client checks
include filesystem permissions, home identity, peer UID/PID, boot identity and
process start identity before accepting the handshake.

The handshake binds Host instance, home, Core product version, client protocol,
Application `yai.studio.application.v1` and catalog
`yai.application_capability_catalog.v1`. Matching commits are never required.
An incompatible identity refuses before ordinary operations.

## Operations and outcomes

Requests carry Application protocol, operation identity, correlation identity and
the operation-specific input. Correlation is not an idempotency token. Effectful
operations have domain-owned submission/plan identities, which clients preserve.

The [released operation inventory](../../contract/operations.json) records public
IDs, input/output contracts, impacts and authority classes. A connected catalog
is the current instance's observation. Metadata is not an authorization grant.
The typed client checks supported operation/schema identity; Core performs actual
disclosure, authority and state admission.

`success`, `partial`, `unauthorized`, `stale`, `core_pending`, `not_implemented`,
`transport_unavailable` and `error` are distinct semantic result states. Preserve
the exact error code and safe message. A missing catalog entry is SDK unsupported;
a Core refusal is a received result, not a dropped connection.

Before-dispatch connection or encoding failure is definite non-delivery. Failure
after dispatch is indeterminate unless the producer supplies stronger evidence.
No SDK layer automatically retries a mutation. Observe its durable identity via
the published execution operations; a new correlation does not authorize replay.

## Public projections and privacy

Mutation receipts bind Case identity/generation and bounded Transition identity
facts. They never export persisted CaseState/Transition layouts. Probe projections
exclude carrier tokens, private process ownership and storage seals. Their
negative controls must survive language projection changes.

The [workflow contract](../../contract/workflows.json) generates shared Rust and
TypeScript definitions. Remaining TypeScript-only projections describe supported
client values, not a TypeScript transport. Unknown fields are rejected where a
closed privacy boundary is declared; otherwise additive response fields may be
ignored by a narrower projection. The typed response retains the received raw
envelope for inspection of diagnostic or additive data.

## Observation and lifecycle

Commercial installation activation uses the product owner, never a Supabase
password client or a local account database. `product().activation_start` accepts
an installation name; Core returns a same-service browser URL and short-lived
correlation code. The user signs into the real commercial account in that browser
and explicitly approves the installation. `activation_poll` preserves the exact
activation reference and returns pending/slow-down/activated/refused/indeterminate
posture. A consumed exchange with a lost response is not automatically retried.

`auth`, `refresh_auth` and `sign_out` expose separate session, service and verified
offline-access observations. Bearer/PKCE credentials, installation private keys
and signed token bytes never belong to these DTOs. Core retains them through its
native credential owner. Sign-out is not entitlement revoke or Case deletion.
Account references, commercial installation references and local YAI identity
are distinct. A contract fixture is not live account activation; native Studio
consumption and commercial issuer evidence require independent qualification.

For bounded Case Work, `conversation.send` accepts an executor Participant and
finite `work_limits` in the typed intent. Its acknowledgement names the one
committed Turn/request; `execution.get` returns a `work` projection with exact
thread, step, provider selection/target, invocation, Operation and outcome
references. New Work may bind an optional exact per-invocation
`max_output_tokens` ceiling; historical Work without it retains its original
intent bytes and provider response contract. `conversation.work.resume` accepts
that same submission identity and observed generation. It is explicit
continuation of a recoverable intent,
not a fresh SEND or automatic retry. A client must preserve `awaiting_review`,
`budget_exhausted`, `delivery_indeterminate` and terminal completion as distinct
postures; an indeterminate delivery is never permission to dispatch again.
These contracts expose YAI meaning and do not grant the executor Participant
the submitting Principal's authority.

Case events carry exact Case, generation, sequence, cursor and affected-view
facts. They invalidate cached views; they are not replacement canonical state.
Resynchronize via supported reads after reconnect, missing continuity or Host
replacement. Preserve unsent local buffers rather than overwriting them with a
fresh server view. Bound local clients refuse replacement before dispatch and
require rediscovery.

Attachment and subscription do not own Core's lifetime or cancel admitted work.
Installed-process launch is optional client lifecycle support, not an embedded
server. Stopping Core is an explicit supported operation, never a side effect of
closing a Studio window.
