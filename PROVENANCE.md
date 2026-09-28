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
