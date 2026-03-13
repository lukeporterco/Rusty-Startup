using System;
using System.Collections.Generic;

namespace RustyStartup.Managed.Diagnostics
{
    public sealed class BootstrapDiagnostics
    {
        private readonly List<BootstrapDiagnosticEvent> _events = new List<BootstrapDiagnosticEvent>();
        private readonly Action<string> _sink;

        public BootstrapDiagnostics(Action<string>? sink = null)
        {
            _sink = sink ?? Console.WriteLine;
        }

        public IReadOnlyList<BootstrapDiagnosticEvent> Events => _events.AsReadOnly();

        public void Emit(string surface, string status, string message, IReadOnlyDictionary<string, string>? data = null)
        {
            var evt = new BootstrapDiagnosticEvent(surface, status, message, data);
            _events.Add(evt);
            _sink(evt.ToJsonLine());
        }
    }
}
