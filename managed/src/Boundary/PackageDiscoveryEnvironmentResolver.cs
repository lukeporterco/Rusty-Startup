using System;
using System.IO;
using RustyStartup.Managed.Bootstrap;
using RustyStartup.Managed.Interop;

namespace RustyStartup.Managed.Boundary
{
    public static class PackageDiscoveryEnvironmentResolver
    {
        private const string LockedPackageIdentity = "rustystartup.core";

        internal static PackageDiscoveryBootstrapFacts Resolve(StartupEntryInput input)
        {
            if (input == null)
            {
                throw new ArgumentNullException(nameof(input));
            }

            var workshopContentRoot = TryResolveWorkshopContentRoot();
            var facts = new PackageDiscoveryBootstrapFacts
            {
                SelfPackageRootValue = NormalizePath(input.ResolvedModRootPath),
                SelfPackageRootSource = "StartupEntryInput.ResolvedModRootPath",
                OfficialDataRootValue = NormalizePath(Path.Combine(AppContext.BaseDirectory, "Data")),
                OfficialDataRootSource = "AppContext.BaseDirectory/Data",
                LocalModsRootValue = NormalizePath(Path.Combine(AppContext.BaseDirectory, "Mods")),
                LocalModsRootSource = "AppContext.BaseDirectory/Mods",
                WorkshopContentRootValue = workshopContentRoot,
                WorkshopContentRootSource = workshopContentRoot != null
                    ? "Environment.SpecialFolder.ProgramFilesX86/Steam/steamapps/workshop/content/294100"
                    : null,
                HistoricalStalePackageIdValue = NormalizeHistoricalStalePackageId(input.ObservedPackageId),
                HistoricalStalePackageIdSource = NormalizeHistoricalStalePackageId(input.ObservedPackageId) != null
                    ? input.ObservedPackageIdSource ?? "StartupEntryInput.ObservedPackageId"
                    : null,
            };

            return facts;
        }

        private static string NormalizePath(string value)
        {
            return Path.GetFullPath(value);
        }

        private static string? TryResolveWorkshopContentRoot()
        {
            var programFilesX86 = Environment.GetFolderPath(Environment.SpecialFolder.ProgramFilesX86);
            if (string.IsNullOrWhiteSpace(programFilesX86))
            {
                return null;
            }

            var candidate = Path.GetFullPath(
                Path.Combine(programFilesX86, "Steam", "steamapps", "workshop", "content", "294100"));

            return candidate;
        }

        private static string? NormalizeHistoricalStalePackageId(string? observedPackageId)
        {
            if (string.IsNullOrWhiteSpace(observedPackageId))
            {
                return null;
            }

            var normalized = observedPackageId.Trim();
            if (string.Equals(normalized, LockedPackageIdentity, StringComparison.OrdinalIgnoreCase))
            {
                return null;
            }

            return normalized;
        }
    }
}
