# RimWorld 1.6 Startup Ownership Pack Audit (Category 1, Merged)

## Executive Summary

* **FACT:** Startup enters managed code at `Verse.Root_Entry::Start` in `Assembly-CSharp.dll`, which immediately calls `Verse.Root::Start`.
* **FACT:** The startup-critical mod/def chain is:

  * `Root::CheckGlobalInit`
  * `Root::Start`
  * `Root::Update`
  * `LongEventHandler::LongEventsUpdate`
  * `PlayDataLoader::LoadAllPlayData(false)`
  * `PlayDataLoader::DoPlayLoad`
  * `LoadedModManager::LoadAllActiveMods`
  * XML, patch, def, cross-ref, implied-def, resolve, and short-hash phases.
* **FACT:** XML parser path selection is command-line gated in `LoadedModManager::ParseAndProcessXML`:

  * `legacy-xml-deserializer` present: `DirectXmlLoader::DefFromNode`
  * otherwise default path: `DirectXmlToObjectNew::DefFromNodeNew`
* **FACT:** DefDatabase population is reflective and parallelized through generic `DefDatabase<T>.AddAllInMods` calls.
* **FACT:** Cross-reference resolution is multi-pass through `DirectXmlCrossRefLoader::ResolveAllWantedCrossReferences`, with later explicit clearing.
* **FACT:** Short-hash assignment is a late explicit startup phase through `ShortHashGiver::GiveAllShortHashes()`.
* **FACT:** Key mutable global structures observed in startup include:

  * `LoadedModManager.runningMods`
  * `LoadedModManager.runningModClasses`
  * `LoadedModManager.patchedDefs`
  * `DefDatabase<T>.defsList`, `defsByName`, `defsByShortHash`
  * `DirectXmlCrossRefLoader.wantedRefs`, `wantedListDictRefs`
  * `ModsConfig.data` and active-mod caches
  * `ShortHashGiver.takenHashesPerDeftype`
  * `LongEventHandler.eventQueue`, `currentEvent`, `toExecuteWhenFinished`
* **FACT:** The local compatibility corpus is large enough to justify evidence-based ownership decisions across a real mod ecosystem.
* **FORCED CONCLUSION:** The correct boundary model is a **split-boundary model**:

  * the first clearly evidenced arbitrary managed-execution boundary begins at `LoadedModManager.CreateModClasses`
  * the semantic-equivalence and snapshot cutoff still extends through `ShortHashGiver.GiveAllShortHashes()`
  * therefore Rusty Startup v1 can still own semantic outcomes through the resolved-def cutoff using a mixed strategy: Rust-owned where deterministic, managed-assisted where startup methods may execute under Harmony or constructor-driven effects.

## Scope Boundary

* **FACT:** `Verse.Root` inherits `UnityEngine.MonoBehaviour` and `Verse.Root_Entry` inherits `Verse.Root`.
* **INFERENCE:** Unity scene/bootstrap behavior before the first `Root_Entry::Start` callback is outside the visible managed startup region and should be treated as a low-visibility pre-entry boundary.

## Exact Startup Entry Chain

### Entry call order

1. **Unity calls `Verse.Root_Entry::Start()`**
2. `Root_Entry::Start` calls `Root::Start`
3. `Root::Start` performs early scene and global setup, then calls `Root::CheckGlobalInit`
4. `Root::CheckGlobalInit`:

   * performs first-time global initialization
   * logs command-line context
   * initializes system-level utilities
   * queues a long event for `StaticConstructorOnStartupUtility::CallAll`
5. Back in `Root::Start`:

   * if `PlayDataLoader.Loaded` is false, queues `PlayDataLoader::LoadAllPlayData(false)`
   * queues interface/sound root initialization work
6. `Root_Entry::Update` calls `Root::Update`
7. `Root::Update` calls `LongEventHandler::LongEventsUpdate(ref flag)`
8. `LongEventsUpdate` dequeues and dispatches queued long events
9. The queued startup long events execute in FIFO order:

   * `StaticConstructorOnStartupUtility::CallAll`
   * `PlayDataLoader::LoadAllPlayData(false)`
   * interface initialization callback

### `PlayDataLoader` startup branch

10. `PlayDataLoader::LoadAllPlayData(false)`:

    * begins profiling
    * calls `DoPlayLoad()`
    * sets `loadedInt = true` on success
    * contains crash-recovery logic including mod reset and cross-ref clearing
