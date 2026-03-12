# Rusty Startup Implementation Plan

## 1. Authority, scope, and no-drift rule

This implementation plan is bound by three authority layers, in this order.

First, the finalized Category 1 through Category 7 documents are the primary authority for how RimWorld 1.6 actually behaves. They define the real startup boundaries, the resolved-def cutoff, the snapshot surface, the integration contract, the benchmark baseline, and the mod-ecosystem constraints.

Second, the QA architecture decisions are binding product decisions. They define what Rusty Startup is allowed to own, what it must prove, how aggressive it may be, where it may reorder, what counts as correctness, what kind of fallback is acceptable, how snapshots behave, and what v1 must already demonstrate.

Third, the earlier project-plan report provides the conceptual frame, but it may not override grounded category evidence or the locked QA decisions.

The no-drift rule for this plan is simple: if a later implementation choice conflicts with the finalized category evidence or the locked QA decisions, the implementation choice is wrong and must be corrected.

## 2. Frozen architecture baseline

Rusty Startup v1 is a replacement-oriented, Rust-first RimWorld startup engine. It owns the deterministic startup region as far as the evidence allows, uses a thin C# shell only where RimWorld and managed runtime constraints require it, and must already demonstrate semantic snapshot and replay in v1.

The semantic target is fully resolved def equivalence to vanilla, not merely unified XML equivalence and not full later runtime-state equivalence. That equivalence cutoff is after short-hash assignment and before the delegated callback-driven live tail.

Managed-adjacent work may be reordered where semantic equivalence is preserved, but Rusty Startup is not allowed to blur or relax the equivalence contract. Solver behavior is aggressive, but it must be justified by real RimWorld mod data rather than abstract optimism.

Snapshots are machine-local only. The internal Rust world-model is authoritative up to the runtime frontier. Diagnostics are mandatory and must explain ownership, replay, fallback, invalidation, and equivalence outcomes clearly enough to support development and compatibility work.

## 3. Canonical RimWorld 1.6 boundary model

The finalized category set resolves the earlier contradiction by separating three boundaries that must not be conflated.

The first arbitrary managed-execution boundary begins at `LoadedModManager.CreateModClasses`. That is where mod constructors begin to exist and startup becomes constructor-sensitive and Harmony-sensitive.

The runtime-sensitive mixed zone starts at `CreateModClasses` and continues through the rest of the deterministic semantic pipeline.

The semantic-equivalence and snapshot cutoff is immediately after `ShortHashGiver.GiveAllShortHashes()` returns inside `PlayDataLoader.DoPlayLoad()`.

The delegated live tail begins only after that cutoff, in the callback-driven region queued through `LongEventHandler.ExecuteWhenFinished(...)`, including bios, later language injection, late static-constructor work, atlas baking, GC, and related cleanup.

This means Rusty Startup v1 is not allowed to treat `CreateModClasses` as the end of ownership. It must instead use a split-boundary model:

* strong Rust ownership or mirroring before `CreateModClasses`
* managed-assisted but still semantically Rust-owned behavior from `CreateModClasses` through `GiveAllShortHashes()`
* delegated live-tail behavior after the cutoff

That split-boundary model is the backbone of the implementation plan.

## 4. Canonical startup chain to implement against

The relevant startup path begins at `Verse.Root_Entry.Start`, passes through `Verse.Root.Start`, `Verse.Root.CheckGlobalInit`, `Verse.Root.Update`, `LongEventHandler.LongEventsUpdate`, `PlayDataLoader.LoadAllPlayData(false)`, `PlayDataLoader.DoPlayLoad`, and then into `LoadedModManager.LoadAllActiveMods`.

Within the core mod-loading chain, the plan must be grounded against this ordered sequence:

* `XmlInheritance.Clear()`
* `InitializeMods()`
* `LoadModContent(hotReload)`
* `CreateModClasses()`
* `LoadModXML(hotReload)`
* `CombineIntoUnifiedXML(...)`
* `TKeySystem.Clear()` and `TKeySystem.Parse(unifiedXml)`
* patch validation and `ApplyPatches(...)`
* `ParseAndProcessXML(...)`
* `ClearCachedPatches()`
* `XmlInheritance.Clear()`

Inside `PlayDataLoader.DoPlayLoad()`, the plan must then track the later deterministic phases that matter to the resolved-def cutoff:

* `DefDatabase<T>.AddAllInMods`
* cross-reference resolution
* early and final `DefOf` rebinding
* implied-def generation pre-resolve and post-resolve
* `ResolveAllReferences()` lifecycle
* resets and finalizers up to the cutoff
* `ShortHashGiver.GiveAllShortHashes()`

