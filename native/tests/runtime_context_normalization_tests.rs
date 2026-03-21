use rustystartup_core::{
    build_runtime_context_report, build_runtime_context_report_from_ffi,
    rs_bootstrap_free_c_string, rs_bootstrap_submit_runtime_context, RuntimeAuthorityPairInput,
    RuntimeContextBootstrapInput, RuntimeContextInput, RuntimeFactInput, RuntimeFactStatus,
    RuntimeFingerprintStatus, RuntimeInputFingerprintClass,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

fn base_input() -> RuntimeContextInput {
    RuntimeContextInput {
        runtime_build_identity: RuntimeAuthorityPairInput {
            fresh: RuntimeFactInput::observed("RimWorld 1.6.4633 rev1261", "runtime_banner"),
            static_install_metadata: RuntimeFactInput::observed(
                "RimWorld 1.6.4630 rev1260",
                "Version.txt",
            ),
        },
        runtime_version_basis: RuntimeAuthorityPairInput {
            fresh: RuntimeFactInput::observed("rev1261", "runtime_banner"),
            static_install_metadata: RuntimeFactInput::observed("rev1260", "Version.txt"),
        },
        parser_mode: RuntimeFactInput::observed("default-new", "command_line"),
        selected_language_key: RuntimeFactInput::observed("en-US", "CultureInfo.CurrentUICulture.Name"),
        platform_basis: rustystartup_core::RuntimePlatformBasisInput {
            platform: RuntimeFactInput::observed("Windows", "RuntimeInformation"),
            operating_system: RuntimeFactInput::observed("Windows 11", "RuntimeInformation.OSDescription"),
            architecture: RuntimeFactInput::observed("x64", "RuntimeInformation.OSArchitecture"),
        },
        selected_self_package_root: RuntimeFactInput::observed(
            absolute_path("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\"),
            "SelfPackageLayoutResolver.PackageRoot",
        ),
        active_self_content_root: RuntimeFactInput::observed(
            absolute_path("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\"),
            "SelfPackageLayoutResolver.SelectedActiveContentRoot",
        ),
        managed_self_assembly_path: RuntimeFactInput::observed(
            absolute_path("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Assemblies\\RustyStartup.Managed.dll"),
            "StartupEntryOrchestrator.ManagedAssemblyPath",
        ),
        native_payload_path: RuntimeFactInput::observed(
            absolute_path("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Native\\win-x64\\rustystartup_core.dll"),
            "SelfPackageLayoutResolver.NativePayloadPath",
        ),
    }
}

#[test]
fn fresh_runtime_evidence_wins_over_static_install_metadata() {
    let report = build_runtime_context_report(base_input());

    assert_eq!(
        report
            .context
            .runtime_build_identity
            .primary
            .value
            .as_deref(),
        Some("RimWorld 1.6.4633 rev1261")
    );
    assert_eq!(
        report.context.runtime_build_identity.primary.status,
        RuntimeFactStatus::Observed
    );
    let secondary = report
        .context
        .runtime_build_identity
        .secondary_authority
        .as_ref()
        .expect("secondary authority should be recorded");
    assert_eq!(secondary.status, RuntimeFactStatus::SecondaryAuthority);
    assert_eq!(
        secondary.value.as_deref(),
        Some("RimWorld 1.6.4630 rev1260")
    );
    assert_eq!(
        report.fingerprint.status,
        rustystartup_core::RuntimeFingerprintStatus::Canonical
    );
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::RuntimeBuildIdentity
        && line.status == RuntimeFactStatus::SecondaryAuthority));
}

#[test]
fn selected_self_package_root_and_active_content_root_normalize_as_path_identity_inputs() {
    let report = build_runtime_context_report(base_input());

    assert_eq!(
        report
            .context
            .selected_self_package_root
            .normalized_value
            .as_deref(),
        Some(expected_windows_or_unix_path(
            "c:/users/lukep/source/repos/rusty startup/rustystartup.core",
            "/Users/lukep/source/repos/Rusty Startup/rustystartup.core"
        ))
    );
    assert_eq!(
        report
            .context
            .active_self_content_root
            .normalized_value
            .as_deref(),
        Some(expected_windows_or_unix_path(
            "c:/users/lukep/source/repos/rusty startup/rustystartup.core/1.6",
            "/Users/lukep/source/repos/Rusty Startup/rustystartup.core/1.6"
        ))
    );
    assert_eq!(
        report.context.selected_self_package_root.status,
        RuntimeFactStatus::Observed
    );
    assert_eq!(
        report.context.active_self_content_root.status,
        RuntimeFactStatus::Observed
    );
    assert!(report
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::BootstrapSelfPackageInputs));
}