11. `PlayDataLoader::DoPlayLoad()` proceeds through the startup-critical sequence:

    * clears graphics and static queues
    * calls `LoadedModManager::LoadAllActiveMods(hotReload:false)`
    * copies defs into global databases via `DefDatabase<T>.AddAllInMods`
    * resolves non-implied cross references
    * performs early `DefOf` rebinding
    * performs language/key-related injections and resets
    * generates implied defs pre-resolve
    * resolves cross refs again and clears them
    * performs final `DefOf` rebinding
    * runs pre-resolve static resets
    * resolves references across def databases
    * generates implied defs post-resolve
    * runs post-resolve resets
    * performs dev-mode error checks where applicable
    * initializes key preferences
    * assigns short hashes via `ShortHashGiver::GiveAllShortHashes()`
    * queues post-finish callbacks such as bios, later language injection, static constructors, atlas baking, GC, and resource unloads

### `LoadedModManager::LoadAllActiveMods` chain

12. `XmlInheritance::Clear()`
13. `InitializeMods()`
14. `LoadModContent(hotReload)`
15. `CreateModClasses()`
16. `LoadModXML(hotReload)`
17. `CombineIntoUnifiedXML(...)`
18. `TKeySystem::Clear()` and `TKeySystem::Parse(unifiedXml)`
19. `ErrorCheckPatches()` on cold load
20. `ApplyPatches(unifiedXml, assetlookup)`
21. `ParseAndProcessXML(unifiedXml, assetlookup, hotReload)`
22. `ClearCachedPatches()`
23. `XmlInheritance::Clear()`

## Boundary Definitions

### First arbitrary managed-execution boundary

This begins at `LoadedModManager.CreateModClasses`.

Reason: this is the first clearly evidenced point where arbitrary `Verse.Mod` subclass constructors are instantiated, allowing constructor-time mod behavior, Harmony setup, and other startup-visible managed side effects to begin.

### Runtime frontier

The runtime-sensitive mixed zone begins at `CreateModClasses`.

This does **not** mean semantic ownership ends there. It means that from this point forward, some phases can no longer be treated as purely isolated vanilla logic with no managed participation risk.

### Semantic-equivalence cutoff

The semantic-equivalence cutoff remains immediately after `ShortHashGiver.GiveAllShortHashes()` returns inside `PlayDataLoader::DoPlayLoad()`.

That is the authoritative cutoff for this Category 1 document.

### Snapshot ownership boundary

The snapshot ownership boundary includes startup products through short-hash completion, including:

* loaded and patched defs
* inheritance-resolved XML products
* materialized def objects
* cross-reference state after resolution
* implied defs
* final resolved references
* final `DefOf` binding state required by the cutoff
* short-hash completion state

### Delegated live tail

The delegated live tail begins only after the semantic cutoff, in queued `LongEventHandler.ExecuteWhenFinished(...)` callbacks and later work such as:

* `SolidBioDatabase.LoadAllBios`
* post-implied language injection
* `StaticConstructorOnStartupUtility.CallAll`
* atlas baking
* GC and resource unload work

## Contradiction Resolution

A prior contradiction existed because one report treated `CreateModClasses` as the end of ownership, while other reports treated ownership as extending through XML, defs, cross-ref resolution, implied defs, and short hashes.

That contradiction is resolved by separating two different boundaries that were previously described with one overloaded word.

The correct interpretation is:

* `CreateModClasses` marks the start of the runtime-sensitive mixed zone
* `GiveAllShortHashes()` marks the semantic-equivalence and snapshot cutoff

This means Rusty Startup can still own the semantic result through the cutoff, even if some phases inside that region require managed assistance or must be treated as Harmony-sensitive.

## Phase-by-Phase Startup Ownership Map

Replaceability classes used below:

* **Rust-owned**
* **managed-assisted but still within Rust semantic ownership**
* **delegated live-tail**
* **excluded from v1**