Anything after that point is outside the v1 semantic cutoff.

## 5. v1 product scope

v1 must already demonstrate all of the following.

It must own the deterministic startup region up to the split-boundary model actually supported by the evidence.

It must implement machine-local semantic snapshot and replay, rather than merely planning for them.

It must validate fully resolved def equivalence under the default parser path.

It must expose diagnostics strong enough to explain why a run took a replay-hit path, a replay-miss path, a managed-assisted path, or a fallback path.

It must maintain near-universal compatibility as a target, with rare pathological fallbacks permitted, but only where that fallback posture is justified by actual mod behavior or explicit runtime-bound constraints.

v1 does not need to own or reproduce the delegated live tail. It does not need to claim full parity under legacy XML-deserializer mode. It does not need portable snapshots. It does not need to fully replace Unity-coupled tail work.

## 6. Core implementation strategy

The implementation strategy for v1 is not a full from-scratch reimplementation of all startup logic in Rust. It is a staged ownership model in which Rust owns the semantic world-model and replay engine, while the C# shell provides the minimum necessary managed bridge and mixed-zone assistance.

The practical rule is:

* pure deterministic discovery, normalization, patching, snapshot logic, fingerprinting, validation, and execution planning belong in Rust wherever possible
* the mixed zone from `CreateModClasses` through `GiveAllShortHashes()` remains semantically owned by Rust, but may use managed-assisted execution or managed-assisted validation where RimWorld behavior is constructor-sensitive, Harmony-sensitive, or parser-sensitive
* the delegated live tail remains vanilla-driven in v1

This is the safest way to stay faithful to both the evidence and the locked architecture.

### 6.1 Concurrency model

Parallel execution is a required implementation property of Rusty Startup, not an optional later optimization.

For the Rust-owned deterministic region, the default execution model is a task graph executed by a bounded Rust worker pool. The scheduler must treat parallel execution as the primary lane on multicore machines, with serial execution reserved for diagnostics, explicit forced-serial benchmarking, or safety fallbacks when a phase is marked non-parallel.

The concurrency model is phase-aware rather than free-form. Each region or phase must be labeled as one of:
- parallel
- partitioned-parallel
- barriered
- serialized
- managed-thread-bound

The scheduler must obey those labels strictly.

The worker pool should be sized from the machine’s available physical cores, with at least one worker and with one core effectively reserved for the main RimWorld thread where practical. The exact heuristic may be tuned during implementation, but the contract is that Rust-owned deterministic work must scale across cores rather than execute as a mostly serial engine.

No worker thread may directly call Unity APIs, invoke mod constructors, trigger Harmony-sensitive runtime behavior, or cross into the delegated live tail. All such operations remain main-thread or managed-boundary work.

The Rust world-model must not be mutated concurrently through shared fine-grained global state. Parallel phases must instead produce region-local or shard-local outputs, followed by deterministic reduction and commit. This keeps the result reproducible and keeps semantic equivalence independent of worker interleaving.

The required barrier classes are:
- input-freeze barriers, where active mod set, content roots, parser mode, and language state become fixed for the run
- canonicalization barriers, where parallel discovery or parsing results are merged into deterministic ordered forms
- mixed-zone entry and exit barriers, where control crosses into managed-assisted behavior
- resolve-pass barriers, where one semantic phase must be globally complete before the next can begin
- cutoff barrier, where final `DefOf` and short-hash state are complete before equivalence validation and replay persistence

Any fallback from a parallel lane to a serialized lane must be surfaced in diagnostics with a specific reason.

## 7. Ownership matrix for implementation

### 7.1 Excluded or boundary-only region

The Unity bootstrap before `Root_Entry.Start` is outside v1 ownership. It is treated only as the entry boundary.

### 7.2 Rust-owned deterministic pre-mixed zone

These phases should be owned or strongly mirrored by Rust in v1:

* active mod set resolution
* mod order and content-root selection
* `LoadFolders.xml` and version-aware root resolution
* raw XML asset discovery from `Defs/` and `Patches/`
* file-order and source-order normalization
* unified XML combination
* patch application
* inheritance registration and resolution
* TKey parsing and mapping state that belongs before the cutoff
* snapshot fingerprinting, invalidation, and replay planning
* diagnostics for all of the above

These map directly to snapshot regions `R0` through early `R3`, plus the planning and control structures around them.

### 7.3 Managed-assisted but semantically Rust-owned mixed zone

