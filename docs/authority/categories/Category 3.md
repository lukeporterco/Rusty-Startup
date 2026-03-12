# Fully Resolved Def Equivalence Pack (Category 3, Merged)

## Executive Summary

This document defines the v1 correctness contract for Rusty Startup Category 3: **fully resolved def equivalence** against vanilla RimWorld 1.6.4633 under the canonical current planning baseline, with runtime baseline authority normalized to the current local planning revision.

The cutoff is fixed at one exact point in vanilla startup: immediately after `Verse.ShortHashGiver.GiveAllShortHashes()` returns inside `Verse.PlayDataLoader.DoPlayLoad()`, and before any `LongEventHandler.ExecuteWhenFinished(...)` callback body executes.

This is intentionally a resolved-def contract, not a full later-startup-state contract.

It includes:

* fully loaded mod XML
* patch-applied and inheritance-resolved XML meaning
* instantiated defs
* database population
* cross-reference resolution
* pre- and post-resolve implied defs
* required reset/finalize consequences needed by the cutoff
* final `DefOf` binding required by the cutoff
* completed short-hash assignment and reverse-map validity

It excludes:

* queued post-cutoff callback effects such as bios load, after-implied language injection, static constructor calls, atlas baking, GC/resource unload, and later runtime-only side effects

The canonical boundary model used by this document is the **split-boundary model**:

* the first arbitrary managed-execution boundary begins at `Verse.LoadedModManager.CreateModClasses`
* the semantic-equivalence cutoff remains after `Verse.ShortHashGiver.GiveAllShortHashes()`
* therefore Rusty Startup can still own semantic outcomes through the Category 3 cutoff, even though some phases inside that region may require managed-assisted execution or validation

The canonical parser policy used by this document is also fixed:

* Rusty Startup v1 targets default parser semantics, meaning the `Verse.DirectXmlToObjectNew.DefFromNodeNew` path
* the legacy parser path behind `legacy-xml-deserializer` is treated as a compatibility-detected managed-assisted fallback lane, not as a first-class Rust parity target for v1

Oracle mode is hybrid:

* **Strict order/index equivalence** for `ThingDef`, `RecipeDef`, `ThingCategoryDef`, `PawnKindDef`, and `ResearchProjectDef`
* **Semantic key equivalence** by `(runtimeType, defName)` for all other `DefDatabase<T>` types

## Canonical Boundary Definitions for Category 3

### First arbitrary managed-execution boundary

`Verse.LoadedModManager.CreateModClasses`

This is where arbitrary mod constructor code begins to execute during startup.

### Semantic-equivalence cutoff

Immediately after `Verse.ShortHashGiver.GiveAllShortHashes()` returns inside `Verse.PlayDataLoader.DoPlayLoad()`.

### Snapshot relevance

Category 3 defines the semantic state that the v1 snapshot model must preserve through its deterministic ownership boundary.

### Delegated live tail

Anything after the Category 3 cutoff that executes via `LongEventHandler.ExecuteWhenFinished(...)` or later runtime-coupled startup work is out of scope for Category 3 equivalence.

## Formal Definition of “Fully Resolved Def Equivalence”

Define `C3_Cutoff_State` as the in-memory state in a run after all of the following complete, in this order:

1. `LoadedModManager.LoadAllActiveMods()`

   * including mod content load
   * XML load
   * XML merge
   * patch application
   * inheritance resolution
   * def parse/process
   * patched-def staging and cleanup

2. `DefDatabase<T>.AddAllInMods()` for all non-abstract `Def` subclasses

3. `DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences(FailMode.Silent)` for non-implied defs

4. `DefOfHelper.RebindAllDefOfs(earlyTryMode: true)`

5. Early language/data operations and pre-resolve implied def generation

6. `DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences(FailMode.LogErrors)` for implied-def-created references, then `DirectXmlCrossRefLoader.Clear()`

7. `DefOfHelper.RebindAllDefOfs(earlyTryMode: false)`

8. `ResetStaticDataPre()`

9. Def resolve pass sequence:

   * `DefDatabase<ThingCategoryDef>.ResolveAllReferences(onlyExactlyMyType: true, parallel: true)`
   * `DefDatabase<RecipeDef>.ResolveAllReferences(onlyExactlyMyType: true, parallel: true)`
   * `DefDatabase<OtherDefType>.ResolveAllReferences(onlyExactlyMyType: true, parallel: false)` for all `Def` subclasses except `ThingDef`, `ThingCategoryDef`, and `RecipeDef`
   * `DefDatabase<ThingDef>.ResolveAllReferences()`

