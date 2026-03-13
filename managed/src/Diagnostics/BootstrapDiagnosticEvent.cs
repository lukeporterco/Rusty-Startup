using System;
using System.Collections.Generic;
using System.Text;

namespace RustyStartup.Managed.Diagnostics
{
    public sealed class BootstrapDiagnosticEvent
    {
        private readonly IReadOnlyDictionary<string, string> _data;

        public BootstrapDiagnosticEvent(
            string surface,
            string status,
            string message,
            IReadOnlyDictionary<string, string>? data = null)
        {
            TimestampUtc = DateTime.UtcNow;
            Surface = surface;
            Status = status;
            Message = message;
            _data = data ?? new Dictionary<string, string>();
        }

        public DateTime TimestampUtc { get; }

        public string Surface { get; }

        public string Status { get; }

        public string Message { get; }

        public IReadOnlyDictionary<string, string> Data => _data;

        public string ToJsonLine()
        {
            var builder = new StringBuilder(256);
            builder.Append('{');
            AppendPair(builder, "timestampUtc", TimestampUtc.ToString("O"));
            builder.Append(',');
            AppendPair(builder, "surface", Surface);
            builder.Append(',');
            AppendPair(builder, "status", Status);
            builder.Append(',');
            AppendPair(builder, "message", Message);
            builder.Append(',');
            builder.Append("\"data\":{");

            var first = true;
            foreach (var pair in _data)
            {
                if (!first)
                {
                    builder.Append(',');
                }

                AppendPair(builder, pair.Key, pair.Value);
                first = false;
            }

            builder.Append("}}");
            return builder.ToString();
        }

        private static void AppendPair(StringBuilder builder, string key, string value)
        {
            builder.Append('"');
            builder.Append(Escape(key));
            builder.Append("\":\"");
            builder.Append(Escape(value));
            builder.Append('"');
        }

        private static string Escape(string value)
        {
            var builder = new StringBuilder(value.Length + 8);
            for (var i = 0; i < value.Length; i++)
            {
                var ch = value[i];
                switch (ch)
                {
                    case '\\':
                        builder.Append("\\\\");
                        break;
                    case '"':
                        builder.Append("\\\"");
                        break;
                    case '\r':
                        builder.Append("\\r");
                        break;
                    case '\n':
                        builder.Append("\\n");
                        break;
                    case '\t':
                        builder.Append("\\t");
                        break;
                    default:
                        builder.Append(ch);
                        break;
                }
            }

            return builder.ToString();
        }
    }
}
