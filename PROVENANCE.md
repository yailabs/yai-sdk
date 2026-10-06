# Provenance

Wire, local transport, and projection declarations were separated from YAI commit
1820aec47f6ef6a09a7bb77fa1caadd03cac50d6, specifically application/yai-host/src/lib.rs,
application/yai-application/src/lib.rs and Studio client declarations. The generic
Linux process identity checks originated in engine/yai-engine/src/resource_control.rs;
no resource authority, fences or Case semantics were extracted. Only client
contracts and generic local transport mechanisms are published here. Server,
Case semantics, persistence, admission and scheduler implementations remain Core.
The current first-party rights holder authorizes this supported boundary under MIT;
this does not relicense the proprietary source repository.

## Canonical YVEX client ownership

YVEX owns its MIT public client implementation, transport, generated management
projections and contract descriptors in `yailabs/yvex/sdk/rust`; TypeScript types
are published from `sdk/typescript` as `@yvex/sdk`. This establishes an independent
YVEX client boundary with no YAI semantic or Studio dependency.

The historical `crates/yvex-sdk` 0.1 package is retained as a compatibility
facade over canonical `yvex-sdk` 0.2. Feature flags forward to that dependency;
`@yai/sdk/yvex` reexports the canonical TypeScript types. Removed implementation
and descriptor files must not be recreated here. The lockfiles identify the exact
reviewed dependency; Git records the migration chronology. Local source overrides
used during coordinated qualification are not a publication contract.
