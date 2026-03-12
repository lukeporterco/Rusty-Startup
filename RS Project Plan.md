# Rusty Startup Project Plan

## Current Architectural Vision

Rusty Startup is a replacement-oriented RimWorld startup engine whose job is to produce vanilla-equivalent startup results much faster than vanilla, while remaining plug-and-play, broadly compatible with existing mods, and biased toward aggressive optimization even when that means accepting temporary instability during early development.

The key idea is not procedural imitation. Rusty Startup does not need to perform startup the same way vanilla does. It needs to reach the same semantic destination. In other words, the loaded game state that matters to mods and to RimWorld should match what vanilla startup would have produced, or be close enough that existing mods behave correctly without any Rusty Startup-specific patches.

That is the core identity of the project. It is not primarily a helper cache, a sidecar optimizer, or a per-mod compatibility framework. It is a Rust-first startup replacement system that aims for semantic equivalence to vanilla output.

## What the Mod Is Trying to Achieve

The vision is a normal RimWorld mod that can be dropped into the mods folder on Linux, macOS, or Windows, loaded like a regular Workshop or local mod, and immediately attempt to reduce startup time as much as possible.

The target is startup acceleration for modded RimWorld specifically. That means the main concern is not a cleaner codebase or a prettier architecture for its own sake. The main concern is making the mod loading pipeline, especially under heavy modlists, dramatically faster while still preserving real mod compatibility.

The compatibility goal is especially important. The intent is that mod authors should not need to ship Rusty Startup support patches, annotate their mods for Rusty Startup, or maintain a second compatibility path just to function correctly. Rusty Startup should carry the burden of compatibility itself.

## Strongest Definition of the Core Architecture

The strongest version of the architecture, and the one that best matches your vision, is this:

> Rusty Startup should be a replacement-oriented startup engine that models the mod world directly, parallelizes every safely parallelizable region aggressively, reproduces predictable RimWorld startup semantics where possible, delegates dynamic managed behavior where necessary, and treats compatibility as an active solving problem rather than a rigid supported-versus-unsupported gate.

That definition is important because it rules out a weaker design. It means the project should not be built first as a wrapper around vanilla startup and only later be turned into a deeper startup replacement. That would center the whole codebase around the wrong assumption, namely that vanilla owns startup and Rusty Startup merely assists it.

So the architecture should be replacement-oriented from the beginning.

## Primary Contract of the Mod

The primary contract of Rusty Startup is semantic equivalence to vanilla startup output.

This means:

- Rusty Startup may use different data structures.
- Rusty Startup may use different execution order internally.
- Rusty Startup may use different caching approaches.
- Rusty Startup may use different threading strategies.
- Rusty Startup may use different implementation languages.
- At the end of startup, the visible startup meaning should match vanilla as closely as possible.

That includes things like definitions, patch effects, ordering consequences, resolved relationships, asset availability, and other startup-generated game state that mods implicitly depend on.

This distinction matters more than almost anything else in the project. Without it, the project risks becoming merely "fast but incompatible." With it, the project has a clear technical standard for every subsystem: correctness is judged by whether it reproduces vanilla-equivalent startup meaning.

## Why Rust Should Dominate the Implementation

The current best fit is one dominant Rust core, with a thin C# integration shell.

The reason is not just that Rust is faster in the abstract. It is that the expensive parts of this problem are likely to be data-heavy, highly parallel, and structurally analyzable. Those are the kinds of workloads where Rust gives you the best return.

C# should exist only where RimWorld, Unity, and the managed mod environment force it to exist. That includes things like the mod entrypoint, Harmony-facing integration, game-side bridging, runtime glue, and any unavoidable managed execution boundary.

So the architecture should not be "C# orchestrates multiple Rust helpers." That would leave managed code acting as the traffic controller and would likely create unnecessary bridge overhead. The cleaner design is "Rust owns the startup engine, C# is the narrow compatibility shell."

## What RimWorld Startup Implies for the Design

