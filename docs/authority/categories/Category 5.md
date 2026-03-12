# Real Mod Ecosystem Evidence Pack (Category 5, Merged)

## Executive Summary

This document defines the real local RimWorld 1.6 mod-ecosystem evidence base that supports Rusty Startup’s compatibility posture.

Corpus scope:

* local Steam Workshop mods root
* local non-Workshop RimWorld `Mods` root
* full discovered corpus with `N = 341` mods

High-signal findings carried forward from the corpus scan:

* the ecosystem is predominantly **hybrid content + code**: `196/341` mods have both meaningful XML volume (`>=10 XML`) and at least one managed assembly
* assembly participation is common: `287/341` mods contain one or more DLLs
* Workshop folder routing is normal rather than exceptional: `301/339` have top-level `1.x` folders and `139/339` include `LoadFolders.xml`
* patch-based def mutation is meaningful but concentrated: `50/341` mods include patch XML, with a small number of highly patch-dense packs
* Harmony, reflection, and static-initialization signals are common enough that any near-universal startup engine must explicitly model a runtime-sensitive mixed zone rather than pretend the whole ecosystem is pure data

Canonical architecture interpretation used by this merged document:

* **split-boundary model**
* first arbitrary managed-execution boundary begins at `Verse.LoadedModManager.CreateModClasses`
* semantic-equivalence and snapshot cutoff remains after `Verse.ShortHashGiver.GiveAllShortHashes()`
* this corpus supports aggressive semantic ownership through the cutoff **only if** Rusty Startup accepts managed-assisted ownership in the mixed zone rather than insisting on a fully pure-Rust lane for every mod stack

Canonical parser policy used by this merged document:

* v1 targets default parser semantics via `Verse.DirectXmlToObjectNew.DefFromNodeNew`
* legacy parser mode behind `legacy-xml-deserializer` is not the primary parity target for v1 and must be treated as a mode-aware managed-assisted or fallback lane

Canonical ecosystem conclusion:

* the local mod ecosystem supports **evidence-based aggressive ownership** up to the runtime frontier and through the resolved-def cutoff
* near-universal compatibility appears realistic **only with**:

  * deterministic folder-routing parity
  * robust patch/inheritance/def-order semantics
  * explicit handling of constructor-time Harmony and other mixed-zone managed behavior
  * diagnostics-first fallback when the ecosystem produces behavior outside the validated deterministic lane

## Corpus Description

### Inspected roots

* Steam Workshop mods:

  * `C:\Program Files (x86)\Steam\steamapps\workshop\content\294100`
* Non-Workshop local mods:

  * `C:\Program Files (x86)\Steam\steamapps\common\RimWorld\Mods`

### Corpus size and composition

* Workshop mods discovered: `339`
* Non-Workshop local mods discovered: `2`
* Total corpus: `341`
* Mods with `About/About.xml`: `341/341`

### Sampling method

The original Category 5 pass used a **full-corpus structural scan**, not a tiny anecdotal sample.

Additional representative examples were then selected by measurable thresholds and outlier behavior, including:

* highest XML counts
* highest DLL counts
* highest patch density
* startup-signal density
* framework/core naming signals
* unusual folder routing or content structure

Prevalence values below use `N = 341` unless marked `Workshop-only (N = 339)`.

### Data extraction approach

The ecosystem evidence uses two layers:

#### Layer A: full-corpus structural evidence

* XML count
* DLL count
* patch XML count
* presence of `LoadFolders.xml`
* top-level `1.x` folders
* common mod directories such as `Defs`, `Assemblies`, `Textures`, `Languages`

#### Layer B: startup-behavior signal evidence

* binary token search across sampled DLLs per mod for:

  * `HarmonyPatch` / `HarmonyLib`
  * static-constructor or type-initializer indicators
  * reflection API tokens
  * assembly-load related tokens

### Merged evidentiary interpretation

The startup-behavior signals are **evidence indicators**, not standalone formal decompilation proofs.

However, the follow-up closure work elevated the ecosystem interpretation by proving a key point directly from local decompilation:

* startup-sensitive constructor-time patching does exist in real local mods
* this validates using the corpus-wide signals as architecture-relevant evidence rather than mere speculation

