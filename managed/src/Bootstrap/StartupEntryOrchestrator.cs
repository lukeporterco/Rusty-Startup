using System;
using System.Collections.Generic;
using RustyStartup.Managed.Boundary;
using RustyStartup.Managed.Diagnostics;
using RustyStartup.Managed.Interop;

namespace RustyStartup.Managed.Bootstrap
{
    public static class StartupEntryOrchestrator
    {
        public static StartupEntryResult Run(StartupEntryInput input, BootstrapDiagnostics diagnostics)
        {
            try
            {
                diagnostics.Emit(
                    surface: "startup_entry_status",
                    status: "begin",
                    message: "Managed startup entry orchestration started.",
                    data: new Dictionary<string, string>
                    {
                        { "startupEntrySource", input.StartupEntrySource },
                    });

                var identity = PackageIdentity.Validate(input.ObservedPackageId, input.ObservedPackageIdSource);
                diagnostics.Emit(
                    surface: "package_identity",
                    status: identity.IsMatch ? "ok" : "mismatch",
                    message: identity.IsMatch ? "Package identity matched lock." : "Package identity mismatch.",
                    data: new Dictionary<string, string>
                    {
                        { "expected", identity.ExpectedPackageId },
                        { "observed", identity.ObservedPackageId },
                        { "source", identity.ObservedSource },
                        { "startupEntrySource", input.StartupEntrySource },
                    });

                if (!identity.IsMatch)
                {
                    return Fallback(
                        diagnostics,
                        "package_identity_mismatch",
                        "Package identity does not match the locked v1 identity.");
                }

                var effectiveSelfAssemblyPath = string.IsNullOrWhiteSpace(input.ManagedSelfAssemblyPath)
                    ? typeof(RustyStartupMod).Assembly.Location
                    : input.ManagedSelfAssemblyPath!;

                SelfPackageLayoutResolution resolution;
                try
                {
                    resolution = SelfPackageLayoutResolver.Resolve(
                        new SelfPackageLayoutResolverInput(
                            input.ResolvedModRootPath,
                            identity.ObservedPackageId,
                            identity.ObservedSource,
                            input.RuntimeVersionBasis,
                            input.RuntimeVersionSource,
                            effectiveSelfAssemblyPath));
                }
                catch (Exception ex)
                {
                    return Failure(
                        diagnostics,
                        "self_package_layout_resolution_failed",
                        "Self-package layout resolution failed: " + ex.Message);
                }

                diagnostics.Emit(
                    surface: "runtime_version_basis",
                    status: "observed",
                    message: "Observed runtime build or version basis for selected active content root resolution.",
                    data: new Dictionary<string, string>
                    {
                        { "runtimeVersionBasis", resolution.RuntimeVersionBasis },
                        { "runtimeVersionSource", resolution.RuntimeVersionSource },
                        { "startupEntrySource", input.StartupEntrySource },
                    });

                diagnostics.Emit(
                    surface: "package_root_resolution",
                    status: resolution.IsResolved ? "resolved" : "failed",
                    message: "Self-package package root resolution result.",
                    data: new Dictionary<string, string>
                    {
                        { "observedPackageId", resolution.ObservedPackageId },
                        { "observedPackageIdSource", resolution.ObservedPackageIdSource },
                        { "packageRoot", resolution.PackageRoot ?? "none" },
                        { "packageRootDetectionSource", resolution.PackageRootDetectionSource ?? "none" },
                        { "resolvedModRoot", input.ResolvedModRootPath },
                    });

                diagnostics.Emit(
                    surface: "active_content_root_selection",
                    status: resolution.IsResolved ? "selected" : "failed",
                    message: "Selected active content root and layout decision basis.",
                    data: new Dictionary<string, string>
                    {
                        { "selectedActiveContentRoot", resolution.SelectedActiveContentRoot ?? "none" },
                        { "layoutDecisionBasis", resolution.LayoutDecisionBasis ?? "none" },
                        { "loadFolderGateStatus", resolution.LoadFolderGateStatus },
                        { "loadFoldersEvidencePath", resolution.LoadFoldersEvidencePath ?? "none" },
                        { "loadFoldersEvidenceStatus", resolution.LoadFoldersEvidenceStatus },
                    });

                diagnostics.Emit(
                    surface: "managed_assembly_path",
                    status: resolution.ManagedAssemblyExists ? "resolved" : "missing",
                    message: "Managed assembly path resolved from selected active content root.",
                    data: new Dictionary<string, string>
                    {
                        { "absoluteManagedAssemblyPath", resolution.AbsoluteManagedAssemblyPath ?? "none" },
                        { "packageRelativeManagedAssemblyPath", resolution.PackageRelativeManagedAssemblyPath ?? "none" },
                        { "managedSelfAssemblyPath", effectiveSelfAssemblyPath },
                        { "managedSelfAssemblySource", input.ManagedSelfAssemblySource },
                        { "exists", resolution.ManagedAssemblyExists ? "true" : "false" },
                    });

                diagnostics.Emit(
                    surface: "native_payload_path",
                    status: resolution.NativePayloadExists ? "resolved" : "missing",
                    message: "Native payload path resolved from selected active content root.",
                    data: new Dictionary<string, string>
                    {
                        { "absoluteNativePayloadPath", resolution.AbsoluteNativePayloadPath ?? "none" },
                        { "packageRelativeNativePayloadPath", resolution.PackageRelativeNativePayloadPath ?? "none" },
                        { "platform", resolution.Platform?.ToString() ?? "none" },
                        { "runtimeIdentifier", resolution.RuntimeIdentifier ?? "none" },
                        { "nativeFileName", resolution.NativeFileName ?? "none" },
                        { "exists", resolution.NativePayloadExists ? "true" : "false" },
                    });

                if (!resolution.IsResolved)
                {
                    return Fallback(
                        diagnostics,
                        resolution.FailureReasonCode,
                        resolution.FailureReason);
                }

                if (!resolution.NativePayloadExists || string.IsNullOrWhiteSpace(resolution.AbsoluteNativePayloadPath) || resolution.Platform == null)
                {
                    diagnostics.Emit(
                        surface: "native_load_result",
                        status: "failure",
                        message: "Native payload is missing at the selected active content root path.",
                        data: new Dictionary<string, string>
                        {
                            { "reasonCode", "native_payload_missing" },
                            { "failureReason", "Native payload is missing at the selected active content root path." },
                            { "absoluteNativePayloadPath", resolution.AbsoluteNativePayloadPath ?? "none" },
                            { "packageRelativeNativePayloadPath", resolution.PackageRelativeNativePayloadPath ?? "none" },
                        });

                    return Failure(
                        diagnostics,
                        "native_payload_missing",
                        "Native payload is missing at the selected active content root path.");
                }

                var resolvedNativePayloadPath = resolution.AbsoluteNativePayloadPath!;
                if (!NativeLoader.TryLoad(
                        resolvedNativePayloadPath,
                        resolution.Platform.Value,
                        out var nativeLibraryHandle,
                        out var loadError))
                {
                    diagnostics.Emit(
                        surface: "native_load_result",
                        status: "failure",
                        message: loadError ?? "Native load failed.",
                        data: new Dictionary<string, string>
                        {
                            { "reasonCode", "native_load_failed" },
                            { "failureReason", loadError ?? "Native load failed." },
                            { "absoluteNativePayloadPath", resolvedNativePayloadPath },
                            { "packageRelativeNativePayloadPath", resolution.PackageRelativeNativePayloadPath ?? "none" },
                            { "platform", resolution.Platform.Value.ToString() },
                        });

                    return Failure(
                        diagnostics,
                        "native_load_failed",
                        loadError ?? "Native load failed.");
                }

                diagnostics.Emit(
                    surface: "native_load_result",
                    status: "loaded",
                    message: "Native library loaded successfully.",
                    data: new Dictionary<string, string>
                    {
                        { "absoluteNativePayloadPath", resolvedNativePayloadPath },
                        { "packageRelativeNativePayloadPath", resolution.PackageRelativeNativePayloadPath ?? "none" },
                        { "platform", resolution.Platform.Value.ToString() },
                        { "runtimeIdentifier", resolution.RuntimeIdentifier ?? "none" },
                        { "nativeFileName", resolution.NativeFileName ?? "none" },
                    });

                if (!NativeBootstrapBindings.TryBind(nativeLibraryHandle!, out var bindings, out var bindError))
                {
                    nativeLibraryHandle!.Dispose();
                    return Failure(
                        diagnostics,
                        "abi_bind_failed",
                        bindError ?? "Native ABI symbol binding failed.");
                }

                var abi = AbiHandshake.Validate(bindings!);
                diagnostics.Emit(
                    surface: "abi_handshake_status",
                    status: abi.IsCompatible ? "ok" : "incompatible",
                    message: abi.IsCompatible ? "ABI handshake succeeded." : "ABI handshake failed.",
                    data: new Dictionary<string, string>
                    {
                        { "expectedAbiVersion", AbiHandshake.ExpectedAbiVersion.ToString() },
                        { "requiredCapabilitiesMask", AbiHandshake.RequiredCapabilitiesMask.ToString() },
                        { "reportedAbiVersion", abi.ReportedAbiVersion.ToString() },
                        { "reportedCapabilities", abi.ReportedCapabilities.ToString() },
                        { "reason", abi.Reason },
                        { "startupEntrySource", input.StartupEntrySource },
                    });

                if (!abi.IsCompatible)
                {
                    nativeLibraryHandle!.Dispose();
                    return Failure(
                        diagnostics,
                        "abi_incompatible",
                        abi.Reason);
                }

                var activationResult = bindings!.Activate();
                diagnostics.Emit(
                    surface: "rust_core_activation_status",
                    status: activationResult == 0 ? "ok" : "failure",
                    message: activationResult == 0 ? "Rust bootstrap activation returned success." : "Rust bootstrap activation returned non-zero status.",
                    data: new Dictionary<string, string>
                    {
                        { "activationResult", activationResult.ToString() },
                        { "activationEntry", "rs_bootstrap_activate" },
                        { "startupEntrySource", input.StartupEntrySource },
                    });

                if (activationResult != 0)
                {
                    nativeLibraryHandle!.Dispose();
                    return Failure(
                        diagnostics,
                        "rust_core_activation_failed",
                        "Rust bootstrap activation returned non-zero status " + activationResult + ".");
                }

                diagnostics.Emit(
                    surface: "startup_entry_status",
                    status: "ready",
                    message: "Managed bootstrap boundary is ready, native ABI is validated, and Rust bootstrap activation completed.",
                    data: new Dictionary<string, string>
                    {
                        { "startupEntrySource", input.StartupEntrySource },
                        { "nativePayloadPath", resolvedNativePayloadPath },
                    });

                return StartupEntryResult.Success(new StartupEntrySession(nativeLibraryHandle!, bindings!, abi));
            }
            catch (Exception ex)
            {
                return Failure(
                    diagnostics,
                    "bootstrap_exception",
                    "Managed startup entry failed: " + ex.Message);
            }
        }

