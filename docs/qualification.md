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
semantics. This record qualifies Linux same-user local IPC and bounded macOS
peer-credential queries; full native Host integration remains Linux-only.

## Product management v2 client — 2026-10-06

The client-owned slice of `YVEX.STUDIO.COMPUTE.CONTROL.PLANE.0` adds the separate
explicit `product-management` grant, 36 reviewed operations, generated Rust and
TypeScript domain projections, typed failures, prepared mutation identities and
exact durable Job observation. Management v1 and finite remote computation remain
independent. YAI's 115-operation inventory is unchanged.

Evidence classes remain separate. Rust unit controls qualify input bounds,
Host/Session/acquisition fences, exact reply/job identity, revoked/refused versus
unavailable outcomes, sensitive-diagnostic redaction and nullable observations.
The isolated SSH lane qualifies one dispatch per prepared invocation, synthetic
receipt observation after reply loss and wrong-key/pin rejection. It runs no
YVEX model and establishes no runtime performance, Spark connectivity or native
Studio product acceptance. Cross-repository producer/schema/disposition parity
is necessary in addition to the standalone checks.


## Product operational consumer slices

### Remote finite client checkpoint 2026-10-06

Producer authority is YVEX `803dd98d4c54d7a26cb3c08def6b350be51b7179`,
`finite.decision.execute`, `yvex.finite.request.v1` / `yvex.finite.response.v1`.
Closed decoding checks exact correlation, both SSH peer identities, alias,
resident generation, six admitted computational-lineage identities, ordered
candidate IDs, raw/uncalibrated score semantics and finite work counters.
Default SDK builds exercise this client without a C archive; local native absence
still refuses and is not silently converted into remote dispatch.

`cargo test --locked -p yvex-sdk --all-targets` passes 18 Rust tests including
wire, identity, population, request-byte and score negatives, early cancellation,
blocked stdin timeout, bounded oversized output and concurrent large-response
draining. `make check-finite-remote` passes 15 real-SSH synthetic protocol controls
plus wrong-host-pin and unenrolled-key refusal. Its exact audit observes one
dispatch per invocation and fresh request IDs, including reply loss and timeout:
no retry occurs. Cancellation after start and producer errors preserve unknown
computational outcome; they do not assert producer cancellation or zero work.
The fixture runs no YVEX process/model and grants no YAI authority.

Standalone negative delta tests refuse a missing/changed finite entry/schema,
management-scope contamination and runtime/retry promotion. Cross-repository
parity separately checks published request/response checksums; a changed layout
requires explicit reviewed client disposition. The two management reads and 115
YAI operations remain separate/unchanged. No new Studio implementation is claimed.

Actual approved DGX listener, YAI key/grant and exact resident finite engine are
not yet supplied to this consumer. Real SDK→YVEX model execution, YAI qualification,
Participant/current-W Fast Search, installed product result and latency/resource
claims remain unearned. Synthetic counters are fixture inputs, not measurements.
No search-quality, Human, Canary, release or commercial promotion follows.

Local-development/profile projections add four operations (115 released total).
Closed DTO and workflow tests cover source distinction, absent commercial profile,
null unknown facts, credential-field rejection and generator wrapper/type collision.
These are standalone client conformance, not development-mode enforcement or
remote profile evidence. Core's dual-build/isolated Host qualification owns enable,
ordinary-build refusal, restart, disable and retained authority. Studio must add its
own explicit dispositions and consume the published truth; SDK tests do not qualify
that UI or the missing native commercial account-profile endpoint.

Browser activation projects five compatibility operations without credentials or
Supabase objects: the released YAI inventory is 111 operations. Pending,
indeterminate, closed-input and identity controls pass independently of a real
commercial issuer or Studio login journey. Core's isolated native qualification
owns the actual vault/Application/CLI proof; SDK does not infer entitlement or
Case authority from these DTOs.

Three native-login projections add typed start/observe/cancel and automatic
callback observation without exporting PKCE/state/code/token material. Standalone
Rust tests, all-target workspace checks, generated workflow negative controls,
TypeScript and documentation/package/legal guards pass. Closed-input/output
tests distinguish account authentication from unsupported entitlement policy.
These are client/conformance claims only: web `841b76a` remains a gated candidate;
real browser/account/Home and other-OS credential runtime evidence are not inferred.