This is the critical v1 region. It begins at `CreateModClasses` and extends through `GiveAllShortHashes()`.

Rusty Startup remains the semantic owner here, but the implementation is allowed to use managed assistance for:

* assembly-load-sensitive or constructor-sensitive mod instantiation
* default parser path behavior where RimWorld’s current managed path is the most reliable parity anchor
* def materialization and insertion ordering where overwrite priority and running-mod order matter
* cross-reference resolution passes
* implied-def generation stages
* reference-resolution lifecycle
* final `DefOf` rebinding
* short-hash assignment

The design intent is not to hand semantic control back to vanilla. It is to use tightly controlled managed operations where they are the most reliable path to matching vanilla semantics, then bring the results back into Rust-owned validation and snapshot logic.

### 7.4 Delegated live tail

These remain delegated in v1:

* `ExecuteWhenFinished(...)` callback body effects
* later language injection after implied defs
* bios
* late static-constructor work
* atlas baking
* GC and resource unload work
* Unity-coupled tail behavior not required for the resolved-def cutoff

The v1 implementation plan must explicitly stop semantic claims at the cutoff and never silently fold delegated live-tail work into equivalence claims.

### 7.5 Per-region concurrency contract

The concurrency contract for v1 is region-specific.

`R0` and `R1` are parallel by default. Mod-root inspection, content-root resolution, XML discovery, patch discovery, byte hashing, and metadata collection should fan out across mods and file trees. The final active-mod ordering and canonical discovered-asset ordering must be committed through a deterministic serial merge.

`R2` and `R3` are partitioned-parallel. XML loading, patch indexing, patch candidate preparation, inheritance indexing, and other structure-building work may execute in parallel, but unified XML emission and final inheritance-resolved canonical state must pass through deterministic merge points that preserve RimWorld ordering semantics.

`R4` is partitioned-parallel. Def parsing, bucket construction, and per-type staging may execute in parallel, but final insertion order, overwrite priority, and any order-sensitive commit behavior must be serialized through deterministic reducers.

`R5` and `R6` are phased-parallel. Index construction and partitionable resolve work may execute in parallel inside a pass, but global pass completion is barriered. No later resolve pass may start until the current pass has committed globally consistent state.

`R7`, `R8`, and `R9` are conservative. Any preparation or indexing work that is provably independent may be parallelized, but final `DefOf` binding and short-hash completion must be treated as cutoff-sensitive commit phases with deterministic ordering guarantees.

The mixed zone is not a free-parallel region. Managed-assisted calls remain main-thread-bound or managed-boundary-bound. Rust may still perform parallel preparation, validation, indexing, or post-call canonicalization around those calls, but the managed crossing itself is not a worker-pool task.

The delegated live tail is outside Rusty Startup’s parallel ownership scope in v1.

## 8. Internal system architecture

The implementation should be divided into narrow modules aligned to fault boundaries and evidence boundaries.

### 8.1 Managed shell

A thin C# shell is required. Its responsibilities are:

* package discovery and path resolution
* runtime revision and environment capture
* explicit native library loading from deterministic package-relative paths
* ABI version validation and function-table binding
* orchestration of startup entry into the Rust core
* mixed-zone managed assistance where v1 requires it
* structured diagnostics and fallback surfacing

The shell is not allowed to become a second owner of startup semantics. It exists to bridge RimWorld to the Rust core.

### 8.2 Native core

The Rust core is the authoritative owner of:

* the startup world-model
* fingerprinting and invalidation
* snapshot manifest logic
* snapshot load/store logic
* XML and patch semantic normalization
* execution planning
* equivalence validation
* compatibility classification
* observability state

### 8.3 Internal modules

The implementation should use the following major modules.

**bootstrap**
Owns startup entry coordination, runtime revision capture, parser-mode detection, platform detection, and shell-to-core handshake.

**package_resolver**
Owns active package root resolution, `LoadFolders.xml` handling, version-aware content-root selection, and deterministic package-relative path calculation.

**modset_model**
Owns the canonical active-mod graph, ordered package IDs, content-root mapping, and dependency/load-order metadata.

**asset_discovery**
Owns deterministic discovery of raw XML and patch assets and any other startup-relevant data that belongs before the cutoff.

**xml_pipeline**
Owns unified XML construction, patch application, inheritance graph handling, and parser-lane control.

**parser_lane_manager**
Owns parser-mode handling. v1 targets default `DirectXmlToObjectNew` semantics. If legacy parser mode is detected, this module must mark the run as non-primary parity and route the run into a managed-assisted or degraded lane.

