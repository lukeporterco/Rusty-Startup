use crate::runtime_context::{
    RuntimeAuthorityPairNormalized, RuntimeContext, RuntimeNormalizedFact, RuntimePathIdentity,
    RuntimePlatformBasisNormalized,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RuntimeInputFingerprintClass {
    RuntimeBuildIdentity,
    RuntimeVersionBasis,
    ParserMode,
    SelectedLanguageKey,
    PlatformBasis,
    MachineLocalPathIdentity,
    BootstrapSelfPackageInputs,
}

impl RuntimeInputFingerprintClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeBuildIdentity => "runtime_build_identity",
            Self::RuntimeVersionBasis => "runtime_version_basis",
            Self::ParserMode => "parser_mode",
            Self::SelectedLanguageKey => "selected_language_key",
            Self::PlatformBasis => "platform_basis",
            Self::MachineLocalPathIdentity => "machine_local_path_identity",
            Self::BootstrapSelfPackageInputs => "bootstrap_self_package_inputs",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeFingerprintStatus {
    Canonical,
    Degraded,
    Unavailable,
}

impl RuntimeFingerprintStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::Degraded => "degraded",
            Self::Unavailable => "unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContextFingerprint {
    pub status: RuntimeFingerprintStatus,
    pub reason: String,
    pub fingerprint_hex: String,
    pub classes_present: Vec<RuntimeInputFingerprintClass>,
}

pub fn compute_runtime_context_fingerprint(context: &RuntimeContext) -> RuntimeContextFingerprint {
    let classes_present = context.classes_present();
    let mut hasher = StableFingerprintHasher::new();

    hasher.push_str("classes");
    for class in &classes_present {
        hasher.push_str(class.as_str());
    }

    push_authority_pair(
        &mut hasher,
        "runtime_build_identity",
        &context.runtime_build_identity,
    );
    push_authority_pair(
        &mut hasher,
        "runtime_version_basis",
        &context.runtime_version_basis,
    );
    push_fact(&mut hasher, "parser_mode", &context.parser_mode);
    push_fact(
        &mut hasher,
        "selected_language_key",
        &context.selected_language_key,
    );
    push_platform_basis(&mut hasher, &context.platform_basis);
    push_path_identity(
        &mut hasher,
        "selected_self_package_root",
        &context.selected_self_package_root,
    );
    push_path_identity(
        &mut hasher,
        "active_self_content_root",
        &context.active_self_content_root,
    );
    for path in &context.machine_local_path_identities {
        push_path_identity(&mut hasher, "machine_local_path_identity", path);
    }

    let (status, reason) = evaluate_fingerprint_status(context);

    RuntimeContextFingerprint {
        status,
        reason,
        fingerprint_hex: format!("{:016x}", hasher.finish()),
        classes_present,
    }
}

fn evaluate_fingerprint_status(context: &RuntimeContext) -> (RuntimeFingerprintStatus, String) {
    if context.has_critical_unavailable_inputs() {
        return (
            RuntimeFingerprintStatus::Unavailable,
            context
                .first_critical_unavailable_reason()
                .unwrap_or_else(|| "critical runtime inputs are unavailable".to_string()),
        );
    }

    if context.has_critical_unsupported_inputs() {
        return (
            RuntimeFingerprintStatus::Unavailable,
            context
                .first_critical_unsupported_reason()
                .unwrap_or_else(|| "critical runtime inputs are unsupported".to_string()),
        );
    }

    if context.has_degraded_inputs() {
        return (
            RuntimeFingerprintStatus::Degraded,
            context
                .first_degraded_reason()
                .unwrap_or_else(|| "runtime context contains degraded inputs".to_string()),
        );
    }

    (
        RuntimeFingerprintStatus::Canonical,
        "all required runtime inputs were normalized without degradation".to_string(),
    )
}

fn push_authority_pair(
    hasher: &mut StableFingerprintHasher,
    label: &str,
    pair: &RuntimeAuthorityPairNormalized,
) {
    hasher.push_str(label);
    push_fact(hasher, "primary", &pair.primary);
    if let Some(secondary) = &pair.secondary_authority {
        push_fact(hasher, "secondary", secondary);
    }
}

fn push_platform_basis(
    hasher: &mut StableFingerprintHasher,
    basis: &RuntimePlatformBasisNormalized,
) {
    hasher.push_str("platform_basis");
    push_fact(hasher, "platform", &basis.platform);
    push_fact(hasher, "operating_system", &basis.operating_system);
    push_fact(hasher, "architecture", &basis.architecture);
}

fn push_path_identity(
    hasher: &mut StableFingerprintHasher,
    label: &str,
    path: &RuntimePathIdentity,
) {
    hasher.push_str(label);
    hasher.push_str(path.class.as_str());
    hasher.push_str(path.status.as_str());
    if let Some(normalized) = &path.normalized_value {
        hasher.push_str(normalized);
    } else {
        hasher.push_str("<none>");
    }
}

fn push_fact(hasher: &mut StableFingerprintHasher, label: &str, fact: &RuntimeNormalizedFact) {
    hasher.push_str(label);
    hasher.push_str(fact.class.as_str());
    hasher.push_str(fact.status.as_str());
    if let Some(value) = &fact.value {
        hasher.push_str(value);
    } else {
        hasher.push_str("<none>");
    }
}

#[derive(Debug, Clone)]
struct StableFingerprintHasher {
    state: u64,
}

impl StableFingerprintHasher {
    fn new() -> Self {
        Self {
            state: 0xcbf2_9ce4_8422_2325,
        }
    }

    fn push_str(&mut self, value: &str) {
        for byte in value.as_bytes() {
            self.state ^= *byte as u64;
            self.state = self.state.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn finish(&self) -> u64 {
        self.state
    }
}
