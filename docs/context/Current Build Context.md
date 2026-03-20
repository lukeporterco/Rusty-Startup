# Current Build Context

## Current phase

Stage 1: bootstrap spine

## Current active slice

SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The managed shell now exposes an explicit RimWorld startup activation path for `rustystartup.core` that:
- enters startup through the thin managed bootstrap layer without becoming a second semantic owner
- consumes the resolved self-package layout inputs established by the previous slice
- loads and validates the native core from a deterministic package-relative path
- verifies ABI compatibility before handing control to Rust
- emits structured, explainable activation and bootstrap diagnostics for success, fallback, and failure cases
- makes the repository itself usable as the canonical local `rustystartup.core` test package for current-machine RimWorld startup validation

## In scope now

- RimWorld startup entry wiring in the managed shell
- explicit Rust-core bootstrap handoff
- deterministic native library loading from the package-relative native path
- ABI validation and binding
- startup environment and runtime capture needed for activation
- explicit activation diagnostics and failure reporting
- use of resolved self-package layout inputs from SLICE-002
- canonical local package scaffold for `rustystartup.core` under package-relative paths
- automated placement of managed and native artifacts into the package-relative local test layout without per-iteration manual copy steps
- scope tracking updates
- implementation-complete bootstrap-contract hardening because no test or fixture surface is currently inside SLICE-003 allowed paths

## Current architectural warnings

- Do not let the shell become a semantic owner.
- Do not hardcode machine-specific paths into activation logic or packaging logic.
- Keep package identity locked to `rustystartup.core`.
- Keep this slice scoped to startup entry, package-relative local packaging, native loading, ABI validation, and Rust-core bootstrap only.
- This slice is implementation-complete but not evidence-complete until required proof surfaces exist in allowed paths.