# Current Build Context

## Current phase

Stage 2: authority spine, SLICE-004 active

## Current active slice

SLICE-004-runtime-context-and-input-fingerprint-model

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The accepted working project state is post-`SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap`. The repository now moves into the first Stage 2 authority-spine slice.

This slice makes the native-owned `RuntimeContext` model and runtime-input fingerprint contract real on top of the stable bootstrap boundary already established by Stage 1. The managed shell remains a thin bootstrap and boundary layer that surfaces already-observable runtime facts into Rust, while the Rust core becomes the canonical owner of runtime-context truth and runtime-input fingerprint semantics.

## In scope now

- native-owned `RuntimeContext` modeling
- explicit bootstrap-to-native runtime input contract
- normalization of runtime version or revision basis, parser mode, language or localization, platform or architecture, and bootstrap-proven self-package inputs
- runtime-input fingerprint classes and fingerprint result modeling
- diagnostics for observed, normalized, fingerprinted, unknown, unavailable, and unsupported runtime-input classes
- proof surfaces for normalization and fingerprint behavior under `native/tests/**` or `evals/**`
- scope tracking updates

## Current architectural warnings

- Do not let the managed shell become the owner of runtime-context truth, package identity, mod identity, content-root selection, or generalized discovery semantics.
- Do not reopen self-package layout resolution from `SLICE-002-package-root-load-folder-and-layout-resolution` or bootstrap activation ownership from `SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap`.
- Keep machine-local runtime or environment keys distinct from portable or package-derived inputs in the fingerprint model.
- Keep parser mode explicit and fingerprinted, but do not pull parser-lane semantics or raw XML discovery into this slice.
- Do not introduce generalized multi-mod package resolution, deterministic modset modeling, raw XML discovery, snapshot storage, replay, equivalence, or mixed-zone execution planning here.
- Unknown or unsupported runtime inputs must remain explicit and diagnostic rather than silently defaulted into false parity claims.