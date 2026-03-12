# ADR Policy

Use an ADR when a decision changes durable architecture.

## ADR required for

- ownership boundary changes
- replay contract changes
- equivalence contract changes
- lane model changes
- module-boundary changes
- storage-format commitments
- compatibility classifier commitments
- benchmark interpretation commitments

## ADR not required for

- local implementation details inside an approved slice
- pure refactors that preserve architecture
- typo fixes and formatting changes

## Naming

`ADR-XXXX-short-title.md`

## Rule

Do not silently rewrite architectural history.
Add a new ADR when a previous decision is superseded.