using System;
using System.Runtime.InteropServices;
using RustyStartup.Managed.Boundary;

namespace RustyStartup.Managed.Interop
{
    public sealed class NativeBootstrapBindings
    {
        private NativeBootstrapBindings(
            GetAbiVersionDelegate getAbiVersion,
            GetCapabilitiesDelegate getCapabilities)
        {
            GetAbiVersion = getAbiVersion;
            GetCapabilities = getCapabilities;
        }

        public GetAbiVersionDelegate GetAbiVersion { get; }

        public GetCapabilitiesDelegate GetCapabilities { get; }

        public static bool TryBind(
            NativeLoader.NativeLibraryHandle nativeLibraryHandle,
            out NativeBootstrapBindings? bindings,
            out string? error)
        {
            bindings = null;
            error = null;

            if (!NativeLoader.TryGetSymbol(nativeLibraryHandle, "rs_bootstrap_get_abi_version", out var abiVersionSymbol, out error))
            {
                return false;
            }

            if (!NativeLoader.TryGetSymbol(nativeLibraryHandle, "rs_bootstrap_get_capabilities", out var capabilitiesSymbol, out error))
            {
                return false;
            }

            var abiVersion = Marshal.GetDelegateForFunctionPointer<GetAbiVersionDelegate>(abiVersionSymbol);
            var capabilities = Marshal.GetDelegateForFunctionPointer<GetCapabilitiesDelegate>(capabilitiesSymbol);

            bindings = new NativeBootstrapBindings(abiVersion, capabilities);
            return true;
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate int GetAbiVersionDelegate();

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate ulong GetCapabilitiesDelegate();
    }
}
