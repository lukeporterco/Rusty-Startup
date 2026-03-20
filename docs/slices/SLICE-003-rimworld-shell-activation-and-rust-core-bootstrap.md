# SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap

## Objective

Make the explicit RimWorld startup-entry, native-loading, ABI-validation, Rust-core bootstrap path, and canonical RimWorld-usable local package surface real for `rustystartup.core`, using the thin managed shell as a narrow bootstrap, environment-capture, native-boundary, and diagnostics layer only.

This slice must make the repository itself capable of acting as the canonical local `rustystartup.core` package root for current-machine startup testing, so that managed and native artifacts land in deterministic package-relative locations without per-iteration manual copy steps.

## Milestone

Stage 1

## Allowed paths

- managed/src/Bootstrap/**
- managed/src/Boundary/**
- managed/src/Diagnostics/**
- managed/src/Interop/**
- managed/src/RustyStartup.Managed.csproj
- native/**
- About/**
- LoadFolders.xml
- 1.6/**
- docs/context/Current Build Context.md
- roadmap/active_slice.yaml

## Forbidden paths

- everything not listed above
- docs/authority/**
- roadmap/roadmap.yaml
- snapshot/**
- equivalence/**
- benchmark code
- any XML discovery, patching, replay, cutoff-semantic, or mixed-zone ownership implementation

## Invariants

- The managed shell must remain thin and must not become a second owner of startup semantics.
- `rustystartup.core` remains the locked shipping identity.
- The shell may resolve paths, capture environment state, explicitly load the native library, validate ABI compatibility, and hand off into Rust, but it may not own the world-model, replay, equivalence, or execution-planning boundaries.
- Activation must be deterministic and package-relative, not hardcoded to machine-specific paths.
- Native activation must use explicit dynamic loading and ABI-checked binding rather than blind static-name probing.
- Any fallback or activation failure must be explicit and diagnostic, not silent.
- This slice builds on the self-package layout resolution established by `SLICE-002-package-root-load-folder-and-layout-resolution` and must not reopen package-root or `LoadFolders.xml` semantics as a new ownership boundary.
- The repository-local package scaffold must be the canonical local test package for `rustystartup.core`, not an ad hoc copy target.
- Managed and native artifacts used for local startup testing must land in deterministic package-relative locations under the selected active content root.
- This slice may make the repository mod-usable for current-machine testing, but it must not hardcode or semantically depend on a machine-specific RimWorld Mods path.
- This slice must preserve deterministic package-relative loading and shell-side diagnostics as a cross-platform invariant, but it must not claim Linux or macOS local validation that has not been evidenced yet.

## Frozen bootstrap activation contract

Later slices must consume shell bootstrap and Rust-core activation through one stable activation contract. That contract must carry at minimum:

- startup-entry status and entry source
- runtime revision or version basis, and its source when available
- observed package identity and identity source
- resolved package root
- selected active content root
- resolved managed assembly path and existence
- deterministic native load path and existence
- native load result
- ABI handshake status
- Rust-core bootstrap or activation status
- fallback status and fallback reason, if any
- explicit failure reason when activation cannot complete

This contract may consume the stable resolver outputs created by `SLICE-002`, but it must not scatter bootstrap truth across unrelated helper state or diagnostics-only fragments.

## Required diagnostics

- startup entry status
- runtime revision or version basis
- package identity and identity source
- resolved package root and selected active content root
- deterministic native load path
- native load result
- ABI handshake status
- Rust-core bootstrap or activation status
- fallback reason, if any
- explicit failure reason, if activation cannot complete

## Required evidence

- the RimWorld-managed startup path reaches the shell bootstrap entry from `Verse.Root_Entry.Start`, or from an explicitly equivalent startup boundary already proven in the managed shell
- the shell consumes resolved package-relative inputs from `SLICE-002` rather than hardcoded absolute paths or placeholder assumptions
- the repository contains a RimWorld-usable `rustystartup.core` package scaffold with `About/About.xml` and a selected active content root that can host managed and native artifacts for the current runtime basis
- the build or packaging path places managed and native artifacts into deterministic package-relative locations used by the local test package, without per-iteration manual copy steps
- the native library is loaded through the explicit bootstrap path and ABI-checked before Rust activation
- diagnostics visibly distinguish success, fallback, and failure cases and expose version, package, path, packaging, and ABI status
- the build context and active-slice metadata move from `SLICE-002` to `SLICE-003` cleanly
- no semantic ownership moved into C#, and no replay, equivalence, snapshot, or XML-discovery work was introduced by this slice
- any current-machine validation claims remain honest; Windows may be locally grounded, but Linux or macOS validation must not be claimed without evidence

## Evidence execution rule

The current repo snapshot does not expose a dedicated test, fixture, or eval surface inside this slice's allowed paths. Until a later control-plane change creates one, the first Codex pass for `SLICE-003` must be audit-first and may shape bootstrap code, native-loading code, ABI binding, diagnostics, and control-plane updates, but it must not claim evidence-complete slice closure.

Required evidence remains mandatory for formal completion. Any report for this slice must distinguish between bootstrap-path progress and evidence-complete closure.

## Exit criteria

- the repository provides a canonical local `rustystartup.core` mod package surface for the current runtime basis
- managed and native artifacts land in package-relative runtime paths without per-iteration manual copy steps
- RimWorld startup can reach the shell bootstrap path on the current machine using that package-relative layout
- native loading and ABI validation are part of the observable bootstrap flow
- bootstrap failure paths are explainable and bounded by diagnostics
- the shell remains a narrow boundary layer that hands control to Rust rather than owning startup meaning
- the stable bootstrap activation contract is available for later slices to consume without reopening shell ownership boundaries
- `SLICE-004-runtime-context-and-input-fingerprint-model` can begin from a stable bootstrap-and-activation boundary

## Non-goals

- no world-model ownership in managed code
- no snapshot serialization
- no replay logic
- no equivalence logic
- no XML discovery or patching
- no benchmark harness implementation
- no mixed-zone semantic implementation beyond the bootstrap handoff itself
- no machine-specific copy-to-install-path deployment contract
- no unsupported platform-validation claims beyond what evidence actually proves