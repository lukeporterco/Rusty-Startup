using RustyStartup.Managed.Diagnostics;

namespace RustyStartup.Managed.Bootstrap
{
    public static class RustyStartupMod
    {
        private static StartupEntrySession? _activeSession;

        public static StartupEntryResult Initialize(StartupEntryInput input, BootstrapDiagnostics? diagnostics = null)
        {
            diagnostics ??= new BootstrapDiagnostics();

            _activeSession?.Dispose();
            _activeSession = null;

            var result = StartupEntryOrchestrator.Run(input, diagnostics);
            if (result.Status == StartupEntryStatus.Ready && result.Session != null)
            {
                _activeSession = result.Session;
            }

            return result;
        }

        public static void Shutdown(BootstrapDiagnostics? diagnostics = null)
        {
            diagnostics ??= new BootstrapDiagnostics();

            _activeSession?.Dispose();
            _activeSession = null;

            diagnostics.Emit(
                surface: "startup_entry_status",
                status: "shutdown",
                message: "Managed bootstrap session shutdown.");
        }
    }
}
