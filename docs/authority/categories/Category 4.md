# Snapshot and Invalidation Surface Pack (Category 4, Merged)

## Executive Summary

This document defines the deterministic startup snapshot surface for Rusty Startup v1 under the canonical current planning baseline.

Canonical local planning baseline used here:

* runtime baseline authority: `RimWorld 1.6.4633 rev1261`
* static `Version.txt` mismatches are treated as superseded when fresh runtime evidence disagrees

Canonical boundary model used here:

* **split-boundary model**
* first arbitrary managed-execution boundary: `Verse.LoadedModManager.CreateModClasses`
* semantic-equivalence and snapshot cutoff: immediately after `Verse.ShortHashGiver.GiveAllShortHashes()` returns
* delegated live tail begins only after that cutoff in `LongEventHandler.ExecuteWhenFinished(...)` callback work and later runtime-coupled startup steps

Canonical parser policy used here:

* Rusty Startup v1 targets default parser semantics via `Verse.DirectXmlToObjectNew.DefFromNodeNew`
* legacy parser mode behind `legacy-xml-deserializer` is not a first-class Rust parity target for v1
* legacy mode must be explicitly fingerprinted and routed into managed-assisted or fallback handling

Core result:

* the snapshot-worthy deterministic region is a dependency-ordered pack `R0` through `R9`
* invalidation must be region-aware and input-aware, not monolithic
* machine-local inputs must be fingerprinted separately from portable content inputs
* replay safety requires both blob integrity validation and Category 3 fully resolved def-equivalence validation at the cutoff

## Snapshot Scope

### Snapshot Contract (Public)

The deterministic startup snapshot contract for v1 is:

* region IDs: `R0` through `R9`
* dependency order: `R0 -> R1 -> R2 -> R3 -> R4 -> R5 -> R6 -> R7 -> R8 -> R9`
* required fingerprint classes per region: byte hash, structural hash, metadata hash fallback, path identity, environment keys, and language keys
* correctness target: restored state must be equivalent to vanilla fully resolved def semantics at the Category 3 cutoff

### Region Definitions (Snapshot-Worthy Products Up To Cutoff)

| Region | Product                                                                  | Primary RimWorld Evidence                                                                                                 | Snapshot Worthiness | Notes                                                                                                                                                          |
| ------ | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- | ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `R0`   | Active mod set, mod order, and resolved content roots per mod            | `Verse.ModsConfig.ActiveModsInLoadOrder`, `Verse.LoadedModManager.InitializeMods`, `Verse.ModContentPack.InitLoadFolders` | Required            | Includes `LoadFolders.xml` version choice, fallback behavior, and conditional folder activation such as `IfModActive`, `IfModActiveAll`, and `IfModNotActive`. |
| `R1`   | Raw XML asset discovery set after folder resolution                      | `Verse.ModContentPack.LoadDefs`, `Verse.ModContentPack.LoadPatches`, `Verse.DirectXmlLoader.XmlAssetsInModFolder`         | Required            | Deterministic file set, ordering, source bytes, and selected parser mode are foundational inputs to all downstream regions.                                    |
| `R2`   | Unified XML document with patch operations applied                       | `Verse.LoadedModManager.CombineIntoUnifiedXML`, `Verse.LoadedModManager.ApplyPatches`                                     | Required            | Represents post-patch semantic XML source before inheritance resolution.                                                                                       |
| `R3`   | Inheritance-resolved XML node graph                                      | `Verse.XmlInheritance.TryRegister`, `Verse.XmlInheritance.Resolve`                                                        | Required            | Includes conditional node inclusion via `MayRequire` and `MayRequireAnyOf` gating paths under the canonical parser policy.                                     |
| `R4`   | Parsed defs assigned to mod or patch buckets and inserted by def type    | `Verse.LoadedModManager.ParseAndProcessXML`, `Verse.DefDatabase<T>.AddAllInMods`                                          | Required            | Includes overwrite ordering by mod overwrite priority and running-mod order.                                                                                   |
| `R5`   | Cross-reference resolution state for non-implied and implied defs        | `Verse.DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences`, `Verse.DirectXmlCrossRefLoader.Clear`                    | Required            | Gated refs and two-pass xref closure are part of deterministic startup meaning.                                                                                |
| `R6`   | Final resolved def graph including implied defs and reference resolution | `RimWorld.DefGenerator.GenerateImpliedDefs_PreResolve/PostResolve`, `DefDatabase<T>.ResolveAllReferences`                 | Required            | This is the core equivalence target surface.                                                                                                                   |
| `R7`   | Final `DefOf` binding state relevant to deterministic identity lookup    | `RimWorld.DefOfHelper.RebindAllDefOfs`                                                                                    | Required            | Must match the final def graph identity semantics by the cutoff.                                                                                               |
| `R8`   | TKey parse/build mappings                                                | `Verse.TKeySystem.Parse`, `Verse.TKeySystem.BuildMappings`                                                                | Required            | Affects translation key indirection and startup data binding behavior before the live tail.                                                                    |
| `R9`   | Final short-hash assignments and per-type short-hash dictionaries        | `Verse.ShortHashGiver.GiveAllShortHashes`                                                                                 | Required            | `defName`-sorted assignment defines stable runtime identity hashes per def type.                                                                               |

