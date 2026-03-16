
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Xml.Linq;

namespace RustyStartup.Managed.Boundary
{
    public static class SelfPackageLayoutResolver
    {
        private const string ManagedAssemblyFileName = "RustyStartup.Managed.dll";

        public static SelfPackageLayoutResolution Resolve(SelfPackageLayoutResolverInput input)
        {
            if (input == null)
            {
                throw new ArgumentNullException(nameof(input));
            }

            var runtimeVersionBasis = NormalizeRequiredValue(input.RuntimeVersionBasis, "1.6");
            var runtimeVersionSource = NormalizeRequiredValue(input.RuntimeVersionSource, "bootstrap_default");
            var observedPackageId = NormalizeRequiredValue(input.ObservedPackageId, "unknown");
            var observedPackageIdSource = NormalizeRequiredValue(input.ObservedPackageIdSource, "unspecified");

            var packageRootCandidates = new List<PackageRootCandidate>();
            if (!string.IsNullOrWhiteSpace(input.CandidateSelfPackageRootPath))
            {
                var hintResult = TryFindSelfPackageRootFromHint(input.CandidateSelfPackageRootPath!, "mod_root_hint");
                if (!hintResult.IsSuccess)
                {
                    return SelfPackageLayoutResolution.Failure(
                        runtimeVersionBasis,
                        runtimeVersionSource,
                        observedPackageId,
                        observedPackageIdSource,
                        "package_root_hint_invalid",
                        hintResult.FailureReason,
                        "unsupported");
                }

                packageRootCandidates.Add(new PackageRootCandidate(hintResult.PackageRootPath!, "mod_root_hint"));
            }

            if (!string.IsNullOrWhiteSpace(input.ManagedSelfAssemblyPath))
            {
                var assemblyResult = TryFindSelfPackageRootFromHint(input.ManagedSelfAssemblyPath!, "managed_self_assembly_path");
                if (!assemblyResult.IsSuccess)
                {
                    return SelfPackageLayoutResolution.Failure(
                        runtimeVersionBasis,
                        runtimeVersionSource,
                        observedPackageId,
                        observedPackageIdSource,
                        "self_assembly_location_unresolved",
                        assemblyResult.FailureReason,
                        "unsupported");
                }

                packageRootCandidates.Add(new PackageRootCandidate(assemblyResult.PackageRootPath!, "managed_self_assembly_path"));
            }

            if (packageRootCandidates.Count == 0)
            {
                return SelfPackageLayoutResolution.Failure(
                    runtimeVersionBasis,
                    runtimeVersionSource,
                    observedPackageId,
                    observedPackageIdSource,
                    "package_root_not_detected",
                    "No self-package root hint or self-assembly location hint produced a canonical rustystartup.core root.",
                    "unsupported");
            }

            var distinctRoots = packageRootCandidates
                .Select(x => x.PackageRootPath)
                .Distinct(GetPathComparer())
                .ToList();

            if (distinctRoots.Count > 1)
            {
                if (IsAncestorPath(distinctRoots[0], distinctRoots[1]) || IsAncestorPath(distinctRoots[1], distinctRoots[0]))
                {
                    return SelfPackageLayoutResolution.Failure(
                        runtimeVersionBasis,
                        runtimeVersionSource,
                        observedPackageId,
                        observedPackageIdSource,
                        "nested_package_root_detected",
                        "Nested rustystartup.core package roots were detected between bootstrap hints.",
                        "unsupported");
                }

                return SelfPackageLayoutResolution.Failure(
                    runtimeVersionBasis,
                    runtimeVersionSource,
                    observedPackageId,
                    observedPackageIdSource,
                    "duplicate_package_root_detected",
                    "Multiple distinct rustystartup.core package roots were detected between bootstrap hints.",
                    "unsupported");
            }

            var packageRoot = distinctRoots[0];
            var packageRootDetectionSource = string.Join(
                "+",
                packageRootCandidates
                    .Where(x => GetPathComparer().Equals(x.PackageRootPath, packageRoot))
                    .Select(x => x.DetectionSource)
                    .Distinct(StringComparer.OrdinalIgnoreCase));

            var ancestorDuplicateCheck = HasAncestorWithSelfIdentity(packageRoot);
            if (ancestorDuplicateCheck.HasDuplicate)
            {
                return SelfPackageLayoutResolution.Failure(
                    runtimeVersionBasis,
                    runtimeVersionSource,
                    observedPackageId,
                    observedPackageIdSource,
                    "nested_package_root_detected",
                    ancestorDuplicateCheck.Reason,
                    "unsupported");
            }

            var loadFoldersPath = Path.Combine(packageRoot, "LoadFolders.xml");
            var loadFoldersStatus = File.Exists(loadFoldersPath) ? "present" : "not_present";
            var selectedContentRoot = packageRoot;
            var layoutDecisionBasis = "root_only_layout";
            var loadFolderGateStatus = "no_conditional_gates";
            if (File.Exists(loadFoldersPath))
            {
                var loadFoldersDecision = EvaluateLoadFolders(loadFoldersPath, packageRoot, runtimeVersionBasis);
                if (!loadFoldersDecision.IsSuccess)
                {
                    return SelfPackageLayoutResolution.Failure(
                        runtimeVersionBasis,
                        runtimeVersionSource,
                        observedPackageId,
                        observedPackageIdSource,
                        loadFoldersDecision.FailureReasonCode,
                        loadFoldersDecision.FailureReason,
                        loadFoldersDecision.LoadFolderGateStatus,
                        packageRoot,
                        packageRootDetectionSource,
                        loadFoldersPath,
                        loadFoldersStatus);
                }

                selectedContentRoot = loadFoldersDecision.SelectedContentRoot!;
                layoutDecisionBasis = "loadfolders_routed_layout";
                loadFolderGateStatus = loadFoldersDecision.LoadFolderGateStatus;
            }
            else
            {
                var runtimeFolderCandidate = Path.Combine(packageRoot, runtimeVersionBasis);
                if (Directory.Exists(runtimeFolderCandidate))
                {
                    selectedContentRoot = runtimeFolderCandidate;
                    layoutDecisionBasis = "version_folder_layout";
                }
            }

            var platform = NativePathResolver.DetectPlatform();
            var architecture = NativePathResolver.DetectArchitectureSegment();
            var ridPrefix = NativePathResolver.GetRidPrefix(platform);
            var rid = ridPrefix + "-" + architecture;
            var nativeFileName = NativePathResolver.GetNativeFileName(platform);

            var managedAssemblyPath = Path.GetFullPath(Path.Combine(selectedContentRoot, "Assemblies", ManagedAssemblyFileName));
            var nativePayloadPath = Path.GetFullPath(Path.Combine(selectedContentRoot, "Native", rid, nativeFileName));
            var managedAssemblyExists = File.Exists(managedAssemblyPath);
            var nativePayloadExists = File.Exists(nativePayloadPath);
            var packageRelativeManagedAssemblyPath = GetPackageRelativePath(packageRoot, managedAssemblyPath);
            var packageRelativeNativePayloadPath = GetPackageRelativePath(packageRoot, nativePayloadPath);

            return SelfPackageLayoutResolution.Success(
                runtimeVersionBasis,
                runtimeVersionSource,
                observedPackageId,
                observedPackageIdSource,
                packageRoot,
                packageRootDetectionSource,
                selectedContentRoot,
                layoutDecisionBasis,
                loadFolderGateStatus,
                loadFoldersPath,
                loadFoldersStatus,
                managedAssemblyPath,
                packageRelativeManagedAssemblyPath,
                managedAssemblyExists,
                nativePayloadPath,
                packageRelativeNativePayloadPath,
                nativePayloadExists,
                platform,
                rid,
                nativeFileName);
        }

