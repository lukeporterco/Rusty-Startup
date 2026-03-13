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

The canonical stage and slice registry lives in `roadmap/roadmap.yaml`. Slice specifications live under `docs/slices/`.

## Current execution order

### Stage 0: repo control plane
Create the repo-resident control system that minimizes in-chat context and constrains Codex to bounded work.

### Stage 1: bootstrap spine
Create package identity, package/layout resolution, native loading path strategy, ABI handshake shape, and startup-entry shell activation.

### Stage 2: authority spine
Create the runtime context, package and mod identity model, deterministic modset model, raw XML discovery surface, and scheduler/reducer contract.

### Stage 3: deterministic semantic spine
Create XML combination, patch application, parser-lane handling, inheritance resolution, and deterministic XML phase barriers.

### Stage 4: mixed-zone bridge
Create the managed-assisted boundary from `CreateModClasses` through `R9` and the Category 3 cutoff equivalence boundary.

### Stage 5: snapshot and replay
Create snapshot manifests, storage contract, invalidation logic, replay restore behavior, and bridge-visible replay status surfaces.

### Stage 6: compatibility and benchmark hardening
Expand evidence-based compatibility validation, fallback honesty, benchmark evidence, and parallelism honesty.

## Non-negotiable sequencing rules

- Do not start with snapshots before the bootstrap and authority spines exist.
- Do not start with mixed-zone ownership before the deterministic spine exists.
- Do not let C# become a second semantic owner.
- Do not implement parallelism as a placeholder-only concept.
- Do not let degraded paths silently masquerade as primary parity paths.

## Active execution rule

Only one slice is active at a time unless the roadmap explicitly declares a stack of dependent slices.
The active slice is declared in `roadmap/active_slice.yaml`.