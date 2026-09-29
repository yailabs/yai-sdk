<!-- docs:metadata
title: SDK API Reference
id: yai-sdk.reference
document: reference
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK API Reference

**Exact public names and their one contract owner.**

[Documentation](../README.md)

| Surface | Canonical definition | Projection / consumer |
| --- | --- | --- |
| Wire envelopes, compatibility and event identities | [Rust wire](../../src/wire.rs), [contracts](../../src/contracts.rs) | Functional Host client and public conformance |
| Published operation meanings and contract IDs | [Operation inventory](../../contract/operations.json) | Connected-Core compatibility check; no semantic implementation |
| Typed common requests/results and methods | [Workflow contract](../../contract/workflows.json) | Generated Rust `workflows`, TypeScript `workflows.ts` |
| Remaining TypeScript-only projections | [Public DTOs](../../typescript/projections.ts) | Studio type consumers; no JS runtime transport |
| Communication and orchestration | [Client](../../src/client.rs) | `Client<T>`, `BoundLocalTransport`, typed `Response<T>` |

Run `cargo doc --no-deps` for the Rust symbol reference. Typed domain accessors
select published operations, not a new semantic namespace. The low-level
`OperationRequest` path remains available for the complete published inventory.

Current typed families are `identity`, `cases`, `materials`, `knowledge`,
`memory`, `authority`, `work`, `handoff`, `resources`, `compute` and `conversation`.
They deliberately cover bounded operations, not every combination admitted by
Core. Recall's convenience input selects a generation cut; its typed selection
summary does not flatten the full returned trace, available in `Response.raw`.
Conversation convenience sends text with optional focused context; other media
remain on the existing published low-level contracts. Compute calls YAI's
registered-target model inventory; it never calls a provider directly.

The workflow descriptor format distinguishes strings, booleans, unsigned integer
domains, arrays, named records, omitted optional fields and nullable values.
`u64` is a JSON integer; JavaScript `number` cannot exactly represent every u64.
TypeScript is a type-only projection, not a promise that arbitrary integer
identities can safely pass through JavaScript arithmetic. Do not round identity
or generation fields; a future runtime client needs a qualified integer policy.

`python3 tools/workflows.py --check` rejects projection drift and operation-schema
disagreement. Edit the contract, not generated Rust or TypeScript.
