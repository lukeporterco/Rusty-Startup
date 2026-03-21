# SLICE-005-mod-identity-and-package-resolution-model

## Objective

Make the native-owned `ModIdentity` model and generalized package-resolution contract real, using the managed shell only to surface already-observable package-discovery environment inputs into Rust when the bootstrap boundary must do so, without turning C# into the owner of package identity, active mod selection, or world-model truth.

This slice establishes the package-identity side of the Stage 2 authority spine and provides the stable basis that later slices will extend for deterministic modset modeling, selected content-root resolution, and raw XML discovery.

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
- any deterministic active-mod/content-root modeling, raw XML discovery, patch application, replay, equivalence, or mixed-zone semantic ownership implementation beyond the explicit package-discovery input contract needed to hand native-owned package-resolution context across the bootstrap boundary

## Invariants

- The Rust core is the canonical owner of generalized package identity and package-resolution semantics.
- The managed shell may surface package-search or discovery-environment facts that are already observable at bootstrap, but it must not become the owner of package identity, mod identity, active mod selection, selected content roots, or world-model truth.
- `rustystartup.core` remains the locked shipping identity; `lukep.rustystartup` is historical debris only and must be surfaced only as stale-state evidence when encountered.
- This slice may resolve canonical package roots, `About/About.xml` metadata, package origins, and identity collisions for discoverable packages, but it must not yet commit deterministic active-mod ordering or per-mod selected content roots for the run.
- Canonical package identity must come from discoverable package metadata plus auditable root provenance, not from hardcoded machine paths, folder names alone, or Workshop item IDs alone.
- Workshop numeric item roots and `About/PublishedFileId.txt` are provenance evidence, not standalone semantic identity.
- Duplicate or nested `About/About.xml` trees, duplicate package IDs, missing package IDs, and ambiguous root cases must be explicit and diagnostic, not silently collapsed into false parity.
- This slice consumes the stable self-package layout outputs from `SLICE-002-package-root-load-folder-and-layout-resolution` and the stable runtime-context and fingerprint basis from `SLICE-004-runtime-context-and-input-fingerprint-model` without reopening those ownership boundaries.
- This slice may define stable extension points for later deterministic modset and content-root work, but it must not smuggle raw XML discovery, snapshot, replay, equivalence, or scheduler semantics into unstructured catch-all fields.

## Required diagnostics

- package search roots or discovery-environment basis consumed by native resolution
- discovered package root and root detection source
- canonical package ID and player-facing or display-name basis when present
- package origin classification such as self, official, local mod, workshop mod, or unknown/ambiguous
- `About/About.xml` presence, parse status, and metadata source
- `About/PublishedFileId.txt` presence/status and source when applicable
- dependency/load-before/load-after metadata presence and parse status, without yet turning it into final deterministic ordering
- duplicate, nested, stale, missing, or conflicting package identity/root findings and resolution/failure status
- emitted canonical `ModIdentity` or package-resolution result, or an explicit reason it could not be produced

## Required evidence

- a native-owned `ModIdentity` type or equivalent package identity module exists and is fed through explicit discovery inputs rather than scattered static state
- generalized package resolution can distinguish self, official, local, and workshop package origins for discoverable package roots on the current machine
- canonical package roots normalize deterministically as path-identity values and attach to stable package identity records
- `About/About.xml` metadata, package IDs, and dependency/load-order metadata normalize deterministically for supported packages
- duplicate package IDs, duplicate or nested `About/About.xml` trees, missing metadata, and stale historical `lukep.rustystartup` identity are surfaced honestly in diagnostics
- workshop-style numeric roots and `PublishedFileId` evidence are represented as provenance rather than mistaken for canonical semantic identity
- at least one proof surface inside `native/tests/**` or `evals/**` exercises stable, ambiguous, and failure cases for package identity and package resolution
- the build context and active-slice metadata move cleanly from `SLICE-004-runtime-context-and-input-fingerprint-model` to this slice
- no deterministic modset/content-root model, raw XML discovery, snapshot storage, replay, equivalence, or mixed-zone semantic ownership is introduced by this slice

## Exit criteria

- the Rust core owns a stable `ModIdentity` or equivalent package identity record that can represent discoverable packages without consulting ad hoc managed globals
- package resolution can enumerate and normalize canonical package roots and package identity metadata for discoverable self, official, local, and workshop packages on the current machine
- duplicate or ambiguous package identities and package roots are explainable and bounded by diagnostics rather than silently collapsed
- the bootstrap boundary can hand only the narrow package-discovery inputs needed by Rust when direct native ownership would otherwise lack environment truth
- stable package identity and package-resolution outputs are available for `SLICE-006-deterministic-modset-and-content-root-model` to consume without inventing their own package truth

## Non-goals

- no deterministic active modset or selected content-root model
- no raw XML asset discovery or patch ordering
- no snapshot storage, replay logic, or equivalence logic
- no mixed-zone bridge or scheduler/reducer implementation
- no generalized parser-lane ownership changes
- no reopening of Rusty Startup self-package layout resolution already settled in `SLICE-002-package-root-load-folder-and-layout-resolution`