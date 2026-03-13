# Current Build Context

## Current phase

Stage 1: bootstrap spine

## Current active slice

SLICE-001-bootstrap-shell

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The thin managed bootstrap shell boundary is being established so package identity, path resolution shape, native loading shape, ABI handshake shape, startup entry shape, and diagnostics surfacing shape become real without introducing semantic ownership.

## In scope now

- managed shell boundary
- package identity shape
- path resolution shape
- native loading shape
- ABI handshake shape
- startup entry shape
- diagnostics surfacing shape
- scope tracking updates

## Explicitly out of scope now

- Rust engine semantics
- snapshot serialization
- replay logic
- equivalence logic
- mixed-zone semantic implementation
- benchmark harness implementation

## Current architectural warnings

- Do not let the shell become a semantic owner.
- Do not move world-model authority into managed code.
- Keep this slice scoped to the thin managed bootstrap boundary only.