| Phase Name                          | Owning Vanilla Class/Method(s)                                                                    | Inputs                                         | Outputs                                                    | Side Effects                                                      | Global State Mutated                                      | Order Sensitivity | Deterministic vs Runtime-Bound                                      | Ownership / Replaceability                                |
| ----------------------------------- | ------------------------------------------------------------------------------------------------- | ---------------------------------------------- | ---------------------------------------------------------- | ----------------------------------------------------------------- | --------------------------------------------------------- | ----------------- | ------------------------------------------------------------------- | --------------------------------------------------------- |
| Unity bootstrap to managed root     | Unity engine -> `Verse.Root_Entry::Start`                                                         | Unity scene objects, engine lifecycle          | managed startup begins                                     | engine lifecycle dispatch                                         | pre-entry engine state                                    | High              | runtime-bound                                                       | excluded from v1 / mirrorable boundary                    |
| Root global init gate               | `Verse.Root::CheckGlobalInit`                                                                     | command-line args, local machine state, prefs  | global-init complete marker; long event queued             | system init, prefs init, log hookup                               | `Root.globalInitDone`, `LongEventHandler.eventQueue`      | High              | mixed but mostly deterministic from local machine state             | managed-assisted but still within Rust semantic ownership |
| Long event queue construction       | `LongEventHandler::QueueLongEvent`                                                                | action, flags, text                            | queued long events                                         | queue append                                                      | `LongEventHandler.eventQueue`                             | High              | deterministic given same enqueue sequence                           | Rust-owned                                                |
| Long event dispatch                 | `Root::Update` -> `LongEventHandler::LongEventsUpdate`                                            | frame ticks, queue state                       | event progression and completion                           | worker-thread dispatch, callbacks, scene load interactions        | `currentEvent`, `eventThread`, `toExecuteWhenFinished`    | High              | runtime-bound                                                       | managed-assisted boundary infrastructure                  |
| Mod config load/cache               | `ModsConfig` loading and recache methods                                                          | `ModsConfig.xml`, installed mods list          | ordered active mod list and flags                          | file read/write fallback/reset behavior                           | `ModsConfig.data`, active-mod caches                      | High              | mostly deterministic with filesystem inputs                         | Rust-owned                                                |
| Initialize running mod packs        | `LoadedModManager.InitializeMods`                                                                 | active mods in load order                      | `ModContentPack` instances                                 | missing-mod deactivation, logging                                 | `LoadedModManager.runningMods` and related config state   | High              | deterministic with filesystem and config                            | Rust-owned                                                |
| Assembly/content reload kickoff     | `LoadedModManager.LoadModContent`, `ModContentPack.ReloadContent`, `ModAssemblyHandler.ReloadAll` | running mods, mod folders                      | loaded assemblies, deferred content reload state           | `Assembly.LoadFrom`, resolver hooks, possible static side effects | assembly lists, resolver state                            | High              | runtime-sensitive                                                   | managed-assisted but still within Rust semantic ownership |
| Instantiate mod classes             | `LoadedModManager.CreateModClasses`                                                               | loaded assemblies and `runningMods`            | `Verse.Mod` instances                                      | arbitrary mod constructors execute                                | `LoadedModManager.runningModClasses`                      | High              | runtime-sensitive, arbitrary managed execution begins here          | managed-assisted but still within Rust semantic ownership |
| XML asset discovery/load            | `LoadedModManager.LoadModXML`, `ModContentPack.LoadDefs`                                          | active mod roots, versioned folders, XML files | `LoadableXmlAsset` sequences                               | file I/O, content overlay behavior                                | per-pack xml asset collections                            | High              | deterministic with filesystem inputs                                | Rust-owned                                                |
| Unified XML construction            | `LoadedModManager.CombineIntoUnifiedXML`                                                          | xml asset collections                          | unified XML document and lookup structures                 | merge, source-index bookkeeping                                   | unified XML structures, source maps                       | High              | deterministic                                                       | Rust-owned                                                |
| Patch validation                    | `LoadedModManager.ErrorCheckPatches`                                                              | unified XML and patch metadata                 | error or warning outputs                                   | validation logging                                                | patch error state                                         | Medium            | deterministic                                                       | Rust-owned                                                |
| Patch application                   | `LoadedModManager.ApplyPatches`                                                                   | unified XML, asset lookup                      | patched unified XML                                        | XML mutation, patch-op dispatch                                   | patched XML tree, patch bookkeeping                       | High              | deterministic target but patch-op behavior may be extended          | managed-assisted but still within Rust semantic ownership |
| Inheritance registration/resolution | `XmlInheritance` phases inside `ParseAndProcessXML`                                               | patched XML                                    | resolved inherited XML nodes                               | inheritance graph bookkeeping                                     | inheritance caches and resolution state                   | High              | deterministic                                                       | Rust-owned                                                |
| Def parsing/materialization         | `LoadedModManager.ParseAndProcessXML`, `DirectXmlLoader` or `DirectXmlToObjectNew`                | patched XML, parser mode                       | instantiated def objects staged into mods or patchedDefs   | reflective instantiation, parser-path branching                   | `mod.defs`, `LoadedModManager.patchedDefs`                | High              | deterministic target but parser-path-sensitive                      | managed-assisted but still within Rust semantic ownership |
| Def DB population                   | `DefDatabase<T>.AddAllInMods`                                                                     | staged defs                                    | population of global def databases                         | reflective generic dispatch, parallel insertion                   | `DefDatabase<T>.defsList`, `defsByName`                   | High              | deterministic core                                                  | Rust-owned                                                |
| Non-implied xref pass               | `DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences(FailMode.Silent)`                        | staged wanted refs                             | partially resolved graph                                   | reference resolution bookkeeping                                  | `wantedRefs`, related dictionaries                        | High              | deterministic target but patch-sensitive ecosystem zone             | managed-assisted but still within Rust semantic ownership |
| Early DefOf and key binding         | `DefOfHelper.RebindAllDefOfs(true)`, key-mapping passes                                           | current databases and key state                | early bound static references, key mappings                | static assignment                                                 | relevant global binding state                             | High              | deterministic                                                       | Rust-owned                                                |
| Implied defs pre-resolve            | `DefGenerator.GenerateImpliedDefs_PreResolve`                                                     | current def graph                              | new implied defs                                           | def generation                                                    | def databases and implied-def state                       | High              | deterministic target but commonly patchable                         | managed-assisted but still within Rust semantic ownership |
| Implied-xref pass and clear         | `DirectXmlCrossRefLoader.ResolveAllWantedCrossReferences(FailMode.LogErrors)` and `Clear()`       | implied refs and pending refs                  | final resolved xref graph                                  | resolution, clear                                                 | cross-ref loaders cleared                                 | High              | deterministic target                                                | managed-assisted but still within Rust semantic ownership |
| Final DefOf rebind                  | `DefOfHelper.RebindAllDefOfs(false)`                                                              | final def graph                                | final bound static references                              | static assignment                                                 | final DefOf state                                         | High              | deterministic                                                       | Rust-owned                                                |
| Pre-resolve resets                  | `PlayDataLoader.ResetStaticDataPre`                                                               | current startup state                          | reset static caches before resolve work                    | static resets                                                     | multiple static holders                                   | Medium            | deterministic target but patch-sensitive                            | managed-assisted but still within Rust semantic ownership |
| Resolve passes                      | `ResolveAllReferences` across def databases                                                       | full def graph                                 | fully resolved defs                                        | reference resolution                                              | many def fields and caches                                | High              | deterministic target but may execute under Harmony-modified methods | managed-assisted but still within Rust semantic ownership |
| Implied defs post-resolve           | `DefGenerator.GenerateImpliedDefs_PostResolve`                                                    | resolved graph                                 | final implied products                                     | def generation                                                    | def databases                                             | High              | deterministic target but patchable                                  | managed-assisted but still within Rust semantic ownership |
| Post-resolve resets                 | `PlayDataLoader.ResetStaticDataPost`                                                              | resolved startup state                         | post-resolve static resets                                 | static resets                                                     | multiple static holders                                   | Medium            | deterministic target but patch-sensitive                            | managed-assisted but still within Rust semantic ownership |
| Short-hash assignment               | `ShortHashGiver.GiveAllShortHashes()`                                                             | final def databases                            | completed short-hash state                                 | hash allocation and per-type dictionary rebuild                   | `takenHashesPerDeftype`, `DefDatabase<T>.defsByShortHash` | High              | deterministic core                                                  | Rust-owned and semantic cutoff anchor                     |
| Post-cutoff callbacks               | `LongEventHandler.ExecuteWhenFinished(...)` callbacks                                             | post-startup queue state                       | bios, later language injection, later static work, atlases | arbitrary callback execution                                      | multiple globals                                          | High              | runtime-bound live tail                                             | delegated live-tail                                       |

