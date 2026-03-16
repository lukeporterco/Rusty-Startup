# Current Build Context

## Current phase

Stage 1: bootstrap spine

## Current active slice

SLICE-002-package-root-load-folder-and-layout-resolution

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The managed shell now exposes a stable bootstrap-local self-package layout resolver contract for `rustystartup.core` that:
- resolves one canonical package root from bootstrap hints and managed self-assembly location
- selects one selected active content root for the current runtime version basis (`root-only`, `version-folder`, or `LoadFolders.xml`-routed)
- derives both managed assembly path and native payload path from that same selected active content root
- emits structured, explainable resolver outputs and failure reasons for unsupported or ambiguous layouts

## In scope now

- self-package root detection
- active load-folder selection
- package-relative layout resolution
- explicit resolution diagnostics
- startup-entry input/runtime basis extension for resolver input
- explicit self-package `LoadFolders.xml` evidence handling
- explicit duplicate/nested self-identity failure handling
- explicit unsupported/ambiguous layout failure handling
- scope tracking updates
- implementation-complete resolver-contract hardening because no test or fixture surface is currently inside SLICE-002 allowed paths

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
- Keep this slice scoped to bootstrap-local self-package root/load-folder/layout resolution only.
- This slice is implementation-complete but not evidence-complete until required proof surfaces exist in allowed paths.
