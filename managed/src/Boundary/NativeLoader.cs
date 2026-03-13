using System;
using System.Runtime.InteropServices;
using System.Text;

namespace RustyStartup.Managed.Boundary
{
    public static class NativeLoader
    {
        private const int RtldNow = 2;

        public static bool TryLoad(string absoluteNativePath, NativePlatform platform, out NativeLibraryHandle? handle, out string? error)
        {
            handle = null;
            error = null;

            IntPtr rawHandle;
            switch (platform)
            {
                case NativePlatform.Windows:
                    rawHandle = LoadLibraryW(absoluteNativePath);
                    if (rawHandle == IntPtr.Zero)
                    {
                        error = "LoadLibraryW failed with Win32 error " + Marshal.GetLastWin32Error() + ".";
                        return false;
                    }

                    break;
                case NativePlatform.Linux:
                case NativePlatform.MacOS:
                    rawHandle = dlopen(absoluteNativePath, RtldNow);
                    if (rawHandle == IntPtr.Zero)
                    {
                        error = "dlopen failed: " + ReadDlError();
                        return false;
                    }

                    break;
                default:
                    error = "Unsupported platform for native loading: " + platform + ".";
                    return false;
            }

            handle = new NativeLibraryHandle(rawHandle, platform);
            return true;
        }

        public static bool TryGetSymbol(NativeLibraryHandle handle, string symbolName, out IntPtr symbolAddress, out string? error)
        {
            symbolAddress = IntPtr.Zero;
            error = null;

            IntPtr rawAddress;
            switch (handle.Platform)
            {
                case NativePlatform.Windows:
                    rawAddress = GetProcAddress(handle.Handle, symbolName);
                    if (rawAddress == IntPtr.Zero)
                    {
                        error = "GetProcAddress failed for '" + symbolName + "' with Win32 error " + Marshal.GetLastWin32Error() + ".";
                        return false;
                    }

                    break;
                case NativePlatform.Linux:
                case NativePlatform.MacOS:
                    ClearDlError();
                    rawAddress = dlsym(handle.Handle, symbolName);
                    var dlError = ReadDlError();
                    if (!string.IsNullOrEmpty(dlError))
                    {
                        error = "dlsym failed for '" + symbolName + "': " + dlError;
                        return false;
                    }

                    if (rawAddress == IntPtr.Zero)
                    {
                        error = "dlsym returned null for '" + symbolName + "'.";
                        return false;
                    }

                    break;
                default:
                    error = "Unsupported platform for symbol lookup: " + handle.Platform + ".";
                    return false;
            }

            symbolAddress = rawAddress;
            return true;
        }

        private static void ClearDlError()
        {
            dlerror();
        }

        private static string ReadDlError()
        {
            var ptr = dlerror();
            return ptr == IntPtr.Zero ? string.Empty : PtrToUtf8(ptr);
        }

        private static string PtrToUtf8(IntPtr value)
        {
            if (value == IntPtr.Zero)
            {
                return string.Empty;
            }

            var bytes = new byte[256];
            var index = 0;
            while (index < bytes.Length - 1)
            {
                var b = Marshal.ReadByte(value, index);
                if (b == 0)
                {
                    break;
                }

                bytes[index] = b;
                index++;
            }

            return Encoding.UTF8.GetString(bytes, 0, index);
        }

        [DllImport("kernel32", SetLastError = true, CharSet = CharSet.Unicode)]
        private static extern IntPtr LoadLibraryW(string lpFileName);

        [DllImport("kernel32", SetLastError = true, CharSet = CharSet.Ansi)]
        private static extern IntPtr GetProcAddress(IntPtr hModule, string procName);

        [DllImport("kernel32", SetLastError = true)]
        private static extern bool FreeLibrary(IntPtr hModule);

        [DllImport("libdl", SetLastError = true)]
        private static extern IntPtr dlopen(string fileName, int flags);

        [DllImport("libdl", SetLastError = true)]
        private static extern IntPtr dlsym(IntPtr handle, string symbol);

        [DllImport("libdl", SetLastError = true)]
        private static extern int dlclose(IntPtr handle);

        [DllImport("libdl", SetLastError = true)]
        private static extern IntPtr dlerror();

        public sealed class NativeLibraryHandle : IDisposable
        {
            private bool _disposed;

            public NativeLibraryHandle(IntPtr handle, NativePlatform platform)
            {
                Handle = handle;
                Platform = platform;
            }

            public IntPtr Handle { get; private set; }

            public NativePlatform Platform { get; }

            public void Dispose()
            {
                if (_disposed || Handle == IntPtr.Zero)
                {
                    return;
                }

                switch (Platform)
                {
                    case NativePlatform.Windows:
                        FreeLibrary(Handle);
                        break;
                    case NativePlatform.Linux:
                    case NativePlatform.MacOS:
                        dlclose(Handle);
                        break;
                }

                Handle = IntPtr.Zero;
                _disposed = true;
            }
        }
    }
}
