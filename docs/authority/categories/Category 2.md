# Runtime Frontier and Live-Tail Pack (Category 2, Merged)

## Executive Summary

This audit uses local RimWorld 1.6 managed binaries and local Workshop assemblies to determine where startup stops being safely pure-deterministic and becomes meaningfully live-runtime-bound.

The final result is a **split-boundary model**.

* The first reliably evidenced arbitrary managed-execution boundary begins at `Verse.LoadedModManager.CreateModClasses`, where `Activator.CreateInstance` instantiates `Verse.Mod` subclasses.
* That point marks the start of the **runtime-sensitive mixed zone**.
* It does **not** mark the end of semantic ownership.
* Semantic ownership for Rusty Startup v1 can still extend through `Verse.ShortHashGiver.GiveAllShortHashes()` by using a mixed strategy:

  * Rust-owned where the startup phase is deterministic and structurally reproducible
  * managed-assisted where the startup phase may execute under Harmony-modified or constructor-influenced managed behavior
* The delegated live tail begins only after short-hash completion, in queued `LongEventHandler.ExecuteWhenFinished(...)` callbacks and later main-thread/runtime-coupled work.

This merged document supersedes earlier wording that treated “runtime frontier” and “semantic cutoff” as the same boundary. They are not the same boundary.

## Canonical Boundary Definitions

### First arbitrary managed-execution boundary

`Verse.LoadedModManager.CreateModClasses`

Reason:
This is the first clearly evidenced point where arbitrary mod code runs during startup via `Activator.CreateInstance(type, modContentPack)` on `Verse.Mod` subclasses.

### Runtime frontier start

The runtime frontier starts at `CreateModClasses`.

Meaning:
From this point forward, startup enters a mixed zone where some phases may still be semantically ownable, but no longer safely treated as pure unmodified vanilla logic.

### Semantic-equivalence cutoff

Immediately after `Verse.ShortHashGiver.GiveAllShortHashes()` returns inside `Verse.PlayDataLoader.DoPlayLoad()`.

Meaning:
This is the correct resolved-def and snapshot cutoff for v1.

### Snapshot ownership boundary

The snapshot ownership boundary includes startup products through short-hash completion.

### Delegated live tail

The delegated live tail begins after the short-hash cutoff, in callback-driven and runtime-coupled work such as:

* `SolidBioDatabase.LoadAllBios`
* post-implied language injection
* `StaticConstructorOnStartupUtility.CallAll`
* atlas baking
* GC and resource unload operations

## Runtime Frontier Candidates

### Candidate C1: `Verse.Root.Start` and `Verse.Root.CheckGlobalInit`

These methods perform startup orchestration, global Unity/process-state initialization, and long-event queue setup.

Assessment:
Mostly deterministic sequencing, but process-global, Unity-coupled, and order-sensitive.

Result:
This is not the true frontier, but it is part of the outer runtime shell that Rusty Startup must mirror rather than replace wholesale.

### Candidate C2: `Verse.LongEventHandler` core scheduling methods

Relevant methods include:

* `QueueLongEvent`
* `LongEventsUpdate`
* `UpdateCurrentAsynchronousEvent`
* `ExecuteToExecuteWhenFinished`

Assessment:
These methods control thread handoff, long-event progression, and callback draining. Callback content is unconstrained managed code.

Result:
This is a runtime-bound scheduling substrate and should remain mirrored or delegated, not redefined as pure deterministic logic.

### Candidate C3: `Verse.ModAssemblyHandler.ReloadAll`

Assessment:
Dynamic assembly loading through `Assembly.LoadFrom`, plus `AppDomain.CurrentDomain.AssemblyResolve` mutation, introduces process-global managed-runtime sensitivity.

Result:
This is inside the mixed zone and requires managed assistance.

### Candidate C4: `Verse.LoadedModManager.CreateModClasses`

Assessment:
This is the first clearly evidenced arbitrary mod code execution boundary.

Result:
This is the correct start of the runtime-sensitive mixed zone.

### Candidate C5: post-constructor startup pipeline

Relevant methods include:

* `LoadedModManager.LoadModXML`
* `CombineIntoUnifiedXML`
* `ApplyPatches`
* `ParseAndProcessXML`
* later def-processing methods in `PlayDataLoader`

Assessment:
These phases are data-heavy and semantically deterministic in target state, but can execute under constructor-driven Harmony changes or other startup-managed side effects after `CreateModClasses`.

Result:
These phases remain semantically ownable, but not purely isolated from runtime-managed influence.

### Candidate C6: `Verse.StaticConstructorOnStartupUtility.CallAll`

Assessment:
Runs all `[StaticConstructorOnStartup]` class constructors with `RuntimeHelpers.RunClassConstructor`. This can create patches, register global mutations, and touch Unity or startup state.

Result:
This belongs in the delegated live tail.

### Candidate C7: `PlayDataLoader.DoPlayLoad` callback tail

Assessment:
`ExecuteWhenFinished(...)` registers work for later callback execution, including bios, language injection, static constructors, atlas work, GC, and resource unload.

Result:
This is the clearest live tail boundary after the semantic cutoff.

### Candidate C8: Unity-coupled mod content paths

Relevant subsystems include:

* `ModContentHolder<T>`
* `ModContentLoader<T>`
* `ModAssetBundlesHandler`

Assessment:
Texture loading, asset-bundle loading, Unity object destroy/unload, and similar work are not pure deterministic transforms.

Result:
These are runtime-coupled and generally remain delegated or carefully mirrored.

### Candidate C9: `Verse.ModLister.RebuildModList` and metadata flow

Assessment:
Mostly deterministic filesystem and metadata discovery, though Steam, DLC, duplicate package handling, and environment-sensitive conditions matter.

Result:
This is largely ownable or mirrorable and does not define the frontier.

## Boundary Analysis Table

| ID | Class / method / location                                                                                                     | Why runtime-bound or mixed                                                | Risk dimensions                                                                       | Trigger                                         | Reorderable?                                                            | Rusty Startup action                                               |
| -- | ----------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ----------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------ |
| C1 | `Verse.Root.Start`, `Verse.Root.CheckGlobalInit`                                                                              | Global startup orchestration with Unity/process coupling                  | timing-sensitive, state-sensitive, Unity-main-thread-sensitive                        | Unity scene root `Start()`                      | Partially, only under strict dependency preservation                    | mirror                                                             |
| C2 | `Verse.LongEventHandler.QueueLongEvent`, `LongEventsUpdate`, `UpdateCurrentAsynchronousEvent`, `ExecuteToExecuteWhenFinished` | Thread handoff and callback draining for arbitrary managed work           | timing-sensitive, Unity-main-thread-sensitive, Harmony-sensitive                      | Any queued long event/callback                  | Not safely reorderable once queue is populated                          | delegate scheduler, mirror semantics                               |
| C3 | `Verse.ModAssemblyHandler.ReloadAll`                                                                                          | Dynamic assembly load plus `AssemblyResolve` mutation                     | reflection-sensitive, state-sensitive, static-constructor-sensitive, AppDomain-global | `ModContentPack.ReloadContent(hotReload:false)` | Not safely reorderable relative to type discovery and mod instantiation | managed-assisted within semantic ownership                         |
| C4 | `Verse.LoadedModManager.CreateModClasses`                                                                                     | First guaranteed arbitrary mod constructor execution                      | Harmony-sensitive, timing-sensitive, state-sensitive                                  | `LoadedModManager.LoadAllActiveMods()`          | Not safely reorderable for universal compatibility                      | runtime frontier start; managed-assisted within semantic ownership |
| C5 | `LoadedModManager.ApplyPatches`, `ParseAndProcessXML`, related startup methods                                                | Data-heavy target, but may already be Harmony-modified after constructors | Harmony-sensitive, reflection-sensitive, timing-sensitive                             | Runs after `CreateModClasses`                   | Reorderable only where equivalence is proven                            | managed-assisted within semantic ownership                         |
| C6 | `Verse.StaticConstructorOnStartupUtility.CallAll`                                                                             | Executes all `[StaticConstructorOnStartup]` initializers                  | static-constructor-sensitive, Harmony-sensitive, Unity-main-thread-sensitive          | Queued from startup and callback tail           | Not safely reorderable in patched stacks                                | delegate live tail                                                 |
| C7 | `PlayDataLoader.DoPlayLoad` `ExecuteWhenFinished` tail                                                                        | Callback boundary for bios/language/static ctors/atlas/GC/resource work   | timing-sensitive, Unity-main-thread-sensitive, Harmony-sensitive                      | End of play-data load pass                      | Not safely reorderable as a group without equivalence proof             | delegate live tail                                                 |
| C8 | `ModContentHolder<T>`, `ModContentLoader<T>`, `ModAssetBundlesHandler`                                                        | Uses Unity APIs and object-lifecycle-sensitive asset behavior             | Unity-main-thread-sensitive, state-sensitive                                          | Mod content reload/clear/unload                 | Not safely reorderable around Unity lifecycle                           | delegate or mirror narrowly                                        |
| C9 | `Verse.ModLister.RebuildModList`, `Verse.ModMetaData` loading                                                                 | Mostly deterministic discovery but environment-sensitive                  | state-sensitive, environment-sensitive                                                | Mod-list rebuild / startup init                 | Largely yes, if precedence and duplicate rules are preserved            | own or mirror                                                      |

