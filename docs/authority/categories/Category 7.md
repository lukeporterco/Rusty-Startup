# Performance Baseline and Benchmark Pack (Category 7, Merged)

## Executive Summary

This document defines the canonical local startup-performance evidence base and benchmark-planning artifact for Rusty Startup v1.

Canonical local planning baseline used here:

* **RimWorld 1.6.4633 rev1261**
* when fresh runtime banner evidence disagrees with static `Version.txt`, runtime evidence is authoritative for planning and gating on this machine

This merged document incorporates the baseline reconciliation follow-up and supersedes older mixed-revision wording.

Core result:

* the benchmark-baseline ambiguity is closed for planning
* benchmark evidence is now split cleanly into:

  * fresh runtime evidence
  * still-valid archived/local artifact evidence
  * stale or currently unverifiable archive-only claims
* the benchmark harness remains phase-aware and ownership-aware, but any hook or command surface derived only from missing archive content is treated as historical, not currently verified-local source evidence

Planning interpretation:

* there is enough local evidence to anchor v1 milestone benchmarking
* there is enough instrumentation shape to define the benchmark contract
* there is not yet a full statistical benchmark distribution across all tiers and modes, so this document remains a planning/gating baseline rather than a final published benchmark study

Canonical architecture assumptions used here:

* split-boundary model
* deterministic ownership and semantic equivalence through the fully resolved def cutoff
* machine-local snapshot/replay in v1
* diagnostics and fallback costs are benchmark-critical, not optional

## Canonical Current Planning Baseline

### Canonical runtime baseline

The canonical planning runtime for this machine is:

* **`RimWorld 1.6.4633 rev1261`**

### Reconciliation rule

If fresh runtime log evidence and static install metadata disagree:

* use the runtime banner as authoritative for planning and gating on this machine
* treat conflicting static metadata as lower-authority evidence

### Current local inconsistency handled by this rule

Observed locally:

* fresh runtime evidence reports `rev1261`
* static `Version.txt` reports `rev1260`

This is now resolved as follows:

* `rev1261` is canonical for implementation planning and benchmark gating
* earlier `rev1260` mentions in old category material are superseded evidence, not a live contradiction

## Available Local Timing Sources

### 1. Runtime logs (`Player.log`, `Player-prev.log`)

Available timing boundaries and evidence include:

* `RimWorld 1.6.4633 rev1261`
* `- Loaded All Assemblies, in ... seconds`
* `- Finished resetting the current domain, in ... seconds`
* `UnloadTime: ... ms`
* `Total: ... ms (...)`
* RustyStartup state/status lines where present, including mode, hit/miss, and reason surfaces

Observed useful local evidence includes:

* a fresh controlled rev1261 run with explicit startup timing lines
* adjacent historical local logs showing different Rusty status paths, including hit and miss cases

Interpretation:

* runtime logs are the primary current evidence source for baseline anchoring on this machine

### 2. Modlist/config control files (`ModsConfig.xml`, `ModsConfig.xml.backup`)

What is measurable:

* stable mod-count anchors and reproducible tier setup points
* current active set
* backup/near-vanilla smaller set

Observed anchor counts:

* `ModsConfig.xml`: `11` active mods
* `ModsConfig.xml.backup`: `4` active mods

Interpretation:

* these files are sufficient to define at least canonical small and medium benchmark tiers without inventing ad hoc loadouts

### 3. Local Rusty cache artifact history

Available local artifact-history sources include bundle directories with files such as:

* `compiled_summary.json`
* `meta.json`
* `defs.cache`
* `textures/manifest.json`

Observed historical workload-shape evidence includes mod-count ranges like:

* `9, 11, 12, 16, 22, 30, 72, 73, 112`

Interpretation:

* these artifacts are still valid for establishing realistic tier boundaries and workload-size expectations on this machine
* they are not equivalent to fresh timing runs, but they are useful for benchmark planning and test-matrix design

### 4. Historical archive-referenced instrumentation shape

Earlier evidence referenced instrumentation surfaces and phase bucket names from a legacy archive path.

Merged interpretation after follow-up closure:

* the general harness direction remains valid
* archive-only hooks and command surfaces are **historical claims** unless re-verified from currently present files
* do not treat the missing legacy archive as a current verified-local source of truth

## Fresh Local Baseline Data

### Controlled datapoint collected locally

Fresh controlled observation:

* local run date: March 11, 2026
* process: `RimWorldWin64.exe`
* source: updated `Player.log`

Extracted startup lines:

* `RimWorld 1.6.4633 rev1261`
* `- Loaded All Assemblies, in 0.401 seconds`
* `- Finished resetting the current domain, in 0.002 seconds`
* `UnloadTime: 1.913300 ms`
* `Total: 203.251600 ms (...)`

Interpretation constraint:

* this is a single fresh controlled observation used to anchor the canonical runtime baseline
* it is **not** a full statistical performance distribution

### Additional recent local evidence

Earlier local logs also showed examples such as:

* one run with `mode0` and `hitMiss=miss`
* one run with `mode1` and `hitMiss=hit`

Merged interpretation:

* this is enough to justify hit/miss/fallback-aware benchmark metrics in the implementation plan
* it is not enough to publish final comparative performance claims yet

## Reconciliation of Earlier Timing Evidence

### Still-valid evidence

These remain valid and usable for planning:

* current and prior local runtime logs
* local config tier anchors (`ModsConfig.xml`, `ModsConfig.xml.backup`)
* local Rusty cache artifact history for workload-size evidence
* general benchmark direction: tiered, mode-aware, hit/miss-aware startup measurement

### Stale or currently unverifiable evidence

These should no longer be treated as current verified-local source evidence:

* claims that depend exclusively on a missing legacy archive path
* hook surfaces or command surfaces that were only evidenced through that missing archive and not re-verified from present files

### Practical planning rule

Use the following evidence priority:

1. fresh runtime log evidence on `rev1261`
2. still-valid local artifact/config evidence
3. historical archive-referenced claims only as secondary design hints until re-verified

## Benchmark-Tier Definition

Canonical benchmark tiers for this machine going forward:

### Tier S (small)

* `4` active mods
* anchored by `ModsConfig.xml.backup`

### Tier M (medium/current)

* `11` active mods
* anchored by `ModsConfig.xml`

### Tier L (large)

* `16-50` active mods
* supported by local artifact-history evidence, not yet freshly timed in this document

### Tier X (extreme)

* `>50` active mods
* supported by local artifact-history evidence, not yet freshly timed in this document

### Comparability rule

For any benchmark comparison:

* keep ordered package IDs fixed
* do not compare across mod-order drift
* do not mix runtime revisions in one benchmark aggregate

## Vanilla Baseline Opportunities

### Cold start

Definition:

* launch after cache disturbance or without immediate prior run effects

Status:

* directly measurable on this machine
* not yet sampled enough in this merged document to form a full cold-start distribution

### Warm start

Definition:

* back-to-back launches with the same modlist and same environment

Status:

* directly measurable
* partially evidenced in local logs and strongly suitable for snapshot/replay comparisons

### Small baseline opportunities

* immediately measurable from the `4`-mod tier
* useful for low-noise sanity benchmarking and fallback validation

### Medium baseline opportunities

* immediately measurable from the `11`-mod tier
* this is the best default developer benchmark tier on the current machine

### Large/extreme opportunities

* workload-shape evidence exists locally through cache artifact history
* these tiers need fresh controlled timing runs later for milestone gating

## Phase-Level Measurement Opportunities

The benchmark harness should remain phase-aware, but this merged document distinguishes between:

* **verified current local timing sources**
* **planned phase buckets for implementation instrumentation**

### Verified current timing surfaces

Currently verified directly from local runtime logs:

* assembly-load timing
  n- domain reset timing
* Unity unload/cleanup timing lines when present
* Rusty status/hit/miss/fallback state lines when present

### Planned canonical phase buckets for implementation instrumentation

The implementation plan should still target phase buckets such as:

* mod discovery
* XML
* patching
* resolve
* texture decode
* atlas
* other

