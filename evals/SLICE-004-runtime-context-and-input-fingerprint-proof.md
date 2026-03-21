# SLICE-004 Runtime Context and Input Fingerprint Proof

## Implemented Surface

- Rust now owns a narrow `RuntimeContext` model and runtime-input fingerprint report path.
- The bootstrap ABI includes an explicit runtime-context submission entrypoint separate from activation.
- Managed bootstrap code captures already-observable runtime facts and forwards them through a flat unmanaged fact block.
- Diagnostics report build identity, version basis, parser mode, language key, platform basis, self-package roots, machine-local path identities, and fingerprint outcome.

## Exercised Cases

- Fresh runtime evidence outranks static install metadata when both are present.
- Selected self-package root and active self content root normalize to stable path-identity values.
- Missing machine-local path identities keep the runtime fingerprint unavailable rather than canonical.
- Relative machine-local path identities are explicitly unsupported rather than silently normalized into canonical identity.
- Legacy parser mode is explicitly degraded and non-primary.
- Missing inputs stay explicit and diagnostic instead of silently defaulting.
- Fingerprint classes keep bootstrap-proven self-package inputs distinct from machine-local path inputs.
- Fingerprints change when machine-local path identity inputs change.
- Canonical fingerprints do not drift when only source-label or diagnostic reason wording changes.
- The explicit FFI/bootstrap round-trip returns the same runtime-context report through `build_runtime_context_report_from_ffi` and `rs_bootstrap_submit_runtime_context`.

## Validation

- `cargo fmt --check`
- `cargo check` in `native/`
- `cargo test` in `native/`
- `dotnet build managed/src/RustyStartup.Managed.csproj` with `RimWorldManagedDir` pointed at the local RimWorld managed assemblies directory

## Notes

- The managed build was initially environment-blocked until the RimWorld managed assemblies path was discovered locally.
- No scope beyond SLICE-004 was introduced.
