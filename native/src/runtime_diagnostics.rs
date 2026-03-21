use crate::runtime_context::{RuntimeContextReport, RuntimeDiagnosticLine};
use crate::runtime_input_fingerprint::RuntimeInputFingerprintClass;

pub fn render_runtime_context_report(report: &RuntimeContextReport) -> String {
    let mut output = String::new();
    push_line(
        &mut output,
        "fingerprint_status",
        report.fingerprint.status.as_str(),
    );
    push_line(
        &mut output,
        "fingerprint_hex",
        &report.fingerprint.fingerprint_hex,
    );
    push_line(
        &mut output,
        "fingerprint_reason",
        &report.fingerprint.reason,
    );
    push_line(
        &mut output,
        "classes_present",
        &render_classes(&report.fingerprint.classes_present),
    );

    for diagnostic in &report.diagnostics {
        push_line(
            &mut output,
            "diagnostic",
            &render_runtime_diagnostic_line(diagnostic),
        );
    }

    output
}

fn render_runtime_diagnostic_line(line: &RuntimeDiagnosticLine) -> String {
    let mut output = String::new();
    push_line(&mut output, "class", line.class.as_str());
    push_line(&mut output, "status", line.status.as_str());
    if let Some(value) = &line.value {
        push_line(&mut output, "value", value);
    }
    if let Some(normalized_value) = &line.normalized_value {
        push_line(&mut output, "normalized", normalized_value);
    }
    push_line(&mut output, "source", &line.source);
    push_line(&mut output, "reason", &line.reason);
    output
}

fn render_classes(classes: &[RuntimeInputFingerprintClass]) -> String {
    classes
        .iter()
        .map(|class| class.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

fn push_line(output: &mut String, key: &str, value: &str) {
    if !output.is_empty() {
        output.push('\n');
    }

    output.push_str(key);
    output.push('=');
    output.push_str(&escape(value));
}

fn escape(value: &str) -> String {
    let mut output = String::with_capacity(value.len() + 8);
    for ch in value.chars() {
        match ch {
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '=' => output.push_str("\\="),
            '|' => output.push_str("\\|"),
            _ => output.push(ch),
        }
    }

    output
}
