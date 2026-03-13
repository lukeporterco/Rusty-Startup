using System;
using System.IO;
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
        public static NativePathResolution Resolve(string modRootPath)
        {
            if (string.IsNullOrWhiteSpace(modRootPath))
            {
                throw new ArgumentException("Mod root path is required.", nameof(modRootPath));
            }

            var absoluteRoot = Path.GetFullPath(modRootPath.Trim());
            var platform = DetectPlatform();
            var architecture = DetectArchitectureSegment();
            var ridPrefix = GetRidPrefix(platform);
            var rid = ridPrefix + "-" + architecture;
            var nativeFileName = GetNativeFileName(platform);
            var relativePath = Path.Combine("1.6", "Native", rid, nativeFileName);
            var absolutePath = Path.GetFullPath(Path.Combine(absoluteRoot, relativePath));
            var exists = File.Exists(absolutePath);

            return new NativePathResolution(
                absoluteRoot,
                relativePath,
                absolutePath,
                platform,
                rid,
                nativeFileName,
                exists);
        }

        private static NativePlatform DetectPlatform()
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

        private static string DetectArchitectureSegment()
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

        private static string GetRidPrefix(NativePlatform platform)
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

        private static string GetNativeFileName(NativePlatform platform)
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

    public sealed class NativePathResolution
    {
        public NativePathResolution(
            string absoluteModRoot,
            string relativeNativePath,
            string absoluteNativePath,
            NativePlatform platform,
            string runtimeIdentifier,
            string nativeFileName,
            bool exists)
        {
            AbsoluteModRoot = absoluteModRoot;
            RelativeNativePath = relativeNativePath;
            AbsoluteNativePath = absoluteNativePath;
            Platform = platform;
            RuntimeIdentifier = runtimeIdentifier;
            NativeFileName = nativeFileName;
            Exists = exists;
        }

        public string AbsoluteModRoot { get; }

        public string RelativeNativePath { get; }

        public string AbsoluteNativePath { get; }

        public NativePlatform Platform { get; }

        public string RuntimeIdentifier { get; }

        public string NativeFileName { get; }

        public bool Exists { get; }
    }
}
