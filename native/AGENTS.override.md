# Native core local instructions

You are inside the Rust-core area.

## Local rules

- Preserve world-model authority here.
- Prefer explicit phase boundaries over convenience abstractions.
- Preserve deterministic reducers and deterministic commits.
- Keep parallelism phase-aware and honest.
- Do not move semantic ownership out to managed code.
- Do not blur replay, equivalence, diagnostics, and compatibility boundaries.

## Architectural center of gravity

This area is the semantic owner through the cutoff.