### Cutoff Boundary

Snapshot ownership stops at completion of:

* `Verse.ShortHashGiver.GiveAllShortHashes()`

The snapshot surface explicitly excludes queued live-tail work beginning with callback execution from:

* `Verse.LongEventHandler.ExecuteWhenFinished(...)`

### Relationship to the Split-Boundary Model

The snapshot model deliberately spans a region that begins before and continues after the first arbitrary managed-execution boundary.

That means:

* the mixed zone starting at `CreateModClasses` does not terminate snapshot ownership
* some phases inside `R4` through `R9` may require managed-assisted ownership or managed-assisted validation
* the snapshot system is anchored to semantic products, not to a simplistic “no managed code has executed” rule

## Snapshot Input Surface

Every input below can change deterministic startup meaning up to the cutoff.

| Input Class                                | Concrete Local Surface                                                      | Portable vs Machine-Local                                                  | Role in Snapshot Validity                                                                            |
| ------------------------------------------ | --------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| Canonical runtime build                    | local runtime banner baseline `1.6.4633 rev1261`                            | Machine-local planning baseline over portable build identity               | Governs startup code semantics, version-specific load-folder selection, and comparison validity.     |
| Static install metadata                    | `Version.txt` and related install metadata                                  | Portable install metadata, but lower authority than fresh runtime evidence | Still useful as a secondary fingerprint, but not authoritative when runtime evidence disagrees.      |
| Managed game assembly semantics            | `Assembly-CSharp.dll` and relevant managed startup assemblies               | Portable per installation build                                            | Startup method bodies, parser behavior, and def pipeline logic come from these assemblies.           |
| Active mod IDs and order                   | `ModsConfig.xml` active-mod list                                            | Machine-local selection of portable content                                | Drives `InitializeMods`, overwrite precedence, content roots, patch order, and mod-context gating.   |
| Known expansion state                      | `ModsConfig.xml` known-expansions and installed official expansions         | Mixed                                                                      | Changes activation checks and `MayRequire` outcomes.                                                 |
| Mod metadata                               | `About/About.xml`, package IDs, supported versions, dependencies            | Portable                                                                   | Affects mod discovery, activation rules, dependency closure, and content-root selection.             |
| Mod root resolution                        | workshop roots, local mods, official data roots                             | Machine-local paths over portable content                                  | Path roots determine discovery and selected content.                                                 |
| Load-folder policy                         | `LoadFolders.xml`, version fallback, conditional load gates                 | Portable rule + machine-local active set                                   | Changes which XML and other content roots are included.                                              |
| Raw Def XML bytes                          | `Defs/**/*.xml` within selected content roots                               | Portable content                                                           | Core source input to unified XML and downstream parse semantics.                                     |
| Raw Patch XML bytes and ordering           | `Patches/**/*.xml` and operation order                                      | Portable content                                                           | Changes patch graph and final unified XML meaning.                                                   |
| XML inheritance and conditional attributes | `ParentName`, `MayRequire`, `MayRequireAnyOf`, and related attributes       | Portable content + active-mod context                                      | Affects which nodes and fields exist before parsing.                                                 |
| Parser mode                                | default parser vs `legacy-xml-deserializer` path                            | Machine-local startup mode                                                 | Must be fingerprinted explicitly because it changes parse semantics and parity claims.               |
| Cross-ref request graph                    | fields and nodes registering wanted refs                                    | Derived from portable content + active context                             | Changes xref closure and resolved def graph.                                                         |
| Implied-def generation semantics           | `DefGenerator` code paths + current graph inputs                            | Build-semantic + content-driven                                            | Affects generated defs and downstream resolution.                                                    |
| Def overwrite precedence                   | overwrite priority + running-mod order                                      | Derived from active set/order                                              | Alters which defs survive and in what database order.                                                |
| `DefOf` binding surface                    | loaded def names and `[DefOf]` types                                        | Build-semantic + content-driven                                            | Must match at the final cutoff.                                                                      |
| TKey map source                            | unified XML parse + TKey mapping rules                                      | Derived deterministic                                                      | Affects startup key binding and mapping behavior.                                                    |
| KeyPrefs                                   | local `KeyPrefs.xml`                                                        | Machine-local user config                                                  | Startup state before the cutoff may depend on current key prefs.                                     |
| Prefs language                             | local `Prefs.xml` language selection or auto-select result                  | Machine-local                                                              | Selects language resources and influences pre-cutoff language-sensitive startup state.               |
| Language content bytes                     | `Languages/**` content roots                                                | Portable content selected by machine-local language key                    | Changes TKey and data-injection inputs around the cutoff.                                            |
| Path roots                                 | install path, save-data path, workshop path, command-line `-savedatafolder` | Machine-local environment                                                  | Changes discovery and machine-local file identity.                                                   |
| Platform key                               | runtime platform and bundle suffix (`_win`, `_linux`, `_mac`)               | Machine-local environment                                                  | Affects platform-sensitive resource selection.                                                       |
| Loaded mod assemblies                      | `Assemblies/*.dll` bytes and load identities                                | Portable content + machine-local load context                              | Constructors, Harmony activity, and managed-assisted mixed-zone semantics can depend on these bytes. |
| Startup-sensitive managed patch state      | constructor-time Harmony setup and startup-target patch presence            | Derived from loaded assemblies and runtime instantiation                   | Must be treated as part of the mixed-zone validity model where relevant.                             |