        private static string NormalizeRequiredValue(string? value, string fallback)
        {
            if (string.IsNullOrWhiteSpace(value))
            {
                return fallback;
            }

            return value!.Trim();
        }

        private static HintResolutionResult TryFindSelfPackageRootFromHint(string hintPath, string source)
        {
            if (string.IsNullOrWhiteSpace(hintPath))
            {
                return HintResolutionResult.Failure("Hint source '" + source + "' was empty.");
            }

            string currentPath;
            try
            {
                currentPath = File.Exists(hintPath)
                    ? Path.GetDirectoryName(Path.GetFullPath(hintPath)) ?? string.Empty
                    : Path.GetFullPath(hintPath);
            }
            catch (Exception ex)
            {
                return HintResolutionResult.Failure("Hint source '" + source + "' could not be normalized: " + ex.Message);
            }

            if (string.IsNullOrWhiteSpace(currentPath) || !Directory.Exists(currentPath))
            {
                return HintResolutionResult.Failure("Hint source '" + source + "' does not point to an existing directory path.");
            }

            var currentDirectory = new DirectoryInfo(currentPath);
            while (currentDirectory != null)
            {
                var aboutPath = Path.Combine(currentDirectory.FullName, "About", "About.xml");
                if (File.Exists(aboutPath))
                {
                    var packageIdResult = TryReadPackageIdFromAboutXml(aboutPath);
                    if (!packageIdResult.IsSuccess)
                    {
                        return HintResolutionResult.Failure("Failed to parse About.xml at '" + aboutPath + "': " + packageIdResult.FailureReason);
                    }

                    if (string.Equals(packageIdResult.PackageId, PackageIdentity.LockedV1PackageId, StringComparison.OrdinalIgnoreCase))
                    {
                        return HintResolutionResult.Resolved(currentDirectory.FullName);
                    }
                }

                currentDirectory = currentDirectory.Parent;
            }

            return HintResolutionResult.Failure(
                "Hint source '" + source + "' did not resolve to a parent directory with About/About.xml packageId '" +
                PackageIdentity.LockedV1PackageId + "'.");
        }