Finite client conformance uses the exact public headers at YVEX `67a7905` and an
SDK-authored C fixture, not YVEX inference. Default standalone tests cover byte
bounds, identity/score negatives and native-client absence. The explicit native
lane compiles generated bindings plus C layout assertions, preserves exact
candidate/generation identity and refuses stale/foreign/non-finite/calibrated/
generated/oversized/unterminated/error responses. A same-size declaration change
and nested candidate/status changes plus a limit delta fail closed. Ordinary
`make check` covers both workspace domains: 43 Rust tests, generator, TypeScript,
documentation and package/legal controls pass. The explicit native lane passes
8 tests plus its C-client result/drift controls. The fixture has no host, weights
or computational capability; its counters are test data, not runtime evidence.

Real finite C-client installation, exact CPU/model inference, Participant-step
integration and production Fast Search remain unqualified here. The current
producer executable package does not install the required native client archive
and headers. No private wire fallback or producer mutation is introduced.
No Human/Canary/commercial distribution or model-quality claim follows.

The later platform-parity lane compares current owner-published inventories
across YAI, this SDK, Studio and YVEX with
`tools/check_platform_parity.py`. Its bounded source run matched 98 YAI
Application operations and 2 YVEX remote management operations; controlled
YAI contract and YVEX operation deltas both refused. Studio's two YVEX
read-only dispositions have controlled UI evidence, and the YVEX SDK has an
isolated real SSH fixture. These results do not establish a deployed YVEX
management endpoint, runtime compatibility or real Case model completion.

## SDK platform qualification

YAI.SDK.PLATFORM.0 is complete at the Linux local client scope. Real model-chain
completion is **BLOCKED** at the external producer, not counted as integration
success. Initial typed workflow evidence was obtained on isolated
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
new isolated Case. Final Core/Studio consumer checks below use the corrected
published SDK implementation `47707a2c472e564b94b888edf064cf873c6a435a`.
Consumer commits are Core `e091b9e4ff94549b625381d0f28ed85d91da5fe6`
and Studio `b33fb51d5b1706d154fb58de07553661487089d6`. Both pin that exact SDK
implementation. Subsequent SDK example/test/evidence updates do not change its
library or wire contract and do not require matching repository HEADs.

### Final software and product evidence

| Test / lane | Authority / oracle | Input / fixture | Expected | Observed | Metric / tolerance | Result | Claim supported |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SDK contracts and typed client | Published protocol and workflow descriptor | Positive, refusal, malformed, replacement and lost-response controls | Preserve identity, classify failure, never replay uncertain dispatch | 26 Rust tests pass; all six examples compile | Exact software predicates; numerical tolerance N/A | PASS | Client behavior, not Core semantics |
| Contract projections | Canonical operations/workflows descriptors | Rust/TypeScript generated projections and stale/invalid descriptor controls | One common owner; drift refuses | 79 shared types, 24 typed operations, 8 generator controls pass; strict TypeScript compiles | Exact regeneration | PASS | Shared projections cannot silently diverge; TS remains types-only |
| Package/legal controls | Independent SemVer and distribution.legal.v1 | Version drift, unknown closure, absent notices/source | Invalid package refuses | 4 version and 11 legal controls pass | Exact refusal | PASS | Guard behavior, not customer distribution readiness |
| Core conformance | Canonical Application/Host implementation | 98 released descriptors, 16 typed input shapes, real store/Host tests | Same operation identities and admitted semantics | 82 tests pass: Application 34+3+33, Host 12 | Exact contracts/lifecycle | PASS | Core implements the released boundary; no semantic owner moved |
| Native two-client integration | Real installed Core and two Tauri/WebKit processes | Isolated Cases; mutation, duplicate submission, Host replacement, window close | Same admitted state; no duplicate mutation; resynchronize replacement | Generations 3→4→5; repeat idempotent; other Case unchanged; both resync; closing one leaves Host/peer alive | Exact identities and generations | PASS | Real Studio→SDK→Core lifecycle, not a model response |
| Studio interaction composition | Existing admitted Application contracts | 17 isolated interaction lanes | Preserve read/action/refusal/context/material behavior | All 17 pass; kernel 30 tests and frontend/native builds pass | Exact assertions, byte-equal retained materials | PASS | Supported client behavior; controlled providers are not model evidence |
| Independent public consumer | Public Git clone, no private source | SDK implementation 47707a2 | Build/test/docs without Core or Studio | make check, all-target examples and cargo doc pass; clean checkout | Exact package identities | PASS | Standalone consumer repository |
| Real model addressing | Core provider discovery and typed SDK compute.models | Registered real YVEX target | Preserve exact model identity without inferring execution readiness | Catalog/health HTTP 200; 32768 capacity; execution_or_resources_qualified=false | Exact identity; capacity not latency | PASS | YAI-mediated discovery only |
| Real inference prerequisite | Core's public provider qualifier; YVEX HTTP result | One fresh probe ID, text/JSON controls; one bounded diagnostic | Producer must complete before admitting a real Case execution | Text and JSON HTTP 503; diagnostic runtime_unavailable, CUDA MoE layer 24 status 1 | No numerical result available | BLOCKED | External producer cannot presently qualify ChatText; full model chain NOT established |

