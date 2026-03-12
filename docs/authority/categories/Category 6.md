# Integration, Packaging, and Native-Boundary Pack (Category 6, Merged)

## Executive Summary

Rusty Startup can be packaged as a normal RimWorld 1.6 mod with a thin C# shell and a Rust native core, but the native boundary must be explicitly controlled by the managed bootstrap rather than relying on implicit runtime discovery.

The strongest v1 packaging shape is:

* standard RimWorld mod metadata in `About/About.xml`
* managed shell DLL in the active `Assemblies` path selected by RimWorld’s load-folder logic
* platform-specific Rust native binaries placed in deterministic package-relative subpaths selected by the shell at runtime
* a strict in-process C ABI function table for startup ownership, semantic snapshot/replay, diagnostics, and fallback control

This merged document incorporates the packaging/identity closure and the later project decision to lock the package ID to:

* **`rustystartup.core`**

That locked package ID supersedes the earlier historical `lukep.rustystartup` identity discussed in prior evidence. The older ID remains relevant only as historical local cache/config evidence and should not be treated as the v1 package identity.

Canonical local planning assumptions used here:

* runtime baseline authority: current local RimWorld 1.6 planning runtime
* split-boundary architecture remains in force
* machine-local semantic snapshots
* plug-and-play expectations apply to the mod package itself, not to portable snapshot artifacts

## Canonical Package Identity

### Locked v1 package ID

The canonical package ID for v1 is:

* **`rustystartup.core`**

### Consequence of the lock

This resolves the earlier identity ambiguity at the project level.

The previous evidence showed:

* `lukep.rustystartup` was active in local `ModsConfig.xml`
* no currently discoverable `About/About.xml` with that package ID was found under scanned local mod roots
* historical Rusty Startup cache artifacts still referenced that older ID

That older state is now historical evidence only.

For all forward-looking implementation planning, package structure, distribution, and diagnostics, the canonical ID is:

* `rustystartup.core`

### Operational rule for v1

Before real v1 test runs, ensure:

* exactly one discoverable RimWorld mod package exists with `packageId = rustystartup.core`
* no stale active-mod state is still referencing the old package ID in test environments unless specifically testing migration behavior

## RimWorld Mod Packaging Reality

### What a normal mod package must contain

RimWorld mod packaging, as reflected in local install documentation and observed real mods, requires:

* `About/About.xml` for mod metadata and package identity
* a package root discoverable by RimWorld’s mod-list rebuild path
* `supportedVersions` compatible with the target game version, here including `1.6`

Observed local evidence is consistent with this structure across:

* base-game data mods
* Workshop mods
* local development mods

### How assemblies are discovered and loaded

Observed layouts and local documentation show RimWorld accepts several valid assembly placement patterns, depending on load-folder routing:

* root `Assemblies/`
* `1.6/Assemblies/`
* `v1.6/Assemblies/` with `LoadFolders.xml`

RimWorld’s load-folder logic determines which content roots become active for the current version. Therefore the correct implementation rule is:

* the C# shell must live in `Assemblies/` under the active content root selected for the running game version

### Where native components could plausibly live

RimWorld does not expose a first-class native-plugin mod folder convention equivalent to `Assemblies/`.

Therefore native payload placement must be treated as a package-relative convention owned by Rusty Startup itself.

Practical deterministic options are:

* under the same active load folder selected for `1.6`, such as `1.6/Native/<platform>/...`
* or under another deterministic mod-relative path resolved explicitly by the shell

The critical point is:

* native payload discovery must be explicit and shell-controlled
* it must not rely on ambient OS search paths or unspecified Unity/Mono probing luck

## Rusty Startup v1 Packaging Contract

### Required package identity

`About/About.xml` must include, at minimum:

* `packageId = rustystartup.core`
* mod name
* author
* `supportedVersions` containing `1.6`
* optional dependency/load-order metadata as needed later

### Recommended local mod structure

A strong v1 local mod layout is:

* `<ModRoot>/About/About.xml`
* `<ModRoot>/Assemblies/RustyStartup.Managed.dll`
* `<ModRoot>/Defs/...` as needed
* `<ModRoot>/Patches/...` as needed
* `<ModRoot>/LoadFolders.xml` if version routing is used
* deterministic native payload paths inside the package, for example:

  * `<ModRoot>/1.6/Native/win-x64/rustystartup_core.dll`
  * `<ModRoot>/1.6/Native/linux-x64/librustystartup_core.so`
  * `<ModRoot>/1.6/Native/macos-x64/librustystartup_core.dylib`
  * `<ModRoot>/1.6/Native/macos-arm64/librustystartup_core.dylib`