        private static PackageIdReadResult TryReadPackageIdFromAboutXml(string aboutXmlPath)
        {
            try
            {
                var document = XDocument.Load(aboutXmlPath);
                var packageId = document
                    .Descendants()
                    .FirstOrDefault(x => string.Equals(x.Name.LocalName, "packageId", StringComparison.OrdinalIgnoreCase))
                    ?.Value
                    ?.Trim();

                if (string.IsNullOrWhiteSpace(packageId))
                {
                    return PackageIdReadResult.Failure("About.xml did not contain a packageId element.");
                }

                    return PackageIdReadResult.FromPackageId(packageId!);
            }
            catch (Exception ex)
            {
                return PackageIdReadResult.Failure(ex.Message);
            }
        }

        private static AncestorDuplicateCheckResult HasAncestorWithSelfIdentity(string packageRoot)
        {
            var current = new DirectoryInfo(packageRoot).Parent;
            while (current != null)
            {
                var aboutPath = Path.Combine(current.FullName, "About", "About.xml");
                if (File.Exists(aboutPath))
                {
                    var packageIdResult = TryReadPackageIdFromAboutXml(aboutPath);
                    if (!packageIdResult.IsSuccess)
                    {
                        return AncestorDuplicateCheckResult.Duplicate(
                            "An ancestor About.xml could not be parsed while validating nested rustystartup.core roots: " + aboutPath);
                    }

                    if (string.Equals(packageIdResult.PackageId, PackageIdentity.LockedV1PackageId, StringComparison.OrdinalIgnoreCase))
                    {
                        return AncestorDuplicateCheckResult.Duplicate(
                            "An ancestor rustystartup.core package root was detected at '" + current.FullName + "'.");
                    }
                }

                current = current.Parent;
            }

            return AncestorDuplicateCheckResult.NoDuplicate();
        }

