use std::path::{Component, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageSearchRootRole {
    SelfPackage,
    OfficialData,
    LocalMods,
    WorkshopContent,
}

impl PackageSearchRootRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfPackage => "self",
            Self::OfficialData => "official",
            Self::LocalMods => "local",
            Self::WorkshopContent => "workshop",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PackageOrigin {
    SelfPackage,
    OfficialData,
    LocalMods,
    WorkshopContent,
    Ambiguous,
    Unknown,
}

impl PackageOrigin {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfPackage => "self",
            Self::OfficialData => "official",
            Self::LocalMods => "local",
            Self::WorkshopContent => "workshop",
            Self::Ambiguous => "ambiguous",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModIdentity {
    pub package_root: String,
    pub normalized_package_root: String,
    pub root_detection_source: String,
    pub provenance_roles: Vec<PackageSearchRootRole>,
    pub origin: PackageOrigin,
    pub package_id: Option<String>,
    pub package_id_canonical: Option<String>,
    pub display_name_basis: Option<String>,
    pub about_status: String,
    pub published_file_id: Option<String>,
    pub published_file_status: String,
    pub dependency_metadata_status: String,
    pub dependency_metadata_present: bool,
    pub load_before_present: bool,
    pub load_after_present: bool,
    pub package_identity_status: String,
    pub stale_package_id: Option<String>,
    pub stale_package_id_source: Option<String>,
}

impl ModIdentity {
    pub fn new(package_root: impl Into<String>, root_detection_source: impl Into<String>) -> Self {
        let package_root = package_root.into();
        Self {
            normalized_package_root: normalize_path_identity(&package_root),
            package_root,
            root_detection_source: root_detection_source.into(),
            provenance_roles: Vec::new(),
            origin: PackageOrigin::Unknown,
            package_id: None,
            package_id_canonical: None,
            display_name_basis: None,
            about_status: "missing".to_string(),
            published_file_id: None,
            published_file_status: "missing".to_string(),
            dependency_metadata_status: "missing".to_string(),
            dependency_metadata_present: false,
            load_before_present: false,
            load_after_present: false,
            package_identity_status: "missing".to_string(),
            stale_package_id: None,
            stale_package_id_source: None,
        }
    }

    pub fn add_provenance_role(&mut self, role: PackageSearchRootRole) {
        if !self.provenance_roles.contains(&role) {
            self.provenance_roles.push(role);
        }
    }

    pub fn provenance_roles_text(&self) -> String {
        if self.provenance_roles.is_empty() {
            return "none".to_string();
        }

        self.provenance_roles
            .iter()
            .map(|role| role.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn origin_text(&self) -> &'static str {
        self.origin.as_str()
    }
}

pub fn canonicalize_package_id(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

pub fn normalize_path_identity(value: &str) -> String {
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