10. Post-resolve implied def generation

11. `ResetStaticDataPost()`

12. `ShortHashGiver.GiveAllShortHashes()` and per-type short-hash dictionary initialization through `DefDatabase<T>.InitializeShortHashDictionary()`

Two runs are `Category3Equivalent` if and only if all predicates below hold.

### P1: Database universe equality

The set of active def database types, meaning all non-abstract `Def` subclasses with database content by cutoff, is identical.

### P2: Strict-type sequence equality

For `T in {ThingDef, RecipeDef, ThingCategoryDef, PawnKindDef, ResearchProjectDef}`:

* `DefDatabase<T>.AllDefsListForReading.Count` is equal
* for each position `i`, defs match by:

  * exact runtime type full name
  * exact `defName`
  * exact `index` and index continuity
  * canonical semantic payload equality

### P3: Non-strict-type key equality

For all other database types:

* key-set equality uses `K = (runtimeTypeFullName, defName)`
* for each key, canonical semantic payload equality must hold
* list/index order differences are tolerated only for this non-strict set

### P4: Canonical semantic payload equality

Compare each def object as a canonical field graph with normalization rules:

* `Def` references compare by `(runtimeTypeFullName, defName)`, not object identity
* ordered `IEnumerable` fields compare in order when order is semantically defined by XML or resolve outputs
* dictionary/map fields compare by semantic key/value equality, independent of internal bucket order
* `ModContentPack` references compare by a stable identity tuple such as `(PackageId, PackageIdPlayerFacing, loadOrder)`
* runtime `Type` values compare by `AssemblyQualifiedName`

Explicitly exclude irrelevant/transient fields such as:

* `Def.debugRandomId`
* `Def.cachedLabelCap`
* object identity or pointer identity
* parser delegate caches or runtime parsing internals that are not part of def semantic state

### P5: Global resolved-def invariants

For every database type `T`:

* `(runtimeType, defName)` uniqueness must hold
* `shortHash` uniqueness within `T`, excluding `0`, must hold
* `DefDatabase<T>.GetByShortHash(def.shortHash)` must resolve to the same semantic def key

And:

* `DirectXmlCrossRefLoader` must have no pending unresolved refs retained in loader structures at cutoff because `Clear()` has already executed before the cutoff

## Object and Database State at the Cutoff

The compared state surface at cutoff is:

### 1. `DefDatabase<T>` internals for all active def types

* `AllDefsListForReading`
* name-map behavior such as `GetNamed` and `GetNamedSilentFail`
* short-hash map behavior such as `GetByShortHash`
* per-def `index`, `shortHash`, and `defNameHash` where startup-finalized

### 2. Def ownership and provenance state

* `LoadedModManager.RunningModsListForReading`
* `LoadedModManager.PatchedDefsForReading` effects after insertion into databases
* each def’s `modContentPack`, `fileName`, and `generated` status where semantically relevant

### 3. Derived global structures finalized by pre/post-reset passes up to the cutoff

* `ThingCategoryNodeDatabase`:

  * `RootNode`
  * parent/child category graph
  * category-to-child thing-def structure
  * category-to-child special-filter structure
  * `allThingCategoryNodes`
* `ResearchProjectDef` resolved coordinates after overlap handling and startup coordinate generation
* `RimWorld.BuildingProperties.FinalizeInit()` outcomes that affect smoothable/unsmoothed linkage consistency
* relevant `RimWorld.StatDef` post-reset/post-immutability state needed by the cutoff

### 4. Final binding state

* final `DefOf` static-field binding after `DefOfHelper.RebindAllDefOfs(earlyTryMode: false)`

## Resolution Guarantees at the Cutoff

By contract, all of the following must already be true at the cutoff.

### XML combination and patching are complete

* the unified `<Defs>` document is built from active mod XML assets
* patch operations are applied in active running-mod order and per-mod patch order

### XML inheritance is fully resolved

* `Name` and `ParentName` parent selection rules have been applied
* `Inherit="false"` overwrite semantics have been applied
* the resolved XML nodes are the ones used for def parsing

