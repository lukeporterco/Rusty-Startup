# SLICE-000-control-plane

## Objective

Create the repo-resident project control plane so Codex can operate with minimal in-chat context and bounded architectural drift.

## Milestone

Stage 0

## Allowed paths

- README.md
- AGENTS.md
- docs/authority/README.md
- docs/authority/categories/Category 1.md
- docs/authority/categories/Category 2.md
- docs/authority/categories/Category 3.md
- docs/authority/categories/Category 4.md
- docs/authority/categories/Category 5.md
- docs/authority/categories/Category 6.md
- docs/authority/categories/Category 7.md
- .github/pull_request_template.md
- .github/workflows/README.md
- codex/rules/default.rules
- docs/Execution Roadmap.md
- docs/architecture/Boundary Map.md
- docs/architecture/Module Map.md
- docs/architecture/Data Model.md
- docs/process/Definition of Done.md
- docs/process/ADR Policy.md
- docs/context/Current Build Context.md
- docs/slices/_template.md
- docs/slices/SLICE-000-control-plane.md
- docs/slices/SLICE-001-bootstrap-shell.md
- docs/adr/ADR-0000-control-plane.md
- roadmap/roadmap.yaml
- roadmap/active_slice.yaml
- managed/README.md
- managed/AGENTS.override.md
- native/README.md
- native/AGENTS.override.md
- evals/README.md
- tools/README.md

## Forbidden paths

- all engine source files
- all package assets
- all build scripts
- all benchmark code

## Invariants

- Keep `QA Architecture Decisions.md`, `RS Project Plan.md`, and `Rusty Startup Implementation Plan.md` in place.
- Treat Category 1 through Category 7 as the primary authority evidence layer and keep them together under `docs/authority/categories/`.
- Do not introduce engine code.
- Do not weaken the split-boundary model.
- Do not expand C# ownership.

## Required diagnostics

None yet. This slice is documentation and control-plane only.

## Required evidence

- Repo tree exists
- Active slice declared
- Root and nested AGENTS files exist
- Roadmap exists
- PR template exists

## Exit criteria

- Codex can infer project identity from repo files alone
- Codex can infer current scope from `roadmap/active_slice.yaml` and this slice file
- Managed and native areas have local instructions
- The repository now has a stable execution-control backbone

## Non-goals

- no Rust code
- no C# code
- no CI workflow logic
- no benchmark logic