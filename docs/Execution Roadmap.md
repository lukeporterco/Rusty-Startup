# Execution Roadmap

## Purpose

The implementation plan answers what must be true.
The roadmap answers in what order do we make those truths real.

The roadmap is subordinate to the authority docs and may not override them.

## Authority relationship

The category documents under `docs/authority/categories/` are the grounded evidence layer.
The QA decisions lock product and architecture choices.
The implementation plan translates those authorities into a concrete architecture.
This roadmap does not override any of them.

## Current execution order

### Stage 0: repo control plane
Create the repo-resident control system that minimizes in-chat context and constrains Codex to bounded work.

### Stage 1: bootstrap spine
Create package identity, native loading path strategy, ABI handshake shape, and startup-entry shell boundaries.

### Stage 2: authority spine
Create the world-model, package resolution, modset model, and deterministic discovery surfaces.

### Stage 3: deterministic semantic spine
Create XML pipeline, parser-lane handling, def pipeline, and deterministic reducers.

### Stage 4: mixed-zone bridge
Create the managed-assisted boundary from `CreateModClasses` through the cutoff.

### Stage 5: equivalence and replay
Create replay manifests, restore logic, and cutoff equivalence proof.

### Stage 6: compatibility and benchmark hardening
Expand corpus validation, fallback honesty, and benchmark evidence.

## Non-negotiable sequencing rules

- Do not start with snapshots before the bootstrap and authority spines exist.
- Do not start with mixed-zone ownership before the deterministic spine exists.
- Do not let C# become a second semantic owner.
- Do not implement parallelism as a placeholder-only concept.
- Do not let degraded paths silently masquerade as primary parity paths.

## Active execution rule

Only one slice is active at a time unless the roadmap explicitly declares a stack of dependent slices.
The active slice is declared in `roadmap/active_slice.yaml`.