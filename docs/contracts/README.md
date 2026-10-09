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

### Product Access source and authenticated profile

The product workflow methods are `access_status`, `enable_development`,
`disable_development` and `account_profile`. Their Application IDs are
`product.access.get`, `product.access.development.enable`,
`product.access.development.disable` and `product.account.profile.get`.
Enable/disable take `ProductDevelopmentAccessInput { expected_revision }`;
reads take `EmptyInput`. `ProductBootstrapProjection.access` additionally carries
the same optional source observation for backward-compatible bootstrap consumption.

`ProductAccessSourceObservation` reports `commercial`, `local_development` or
`unconfigured_pre_release`, `allowed/refused/not_enforced`, serving build capability,
explicit profile opt-in/revision, commercial quota applicability and precise refusal.
`ProductAccountProfileObservation` reports `not_applicable`, `authentication_required`,
`unavailable`, `current`, `stale` or `unsupported_contract`; optional holder/name/email,
`not_reported/verified/unverified` email verification and observation/reason fields.
Current authenticated native profile display is unavailable except holder reference;
the browser-only account route is not a supported native fetch contract.

Neither DTO exposes credentials, entitlement bytes or provider objects. Commercial
profile unavailability is not an access refusal; local development is not a paid
plan. `product_development_build_required` refuses an ordinary serving build even
if a client requests development. Unknown operation/contract remains unsupported,
not a local-preview fallback. Core product ALLOW still requires Case authority.

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

The independent YVEX domain also projects the schema-v1 public finite producer
through `yvex-sdk::finite`, not the YAI Application wire. Question/context and
ordered opaque candidates cross the public native C client; results retain
model/binding/tokenizer/input/program/population/result identities and engine
generation. Scores are model logits and relative **uncalibrated** probabilities.
Wrong generation/population, non-finite scores, generated output and unsupported
calibration refuse. A YVEX computational result does not admit a YAI Decision;
the YAI consumer still owns disclosure, current-W validation and qualification.

Commercial account login uses the Product Auth owner, never a Supabase client or
a local account database. `product().login_start` accepts `{client: studio|cli,
name}` and returns `ProductLoginObservation` (`yai.product_login.v1`). Open its
`authorization_url` in the system browser on the same machine as the Host.
`product().login` observes automatic callback completion; `cancel_login` accepts
the exact `login_ref` and cancels local waiting, not remote account access.
Awaiting browser, exchanging, authenticated, cancelled, expired, unavailable,
refused, interrupted and indeterminate are distinct typed postures. A validated
callback sets `callback_received`; it is not a licensed-access or window-focus
claim. A client disconnect observes the same pending attempt. After Host loss,
waiting is interrupted and an already-started exchange is indeterminate; no
automatic exchange retry occurs.

Core owns the loopback listener, independent state and PKCE S256, fixed origin
and issuer, 600-second request and one-use token exchange. The public web
[candidate contract](https://github.com/yailabs/web/blob/841b76a96a5ebde349bfa8db420c9a177d210fa3/docs/native-account-login.md)
uses `yai.native-session.v1` with a signed v2 entitlement; refresh deliberately
still uses `yai.product-activation.v1`. Legacy `activation_start/poll` remain
compatibility operations, not the selected product sign-in flow. No entered code,
copied license or separate installation confirmation is required. Fully headless
authentication is deferred; a browser on another machine cannot complete a local
loopback callback.

`auth`, `refresh_auth` and `sign_out` expose separate session, service and verified
offline-access observations. Bearer/PKCE credentials, installation private keys
and signed token bytes never belong to these DTOs. Core retains them through its
native credential owner. Sign-out is not entitlement revoke or Case deletion.
Account references, commercial installation references and local YAI identity
are distinct. A contract fixture is not live account activation; native Studio
consumption and commercial issuer evidence require independent qualification.
An authenticated v2 account can still have `licensed_progress_allowed: false`
and `verification_refusal: unsupported_policy` for unadmitted draft policy.
Do not navigate into licensed operation based on login posture alone. Required
public configuration belongs to Core's `product-commerce.json` (service origin,
pinned issuer/public keys, explicit loopback-HTTP development opt-in), not Studio
Supabase environment variables. Missing configuration is `core_pending` with
`commercial_service_not_configured`; 503 rollout refusal is not revocation.

`VerifiedProductAccessObservation` optionally carries an opaque `plan_ref` and
a typed `ProductAccessPolicy`. The policy projects identity/revision, scope,
commercial owner/assignment, capability grants and a map of quantitative limits.
A null limit is unlimited; absent policy and unknown usage are not unlimited or
zero. SDK does not admit revisions, derive grants from plan names or manufacture
usage. The [Core policy owner](https://github.com/yailabs/yai/blob/main/docs/reference/product-access-policy.md)
defines local admission. Member/pool vocabulary is representable, not evidence
that pooled offline enforcement or future advanced features exist.

The Core-required policy is not yet issued by web `841b76a`, which still signs
the explicitly unadmitted quota draft. Typed projection and fixture verification
do not qualify commercial login, a live final policy, Studio Home or billing.

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

The TypeScript `@yai/sdk/projections` surface uses the generated
`ConversationIntent`, `ConversationPosture` and `CaseWorkObservation` owners
directly. `ConversationSendInput.intent` therefore carries the same optional
executor, Workflow execution and finite limits as the Rust workflows;
`ConversationExecution.work` retains exact ordered step lineage. An ordinary
SEND with no `work_limits` stays an ordinary SEND. Explicit continuation uses
`ConversationWorkResumeInput.observed_generation`, not a new submission ID.
These are compatible projections of the existing operations, not streaming or
new client-side execution semantics. A primary provider result, a Work answer
and completion of an authorized effect must not be conflated.

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
