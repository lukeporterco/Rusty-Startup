use rustystartup_core::{
    build_package_discovery_report, PackageDiscoveryInput, PackageDiscoveryTextInput,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn self_official_local_and_workshop_roots_resolve_with_expected_origins() {
    let workspace = TempWorkspace::new("origins");
    let self_root = package_root(
        workspace.root(),
        "self",
        "rustystartup.core",
        "Rusty Startup",
        None,
    );
    let official_basis = workspace.root().join("official_data");
    let local_basis = workspace.root().join("local_mods");
    let workshop_basis = workspace.root().join("workshop");
    let official_root = package_root(
        &official_basis,
        "official_mod",
        "ludeon.core",
        "RimWorld Core",
        None,
    );
    let local_root = package_root(&local_basis, "local_mod", "local.mod", "Local Mod", None);
    let workshop_root = package_root(
        &workshop_basis,
        "294100",
        "workshop.mod",
        "Workshop Mod",
        Some("123456789"),
    );

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(self_root.to_string_lossy().as_ref(), "test.self"),
        official_data_root: text(official_basis.to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(local_basis.to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: text(workshop_basis.to_string_lossy().as_ref(), "test.workshop"),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert_eq!(report.result.status, "resolved");
    assert!(
        report
            .result
            .identities
            .iter()
            .any(
                |identity| identity.package_id_canonical.as_deref() == Some("rustystartup.core")
                    && identity.origin.as_str() == "self"
            ),
        "{:?}",
        report.result.identities
    );
    assert!(
        report
            .result
            .identities
            .iter()
            .any(
                |identity| identity.package_id_canonical.as_deref() == Some("ludeon.core")
                    && identity.origin.as_str() == "official"
            ),
        "{:?}",
        report.result.identities
    );
    assert!(
        report
            .result
            .identities
            .iter()
            .any(
                |identity| identity.package_id_canonical.as_deref() == Some("local.mod")
                    && identity.origin.as_str() == "local"
            ),
        "{:?}",
        report.result.identities
    );
    assert!(
        report
            .result
            .identities
            .iter()
            .any(
                |identity| identity.package_id_canonical.as_deref() == Some("workshop.mod")
                    && identity.origin.as_str() == "workshop"
            ),
        "{:?}",
        report.result.identities
    );
    assert!(report
        .diagnostics
        .iter()
        .any(|line| line.contains("published_file_id_status=root=")
            && line.contains("status=present")));
    assert!(official_root.exists());
    assert!(local_root.exists());
    assert!(workshop_root.exists());
}

#[test]
fn duplicate_package_ids_across_roots_produce_conflict_diagnostics() {
    let workspace = TempWorkspace::new("duplicates");
    let duplicate_basis = workspace.root().join("dups");
    let _root_a = package_root(
        &duplicate_basis,
        "mod_a",
        "duplicate.mod",
        "Duplicate A",
        None,
    );
    let _root_b = package_root(
        &duplicate_basis,
        "mod_b",
        "duplicate.mod",
        "Duplicate B",
        None,
    );

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(duplicate_basis.to_string_lossy().as_ref(), "test.self"),
        official_data_root: text(duplicate_basis.to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(duplicate_basis.to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert_eq!(report.result.status, "failed");
    assert!(report
        .diagnostics
        .iter()
        .any(|line| line
            .contains("package_identity_status=package_id=duplicate.mod|status=conflict")));
}

#[test]
fn nested_about_trees_produce_nested_diagnostics() {
    let workspace = TempWorkspace::new("nested");
    let root = package_root(
        &workspace.root().join("root"),
        "root",
        "root.mod",
        "Root Mod",
        None,
    );
    let nested = package_root(&root, "nested", "nested.mod", "Nested Mod", None);

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

    assert_eq!(report.result.status, "failed");
    assert!(report
        .diagnostics
        .iter()
        .any(|line| line.contains("nested_about_tree_status=root=")
            && line.contains("status=conflict")));
    assert!(nested.exists());
}

#[test]
fn missing_package_id_is_explicit_and_prevents_canonical_identity() {
    let workspace = TempWorkspace::new("missing_package_id");
    let root = workspace.root().join("root");
    let about_dir = root.join("About");
    fs::create_dir_all(&about_dir).unwrap();
    fs::write(
        about_dir.join("About.xml"),
        "<ModMetaData><name>Missing Package ID</name></ModMetaData>",
    )
    .unwrap();

    let report = build_package_discovery_report(PackageDiscoveryInput {
        self_package_root: text(root.to_string_lossy().as_ref(), "test.self"),
        official_data_root: text(workspace.root().to_string_lossy().as_ref(), "test.official"),
        local_mods_root: text(workspace.root().to_string_lossy().as_ref(), "test.local"),
        workshop_content_root: PackageDiscoveryTextInput::default(),
        historical_stale_package_id: PackageDiscoveryTextInput::default(),
    });

    assert_eq!(report.result.status, "failed");
    assert!(report.diagnostics.iter().any(
        |line| line.contains("about_metadata_status=root=") && line.contains("status=present")
    ));
    assert!(report
        .diagnostics
        .iter()
        .any(|line| line.contains("package_identity_status=root=")
            && line.contains("status=missing")));
    assert!(report
        .result
        .identities
        .iter()
        .all(|identity| identity.package_id_canonical.is_none()));
}

#[test]
fn stale_lukep_package_identity_is_reported_as_stale_state_only() {
    let workspace = TempWorkspace::new("stale");
    let root = package_root(
        &workspace.root().join("root"),
        "root",
        "lukep.rustystartup",
        "Rusty Startup",
        None,
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

    assert_eq!(report.result.status, "failed");
    assert!(report.diagnostics.iter().any(
        |line| line.contains("package_identity_status=root=") && line.contains("status=stale")
    ));
    assert!(report
        .result
        .identities
        .iter()
        .all(|identity| identity.package_id_canonical.is_none()));
}

fn text(value: &str, source: &str) -> PackageDiscoveryTextInput {
    PackageDiscoveryTextInput {
        value: Some(value.to_string()),
        source: Some(source.to_string()),
    }
}

fn package_root(
    base: &Path,
    child: &str,
    package_id: &str,
    name: &str,
    published_file_id: Option<&str>,
) -> PathBuf {
    let root = base.join(child);
    let about_dir = root.join("About");
    fs::create_dir_all(&about_dir).unwrap();
    fs::write(
        about_dir.join("About.xml"),
        format!(
            "<ModMetaData><name>{}</name><packageId>{}</packageId><modDependencies><li>test</li></modDependencies></ModMetaData>",
            name, package_id
        ),
    )
    .unwrap();

    if let Some(published_file_id) = published_file_id {
        fs::write(about_dir.join("PublishedFileId.txt"), published_file_id).unwrap();
    }

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