#[test]
fn legacy_parser_mode_is_explicitly_degraded() {
    let mut input = base_input();
    input.parser_mode = RuntimeFactInput::observed("legacy-xml-deserializer", "command_line");

    let report = build_runtime_context_report(input);

    assert_eq!(
        report.context.parser_mode.status,
        RuntimeFactStatus::Degraded
    );
    assert_eq!(
        report.context.parser_mode.value.as_deref(),
        Some("legacy-xml-deserializer")
    );
    assert_eq!(
        report.fingerprint.status,
        rustystartup_core::RuntimeFingerprintStatus::Degraded
    );
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::ParserMode
        && line.status == RuntimeFactStatus::Degraded
        && line.reason.contains("non-primary parity")));
    assert_eq!(
        report.fingerprint.status,
        rustystartup_core::RuntimeFingerprintStatus::Degraded
    );
}

#[test]
fn missing_inputs_remain_explicit_and_diagnostic() {
    let mut input = base_input();
    input.selected_language_key = RuntimeFactInput::missing();
    input.managed_self_assembly_path = RuntimeFactInput::missing();
    input.native_payload_path = RuntimeFactInput::missing();

    let report = build_runtime_context_report(input);

    assert_eq!(
        report.context.selected_language_key.status,
        RuntimeFactStatus::Unavailable
    );
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::SelectedLanguageKey
        && line.status == RuntimeFactStatus::Unavailable));
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::MachineLocalPathIdentity
        && line.status == RuntimeFactStatus::Unavailable));
    assert_eq!(
        report.fingerprint.status,
        rustystartup_core::RuntimeFingerprintStatus::Unavailable
    );
}

#[test]
fn missing_machine_local_path_inputs_prevent_canonical_fingerprint() {
    let mut input = base_input();
    input.managed_self_assembly_path = RuntimeFactInput::missing();
    input.native_payload_path = RuntimeFactInput::missing();

    let report = build_runtime_context_report(input);

    assert_eq!(
        report
            .context
            .machine_local_path_identities
            .iter()
            .map(|path| path.status)
            .collect::<Vec<_>>(),
        vec![
            RuntimeFactStatus::Unavailable,
            RuntimeFactStatus::Unavailable
        ]
    );
    assert!(report
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::MachineLocalPathIdentity));
    assert_eq!(
        report.fingerprint.status,
        RuntimeFingerprintStatus::Unavailable
    );
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::MachineLocalPathIdentity
        && line.status == RuntimeFactStatus::Unavailable));
}

#[test]
fn relative_machine_local_path_inputs_are_unsupported_and_prevent_canonical_fingerprint() {
    let mut input = base_input();
    input.managed_self_assembly_path = RuntimeFactInput::observed(
        "managed/RustyStartup.Managed.dll",
        "StartupEntryOrchestrator.ManagedAssemblyPath",
    );
    input.native_payload_path = RuntimeFactInput::observed(
        "native/win-x64/rustystartup_core.dll",
        "SelfPackageLayoutResolver.NativePayloadPath",
    );

    let report = build_runtime_context_report(input);

    assert_eq!(
        report
            .context
            .machine_local_path_identities
            .iter()
            .map(|path| path.status)
            .collect::<Vec<_>>(),
        vec![
            RuntimeFactStatus::Unsupported,
            RuntimeFactStatus::Unsupported
        ]
    );
    assert!(report
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::MachineLocalPathIdentity));
    assert_eq!(
        report.fingerprint.status,
        RuntimeFingerprintStatus::Unavailable
    );
    assert!(report.diagnostics.iter().any(|line| line.class
        == RuntimeInputFingerprintClass::MachineLocalPathIdentity
        && line.status == RuntimeFactStatus::Unsupported));
}

#[test]
fn ffi_bootstrap_round_trip_reports_runtime_context() {
    let (_keepalive, ffi_input) = canonical_ffi_input();
    let report_from_ffi = build_runtime_context_report_from_ffi(&ffi_input);

    assert_eq!(
        report_from_ffi.context.parser_mode.value.as_deref(),
        Some("default-new")
    );
    assert_eq!(
        report_from_ffi
            .context
            .selected_language_key
            .value
            .as_deref(),
        Some("en-US")
    );
    assert_eq!(
        report_from_ffi.fingerprint.status,
        RuntimeFingerprintStatus::Canonical
    );
    assert!(report_from_ffi
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::MachineLocalPathIdentity));

    let rendered_pointer = unsafe { rs_bootstrap_submit_runtime_context(&ffi_input) };
    assert!(!rendered_pointer.is_null());

    let rendered = unsafe {
        CStr::from_ptr(rendered_pointer)
            .to_str()
            .expect("FFI bootstrap report should be valid UTF-8")
            .to_owned()
    };
    unsafe {
        rs_bootstrap_free_c_string(rendered_pointer);
    }

    assert!(rendered.contains("fingerprint_status=canonical"));
    assert!(rendered.contains(&format!(
        "fingerprint_hex={}",
        report_from_ffi.fingerprint.fingerprint_hex
    )));
    assert!(rendered.contains("classes_present=runtime_build_identity,runtime_version_basis,parser_mode,selected_language_key,platform_basis,bootstrap_self_package_inputs,machine_local_path_identity"));
}

