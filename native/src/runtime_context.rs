use crate::runtime_input_fingerprint::{
    compute_runtime_context_fingerprint, RuntimeContextFingerprint, RuntimeInputFingerprintClass,
};
use std::borrow::Borrow;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Component, Path};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RuntimeContextBootstrapInput {
    pub runtime_build_identity_fresh_value: *const c_char,
    pub runtime_build_identity_fresh_source: *const c_char,
    pub runtime_build_identity_static_value: *const c_char,
    pub runtime_build_identity_static_source: *const c_char,
    pub runtime_version_basis_fresh_value: *const c_char,
    pub runtime_version_basis_fresh_source: *const c_char,
    pub runtime_version_basis_static_value: *const c_char,
    pub runtime_version_basis_static_source: *const c_char,
    pub parser_mode_value: *const c_char,
    pub parser_mode_source: *const c_char,
    pub selected_language_key_value: *const c_char,
    pub selected_language_key_source: *const c_char,
    pub platform_value: *const c_char,
    pub platform_source: *const c_char,
    pub operating_system_value: *const c_char,
    pub operating_system_source: *const c_char,
    pub architecture_value: *const c_char,
    pub architecture_source: *const c_char,
    pub selected_self_package_root_value: *const c_char,
    pub selected_self_package_root_source: *const c_char,
    pub active_self_content_root_value: *const c_char,
    pub active_self_content_root_source: *const c_char,
    pub managed_self_assembly_path_value: *const c_char,
    pub managed_self_assembly_path_source: *const c_char,
    pub native_payload_path_value: *const c_char,
    pub native_payload_path_source: *const c_char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeFactStatus {
    Observed,
    SecondaryAuthority,
    Degraded,
    Unknown,
    Unavailable,
    Unsupported,
}

impl RuntimeFactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::SecondaryAuthority => "secondary_authority",
            Self::Degraded => "degraded",
            Self::Unknown => "unknown",
            Self::Unavailable => "unavailable",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeFactInput {
    pub value: Option<String>,
    pub source: Option<String>,
}

impl RuntimeFactInput {
    pub fn observed(value: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            value: Some(value.into()),
            source: Some(source.into()),
        }
    }

    pub fn missing() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeAuthorityPairInput {
    pub fresh: RuntimeFactInput,
    pub static_install_metadata: RuntimeFactInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimePlatformBasisInput {
    pub platform: RuntimeFactInput,
    pub operating_system: RuntimeFactInput,
    pub architecture: RuntimeFactInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuntimeContextInput {
    pub runtime_build_identity: RuntimeAuthorityPairInput,
    pub runtime_version_basis: RuntimeAuthorityPairInput,
    pub parser_mode: RuntimeFactInput,
    pub selected_language_key: RuntimeFactInput,
    pub platform_basis: RuntimePlatformBasisInput,
    pub selected_self_package_root: RuntimeFactInput,
    pub active_self_content_root: RuntimeFactInput,
    pub managed_self_assembly_path: RuntimeFactInput,
    pub native_payload_path: RuntimeFactInput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeNormalizedFact {
    pub class: RuntimeInputFingerprintClass,
    pub value: Option<String>,
    pub source: String,
    pub status: RuntimeFactStatus,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeAuthorityPairNormalized {
    pub class: RuntimeInputFingerprintClass,
    pub primary: RuntimeNormalizedFact,
    pub secondary_authority: Option<RuntimeNormalizedFact>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePathIdentity {
    pub class: RuntimeInputFingerprintClass,
    pub raw_value: Option<String>,
    pub normalized_value: Option<String>,
    pub source: String,
    pub status: RuntimeFactStatus,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePlatformBasisNormalized {
    pub platform: RuntimeNormalizedFact,
    pub operating_system: RuntimeNormalizedFact,
    pub architecture: RuntimeNormalizedFact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeDiagnosticLine {
    pub class: RuntimeInputFingerprintClass,
    pub status: RuntimeFactStatus,
    pub value: Option<String>,
    pub normalized_value: Option<String>,
    pub source: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext {
    pub runtime_build_identity: RuntimeAuthorityPairNormalized,
    pub runtime_version_basis: RuntimeAuthorityPairNormalized,
    pub parser_mode: RuntimeNormalizedFact,
    pub selected_language_key: RuntimeNormalizedFact,
    pub platform_basis: RuntimePlatformBasisNormalized,
    pub selected_self_package_root: RuntimePathIdentity,
    pub active_self_content_root: RuntimePathIdentity,
    pub machine_local_path_identities: Vec<RuntimePathIdentity>,
}

impl RuntimeContext {
    pub fn classes_present(&self) -> Vec<RuntimeInputFingerprintClass> {
        let mut classes = vec![
            RuntimeInputFingerprintClass::RuntimeBuildIdentity,
            RuntimeInputFingerprintClass::RuntimeVersionBasis,
            RuntimeInputFingerprintClass::ParserMode,
            RuntimeInputFingerprintClass::SelectedLanguageKey,
            RuntimeInputFingerprintClass::PlatformBasis,
        ];

        if self.has_self_package_inputs() {
            classes.push(RuntimeInputFingerprintClass::BootstrapSelfPackageInputs);
        }

        if self.has_machine_local_path_inputs() {
            classes.push(RuntimeInputFingerprintClass::MachineLocalPathIdentity);
        }

        classes
    }

    pub fn has_self_package_inputs(&self) -> bool {
        is_positive_fact(&self.selected_self_package_root)
            && is_positive_fact(&self.active_self_content_root)
    }

    pub fn has_machine_local_path_inputs(&self) -> bool {
        !self.machine_local_path_identities.is_empty()
    }

    pub fn has_degraded_inputs(&self) -> bool {
        self.runtime_build_identity.primary.status == RuntimeFactStatus::SecondaryAuthority
            || self.runtime_build_identity.primary.status == RuntimeFactStatus::Degraded
            || self.runtime_version_basis.primary.status == RuntimeFactStatus::SecondaryAuthority
            || self.runtime_version_basis.primary.status == RuntimeFactStatus::Degraded
            || self.parser_mode.status == RuntimeFactStatus::Degraded
            || self.selected_language_key.status == RuntimeFactStatus::Degraded
            || self.platform_basis.platform.status == RuntimeFactStatus::Degraded
            || self.platform_basis.operating_system.status == RuntimeFactStatus::Degraded
            || self.platform_basis.architecture.status == RuntimeFactStatus::Degraded
            || self.selected_self_package_root.status == RuntimeFactStatus::Degraded
            || self.active_self_content_root.status == RuntimeFactStatus::Degraded
            || self.machine_local_path_identities.iter().any(|fact| {
                fact.status == RuntimeFactStatus::Degraded
                    || fact.status == RuntimeFactStatus::SecondaryAuthority
            })
    }

    pub fn has_critical_unavailable_inputs(&self) -> bool {
        self.runtime_build_identity.primary.status == RuntimeFactStatus::Unavailable
            || self.runtime_version_basis.primary.status == RuntimeFactStatus::Unavailable
            || self.parser_mode.status == RuntimeFactStatus::Unavailable
            || self.selected_language_key.status == RuntimeFactStatus::Unavailable
            || self.platform_basis.platform.status == RuntimeFactStatus::Unavailable
            || self.platform_basis.operating_system.status == RuntimeFactStatus::Unavailable
            || self.platform_basis.architecture.status == RuntimeFactStatus::Unavailable
            || self.selected_self_package_root.status == RuntimeFactStatus::Unavailable
            || self.active_self_content_root.status == RuntimeFactStatus::Unavailable
            || self
                .machine_local_path_identities
                .iter()
                .any(|fact| fact.status == RuntimeFactStatus::Unavailable)
    }

    pub fn has_critical_unsupported_inputs(&self) -> bool {
        self.runtime_build_identity.primary.status == RuntimeFactStatus::Unsupported
            || self.runtime_version_basis.primary.status == RuntimeFactStatus::Unsupported
            || self.parser_mode.status == RuntimeFactStatus::Unsupported
            || self.selected_language_key.status == RuntimeFactStatus::Unsupported
            || self.platform_basis.platform.status == RuntimeFactStatus::Unsupported
            || self.platform_basis.operating_system.status == RuntimeFactStatus::Unsupported
            || self.platform_basis.architecture.status == RuntimeFactStatus::Unsupported
            || self.selected_self_package_root.status == RuntimeFactStatus::Unsupported
            || self.active_self_content_root.status == RuntimeFactStatus::Unsupported
            || self
                .machine_local_path_identities
                .iter()
                .any(|fact| fact.status == RuntimeFactStatus::Unsupported)
    }

    pub fn first_critical_unavailable_reason(&self) -> Option<String> {
        first_reason_from_text_facts(
            &[
                &self.runtime_build_identity.primary,
                &self.runtime_version_basis.primary,
                &self.parser_mode,
                &self.selected_language_key,
                &self.platform_basis.platform,
                &self.platform_basis.operating_system,
                &self.platform_basis.architecture,
            ],
            RuntimeFactStatus::Unavailable,
        )
        .or_else(|| {
            first_reason_from_path_facts(
                [
                    &self.selected_self_package_root,
                    &self.active_self_content_root,
                ]
                .iter()
                .copied(),
                RuntimeFactStatus::Unavailable,
            )
        })
        .or_else(|| {
            first_reason_from_path_facts(
                &self.machine_local_path_identities,
                RuntimeFactStatus::Unavailable,
            )
        })
    }

    pub fn first_critical_unsupported_reason(&self) -> Option<String> {
        first_reason_from_text_facts(
            &[
                &self.runtime_build_identity.primary,
                &self.runtime_version_basis.primary,
                &self.parser_mode,
                &self.selected_language_key,
                &self.platform_basis.platform,
                &self.platform_basis.operating_system,
                &self.platform_basis.architecture,
            ],
            RuntimeFactStatus::Unsupported,
        )
        .or_else(|| {
            first_reason_from_path_facts(
                [
                    &self.selected_self_package_root,
                    &self.active_self_content_root,
                ]
                .iter()
                .copied(),
                RuntimeFactStatus::Unsupported,
            )
        })
        .or_else(|| {
            first_reason_from_path_facts(
                &self.machine_local_path_identities,
                RuntimeFactStatus::Unsupported,
            )
        })
    }

    pub fn first_degraded_reason(&self) -> Option<String> {
        first_reason_from_text_facts(
            &[
                &self.runtime_build_identity.primary,
                &self.runtime_version_basis.primary,
                &self.parser_mode,
                &self.selected_language_key,
                &self.platform_basis.platform,
                &self.platform_basis.operating_system,
                &self.platform_basis.architecture,
            ],
            RuntimeFactStatus::Degraded,
        )
        .or_else(|| {
            first_reason_from_path_facts(
                [
                    &self.selected_self_package_root,
                    &self.active_self_content_root,
                ]
                .iter()
                .copied(),
                RuntimeFactStatus::Degraded,
            )
        })
        .or_else(|| {
            self.runtime_build_identity
                .secondary_authority
                .as_ref()
                .map(|fact| fact.reason.clone())
        })
        .or_else(|| {
            self.runtime_build_identity
                .primary
                .status
                .eq(&RuntimeFactStatus::SecondaryAuthority)
                .then(|| self.runtime_build_identity.primary.reason.clone())
        })
        .or_else(|| {
            self.runtime_version_basis
                .primary
                .status
                .eq(&RuntimeFactStatus::SecondaryAuthority)
                .then(|| self.runtime_version_basis.primary.reason.clone())
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContextReport {
    pub context: RuntimeContext,
    pub fingerprint: RuntimeContextFingerprint,
    pub diagnostics: Vec<RuntimeDiagnosticLine>,
}

pub fn build_runtime_context_report_from_ffi(
    input: *const RuntimeContextBootstrapInput,
) -> RuntimeContextReport {
    let input = unsafe {
        match input.as_ref() {
            Some(value) => value.to_runtime_context_input(),
            None => RuntimeContextInput::default(),
        }
    };

    build_runtime_context_report(input)
}

pub fn build_runtime_context_report(input: RuntimeContextInput) -> RuntimeContextReport {
    let runtime_build_identity = normalize_authority_pair(
        RuntimeInputFingerprintClass::RuntimeBuildIdentity,
        &input.runtime_build_identity,
        "fresh runtime evidence outranks static install metadata",
    );
    let runtime_version_basis = normalize_authority_pair(
        RuntimeInputFingerprintClass::RuntimeVersionBasis,
        &input.runtime_version_basis,
        "fresh runtime evidence outranks static install metadata",
    );
    let parser_mode = normalize_parser_mode(&input.parser_mode);
    let selected_language_key = normalize_text_fact(
        RuntimeInputFingerprintClass::SelectedLanguageKey,
        &input.selected_language_key,
        RuntimeFactStatus::Unavailable,
        "selected language key was not provided",
        "selected_language_key",
    );
    let platform = normalize_text_fact(
        RuntimeInputFingerprintClass::PlatformBasis,
        &input.platform_basis.platform,
        RuntimeFactStatus::Unavailable,
        "platform basis was not provided",
        "platform",
    );
    let operating_system = normalize_text_fact(
        RuntimeInputFingerprintClass::PlatformBasis,
        &input.platform_basis.operating_system,
        RuntimeFactStatus::Unavailable,
        "operating system basis was not provided",
        "operating_system",
    );
    let architecture = normalize_text_fact(
        RuntimeInputFingerprintClass::PlatformBasis,
        &input.platform_basis.architecture,
        RuntimeFactStatus::Unavailable,
        "architecture basis was not provided",
        "architecture",
    );
    let selected_self_package_root = normalize_path_identity(
        RuntimeInputFingerprintClass::BootstrapSelfPackageInputs,
        &input.selected_self_package_root,
        "selected self-package root",
        true,
    );
    let active_self_content_root = normalize_path_identity(
        RuntimeInputFingerprintClass::BootstrapSelfPackageInputs,
        &input.active_self_content_root,
        "active self content root",
        true,
    );
    let managed_self_assembly_path = normalize_path_identity(
        RuntimeInputFingerprintClass::MachineLocalPathIdentity,
        &input.managed_self_assembly_path,
        "managed self assembly path",
        true,
    );
    let native_payload_path = normalize_path_identity(
        RuntimeInputFingerprintClass::MachineLocalPathIdentity,
        &input.native_payload_path,
        "native payload path",
        true,
    );

    let context = RuntimeContext {
        runtime_build_identity,
        runtime_version_basis,
        parser_mode,
        selected_language_key,
        platform_basis: RuntimePlatformBasisNormalized {
            platform,
            operating_system,
            architecture,
        },
        selected_self_package_root,
        active_self_content_root,
        machine_local_path_identities: vec![managed_self_assembly_path, native_payload_path],
    };

    let mut diagnostics = Vec::new();
    push_authority_pair_diagnostics(&mut diagnostics, &context.runtime_build_identity);
    push_authority_pair_diagnostics(&mut diagnostics, &context.runtime_version_basis);
    push_fact_diagnostic(&mut diagnostics, &context.parser_mode);
    push_fact_diagnostic(&mut diagnostics, &context.selected_language_key);
    push_fact_diagnostic(&mut diagnostics, &context.platform_basis.platform);
    push_fact_diagnostic(&mut diagnostics, &context.platform_basis.operating_system);
    push_fact_diagnostic(&mut diagnostics, &context.platform_basis.architecture);
    push_path_diagnostic(&mut diagnostics, &context.selected_self_package_root);
    push_path_diagnostic(&mut diagnostics, &context.active_self_content_root);
    for path in &context.machine_local_path_identities {
        push_path_diagnostic(&mut diagnostics, path);
    }

    let fingerprint = compute_runtime_context_fingerprint(&context);

    RuntimeContextReport {
        context,
        fingerprint,
        diagnostics,
    }
}

fn normalize_authority_pair(
    class: RuntimeInputFingerprintClass,
    input: &RuntimeAuthorityPairInput,
    secondary_reason: &str,
) -> RuntimeAuthorityPairNormalized {
    let fresh = normalize_text_fact(
        class,
        &input.fresh,
        RuntimeFactStatus::Unavailable,
        "fresh runtime evidence was not provided",
        "fresh_runtime_evidence",
    );
    let static_install = normalize_text_fact(
        class,
        &input.static_install_metadata,
        RuntimeFactStatus::Unavailable,
        "static install metadata was not provided",
        "static_install_metadata",
    );

    if fresh.status == RuntimeFactStatus::Observed {
        RuntimeAuthorityPairNormalized {
            class,
            primary: fresh,
            secondary_authority: if static_install.status == RuntimeFactStatus::Observed {
                Some(RuntimeNormalizedFact {
                    class,
                    value: static_install.value,
                    source: static_install.source,
                    status: RuntimeFactStatus::SecondaryAuthority,
                    reason: secondary_reason.to_string(),
                })
            } else if static_install.status == RuntimeFactStatus::Unavailable {
                None
            } else {
                Some(RuntimeNormalizedFact {
                    class,
                    value: static_install.value,
                    source: static_install.source,
                    status: RuntimeFactStatus::SecondaryAuthority,
                    reason: secondary_reason.to_string(),
                })
            },
        }
    } else if static_install.status == RuntimeFactStatus::Observed {
        RuntimeAuthorityPairNormalized {
            class,
            primary: RuntimeNormalizedFact {
                class,
                value: static_install.value.clone(),
                source: static_install.source.clone(),
                status: RuntimeFactStatus::SecondaryAuthority,
                reason:
                    "static install metadata used because fresh runtime evidence was unavailable"
                        .to_string(),
            },
            secondary_authority: None,
        }
    } else {
        RuntimeAuthorityPairNormalized {
            class,
            primary: RuntimeNormalizedFact {
                class,
                value: None,
                source: "unspecified".to_string(),
                status: RuntimeFactStatus::Unavailable,
                reason: "neither fresh runtime evidence nor static install metadata was provided"
                    .to_string(),
            },
            secondary_authority: None,
        }
    }
}

fn normalize_text_fact(
    class: RuntimeInputFingerprintClass,
    input: &RuntimeFactInput,
    missing_status: RuntimeFactStatus,
    missing_reason: &str,
    source_label: &str,
) -> RuntimeNormalizedFact {
    let source = normalize_source(input.source.as_deref(), source_label);

    match input
        .value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) => RuntimeNormalizedFact {
            class,
            value: Some(value.to_string()),
            source,
            status: RuntimeFactStatus::Observed,
            reason: format!("{source_label} observed"),
        },
        None => RuntimeNormalizedFact {
            class,
            value: None,
            source,
            status: missing_status,
            reason: missing_reason.to_string(),
        },
    }
}

fn normalize_parser_mode(input: &RuntimeFactInput) -> RuntimeNormalizedFact {
    let source = normalize_source(input.source.as_deref(), "parser_mode_source");

    match input
        .value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        Some(value) if value.eq_ignore_ascii_case("legacy-xml-deserializer") => {
            RuntimeNormalizedFact {
                class: RuntimeInputFingerprintClass::ParserMode,
                value: Some("legacy-xml-deserializer".to_string()),
                source,
                status: RuntimeFactStatus::Degraded,
                reason: "legacy parser mode is explicitly non-primary parity".to_string(),
            }
        }
        Some(value)
            if value.eq_ignore_ascii_case("default")
                || value.eq_ignore_ascii_case("default-new")
                || value.eq_ignore_ascii_case("primary") =>
        {
            RuntimeNormalizedFact {
                class: RuntimeInputFingerprintClass::ParserMode,
                value: Some("default-new".to_string()),
                source,
                status: RuntimeFactStatus::Observed,
                reason: "default parser mode selected".to_string(),
            }
        }
        Some(value) => RuntimeNormalizedFact {
            class: RuntimeInputFingerprintClass::ParserMode,
            value: Some(value.to_string()),
            source,
            status: RuntimeFactStatus::Unsupported,
            reason: "parser mode was not recognized as a supported primary or legacy lane"
                .to_string(),
        },
        None => RuntimeNormalizedFact {
            class: RuntimeInputFingerprintClass::ParserMode,
            value: None,
            source,
            status: RuntimeFactStatus::Unavailable,
            reason: "parser mode was not provided".to_string(),
        },
    }
}

fn normalize_path_identity(
    class: RuntimeInputFingerprintClass,
    input: &RuntimeFactInput,
    source_label: &str,
    required_absolute: bool,
) -> RuntimePathIdentity {
    let source = normalize_source(input.source.as_deref(), source_label);
    let raw_value = input
        .value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string());

    match raw_value {
        Some(raw) => {
            let normalized = normalize_path_string(&raw);
            let is_absolute = Path::new(&raw).is_absolute();
            if required_absolute && !is_absolute {
                RuntimePathIdentity {
                    class,
                    raw_value: Some(raw),
                    normalized_value: Some(normalized),
                    source,
                    status: RuntimeFactStatus::Unsupported,
                    reason: format!("{source_label} must be an absolute path identity"),
                }
            } else {
                RuntimePathIdentity {
                    class,
                    raw_value: Some(raw),
                    normalized_value: Some(normalized),
                    source,
                    status: RuntimeFactStatus::Observed,
                    reason: format!("{source_label} normalized to stable path identity"),
                }
            }
        }
        None => RuntimePathIdentity {
            class,
            raw_value: None,
            normalized_value: None,
            source,
            status: RuntimeFactStatus::Unavailable,
            reason: format!("{source_label} was not provided"),
        },
    }
}

fn normalize_path_string(value: &str) -> String {
    let mut components = Vec::new();
    let mut prefix = None;
    let mut absolute = false;

    for component in Path::new(value).components() {
        match component {
            Component::Prefix(prefix_component) => {
                let mut normalized = prefix_component
                    .as_os_str()
                    .to_string_lossy()
                    .replace('\\', "/");
                if cfg!(windows) {
                    normalized = normalized.to_lowercase();
                }
                prefix = Some(normalized);
            }
            Component::RootDir => {
                absolute = true;
            }
            Component::CurDir => {}
            Component::ParentDir => {
                components.pop();
            }
            Component::Normal(part) => {
                components.push(part.to_string_lossy().replace('\\', "/"));
            }
        }
    }

    let mut normalized = String::new();
    if let Some(prefix) = prefix {
        normalized.push_str(&prefix);
        if absolute && !normalized.ends_with('/') {
            normalized.push('/');
        }
    } else if absolute {
        normalized.push('/');
    }

    normalized.push_str(&components.join("/"));

    while normalized.ends_with('/') && normalized.len() > 1 {
        normalized.pop();
    }

    if cfg!(windows) {
        normalized = normalized.replace('\\', "/").to_lowercase();
    }

    normalized
}

fn normalize_source(source: Option<&str>, fallback: &str) -> String {
    source
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| fallback.to_string())
}

fn push_authority_pair_diagnostics(
    diagnostics: &mut Vec<RuntimeDiagnosticLine>,
    pair: &RuntimeAuthorityPairNormalized,
) {
    push_fact_diagnostic(diagnostics, &pair.primary);
    if let Some(secondary) = &pair.secondary_authority {
        push_fact_diagnostic(diagnostics, secondary);
    }
}

fn push_fact_diagnostic(
    diagnostics: &mut Vec<RuntimeDiagnosticLine>,
    fact: &RuntimeNormalizedFact,
) {
    diagnostics.push(RuntimeDiagnosticLine {
        class: fact.class,
        status: fact.status,
        value: fact.value.clone(),
        normalized_value: None,
        source: fact.source.clone(),
        reason: fact.reason.clone(),
    });
}

fn push_path_diagnostic(diagnostics: &mut Vec<RuntimeDiagnosticLine>, fact: &RuntimePathIdentity) {
    diagnostics.push(RuntimeDiagnosticLine {
        class: fact.class,
        status: fact.status,
        value: fact.raw_value.clone(),
        normalized_value: fact.normalized_value.clone(),
        source: fact.source.clone(),
        reason: fact.reason.clone(),
    });
}

fn is_positive_fact(fact: &RuntimePathIdentity) -> bool {
    matches!(
        fact.status,
        RuntimeFactStatus::Observed | RuntimeFactStatus::SecondaryAuthority
    )
}

fn first_reason_from_text_facts(
    facts: &[&RuntimeNormalizedFact],
    status: RuntimeFactStatus,
) -> Option<String> {
    facts
        .iter()
        .find(|fact| fact.status == status)
        .map(|fact| fact.reason.clone())
}

fn first_reason_from_path_facts<I, T>(facts: I, status: RuntimeFactStatus) -> Option<String>
where
    I: IntoIterator<Item = T>,
    T: Borrow<RuntimePathIdentity>,
{
    facts
        .into_iter()
        .find(|fact| fact.borrow().status == status)
        .map(|fact| fact.borrow().reason.clone())
}

impl RuntimeContextBootstrapInput {
    unsafe fn to_runtime_context_input(&self) -> RuntimeContextInput {
        RuntimeContextInput {
            runtime_build_identity: RuntimeAuthorityPairInput {
                fresh: RuntimeFactInput {
                    value: read_optional_c_string(self.runtime_build_identity_fresh_value),
                    source: read_optional_c_string(self.runtime_build_identity_fresh_source),
                },
                static_install_metadata: RuntimeFactInput {
                    value: read_optional_c_string(self.runtime_build_identity_static_value),
                    source: read_optional_c_string(self.runtime_build_identity_static_source),
                },
            },
            runtime_version_basis: RuntimeAuthorityPairInput {
                fresh: RuntimeFactInput {
                    value: read_optional_c_string(self.runtime_version_basis_fresh_value),
                    source: read_optional_c_string(self.runtime_version_basis_fresh_source),
                },
                static_install_metadata: RuntimeFactInput {
                    value: read_optional_c_string(self.runtime_version_basis_static_value),
                    source: read_optional_c_string(self.runtime_version_basis_static_source),
                },
            },
            parser_mode: RuntimeFactInput {
                value: read_optional_c_string(self.parser_mode_value),
                source: read_optional_c_string(self.parser_mode_source),
            },
            selected_language_key: RuntimeFactInput {
                value: read_optional_c_string(self.selected_language_key_value),
                source: read_optional_c_string(self.selected_language_key_source),
            },
            platform_basis: RuntimePlatformBasisInput {
                platform: RuntimeFactInput {
                    value: read_optional_c_string(self.platform_value),
                    source: read_optional_c_string(self.platform_source),
                },
                operating_system: RuntimeFactInput {
                    value: read_optional_c_string(self.operating_system_value),
                    source: read_optional_c_string(self.operating_system_source),
                },
                architecture: RuntimeFactInput {
                    value: read_optional_c_string(self.architecture_value),
                    source: read_optional_c_string(self.architecture_source),
                },
            },
            selected_self_package_root: RuntimeFactInput {
                value: read_optional_c_string(self.selected_self_package_root_value),
                source: read_optional_c_string(self.selected_self_package_root_source),
            },
            active_self_content_root: RuntimeFactInput {
                value: read_optional_c_string(self.active_self_content_root_value),
                source: read_optional_c_string(self.active_self_content_root_source),
            },
            managed_self_assembly_path: RuntimeFactInput {
                value: read_optional_c_string(self.managed_self_assembly_path_value),
                source: read_optional_c_string(self.managed_self_assembly_path_source),
            },
            native_payload_path: RuntimeFactInput {
                value: read_optional_c_string(self.native_payload_path_value),
                source: read_optional_c_string(self.native_payload_path_source),
            },
        }
    }
}

unsafe fn read_optional_c_string(ptr: *const c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }

    CStr::from_ptr(ptr).to_str().ok().map(str::to_string)
}