RimWorld's startup pipeline is already staged in a way that supports this mental model. Vanilla startup loads active mods, copies defs from mods into global databases, resolves cross references, generates implied defs, rebinds DefOfs, resolves references, assigns short hashes, and then schedules later work including backstory loading, language injection after implied defs, and static constructor calls. RimWorld also loads mod assemblies from `.dll` files in mod `Assemblies/` folders rather than compiling mod source at runtime.

That matters because it tells us two things:

1. There really is a large deterministic, data-heavy startup region that can be modeled, accelerated, parallelized, and cached.
2. There is also a live-runtime region that cannot simply be treated as static data replay, because some startup effects depend on managed execution timing, static constructors, Harmony behavior, and Unity-bound runtime state.

So the architecture needs to respect both realities at once.

## Central Architectural Model

The best overarching model is not a single monolithic loader. It is a staged execution graph built around a normalized internal world-model of startup-relevant content.

That means Rusty Startup should ingest the mod universe, classify it, normalize it into an internal representation, run transformation and resolution stages against that model, and only later cross into the narrower managed execution boundary where necessary.

The internal world-model is one of the most important concepts in the project. Instead of thinking in terms of "scan some files, build some caches, call some hooks," Rusty Startup should think in terms of "construct an internal semantic model of startup inputs and startup meaning."

That model becomes the source of truth for the engine.

## Execution Model

The execution model should be a deterministic staged graph with aggressive parallelism inside safe regions.

Not all startup work is equally parallelizable. Some work is pure data discovery. Some work is analysis. Some work is dependency reasoning. Some work is semantic transformation. Some work is Unity-bound or runtime-bound.

Instead of making the whole process one giant multithreaded blob, startup should be explicitly divided into phase families while making internals heavily parallel where safe.

### Conceptual Stage Map

1. Raw input discovery.
2. Content classification.
3. Normalization into the internal world-model.
4. Dependency and ordering analysis.
5. Deterministic semantic processing.
6. Snapshot and cache evaluation.
7. Execution plan synthesis.
8. Managed/runtime frontier.
9. Final reconciliation and integrity verification.

This is a conceptual stage map, not a concrete API or file layout.

## Parallelism Philosophy

Parallelism should be a first-class design assumption, not an optional extra layered on later.

That means subsystems should be written from the beginning in ways that make safe parallel execution natural. Work units should be explicit, dependencies should be visible, shared mutable state should be minimized, and serialization points should be intentional rather than accidental.

The architecture should assume that many expensive startup tasks are batchable and parallel-friendly, including file discovery, hashing, fingerprint generation, content indexing, XML parsing, structural normalization, and cache validation.

At the same time, the architecture should not force unsafe parallelism into parts of startup that depend on ordered side effects or runtime state.

## Compatibility Philosophy

Compatibility should be treated as an active solving problem.

That means Rusty Startup should not default to simple categories like "supported mods" and "unsupported mods" unless absolutely necessary. Real modded RimWorld is too messy for that to be the main architecture.

A better approach is to classify each mod, file family, assembly, patch set, or content region according to how aggressively the engine can take ownership of it.

This classifier is one of the central architectural subsystems, because it decides what the engine owns, what it partially owns, what it delegates, and what it treats as dangerous.

## Fail-Open and Fail-Soft Behavior

Rusty Startup should operate mostly in fail-open mode (roughly 90%) and reserve fail-soft for fatal, unsolvable, game-breaking, or cross-file integrity failures.

Architecturally, that means faults must be containable:

- A bad XML region should not poison the whole engine.
- A cache mismatch should not destroy startup globally.
- A texture or asset indexing anomaly should not force total startup failure.
- An assembly that cannot be safely reasoned about ahead of time should route into a more conservative execution lane.

Only when the engine loses trust in cross-cutting startup integrity should it move into fail-soft behavior.

## Snapshot and Replay Vision

The strongest phrasing of your "perfect snapshot" concept is:

> Build a canonical semantic startup snapshot and replay as much of it as safely possible.

That snapshot should represent deterministic semantic products of startup, not just arbitrary file caches. For stable combinations of game version, active modlist, mod versions, settings, language state, and other startup-relevant inputs, the engine should reuse a previously validated semantic snapshot and skip large portions of recomputation.

The realistic design is:

1. Compute and store semantic snapshots for deterministic startup regions.
2. Restore those snapshots on later boots when the fingerprinted environment matches.
3. Execute a much narrower live frontier for the parts that still must happen in a real process.

## Snapshot Keying and Invalidation

Snapshot validity should be semantically aware, not just "same modlist."

At minimum, keying likely needs:

- Game version
- Active mod set and load order
- Mod versions or content fingerprints
- Assembly fingerprints
- Relevant XML fingerprints
- Language/localization-sensitive inputs
- Platform-sensitive inputs
- Settings or flags that affect startup semantics

## Runtime Frontier

The runtime frontier is the boundary between work Rusty Startup can safely model/cache/replay and work that must still happen inside live managed Unity-coupled runtime.

This is not a weakness. It is a disciplined boundary that keeps the system ambitious and realistic.

## Likely Reconstructable Regions

Strong candidates for Rust ownership, acceleration, and snapshot reuse include:

- File and folder discovery
- Content indexing
- Version-aware content resolution
- Metadata normalization
- Dependency graphing
- Load-order consequence modeling
- XML parsing and normalization
- XML patch consequence modeling (large parts)
- Lookup table generation
- Reference planning
- Def-like structure preparation
- Asset enumeration and indexing
- Fingerprint and invalidation computation
- Compatibility classification
- Execution-plan synthesis

## Likely Live-Bound or Conservative Regions

Areas likely requiring a narrower boundary include:

- Assembly loading in the live process
- Harmony-sensitive runtime patch interactions
- Static-constructor-sensitive behaviors
- Reflection-heavy or side-effect-heavy mod initialization behavior
- Unity main-thread-coupled asset behaviors
- Any startup behavior whose meaning depends on runtime order, timing, or live managed state in ways that cannot be confidently reconstructed ahead of time

## Project Structure Implications

Because the architecture is built around trust, execution, and failure boundaries, modularization should follow those boundaries rather than giant "god files."

Likely subsystem boundaries:

- Discovery
- Normalization
- World-model construction
- Dependency analysis
- XML/structured-content processing
- Asset indexing
- Snapshot serialization and replay
- Compatibility classification
- Execution planning
- Managed bridge
- Runtime frontier handling
- Diagnostics and observability
- Integrity verification

Each subsystem should have a narrow responsibility, a visible trust boundary, and a clear failure surface.

## Plug-and-Play and Distribution Implications

The outward-facing packaging should behave like a normal RimWorld mod. Players should not need a custom launcher, special installation tooling, manual patch generation, or external preprocessing to benefit.

Cross-platform compatibility is a core requirement. Anything involving native Rust binaries, managed bridging, path logic, or platform-specific loading must be designed for Linux, macOS, and Windows from the start.

## Development Posture

The project is intentionally performance-first and solver-oriented.

This does not remove the need for semantic correctness. Without that anchor, the project can become fast in ways that undermine compatibility.

The posture is best described as:

> Performance-first, compatibility-driven, semantics-anchored, and recovery-oriented.

## Condensed Identity Statement

Rusty Startup is a plug-and-play, cross-platform, Rust-first RimWorld mod that aims to replace large portions of vanilla startup with a faster, parallelized, semantically equivalent startup engine, using a thin C# shell only where RimWorld and Unity require managed integration. Its core strategy is to build a normalized internal model of modded startup state, aggressively accelerate and cache deterministic startup work, restore validated semantic snapshots when possible, and confine live managed execution to a narrower runtime frontier, all while treating compatibility as an active solving problem and failing open for most local errors.

## Condensed Architecture Statement

The architecture should center on one dominant Rust core that owns discovery, normalization, semantic modeling, compatibility classification, execution planning, and snapshot replay, with C# acting only as the minimal entry and runtime bridge. Startup should be represented as a deterministic staged graph with heavy parallelism inside safe stages, not as a monolithic loader or a thin cache around vanilla. The engine's primary contract is vanilla-equivalent startup output, not procedural imitation, and its failure model should isolate faults locally whenever possible so that unknown mods, strange definitions, or partial cache invalidations degrade gracefully instead of collapsing the whole process.