**def_pipeline**
Owns the semantic model for parsed defs, insertion buckets, overwrite behavior, and the preconditions for def-database population.

**mixed_zone_bridge**
Owns all managed-assisted operations between `CreateModClasses` and `GiveAllShortHashes()`. It is the only module allowed to intentionally cross into constructor-sensitive or Harmony-sensitive mixed-zone behavior.

**xref_and_resolve**
Owns cross-reference state, resolve passes, implied-def phase coordination, and final resolved-def graph construction as represented in the Rust world-model.

**defof_and_hash**
Owns the semantic representation and validation rules for final `DefOf` binding and short-hash completion.

**snapshot**
Owns snapshot manifests, region serialization, load/save rules, replay safety checks, and partial invalidation handling.

**equivalence**
Owns the Category 3 cutoff oracle, mismatch classification, and proof that restored or recomputed state matches fully resolved def equivalence.

**compatibility_classifier**
Owns evidence-based aggressive ownership decisions grounded in real mod structure and run-time signals.

**diagnostics**
Owns all structured explainability surfaces for ownership, replay, invalidation, fallback, parser mode, and mixed-zone decisions.

**benchmarking**
Owns instrumentation surfaces and the benchmark row schema required by Category 7.

**task_scheduler**
Owns the phase-aware task graph, worker-pool policy, work submission, barrier coordination, and the distinction between parallel, partitioned-parallel, serialized, and managed-thread-bound phases.

**deterministic_reducer**
Owns shard merge, canonical ordering, deterministic commit, and the guarantee that multicore execution does not change semantic output.

## 9. Canonical data model

The core must work from a stable semantic world-model instead of ad hoc structures.

At minimum, the world-model should contain the following records.

**RuntimeContext**
Canonical runtime revision, parser mode, OS/arch, selected load roots, language key, and other machine-local environment keys relevant to snapshot validity.

**ModIdentity**
Package ID, source root, selected content root, local or Workshop origin, load-order position, and dependency metadata.

**DiscoveredXmlAsset**
Resolved path, source bytes hash, mod provenance, logical bucket (`Defs` or `Patches`), and ordering metadata.

**UnifiedXmlState**
The post-combination, post-patch semantic XML state needed for replay and equivalence.

**ResolvedInheritanceGraph**
The inheritance-resolved node graph with gating decisions already applied.

**ParsedDefBucket**
Typed def records grouped by mod and by final insertion order.

**ResolvedDefGraph**
The final graph of defs and their resolved references through the cutoff.

**DefOfState**
The final deterministic `DefOf` identity/binding state that belongs at the cutoff.

**ShortHashState**
The per-def-type short-hash assignment surface through `GiveAllShortHashes()`.

**SnapshotManifest**
Versioned machine-local manifest containing input fingerprints, parser mode, lane mode, region hashes, invalidation reasons, and compatibility metadata.

**RunDecisionState**
Replay hit or miss, mixed-zone lane, fallback reason, equivalence result, and diagnostics summary for the current launch.

## 10. Snapshot architecture

### 10.1 Public snapshot contract

The v1 snapshot contract is fixed to regions `R0` through `R9` with dependency order:

`R0 -> R1 -> R2 -> R3 -> R4 -> R5 -> R6 -> R7 -> R8 -> R9`

The replay target is not merely restored bytes. The replay target is fully resolved def equivalence at the Category 3 cutoff.

### 10.2 Region ownership

`R0`: active mod set, mod order, and resolved content roots per mod.

`R1`: raw XML asset discovery set after folder resolution.

`R2`: unified XML document after patch application.

`R3`: inheritance-resolved XML node graph.

`R4`: parsed defs assigned to buckets and inserted by def type.

`R5`: cross-reference resolution state.

`R6`: final resolved def graph, including implied defs and reference resolution.

`R7`: final `DefOf` binding state.

`R8`: TKey parse and build mappings required before the cutoff.

`R9`: final short-hash assignments and dictionaries.

### 10.3 Replay rules

The replay engine must preserve the dependency chain `R0 -> R1 -> R2 -> R3 -> R4 -> R5 -> R6 -> R7 -> R8 -> R9`, but it is not required to execute every replay-related operation serially.

Fingerprint evaluation, manifest checks, blob integrity checks, and shard-level validation may execute in parallel where region dependencies permit it. The decision about the largest valid snapshot prefix must still be committed deterministically.

The replay system must support at least three classes of execution.

