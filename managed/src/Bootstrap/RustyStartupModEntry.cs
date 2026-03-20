using System;
using System.IO;
using RustyStartup.Managed.Diagnostics;
using Verse;

namespace RustyStartup.Managed.Bootstrap
{
    [StaticConstructorOnStartup]
    public static class RustyStartupModEntry
    {
        private const string StartupEntrySource = "Verse.StaticConstructorOnStartup";
        private const string RuntimeVersionBasis = "1.6";
        private const string RuntimeVersionSource = "About/About.xml:supportedVersions";
        private const string ObservedPackageIdentity = "rustystartup.core";
        private const string ObservedPackageIdentitySource = "About/About.xml:packageId";
        private const string ManagedSelfAssemblySource = "RustyStartup.Managed.Bootstrap.RustyStartupModEntry.Assembly.Location";

        static RustyStartupModEntry()
        {
            var managedAssemblyPath = typeof(RustyStartupModEntry).Assembly.Location;
            var resolvedModRootPath = ResolveModRoot(managedAssemblyPath);
            var input = new StartupEntryInput(
                StartupEntrySource,
                resolvedModRootPath,
                ObservedPackageIdentity,
                ObservedPackageIdentitySource,
                RuntimeVersionBasis,
                RuntimeVersionSource,
                managedAssemblyPath,
                ManagedSelfAssemblySource);

            RustyStartupMod.Initialize(input, new BootstrapDiagnostics());
        }

        private static string ResolveModRoot(string managedAssemblyPath)
        {
            var assemblyDirectory = Path.GetDirectoryName(managedAssemblyPath);
            if (string.IsNullOrWhiteSpace(assemblyDirectory))
            {
                return AppContext.BaseDirectory;
            }

            var packageAssembliesDirectory = Directory.GetParent(assemblyDirectory);
            if (packageAssembliesDirectory?.Parent != null)
            {
                return packageAssembliesDirectory.Parent.FullName;
            }

            return AppContext.BaseDirectory;
        }
    }
}
