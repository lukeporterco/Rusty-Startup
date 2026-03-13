# SLICE-002-package-root-load-folder-and-layout-resolution

## Objective

Make Rusty Startupâ€™s bootstrap-local self-package root detection, active load-folder selection, and package-relative managed/native path resolution real and explainable for the current RimWorld 1.6 contract, without turning the managed shell into the canonical owner of generalized modset or `R0` content-root semantics.

## Milestone

Stage 1

## Allowed paths

- managed/src/Boundary/**
- managed/src/Bootstrap/**
- managed/src/Diagnostics/**
- docs/context/Current Build Context.md
- roadmap/active_slice.yaml

## Forbidden paths

- everything not listed above
- native/**
- docs/authority/**
- roadmap/roadmap.yaml
- snapshot/**
- equivalence/**
- benchmark code
- any XML discovery, patching, generalized modset modeling, scheduler work, replay, or cutoff-semantic implementation

## Invariants

- Package identity remains locked to `rustystartup.core`; `lukep.rustystartup` is historical evidence only.
- This slice covers bootstrap-local self-resolution for Rusty Startupâ€™s own shipping package only.
- Resolver logic must derive the package root and selected active content root from discoverable package layout, current runtime version input, and explicit load-folder evidence, not from hardcoded machine paths or blind folder assumptions.
- Managed shell code may compute the self-package assembly and native paths required to activate the core, but canonical generalized `package_resolver` ownership and global `R0` content-root semantics remain native-side work.
- One canonical package root is allowed for the shipping identity; duplicate or nested Rusty Startup package identities must be surfaced as a diagnostic failure, not silently tolerated.
- Unsupported or ambiguous layout cases, including load-folder rules the bootstrap-local resolver cannot prove, must fail honestly with explicit diagnostics rather than silently approximating parity.
- Managed assembly path and native payload path must both be derived from the same selected active content root.

## Frozen bootstrap input contract

The bootstrap-local resolver for this slice may consume only the following startup inputs:

- candidate self-package root path input, represented by the current modRootPath startup field or an explicitly equivalent self-root hint
- observed package identity
- observed package identity source
- runtime build or version basis used for active content-root selection
- runtime build or version source
- managed self-assembly path or an explicitly equivalent self-location hint
- explicit LoadFolders.xml evidence derived from the resolved self-package root

No generalized modset, external package enumeration, or canonical R0 ownership inputs may be introduced in this slice.

## Supported self-layout matrix

The current 1.6 bootstrap-local resolver for this slice supports only the following self-package layouts:

- root-only layout, where the package root itself is the selected active content root
- version-folder layout, where the selected runtime version folder under the canonical package root becomes the active content root
- LoadFolders.xml-routed layout, where the selected active content root is chosen from explicit load-folder evidence for the current runtime version
- duplicate or nested ustystartup.core identities, which must fail with explicit diagnostics rather than choosing one implicitly
- unsupported or ambiguous layout cases, including conditional load-folder rules the bootstrap-local resolver cannot prove, which must fail honestly with explicit diagnostics

No other self-layout cases are treated as supported by this slice.

## Frozen stable resolver output

Later slices must consume package and layout resolution through one stable result object. That resolver output must carry at minimum:

- observed runtime build or version basis
- observed package identity and identity source
- resolved package root
- package root detection source
- selected active content root
- layout decision basis
- load-folder gate status, or an explicit note that no conditional gates were involved
- resolved managed assembly path and existence
- resolved package-relative native payload path and existence
- explicit failure reason when resolution cannot prove the current layout

Do not scatter these outputs across unrelated helper methods or diagnostics-only state.

## Required diagnostics

- observed runtime build or version basis used for folder selection
- observed package identity and identity source
- resolved package root and detection source
- selected active content root
- layout decision basis such as root-only layout, version-folder layout, `LoadFolders.xml`-routed layout, or unsupported/ambiguous case
- any load-folder gate status needed for the decision, or an explicit note that no conditional gates were involved
- resolved managed assembly path and existence
- resolved package-relative native payload path and existence
- explicit failure reason when resolution cannot prove the current layout

## Required evidence

- proof that root-only, version-folder, and `LoadFolders.xml`-routed self-package layouts resolve deterministically for the current 1.6 contract
- proof that the selected active content root drives both managed assembly and native payload path calculation
- proof that duplicate roots, stale identity state, unsupported layouts, or ambiguous layout conditions produce honest diagnostics
- proof that no blind mod-root-relative `1.6/Native/...` assumption remains once active content-root selection is introduced
- current build context updated to reflect concrete self-package layout resolution rather than generic bootstrap wording

## Evidence execution rule

The current repo snapshot does not expose any test or fixture surface inside this slice's allowed paths. Until a later control-plane change creates one, the first Codex pass for SLICE-002 must be audit-first and may shape resolver code, diagnostics, and contracts, but it must not claim evidence-complete slice closure.

Required evidence remains mandatory for formal completion. Any report for this slice must distinguish between code-shape progress and evidence-complete closure.

## Evidence execution rule

The current repo snapshot does not expose any test or fixture surface inside this slice's allowed paths. Until a later control-plane change creates one, the first Codex pass for SLICE-002 must be audit-first and may shape resolver code, diagnostics, and contracts, but it must not claim evidence-complete slice closure.

Required evidence remains mandatory for formal completion. Any report for this slice must distinguish between code-shape progress and evidence-complete closure.

## Exit criteria

- the shell can resolve the canonical Rusty Startup package root on the current machine without hardcoded absolute paths
- the shell can determine the active content root for the shipping package under the current 1.6 contract and record why that root was selected
- managed assembly and native payload paths are computed from the same selected content root and package root as stable resolver outputs
- resolution failures are structured, explainable, and safe for the next slice to consume
- `SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap` can activate RimWorld shell entry and Rust-core bootstrap from stable self-package layout inputs rather than placeholder assumptions

## Non-goals

- no Rust core world-model ownership
- no generalized multi-mod `package_resolver` or canonical `R0` implementation
- no raw XML discovery or patching
- no scheduler, reducer, or parallelism implementation
- no replay, equivalence, or mixed-zone semantic implementation