        private static LoadFoldersDecision EvaluateLoadFolders(string loadFoldersPath, string packageRoot, string runtimeVersionBasis)
        {
            XDocument document;
            try
            {
                document = XDocument.Load(loadFoldersPath);
            }
            catch (Exception ex)
            {
                return LoadFoldersDecision.Failure("loadfolders_parse_failed", "LoadFolders.xml could not be parsed: " + ex.Message, "unsupported");
            }

            var root = document.Root;
            if (root == null)
            {
                return LoadFoldersDecision.Failure("loadfolders_invalid", "LoadFolders.xml did not contain a root element.", "unsupported");
            }

            var rules = new List<LoadFolderRule>();
            foreach (var element in root.Elements())
            {
                if (element.HasElements)
                {
                    return LoadFoldersDecision.Failure(
                        "loadfolders_ambiguous_or_conditional",
                        "LoadFolders.xml rule '" + element.Name.LocalName + "' contains nested elements that this bootstrap-local resolver cannot prove.",
                        "unsupported_conditional_or_ambiguous");
                }

                if (element.Attributes().Any())
                {
                    return LoadFoldersDecision.Failure(
                        "loadfolders_ambiguous_or_conditional",
                        "LoadFolders.xml rule '" + element.Name.LocalName + "' contains attributes that this bootstrap-local resolver cannot prove.",
                        "unsupported_conditional_or_ambiguous");
                }

                var key = (element.Name.LocalName ?? string.Empty).Trim();
                var folder = (element.Value ?? string.Empty).Trim();
                if (string.IsNullOrWhiteSpace(key) || string.IsNullOrWhiteSpace(folder))
                {
                    return LoadFoldersDecision.Failure("loadfolders_invalid", "LoadFolders.xml contained an empty key or folder value.", "unsupported");
                }

                rules.Add(new LoadFolderRule(key, folder));
            }

            if (rules.Count == 0)
            {
                return LoadFoldersDecision.Failure("loadfolders_invalid", "LoadFolders.xml did not contain any load-folder rules.", "unsupported");
            }

            var runtimeKeys = GetRuntimeKeys(runtimeVersionBasis);
            var runtimeMatches = rules.Where(x => runtimeKeys.Contains(x.Key, StringComparer.OrdinalIgnoreCase)).ToList();
            if (runtimeMatches.Count > 1)
            {
                return LoadFoldersDecision.Failure(
                    "loadfolders_ambiguous_runtime_rule",
                    "LoadFolders.xml contained multiple runtime rules for basis '" + runtimeVersionBasis + "'.",
                    "unsupported_conditional_or_ambiguous");
            }

            LoadFolderRule? selectedRule = null;
            if (runtimeMatches.Count == 1)
            {
                selectedRule = runtimeMatches[0];
            }
            else
            {
                var defaultRules = rules.Where(x => string.Equals(x.Key, "default", StringComparison.OrdinalIgnoreCase)).ToList();
                if (defaultRules.Count > 1)
                {
                    return LoadFoldersDecision.Failure(
                        "loadfolders_ambiguous_default_rule",
                        "LoadFolders.xml contained multiple default rules.",
                        "unsupported_conditional_or_ambiguous");
                }

                if (defaultRules.Count == 1)
                {
                    selectedRule = defaultRules[0];
                }
            }

            if (selectedRule == null)
            {
                return LoadFoldersDecision.Failure(
                    "loadfolders_runtime_not_resolved",
                    "LoadFolders.xml did not provide a provable rule for runtime basis '" + runtimeVersionBasis + "'.",
                    "unsupported_conditional_or_ambiguous");
            }

            var selectedFolder = selectedRule.Folder
                .Replace('/', Path.DirectorySeparatorChar)
                .Replace('\\', Path.DirectorySeparatorChar);

            string selectedContentRoot;
            try
            {
                selectedContentRoot = Path.GetFullPath(Path.Combine(packageRoot, selectedFolder));
            }
            catch (Exception ex)
            {
                return LoadFoldersDecision.Failure("loadfolders_invalid_path", "LoadFolders.xml selected folder path is invalid: " + ex.Message, "unsupported");
            }

            if (!IsSamePathOrDescendant(packageRoot, selectedContentRoot))
            {
                return LoadFoldersDecision.Failure("loadfolders_out_of_package_root", "LoadFolders.xml selected content root outside the package root.", "unsupported");
            }

            if (!Directory.Exists(selectedContentRoot))
            {
                return LoadFoldersDecision.Failure(
                    "loadfolders_selected_root_missing",
                    "LoadFolders.xml selected content root does not exist: " + selectedContentRoot,
                    "unsupported");
            }

            return LoadFoldersDecision.Resolved(selectedContentRoot, "no_conditional_gates");
        }