## Live Tail Map

### Runtime-sensitive mixed zone

This begins at:
`LoadedModManager.CreateModClasses`

The mixed zone includes:

1. `CreateModClasses`
2. constructor-time mod behavior
3. post-constructor XML and patch pipeline
4. def materialization and database population
5. cross-reference resolution
6. implied-def generation
7. resolve passes
8. short-hash assignment

This zone is not purely runtime tail. It is a mixed region where semantic ownership remains possible, but managed participation risk must be acknowledged.

### Delegated live tail

This begins only after short-hash completion.

Execution map:

1. `LongEventHandler.ExecuteWhenFinished(...)` callback registrations are drained
2. bios load runs
3. post-implied language injection runs
4. `StaticConstructorOnStartupUtility.CallAll` runs where queued for later stage execution
5. atlas bake and texture/graphics cleanup work runs
6. GC and resource unload work runs
7. normal root/UI flow resumes

### Snapshot/replay implication

Rusty Startup can own semantic products through the short-hash cutoff.

However:

* the mixed zone may require managed-assisted replay or validation logic
* the delegated live tail remains out of semantic snapshot scope for v1

## Static Constructor and Harmony Risk Audit

### Vanilla startup anchors

1. `Root.CheckGlobalInit` queues `StaticConstructorOnStartupUtility.CallAll`
2. `PlayDataLoader.DoPlayLoad` also schedules later callback-tail work
3. `StaticConstructorOnStartupUtility.CallAll` uses `RuntimeHelpers.RunClassConstructor` over `[StaticConstructorOnStartup]` types

### Local mod evidence

#### `CharacterEditor`

* `CharacterEditor.CharacterEditorMod` constructor calls `CEditor.Initialize`
* `CEditor.Initialize` performs Harmony patching of startup/UI/game-flow methods such as `MainMenuDrawer`, `UIRoot_Entry`, `Map.FinalizeInit`, `Game.LoadGame`, and others

#### `SimpleChecklist`

* constructor creates Harmony instance and applies patches

#### `ilyvion.LoadingProgress`

* constructor-time or startup-time patching targets methods including `LoadedModManager`, `PlayDataLoader`, `DefGenerator`, `ShortHashGiver`, and `DirectXmlCrossRefLoader`

#### `Harmony` mod and similar patterns

* startup inspection of loaded assemblies and `PatchAll()` patterns are present in real local mod data

### Conclusion from local evidence

The significance of `CreateModClasses` is not merely that mod instances now exist. It is that after this point, startup methods may execute under altered method bodies, constructor-established global state, or startup-triggered patch registrations.

Therefore, the post-constructor startup chain cannot be treated as naively pure vanilla logic, even when the target semantic output remains deterministic.