The table retains the earlier exact run. A separate YVEX-side reverse-chain
handoff on 2026-09-29 reports a newer synthetic HTTP 503 with CUDA MoE layer 30,
device status 1. It is not a rerun of this SDK qualification lane, does not
identify the first invalid numeric operation and supplies no completed governed
Case result. Contract parity remains PASS independently of this producer gate.

Native run `native-portfolio-1790682024243503055` binds Core executable SHA-256
`3c0e835f8906cb07b6ea085b34c315c36027ea7176f751d974c695df5ef84bd9`,
Studio executable `89f2171e9202e5ddfccd6cb4801fb169eecb962557631b9a3daeb92be26da8f6`
and SDK test client `c4c662c7a20074836689ff8f49abc5163143e55e06c027510fff0c6331712636`.
The 17 lanes cover application, compute, conversation, effects, environment,
execution, identity, memory, policy, policy intake, resources, resource setup,
source policy, work, execution context, operational live reads and material preview.
The operational-live run observed two attachments and zero events: mutation
fanout is proved by the native run, not inferred from that read-only lane.
Context run `context-capacity-1790682351273023831` uses a controlled HTTP provider.
Material preview uses two independently acquired 87-byte Markdown fixtures, not
private source dependencies. Initial missing fixture configuration was corrected
before the context lane passed; no failed setup is reported as successful.

Reproduction owners: SDK `make check` and `cargo doc --no-deps`; Core
`cargo test --manifest-path application/Cargo.toml -p yai-host -p yai-application --locked`,
`make check-docs test-roadmap` and `python3 tests/test_product_boundary.py`; Studio
`npm run check`, `npm run test:kernel`, `npm run build`,
`npm run desktop:build -- -- --locked` and its existing `tests/studio/` harnesses.
Native harness `tests/studio/native-portfolio.py` takes explicit installed Core,
Studio and public SDK client binaries and disposable profiles. Evidence and build
products remain outside Git. Human acceptance, Canary, full Core Golden and
customer package qualification were not run for this SDK milestone.

### External producer limit

Fresh probe `probe:sdk-platform-20260929-01` on target
`provider-target:6369eba7d4b03ea9737a3bd69719720c` retained qualification
`provider-qualification:89b50a4bcbf8b6913f99f2f257b35360` in an isolated Core profile.
Catalog/health completed; text returned HTTP 503 after 10585 ms and JSON after
7093 ms. Only model addressing/health qualified, not ChatText. A fresh bounded
`Reply OK.` diagnostic (8 output tokens maximum) returned:

```json
{"error":{"message":"deferred CUDA MoE layer 24 reported device status 1","type":"server_error","param":null,"code":"runtime_unavailable"}}
```

Exact model:
`deepseek4-v4-flash-dspark-deepseek-v4-flash-mixed-iq2xxs-q2k-mxfp4-v1-cuda`,
engine generation 1; artifact
`b669d80726cf83331c0d8016debbde44cf965a1503c33f605e92ea4e550ee87f`, binding
`8cdb4929c523bd42e3fb82fa18ceed0a0a6732d6efd1d852c88398c8d2d6cd5e`.
The live YVEX process and loaded model were not restarted or modified. No Case
SEND was forced past failed provider qualification and no historical indeterminate
request was replayed. There is no completed model result, canonical Case result or
Studio model-result presentation to claim. Repair/qualification of that producer
is outside the SDK boundary. Discovery success is not inference support.

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

## Product bootstrap and Workspace catalog

The product wave adds four typed workflows and a Workspace filter to Case list
and recent reads: 103 operation descriptors, 99 shared types, 29 typed workflows.
Standalone Rust tests, TypeScript checks, generated workflow verification and the
private-boundary guard pass. The built `examples/conformance` consumer validates
all 103 descriptors against the actual updated Core Host in an isolated temporary
installation (`/tmp/yai-product-live-sdk-conformance.log`).