## Ecosystem Pattern Inventory

### XML-heavy mods

Observed broadly, including extremely large XML surfaces.

Representative examples:

* Combat Extended (`7223 XML`)
* Dubs Bad Hygiene (`2755 XML`)
* Save Our Ship 2 (`1886 XML`)
* Vanilla Ideology Expanded - Memes and Structures (`1150 XML`)

Merged interpretation:

* this strongly supports Rust-owned deterministic ownership of XML discovery, merge, patch application, inheritance resolution, and resolved-def equivalence checking
* it also means performance-first indexing and snapshotting are mandatory, because correctness at this scale is otherwise too expensive

### Assembly-heavy mods

Assembly-heavy footprints are common and can be large.

Representative examples:

* Vanilla Expanded Framework (`77 DLL`)
* Melee Animation (`46 DLL`)
* Prison Labor (`34 DLL`)
* Adaptive Storage Framework (`33 DLL`)

Merged interpretation:

* startup ownership cannot be built around XML alone
* assembly discovery and identity must be treated as early semantic inputs because constructor-time and startup-patching behavior can influence mixed-zone semantics

### Harmony-heavy mods

Harmony tokens are widespread in scanned DLLs.

Signal prevalence:

* `267/341` with at least one Harmony signal
* `222/341` Harmony-heavy by threshold (`>=2 sampled DLL hits`)

Representative examples:

* Vanilla Expanded Framework
* Prison Labor
* Dubs Bad Hygiene
* Integrated Implants

Merged interpretation:

* the ecosystem directly supports the split-boundary model
* the existence of Harmony-heavy mods does **not** mean aggressive ownership is impossible
* it means that the region after `CreateModClasses` must be treated as a managed-sensitive mixed zone, not as a naively pure deterministic lane

### Patch-dense mods

Patch XML is concentrated but operationally important.

* `50/341` have patch XML
* top densities include:

  * Combat Extended (`94`)
  * Project RimFactory Revived (`37`)
  * Muzzle Flash (`32`)
  * More Linkables (`27`)
  * LWM's Deep Storage (`26`)

Merged interpretation:

* patch chain determinism is one of the core compatibility pillars
* replay correctness depends on preserving patch order, source provenance, and overwrite precedence

### Library/framework mods

Framework/core mods are materially represented.

Name/package signal count:

* `8/341`

Representative examples:

* Vanilla Expanded Framework
* Adaptive Storage Framework
* Vehicle Framework
* HugsLib
* Fortified Features Framework

Merged interpretation:

* framework mods amplify the impact of startup-ownership mistakes across many dependents
* they also strengthen the case for diagnostics-first compatibility classification because one wrong assumption can fan out through large dependency trees

### Large content packs

Large XML + texture packages are common enough to affect performance design.

Examples by `(XML + texture)` size score:

* Combat Extended (`8793`)
* Vanilla Textures Expanded (`5604`)
* Dubs Bad Hygiene (`3906`)
* Save Our Ship 2 (`2884`)
* Vanilla Ideology Expanded - Memes and Structures (`2552`)

Merged interpretation:

* large packs strengthen the case for semantic snapshot/replay in v1
* they also make strict resolved-def equivalence more important, because scale magnifies subtle ordering or patching mistakes

### Unusual folder-structure mods

Versioned and load-routed structures are the norm, not exceptions, in Workshop content.

Workshop frequencies:

* with top-level `1.x` folders: `301/339`
* with `LoadFolders.xml`: `139/339`
* common top-level folder frequencies:

  * `1.6 (281)`
  * `1.5 (274)`
  * `1.4 (227)`
  * `LoadFolders.xml (139)`

Merged interpretation:

* deterministic folder resolution is a first-class compatibility requirement
* there is no credible near-universal compatibility story without exact parity in load-folder routing and conditional root selection

### Static-constructor-heavy / reflection-heavy startup signals

Startup dynamism signals are common in sampled DLLs.

Signal prevalence:

* reflection-heavy signal (`>=2 sampled DLL hits`): `258/341`
* static-constructor-heavy signal (`>=2 sampled DLL hits`): `246/341`
* assembly-load signal (`>=1 sampled DLL hit`): `104/341`

