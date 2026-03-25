# Current Build Context

## Current phase

Stage 2: authority spine, SLICE-006 active

## Current active slice

SLICE-006-deterministic-modset-and-content-root-model

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The accepted working project state is Stage 2 with `SLICE-006-deterministic-modset-and-content-root-model` active. The repository remains in Stage 2, and the active implementation slice is now declared.

`SLICE-004-runtime-context-and-input-fingerprint-model` made the native-owned `RuntimeContext` model and runtime-input fingerprint contract real on top of the stable bootstrap boundary already established by Stage 1. `SLICE-005-mod-identity-and-package-resolution-model` completed the native-owned package-discovery contract, package identity model, and generalized package-resolution truth on top of that basis, without turning the managed shell into the owner of package identity, active mod selection, or world-model semantics. `SLICE-006-deterministic-modset-and-content-root-model` now consumes those stable outputs to make deterministic modset state and selected content-root truth real.

## In scope now

- control-plane consistency tracking
- completed proof surfaces for package identity and package-resolution behavior under `native/tests/**` and `evals/**`
- repository-state handoff for the active SLICE-006 slice

## Current architectural warnings

- Do not let the managed shell become the owner of generalized package identity, mod identity, package resolution, active mod selection, or world-model truth.
- Do not reopen self-package layout resolution from `SLICE-002-package-root-load-folder-and-layout-resolution` or runtime-context ownership from `SLICE-004-runtime-context-and-input-fingerprint-model`.
- Do not reopen package identity or package-resolution ownership from `SLICE-005-mod-identity-and-package-resolution-model`.
- Do not introduce raw XML discovery, patch application, snapshot storage, replay, equivalence, scheduler/reducer work, or mixed-zone execution planning here.
- Do not widen scope beyond deterministic modset and selected content-root truth for `R0`.
- Duplicate or nested `About/About.xml` trees, stale historical package IDs, and ambiguous package-root cases must remain explicit and diagnostic rather than silently collapsed.
- Local development layouts may be useful evidence, but they must not be mistaken for the shipping or Workshop contract.
