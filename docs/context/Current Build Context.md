# Current Build Context

## Current phase

Stage 1: bootstrap spine

## Current active slice

SLICE-002-package-root-load-folder-and-layout-resolution

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

Self-package root detection, active load-folder selection, and package-relative layout resolution are being established so the current 1.6 package/layout contract becomes real and explainable without turning the managed shell into the owner of global startup semantics.

## In scope now

- self-package root detection
- active load-folder selection
- package-relative layout resolution
- explicit resolution diagnostics
- scope tracking updates
- audit-first resolver-contract hardening because no test or fixture surface is currently inside SLICE-002 allowed paths

## Explicitly out of scope now

- Rust engine semantics
- snapshot serialization
- replay logic
- equivalence logic
- XML discovery or patch application
- benchmark harness implementation

## Current architectural warnings

- Do not let the shell become a semantic owner.
- Do not hardcode machine-specific paths into resolver logic.
- Keep package identity locked to `rustystartup.core`.
- Keep this slice scoped to root/load-folder/layout resolution only.