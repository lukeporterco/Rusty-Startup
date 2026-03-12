# SLICE-001-bootstrap-shell

## Objective

Create the thin managed shell boundary for package identity, path resolution shape, native loading shape, and Rust-core handshake shape.

## Milestone

Stage 1

## Allowed paths

- managed/**
- docs/context/Current Build Context.md
- roadmap/active_slice.yaml

## Forbidden paths

- native/**
- snapshot/**
- equivalence/**
- benchmark code

## Invariants

- The shell must not become a second owner of startup semantics.
- Shell responsibilities remain bootstrap, native loading, environment capture, diagnostics surfacing, and managed assistance only.

## Required diagnostics

- package identity
- native load path
- ABI handshake status
- startup entry status

## Required evidence

- bootstrap entry shape exists
- shell boundary responsibilities are explicit
- no semantic ownership moved into C#

## Exit criteria

- the shell boundary exists as a narrow, explicit layer
- responsibilities match the implementation plan
- the next slice can begin from a stable shell boundary

## Non-goals

- no replay logic
- no equivalence logic
- no mixed-zone semantic implementation