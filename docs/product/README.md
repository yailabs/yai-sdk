<!-- docs:metadata
title: SDK Product Intent
id: yai-sdk.product
document: product
status: mixed
owner: sdk
audience: [developer, engineer]
publication: {html: true, pdf: false, index: true}
-->

# SDK Product Intent

**An independently usable developer boundary, not a second YAI engine.**

[Documentation](../README.md)

The SDK serves external client authors and first-party Studio with the same
public contract. A client expresses bounded intent, observes admitted results,
and reacts to invalidations without reading stores or reconstructing authority.

CURRENT: Linux local Rust client, compatibility negotiation, public projections,
capability discovery, failure/indeterminate-delivery distinctions and conformance;
37 typed workflow methods in 11 families, with 118 shared contract types projected
mechanically to Rust and TypeScript. Remaining low-level operations and
TypeScript-only projections remain explicit, not fictitious complete typed coverage.

Neither a convenience method nor a discovered capability promises permission.
The client must handle current disclosure, stale state, unavailable producers and
unsupported instances. The YAI client domain consumes Core-governed semantics.
The independent `yvex-sdk` domain projects YVEX-owned producer contracts; it does
not grant Case authority or acquire Core semantics. Studio may compose both.

Product login initiates ordinary browser account authentication through Core and
observes its automatic local callback, without entered codes or separate device
confirmation. Legacy pairing is compatibility-only. Client DTOs never
contain bearer credentials, installation private keys or signed entitlement bytes.
Account session, verified offline product access and Case authority are separate.

This milestone does not promise network transport, portable OS support, model
quality, calibration, human acceptance or customer-binary distribution readiness.
