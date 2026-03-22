use crate::package_resolution::PackageDiscoveryReport;

pub fn render_package_discovery_report(report: &PackageDiscoveryReport) -> String {
    let mut output = String::new();

    push_line(
        &mut output,
        "package_resolution_result",
        &report.result.status,
    );
    push_line(
        &mut output,
        "package_resolution_reason_code",
        &report.result.reason_code,
    );
    push_line(
        &mut output,
        "package_resolution_reason",
        &report.result.reason,
    );

    for diagnostic in &report.diagnostics {
        push_line(&mut output, "diagnostic", diagnostic);
    }

    output
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
