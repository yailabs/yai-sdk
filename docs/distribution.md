# Distribution qualification

First-party SDK material is MIT, copyright Francesco Maiomascio where authored
and owned. The public SDK grant does not relicense proprietary Core or Studio.
Dependencies retain their original terms; Git references are not bundled code.

The inherited `distribution.legal.v1` tool binds an actual payload, source/tree,
build inputs and reviewed shipped-component closure. It requires full original
notices and license texts, and corresponding sources for any reciprocal covered
material actually shipped. Unknown membership, missing notice or unresolved
corresponding source fails closed. Source availability is not inferred from a
successful compile. The tool itself retains its MIT provenance from the shared
pre-incorporation legal-readiness delivery.

No SDK binary/customer package has been qualified by this repository migration.
The `sdk` policy profile is explicitly unresolved pending its exact artifact
closure. `make qualify-distribution DISTRIBUTION_PACKAGE=...` enforces the policy;
ordinary `cargo build` and TypeScript checks do not grant release readiness.

The current SDK has no direct MPL dependency. This is not a claim about a future
consumer executable's complete static/toolchain closure. Studio's Tauri/frontend
and MPL source obligations remain Studio-owned; Core and Studio are separate
proprietary products, not material included in this SDK source package.