### Recommended Workshop structure

Workshop distribution should preserve the same identity and assembly-loading rules:

* one discoverable package root
* one `About/About.xml`
* one canonical `packageId = rustystartup.core`
* assembly placement in the active load folder’s `Assemblies/`
* native payloads under deterministic package-relative paths selected by the shell

### Package ID rules

For v1:

* use `rustystartup.core` consistently across local and Workshop builds
* do not use alternate debug/release package IDs
* do not retain `lukep.rustystartup` as a shipping identity
* if migration compatibility is ever needed, treat it as a config-cleanup or transition concern, not as a dual-identity packaging model

## Thin C# Shell Requirements

The managed shell is minimal in scope, but critical in responsibility.

### Minimum shell responsibilities

1. Mod bootstrap and lifecycle entry

* enter through RimWorld’s normal mod assembly load pass
* initialize in load-order-safe timing

2. Package-relative native resolution and loading

* resolve the active mod root
* resolve the active content folder selected for `1.6`
* select OS/architecture-specific native binary
* explicitly load that binary from a known absolute path

3. Stable ABI enforcement

* bind a strict C ABI function table
* verify ABI version compatibility before activating the Rust core
* reject mismatched native payloads cleanly

4. Fault isolation and explainability

* emit structured diagnostics for every critical step:

  * package root detection
  * active load-folder selection
  * native path resolution
  * native file existence
  * native load result
  * symbol binding result
  * ABI version check result
* convert native failures into actionable RimWorld log output
* preserve fallback behavior rather than hard-crashing startup where possible

5. Snapshot/replay bridge

* expose Rust-owned snapshot/replay entrypoints
* surface state transitions and reasons for replay hit, miss, invalidation, and fallback

### Managed shell placement rule

The managed shell DLL must live in the RimWorld-discoverable `Assemblies/` path for the active load folder used by the package.

## Rust Native Core Boundary

### Realistic native-boundary options

1. Static-name P/Invoke

* simple call sites
* poor path-control and weaker explainability when probing fails
* not recommended for this project

2. Explicit dynamic load plus function delegates

* shell computes the exact native path
* shell loads library explicitly
* shell binds symbols and validates ABI/function table
* strongest fit for deterministic ownership and diagnostics
* **recommended for v1**

3. Out-of-process helper

* stronger isolation in theory
* significantly worse plug-and-play story
* unnecessary complexity for v1
* not recommended except as a conceptual fallback only

### Recommended v1 native boundary

Use:

* a single in-process C ABI
* explicit shell-controlled native loading
* a versioned function table
* strict error/result contracts surfaced through logs and diagnostics

This is the strongest practical boundary for the current architecture because it matches:

* plug-and-play expectations
* machine-local snapshot ownership
* performance-first startup goals
* strong explainability requirements

## Cross-Platform Constraints

### Confirmed locally

The current machine confirms:

* Windows RimWorld install layout
* managed Unity/Mono runtime environment in local logs
* normal mod discovery and package-ID resolution flow through local decompilation and config state

### Not fully validated locally

The following remain implementation/test tasks rather than architecture blockers:

* exact Linux runtime native-loading behavior for packaged RimWorld mods
* exact macOS runtime native-loading behavior for packaged RimWorld mods
* final naming/placement validation for all non-Windows native artifacts in real package deployment

### Cross-platform rule for planning

These unverified items do not block the implementation plan.

They do mean:

* the packaging contract must reserve explicit per-platform payload paths
* the shell must own all platform selection logic
* cross-platform release sign-off requires real platform validation later

## Workshop and Local-Mod Constraints

### Workshop realities

Observed local Workshop behavior shows:

* Workshop mods live under numeric item roots
* `About/PublishedFileId.txt` is extremely common, though not universal
* load-folder naming/casing/layout varies
* mixed content-root structures are normal
* duplicate/nested `About.xml` trees can exist and create ambiguity if packaging is sloppy

### Local development realities

Observed local development mods include junction-based roots pointing into source repositories.

This is useful for development, but it can hide assumptions that do not hold in Workshop packaging.

Therefore the v1 packaging contract must be judged against:

* a real discoverable RimWorld package root
* not only a development junction workflow

### Packaging discipline rule

Rusty Startup should ship with:

* one canonical package root
* one canonical `About/About.xml`
* one canonical `packageId = rustystartup.core`
* no nested duplicate package identities inside the shipping package

## Integration Risks

### Risk 1: stale package identity state

Historical configs or caches may still reference `lukep.rustystartup`.

Consequence:

* test environments could accidentally activate stale config state or create confusing logs

Mitigation:

* treat old ID references as migration debris only
* clean test configs or explicitly detect/report the mismatch

### Risk 2: native load failure modes

Possible causes:

* OS/arch mismatch
* missing native file
* wrong package-relative path
* symbol mismatch
* ABI mismatch

Mitigation:

* shell-side explicit path resolution
* explicit ABI validation
* diagnostics with full path/platform/symbol details

### Risk 3: load-folder selection drift

If `LoadFolders.xml` or version fallback selects a different folder than assumed, assembly or native paths can be wrong.

Mitigation:

* shell must resolve paths from actual selected mod content roots, not hardcoded assumptions

### Risk 4: duplicate metadata collisions

Duplicate or nested `About.xml` identities in packages can confuse mod identity and troubleshooting.

Mitigation:

* one canonical package identity per shipping package
* no nested duplicate Rusty Startup package IDs

### Risk 5: explainability regression at the native boundary

Without strict shell-side diagnostics, native failures become opaque and violate one of the project’s core requirements.

Mitigation:

* structured shell-side logging is mandatory, not optional

## Recommended Boundary for v1

### Final v1 package contract

1. **Package identity**

* `packageId = rustystartup.core`

2. **Managed shell placement**

* ship `RustyStartup.Managed.dll` in the active RimWorld `Assemblies/` path for the selected 1.6 content root

3. **Native payload placement**

* ship per-platform Rust binaries under deterministic package-relative paths
* resolve them explicitly from the shell

4. **Shell loading flow**

* detect platform/arch
* compute absolute package-relative native path
* explicitly load native library
* bind function table
* verify ABI version
* activate Rust core
* on failure, emit structured diagnostics and enter fallback behavior rather than crashing blindly

5. **Boundary API focus**

* semantic snapshot/replay entrypoints
* ownership handoff for deterministic startup phases
* diagnostics and status reporting
* fallback and degraded-mode control

This is the strongest practical integration approach for the architecture currently locked in.

## Historical Identity Closure Incorporated

The earlier package/discovery blocker is now interpreted as follows:

* the local environment previously showed `lukep.rustystartup` active in config but not discoverable in current scanned mod roots
* historical LocalLow cache data existed for that old ID
* that mismatch is now closed at the planning level by adopting `rustystartup.core` as the canonical v1 package ID

So the earlier evidence remains useful only as:

* proof that stale config/cache state can exist
* proof that diagnostics and identity-cleanup logic matter

It is no longer a live package-identity ambiguity for the implementation plan.

## Residual Risks Worth Tracking

These do not reopen the Category 6 architecture, but they remain implementation notes:

1. Linux/macOS packaged native-loading behavior still needs platform validation before release.
2. If migration from old local configs is desired, a transition policy may be needed for stale `lukep.rustystartup` references.
3. Real Workshop packaging should be validated against a non-junction shipping package before release.
4. Native payload signing, quarantine, or platform-specific trust issues may arise later on macOS and should be treated as release-validation tasks.

## Canonical Wording Going Forward

Use these phrases consistently:

* **canonical v1 package ID**: `rustystartup.core`
* **managed shell**: thin RimWorld-discoverable bootstrap DLL in `Assemblies/`
* **native boundary**: explicit shell-controlled in-process C ABI load
* **native payload placement**: deterministic package-relative per-platform paths
* **old ID**: `lukep.rustystartup` is historical evidence only, not the shipping identity

## Source Evidence Appendix

### Local paths and files represented in the merged evidence

* local RimWorld install metadata and version files
* local `Player.log` / `Player-prev.log`
* local `ModsConfig.xml`
* local Rusty Startup LocalLow cache/config artifacts referenced in the historical identity closure
* local `ModUpdating.txt`
* local base-game and Workshop `About/About.xml` examples
* local Workshop and local mod roots used in prior packaging scan work

### Key vanilla types and methods represented in the merged evidence

* `Verse.ModLister.RebuildModList`
* `Verse.ModMetaData`
* `Verse.ModsConfig`
* `Verse.ModContentPack.InitLoadFolders`
* `Verse.ModAssemblyHandler.ReloadAll`
* `Verse.GenFilePaths`

### Follow-up findings incorporated here

* packaging/identity closure:

  * earlier local `lukep.rustystartup` mismatch explained as active-config entry plus no currently discoverable mod root for that ID
* current project decision:

  * canonical v1 package ID is now `rustystartup.core`

### Representative command/evidence types used in the earlier audit work

* local config inspection
* recursive `About.xml` package-ID scans over discoverable roots
* local cache artifact inspection
* local decompilation of mod discovery and content-pack types
* local Workshop/package layout inspection