**full replay hit**  
All required inputs and region dependencies are valid. Validation may run in parallel across independent fingerprint classes or region-local shards, but the final replay decision is a single deterministic commit. Region restoration may decode or materialize shard-local data in parallel, followed by deterministic region commit and cutoff validation.

**partial replay**  
The engine restores the largest valid prefix and recomputes the invalidated suffix. Recomputed regions must use the same per-region concurrency contract as cold execution. Prefix restoration and suffix recomputation may overlap only where dependencies allow it. No downstream region may commit before all of its prerequisites are valid and globally committed.

**degraded or fallback replay path**  
If parser mode, mixed-zone risk, runtime drift, or compatibility classification prevents a primary replay claim, the replay engine must explicitly downgrade the run. Degraded replay may still use parallel work inside safe deterministic regions, but the run must not present itself as a primary parity hit.

Parallel replay is therefore allowed, but only under dependency-respecting, barrier-aware, deterministic commit rules. Replay speed must never come from weakening semantic ordering guarantees.

### 10.4 Invalidation classes

The plan must treat all of the following as snapshot-relevant inputs:

* runtime revision and build identity
* active mod set and exact order
* selected content roots and `LoadFolders.xml` resolution
* XML and patch bytes or structural equivalents
* parser mode
* language-sensitive inputs
* assembly fingerprints where they can affect deterministic startup meaning
* machine-local environment keys required by the snapshot contract

### 10.5 Snapshot storage and corruption handling

Snapshots are machine-local. The manifest must be versioned and include enough metadata to reject incompatible or stale blobs cleanly.

Blob corruption, parser-mode drift, mixed-zone lane drift, and build/revision drift must all surface explicit invalidation reasons. The system must not silently reuse stale state.

## 11. Equivalence contract

The correctness contract for v1 is Category 3 fully resolved def equivalence.

That means the implementation must verify, at minimum:

* database-universe equality for the relevant def types
* strict-type sequence equality where order is semantically important
* key equality where exact ordering is not semantically load-bearing
* canonical semantic payload equality for the def graph
* global resolved-def invariants through the cutoff

The equivalence claim includes the semantic consequences of mixed-zone phases as long as the final cutoff state matches the oracle.

The equivalence claim explicitly excludes post-cutoff callback effects, later language injection, late static-constructor consequences, atlas baking, GC, and other delegated live-tail behavior.

The implementation must keep this boundary crisp. No diagnostic or benchmark output may imply full startup parity beyond the cutoff.

## 12. Parser policy

v1 targets the default parser path:

`Verse.DirectXmlToObjectNew.DefFromNodeNew`

If `legacy-xml-deserializer` is active, the run must be marked explicitly in diagnostics. v1 must not claim full Rust parser-parity ownership for that run. Instead, it must route the run into a managed-assisted or delegated lane as required by the evidence.

This means the primary parity contract, snapshot contract, and benchmark claims all bind to default parser semantics.

Legacy parser runs are allowed, but they are not allowed to silently masquerade as primary parity runs.

## 13. Compatibility classifier

The compatibility classifier should not be a binary supported/unsupported switch. It should classify startup work into ownership lanes that reflect the split-boundary model.

At minimum, the classifier should support:

**Lane A: Rust-owned deterministic lane**
Used where the evidence shows pure or strongly mirrorable deterministic behavior.

**Lane B: managed-assisted but semantically owned lane**
Used in the mixed zone where constructors, Harmony, or parser behavior can matter but Rusty Startup still owns the semantic result.

**Lane C: fallback or degraded lane**
Used where v1 cannot safely prove full parity, such as legacy parser mode or rare startup patterns that exceed the proven mixed-zone model.

Classifier inputs should include:

* parser mode
* mod corpus signals
* assembly presence and structure
* observed constructor-sensitive or Harmony-sensitive signals
* runtime revision
* snapshot validity state

The classifier’s output must be recorded in diagnostics and benchmark rows.

## 14. Managed shell and native boundary

### 14.1 Package identity

The locked v1 package identity is `rustystartup.core`.

The project must not ship under `lukep.rustystartup`. Any traces of that identity are migration debris, not a shipping contract.

### 14.2 Package layout

The recommended local and Workshop structure is one canonical package root with:

* `About/About.xml`
* `Assemblies/RustyStartup.Managed.dll` or the version-routed `Assemblies/` path under the active content root
* deterministic native payloads under package-relative platform-specific paths, for example under `1.6/Native/...`
* optional `Defs/`, `Patches/`, and `LoadFolders.xml` where required

### 14.3 Native loading strategy

v1 should use explicit dynamic loading plus function delegates, not blind static-name probing.

