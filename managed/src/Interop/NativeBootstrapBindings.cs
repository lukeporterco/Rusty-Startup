using System;
using System.Runtime.InteropServices;
using RustyStartup.Managed.Boundary;

namespace RustyStartup.Managed.Interop
{
    public sealed class NativeBootstrapBindings
    {
        private NativeBootstrapBindings(
            GetAbiVersionDelegate getAbiVersion,
            GetCapabilitiesDelegate getCapabilities,
            SubmitRuntimeContextDelegate submitRuntimeContext,
            FreeCStringDelegate freeCString,
            ActivateBootstrapDelegate activateBootstrap)
        {
            GetAbiVersion = getAbiVersion;
            GetCapabilities = getCapabilities;
            SubmitRuntimeContextBootstrap = submitRuntimeContext;
            FreeCString = freeCString;
            ActivateBootstrap = activateBootstrap;
        }

        public GetAbiVersionDelegate GetAbiVersion { get; }

        public GetCapabilitiesDelegate GetCapabilities { get; }

        public SubmitRuntimeContextDelegate SubmitRuntimeContextBootstrap { get; }

        public FreeCStringDelegate FreeCString { get; }

        public ActivateBootstrapDelegate ActivateBootstrap { get; }

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

            if (!NativeLoader.TryGetSymbol(nativeLibraryHandle, "rs_bootstrap_submit_runtime_context", out var runtimeContextSymbol, out error))
            {
                return false;
            }

            if (!NativeLoader.TryGetSymbol(nativeLibraryHandle, "rs_bootstrap_free_c_string", out var freeCStringSymbol, out error))
            {
                return false;
            }

            if (!NativeLoader.TryGetSymbol(nativeLibraryHandle, "rs_bootstrap_activate", out var activationSymbol, out error))
            {
                return false;
            }

            var abiVersion = Marshal.GetDelegateForFunctionPointer<GetAbiVersionDelegate>(abiVersionSymbol);
            var capabilities = Marshal.GetDelegateForFunctionPointer<GetCapabilitiesDelegate>(capabilitiesSymbol);
            var submitRuntimeContext = Marshal.GetDelegateForFunctionPointer<SubmitRuntimeContextDelegate>(runtimeContextSymbol);
            var freeCString = Marshal.GetDelegateForFunctionPointer<FreeCStringDelegate>(freeCStringSymbol);
            var activation = Marshal.GetDelegateForFunctionPointer<ActivateBootstrapDelegate>(activationSymbol);

            bindings = new NativeBootstrapBindings(abiVersion, capabilities, submitRuntimeContext, freeCString, activation);
            return true;
        }

        public string SubmitRuntimeContext(ref RuntimeContextBootstrapInput input)
        {
            var reportPointer = SubmitRuntimeContextBootstrap(ref input);
            if (reportPointer == IntPtr.Zero)
            {
                return string.Empty;
            }

            try
            {
                return ReadUtf8(reportPointer);
            }
            finally
            {
                FreeCString(reportPointer);
            }
        }

        public int Activate()
        {
            return ActivateBootstrap();
        }

        private static string ReadUtf8(IntPtr value)
        {
            if (value == IntPtr.Zero)
            {
                return string.Empty;
            }

            var length = 0;
            while (Marshal.ReadByte(value, length) != 0)
            {
                length++;
            }

            var bytes = new byte[length];
            for (var i = 0; i < length; i++)
            {
                bytes[i] = Marshal.ReadByte(value, i);
            }

            return System.Text.Encoding.UTF8.GetString(bytes);
        }

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate int GetAbiVersionDelegate();

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate ulong GetCapabilitiesDelegate();

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate IntPtr SubmitRuntimeContextDelegate(ref RuntimeContextBootstrapInput input);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate void FreeCStringDelegate(IntPtr value);

        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        public delegate int ActivateBootstrapDelegate();
    }
}
