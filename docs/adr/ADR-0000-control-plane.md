# ADR-0000: Repo control plane before engine code

## Status

Accepted

## Context

Rusty Startup is architecture-sensitive and drift-prone if work is driven by freeform prompts or isolated local tickets.

The repo needs a control plane that makes project identity, current scope, and local work rules visible to Codex without requiring large in-chat context.

The category evidence layer must be explicit in the repository layout because it is the primary authority for real RimWorld startup behavior.

## Decision

Adopt a repo-resident control plane before engine code:
- the category evidence layer lives under `docs/authority/categories/`
- the three legacy authority docs remain in the root
- root `AGENTS.md` defines global repo rules
- nested `AGENTS.override.md` files define local rules
- roadmap and active slice files define current scope
- slice files define allowed paths and exit conditions
- architecture and process docs live under `docs/`
- PR template requires slice and gate declaration

## Consequences

Positive:
- lower context drift
- more consistent Codex behavior
- bounded work units
- clearer review surface

Negative:
- more upfront repo setup
- process files must be kept current