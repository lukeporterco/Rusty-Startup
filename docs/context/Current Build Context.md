# Current Build Context

## Current phase

Stage 2: authority spine, SLICE-005 active

## Current active slice

SLICE-005-mod-identity-and-package-resolution-model

## Current authority layout

- Category evidence lives under `docs/authority/categories/`
- QA and project/implementation authority docs remain in the repo root

## What is being made true now

The accepted working project state is post-`SLICE-004-runtime-context-and-input-fingerprint-model`. The repository remains in Stage 2, and `SLICE-005-mod-identity-and-package-resolution-model` is now the active implementation slice.

`SLICE-004-runtime-context-and-input-fingerprint-model` made the native-owned `RuntimeContext` model and runtime-input fingerprint contract real on top of the stable bootstrap boundary already established by Stage 1. `SLICE-005-mod-identity-and-package-resolution-model` now builds on that basis to make native-owned mod identity and generalized package-resolution truth real, without turning the managed shell into the owner of package identity, active mod selection, or world-model semantics.

## In scope now

- native-owned `ModIdentity` or equivalent package identity modeling
- generalized package root discovery and package-resolution inputs for self, official, local, and Workshop package origins
- canonical package identity normalization from `About/About.xml` and related package metadata
- package origin and provenance classification, including Workshop-style numeric roots and `PublishedFileId` evidence where present
- duplicate, nested, stale, missing, or ambiguous package identity/root detection with explicit diagnostics
- narrow bootstrap-to-native inputs needed to describe package search roots or discovery environment
- proof surfaces for package identity and package-resolution behavior under `native/tests/**` or `evals/**`
- scope tracking updates

## Current architectural warnings

- Do not let the managed shell become the owner of generalized package identity, mod identity, package resolution, active mod selection, or world-model truth.
- Do not reopen self-package layout resolution from `SLICE-002-package-root-load-folder-and-layout-resolution` or runtime-context ownership from `SLICE-004-runtime-context-and-input-fingerprint-model`.
- Do not commit deterministic active-mod ordering or per-mod selected content roots in this slice; those belong to later Stage 2 work.
- Do not introduce raw XML discovery, patch application, snapshot storage, replay, equivalence, or mixed-zone execution planning here.
- Duplicate or nested `About/About.xml` trees, stale historical package IDs, and ambiguous package-root cases must remain explicit and diagnostic rather than silently collapsed.
- Local development layouts may be useful evidence, but they must not be mistaken for the shipping or Workshop contract.