        private static string[] GetRuntimeKeys(string runtimeVersionBasis)
        {
            var value = runtimeVersionBasis.Trim();
            if (value.StartsWith("v", StringComparison.OrdinalIgnoreCase))
            {
                return new[] { value, value.Substring(1) };
            }

            return new[] { value, "v" + value };
        }
        private static bool IsAncestorPath(string ancestor, string descendant)
        {
            var normalizedAncestor = EnsureTrailingDirectorySeparator(Path.GetFullPath(ancestor));
            var normalizedDescendant = EnsureTrailingDirectorySeparator(Path.GetFullPath(descendant));
            return normalizedDescendant.StartsWith(normalizedAncestor, GetPathComparison());
        }

        private static bool IsSamePathOrDescendant(string parent, string child)
        {
            if (GetPathComparer().Equals(parent, child))
            {
                return true;
            }

            return IsAncestorPath(parent, child);
        }

        private static string EnsureTrailingDirectorySeparator(string path)
        {
            if (path.EndsWith(Path.DirectorySeparatorChar.ToString(), GetPathComparison()) ||
                path.EndsWith(Path.AltDirectorySeparatorChar.ToString(), GetPathComparison()))
            {
                return path;
            }

            return path + Path.DirectorySeparatorChar;
        }

        private static string GetPackageRelativePath(string packageRoot, string absolutePath)
        {
            var fromPath = EnsureTrailingDirectorySeparator(Path.GetFullPath(packageRoot));
            var toPath = Path.GetFullPath(absolutePath);

            var relativeUri = new Uri(fromPath).MakeRelativeUri(new Uri(toPath));
            return Uri.UnescapeDataString(relativeUri.ToString())
                .Replace('/', Path.DirectorySeparatorChar);
        }

        private static StringComparer GetPathComparer()
        {
            return RuntimeInformation.IsOSPlatform(OSPlatform.Windows)
                ? StringComparer.OrdinalIgnoreCase
                : StringComparer.Ordinal;
        }

        private static StringComparison GetPathComparison()
        {
            return RuntimeInformation.IsOSPlatform(OSPlatform.Windows)
                ? StringComparison.OrdinalIgnoreCase
                : StringComparison.Ordinal;
        }
    }

    public sealed class SelfPackageLayoutResolverInput
    {
        public SelfPackageLayoutResolverInput(
            string? candidateSelfPackageRootPath,
            string? observedPackageId,
            string? observedPackageIdSource,
            string? runtimeVersionBasis,
            string? runtimeVersionSource,
            string? managedSelfAssemblyPath)
        {
            CandidateSelfPackageRootPath = candidateSelfPackageRootPath;
            ObservedPackageId = observedPackageId;
            ObservedPackageIdSource = observedPackageIdSource;
            RuntimeVersionBasis = runtimeVersionBasis;
            RuntimeVersionSource = runtimeVersionSource;
            ManagedSelfAssemblyPath = managedSelfAssemblyPath;
        }

        public string? CandidateSelfPackageRootPath { get; }

        public string? ObservedPackageId { get; }

        public string? ObservedPackageIdSource { get; }

        public string? RuntimeVersionBasis { get; }

        public string? RuntimeVersionSource { get; }

        public string? ManagedSelfAssemblyPath { get; }
    }