        private static StartupEntryResult Fallback(BootstrapDiagnostics diagnostics, string reasonCode, string reason)
        {
            diagnostics.Emit(
                surface: "startup_entry_status",
                status: "fallback",
                message: reason,
                data: new Dictionary<string, string>
                {
                    { "reasonCode", reasonCode },
                    { "fallbackReason", reason },
                });

            diagnostics.Emit(
                surface: "rust_core_activation_status",
                status: "not_attempted",
                message: "Rust bootstrap activation was not attempted because startup fell back before activation.",
                data: new Dictionary<string, string>
                {
                    { "reasonCode", reasonCode },
                    { "fallbackReason", reason },
                });

            return StartupEntryResult.Fallback(reasonCode, reason);
        }

        private static StartupEntryResult Failure(BootstrapDiagnostics diagnostics, string reasonCode, string reason)
        {
            diagnostics.Emit(
                surface: "startup_entry_status",
                status: "failure",
                message: reason,
                data: new Dictionary<string, string>
                {
                    { "reasonCode", reasonCode },
                    { "failureReason", reason },
                });

            diagnostics.Emit(
                surface: "rust_core_activation_status",
                status: "failure",
                message: "Rust bootstrap activation could not complete.",
                data: new Dictionary<string, string>
                {
                    { "reasonCode", reasonCode },
                    { "failureReason", reason },
                });

            return StartupEntryResult.Failure(reasonCode, reason);
        }
    }