Representative examples:

* Character Editor
* RimHUD
* Common Sense
* Roads of the Rim (Continued)
* QualityBuilder

Merged interpretation:

* the ecosystem absolutely does contain startup behavior that cannot be treated as a purely closed static XML problem
* however, the parser/initializer closure work also means this does **not** automatically push the ownership boundary earlier than `CreateModClasses`
* instead, these signals justify the mixed-zone model and stronger diagnostics, not a collapse back to wrapper-only architecture

## Startup-Relevant Pattern Tables

| Pattern                                     |                                                          Prevalence in corpus | Representative examples                                       | Why it matters for Rusty Startup ownership / solver design                                                                                |
| ------------------------------------------- | ----------------------------------------------------------------------------: | ------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| XML-heavy content                           |                                                     `76/341` with `>=100 XML` | Combat Extended, Dubs Bad Hygiene, SOS2, VIE Memes/Structures | Strongly supports aggressive deterministic ownership of def ingestion, merge, patching, inheritance resolution, and equivalence checking. |
| Assembly presence                           |                                                           `287/341` with DLLs | VEF, Melee Animation, Prison Labor                            | Solver must model loaded assemblies and startup-sensitive code presence, not just XML graphs.                                             |
| Hybrid XML + assembly                       |                                             `196/341` (`XML>=10` and `DLL>0`) | CE, PRF Revived, Rimatomics, Rimefeller                       | Realistic target for Rust-owned preprocessing up to the mixed zone and semantic ownership through the cutoff.                             |
| Harmony-heavy signal                        |                                                        `222/341` by threshold | VEF, Prison Labor, Dubs Bad Hygiene                           | Validates explicit runtime-frontier model and managed-assisted mixed-zone handling.                                                       |
| Patch-dense XML mutation                    |                        `5/341` (`patch XML>=10`), `50/341` with any patch XML | CE, PRF Revived, Muzzle Flash, LWM's Deep Storage             | Patch pipeline determinism and diagnostics are mandatory for equivalence and replay.                                                      |
| Framework / library mods                    |                                                   `8/341` name/package signal | VEF, HugsLib, Vehicle Framework, Adaptive Storage Framework   | Framework hubs amplify startup mistakes and require compatibility classification that understands dependency blast radius.                |
| Versioned / load-routed layout              |                        `301/339` top-level `1.x`; `139/339` `LoadFolders.xml` | Vanilla Expanded modules, Prison Labor, SOS2                  | Folder routing must be deterministic, fingerprinted, and explainable in snapshot metadata.                                                |
| Reflection / static-init / asm-load signals | `258/341` reflection-heavy; `246/341` static-heavy; `104/341` asm-load signal | Character Editor, RimHUD, Common Sense, QualityBuilder        | Justifies mixed-zone handling, managed-assisted ownership, and explicit fallback tags.                                                    |
| Large content packs                         |                                                     multiple very large packs | CE, Vanilla Textures Expanded, SOS2, VFE modules              | Makes snapshot/replay and fast structural hashing central to the v1 plan.                                                                 |

## Compatibility Risk Register