Core owns the separate product profile/claim store and membership checks. CLI
qualification exercises fresh/init/select, unauthorized and stale refusal, claim
import/removal and restart through the public SDK. Studio's actual native desktop
qualifies first-run, Workspace selection/switching, Host replacement and retained
Case/draft continuity (`/tmp/studio-native-product-final/result.json`). This is local
consumer proof, not production account authentication or signed-license validity.
No account service, issuer, trust-root distribution or payment producer exists.

## macOS peer credentials 2026-10-01

Bounded repair of the SDK compilation failure exposed by Studio's pinned
`22feee68` dependency: `libc::ucred` and `SO_PEERCRED` are Linux APIs.
The published-main baseline is `506a2ed86d10ce7deaab1851931fa85b0bdab2b5`.
The isolated Exon snapshot `e0e2323005c49480514b571346c8638c1ab0be1c` has the
same affected transport source; its three unpublished commits were inspected
and tested in a separate copy and are not included in this repair.

On macOS arm64 26.6.2, `cargo check --locked --workspace --all-targets` and
`cargo test --locked --test native_peer` pass (5 tests). Native controls check
the actual UID/PID of a socket pair, the distinct connecting child's PID on
an accepted Unix socket, rejection of an IPv4 descriptor, and explicit refusal
of the unported process/start identity. The helper entry is a subprocess
fixture, not a separate capability claim. No credentials are mocked.

An isolated copy of local Studio `7074c31`, preserving its local case-sensitive
module repair, compiles with the candidate through a temporary Cargo path
override. This qualifies resolution of the compilation failure, not Studio
release or full native runtime integration. Linux `cargo test --locked
--workspace` passes all 38 tests, including the 4 applicable native peer
controls. `npm ci && npm run check`, `python3 tools/check.py`, package identity
and workflow contract controls pass. The public wire/product versions remain
unchanged; no release tag is created.

The full macOS SDK suite is not green: an existing Linux executable test
requires `/usr/bin/sleep`, absent on this Mac, and process identity remains
Linux-only. The isolated Studio native suite reports 8 pass, 1 ignored,
1 failure at its existing `less` alternate-screen expectation. These broader
boundaries are retained and are not suppressed to claim native qualification.

## Native network management — 2026-10-06

The SDK projects producer `network-management.md` identity/pairing schemas and
preserves the 36 product operation meanings. Generated TypeScript contains safe
connection/profile/discovery records only. Rust owns TLS, native credentials and
protected local companion transport. Standalone loopback tests prove exact pin
verification before bearer dispatch, body/framing bounds, redirect refusal,
redacted errors, and revocation distinct from transport failure.

`YVEX_SDK_TEST_NATIVE_CREDENTIALS=1 python3 tests/test_network_native.py` passed
against a disposable real producer listener and the actual desktop Secret
Service. Fresh processes restored one uniquely generated test credential;
approved HTTPS and same-user UDS observed 36 operations; exact job receipt
recovery, revocation, outage and explicit new pairing identity passed. Generated
credentials were deleted. No operator credentials were enumerated or reused.

Producer `tests/integration/product_management_sessions.py` also passed with
`YVEX_SESSION_MANAGEMENT_TRANSPORT=https-sdk`, `YVEX_SDK_CONNECTIONS_EXAMPLE` set
to the built `connections` example, and explicit `YVEX_TEST_FIXTURE_CAPACITY=1`.
This used a tiny compiled CPU model and actual native-vault credential: Engine
load, Session create, one-token generation, fork/reset/close/unload, exact lost
acknowledgement recovery with one Turn, stale Session lifetime and restarted Host
fences all passed. Idle cancellation truthfully failed; active cancellation is a
separate producer-native control. The declared 128 GiB admission fixture is not
physical memory evidence. No Spark, real model quality, YAI Case execution or
human acceptance is inferred.

DNS-SD client browsing is bounded to ten seconds/64 hints; advertised labels and
fingerprints do not establish trust. Interface-scoped link-local IPv6 addresses
are omitted rather than losing scope. Real LAN discovery and operator deployment
remain distinct qualification exits. Shared safe metadata and credentials are
implemented through the SDK; YAI CLI adoption remains with its concurrent owner.

Native Studio HTTPS/local qualification exposed transient profile-registry
contention between independent snapshot reads. SDK metadata reads now share an
OS read lock; exclusive updates and contended reads wait at most one second on
the native worker. Three regression controls pass: twelve parallel get/list
readers while another reader holds the registry, a short exclusive writer, and a
finite stalled-lock refusal. This waits only for local metadata access; no
pairing, management submission or lost-response retry occurs.
