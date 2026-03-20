# Current Build Context

## Current phase

Stage 1: bootstrap spine, SLICE-003 formally complete

## Current active slice

SLICE-003-rimworld-shell-activation-and-rust-core-bootstrap

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The repository now has the canonical local `rustystartup.core` package scaffold, deterministic package-relative artifact placement, and a real vanilla RimWorld static-constructor startup entry that performs explicit native load, ABI validation, and Rust bootstrap activation while the managed shell remains a thin bootstrap and boundary layer. SLICE-003 is formally complete on the current Windows machine.

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
- startup-entry, native activation, and diagnostics hardening while the remaining host-program proof stays evidence-incomplete

## Current architectural warnings

- Do not let the shell become a semantic owner.
- Do not hardcode machine-specific paths into activation logic or packaging logic.
- Keep package identity locked to `rustystartup.core`.
- Keep this slice scoped to startup entry, package-relative local packaging, native loading, ABI validation, and Rust-core bootstrap only.
- SLICE-003 is formally complete on the current Windows machine.
- The active control-plane state remains on SLICE-003 because no next-slice control artifact has been introduced in `docs/slices/`.