| Risk                                    | Observed evidence                                                                              | Severity    | Why it stresses architecture                                                   | Merged mitigation direction                                                                               |
| --------------------------------------- | ---------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| Versioned folder routing divergence     | `301/339` Workshop mods use top-level `1.x`; `139/339` use `LoadFolders.xml`                   | High        | Wrong folder selection invalidates equivalence before any patching or parsing. | Deterministic load-folder resolver with auditable decisions and snapshot provenance.                      |
| Patch order and conflict chains         | `50/341` with patch XML; a few very dense packs (`CE=94`, `PRF=37`)                            | High        | Small ordering changes cascade into def graph mismatches.                      | Canonical patch pipeline, patch provenance capture, and mismatch diagnostics.                             |
| Constructor-time Harmony mutation       | Harmony-heavy signals common; local decompilation proves real startup patching in constructors | High        | Startup methods in the mixed zone may execute under modified bodies.           | Split-boundary model, managed-assisted ownership after `CreateModClasses`, and explicit lane diagnostics. |
| Reflection/static-init unpredictability | Reflection/static signals common (`258/341`, `246/341`)                                        | High        | Startup side effects may be load-order-sensitive or data-dependent.            | Early detection, conservative lane selection, and fallback/delegation reason taxonomy.                    |
| Framework dependency blast radius       | framework/core mods materially present                                                         | Medium-High | One incorrect assumption can affect many dependents.                           | Framework-aware compatibility policy and hub-focused test stacks.                                         |
| Scale/performance pressure              | very large XML + texture + assembly packs present                                              | High        | Correctness becomes expensive without replay and indexing.                     | Semantic snapshot/replay, region-aware invalidation, and fast structural hashing.                         |
| Parser-path mismatch                    | local RimWorld 1.6 has both default and legacy parser lanes                                    | Medium      | Claiming parity under the wrong parser lane would invalidate equivalence.      | Canonical default-parser target; legacy mode as explicit managed-assisted/fallback lane.                  |
| Over-generalizing startup signals       | corpus scan uses heuristics for many DLLs                                                      | Medium      | Heuristic prevalence alone is not enough for hard no-go rules.                 | Use corpus scan for architecture pressure and decompilation closure for hard constraints.                 |

## Evidence-Based Ownership Opportunities

This section defines where the real local ecosystem supports aggressive ownership.

### Opportunity O1: deterministic mod-root and folder routing ownership

Why supported:

* versioned folders and `LoadFolders.xml` are extremely common and structurally auditable
* these are exactly the kind of deterministic content-selection problems Rusty Startup should own

Consequence:

* Rusty Startup should fully own load-folder resolution and content-root selection logic in v1

### Opportunity O2: XML and patch semantic ownership

Why supported:

* XML-heavy mods are widespread
* patch XML is common enough to matter but structurally deterministic enough to normalize

Consequence:

* Rusty Startup should fully own XML discovery, merge, patch application semantics, inheritance resolution, and the resulting semantic graph up to parser/materialization

### Opportunity O3: resolved-def ownership through the cutoff

Why supported:

* hybrid XML + assembly mods are common, but the local ecosystem evidence still points to a large deterministic semantic surface that survives even when managed code participates
* constructor-time patching proves the mixed-zone concern is real, but does not disprove semantic ownership through the cutoff

Consequence:

* Rusty Startup should own the semantic outcome through the Category 3 cutoff, with managed-assisted handling where the mixed zone requires it

### Opportunity O4: snapshot/replay is justified by real ecosystem scale

Why supported:

* very large content packs and hybrid mod stacks make repeated full startup processing expensive
* machine-local snapshotting is a rational fit for the observed ecosystem

Consequence:

* snapshot/replay is not optional optimization fluff; it is a realistic necessity for performance-first correctness at this ecosystem scale

## Evidence-Based No-Go or Caution Zones

This section defines where the local ecosystem says caution or delegation is still required.

### Caution C1: treating post-`CreateModClasses` startup as if no managed behavior can matter

Why not justified:

* local decompilation proved constructor-time Harmony patching against startup pipeline methods
* Harmony-heavy ecosystem prevalence is high

Consequence:

* after `CreateModClasses`, Rusty Startup must treat startup as a managed-sensitive mixed zone, not a purely isolated Rust lane

### Caution C2: claiming universal parity for legacy parser mode in v1

Why not justified:

* parser closure work chose default parser semantics as the canonical v1 target
* legacy mode remains outside the primary parity target

Consequence:

* legacy parser mode must be diagnosed and routed into managed-assisted or fallback behavior

### Caution C3: assuming startup signals alone prove exact runtime behavior

Why not justified:

* much of the corpus-wide startup behavior scan is heuristic
* hard implementation constraints still need direct local decompilation proof where boundary decisions are involved

Consequence:

* the corpus justifies architecture posture and prioritization, but exact solver rules still need proof-oriented closure at the critical edges

### Caution C4: extending ownership into the delegated live tail by default

Why not justified:

* callback-driven tail work includes bios, post-implied language injection, static-constructor execution, atlas work, and resource cleanup

Consequence:

* near-universal compatibility for v1 is more realistic if the live tail remains delegated rather than silently absorbed into the deterministic model

## Implications for Near-Universal Compatibility