## Unity/Main-Thread and Asset-Coupled Startup Audit

The following remain runtime-coupled rather than safe pure-data territory:

* Unity texture loading
* asset-bundle load/unload
* Unity object destruction and cleanup
* some root/UI flow work
* callback-drained graphics/resource operations

These do not define the semantic cutoff, but they do constrain what Rusty Startup should snapshot or take over directly.

## Strongest Practical Runtime Frontier Recommendation

### Forced recommendation

Adopt the **split-boundary model** as the authoritative Category 2 position.

That means:

* runtime frontier start: `LoadedModManager.CreateModClasses`
* semantic/snapshot cutoff: immediately after `ShortHashGiver.GiveAllShortHashes()`

### Practical meaning for v1

Rusty Startup v1 should:

* treat everything before `CreateModClasses` as strongly ownable or mirrorable deterministic startup
* treat the region from `CreateModClasses` through `GiveAllShortHashes()` as semantically ownable but runtime-sensitive, requiring managed-assisted ownership where needed
* treat the callback-driven region after short hashes as the delegated live tail

### Why this is stronger than the earlier Category 2 conclusion

The earlier standalone Category 2 document was correct that arbitrary managed execution begins at `CreateModClasses`.

It was incomplete because it implicitly treated that point as the end of semantic ownership.

The boundary reconciliation work showed that this conflated:

* first arbitrary managed-execution boundary
* semantic-equivalence cutoff
* snapshot ownership boundary

These are now separated.

## Reconciliation Against Earlier Category 2 Wording

### Still valid

* `CreateModClasses` is the first reliably evidenced arbitrary managed-execution boundary
* startup becomes Harmony-sensitive and constructor-sensitive after that point
* blind replacement after `CreateModClasses` is unsafe
* callback-tail and Unity-coupled work remain delegated

### Corrected

* `CreateModClasses` is **not** the true end of Rusty Startup ownership
* the strongest practical runtime model is **not** “frontier at `CreateModClasses` and stop there”
* the correct model is “runtime-sensitive mixed zone starts at `CreateModClasses`, semantic ownership extends through short hashes”

### Canonical wording going forward

Use these phrases consistently:

* **runtime frontier start**: `CreateModClasses`
* **semantic cutoff**: after `GiveAllShortHashes()`
* **delegated live tail**: callback-driven region after short hashes

## Residual Risks Worth Tracking

These do not reopen the core boundary decision, but they remain real implementation notes:

1. Assembly-load edge cases and rare initializer behavior could permit managed side effects earlier than constructor instantiation in some assemblies
2. Harmony patch surfaces inside the mixed zone may require proof-oriented mirroring rather than direct reimplementation in some subsystems
3. Unity-coupled asset paths remain outside the v1 semantic snapshot region

## Source Evidence Appendix

### Core local artifact examined

* `Assembly-CSharp.dll` from the local RimWorld 1.6 install

### Key vanilla types and methods examined

* `Verse.Root.Start`
* `Verse.Root.CheckGlobalInit`
* `Verse.LongEventHandler.QueueLongEvent`
* `Verse.LongEventHandler.LongEventsUpdate`
* `Verse.ModAssemblyHandler.ReloadAll`
* `Verse.LoadedModManager.CreateModClasses`
* `Verse.LoadedModManager.LoadModXML`
* `Verse.LoadedModManager.ApplyPatches`
* `Verse.LoadedModManager.ParseAndProcessXML`
* `Verse.PlayDataLoader.DoPlayLoad`
* `Verse.StaticConstructorOnStartupUtility.CallAll`
* `Verse.ShortHashGiver.GiveAllShortHashes`

### Representative local mod evidence used

* `CharacterEditor`
* `SimpleChecklist`
* `ilyvion.LoadingProgress`
* Harmony-heavy startup-patching mods present in the local Workshop corpus

### Representative commands used in the original evidence-gathering process

* decompilation of RimWorld startup types
* decompilation of sampled local mod assemblies
* targeted searches for startup-sensitive method references in local mod assemblies
