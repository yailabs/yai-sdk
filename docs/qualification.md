# Local client boundary qualification

The v0.1.0 SDK is a supported client contract, not an implementation of Core
semantics. This record qualifies Linux same-user local IPC only.

| Owner / oracle | Expected | Observed | Result / scope |
| --- | --- | --- | --- |
| Standalone Rust conformance | Negotiate before ordinary calls; refuse stale process, incompatible protocol, wrong correlation and oversized frames | 14 tests pass; post-dispatch disconnect is explicitly indeterminate, with no automatic replay | PASS: client protocol/software contract, not Case semantics |
| Public TypeScript projection compilation | All current Studio projections compile without private source | 127 exported DTOs type-check | PASS: public representation boundary |
| Independent package/version guard | Cargo, npm and lock projections agree; malformed prerelease and drift refuse | Four positive/negative regression tests pass | PASS: independent SemVer authority |
| Private Core server conformance | Real server implements SDK framing, negotiation, catalog and invalidation | 12 Host tests pass, including lost-handshake cleanup and lost-response recovery | PASS: server/client composition |
| Independent Studio native client | Two desktop processes observe the same admitted Case update and reconnect after Host replacement | Case generations 3→4→5; repeat is idempotent; second Case unchanged; closing one window leaves Host and other client alive | PASS: real local product integration |
| Distribution legal negative controls | Missing closure, notices or reciprocal source must refuse | 11 tests pass; actual customer artifact remains UNQUALIFIED | PASS: fail-closed mechanism, not distribution readiness |

The native repeat used Studio source
`6f28400811550c4281c82a26b7b34d2cf5b01b6c`, public SDK
`6a8c21e85addc82fef6a3b79868ff9d3369d57d1`, and independently installed Core
implementation `29d489e64384a60906403da93d6969f7ee02f256`, whose SDK pin is
`8432986db092b12294210301d9b42f888f281d61`. These compatible pins intentionally
demonstrate that Git equality is not the client contract. The later SDK commit
adds only a disconnect-after-dispatch conformance fixture.

No production provider request, model-quality, human acceptance, internet
transport, customer package or security-certification claim follows from these
software and local product checks. Core and Studio retain their own evidence and
project-control authorities; this repository does not mirror their roadmaps.
