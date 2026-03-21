using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;

namespace RustyStartup.Managed.Interop
{
    [StructLayout(LayoutKind.Sequential)]
    public struct RuntimeContextBootstrapInput
    {
        public IntPtr RuntimeBuildIdentityFreshValue;
        public IntPtr RuntimeBuildIdentityFreshSource;
        public IntPtr RuntimeBuildIdentityStaticValue;
        public IntPtr RuntimeBuildIdentityStaticSource;
        public IntPtr RuntimeVersionBasisFreshValue;
        public IntPtr RuntimeVersionBasisFreshSource;
        public IntPtr RuntimeVersionBasisStaticValue;
        public IntPtr RuntimeVersionBasisStaticSource;
        public IntPtr ParserModeValue;
        public IntPtr ParserModeSource;
        public IntPtr SelectedLanguageKeyValue;
        public IntPtr SelectedLanguageKeySource;
        public IntPtr PlatformValue;
        public IntPtr PlatformSource;
        public IntPtr OperatingSystemValue;
        public IntPtr OperatingSystemSource;
        public IntPtr ArchitectureValue;
        public IntPtr ArchitectureSource;
        public IntPtr SelectedSelfPackageRootValue;
        public IntPtr SelectedSelfPackageRootSource;
        public IntPtr ActiveSelfContentRootValue;
        public IntPtr ActiveSelfContentRootSource;
        public IntPtr ManagedSelfAssemblyPathValue;
        public IntPtr ManagedSelfAssemblyPathSource;
        public IntPtr NativePayloadPathValue;
        public IntPtr NativePayloadPathSource;
    }

    internal sealed class RuntimeContextBootstrapFacts
    {
        public string? RuntimeBuildIdentityFreshValue { get; set; }
        public string? RuntimeBuildIdentityFreshSource { get; set; }
        public string? RuntimeBuildIdentityStaticValue { get; set; }
        public string? RuntimeBuildIdentityStaticSource { get; set; }
        public string? RuntimeVersionBasisFreshValue { get; set; }
        public string? RuntimeVersionBasisFreshSource { get; set; }
        public string? RuntimeVersionBasisStaticValue { get; set; }
        public string? RuntimeVersionBasisStaticSource { get; set; }
        public string? ParserModeValue { get; set; }
        public string? ParserModeSource { get; set; }
        public string? SelectedLanguageKeyValue { get; set; }
        public string? SelectedLanguageKeySource { get; set; }
        public string? PlatformValue { get; set; }
        public string? PlatformSource { get; set; }
        public string? OperatingSystemValue { get; set; }
        public string? OperatingSystemSource { get; set; }
        public string? ArchitectureValue { get; set; }
        public string? ArchitectureSource { get; set; }
        public string? SelectedSelfPackageRootValue { get; set; }
        public string? SelectedSelfPackageRootSource { get; set; }
        public string? ActiveSelfContentRootValue { get; set; }
        public string? ActiveSelfContentRootSource { get; set; }
        public string? ManagedSelfAssemblyPathValue { get; set; }
        public string? ManagedSelfAssemblyPathSource { get; set; }
        public string? NativePayloadPathValue { get; set; }
        public string? NativePayloadPathSource { get; set; }
    }

    internal sealed class RuntimeContextBootstrapInputScope : IDisposable
    {
        private readonly List<IntPtr> _allocatedStrings = new List<IntPtr>();

        public RuntimeContextBootstrapInput Input;

        private RuntimeContextBootstrapInputScope()
        {
        }

        public static RuntimeContextBootstrapInputScope Create(RuntimeContextBootstrapFacts facts)
        {
            if (facts == null)
            {
                throw new ArgumentNullException(nameof(facts));
            }

            var scope = new RuntimeContextBootstrapInputScope();
            scope.Input = new RuntimeContextBootstrapInput
            {
                RuntimeBuildIdentityFreshValue = scope.Encode(facts.RuntimeBuildIdentityFreshValue),
                RuntimeBuildIdentityFreshSource = scope.Encode(facts.RuntimeBuildIdentityFreshSource),
                RuntimeBuildIdentityStaticValue = scope.Encode(facts.RuntimeBuildIdentityStaticValue),
                RuntimeBuildIdentityStaticSource = scope.Encode(facts.RuntimeBuildIdentityStaticSource),
                RuntimeVersionBasisFreshValue = scope.Encode(facts.RuntimeVersionBasisFreshValue),
                RuntimeVersionBasisFreshSource = scope.Encode(facts.RuntimeVersionBasisFreshSource),
                RuntimeVersionBasisStaticValue = scope.Encode(facts.RuntimeVersionBasisStaticValue),
                RuntimeVersionBasisStaticSource = scope.Encode(facts.RuntimeVersionBasisStaticSource),
                ParserModeValue = scope.Encode(facts.ParserModeValue),
                ParserModeSource = scope.Encode(facts.ParserModeSource),
                SelectedLanguageKeyValue = scope.Encode(facts.SelectedLanguageKeyValue),
                SelectedLanguageKeySource = scope.Encode(facts.SelectedLanguageKeySource),
                PlatformValue = scope.Encode(facts.PlatformValue),
                PlatformSource = scope.Encode(facts.PlatformSource),
                OperatingSystemValue = scope.Encode(facts.OperatingSystemValue),
                OperatingSystemSource = scope.Encode(facts.OperatingSystemSource),
                ArchitectureValue = scope.Encode(facts.ArchitectureValue),
                ArchitectureSource = scope.Encode(facts.ArchitectureSource),
                SelectedSelfPackageRootValue = scope.Encode(facts.SelectedSelfPackageRootValue),
                SelectedSelfPackageRootSource = scope.Encode(facts.SelectedSelfPackageRootSource),
                ActiveSelfContentRootValue = scope.Encode(facts.ActiveSelfContentRootValue),
                ActiveSelfContentRootSource = scope.Encode(facts.ActiveSelfContentRootSource),
                ManagedSelfAssemblyPathValue = scope.Encode(facts.ManagedSelfAssemblyPathValue),
                ManagedSelfAssemblyPathSource = scope.Encode(facts.ManagedSelfAssemblyPathSource),
                NativePayloadPathValue = scope.Encode(facts.NativePayloadPathValue),
                NativePayloadPathSource = scope.Encode(facts.NativePayloadPathSource),
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

        private IntPtr Encode(string? value)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                return IntPtr.Zero;
            }

            var bytes = Encoding.UTF8.GetBytes(value.Trim());
            var buffer = Marshal.AllocHGlobal(bytes.Length + 1);
            Marshal.Copy(bytes, 0, buffer, bytes.Length);
            Marshal.WriteByte(buffer, bytes.Length, 0);
            _allocatedStrings.Add(buffer);
            return buffer;
        }
    }
}