The shell must:

* detect platform and architecture
* compute the selected package-relative native path
* explicitly load the native library
* bind the function table
* verify ABI compatibility
* activate the Rust core
* log a structured failure and enter fallback behavior if activation fails

### 14.4 Cross-platform rule

Windows is locally grounded. Linux and macOS loading details are not fully validated on this machine, but they are not architecture blockers. The implementation plan must therefore preserve deterministic package-relative native loading and shell-side diagnostics as a cross-platform invariant, while leaving final platform verification as an implementation validation task.

## 15. Execution flow

### 15.1 Cold start without snapshot hit

1. Shell captures runtime revision, parser mode, platform, language, and package roots.
2. Shell loads Rust core and validates ABI.
3. Rust core builds `RuntimeContext` and `ModIdentity` set.
4. Rust computes `R0` and `R1` inputs.
5. Rust evaluates snapshot manifest validity.
6. If no valid snapshot exists, Rust proceeds through `R2` and `R3` directly.
7. At the mixed zone, the managed shell performs the required assisted operations while Rust remains the semantic owner and collects canonical world-model outputs for `R4` through `R9`.
8. Rust validates fully resolved def equivalence at the cutoff.
9. Rust persists snapshot regions and manifest.
10. Control returns to RimWorld’s delegated live tail.

### 15.2 Warm start with full replay hit

1. Shell captures environment and loads Rust core.
2. Rust validates manifest fingerprints.
3. Rust restores `R0` through `R9`.
4. Rust runs replay safety checks and equivalence validation.
5. Diagnostics record replay hit and lane choice.
6. Control returns to RimWorld for the delegated live tail.

### 15.3 Partial replay path

1. Rust restores the largest valid snapshot prefix.
2. Rust recomputes invalidated downstream regions only.
3. Mixed-zone assistance is invoked if the invalidated region crosses into the mixed zone.
4. Equivalence validation and diagnostics proceed as usual.

### 15.4 Degraded lane

If parser mode, mixed-zone risk, or a proven compatibility hazard prevents a primary parity claim, Rusty Startup must downgrade the run explicitly and surface the reason. The degraded path is still allowed to solve aggressively, but it must not hide the fact that the run is outside the primary parity contract.

### 15.5 Concurrency barriers and merge points

The execution flow has a small number of mandatory global barriers.

The first barrier is the input-freeze barrier. Before any meaningful parallel work begins, the run must fix the active mod set, ordered package IDs, selected content roots, parser mode, runtime revision, and language-sensitive state.

The second barrier is the discovery canonicalization barrier. Parallel file-system and asset discovery may fan out broadly, but the resulting discovered sets must be merged into a deterministic ordered representation before unified XML and patch work proceed.

The third barrier is the pre-mixed-zone barrier. Rust-owned deterministic preprocessing must commit its canonical inputs before control crosses into managed-assisted behavior around `CreateModClasses` and later mixed-zone phases.

The fourth barrier class is the resolve-pass barrier. Parallel work inside a resolve pass is allowed, but pass completion must be global before the next semantic phase begins.

The fifth barrier is the cutoff barrier. Final `DefOf` state, TKey state that belongs before the cutoff, and short-hash completion must all be globally committed before equivalence validation, replay persistence, or handoff to the delegated live tail.

All parallel phases must end in deterministic merge points. Those merge points are not optional implementation details. They are part of how Rusty Startup preserves semantic equivalence while using multicore execution.

## 16. Diagnostics and explainability

Diagnostics are a required subsystem, not optional polish.

Each run must explain:

* runtime revision and parser mode
* active package IDs and mod count
* selected content roots
* package identity and native load path
* snapshot hit or miss
* invalidation reasons
* chosen ownership lane
* fallback reason if any
* equivalence pass or mismatch result
* any mixed-zone managed assistance that materially affected the run

The system should emit a stable machine-readable structure as well as human-readable RimWorld log lines.

At minimum, diagnostics should make the following impossible:

* a replay hit with no visible explanation of why it was valid
* a fallback with no visible reason
* a legacy parser run that is indistinguishable from a primary-parity run
* a native loading failure that lacks path and ABI details

### 16.1 Parallelism diagnostics

Diagnostics must make concurrency behavior visible.

Each run must record:
- whether the scheduler ran in parallel-primary, forced-serial, or degraded-serial mode
- worker-pool size
- which regions ran in parallel, partitioned-parallel, or serialized mode
- any fallback from parallel to serial, with reason
- barrier wait times where available
- deterministic reducer or merge timings where available
- whether mixed-zone work forced a serialized boundary crossing
- whether replay validation and replay restore used parallel work or serial work

