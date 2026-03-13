using System;
using RustyStartup.Managed.Interop;

namespace RustyStartup.Managed.Boundary
{
    public static class AbiHandshake
    {
        public const int ExpectedAbiVersion = 1;
        public const ulong RequiredCapabilitiesMask = 0x1UL;

        public static AbiHandshakeStatus Validate(NativeBootstrapBindings bindings)
        {
            try
            {
                var reportedAbiVersion = bindings.GetAbiVersion();
                var reportedCapabilities = bindings.GetCapabilities();

                if (reportedAbiVersion != ExpectedAbiVersion)
                {
                    return AbiHandshakeStatus.Failure(
                        reportedAbiVersion,
                        reportedCapabilities,
                        "ABI version mismatch.");
                }

                if ((reportedCapabilities & RequiredCapabilitiesMask) != RequiredCapabilitiesMask)
                {
                    return AbiHandshakeStatus.Failure(
                        reportedAbiVersion,
                        reportedCapabilities,
                        "ABI capability mismatch.");
                }

                return AbiHandshakeStatus.Success(reportedAbiVersion, reportedCapabilities);
            }
            catch (Exception ex)
            {
                return AbiHandshakeStatus.Failure(-1, 0, "ABI handshake call failed: " + ex.Message);
            }
        }
    }

    public sealed class AbiHandshakeStatus
    {
        private AbiHandshakeStatus(
            bool isCompatible,
            int reportedAbiVersion,
            ulong reportedCapabilities,
            string reason)
        {
            IsCompatible = isCompatible;
            ReportedAbiVersion = reportedAbiVersion;
            ReportedCapabilities = reportedCapabilities;
            Reason = reason;
        }

        public bool IsCompatible { get; }

        public int ReportedAbiVersion { get; }

        public ulong ReportedCapabilities { get; }

        public string Reason { get; }

        public static AbiHandshakeStatus Success(int reportedAbiVersion, ulong reportedCapabilities)
        {
            return new AbiHandshakeStatus(true, reportedAbiVersion, reportedCapabilities, "compatible");
        }

        public static AbiHandshakeStatus Failure(int reportedAbiVersion, ulong reportedCapabilities, string reason)
        {
            return new AbiHandshakeStatus(false, reportedAbiVersion, reportedCapabilities, reason);
        }
    }
}
