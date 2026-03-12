# Definition of Done

A change is done only when all applicable items below are true.

## Slice completion

- The change satisfies the active slice objective.
- The change stayed inside allowed paths.
- The exit criteria in the active slice are met.

## Architectural completion

- No drift from authority docs was introduced.
- Responsibility boundaries remained intact.
- Any architecture-affecting decision is reflected in an ADR if needed.

## Evidence completion

- Diagnostics required by the slice are present.
- Any claimed lane, replay behavior, or equivalence result is observable.
- Any claimed concurrency behavior is observable.
- Any deferred work is stated explicitly.

## Gate awareness

Use the implementation-plan gates as the completion standard:
- Gate A: boundary correctness
- Gate B: equivalence honesty
- Gate C: snapshot honesty
- Gate D: lane honesty
- Gate E: package and bootstrap correctness
- Gate F: benchmark honesty
- Gate G: parallelism honesty