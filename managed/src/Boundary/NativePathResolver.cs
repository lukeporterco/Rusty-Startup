using System;
using System.Runtime.InteropServices;

namespace RustyStartup.Managed.Boundary
{
    public enum NativePlatform
    {
        Windows,
        Linux,
        MacOS,
    }

    public static class NativePathResolver
    {
        public static NativePlatform DetectPlatform()
        {
            if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            {
                return NativePlatform.Windows;
            }

            if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
            {
                return NativePlatform.Linux;
            }

            if (RuntimeInformation.IsOSPlatform(OSPlatform.OSX))
            {
                return NativePlatform.MacOS;
            }

            throw new PlatformNotSupportedException("Unsupported operating system for RustyStartup native loading.");
        }

        public static string DetectArchitectureSegment()
        {
            switch (RuntimeInformation.OSArchitecture)
            {
                case Architecture.X64:
                    return "x64";
                case Architecture.Arm64:
                    return "arm64";
                default:
                    throw new PlatformNotSupportedException(
                        "Unsupported architecture for RustyStartup native loading: " + RuntimeInformation.OSArchitecture);
            }
        }

        public static string GetRidPrefix(NativePlatform platform)
        {
            switch (platform)
            {
                case NativePlatform.Windows:
                    return "win";
                case NativePlatform.Linux:
                    return "linux";
                case NativePlatform.MacOS:
                    return "macos";
                default:
                    throw new ArgumentOutOfRangeException(nameof(platform), platform, null);
            }
        }

        public static string GetNativeFileName(NativePlatform platform)
        {
            switch (platform)
            {
                case NativePlatform.Windows:
                    return "rustystartup_core.dll";
                case NativePlatform.Linux:
                    return "librustystartup_core.so";
                case NativePlatform.MacOS:
                    return "librustystartup_core.dylib";
                default:
                    throw new ArgumentOutOfRangeException(nameof(platform), platform, null);
            }
        }
    }
}
