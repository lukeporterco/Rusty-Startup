# Current Build Context

## Current phase

Stage 0: repo control plane

## Current active slice

SLICE-000-control-plane

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The repository becomes self-describing enough that Codex can work from repo context instead of chat context.

## In scope now

- repo instructions
- authority layout
- roadmap
- slice structure
- ADR policy
- module map
- boundary map
- managed/native directory instructions
- PR template
- Codex rules
- process docs

## Explicitly out of scope now

- Rust engine code
- C# shell code
- workflow automation code
- snapshot serialization code
- equivalence implementation
- benchmark harness implementation

## Current architectural warnings

- Do not let the shell become a semantic owner.
- Do not create code before the control plane exists.
- Do not create giant architecture documents that compete with the implementation plan.