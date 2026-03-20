# SLICE-004-runtime-context-and-input-fingerprint-model

## Objective

Make the native-owned `RuntimeContext` record and stable runtime-input fingerprint contract real, using the thin managed shell only to surface already-observable bootstrap and runtime facts into Rust without turning C# into a generalized package, modset, or world-model owner.

This slice establishes the runtime-input side of the Stage 2 authority spine and provides the stable basis that later slices will extend for generalized package identity, deterministic modset modeling, and raw XML discovery.

## Milestone

Stage 2

## Allowed paths

- native/src/**
- native/tests/**
- native/Cargo.toml
- managed/src/Bootstrap/**
- managed/src/Boundary/**
- managed/src/Diagnostics/**
- managed/src/Interop/**
- managed/src/RustyStartup.Managed.csproj
- evals/**
- docs/context/Current Build Context.md
- roadmap/active_slice.yaml

## Forbidden paths

- everything not listed above
- docs/authority/**
- docs/architecture/**
- roadmap/roadmap.yaml
- snapshot/**
- equivalence/**
- benchmark code
- any generalized multi-mod package resolution, deterministic modset/content-root modeling, raw XML discovery, patch application, replay, equivalence, or mixed-zone semantic ownership implementation beyond the explicit runtime-input contract needed to hand native-owned context across the bootstrap boundary

## Invariants

- The Rust core is the canonical owner of `RuntimeContext` and runtime-input fingerprint semantics.
- The managed shell may capture and pass runtime facts that are already observable at bootstrap, but it must not become the owner of generalized package identity, mod identity, modset discovery, content-root selection, or world-model truth.
- This slice may normalize and fingerprint runtime build identity, parser mode, selected language key, platform or architecture basis, machine-local environment keys, and bootstrap-proven self-package inputs already available from `SLICE-002-package-root-load-folder-and-layout-resolution` and `SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap`.
- This slice consumes the stable self-package layout outputs from Stage 1 and must not reopen self-package layout resolution as a new ownership boundary.
- Fresh runtime evidence is authoritative for runtime build identity; static install metadata such as `Version.txt` is secondary fingerprint input only and must not override fresh runtime evidence when they disagree.
- Machine-local runtime or environment keys must remain distinct from portable or package-derived inputs; the fingerprint model must not blur them into one opaque hash.
- Machine-local roots consumed by `RuntimeContext` must normalize to stable path-identity values rather than ad hoc display strings.
- Unknown, unsupported, or unavailable runtime inputs must be explicit and diagnostic, not silently defaulted into false parity claims.
- Legacy parser mode behind `legacy-xml-deserializer` must be explicitly fingerprinted and surfaced as a non-primary-parity or degraded runtime input, even where full parser-lane handling is deferred to later slices.
- This slice may define stable extension points for later Stage 2 slices, but it must not smuggle future mod/package, XML, scheduler, snapshot, replay, or equivalence semantics into unstructured catch-all fields.

## Required diagnostics

- authoritative runtime build identity and evidence source, including when static install metadata is secondary
- runtime version or revision basis and source
- parser mode and parser-mode basis
- selected language key and selection source
- platform, OS, and architecture basis
- selected self-package root and active self content root as consumed bootstrap inputs, with source
- normalized machine-local path-identity inputs used by `RuntimeContext`
- runtime-input fingerprint classes present for the current run
- normalized per-class status, including unknown, unavailable, unsupported, degraded, or secondary-authority reasons
- canonical runtime-context fingerprint result, or an explicit reason it could not be produced

## Required evidence

- a native-owned `RuntimeContext` type or module exists and is fed through an explicit bootstrap contract rather than implicit globals or scattered static state
- runtime build identity, parser mode, selected language key, platform or architecture basis, machine-local environment keys, and bootstrap-proven self-package inputs normalize deterministically for the current machine
- fresh runtime evidence is preferred over static install metadata when both are present
- fingerprinting distinguishes machine-local runtime keys from portable or package-derived inputs rather than collapsing them into one opaque value
- selected self-package root and active self content root normalize deterministically as path-identity inputs
- missing or unsupported runtime inputs are surfaced honestly in diagnostics and fingerprint status
- legacy parser mode is visibly distinguishable from the default primary-parity parser lane in diagnostics and fingerprint status
- at least one proof surface inside `native/tests/**` or `evals/**` exercises normalization and fingerprint behavior for both stable and failure cases
- the build context and active-slice metadata move cleanly from `SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap` to this slice
- no generalized multi-mod package resolution, deterministic modset modeling, raw XML discovery, snapshot storage, replay, equivalence, or mixed-zone semantic ownership is introduced by this slice

## Evidence execution rule

This slice explicitly authorizes proof surfaces under `native/tests/**` and `evals/**`.

Formal completion requires a repo-local proof surface for runtime-context normalization and fingerprint behavior. Do not treat model code or bootstrap handoff code alone as evidence-complete closure.

## Exit criteria

- the Rust core owns a stable `RuntimeContext` record that can represent the current machine's startup-relevant runtime inputs without consulting ad hoc managed globals
- the bootstrap boundary can hand runtime inputs into Rust through an explicit contract that remains narrow and explainable
- the runtime-input fingerprint model is stable enough for later mod identity, package resolution, deterministic modset, and raw XML discovery slices to extend without reopening bootstrap or ownership boundaries
- diagnostics explain which runtime-input classes were observed, normalized, fingerprinted, degraded, unknown, unavailable, unsupported, or secondary-authority
- `SLICE-005-mod-identity-and-package-resolution-model` can begin from a stable runtime-context and fingerprint basis rather than inventing its own bootstrap truth

## Non-goals

- no generalized multi-mod package identity or package resolution
- no deterministic modset or content-root model
- no raw XML asset discovery
- no snapshot manifest storage or replay logic
- no equivalence logic
- no mixed-zone bridge or scheduler/reducer implementation