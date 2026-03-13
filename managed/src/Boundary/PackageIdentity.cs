using System;

namespace RustyStartup.Managed.Boundary
{
    public static class PackageIdentity
    {
        public const string LockedV1PackageId = "rustystartup.core";

        public static PackageIdentityStatus Validate(string? observedPackageId, string? observedSource)
        {
            var normalizedObserved = string.IsNullOrWhiteSpace(observedPackageId)
                ? "unknown"
                : observedPackageId!.Trim();

            var source = string.IsNullOrWhiteSpace(observedSource)
                ? "unspecified"
                : observedSource!.Trim();

            var isMatch = string.Equals(
                LockedV1PackageId,
                normalizedObserved,
                StringComparison.OrdinalIgnoreCase);

            return new PackageIdentityStatus(
                LockedV1PackageId,
                normalizedObserved,
                source,
                isMatch);
        }
    }

    public sealed class PackageIdentityStatus
    {
        public PackageIdentityStatus(
            string expectedPackageId,
            string observedPackageId,
            string observedSource,
            bool isMatch)
        {
            ExpectedPackageId = expectedPackageId;
            ObservedPackageId = observedPackageId;
            ObservedSource = observedSource;
            IsMatch = isMatch;
        }

        public string ExpectedPackageId { get; }

        public string ObservedPackageId { get; }

        public string ObservedSource { get; }

        public bool IsMatch { get; }
    }
}