The chosen architecture remains realistic against the observed local ecosystem, but only in its **merged** form.

That means near-universal compatibility is realistic if and only if Rusty Startup does all of the following:

1. Preserves exact folder-routing parity
2. Preserves patch ordering and provenance
3. Targets default parser semantics explicitly
4. Treats constructor-time and Harmony-sensitive startup as part of a mixed zone beginning at `CreateModClasses`
5. Maintains semantic ownership through the resolved-def cutoff rather than retreating to a shallow wrapper model
6. Uses diagnostics-first fallback for runs that land outside the primary validated lane

The ecosystem evidence therefore supports the current architecture decisions.

It does **not** support:

* a naive “all startup is pure data” model
* a simplistic early frontier that ends ownership at `CreateModClasses`
* a v1 promise of full legacy-parser parity without explicit fallback logic

## Relationship to the Split-Boundary Model

Category 5 now explicitly incorporates the boundary reconciliation.

That means:

* the corpus evidence is interpreted through two separate boundaries
* the first arbitrary managed-execution boundary is at `CreateModClasses`
* the semantic-equivalence cutoff remains after `GiveAllShortHashes()`

This matters because the original ecosystem scan already suggested aggressive ownership was plausible, but it lacked the formal language to distinguish “runtime-sensitive” from “semantically ownable.”

The split-boundary model fixes that.

## Relationship to the Parser / Initializer Closure

Category 5 also incorporates the parser/initializer closure.

That means:

* parser-path multiplicity is now treated as a concrete mode distinction, not a vague ecosystem concern
* legacy parser mode is explicitly outside the primary parity target for v1
* sampled module-initializer evidence does not move the architecture boundary, so the ecosystem evidence continues to support the current ownership model

## Residual Risks Worth Tracking

These do not reopen the core ecosystem conclusion, but they remain planning notes:

1. Long-tail unsampled mods could still reveal new mixed-zone pathologies that need classifier rules.
2. Framework hub stacks deserve explicit compatibility test packs because dependency blast radius is large.
3. Legacy parser runs will need visible diagnostics so parity claims remain honest.
4. Extremely patch-dense stacks may require more detailed provenance capture than average modlists.

## Canonical Wording Going Forward

Use these phrases consistently:

* **evidence-based aggressive ownership**: aggressive semantic ownership grounded in real local mod data, not abstract optimism
* **runtime-sensitive mixed zone**: begins at `LoadedModManager.CreateModClasses`
* **semantic cutoff**: immediately after `ShortHashGiver.GiveAllShortHashes()`
* **primary parser lane**: default parser semantics via `DirectXmlToObjectNew`
* **legacy parser lane**: managed-assisted or fallback, not the primary v1 parity target

## Source Evidence Appendix

### Local roots inspected

* `C:\Program Files (x86)\Steam\steamapps\workshop\content\294100`
* `C:\Program Files (x86)\Steam\steamapps\common\RimWorld\Mods`

### Corpus metrics incorporated from the original Category 5 scan

* total corpus size: `341`
* Workshop mods: `339`
* local mods: `2`
* XML-heavy, DLL-heavy, patch-dense, Harmony-heavy, reflection-heavy, static-heavy, and load-routed prevalence figures used throughout this merged document

### Follow-up findings incorporated here

* split-boundary reconciliation:

  * first arbitrary managed-execution boundary begins at `CreateModClasses`
  * semantic ownership/cutoff remains through `GiveAllShortHashes()`
* parser/initializer closure:

  * default parser semantics are the canonical v1 target
  * legacy parser mode is not the primary v1 parity target
  * sampled module-initializer evidence does not force an earlier ownership boundary

### Representative local examples referenced

* Combat Extended
* Dubs Bad Hygiene
* Save Our Ship 2
* Vanilla Expanded Framework
* Prison Labor
* Character Editor
* SimpleChecklist
* ilyvion.LoadingProgress
* other Harmony-heavy and patch-heavy Workshop examples from the local corpus

### Evidence-gathering methods represented in the merged document

* full-corpus filesystem and folder-structure scan
* XML / DLL / patch count extraction
* startup-signal token scans over sampled DLLs
* targeted local decompilation of startup-relevant mod assemblies in follow-up closure work
