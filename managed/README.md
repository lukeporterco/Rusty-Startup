# Managed shell area

This directory will contain the thin C# shell only.

## Allowed long-term responsibilities

- package discovery and path resolution
- environment capture
- deterministic native load path handling
- ABI validation and binding
- startup entry into Rust
- mixed-zone managed assistance
- diagnostics surfacing

## Forbidden architectural direction

This area must not become a second owner of startup semantics.
Do not move world-model authority, replay authority, equivalence authority, or execution planning authority into managed code.