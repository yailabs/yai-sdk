<!-- docs:metadata
title: SDK Versions and Releases
id: yai-sdk.releases
document: reference
status: current
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK Versions and Releases

**Independent SemVer; explicit protocol compatibility.**

[Documentation](../README.md)

Root `Cargo.toml` owns SDK product version `0.1.0`; npm metadata and lockfiles are
checked projections. Source commits on main are not release tags or package
publication. Future qualified tags use `vMAJOR.MINOR.PATCH`.

Before 1.0, a breaking public API change requires a minor increment once a release
exists; patches preserve its supported contract. Additive client conveniences do
not invent a new wire version. A wire, event or projection incompatibility must
change its own contract identity and be rejected or explicitly negotiated.

Core reports its independent product version. Client acceptance depends on the
supported wire/Application/catalog identities, not equal product versions or Git
commits. New Core operations can remain absent on an older instance; discovery is
mandatory before typed execution.

The TypeScript package is private-to-publication metadata (`private: true`) with
types-only exports, not a shipped runtime client. Do not claim npm/crates.io
availability without actual publication. Source tests do not qualify customer
packages; [distribution policy](../distribution.md) owns exact legal closure.