The system must make it possible to answer, from diagnostics alone, whether Rusty Startup actually used multicore execution or merely remained architecturally compatible with it.

## 17. Benchmark and measurement plan

The benchmark baseline is canonicalized to `RimWorld 1.6.4633 rev1261` on the current machine.

### 17.1 Multicore measurement requirements

Benchmarking must prove that multicore execution is real, not merely assumed.

In addition to wall-clock timings, the benchmark harness must record enough information to distinguish:
- primary parallel runs
- forced-serial control runs
- degraded runs
- replay-hit runs
- replay-miss runs

At minimum, benchmarking must support a forced-serial comparison mode for the Rust-owned deterministic region. That mode exists only for measurement and validation. Its purpose is to prove that the multicore scheduler produces real gains and to identify where the engine is still unintentionally serial.

Where practical, benchmark rows should also record:
- worker count
- region-level parallel or serialized status
- total serialized share of the run inside Rust-owned regions
- barrier-heavy phases
- replay validation cost versus recomputation cost

The benchmark harness does not need per-thread micro-profiling in v1, but it must be able to prove that parallel execution materially contributes to startup reduction.

The benchmark tiers are:

* Tier S: 4 active mods
* Tier M: 11 active mods
* Tier L: 16 to 50 active mods
* Tier X: more than 50 active mods

The minimum valid measurement set for implementation planning is:

* runtime revision banner including revision
* active mod count and ordered package IDs
* `Loaded All Assemblies` timing
* `Finished resetting the current domain` timing
* `UnloadTime` if present
* `Total: ... ms` line if present
* Rusty status line including mode, hit or miss, and reason

At minimum, milestone planning requires:

* at least 3 warm runs for Tier S on `rev1261`
* at least 3 warm runs for Tier M on `rev1261`
* at least one miss and one hit where Rusty behavior applies
* no mixed-version aggregation

The benchmark harness should keep a row schema that records runtime revision, tier, requested mode, effective mode, hit or miss, reason, wall-clock, any phase buckets, and snapshot sizes or related metadata.

Phase buckets should remain aligned to ownership boundaries, for example discovery, XML, patching, resolve, validation, replay, and tail-related buckets.

## 18. Milestone ladder

### Milestone 0: packaging and shell activation

Goals:

* package identity fixed to `rustystartup.core`
* deterministic package-root and active load-folder resolution
* native load path computation
* explicit dynamic native loading
* ABI validation
* structured bootstrap diagnostics

Exit criteria:

* managed shell loads in RimWorld 1.6
* Rust core activates on the current machine
* diagnostics clearly show version, package, path, and ABI status

### Milestone 1: world-model and discovery foundation

Goals:

* implement `RuntimeContext`, `ModIdentity`, and package-root resolution
* implement deterministic mod-set modeling
* implement `R0` and `R1`
* fingerprint runtime revision, parser mode, language, and content roots

Exit criteria:

* reproducible mod-set and XML-discovery model
* stable manifest inputs for `R0` and `R1`
* no ambiguity in active package IDs or selected content roots

### Milestone 2: XML and patch semantic core

Goals:

* implement `R2` and `R3`
* unify XML and patch logic under the Rust world-model
* encode parser-mode handling
* bind diagnostics to parser lane and patch results

Exit criteria:

* deterministic unified XML and inheritance-resolved graph for default parser lane
* explicit degraded behavior for legacy parser mode

### Milestone 3: mixed-zone bridge and def pipeline

Goals:

* implement managed-assisted mixed-zone bridge beginning at `CreateModClasses`
* materialize `R4` through `R6`
* encode constructor-sensitive and Harmony-sensitive lane handling
* model resolved def graph in Rust

Exit criteria:

* mixed-zone operations produce stable resolved def graph candidates
* diagnostics clearly identify mixed-zone managed assistance

### Milestone 4: `DefOf`, TKey, and short-hash cutoff completion

Goals:

* implement `R7`, `R8`, and `R9`
* complete the fully resolved def cutoff surface
* finalize equivalence validator against Category 3

Exit criteria:

* a run can prove or fail fully resolved def equivalence at the cutoff
* short-hash state is captured and restorable in the world-model

### Milestone 5: snapshot and replay v1

Goals:

* persist region manifests and region blobs
* implement full replay hit, partial replay, and degraded replay lanes
* surface hit, miss, and invalidation reasons in diagnostics

Exit criteria:

