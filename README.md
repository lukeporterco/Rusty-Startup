# Rusty Startup

Rusty Startup is a Rust-first, replacement-oriented RimWorld startup engine.

Its job is to reach vanilla-equivalent startup meaning faster than vanilla, while remaining plug-and-play and broadly compatible with existing mods.

## Authority order

The files below are the project authority stack and should be read in this order:

1. `docs/authority/categories/Category 1.md`
2. `docs/authority/categories/Category 2.md`
3. `docs/authority/categories/Category 3.md`
4. `docs/authority/categories/Category 4.md`
5. `docs/authority/categories/Category 5.md`
6. `docs/authority/categories/Category 6.md`
7. `docs/authority/categories/Category 7.md`
8. `QA Architecture Decisions.md`
9. `RS Project Plan.md`
10. `Rusty Startup Implementation Plan.md`

Operational execution is then governed by:

11. `docs/Execution Roadmap.md`
12. `docs/context/Current Build Context.md`
13. `roadmap/active_slice.yaml`
14. The active slice file in `docs/slices/`

## Core rule

The category documents establish what RimWorld 1.6 actually does.
The plan answers what must be true for Rusty Startup.
The roadmap answers in what order do we make those truths real.

## Working model

- Rust owns startup semantics through the cutoff.
- C# is a thin shell and managed bridge only.
- The internal world-model is authoritative up to the runtime frontier.
- Diagnostics are mandatory.
- Snapshot and replay are part of v1, not a later add-on.
- Parallelism must be real and observable, not aspirational.

## Repo posture

This repository uses repo-resident instructions so Codex can operate with minimal in-chat context.