use rustystartup_core::{
    build_package_discovery_report, normalize_path_identity, PackageDiscoveryInput,
    PackageDiscoveryTextInput,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn relative_root_inputs_are_explicitly_unsupported() {
    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text("relative/self", "test.self"),
        official_data_root: absolute_root("official"),
        local_mods_root: absolute_root("local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert_eq!(report.result.status, "failed");
    assert!(report
        .diagnostics
        .iter()
        .any(|line| line.contains("package_search_root_status=role=self|status=unsupported")));
}

#[test]
fn discovery_root_normalization_is_stable_and_deterministic() {
    let temp = TempWorkspace::new("normalize");
    let root = temp
        .root()
        .join("mods")
        .join("..")
        .join("mods")
        .join("package");
    fs::create_dir_all(&root).unwrap();

    let normalized = normalize_path_identity(root.to_string_lossy().as_ref());
    let normalized_again = normalize_path_identity(root.to_string_lossy().as_ref());

    assert_eq!(normalized, normalized_again);
    assert!(!normalized.is_empty());
}

#[test]
fn package_discovery_basis_keeps_optional_workshop_root_optional() {
    let workspace = TempWorkspace::new("optional_workshop");
    let self_root = package_root(&workspace, "self");
    let official_root = package_root(&workspace, "official");
    let local_root = package_root(&workspace, "local");

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(self_root.to_string_lossy().as_ref(), "test.self"),
        official_data_root: text(official_root.to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(local_root.to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert!(report
        .diagnostics
        .iter()
        .any(|line| line.contains("package_search_root_status=role=workshop|status=unavailable")));
}

fn absolute_root(label: &str) -> PackageDiscoveryTextInput {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rustystartup_slice005_{}_{}_{}",
        label,
        process::id(),
        unique_stamp()
    ));
    fs::create_dir_all(&path).unwrap();
    PackageDiscoveryTextInput {
        value: Some(path.to_string_lossy().to_string()),
        source: Some(format!("test.{}", label)),
    }
}

fn text(value: &str, source: &str) -> PackageDiscoveryTextInput {
    PackageDiscoveryTextInput {
        value: Some(value.to_string()),
        source: Some(source.to_string()),
    }
}

fn package_root(workspace: &TempWorkspace, label: &str) -> PathBuf {
    let root = workspace.root().join(label);
    let about_dir = root.join("About");
    fs::create_dir_all(&about_dir).unwrap();
    fs::write(
        about_dir.join("About.xml"),
        format!(
            "<ModMetaData><name>{}</name><packageId>{}.package</packageId></ModMetaData>",
            label, label
        ),
    )
    .unwrap();
    root
}

fn unique_stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos()
}

struct TempWorkspace {
    root: PathBuf,
}

impl TempWorkspace {
    fn new(label: &str) -> Self {
        let mut root = std::env::temp_dir();
        root.push(format!(
            "rustystartup_slice005_{}_{}_{}",
            label,
            process::id(),
            unique_stamp()
        ));
        fs::create_dir_all(&root).unwrap();
        Self { root }
    }

    fn root(&self) -> &Path {
        &self.root
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
