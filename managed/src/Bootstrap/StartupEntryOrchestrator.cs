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

            NativePathResolution resolution;
            try
            {
                resolution = NativePathResolver.Resolve(input.ModRootPath);
            }
            catch (Exception ex)
            {
                return Fallback(
                    diagnostics,
                    "native_path_resolution_failed",
                    "Native path resolution failed: " + ex.Message);
            }

            diagnostics.Emit(
                surface: "native_load_path",
                status: resolution.Exists ? "resolved" : "missing",
                message: "Deterministic package-relative native path was resolved.",
                data: new Dictionary<string, string>
                {
                    { "modRoot", resolution.AbsoluteModRoot },
                    { "relativePath", resolution.RelativeNativePath },
                    { "absolutePath", resolution.AbsoluteNativePath },
                    { "rid", resolution.RuntimeIdentifier },
                    { "platform", resolution.Platform.ToString() },
                    { "exists", resolution.Exists ? "true" : "false" },
                });

            if (!resolution.Exists)
            {
                return Fallback(
                    diagnostics,
                    "native_binary_missing",
                    "Native binary is missing at resolved path.");
            }

            if (!NativeLoader.TryLoad(
                    resolution.AbsoluteNativePath,
                    resolution.Platform,
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
        {
            ModRootPath = modRootPath ?? throw new ArgumentNullException(nameof(modRootPath));
            ObservedPackageId = observedPackageId;
            ObservedPackageIdSource = observedPackageIdSource;
        }

        public string ModRootPath { get; }

        public string? ObservedPackageId { get; }

        public string? ObservedPackageIdSource { get; }
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
