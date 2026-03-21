use rustystartup_core::{
    build_runtime_context_report, compute_runtime_context_fingerprint, RuntimeAuthorityPairInput,
    RuntimeContextInput, RuntimeFactInput, RuntimeFingerprintStatus, RuntimeInputFingerprintClass,
    RuntimePlatformBasisInput,
};

fn context_with_path_variation(
    selected_self_package_root: &str,
    active_self_content_root: &str,
) -> RuntimeContextInput {
    RuntimeContextInput {
        runtime_build_identity: RuntimeAuthorityPairInput {
            fresh: RuntimeFactInput::observed("RimWorld 1.6.4633 rev1261", "runtime_banner"),
            static_install_metadata: RuntimeFactInput::observed("RimWorld 1.6.4630 rev1260", "Version.txt"),
        },
        runtime_version_basis: RuntimeAuthorityPairInput {
            fresh: RuntimeFactInput::observed("rev1261", "runtime_banner"),
            static_install_metadata: RuntimeFactInput::observed("rev1260", "Version.txt"),
        },
        parser_mode: RuntimeFactInput::observed("default-new", "command_line"),
        selected_language_key: RuntimeFactInput::observed("en-US", "CultureInfo.CurrentUICulture.Name"),
        platform_basis: RuntimePlatformBasisInput {
            platform: RuntimeFactInput::observed("Windows", "RuntimeInformation"),
            operating_system: RuntimeFactInput::observed("Windows 11", "RuntimeInformation.OSDescription"),
            architecture: RuntimeFactInput::observed("x64", "RuntimeInformation.OSArchitecture"),
        },
        selected_self_package_root: RuntimeFactInput::observed(selected_self_package_root, "SelfPackageLayoutResolver.PackageRoot"),
        active_self_content_root: RuntimeFactInput::observed(active_self_content_root, "SelfPackageLayoutResolver.SelectedActiveContentRoot"),
        managed_self_assembly_path: RuntimeFactInput::observed(
            "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Assemblies\\RustyStartup.Managed.dll",
            "StartupEntryOrchestrator.ManagedAssemblyPath",
        ),
        native_payload_path: RuntimeFactInput::observed(
            "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\Native\\win-x64\\rustystartup_core.dll",
            "SelfPackageLayoutResolver.NativePayloadPath",
        ),
    }
}

#[test]
fn fingerprint_changes_when_machine_local_path_inputs_change() {
    let first = build_runtime_context_report(context_with_path_variation(
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\",
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\",
    ));
    let second = build_runtime_context_report(context_with_path_variation(
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core.alt\\",
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core.alt\\1.6\\",
    ));

    assert_ne!(
        first.fingerprint.fingerprint_hex,
        second.fingerprint.fingerprint_hex
    );
}

#[test]
fn fingerprint_classes_keep_bootstrap_inputs_distinct_from_machine_local_paths() {
    let report = build_runtime_context_report(context_with_path_variation(
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\",
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\",
    ));

    assert!(report
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::BootstrapSelfPackageInputs));
    assert!(report
        .fingerprint
        .classes_present
        .contains(&RuntimeInputFingerprintClass::MachineLocalPathIdentity));
    assert_eq!(
        report.fingerprint.status,
        RuntimeFingerprintStatus::Canonical
    );
}

#[test]
fn fingerprint_ignores_source_labels_and_reason_text() {
    let report = build_runtime_context_report(context_with_path_variation(
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\",
        "C:\\Users\\lukep\\source\\repos\\Rusty Startup\\rustystartup.core\\1.6\\",
    ));

    let baseline_context = report.context.clone();
    let baseline_fingerprint = compute_runtime_context_fingerprint(&baseline_context);

    let mut variant_context = baseline_context.clone();
    variant_context.runtime_build_identity.primary.source = "other_build_source".to_string();
    variant_context.runtime_build_identity.primary.reason =
        "alternate build diagnostic prose".to_string();
    if let Some(secondary) = variant_context
        .runtime_build_identity
        .secondary_authority
        .as_mut()
    {
        secondary.source = "other_static_source".to_string();
        secondary.reason = "alternate secondary diagnostic prose".to_string();
    }
    variant_context.runtime_version_basis.primary.source = "other_version_source".to_string();
    variant_context.runtime_version_basis.primary.reason =
        "alternate version diagnostic prose".to_string();
    variant_context.parser_mode.source = "other_parser_source".to_string();
    variant_context.parser_mode.reason = "alternate parser diagnostic prose".to_string();
    variant_context.selected_language_key.source = "other_language_source".to_string();
    variant_context.selected_language_key.reason =
        "alternate language diagnostic prose".to_string();
    variant_context.platform_basis.platform.source = "other_platform_source".to_string();
    variant_context.platform_basis.platform.reason =
        "alternate platform diagnostic prose".to_string();
    variant_context.selected_self_package_root.source = "other_package_source".to_string();
    variant_context.selected_self_package_root.reason =
        "alternate package diagnostic prose".to_string();
    variant_context.active_self_content_root.source = "other_content_source".to_string();
    variant_context.active_self_content_root.reason =
        "alternate content diagnostic prose".to_string();
    variant_context.machine_local_path_identities[0].source = "other_managed_source".to_string();
    variant_context.machine_local_path_identities[0].reason =
        "alternate managed path diagnostic prose".to_string();
    variant_context.machine_local_path_identities[1].source = "other_native_source".to_string();
    variant_context.machine_local_path_identities[1].reason =
        "alternate native path diagnostic prose".to_string();

    let variant_fingerprint = compute_runtime_context_fingerprint(&variant_context);

    assert_eq!(
        baseline_fingerprint.fingerprint_hex,
        variant_fingerprint.fingerprint_hex
    );
    assert_eq!(baseline_fingerprint.status, variant_fingerprint.status);
    assert_eq!(
        baseline_fingerprint.classes_present,
        variant_fingerprint.classes_present
    );
}
