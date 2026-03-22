use rustystartup_core::{
    build_package_discovery_report, PackageDiscoveryInput, PackageDiscoveryTextInput,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn rendered_discovery_report_contains_required_diagnostics() {
    let workspace = TempWorkspace::new("rendered");
    let root = package_root(
        &workspace.root().join("root"),
        "root",
        "rendered.mod",
        "Rendered Mod",
    );

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(
            root.parent().unwrap().to_string_lossy().as_ref(),
            "test.self",
        ),
        official_data_root: text(workspace.root().to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(workspace.root().to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    let rendered = rustystartup_core::render_package_discovery_report(&report);
    assert!(rendered.contains("package_resolution_result="));
    assert!(rendered.contains("diagnostic=package_discovery_basis"));
    assert!(rendered.contains("diagnostic=package_search_root_status"));
    assert!(rendered.contains("diagnostic=discovered_package_root"));
    assert!(rendered.contains("diagnostic=about_metadata_status"));
    assert!(rendered.contains("diagnostic=published_file_id_status"));
    assert!(rendered.contains("diagnostic=dependency_metadata_status"));
    assert!(rendered.contains("diagnostic=package_origin_status"));
    assert!(rendered.contains("diagnostic=package_identity_status"));
}

#[test]
fn package_resolution_result_exposes_canonical_or_failure_reason() {
    let workspace = TempWorkspace::new("failure_reason");
    let root = workspace.root().join("missing_metadata");
    fs::create_dir_all(root.join("About")).unwrap();

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(root.to_string_lossy().as_ref(), "test.self"),
        official_data_root: text(workspace.root().to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(workspace.root().to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert_eq!(report.result.status, "failed");
    assert!(!report.result.reason_code.is_empty());
    assert!(!report.result.reason.is_empty());
}

fn text(value: &str, source: &str) -> PackageDiscoveryTextInput {
    PackageDiscoveryTextInput {
        value: Some(value.to_string()),
        source: Some(source.to_string()),
    }
}

fn package_root(base: &Path, child: &str, package_id: &str, name: &str) -> PathBuf {
    let root = base.join(child);
    let about_dir = root.join("About");
    fs::create_dir_all(&about_dir).unwrap();
    fs::write(
        about_dir.join("About.xml"),
        format!(
            "<ModMetaData><name>{}</name><packageId>{}</packageId></ModMetaData>",
            name, package_id
        ),
    )
    .unwrap();
    root
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

fn unique_stamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos()
}