* v1 demonstrates semantic snapshot and replay honestly
* replay-hit and replay-miss runs are benchmarked separately

### Milestone 6: compatibility and benchmark hardening

Goals:

* broaden mod-corpus validation
* refine compatibility classifier using real local and Workshop evidence
* complete minimum benchmark set for Tier S and Tier M
* verify fallback honesty under edge cases

Exit criteria:

* near-universal compatibility target is supported by evidence, not just theory
* benchmark outputs are ownership-aware and lane-aware

### Concurrency operationalization overlay

Parallelism must become explicit across the milestone ladder.

By the end of Milestone 1, the scheduler skeleton, worker-pool policy, and deterministic reducer contract must exist, even if only `R0` and `R1` use them initially.

By the end of Milestone 2, XML and patch work must run through the real scheduler with deterministic merge points rather than a serial placeholder path.

By the end of Milestone 3, mixed-zone entry and exit must be treated as explicit concurrency barriers, and surrounding preparation or validation work must already use the per-region concurrency contract.

By the end of Milestone 4, the cutoff-sensitive phases must have explicit serialized or conservative commit rules, not ad hoc thread usage.

By the end of Milestone 5, replay hit and replay miss paths must both be able to use parallel deterministic-region work where valid, and the diagnostics must report when that did or did not happen.

By the end of Milestone 6, benchmark evidence must show that multicore execution is materially active on the current machine for the Rust-owned deterministic region.

## 19. Acceptance gates

A feature is not complete when it merely runs. It is complete only when it passes the gates that match the architecture.

### Gate A: boundary correctness

The implementation must preserve the split-boundary model:

* runtime frontier start at `CreateModClasses`
* semantic cutoff after `GiveAllShortHashes()`
* delegated live tail after the cutoff

### Gate B: equivalence honesty

Any run claiming primary parity must satisfy the Category 3 equivalence contract under default parser semantics.

### Gate C: snapshot honesty

Any replay hit must be backed by valid fingerprints, valid blobs, and successful cutoff validation.

### Gate D: lane honesty

Every run must clearly identify whether it was Rust-owned, managed-assisted, or degraded.

### Gate E: package and bootstrap correctness

`rustystartup.core` must load as a normal RimWorld mod package with deterministic native-path resolution and structured failure reporting.

### Gate F: benchmark honesty

Performance claims must keep hit, miss, fallback, and tier state separated. No blended average is acceptable if it hides the path actually taken.

### Gate G: parallelism honesty

A build does not satisfy the implementation plan unless parallelism is operationalized honestly.

The Rust-owned deterministic region must use the real scheduler on multicore machines by default.

Forced-serial execution must exist for benchmarking and debugging, but it must not silently become the normal path.

Every region claimed as parallel or partitioned-parallel in the plan must either run that way in the implementation or emit a diagnostic reason why it did not.

Mixed-zone managed crossings must remain boundary-controlled and may not be parallelized by simply ignoring managed-thread constraints.

Benchmark evidence must be able to distinguish speed gained from replay, speed gained from parallel deterministic work, and speed gained from degraded or narrowed execution lanes.

## 20. Explicit non-goals for v1

To prevent scope drift, v1 does not attempt the following.

* claiming full parity under legacy parser mode
* taking ownership of post-cutoff callback-driven live-tail behavior
* treating Linux and macOS native loading as already verified locally when they are not
* collapsing mixed-zone managed assistance into an unobservable black box
* hiding fallback or degraded runs behind optimistic language
* portable snapshots
* a universal “all mods, all modes, all tails” claim beyond the evidence-supported boundary

## 21. Residual validation tasks that do not block the plan

A few items remain implementation-validation tasks rather than architecture blockers.

* final Linux and macOS native-load verification
* broader empirical validation of rare assembly-initializer or constructor edge cases
* expansion of Tier L and Tier X fresh benchmarks beyond the current planning baseline
* cleanup of any stale local config or metadata debris from old package identities

These do not change the architecture. They are validation work to perform while implementing the plan.

## 22. Final implementation directive

Build Rusty Startup v1 as a split-boundary, Rust-authoritative startup engine.

Own the deterministic startup region semantically through `GiveAllShortHashes()`.

Use the C# shell only as the explicit bootstrap, native boundary, and mixed-zone assistance layer.

Treat default parser semantics as the primary parity lane.

Implement machine-local snapshot and replay across `R0` through `R9`.

Prove fully resolved def equivalence at the cutoff.

Delegate the live tail after the cutoff.

Explain every important decision in diagnostics.

Do not drift from these boundaries.
