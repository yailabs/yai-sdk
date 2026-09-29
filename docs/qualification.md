<!-- docs:metadata
title: SDK Qualification
id: yai-sdk.qualification
document: evaluation
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# Local client boundary qualification

[Documentation](README.md)

The v0.1.0 SDK is a supported client contract, not an implementation of Core
semantics. This record qualifies Linux same-user local IPC only.

## Platform work in progress

The current platform Task is not yet closed. New evidence obtained on isolated
profiles against Core `1f012d9` / installed executable SHA-256
`bef4f843ffb56efae0200dcf4f034d06f45b779c8fde33eeb974898fc65b0aa8`:

| Authority / input | Expected | Observed | Claim |
| --- | --- | --- | --- |
| Real Core catalog / released SDK inventory | Every released operation keeps input/output, impact and authority identities | 98 exact matches | PASS: released contract identity conformance, not semantics |
| Real typed create/role/link/open / fresh Case | Admitted mutations followed by authorized read | Generations 1→2→3; exact Case/Participant returned | PASS: typed action composition, no fixture provider |
| Real identity + Recall v2 / same Case at generation 3 | Current disclosure and exact cut preserved | Success; cut 3, closure complete, zero selected items and explicit limitations | PASS: typed Recall composition; not retrieval quality |
| Refusal control / unlinked principal | Opening Case must not infer authority | `unauthorized` | PASS: refusal preserved |

The first role-add test revealed a real projection mismatch: this operation returns
`StateMutationReceipt`, not `WorkCommit`. The SDK retained the received successful
envelope, did not replay the mutation, corrected its descriptor and qualified a
new isolated Case. Full Studio and model-chain requalification remains pending.

## Previously qualified extraction scope

| Owner / oracle | Expected | Observed | Result / scope |
| --- | --- | --- | --- |
| Standalone Rust conformance | Negotiate before ordinary calls; refuse stale process, incompatible protocol, wrong correlation and oversized frames | 16 tests pass (4 unit, 12 conformance); post-dispatch disconnect is explicitly indeterminate, with no automatic replay | PASS: client protocol/software contract, not Case semantics |
| Public TypeScript projection compilation | All current Studio projections compile without private source | 127 exported DTOs type-check | PASS: public representation boundary |
| Independent package/version guard | Cargo, npm and lock projections agree; malformed prerelease and drift refuse | Four positive/negative regression tests pass | PASS: independent SemVer authority |
| Private Core server conformance | Real server implements SDK framing, negotiation, catalog and invalidation | 12 Host tests pass, including lost-handshake cleanup and lost-response recovery | PASS: server/client composition |
| Independent Studio native client | Two desktop processes observe the same admitted Case update and reconnect after Host replacement | Case generations 3→4→5; repeat is idempotent; second Case unchanged; closing one window leaves Host and other client alive | PASS: real local product integration |
| Canonical Core producer and SDK projection contracts | Mutation/probe results expose public facts without persisted CaseState, Transition or carrier ownership | Strict receipt decoding and provider projection controls pass; actual Studio consumers use public receipts plus independently disclosed reads | PASS: no private persisted-layout client dependency; no semantic authority moved into SDK |
| Final Studio SDK interaction composition | Preserve qualified read, mutation, refusal, lost-response and observation behavior | All 17 isolated interaction lanes pass against the final Core binary; application approve/deny/defer and exact CLI replay pass 3 independently initialized repetitions | PASS: supported local consumer composition, not provider/model quality |
| Distribution legal negative controls | Missing closure, notices or reciprocal source must refuse | 11 tests pass; actual customer artifact remains UNQUALIFIED | PASS: fail-closed mechanism, not distribution readiness |

Final evidence binds public SDK implementation
`ec019ec4a303a5260c3525f0c8c7e64d68d5607e`, independently cloned Studio source
`f19c6bac00b6868c713efb7d0101c2d8ba7156e8`, and Core source
`89aa2798762b1fc42733e9edb7ab7ef23babec84`, including coherent verification
snapshot repair `383e2c891500161581214da1dde239077870dfe0`.
Native run `native-portfolio-1790621814764630276` used Core binary SHA-256
`bef4f843ffb56efae0200dcf4f034d06f45b779c8fde33eeb974898fc65b0aa8`.
The independently built SDK test client SHA-256 was
`fdc88d0d60bdbb80e2a47a198ae58f0280ee8a47f0c83dba5d63c8aab980b0ad`.
An independent public SDK clone passed Rust, TypeScript, version and legal checks
without private source. These source pins identify evidence, not compatibility;
product SemVer and negotiated protocol/capability identities own compatibility.

The 17 interaction lanes cover application, compute, conversation, effects,
environment, execution, identity, memory, policy, policy intake, resources,
resource setup, source policy, work, exact execution context, two-client
operational invalidation and retained-material preview. They used disposable
profiles and controlled providers/resources, not operator Cases or production
provider requests. An earlier review-verification race was not counted as PASS:
Core repaired the shared read-snapshot owner, then the final binary passed the
three repeated application controls. Core owns the underlying semantic and
store-regression evidence; SDK does not implement or duplicate that mechanism.

No production provider request, model-quality, human acceptance, internet
transport, customer package or security-certification claim follows from these
software and local product checks. Core and Studio retain their own evidence and
project-control authorities; this repository does not mirror their roadmaps.