## Invalidation Graph

### Invalidation Rules

* Invalidate from the **earliest affected region** and rebuild the full downstream closure to `R9`.
* Closure rule: if region `Rx` is invalid, then regions `Rx+1 ... R9` are invalid.
* If integrity validation fails for a region blob, treat that region as invalid regardless of key matches.
* If parser mode changes, invalidate from the earliest parser-sensitive region.
* If canonical runtime baseline changes, invalidate the full region chain unless a future version-compatibility bridge explicitly says otherwise.

### Input To Region Invalidation Mapping

| Input Change                                           | Earliest Invalid Region | Downstream Closure             | Why                                                                                                                  |
| ------------------------------------------------------ | ----------------------- | ------------------------------ | -------------------------------------------------------------------------------------------------------------------- |
| Canonical runtime build changed                        | `R0`                    | `R0..R9`                       | Build/revision changes can alter startup code semantics, parser behavior, implied-def generation, and resolve logic. |
| Active mod list changed (add/remove/reorder)           | `R0`                    | `R0..R9`                       | Changes running mods, content roots, patch set/order, and overwrite precedence.                                      |
| Known expansion activation changed                     | `R0`                    | `R0..R9`                       | Alters activation checks and `MayRequire` outcomes.                                                                  |
| Mod root path moved, missing, or repointed             | `R0`                    | `R0..R9`                       | Changes resolved content roots and discovery results.                                                                |
| `LoadFolders.xml` changed                              | `R0`                    | `R0..R9`                       | Changes selected folder sets and all downstream XML/content inputs.                                                  |
| Conditional load-folder gate state changed             | `R0`                    | `R0..R9`                       | Same mod can expose different content roots under a different active-mod context.                                    |
| Raw Def XML bytes changed                              | `R1`                    | `R1..R9`                       | Changes unified XML and everything downstream.                                                                       |
| Raw Patch XML bytes or order changed                   | `R1`                    | `R1..R9`                       | Changes patch graph and final semantic XML source.                                                                   |
| Parser mode changed (default vs legacy)                | `R3`                    | `R3..R9`                       | Changes def materialization semantics and therefore all later semantic products.                                     |
| XML inheritance or conditional gating outcome changed  | `R3`                    | `R3..R9`                       | Alters final resolved XML node graph before parsing.                                                                 |
| Mod assembly bytes changed                             | `R0`                    | `R0..R9`                       | Constructor-time and patch-time startup behavior in the mixed zone can change, altering semantic outcomes.           |
| Constructor-time Harmony patch state changed           | `R0`                    | `R0..R9`                       | May alter methods executed inside the mixed zone after `CreateModClasses`.                                           |
| Def overwrite priority changed                         | `R4`                    | `R4..R9`                       | Alters surviving defs and list/index order.                                                                          |
| Cross-ref registration semantics changed               | `R5`                    | `R5..R9`                       | Alters resolved graph.                                                                                               |
| Implied-def generation logic changed                   | `R6`                    | `R6..R9`                       | Alters generated defs and final graph.                                                                               |
| `DefOf` binding-relevant names or types changed        | `R7`                    | `R7..R9`                       | Changes final static binding state.                                                                                  |
| TKey source changed                                    | `R8`                    | `R8..R9`                       | Changes translation-key mappings before cutoff finalization.                                                         |
| Language selection changed                             | `R8`                    | `R8..R9`                       | Affects language-sensitive pre-cutoff data and key mapping semantics.                                                |
| Short-hash algorithm or contributing inputs changed    | `R9`                    | `R9`                           | Changes final short-hash identity surface.                                                                           |
| Machine-local path root changed without content change | `R0`                    | `R0..R9` or narrower in future | Path identity is machine-local and can affect lookup, root selection, or content provenance semantics.               |