These remain valid planning buckets because they align with the startup ownership model, even if archive-only instrumentation code is not currently treated as verified-local evidence.

### Ownership relevance of phase buckets

These buckets matter because they separate:

* deterministic discovery cost
* semantic XML/patch/resolve cost
* replay hit-path cost
* validation cost
* fallback/degradation cost
* graphics/texture/atlas tail cost

That makes them directly useful for milestone gating under the Rusty Startup architecture.

## Benchmark Harness Plan

### Benchmark matrix

Use this matrix for implementation planning:

* tiers: `{S, M, L, X}`
* execution class: `{cold, warm}`
* Rusty mode state: `{mode0 miss, mode1 hit, mode2 hit-or-fallback}` where applicable

This yields a planning matrix of:

* `4 x 2 x 3 = 24` cells

### Repetition policy

Recommended minimum repetition:

* cold runs: `n=5` per relevant cell
* warm runs: `n=10` per relevant cell

This is still the right target for milestone-quality measurement, even though the current document contains only a small amount of fresh direct timing evidence.

### Per-run capture requirements

Per run, capture at minimum:

* runtime version banner line with revision
* ordered package IDs / active mod count
* assembly-load timing
* domain-reset timing
* unload/total timing lines when present
* Rusty status line including mode, hit/miss, and reason
* cache/replay metadata where available

### Row schema

Each benchmark row should include:

* `run_id`
* `timestamp_local`
* `runtime_revision`
* `tier`
* `thermal_state`
* `requested_mode`
* `effective_mode`
* `hit_miss`
* `reason`
* `wall_clock_ms`
* any available phase bucket timings
* `defs_cache_bytes`
* `manifest_bytes`
* `fallback_reason` or `none`

## Core Metrics

### 1. Wall-clock startup

Definition:

* elapsed startup duration to the chosen ready boundary or closest reproducible external timing marker

Use:

* primary user-facing KPI

### 2. Verified startup timing lines

Definition:

* assembly-load time
* domain-reset time
* unload/total timing block where present

Use:

* low-overhead coarse baseline metrics directly grounded in current local logs

### 3. Rusty mode outcome

Definition:

* `mode`, `hitMiss`, and `reason`

Use:

* essential for separating:

  * miss-path costs
  * replay-hit costs
  * invalidation/fallback costs

### 4. Snapshot/replay state metrics

Definition:

* cache sizes, manifest sizes, bundle metadata, and snapshot-validity context

Use:

* supports cost/performance interpretation for replay and invalidation behavior

### 5. Future phase-bucket metrics

Definition:

* mod discovery / XML / patching / resolve / texture / atlas / other bucket timings once current instrumentation is present and verified

Use:

* milestone diagnostics and ownership-aware optimization tracking

## Minimum Valid Measurement Set for Implementation Planning

The follow-up closure fixed the minimum valid set for planning on this machine.

Required data fields per run:

* runtime version banner including revision
* active mod count and ordered package IDs
* `Loaded All Assemblies` seconds
* `Finished resetting the current domain` seconds
* `UnloadTime` ms when present
* `Total: ... ms` line when present
* Rusty status line including `mode`, `hitMiss`, and `reason`

Minimum sample requirement for milestone planning gates:

* at least `3` warm runs for Tier S on `rev1261`
* at least `3` warm runs for Tier M on `rev1261`
* at least one observed miss and one observed hit where Rusty behavior applies
* no mixed-version aggregation

Fresh vs archived labeling rule:

* **fresh**: collected on or after the baseline-reconciliation memo date with explicit environment capture
* **archived**: prior logs or historical artifacts

## Success Criteria Suggestions

These are planning-grade milestone targets, not final public claims.

### Milestone type A: baseline integrity

Success means:

* benchmark runs are revision-locked to `rev1261`
* tier identity is stable
* mode/hit/miss/fallback state is recorded every run

### Milestone type B: snapshot/replay honesty

Success means:

* hit-path runs are benchmarked separately from miss-path and fallback-path runs
* benchmark output never collapses these into a single undifferentiated average

### Milestone type C: ownership-aware performance

Success means:

* later instrumentation can attribute performance to startup ownership buckets rather than only overall wall-clock
* regressions in validation/fallback costs are visible rather than hidden

## Threats to Measurement Validity

The following are real threats to reliable startup benchmarking on this machine:

1. Mixed-revision evidence

* solved at planning level by locking to `rev1261`
* still a risk if later runs are not revision-checked

2. Mod-order drift

* invalidates comparability across runs

3. Cold/warm contamination

* OS cache state and immediate prior launch history can distort comparisons

4. Logging/instrumentation drift

* archive-only hooks should not be assumed current unless re-verified

5. Artifact-history overinterpretation

* historical cache metadata is good for tier definition, not a substitute for fresh timing runs

6. Fallback-mode ambiguity

* hit, miss, invalidate, and fallback costs must be separated or benchmark conclusions become misleading

## Relationship to the Architecture Baseline

Category 7 is not a generic performance appendix. It is tied directly to the Rusty Startup architecture.

That means benchmarks must distinguish:

* deterministic ownership cost
* mixed-zone managed-assisted cost
* snapshot validation cost
* replay hit-path cost
* fallback/degraded-mode cost
* delegated live-tail costs where measurable

This is what makes the benchmark plan useful for implementation planning rather than just curiosity.

## Relationship to the Baseline Reconciliation Follow-Up

This merged Category 7 incorporates the follow-up in four direct ways:

1. The canonical planning runtime is now fixed to `rev1261`.
2. Static `Version.txt` mismatch no longer counts as an unresolved contradiction.
3. Fresh local runtime evidence is explicitly separated from archived evidence.
4. Archive-only instrumentation claims are downgraded to historical planning hints unless re-verified.

## Residual Risks Worth Tracking

These do not block the implementation plan, but they remain true:

1. A full statistical benchmark dataset across all tiers and modes still needs to be collected later.
2. Some phase-bucket instrumentation remains planning-level until re-verified from current source or rebuilt in the implementation.
3. Large and extreme tiers are still supported more strongly by artifact history than by fresh timing runs in this document.
4. Any future benchmark gate must remain revision-locked to avoid reintroducing the old ambiguity.

## Canonical Wording Going Forward

Use these phrases consistently:

* **canonical local planning baseline**: `RimWorld 1.6.4633 rev1261`
* **fresh runtime evidence**: authoritative for baseline locking on this machine
* **archived evidence**: useful for planning, but lower authority than fresh current runs
* **benchmark honesty**: separate hit, miss, invalidation, and fallback behavior in all comparisons

## Source Evidence Appendix

### Exact local version/build sources represented here

* `C:\Program Files (x86)\Steam\steamapps\common\RimWorld\Version.txt`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\Player.log`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\Player-prev.log`

### Exact local config/tier anchors represented here

* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\Config\ModsConfig.xml`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\Config\ModsConfig.xml.backup`

### Exact local artifact-history sources represented here

* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\RustyStartup\RustyCache\bundles\1.6\...\compiled_summary.json`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\RustyStartup\RustyCache\bundles\1.6\...\meta.json`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\RustyStartup\RustyCache\bundles\1.6\...\defs.cache`
* `C:\Users\lukep\AppData\LocalLow\Ludeon Studios\RimWorld by Ludeon Studios\RustyStartup\RustyCache\bundles\1.6\...\textures\manifest.json`

### Stale/missing archived reference represented here

* `C:\Users\lukep\source\repos\Rusty Startup\legacy\Safe V1 Package (ENTIRE PROJECT).zip`

### Follow-up findings incorporated here

* canonical local planning runtime fixed to `rev1261`
* fresh runtime evidence overrides conflicting static install metadata for planning on this machine
* archive-only instrumentation claims remain useful design hints but are not treated as currently verified-local evidence

### Representative commands/evidence types represented in the earlier audit work

* reading `Version.txt`
* filtering `Player.log` / `Player-prev.log` for runtime/timing/status lines
* launching and terminating a controlled RimWorld runtime process for fresh observation
* parsing `ModsConfig.xml` and `ModsConfig.xml.backup`
* inspecting local Rusty cache bundle metadata
