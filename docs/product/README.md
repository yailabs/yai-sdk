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
capability discovery, failure/indeterminate-delivery distinctions and conformance.
TARGET: ordinary supported workflows expressed through ergonomic typed entry
points, with documented and mechanically coherent language projections.

Neither a convenience method nor a discovered capability promises permission.
The client must handle current disclosure, stale state, unavailable producers and
unsupported instances. YVEX remains a computational producer behind Core provider
realization; this library is not a direct YVEX or provider-plugin SDK.

This milestone does not promise network transport, portable OS support, model
quality, calibration, human acceptance or customer-binary distribution readiness.