### Dependency Closure Summary

* `R0` changes invalidate everything.
* `R1` changes invalidate XML-derived and later semantic regions.
* `R3` changes invalidate parse and all later graph regions.
* `R4` changes invalidate all resolved-def and hash regions.
* `R6` changes invalidate binding, TKey-finalization impacts, and short hashes.
* `R9` changes are terminal and local if nothing upstream changed.

## Fingerprinting Recommendations

### Fingerprint Classes

Use the following fingerprint types:

* **byte hash** for exact file bytes where startup semantics depend on exact content
* **structural hash** for canonicalized XML or semantic graphs where internal formatting should not matter
* **metadata hash fallback** only where byte hashing is too costly and semantics are proven not to depend on exact bytes
* **path identity** for machine-local roots and resolved content provenance
* **environment keys** for platform and save-data-root-sensitive behavior
* **language keys** for language selection and language-content inputs
* **mode keys** for parser mode and mixed-zone managed-assistance mode

### Recommended Fingerprint Per Input Class

* Game/runtime build: runtime banner string + key managed assembly byte hash
* Active mod set/order: ordered package-ID list + per-mod stable metadata identity
* Mod roots: canonical normalized absolute path identity + selected content-root list
* `LoadFolders.xml`: byte hash plus resolved selected-folder structural identity
* Def/Patch XML: byte hash and, where useful, structural hash after normalization
* Parser mode: explicit mode bit (`default-new` vs `legacy`) stored in snapshot metadata
* Mod assemblies: byte hash plus assembly identity tuple
* Language selection: explicit selected language key + relevant language content fingerprints
* Machine-local path roots: normalized path identity, not just relative names
* TKey source: structural hash of generated mapping source

### Fingerprint Rules For Mixed-Zone Safety

Because the split-boundary model allows semantic ownership across a runtime-sensitive mixed zone, the snapshot metadata should also record:

