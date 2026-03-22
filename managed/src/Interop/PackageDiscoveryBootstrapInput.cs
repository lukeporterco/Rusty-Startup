using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices;
using System.Text;

namespace RustyStartup.Managed.Interop
{
    [StructLayout(LayoutKind.Sequential)]
    public struct PackageDiscoveryBootstrapInput
    {
        public IntPtr SelfPackageRootValue;
        public IntPtr SelfPackageRootSource;
        public IntPtr OfficialDataRootValue;
        public IntPtr OfficialDataRootSource;
        public IntPtr LocalModsRootValue;
        public IntPtr LocalModsRootSource;
        public IntPtr WorkshopContentRootValue;
        public IntPtr WorkshopContentRootSource;
        public IntPtr HistoricalStalePackageIdValue;
        public IntPtr HistoricalStalePackageIdSource;
    }

    internal sealed class PackageDiscoveryBootstrapFacts
    {
        public string? SelfPackageRootValue { get; set; }
        public string? SelfPackageRootSource { get; set; }
        public string? OfficialDataRootValue { get; set; }
        public string? OfficialDataRootSource { get; set; }
        public string? LocalModsRootValue { get; set; }
        public string? LocalModsRootSource { get; set; }
        public string? WorkshopContentRootValue { get; set; }
        public string? WorkshopContentRootSource { get; set; }
        public string? HistoricalStalePackageIdValue { get; set; }
        public string? HistoricalStalePackageIdSource { get; set; }
    }

    internal sealed class PackageDiscoveryBootstrapInputScope : IDisposable
    {
        private readonly List<IntPtr> _allocatedStrings = new List<IntPtr>();

        public PackageDiscoveryBootstrapInput Input;

        private PackageDiscoveryBootstrapInputScope()
        {
        }

        public static PackageDiscoveryBootstrapInputScope Create(PackageDiscoveryBootstrapFacts facts)
        {
            if (facts == null)
            {
                throw new ArgumentNullException(nameof(facts));
            }

            var scope = new PackageDiscoveryBootstrapInputScope();
            scope.Input = new PackageDiscoveryBootstrapInput
            {
                SelfPackageRootValue = scope.EncodeRequiredAbsolutePath(facts.SelfPackageRootValue, nameof(facts.SelfPackageRootValue)),
                SelfPackageRootSource = scope.EncodeRequiredSource(facts.SelfPackageRootSource, nameof(facts.SelfPackageRootSource)),
                OfficialDataRootValue = scope.EncodeRequiredAbsolutePath(facts.OfficialDataRootValue, nameof(facts.OfficialDataRootValue)),
                OfficialDataRootSource = scope.EncodeRequiredSource(facts.OfficialDataRootSource, nameof(facts.OfficialDataRootSource)),
                LocalModsRootValue = scope.EncodeRequiredAbsolutePath(facts.LocalModsRootValue, nameof(facts.LocalModsRootValue)),
                LocalModsRootSource = scope.EncodeRequiredSource(facts.LocalModsRootSource, nameof(facts.LocalModsRootSource)),
                WorkshopContentRootValue = scope.EncodeOptionalAbsolutePath(facts.WorkshopContentRootValue, nameof(facts.WorkshopContentRootValue)),
                WorkshopContentRootSource = scope.EncodeOptionalSource(facts.WorkshopContentRootSource, nameof(facts.WorkshopContentRootSource)),
                HistoricalStalePackageIdValue = scope.EncodeOptionalValue(facts.HistoricalStalePackageIdValue),
                HistoricalStalePackageIdSource = scope.EncodeOptionalSource(facts.HistoricalStalePackageIdSource, nameof(facts.HistoricalStalePackageIdSource)),
            };

            return scope;
        }

        public void Dispose()
        {
            foreach (var pointer in _allocatedStrings)
            {
                Marshal.FreeHGlobal(pointer);
            }

            _allocatedStrings.Clear();
        }

        private IntPtr EncodeRequiredAbsolutePath(string? value, string fieldName)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                throw new ArgumentException(fieldName + " must be provided.");
            }

            var trimmed = value.Trim();
            if (!PathIsAbsolute(trimmed))
            {
                throw new ArgumentException(fieldName + " must be an absolute path.");
            }

            return Encode(trimmed);
        }

        private IntPtr EncodeOptionalAbsolutePath(string? value, string fieldName)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                return IntPtr.Zero;
            }

            var trimmed = value.Trim();
            if (!PathIsAbsolute(trimmed))
            {
                throw new ArgumentException(fieldName + " must be an absolute path when provided.");
            }

            return Encode(trimmed);
        }

        private IntPtr EncodeRequiredSource(string? value, string fieldName)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                throw new ArgumentException(fieldName + " must be provided.");
            }

            return Encode(value.Trim());
        }

        private IntPtr EncodeOptionalSource(string? value, string fieldName)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                return IntPtr.Zero;
            }

            return Encode(value.Trim());
        }

        private IntPtr EncodeOptionalValue(string? value)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                return IntPtr.Zero;
            }

            return Encode(value.Trim());
        }

        private IntPtr Encode(string value)
        {
            var bytes = Encoding.UTF8.GetBytes(value);
            var buffer = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, buffer, bytes.Length);
            Marshal.WriteByte(buffer, bytes.Length, 0);
            _allocatedStrings.Add(buffer);
            return buffer;
        }

        private static bool PathIsAbsolute(string value)
        {
            if (Path.IsPathRooted(value))
            {
                return true;
            }

            return value.StartsWith("/", StringComparison.Ordinal) || value.StartsWith("\\\\", StringComparison.Ordinal);
        }
    }
}
