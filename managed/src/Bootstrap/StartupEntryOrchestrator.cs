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
            diagnostics.Emit(
                surface: "startup_entry_status",
                status: "begin",
                message: "Managed startup entry orchestration started.");

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
                        input.ModRootPath,
                        identity.ObservedPackageId,
                        identity.ObservedSource,
                        input.RuntimeVersionBasis,
                        input.RuntimeVersionSource,
                        effectiveSelfAssemblyPath));
            }
            catch (Exception ex)
            {
                return Fallback(
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
                return Fallback(
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
                return Fallback(
                    diagnostics,
                    "native_load_failed",
                    loadError ?? "Native load failed.");
            }

            if (!NativeBootstrapBindings.TryBind(nativeLibraryHandle!, out var bindings, out var bindError))
            {
                nativeLibraryHandle!.Dispose();
                return Fallback(
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
                });

            if (!abi.IsCompatible)
            {
                nativeLibraryHandle!.Dispose();
                return Fallback(
                    diagnostics,
                    "abi_incompatible",
                    abi.Reason);
            }

            diagnostics.Emit(
                surface: "startup_entry_status",
                status: "ready",
                message: "Managed bootstrap boundary is ready and native ABI is validated.");

            return StartupEntryResult.Success(new StartupEntrySession(nativeLibraryHandle!, bindings!, abi));
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
                });

            return StartupEntryResult.Fallback(reasonCode, reason);
        }
    }

    public sealed class StartupEntryInput
    {
        public StartupEntryInput(string modRootPath, string? observedPackageId, string? observedPackageIdSource)
            : this(modRootPath, observedPackageId, observedPackageIdSource, "1.6", "bootstrap_default", null)
        {
        }

        public StartupEntryInput(
            string modRootPath,
            string? observedPackageId,
            string? observedPackageIdSource,
            string? runtimeVersionBasis,
            string? runtimeVersionSource,
            string? managedSelfAssemblyPath)
        {
            ModRootPath = modRootPath ?? throw new ArgumentNullException(nameof(modRootPath));
            ObservedPackageId = observedPackageId;
            ObservedPackageIdSource = observedPackageIdSource;
            RuntimeVersionBasis = runtimeVersionBasis;
            RuntimeVersionSource = runtimeVersionSource;
            ManagedSelfAssemblyPath = managedSelfAssemblyPath;
        }

        public string ModRootPath { get; }

        public string? ObservedPackageId { get; }

        public string? ObservedPackageIdSource { get; }

        public string? RuntimeVersionBasis { get; }

        public string? RuntimeVersionSource { get; }

        public string? ManagedSelfAssemblyPath { get; }
    }

    public enum StartupEntryStatus
    {
        Ready,
        Fallback,
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