### Def object instantiation is complete for non-abstract nodes

* `Class=` override type resolution matches vanilla semantics
* field matching, aliases, and `IgnoreIfNoMatchingField` behavior match the canonical parser policy

### Conditional inclusion behavior is applied

* `MayRequire` and `MayRequireAnyOf` gate node, field, list, and dictionary inclusion exactly as required by the chosen parser semantics for v1

### Cross-references are resolved to the cutoff level

* object, list, and dictionary cross refs from XML parsing are resolved through both startup passes
* loader pending state is cleared after the second pass

### Def-level resolve lifecycle is complete to this cutoff

* `ResolveReferences()` pass sequence matches vanilla ordering constraints
* pre- and post-resolve implied defs are generated and inserted
* final `DefOf` rebind is completed
* short hashes are assigned and reverse maps initialized

## Parser Policy Incorporated Into Category 3

This merged document incorporates the parser-parity closure.

### Canonical parser target for v1

Rusty Startup v1 targets the default parser path:

* `Verse.DirectXmlToObjectNew.DefFromNodeNew`

### Legacy parser handling

If `legacy-xml-deserializer` is detected:

* Rusty Startup v1 must mark parser mode explicitly in diagnostics
* Rusty Startup v1 must not claim full Rust parser-parity ownership for that run
* Rusty Startup v1 must switch affected parse-equivalence assertions into managed-assisted or delegated handling

### Consequence for Category 3

The Category 3 equivalence oracle is bound to default parser semantics.

This means:

* full Category 3 parity claims are defined against the default parser path
* legacy mode is outside the primary parity contract for v1 and must be treated as a mode-aware fallback lane

## What Is Included

Included in Category 3 equivalence:

1. Full deterministic def pipeline through short-hash completion
2. Def database contents and lookup behavior at the cutoff
3. Resolved reference topology between defs
4. Implied and generated def products needed before the runtime tail
5. Required derived indexes and graphs created by startup reset/finalize calls up to the cutoff
6. The semantic consequences of managed-assisted mixed-zone phases, as long as the final cutoff state still matches the Category 3 oracle

## What Is Explicitly Excluded

Excluded from Category 3 equivalence:

1. All post-cutoff `ExecuteWhenFinished(...)` callback effects, including:

   * `SolidBioDatabase.LoadAllBios()`
   * `LoadedLanguage.InjectIntoData_AfterImpliedDefs()` and subsequent language-side cache clears
   * `StaticConstructorOnStartupUtility.CallAll()` and related late static-constructor consequences
   * atlas baking such as `GlobalTextureAtlasManager.BakeStaticAtlases()`
   * GC and resource-unload steps such as `GC.Collect`, `Resources.UnloadUnusedAssets`, and filesystem cache clears

2. Non-semantic runtime artifacts:

   * profiler markers and timings
   * log message ordering or counts
   * object identity or pointer identity
   * dictionary bucket layout or unrelated runtime hash-table internals

3. Legacy parser-path parity as a first-class Rust v1 target

## Equivalence Comparison Surface

Practical oracle definition:

1. Build a per-type snapshot surface at cutoff for all active `DefDatabase<T>` instances
2. Apply strict sequence comparison for strict types:

   * `ThingDef`
   * `RecipeDef`
   * `ThingCategoryDef`
   * `PawnKindDef`
   * `ResearchProjectDef`
3. Apply key-based semantic comparison for non-strict types using key `(runtimeTypeFullName, defName)`
4. Canonicalize field graphs with def-reference normalization
5. Validate global invariants such as uniqueness, reverse short-hash map consistency, and index continuity where strict
6. Validate required derived structures such as:

   * `ThingCategoryNodeDatabase`
   * research coordinate outcomes
   * building smoothing/finalize linkage
   * stat immutability/cacheability consequences required by the cutoff

### Semantic mismatch rules

Structural mismatch:

* missing or extra def keys
* wrong runtime type for the same def key
* strict-type order or index mismatch

Semantic mismatch:

* differing canonical field values
* differing normalized def-reference targets
* differing implied or generated def outputs

Invariant mismatch:

* short-hash collisions within a type
* reverse short-hash lookup resolving to the wrong semantic key
* invalid strict-type index continuity

### Acceptable internal differences

Acceptable internal differences include:

* `Def.debugRandomId`
* transient label/cache internals such as `cachedLabelCap`
* object identity and pointer identity
* parser internal delegate cache layout
* equivalent dictionary ordering differences where order is not semantically meaningful

## Mod-Relevant Consequences

If Category 3 equivalence holds, then the following mod-visible startup guarantees are expected to hold by the cutoff:

1. Mods that depend on database lookups by `defName` or by resolved references see the same def universe
2. Mods that rely on pre- and post-resolve implied defs see the same generated products
3. Mods that rely on final short-hash assignment and reverse lookup behavior see the same hash-visible state
4. Mods that rely on final `DefOf` binding by the cutoff see the same binding results
5. Mods that interact with strict-order-sensitive def families, especially `ThingDef`-adjacent systems, see the same ordered semantic state

This is the reason Category 3 is deep enough to matter without needing to extend all the way into later callback-driven startup state.

## Relationship to the Split-Boundary Model

Category 3 explicitly adopts the split-boundary reconciliation.

That means:

* `CreateModClasses` is the first arbitrary managed-execution boundary
* it is **not** the end of Category 3 ownership or correctness responsibility
* phases after `CreateModClasses` may still be inside Rusty Startup semantic ownership if the final cutoff state satisfies this equivalence contract

So Category 3 is compatible with a startup engine that:

* uses Rust-owned deterministic processing where possible
* uses managed-assisted execution in the mixed zone where startup methods may be Harmony-sensitive or constructor-influenced
* still asserts semantic success only at the Category 3 cutoff

## Residual Risks Worth Tracking

These do not reopen the Category 3 contract, but they remain implementation notes:

1. Unsampled long-tail mods could still expose additional parser-sensitive or startup-sensitive edge cases
2. Legacy parser mode remains outside the primary v1 parity target and requires explicit diagnostics and fallback behavior
3. Harmony modifications in the mixed zone may require proof-oriented mirroring or validation, not naive method replacement
4. Any future broadening of equivalence past short hashes would require a new category contract, not a silent extension of this one

## Canonical Wording Going Forward

Use these phrases consistently:

* **first arbitrary managed-execution boundary**: `LoadedModManager.CreateModClasses`
* **semantic-equivalence cutoff**: immediately after `ShortHashGiver.GiveAllShortHashes()`
* **Category 3 parser target**: default parser semantics via `DirectXmlToObjectNew`
* **legacy parser mode**: managed-assisted fallback lane, not primary v1 parity target

## Source Evidence Appendix

### Core local artifact examined

* `Assembly-CSharp.dll` from the local RimWorld 1.6 install

### Key vanilla types and methods examined

* `Verse.PlayDataLoader.DoPlayLoad`
* `Verse.LoadedModManager.LoadAllActiveMods`
* `Verse.LoadedModManager.ParseAndProcessXML`
* `Verse.DirectXmlToObjectNew.DefFromNodeNew`
* `Verse.DirectXmlLoader.DefFromNode`
* `Verse.DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences`
* `Verse.DirectXmlCrossRefLoader.Clear`
* `Verse.DefOfHelper.RebindAllDefOfs`
* `Verse.DefGenerator.GenerateImpliedDefs_PreResolve`
* `Verse.DefGenerator.GenerateImpliedDefs_PostResolve`
* `Verse.ShortHashGiver.GiveAllShortHashes`
* relevant `DefDatabase<T>.AddAllInMods`, `ResolveAllReferences`, and `InitializeShortHashDictionary` behavior

### Follow-up findings incorporated here

* split-boundary reconciliation:

  * runtime frontier starts at `CreateModClasses`
  * semantic cutoff remains after `GiveAllShortHashes()`
* parser/initializer closure:

  * default parser semantics are the canonical v1 target
  * legacy parser mode is explicitly mode-detected fallback territory
  * sampled module-initializer evidence does not move the ownership boundary

### Representative local mod evidence used in prior closure work

* local Workshop assemblies sampled for startup patching patterns and initializer risk
* local mod evidence supporting Harmony sensitivity inside the mixed zone without moving the Category 3 cutoff

### Representative commands used in the evidence-gathering process

* decompilation of `Assembly-CSharp.dll` core startup and parsing types
* decompilation of sampled local mod assemblies
* targeted searches for parser-path selection, startup method references, and module-initializer signatures