## Def Lifecycle Map

1. XML files are discovered through mod content packs and versioned folder overlays.
2. XML assets are loaded into `LoadableXmlAsset` collections.
3. `CombineIntoUnifiedXML` produces the unified XML document.
4. Patch validation and patch application mutate the unified XML.
5. Inheritance is registered and resolved.
6. Parser path is chosen:

   * legacy path if command-line gated
   * default new path otherwise
7. XML nodes are materialized into def objects.
8. Def objects are staged into mod-local collections or patched-def collections.
9. `DefDatabase<T>.AddAllInMods` copies defs into global databases.
10. Cross references are resolved.
11. `DefOf` binding is performed.
12. Implied defs are generated pre-resolve.
13. Cross refs are resolved again and cleared.
14. `DefOf` is rebound.
15. Reference resolution passes complete.
16. Implied defs are generated post-resolve.
17. Static resets and checks complete.
18. Short hashes are assigned.
19. The fully resolved def-equivalence cutoff is reached.

## Ownership Recommendations for Rusty Startup

Under the current architecture baseline, Category 1 supports the following ownership posture:

### Rust-owned deterministic core

Rusty Startup should own:

* mod-order and mod-pack initialization logic
* XML discovery and deterministic file overlay behavior
* unified XML construction
* patch validation bookkeeping where deterministic
* inheritance resolution
* def database population semantics
* deterministic binding and hashing phases
* the semantic model through the short-hash cutoff

