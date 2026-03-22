use crate::package_resolution::{resolve_package_discovery, PackageDiscoveryReport};
use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PackageDiscoveryBootstrapInput {
    pub self_package_root_value: *const c_char,
    pub self_package_root_source: *const c_char,
    pub official_data_root_value: *const c_char,
    pub official_data_root_source: *const c_char,
    pub local_mods_root_value: *const c_char,
    pub local_mods_root_source: *const c_char,
    pub workshop_content_root_value: *const c_char,
    pub workshop_content_root_source: *const c_char,
    pub historical_stale_package_id_value: *const c_char,
    pub historical_stale_package_id_source: *const c_char,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageDiscoveryTextInput {
    pub value: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageDiscoveryInput {
    pub self_package_root: PackageDiscoveryTextInput,
    pub official_data_root: PackageDiscoveryTextInput,
    pub local_mods_root: PackageDiscoveryTextInput,
    pub workshop_content_root: PackageDiscoveryTextInput,
    pub historical_stale_package_id: PackageDiscoveryTextInput,
}

pub fn build_package_discovery_report(input: PackageDiscoveryInput) -> PackageDiscoveryReport {
    resolve_package_discovery(input)
}

pub fn build_package_discovery_report_from_ffi(
    input: *const PackageDiscoveryBootstrapInput,
) -> PackageDiscoveryReport {
    let input = unsafe {
        match input.as_ref() {
            Some(value) => value.to_package_discovery_input(),
            None => PackageDiscoveryInput::default(),
        }
    };

    build_package_discovery_report(input)
}

impl PackageDiscoveryBootstrapInput {
    unsafe fn to_package_discovery_input(&self) -> PackageDiscoveryInput {
        PackageDiscoveryInput {
            self_package_root: PackageDiscoveryTextInput {
                value: read_optional_c_string(self.self_package_root_value),
                source: read_optional_c_string(self.self_package_root_source),
            },
            official_data_root: PackageDiscoveryTextInput {
                value: read_optional_c_string(self.official_data_root_value),
                source: read_optional_c_string(self.official_data_root_source),
            },
            local_mods_root: PackageDiscoveryTextInput {
                value: read_optional_c_string(self.local_mods_root_value),
                source: read_optional_c_string(self.local_mods_root_source),
            },
            workshop_content_root: PackageDiscoveryTextInput {
                value: read_optional_c_string(self.workshop_content_root_value),
                source: read_optional_c_string(self.workshop_content_root_source),
            },
            historical_stale_package_id: PackageDiscoveryTextInput {
                value: read_optional_c_string(self.historical_stale_package_id_value),
                source: read_optional_c_string(self.historical_stale_package_id_source),
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