* which loaded assemblies contributed to constructor-time startup participation
* whether startup-sensitive Harmony or startup-method patching was detected or expected in the run mode
* whether the replay lane is pure deterministic or managed-assisted

This is not because every run must invalidate on any patch presence. It is because replay safety and diagnostics depend on knowing which semantic lane produced the snapshot.

## Excluded or Unsafe Snapshot Regions

These are outside v1 snapshot ownership:

1. Any callback-driven work executed only after the Category 3 cutoff:

   * `SolidBioDatabase.LoadAllBios()`
   * post-implied language injection
   * `StaticConstructorOnStartupUtility.CallAll()` when executed in the post-cutoff callback tail
   * atlas baking
   * GC/resource unload operations

2. Unity-coupled or object-lifecycle-sensitive content behavior:

   * asset-bundle load/unload side effects
   * Unity texture/object destroy/unload state
   * UI/root frame progression state

3. Legacy parser parity as a primary Rust snapshot lane

   * legacy mode may still be supported via managed-assisted fallback, but it is not part of the primary v1 Rust semantic snapshot target

4. Arbitrary live runtime state not required to satisfy the fully resolved def-equivalence contract

## Replay Safety Conditions

A snapshot is safe to restore only if all of the following are true:

1. All required region blobs pass integrity validation.
2. All required fingerprint classes match for the restored lane.
3. The canonical runtime build and planning revision are compatible with the snapshot.
4. Parser mode matches exactly.
5. Active mod order and selected content roots match exactly.
6. The selected language and machine-local path keys match according to the stored environment model.
7. Any mixed-zone managed-assistance assumptions recorded in the snapshot metadata remain valid.
8. Restored state passes the Category 3 fully resolved def-equivalence oracle or a validated equivalent restoration check at the cutoff.

### Replay Modes

Rusty Startup should conceptually support these replay modes:

* **Pure deterministic replay lane**

  * used when the snapshot was produced and restored under the canonical default parser lane with no extra managed-assistance requirement beyond mirrored shell behavior

* **Managed-assisted replay lane**

  * used when the mixed zone required managed participation or when startup-sensitive patching/constructor context is part of the semantic lane

* **Fallback lane**

  * used when parser mode, build, or mixed-zone validity prevents claiming primary parity ownership

## Corruption and Partial Invalidation Cases

### Blob Corruption

If any region blob fails integrity validation:

* mark that region invalid
* discard that region and every downstream region
* do not attempt partial trust inside the corrupted region

### Partial Upstream Invalidation

If only an upstream region is invalid, Rusty Startup should retain any upstream-independent metadata where safe, but must rebuild the full downstream closure from the earliest invalid region.

### Parser-Mode Drift

If parser mode differs from the snapshot:

* invalidate from `R3` through `R9`
* do not silently reuse parser-sensitive semantic products
* route into managed-assisted or fallback behavior if the run is in legacy mode

### Mixed-Zone Lane Drift

If the snapshot metadata says the snapshot was produced under a managed-assisted lane, but the current run cannot validate that lane’s assumptions:

* invalidate from the earliest affected mixed-zone region, conservatively `R0` unless future implementation narrows it
* do not claim pure deterministic replay

### Build/Revision Drift

If runtime revision differs from the canonical build stored in snapshot metadata:

* invalidate `R0..R9`
* only a future explicit cross-revision compatibility policy may override this

## Relationship to Category 3

Category 4 is subordinate to the Category 3 equivalence contract.

That means:

* the point of restoring `R0..R9` is to recreate the exact semantic state required by Category 3
* replay safety is not merely “cache hit” safety
* replay safety means “safe restoration of fully resolved def-equivalent startup state at the cutoff”

So any future implementation detail that improves cache speed but weakens the Category 3 equivalence guarantee is invalid for this category.

## Relationship to the Runtime Frontier

The runtime frontier start at `CreateModClasses` does not shrink the snapshot scope.

Instead, it changes how the snapshot model must reason about validity:

* regions after `CreateModClasses` may still be snapshot-worthy
* but the replay metadata must preserve whether the semantic lane was pure deterministic or managed-assisted
* this is the practical consequence of the split-boundary model for Category 4