### Managed-assisted but still semantically owned region

Rusty Startup can still own the semantic result across these phases, even if actual execution must acknowledge managed participation risk:

* assembly reload and content reload kickoff
* mod class instantiation boundary
* patch application where custom or extended patch ops are involved
* parser-sensitive def materialization
* xref resolution in a Harmony-sensitive ecosystem
* implied-def generation and resolve phases where patched behavior may exist
* static reset regions that remain inside the deterministic startup chain

### Delegated live tail

Rusty Startup should treat the later callback-driven region after `GiveAllShortHashes()` as the delegated live tail for v1.

## High-Risk Ambiguities Still Worth Tracking

These do not reopen the core boundary decision, but they remain important implementation notes:

1. Assembly-load and module-initializer edge cases may allow arbitrary managed execution earlier than constructor instantiation in some cases.
2. Parser-path parity remains a compatibility-sensitive detail and must be handled according to the authoritative parser policy in the parser/initializer closure work.
3. Custom patch operations or Harmony modifications that alter startup methods can affect how much of the mixed zone is directly reproducible versus mirrored.
4. Pre-entry Unity/engine behavior remains outside this managed ownership audit.

## Canonical Wording Going Forward

Use the following wording consistently:

* **Runtime frontier start:** `LoadedModManager.CreateModClasses`
* **Semantic and snapshot cutoff:** immediately after `ShortHashGiver.GiveAllShortHashes()`
* **Model:** split-boundary model

This prevents the earlier contradiction from reappearing.

## Source Evidence Appendix

### Core local artifact examined

* `Assembly-CSharp.dll` from the local RimWorld 1.6 install

### Key vanilla types and methods examined

* `Verse.Root_Entry.Start`
* `Verse.Root.Start`
* `Verse.Root.CheckGlobalInit`
* `Verse.Root.Update`
* `Verse.LongEventHandler.LongEventsUpdate`
* `Verse.PlayDataLoader.LoadAllPlayData`
* `Verse.PlayDataLoader.DoPlayLoad`
* `Verse.LoadedModManager.LoadAllActiveMods`
* `Verse.LoadedModManager.CreateModClasses`
* `Verse.LoadedModManager.LoadModXML`
* `Verse.LoadedModManager.CombineIntoUnifiedXML`
* `Verse.LoadedModManager.ApplyPatches`
* `Verse.LoadedModManager.ParseAndProcessXML`
* `Verse.ShortHashGiver.GiveAllShortHashes`
* `Verse.ModAssemblyHandler.ReloadAll`

### Representative local mod evidence used in the reconciliation work

* local mods whose constructors or startup code patch startup-relevant methods
* local mods showing Harmony activity during startup-sensitive phases
* local mod evidence supporting the existence of the runtime-sensitive mixed zone beginning at `CreateModClasses`

### Representative commands used in the evidence-gathering process

* decompilation of core RimWorld startup types
* decompilation of sampled local mod assemblies
* targeted searches for startup method references in local mod assemblies
