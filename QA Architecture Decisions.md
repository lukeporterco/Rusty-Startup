# QA Architecture Decisions

## Baseline

Here is the baseline exactly as your answers define it.

## 1) Replacement Scope: C

Rusty Startup v1 is allowed to own everything that can be modeled deterministically before live managed behavior becomes unavoidable.

This means v1 is not limited to XML or defs only. It is allowed to take over the entire deterministic startup region up to the runtime frontier. That gives the project the scale it needs to be architecturally honest from the start.

## 2) Semantic-Equivalence Cutoff: B+

The correctness target is fully resolved def equivalence.

That means Rusty Startup must do more than match unified XML or partially parsed semantic state. It must match the fully resolved def-level results that vanilla startup would produce, but it does not have to reproduce the entire later startup state beyond that boundary.

This is an excellent target because it is deep enough to matter, but not so deep that the first implementation becomes trapped by every later runtime side effect.

## 3) Managed Ordering Conservatism: C

Rusty Startup may reorder managed-adjacent work where semantic equivalence can still be preserved.

This is a strong performance-oriented choice. It gives the execution graph real freedom instead of forcing vanilla-order imitation around every boundary. It also means the equivalence oracle becomes even more important, because reordered work must still prove it lands in the same resolved def state.

## 4) Snapshot Boundary: B

Snapshots should cover everything up to the runtime frontier, with only a narrow live-execution tail.

This fits the architecture very well. The snapshot system is now defined as a semantic replay system over the full deterministic region, not a shallow cache and not an unrealistic whole-process restore.

## 5) Unknown Managed Behavior Policy: Evidence-Based Aggressive Ownership

This is one of the most important decisions in the whole set.

Rusty Startup should aggressively attempt ownership and solving, but solver behavior must be grounded in real RimWorld mod data. That means decisions about what can be modeled, accelerated, or partially replaced should be based on actual mod ecosystems, not abstract optimism.

This gives the project an aggressive solver identity without making it blind or speculative.

## 6) Snapshot Portability Model: A

Snapshots are machine-local only.

This simplifies the entire storage and invalidation model. The mod remains plug-and-play, but the semantic snapshot artifacts do not need to be portable. That is a very practical choice and removes a lot of cross-machine ambiguity.

## 7) Minimum v1 Scope: D

v1 must already demonstrate the semantic snapshot/replay model.

This is a major architectural commitment, and it is the right one given the project's identity. It means v1 cannot merely replace deeper startup ownership in principle. It must already prove the checkpoint-and-replay concept in practice.

## 8) Fallback Posture: D- (Aligned with #5)

Rusty Startup should prefer solving, but solving must be grounded in real mod data.

This preserves your fail-open solver philosophy while placing an evidence requirement on it. In effect, fallback is not the default for uncertainty. The default is to solve aggressively, but only where that aggression is justified by actual RimWorld mod behavior.

## 9) Target Compatibility Claim: C

The v1 target is near-universal compatibility, with only rare pathological cases expected to fall back.

This sets a very ambitious test burden, but it also matches the stated vision. The project is not aiming to be a niche optimizer for easy modlists. It is aiming to work with the real mod ecosystem.

## 10) Authority of the Internal World-Model: C

The Rusty Startup internal world-model is the primary authority up to the runtime frontier.

This is one of the defining architecture choices. It means the internal model is not advisory. It is the actual semantic source of truth for the deterministic startup region. That keeps the architecture replacement-oriented and prevents drift back toward a vanilla-centered wrapper design.

## 11) Diagnostics and Observability Strictness: C+

Diagnostics must provide strong explainability, slightly beyond normal "full explainability" expectations, but not all the way to maximal observability as a core product unto itself.

Practically, this means Rusty Startup must be able to explain ownership decisions, replay decisions, delegation decisions, invalidation causes, and equivalence failures in a way that supports development and compatibility work. This will be a core implementation requirement, not a late add-on.

## 12) Performance Versus Conservatism: C

The project is performance-first as long as semantic correctness remains plausibly maintainable.

This keeps the tone of the project aligned with your original vision. Rusty Startup is not a cautious academic reimplementation. It is an aggressive performance engine anchored by a resolved-def equivalence target.

## Combined Interpretation

These answers produce a very clear architectural identity:

> Rusty Startup v1 is a Rust-first, replacement-oriented startup engine that owns the full deterministic startup region up to a narrow runtime frontier, uses an authoritative internal semantic world-model, targets fully resolved def equivalence to vanilla, allows reordering where equivalence is preserved, aggressively solves compatibility problems using real mod ecosystem evidence, stores machine-local semantic snapshots for replay, and must prove the snapshot/replay model in its first real release while aiming for near-universal compatibility.

That is now specific enough to drive a real implementation plan.

Just as importantly, the decisions are internally consistent. There are no major contradictions between them. For example, 1C, 4B, 7D, and 10C all reinforce each other. 5, 8, and 9 also reinforce each other. 2B+ gives the project a deep but still tractable equivalence target. 3C and 12C preserve room for serious optimization.

## What Changes Next

Because these decisions are now fixed, the next implementation plan should be built around five hard deliverables:

1. A RimWorld 1.6 ownership map that divides the startup chain into Rust-owned deterministic phases, runtime-frontier phases, and delegated live-tail phases.
2. A resolved-def equivalence oracle that defines exactly what must match vanilla at the chosen cutoff.
3. A semantic snapshot model that covers the entire deterministic region up to that runtime frontier.
4. An evidence-based compatibility classifier grounded in real RimWorld mod data, including Workshop mod behavior.
5. A milestone ladder where v1 already includes snapshot/replay, not just deeper startup takeover.

Those are now the immovable pillars of the implementation plan.