## Residual Risks Worth Tracking

These do not reopen the Category 4 design, but they remain implementation notes:

1. Long-tail unsampled mods may reveal narrower invalidation edges inside the mixed zone than the current conservative model captures.
2. Future cross-platform native-loading validation may refine which machine-local environment keys are required.
3. Legacy parser mode remains a secondary lane and may require stronger managed-assisted replay constraints if deeper support is later expanded.
4. If future implementation extends the semantic cutoff past short hashes, Category 4 would need a new region model rather than a silent expansion of `R0..R9`.

## Canonical Wording Going Forward

Use these phrases consistently:

* **runtime frontier start**: `LoadedModManager.CreateModClasses`
* **semantic/snapshot cutoff**: immediately after `ShortHashGiver.GiveAllShortHashes()`
* **primary parser lane**: default parser semantics via `DirectXmlToObjectNew`
* **legacy parser lane**: managed-assisted fallback, not primary parity target
* **snapshot correctness target**: Category 3 fully resolved def equivalence

## Source Evidence Appendix

### Core local artifacts examined

* local RimWorld managed startup assemblies, especially `Assembly-CSharp.dll`
* local runtime version evidence establishing the canonical current planning revision
* local config, mod-root, and cache-path surfaces used to identify machine-local invalidation inputs

### Key vanilla types and methods examined

* `Verse.ModsConfig.ActiveModsInLoadOrder`
* `Verse.LoadedModManager.InitializeMods`
* `Verse.ModContentPack.InitLoadFolders`
* `Verse.ModContentPack.LoadDefs`
* `Verse.ModContentPack.LoadPatches`
* `Verse.DirectXmlLoader.XmlAssetsInModFolder`
* `Verse.LoadedModManager.CombineIntoUnifiedXML`
* `Verse.LoadedModManager.ApplyPatches`
* `Verse.XmlInheritance.TryRegister`
* `Verse.XmlInheritance.Resolve`
* `Verse.LoadedModManager.ParseAndProcessXML`
* `Verse.DefDatabase<T>.AddAllInMods`
* `Verse.DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences`
* `Verse.DirectXmlCrossRefLoader.Clear`
* `RimWorld.DefGenerator.GenerateImpliedDefs_PreResolve`
* `RimWorld.DefGenerator.GenerateImpliedDefs_PostResolve`
* `RimWorld.DefOfHelper.RebindAllDefOfs`
* `Verse.TKeySystem.Parse`
* `Verse.TKeySystem.BuildMappings`
* `Verse.ShortHashGiver.GiveAllShortHashes`
* `Verse.LongEventHandler.ExecuteWhenFinished`
* `Verse.PlayDataLoader.DoPlayLoad`
* `Verse.GenCommandLine.CommandLineArgPassed`
* `Verse.DirectXmlToObjectNew.DefFromNodeNew`
* `Verse.DirectXmlLoader.DefFromNode`
* `Verse.ModAssemblyHandler.ReloadAll`

### Follow-up findings incorporated here

* split-boundary reconciliation:

  * first arbitrary managed-execution boundary begins at `CreateModClasses`
  * semantic/snapshot cutoff remains after `GiveAllShortHashes()`
* parser/initializer closure:

  * default parser semantics are the canonical v1 target
  * legacy parser mode must be fingerprinted and treated as a fallback lane
  * sampled module-initializer evidence does not move the snapshot boundary
* baseline reconciliation:

  * canonical local planning baseline is `1.6.4633 rev1261`
  * runtime evidence overrides conflicting static version metadata for planning/gating on this machine

### Representative local environment surfaces used

* local `ModsConfig.xml`
* local `Prefs.xml`
* local `KeyPrefs.xml`
* install path, workshop path, save-data path, and related machine-local roots
* local RustyStartup cache path conventions and bundle artifact layout where relevant to historical cache shape

### Representative commands used in the evidence-gathering process

* decompilation of local RimWorld startup types
* inspection of local config and path-root surfaces
* searches for parser-mode selection, startup method references, and version/build evidence
* inspection of local runtime logs for canonical build reconciliation
