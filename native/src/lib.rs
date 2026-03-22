#![allow(non_snake_case)]

mod mod_identity;
mod package_discovery_contract;
mod package_resolution;
mod package_resolution_diagnostics;
mod runtime_context;
mod runtime_diagnostics;
mod runtime_input_fingerprint;

use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::atomic::{AtomicUsize, Ordering};

pub use mod_identity::{
    canonicalize_package_id, normalize_path_identity, ModIdentity, PackageOrigin,
    PackageSearchRootRole,
};
pub use package_discovery_contract::{
    build_package_discovery_report, build_package_discovery_report_from_ffi,
    PackageDiscoveryBootstrapInput, PackageDiscoveryInput, PackageDiscoveryTextInput,
};
pub use package_resolution::{
    resolve_package_discovery, PackageDiscoveryReport, PackageResolutionResult,
};
pub use package_resolution_diagnostics::render_package_discovery_report;
pub use runtime_context::{
    build_runtime_context_report, build_runtime_context_report_from_ffi, RuntimeAuthorityPairInput,
    RuntimeAuthorityPairNormalized, RuntimeContext, RuntimeContextBootstrapInput,
    RuntimeContextInput, RuntimeContextReport, RuntimeDiagnosticLine, RuntimeFactInput,
    RuntimeFactStatus, RuntimePathIdentity, RuntimePlatformBasisInput,
    RuntimePlatformBasisNormalized,
};
pub use runtime_diagnostics::render_runtime_context_report;
pub use runtime_input_fingerprint::{
    compute_runtime_context_fingerprint, RuntimeContextFingerprint, RuntimeFingerprintStatus,
    RuntimeInputFingerprintClass,
};

const ABI_VERSION: i32 = 2;
const CAPABILITIES: u64 = 0x7;

static ACTIVATION_COUNT: AtomicUsize = AtomicUsize::new(0);

#[no_mangle]
pub extern "C" fn rs_bootstrap_get_abi_version() -> i32 {
    ABI_VERSION
}

#[no_mangle]
pub extern "C" fn rs_bootstrap_get_capabilities() -> u64 {
    CAPABILITIES
}

#[no_mangle]
pub unsafe extern "C" fn rs_bootstrap_submit_package_discovery_basis(
    input: *const PackageDiscoveryBootstrapInput,
) -> *mut c_char {
    let report = package_discovery_contract::build_package_discovery_report_from_ffi(input);
    let rendered = render_package_discovery_report(&report);

    match CString::new(rendered) {
        Ok(value) => value.into_raw(),
        Err(_) => CString::new(
            "package_resolution_result=failed\npackage_resolution_reason_code=render_failed\npackage_resolution_reason=report rendering failed",
        )
        .expect("static package-discovery report is valid")
        .into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rs_bootstrap_submit_runtime_context(
    input: *const RuntimeContextBootstrapInput,
) -> *mut c_char {
    let report = runtime_context::build_runtime_context_report_from_ffi(input);
    let rendered = render_runtime_context_report(&report);

    match CString::new(rendered) {
        Ok(value) => value.into_raw(),
        Err(_) => CString::new(
            "fingerprint_status=unavailable\nfingerprint_reason=report rendering failed",
        )
        .expect("static report is valid")
        .into_raw(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn rs_bootstrap_free_c_string(value: *mut c_char) {
    if value.is_null() {
        return;
    }

    drop(CString::from_raw(value));
}

#[no_mangle]
pub extern "C" fn rs_bootstrap_activate() -> i32 {
    ACTIVATION_COUNT.fetch_add(1, Ordering::SeqCst);
    0
}