    public sealed class StartupEntryInput
    {
        public StartupEntryInput(string resolvedModRootPath, string? observedPackageId, string? observedPackageIdSource)
            : this("bootstrap_default", resolvedModRootPath, observedPackageId, observedPackageIdSource, "1.6", "bootstrap_default", null, "bootstrap_default")
        {
        }

        public StartupEntryInput(
            string startupEntrySource,
            string resolvedModRootPath,
            string? observedPackageId,
            string? observedPackageIdSource,
            string? runtimeVersionBasis,
            string? runtimeVersionSource,
            string? managedSelfAssemblyPath)
            : this(startupEntrySource, resolvedModRootPath, observedPackageId, observedPackageIdSource, runtimeVersionBasis, runtimeVersionSource, managedSelfAssemblyPath, "bootstrap_default")
        {
        }

        public StartupEntryInput(
            string startupEntrySource,
            string resolvedModRootPath,
            string? observedPackageId,
            string? observedPackageIdSource,
            string? runtimeVersionBasis,
            string? runtimeVersionSource,
            string? managedSelfAssemblyPath,
            string? managedSelfAssemblySource)
        {
            StartupEntrySource = string.IsNullOrWhiteSpace(startupEntrySource) ? "bootstrap_default" : startupEntrySource.Trim();
            ResolvedModRootPath = resolvedModRootPath ?? throw new ArgumentNullException(nameof(resolvedModRootPath));
            ObservedPackageId = observedPackageId;
            ObservedPackageIdSource = observedPackageIdSource;
            RuntimeVersionBasis = runtimeVersionBasis;
            RuntimeVersionSource = runtimeVersionSource;
            ManagedSelfAssemblyPath = managedSelfAssemblyPath;
            ManagedSelfAssemblySource = string.IsNullOrWhiteSpace(managedSelfAssemblySource) ? "bootstrap_default" : managedSelfAssemblySource.Trim();
        }

