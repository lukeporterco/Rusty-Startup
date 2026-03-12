# Boundary Map

## Entry boundary

Unity bootstrap before `Root_Entry.Start` is outside v1 ownership.

## Split-boundary model

### Rust-owned deterministic pre-mixed zone
Rust owns deterministic discovery, normalization, patching, fingerprinting, snapshot planning, and diagnostics.

### Managed-assisted but semantically Rust-owned mixed zone
The mixed zone begins at `CreateModClasses` and extends through `GiveAllShortHashes()`.

### Delegated live tail
Anything after the cutoff remains delegated in v1.

## Cutoff

The semantic cutoff is immediately after `ShortHashGiver.GiveAllShortHashes()` returns.

## Claim boundary

Rusty Startup may claim resolved-def equivalence through the cutoff.
It may not claim full startup parity beyond the cutoff.