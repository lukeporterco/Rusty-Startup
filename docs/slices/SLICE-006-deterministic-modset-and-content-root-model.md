# SLICE-006-deterministic-modset-and-content-root-model

## Objective

Make the native-owned deterministic `R0` modset model real, producing the canonical active mod set, exact mod order, and selected content root per active package from stable runtime-context and package-resolution inputs, using the managed shell only to surface already-observable active-mod selection facts when the bootstrap boundary must do so, without turning C# into the owner of active mod selection, content-root semantics, or world-model truth.

This slice establishes the deterministic modset side of the Stage 2 authority spine and provides the stable basis that later slices will extend for raw XML asset discovery and scheduler-bound deterministic phase planning.

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
- any raw XML asset discovery, patch application, unified XML, parser-lane handling, inheritance resolution, scheduler/reducer implementation, replay, equivalence, or mixed-zone semantic ownership beyond the explicit active-mod selection and content-root contract needed to hand native-owned `R0` truth across the bootstrap boundary

## Invariants

- The Rust core is the canonical owner of deterministic modset semantics, ordered package IDs, selected content roots per active package, and `R0` truth.
- The managed shell may surface already-observable active-mod configuration facts, but it must not become the owner of modset modeling, selected content roots, dependency/load-order semantics, or world-model truth.
- This slice consumes the stable runtime-context and runtime-input fingerprint basis from `SLICE-004-runtime-context-and-input-fingerprint-model` and the stable package identity and package-resolution outputs from `SLICE-005-mod-identity-and-package-resolution-model` without reopening those ownership boundaries.
- Canonical active mod records must be derived from canonical package identities plus an auditable active-mod order basis and auditable load-folder rules, not from folder names alone, filesystem enumeration order, display names alone, or ad hoc managed globals.
- Deterministic active order must preserve the supplied active-mod order basis when that basis is valid. Dependency, `loadBefore`, and `loadAfter` metadata may validate, annotate, or bound the model here, but this slice must not silently invent a new ordering policy that diverges from the observed active set without diagnostics.
- `LoadFolders.xml` version choice, fallback behavior, and conditional gates such as `IfModActive`, `IfModActiveAll`, and `IfModNotActive` must influence selected content-root resolution honestly and diagnostically.
- Official, self, local, and workshop package-origin truth remains the package-resolution output from `SLICE-005-mod-identity-and-package-resolution-model`; this slice must not recompute origin or canonical identity through a second authority path.
- Duplicate package identities, missing active package matches, unsupported or ambiguous content-root gates, stale historical package IDs, and development-only layout assumptions must remain explicit and diagnostic rather than silently collapsed into false parity.
- This slice may define stable extension points for `SLICE-007-raw-xml-asset-discovery-and-r1-model` to consume, but it must not itself enumerate raw XML assets or patch files.

## Frozen bootstrap input contract

This slice may consume only the following startup inputs across the bootstrap boundary:

- stable runtime version or build basis from the `RuntimeContext` contract established by `SLICE-004-runtime-context-and-input-fingerprint-model`
- stable generalized package-resolution outputs from `SLICE-005-mod-identity-and-package-resolution-model`
- ordered active-mod selection basis from `Verse.ModsConfig.ActiveModsInLoadOrder` or an explicitly equivalent already-observable runtime source
- explicit source labels for the active-mod selection basis and any per-entry identity hint needed to bind active entries to canonical package identities when package IDs alone are ambiguous

No raw XML discovery, parser-lane, snapshot, replay, equivalence, scheduler, reducer, or mixed-zone execution inputs may be introduced in this slice.

## Frozen stable modset output

Later slices must consume active modset and selected content-root resolution through one stable result object. That output must carry at minimum:

- ordered package IDs for the run
- active-mod entries bound to canonical `ModIdentity` records or explicitly equivalent package identity outputs
- selected content root per active package
- content-root decision basis per active package
- load-folder gate status per active package, or an explicit note that no conditional gates were involved
- dependency, `loadBefore`, and `loadAfter` metadata presence and status attached to each active package
- explicit failure reason when canonical active-mod binding or selected content-root resolution cannot be proved

Do not scatter these outputs across unrelated helpers or diagnostics-only state.

## Required diagnostics

- active-mod selection basis and source
- canonical ordered package IDs emitted for the run
- per-entry mapping from active selection entry to canonical package identity, package root, and origin
- selected content root per active package and the decision basis for that selection
- `LoadFolders.xml` version choice, fallback status, and any conditional gate evaluation such as `IfModActive`, `IfModActiveAll`, and `IfModNotActive`
- dependency, `loadBefore`, and `loadAfter` metadata presence and any validation status relevant to the active modset model
- missing, duplicate, ambiguous, unsupported, inactive, or stale package-match findings and resolution or failure status
- local-development versus discoverable package-root basis when that distinction matters to the selection result
- emitted canonical modset result, or an explicit reason it could not be produced

## Required evidence

- a native-owned `ModsetState`, active-mod record, or equivalent deterministic modset module exists and is fed by explicit package-resolution and runtime inputs rather than scattered static state
- generalized active-mod resolution can bind the ordered active selection basis to canonical package identities discovered under self, official, local, and workshop roots
- selected content roots resolve deterministically for active packages under root-only, version-folder, and `LoadFolders.xml`-routed cases on the current machine
- conditional load-folder gates that depend on the active modset are handled honestly for supported cases and produce explicit diagnostics for unsupported or ambiguous cases
- ordered package IDs and selected content roots are stable across equivalent inputs and do not drift on non-semantic wording or source-label changes alone
- at least one proof surface inside `native/tests/**` or `evals/**` exercises stable, conflicting, missing, and gate-driven active-modset and content-root cases
- the build context and active-slice metadata move cleanly from no active slice after `SLICE-005-mod-identity-and-package-resolution-model` to active `SLICE-006-deterministic-modset-and-content-root-model`
- no raw XML discovery, patch application, snapshot storage, replay, equivalence, scheduler/reducer, or mixed-zone semantic ownership is introduced by this slice

## Evidence execution rule

This slice explicitly authorizes proof surfaces under `native/tests/**` and `evals/**`.

Formal completion requires a repo-local proof surface for deterministic active-mod ordering and selected content-root resolution. Do not treat model code or bootstrap handoff code alone as evidence-complete closure.

## Exit criteria

- the Rust core owns a stable `ModsetState` or equivalent record that represents the active mod set, exact order, canonical identity binding, and selected content root per active package without consulting ad hoc managed globals
- active-mod selection input can be handed across the bootstrap boundary through a narrow, explicit, and explainable contract
- selected content-root decisions are deterministic, auditable, and bounded by diagnostics for self, official, local, and workshop packages on the current machine
- duplicate, missing, unsupported, ambiguous, or stale active-mod and content-root cases are explainable and bounded by diagnostics rather than silently approximated
- stable deterministic modset outputs are available for `SLICE-007-raw-xml-asset-discovery-and-r1-model` to consume without inventing package truth or active-mod and content-root truth

## Non-goals

- no raw XML asset discovery or `R1`
- no XML combination, patch application, parser-lane handling, or inheritance resolution
- no snapshot manifest storage, replay, or equivalence logic
- no scheduler, reducer, or parallel execution contract implementation
- no mixed-zone bridge or resolved-def ownership work beyond the narrow active-mod and configuration handoff needed to seed native-owned `R0`