    public sealed class SelfPackageLayoutResolution
    {
        private SelfPackageLayoutResolution(
            bool isResolved,
            string runtimeVersionBasis,
            string runtimeVersionSource,
            string observedPackageId,
            string observedPackageIdSource,
            string? packageRoot,
            string? packageRootDetectionSource,
            string? selectedActiveContentRoot,
            string? layoutDecisionBasis,
            string loadFolderGateStatus,
            string? loadFoldersEvidencePath,
            string loadFoldersEvidenceStatus,
            string? absoluteManagedAssemblyPath,
            string? packageRelativeManagedAssemblyPath,
            bool managedAssemblyExists,
            string? absoluteNativePayloadPath,
            string? packageRelativeNativePayloadPath,
            bool nativePayloadExists,
            NativePlatform? platform,
            string? runtimeIdentifier,
            string? nativeFileName,
            string failureReasonCode,
            string failureReason)
        {
            IsResolved = isResolved;
            RuntimeVersionBasis = runtimeVersionBasis;
            RuntimeVersionSource = runtimeVersionSource;
            ObservedPackageId = observedPackageId;
            ObservedPackageIdSource = observedPackageIdSource;
            PackageRoot = packageRoot;
            PackageRootDetectionSource = packageRootDetectionSource;
            SelectedActiveContentRoot = selectedActiveContentRoot;
            LayoutDecisionBasis = layoutDecisionBasis;
            LoadFolderGateStatus = loadFolderGateStatus;
            LoadFoldersEvidencePath = loadFoldersEvidencePath;
            LoadFoldersEvidenceStatus = loadFoldersEvidenceStatus;
            AbsoluteManagedAssemblyPath = absoluteManagedAssemblyPath;
            PackageRelativeManagedAssemblyPath = packageRelativeManagedAssemblyPath;
            ManagedAssemblyExists = managedAssemblyExists;
            AbsoluteNativePayloadPath = absoluteNativePayloadPath;
            PackageRelativeNativePayloadPath = packageRelativeNativePayloadPath;
            NativePayloadExists = nativePayloadExists;
            Platform = platform;
            RuntimeIdentifier = runtimeIdentifier;
            NativeFileName = nativeFileName;
            FailureReasonCode = failureReasonCode;
            FailureReason = failureReason;
        }

        public bool IsResolved { get; }

        public string RuntimeVersionBasis { get; }

        public string RuntimeVersionSource { get; }

        public string ObservedPackageId { get; }

        public string ObservedPackageIdSource { get; }

        public string? PackageRoot { get; }

        public string? PackageRootDetectionSource { get; }

        public string? SelectedActiveContentRoot { get; }

        public string? LayoutDecisionBasis { get; }

        public string LoadFolderGateStatus { get; }

        public string? LoadFoldersEvidencePath { get; }

        public string LoadFoldersEvidenceStatus { get; }

        public string? AbsoluteManagedAssemblyPath { get; }

        public string? PackageRelativeManagedAssemblyPath { get; }

        public bool ManagedAssemblyExists { get; }

        public string? AbsoluteNativePayloadPath { get; }

        public string? PackageRelativeNativePayloadPath { get; }

        public bool NativePayloadExists { get; }

        public NativePlatform? Platform { get; }
        public string? RuntimeIdentifier { get; }

        public string? NativeFileName { get; }

        public string FailureReasonCode { get; }

        public string FailureReason { get; }

        public static SelfPackageLayoutResolution Success(
            string runtimeVersionBasis,
            string runtimeVersionSource,
            string observedPackageId,
            string observedPackageIdSource,
            string packageRoot,
            string packageRootDetectionSource,
            string selectedActiveContentRoot,
            string layoutDecisionBasis,
            string loadFolderGateStatus,
            string loadFoldersEvidencePath,
            string loadFoldersEvidenceStatus,
            string absoluteManagedAssemblyPath,
            string packageRelativeManagedAssemblyPath,
            bool managedAssemblyExists,
            string absoluteNativePayloadPath,
            string packageRelativeNativePayloadPath,
            bool nativePayloadExists,
            NativePlatform platform,
            string runtimeIdentifier,
            string nativeFileName)
        {
            return new SelfPackageLayoutResolution(
                true,
                runtimeVersionBasis,
                runtimeVersionSource,
                observedPackageId,
                observedPackageIdSource,
                packageRoot,
                packageRootDetectionSource,
                selectedActiveContentRoot,
                layoutDecisionBasis,
                loadFolderGateStatus,
                loadFoldersEvidencePath,
                loadFoldersEvidenceStatus,
                absoluteManagedAssemblyPath,
                packageRelativeManagedAssemblyPath,
                managedAssemblyExists,
                absoluteNativePayloadPath,
                packageRelativeNativePayloadPath,
                nativePayloadExists,
                platform,
                runtimeIdentifier,
                nativeFileName,
                "none",
                "none");
        }

