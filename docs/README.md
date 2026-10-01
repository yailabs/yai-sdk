<!-- docs:metadata
title: SDK Documentation
id: yai-sdk.docs
document: reference
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK Documentation

**Integrate through a public contract without learning Core internals.**

| Question | Owner |
| --- | --- |
| What is this developer product? | [Product intent](product/README.md) and [public entry point](../README.md) |
| How does client ownership work? | [Architecture](architecture.md) |
| What is the supported wire and result contract? | [Contracts](contracts/README.md) |
| Which exact operations and types can I use? | [Reference](reference/README.md) |
| How do I connect, act and observe? | [Guides](guides/README.md) |
| How do I build and test independently? | [Development / QA](development/README.md) |
| What has actually passed? | [Qualification](qualification.md) |
| What is selected and what remains open? | [Tasks](TASKS.md) |
| What compatibility can I depend on? | [Releases](releases/README.md) |
| Why this contract design? | [Decisions](decisions/README.md) |

CURRENT means implemented at the stated scope; TARGET describes intended work;
OPEN means unearned capability. Tests, live integration, model execution and human
acceptance are separate evidence classes. Core owns semantic admission; SDK Tasks
do not duplicate Core or Studio project control. Git retains forensic history.

Canonical Markdown uses hidden `docs:metadata` with the common YAI document roles.
This SDK profile uses one owner per question and relative internal links. Exact
contract definitions and checked projections are not independent prose owners.

Product bootstrap workflows project `product.bootstrap.get`,
`product.profile.initialize`, `product.workspace.select` and
`product.entitlement.record`. Entitlement claims remain explicitly unverified;
product account connectivity and YAI membership/Case authority are independent.
The local profile revision fences stale selection or entitlement edits. A repeated
exact selection or retained claim is idempotent. Clients must not infer a login,
license grant or Participant role from these projections.