fn expected_windows_or_unix_path(windows: &str, unix: &str) -> &'static str {
    if cfg!(windows) {
        Box::leak(windows.to_string().into_boxed_str())
    } else {
        Box::leak(unix.to_string().into_boxed_str())
    }
}

fn absolute_path(path: &str) -> String {
    path.to_string()
}

fn canonical_ffi_input() -> (Vec<CString>, RuntimeContextBootstrapInput) {
    let mut keepalive = Vec::new();
    let input = RuntimeContextBootstrapInput {
        runtime_build_identity_fresh_value: encode_optional(
            &mut keepalive,
            Some("RimWorld 1.6.4633 rev1261"),
        ),
        runtime_build_identity_fresh_source: encode_optional(&mut keepalive, Some("runtime_banner")),
        runtime_build_identity_static_value: encode_optional(
            &mut keepalive,
            Some("RimWorld 1.6.4630 rev1260"),
        ),
        runtime_build_identity_static_source: encode_optional(&mut keepalive, Some("Version.txt")),
        runtime_version_basis_fresh_value: encode_optional(&mut keepalive, Some("rev1261")),
        runtime_version_basis_fresh_source: encode_optional(&mut keepalive, Some("runtime_banner")),
        runtime_version_basis_static_value: encode_optional(&mut keepalive, Some("rev1260")),
        runtime_version_basis_static_source: encode_optional(&mut keepalive, Some("Version.txt")),
        parser_mode_value: encode_optional(&mut keepalive, Some("default-new")),
        parser_mode_source: encode_optional(&mut keepalive, Some("command_line")),
        selected_language_key_value: encode_optional(&mut keepalive, Some("en-US")),
        selected_language_key_source: encode_optional(
            &mut keepalive,
            Some("CultureInfo.CurrentUICulture.Name"),
        ),
        platform_value: encode_optional(&mut keepalive, Some("Windows")),
        platform_source: encode_optional(&mut keepalive, Some("RuntimeInformation")),
        operating_system_value: encode_optional(&mut keepalive, Some("Windows 11")),
        operating_system_source: encode_optional(
            &mut keepalive,
            Some("RuntimeInformation.OSDescription"),
        ),
        architecture_value: encode_optional(&mut keepalive, Some("x64")),
        architecture_source: encode_optional(
            &mut keepalive,
            Some("RuntimeInformation.OSArchitecture"),
        ),
        selected_self_package_root_value: encode_optional(
            &mut keepalive,
            Some("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\"),
        ),
        selected_self_package_root_source: encode_optional(
            &mut keepalive,
            Some("SelfPackageLayoutResolver.PackageRoot"),
        ),
        active_self_content_root_value: encode_optional(
            &mut keepalive,
            Some("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\"),
        ),
        active_self_content_root_source: encode_optional(
            &mut keepalive,
            Some("SelfPackageLayoutResolver.SelectedActiveContentRoot"),
        ),
        managed_self_assembly_path_value: encode_optional(
            &mut keepalive,
            Some("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Assemblies\\RustyStartup.Managed.dll"),
        ),
        managed_self_assembly_path_source: encode_optional(
            &mut keepalive,
            Some("StartupEntryOrchestrator.ManagedAssemblyPath"),
        ),
        native_payload_path_value: encode_optional(
            &mut keepalive,
            Some("C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Native\\win-x64\\rustystartup_core.dll"),
        ),
        native_payload_path_source: encode_optional(
            &mut keepalive,
            Some("SelfPackageLayoutResolver.NativePayloadPath"),
        ),
    };

    (keepalive, input)
}

fn encode_optional(keepalive: &mut Vec<CString>, value: Option<&str>) -> *const c_char {
    match value {
        Some(value) => {
            let owned = CString::new(value).expect("test data should not contain NUL");
            let pointer = owned.as_ptr();
            keepalive.push(owned);
            pointer
        }
        None => ptr::null(),
    }
}