        public static SelfPackageLayoutResolution Failure(
            string runtimeVersionBasis,
            string runtimeVersionSource,
            string observedPackageId,
            string observedPackageIdSource,
            string failureReasonCode,
            string failureReason,
            string loadFolderGateStatus,
            string? packageRoot = null,
            string? packageRootDetectionSource = null,
            string? loadFoldersEvidencePath = null,
            string loadFoldersEvidenceStatus = "unknown")
        {
            return new SelfPackageLayoutResolution(
                false,
                runtimeVersionBasis,
                runtimeVersionSource,
                observedPackageId,
                observedPackageIdSource,
                packageRoot,
                packageRootDetectionSource,
                null,
                "unsupported_or_ambiguous_layout",
                loadFolderGateStatus,
                loadFoldersEvidencePath,
                loadFoldersEvidenceStatus,
                null,
                null,
                false,
                null,
                null,
                false,
                null,
                null,
                null,
                failureReasonCode,
                failureReason);
        }
    }

    internal sealed class PackageRootCandidate
    {
        public PackageRootCandidate(string packageRootPath, string detectionSource)
        {
            PackageRootPath = packageRootPath;
            DetectionSource = detectionSource;
        }

        public string PackageRootPath { get; }

        public string DetectionSource { get; }
    }

    internal sealed class HintResolutionResult
    {
        private HintResolutionResult(bool success, string? packageRootPath, string failureReason)
        {
            IsSuccess = success;
            PackageRootPath = packageRootPath;
            FailureReason = failureReason;
        }

        public bool IsSuccess { get; }

        public string? PackageRootPath { get; }

        public string FailureReason { get; }

        public static HintResolutionResult Resolved(string packageRootPath)
        {
            return new HintResolutionResult(true, packageRootPath, "none");
        }

        public static HintResolutionResult Failure(string reason)
        {
            return new HintResolutionResult(false, null, reason);
        }
    }

    internal sealed class PackageIdReadResult
    {
        private PackageIdReadResult(bool success, string packageId, string failureReason)
        {
            IsSuccess = success;
            PackageId = packageId;
            FailureReason = failureReason;
        }

        public bool IsSuccess { get; }

        public string PackageId { get; }

        public string FailureReason { get; }

        public static PackageIdReadResult FromPackageId(string packageId)
        {
            return new PackageIdReadResult(true, packageId, "none");
        }

        public static PackageIdReadResult Failure(string reason)
        {
            return new PackageIdReadResult(false, string.Empty, reason);
        }
    }

    internal sealed class AncestorDuplicateCheckResult
    {
        private AncestorDuplicateCheckResult(bool hasDuplicate, string reason)
        {
            HasDuplicate = hasDuplicate;
            Reason = reason;
        }

        public bool HasDuplicate { get; }

        public string Reason { get; }

        public static AncestorDuplicateCheckResult NoDuplicate()
        {
            return new AncestorDuplicateCheckResult(false, "none");
        }

        public static AncestorDuplicateCheckResult Duplicate(string reason)
        {
            return new AncestorDuplicateCheckResult(true, reason);
        }
    }

    internal sealed class LoadFolderRule
    {
        public LoadFolderRule(string key, string folder)
        {
            Key = key;
            Folder = folder;
        }

        public string Key { get; }

        public string Folder { get; }
    }

    internal sealed class LoadFoldersDecision
    {
        private LoadFoldersDecision(bool success, string? selectedContentRoot, string failureReasonCode, string failureReason, string loadFolderGateStatus)
        {
            IsSuccess = success;
            SelectedContentRoot = selectedContentRoot;
            FailureReasonCode = failureReasonCode;
            FailureReason = failureReason;
            LoadFolderGateStatus = loadFolderGateStatus;
        }

        public bool IsSuccess { get; }

        public string? SelectedContentRoot { get; }

        public string FailureReasonCode { get; }

        public string FailureReason { get; }

        public string LoadFolderGateStatus { get; }

        public static LoadFoldersDecision Resolved(string selectedContentRoot, string loadFolderGateStatus)
        {
            return new LoadFoldersDecision(true, selectedContentRoot, "none", "none", loadFolderGateStatus);
        }

        public static LoadFoldersDecision Failure(string reasonCode, string reason, string loadFolderGateStatus)
        {
            return new LoadFoldersDecision(false, null, reasonCode, reason, loadFolderGateStatus);
        }
    }
}
