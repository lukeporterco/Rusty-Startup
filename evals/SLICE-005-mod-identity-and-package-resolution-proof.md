# SLICE-005 Mod Identity and Package Resolution Proof

## Scope
- Frozen contract: capability-gated new native symbol for package-discovery basis submission
- ABI version: 2
- Capability mask: `0x3` -> `0x7`
- Ownership boundary: managed passes discovery-basis facts only; native owns identity and package-resolution semantics

## Commands run
1. `cargo fmt --check` in `native/`
2. `cargo test` in `native/`
3. `dotnet build managed/src/RustyStartup.Managed.csproj -nologo` with `RimWorldManagedDir=C:\Program Files (x86)\Steam\steamapps\common\RimWorld\RimWorldWin64_Data\Managed`

## Test matrix
- Required-root validation and absolute-path handling
- Stable package-root normalization
- Self, official, local, and workshop origin classification
- About/About.xml metadata normalization for canonical identity and display-name basis
- PublishedFileId provenance handling
- Duplicate package ID conflict diagnostics
- Nested About tree diagnostics
- Missing package ID / missing metadata failure handling
- Stale `lukep.rustystartup` surfaced as stale-state evidence only
- Rendered diagnostic coverage for the package-discovery report

## Outcomes
- `cargo fmt --check`: pass
- `cargo test`: pass
- Managed build: pass

## Proof notes
- Native tests cover the contract boundary and the package-resolution report surface.
- Diagnostics emitted by the package-discovery report include the required package discovery basis, search-root status, discovered root, About metadata, PublishedFileId, dependency metadata, origin, identity, and final resolution result lines.
- `lukep.rustystartup` is handled as stale evidence only; it is not promoted to canonical identity.

## Completion state
- Evidence-complete
- No forbidden scope introduced
