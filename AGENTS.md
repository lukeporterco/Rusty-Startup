# Rusty Startup repository instructions

## Read order before making changes

1. `docs/authority/categories/Category 1.md`
2. `docs/authority/categories/Category 2.md`
3. `docs/authority/categories/Category 3.md`
4. `docs/authority/categories/Category 4.md`
5. `docs/authority/categories/Category 5.md`
6. `docs/authority/categories/Category 6.md`
7. `docs/authority/categories/Category 7.md`
8. `QA Architecture Decisions.md`
9. `RS Project Plan.md`
10. `Rusty Startup Implementation Plan.md`
11. `docs/Execution Roadmap.md`
12. `docs/context/Current Build Context.md`
13. `roadmap/active_slice.yaml`
14. The active slice file under `docs/slices/`

## Repository identity

Rusty Startup is a replacement-oriented, Rust-first RimWorld startup engine.

It is not a wrapper-first helper around vanilla startup.
It is not a C#-orchestrated system with Rust helpers.
It is not allowed to drift from the split-boundary model.

## Hard architectural rules

- Rust is the semantic owner through the cutoff.
- C# is the explicit bootstrap, native boundary, and mixed-zone assistance layer only.
- The internal world-model is authoritative up to the runtime frontier.
- Primary correctness is semantic equivalence to vanilla startup output at the resolved-def cutoff.
- v1 must already demonstrate machine-local snapshot and replay.
- Diagnostics are required for ownership, replay, invalidation, fallback, parser mode, and mixed-zone decisions.
- Parallelism must be operationalized honestly and visible in diagnostics.
- Do not claim parity past the cutoff.
- Do not silently treat degraded lanes as primary parity.
- Do not move responsibilities from Rust to C# for convenience.

## Work unit rules

- Only work inside the active slice.
- Only modify files listed in the active slice allowed paths.
- If a requested change exceeds the active slice, stop and say so.
- Do not invent new architecture that conflicts with the authority docs.
- Do not create giant mixed-purpose patches.
- Prefer small, coherent diffs that preserve module boundaries.

## File discipline

- Keep `QA Architecture Decisions.md`, `RS Project Plan.md`, and `Rusty Startup Implementation Plan.md` in the repo root unless an explicit repository reorganization slice says otherwise.
- Keep Category 1 through Category 7 under `docs/authority/categories/`.
- Put process and architecture control files under `docs/` and `roadmap/`.
- Put managed-shell-specific instructions under `managed/`.
- Put Rust-core-specific instructions under `native/`.

## Completion discipline

A change is not done because code compiles.
A change is done only when the active slice exit conditions are satisfied and the definition of done is met.