        public string StartupEntrySource { get; }

        public string ResolvedModRootPath { get; }

        public string ModRootPath => ResolvedModRootPath;

        public string? ObservedPackageId { get; }

        public string? ObservedPackageIdSource { get; }

        public string? RuntimeVersionBasis { get; }

        public string? RuntimeVersionSource { get; }

        public string? ManagedSelfAssemblyPath { get; }

        public string ManagedSelfAssemblySource { get; }
    }

    public enum StartupEntryStatus
    {
        Ready,
        Fallback,
        Failure,
    }

    public sealed class StartupEntryResult
    {
        private StartupEntryResult(
            StartupEntryStatus status,
            string reasonCode,
            string reason,
            StartupEntrySession? session)
        {
            Status = status;
            ReasonCode = reasonCode;
            Reason = reason;
            Session = session;
        }

        public StartupEntryStatus Status { get; }

        public string ReasonCode { get; }

        public string Reason { get; }

        public StartupEntrySession? Session { get; }

        public static StartupEntryResult Success(StartupEntrySession session)
        {
            return new StartupEntryResult(
                StartupEntryStatus.Ready,
                "none",
                "Bootstrap ready.",
                session);
        }

        public static StartupEntryResult Fallback(string reasonCode, string reason)
        {
            return new StartupEntryResult(
                StartupEntryStatus.Fallback,
                reasonCode,
                reason,
                null);
        }

        public static StartupEntryResult Failure(string reasonCode, string reason)
        {
            return new StartupEntryResult(
                StartupEntryStatus.Failure,
                reasonCode,
                reason,
                null);
        }
    }

    public sealed class StartupEntrySession : IDisposable
    {
        private readonly NativeLoader.NativeLibraryHandle _nativeLibraryHandle;
        private bool _disposed;

        public StartupEntrySession(
            NativeLoader.NativeLibraryHandle nativeLibraryHandle,
            NativeBootstrapBindings bindings,
            AbiHandshakeStatus abiHandshakeStatus)
        {
            _nativeLibraryHandle = nativeLibraryHandle;
            Bindings = bindings;
            AbiHandshakeStatus = abiHandshakeStatus;
        }

        public NativeBootstrapBindings Bindings { get; }

        public AbiHandshakeStatus AbiHandshakeStatus { get; }

        public void Dispose()
        {
            if (_disposed)
            {
                return;
            }

            _nativeLibraryHandle.Dispose();
            _disposed = true;
        }
    }
}
