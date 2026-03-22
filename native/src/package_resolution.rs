use crate::mod_identity::{
    canonicalize_package_id, normalize_path_identity, ModIdentity, PackageOrigin,
    PackageSearchRootRole,
};
use crate::package_discovery_contract::{PackageDiscoveryInput, PackageDiscoveryTextInput};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageDiscoveryReport {
    pub result: PackageResolutionResult,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageResolutionResult {
    pub status: String,
    pub reason_code: String,
    pub reason: String,
    pub identities: Vec<ModIdentity>,
}

#[derive(Debug, Clone)]
struct SearchRootBasis {
    role: PackageSearchRootRole,
    normalized_value: String,
    source: String,
}

pub fn resolve_package_discovery(input: PackageDiscoveryInput) -> PackageDiscoveryReport {
    let mut diagnostics = Vec::new();
    let mut identities_by_root: BTreeMap<String, ModIdentity> = BTreeMap::new();
    let mut provenance_by_root: BTreeMap<String, Vec<PackageSearchRootRole>> = BTreeMap::new();
    let mut fatal_root_issue = false;

    push_basis_diagnostic(&mut diagnostics, &input);

    let bases = [
        (
            PackageSearchRootRole::SelfPackage,
            "self_package_root",
            &input.self_package_root,
            true,
        ),
        (
            PackageSearchRootRole::OfficialData,
            "official_data_root",
            &input.official_data_root,
            true,
        ),
        (
            PackageSearchRootRole::LocalMods,
            "local_mods_root",
            &input.local_mods_root,
            true,
        ),
        (
            PackageSearchRootRole::WorkshopContent,
            "workshop_content_root",
            &input.workshop_content_root,
            false,
        ),
    ];

    let mut resolved_bases = Vec::new();
    for (role, label, fact, required) in bases {
        match resolve_search_root(role, label, fact, required, &mut diagnostics) {
            Some(basis) => resolved_bases.push(basis),
            None if required => fatal_root_issue = true,
            None => {}
        }
    }

    for basis in &resolved_bases {
        for candidate_root in discover_candidate_roots(&basis.normalized_value) {
            let about_path = candidate_root.join("About").join("About.xml");
            if !about_path.exists() {
                continue;
            }

            let normalized_candidate =
                normalize_path_identity(candidate_root.to_string_lossy().as_ref());
            let root_detection_source = format!("discovered_from={}", basis.source);
            let identity = identities_by_root
                .entry(normalized_candidate.clone())
                .or_insert_with(|| {
                    ModIdentity::new(
                        candidate_root.to_string_lossy().to_string(),
                        root_detection_source.clone(),
                    )
                });

            identity.root_detection_source = root_detection_source.clone();
            identity.add_provenance_role(basis.role);
            provenance_by_root
                .entry(normalized_candidate.clone())
                .or_default()
                .push(basis.role);

            analyze_package_root(&about_path, basis, identity);
            emit_discovered_package_root(&mut diagnostics, basis, identity);
        }
    }

    let mut identities: Vec<ModIdentity> = identities_by_root.into_values().collect();
    mark_origin_classification(&mut identities, &provenance_by_root);
    mark_nested_about_trees(&mut identities, &mut diagnostics);
    mark_duplicate_package_ids(&mut identities, &mut diagnostics);
    mark_stale_package_ids(&mut identities, &input, &mut diagnostics);
    finalize_identity_statuses(&mut identities, &mut diagnostics);

    let mut status = "resolved".to_string();
    let mut reason_code = "resolved".to_string();
    let mut reason =
        "canonical package identities resolved from discoverable metadata and auditable provenance."
            .to_string();

    if fatal_root_issue {
        status = "failed".to_string();
        reason_code = "required_discovery_root_missing".to_string();
        reason = "one or more required discovery roots were missing, unavailable, or unsupported"
            .to_string();
    }

    if identities.is_empty() {
        status = "failed".to_string();
        reason_code = "no_package_metadata_found".to_string();
        reason = "no discoverable package metadata produced a canonical ModIdentity".to_string();
    }

    if identities
        .iter()
        .any(|identity| identity.package_identity_status == "conflict")
    {
        status = "failed".to_string();
        reason_code = "duplicate_package_id_conflict".to_string();
        reason = "duplicate package IDs were discovered across multiple package roots".to_string();
    }

    if identities
        .iter()
        .any(|identity| identity.about_status == "nested_about_tree")
    {
        status = "failed".to_string();
        reason_code = "nested_about_tree_detected".to_string();
        reason = "nested About/About.xml trees were discovered under one or more package roots"
            .to_string();
    }

    if status == "resolved"
        && !identities
            .iter()
            .any(|identity| identity.package_identity_status == "canonical")
    {
        status = "failed".to_string();
        reason_code = "no_canonical_package_identity".to_string();
        reason =
            "no canonical package identity could be produced from the supplied discovery basis"
                .to_string();
    }

    diagnostics.push(format!(
        "package_resolution_result=status={}|reason_code={}|reason={}",
        status, reason_code, reason
    ));

    if status == "resolved" {
        append_canonical_identity_summary(&identities, &mut diagnostics);
    }

    PackageDiscoveryReport {
        result: PackageResolutionResult {
            status,
            reason_code,
            reason,
            identities,
        },
        diagnostics,
    }
}

fn push_basis_diagnostic(diagnostics: &mut Vec<String>, input: &PackageDiscoveryInput) {
    diagnostics.push(format!(
        "package_discovery_basis=self_root={}|self_source={}|official_root={}|official_source={}|local_root={}|local_source={}|workshop_root={}|workshop_source={}|stale_id={}|stale_source={}",
        render_text(&input.self_package_root.value),
        render_text(&input.self_package_root.source),
        render_text(&input.official_data_root.value),
        render_text(&input.official_data_root.source),
        render_text(&input.local_mods_root.value),
        render_text(&input.local_mods_root.source),
        render_text(&input.workshop_content_root.value),
        render_text(&input.workshop_content_root.source),
        render_text(&input.historical_stale_package_id.value),
        render_text(&input.historical_stale_package_id.source),
    ));
}

fn resolve_search_root(
    role: PackageSearchRootRole,
    label: &str,
    fact: &PackageDiscoveryTextInput,
    required: bool,
    diagnostics: &mut Vec<String>,
) -> Option<SearchRootBasis> {
    let source = fact
        .source
        .clone()
        .unwrap_or_else(|| format!("{}_source", label));
    let Some(raw_value) = fact.value.clone() else {
        diagnostics.push(format!(
            "package_search_root_status=role={}|status=unavailable|source={}|reason={}",
            role.as_str(),
            source,
            if required {
                "required discovery root was not provided"
            } else {
                "optional discovery root was not provided"
            }
        ));
        return None;
    };

    if !Path::new(&raw_value).is_absolute() {
        diagnostics.push(format!(
            "package_search_root_status=role={}|status=unsupported|path={}|source={}|reason=discovery root must be an absolute path",
            role.as_str(),
            raw_value,
            source
        ));
        return None;
    }

    let normalized_value = normalize_path_identity(&raw_value);
    let exists = Path::new(&raw_value).exists();
    let status = if exists { "observed" } else { "missing" };
    let reason = if exists {
        "discovery root observed"
    } else {
        "discovery root path does not exist"
    };
    diagnostics.push(format!(
        "package_search_root_status=role={}|status={}|path={}|normalized={}|source={}|reason={}",
        role.as_str(),
        status,
        raw_value,
        normalized_value,
        source,
        reason
    ));

    if !exists && required {
        return None;
    }

    Some(SearchRootBasis {
        role,
        normalized_value,
        source,
    })
}

fn discover_candidate_roots(normalized_basis: &str) -> Vec<PathBuf> {
    let basis = PathBuf::from(normalized_basis);
    let mut candidates = vec![basis.clone()];

    if let Ok(entries) = fs::read_dir(&basis) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                candidates.push(entry.path());
            }
        }
    }

    candidates
}

fn analyze_package_root(about_path: &Path, basis: &SearchRootBasis, identity: &mut ModIdentity) {
    let content = match fs::read_to_string(about_path) {
        Ok(value) => value,
        Err(_) => {
            identity.about_status = "parse_failed".to_string();
            identity.package_identity_status = "missing".to_string();
            return;
        }
    };

    identity.about_status = "present".to_string();
    identity.package_id = extract_tag_text(&content, "packageId");
    identity.display_name_basis =
        extract_tag_text(&content, "name").or_else(|| extract_tag_text(&content, "displayName"));
    identity.dependency_metadata_present = content.contains("<modDependencies")
        || content.contains("<loadBefore")
        || content.contains("<loadAfter");
    identity.dependency_metadata_status = if identity.dependency_metadata_present {
        "present".to_string()
    } else {
        "missing".to_string()
    };
    identity.load_before_present = content.contains("<loadBefore");
    identity.load_after_present = content.contains("<loadAfter");

    let published_file_id_path = about_path
        .parent()
        .map(|path| path.join("PublishedFileId.txt"));
    if let Some(path) = published_file_id_path {
        if !path.exists() {
            identity.published_file_status = "missing".to_string();
        } else {
            match fs::read_to_string(&path) {
                Ok(value) => {
                    let trimmed = value.trim();
                    if trimmed.is_empty() {
                        identity.published_file_status = "missing".to_string();
                    } else {
                        identity.published_file_status = "present".to_string();
                        identity.published_file_id = Some(trimmed.to_string());
                    }
                }
                Err(_) => {
                    identity.published_file_status = "parse_failed".to_string();
                }
            }
        }
    }

    if let Some(package_id) = &identity.package_id {
        let canonical = canonicalize_package_id(package_id);
        identity.package_id_canonical = Some(canonical);
        identity.package_identity_status = "canonical".to_string();
    } else {
        identity.package_identity_status = "missing".to_string();
    }

    identity.root_detection_source =
        format!("{}|about={}", basis.source, about_path.to_string_lossy());
}

fn mark_origin_classification(
    identities: &mut [ModIdentity],
    provenance_by_root: &BTreeMap<String, Vec<PackageSearchRootRole>>,
) {
    for identity in identities {
        let roles = provenance_by_root
            .get(&identity.normalized_package_root)
            .cloned()
            .unwrap_or_default();
        let has_self = roles.contains(&PackageSearchRootRole::SelfPackage);
        let has_official = roles.contains(&PackageSearchRootRole::OfficialData);
        let has_local = roles.contains(&PackageSearchRootRole::LocalMods);
        let has_workshop = roles.contains(&PackageSearchRootRole::WorkshopContent);

        identity.origin = if has_self {
            PackageOrigin::SelfPackage
        } else if roles.len() > 1 {
            PackageOrigin::Ambiguous
        } else if has_workshop {
            PackageOrigin::WorkshopContent
        } else if has_official {
            PackageOrigin::OfficialData
        } else if has_local {
            PackageOrigin::LocalMods
        } else {
            PackageOrigin::Unknown
        };

        if roles.len() > 1 && !has_self {
            identity.package_identity_status = "ambiguous".to_string();
        }
    }
}

fn mark_nested_about_trees(identities: &mut [ModIdentity], diagnostics: &mut Vec<String>) {
    for identity in identities {
        let nested = collect_nested_about_trees(Path::new(&identity.package_root));
        if nested.is_empty() {
            continue;
        }

        identity.about_status = "nested_about_tree".to_string();
        identity.package_identity_status = "conflict".to_string();

        for nested_path in nested {
            diagnostics.push(format!(
                "nested_about_tree_status=root={}|nested={}|status=conflict|reason=nested About/About.xml tree detected",
                identity.normalized_package_root,
                normalize_path_identity(nested_path.to_string_lossy().as_ref())
            ));
        }
    }
}

fn collect_nested_about_trees(root: &Path) -> Vec<PathBuf> {
    let mut nested = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(current) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&current) {
            for entry in entries.flatten() {
                if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                    let path = entry.path();
                    if path != root && path.join("About").join("About.xml").exists() {
                        nested.push(path.clone());
                    }
                    stack.push(path);
                }
            }
        }
    }

    nested
}

fn mark_duplicate_package_ids(identities: &mut [ModIdentity], diagnostics: &mut Vec<String>) {
    let mut package_id_map: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (index, identity) in identities.iter().enumerate() {
        if identity.package_identity_status == "stale" {
            continue;
        }

        if let Some(package_id) = &identity.package_id_canonical {
            package_id_map
                .entry(package_id.clone())
                .or_default()
                .push(index);
        }
    }

    for (package_id, indices) in package_id_map {
        if indices.len() < 2 {
            continue;
        }

        let roots = indices
            .iter()
            .map(|index| identities[*index].normalized_package_root.clone())
            .collect::<Vec<_>>()
            .join(",");

        diagnostics.push(format!(
            "package_identity_status=package_id={}|status=conflict|roots={}|reason=duplicate package ID discovered across multiple package roots",
            package_id,
            roots
        ));

        for index in indices {
            identities[index].package_identity_status = "conflict".to_string();
            identities[index].package_id_canonical = None;
        }
    }
}

fn mark_stale_package_ids(
    identities: &mut [ModIdentity],
    input: &PackageDiscoveryInput,
    diagnostics: &mut Vec<String>,
) {
    let stale_input = input
        .historical_stale_package_id
        .value
        .as_ref()
        .map(|value| canonicalize_package_id(value));

    for identity in identities {
        let encountered_stale = identity
            .package_id
            .as_ref()
            .map(|value| canonicalize_package_id(value) == "lukep.rustystartup")
            .unwrap_or(false)
            || stale_input
                .as_ref()
                .and_then(|value| {
                    identity
                        .package_id_canonical
                        .as_ref()
                        .map(|package_id| package_id == value)
                })
                .unwrap_or(false);

        if encountered_stale {
            identity.package_identity_status = "stale".to_string();
            identity.package_id_canonical = None;
            identity.stale_package_id = identity
                .package_id
                .as_ref()
                .map(|value| canonicalize_package_id(value));
            identity.stale_package_id_source = input.historical_stale_package_id.source.clone();
            diagnostics.push(format!(
                "package_identity_status=root={}|status=stale|package_id={}|source={}|reason=historical lukep.rustystartup state encountered",
                identity.normalized_package_root,
                render_text(&identity.stale_package_id),
                render_text(&identity.stale_package_id_source)
            ));
        }
    }
}

fn finalize_identity_statuses(identities: &mut [ModIdentity], diagnostics: &mut Vec<String>) {
    for identity in identities {
        diagnostics.push(format!(
            "about_metadata_status=root={}|status={}|package_id={}|display_name_basis={}|source={}",
            identity.normalized_package_root,
            identity.about_status,
            render_text(&identity.package_id),
            render_text(&identity.display_name_basis),
            identity.root_detection_source
        ));
        diagnostics.push(format!(
            "published_file_id_status=root={}|status={}|value={}|source={}",
            identity.normalized_package_root,
            identity.published_file_status,
            render_text(&identity.published_file_id),
            identity.root_detection_source
        ));
        diagnostics.push(format!(
            "dependency_metadata_status=root={}|status={}|load_before={}|load_after={}",
            identity.normalized_package_root,
            identity.dependency_metadata_status,
            identity.load_before_present,
            identity.load_after_present
        ));
        diagnostics.push(format!(
            "package_origin_status=root={}|origin={}|provenance_roles={}|source={}",
            identity.normalized_package_root,
            identity.origin_text(),
            identity.provenance_roles_text(),
            identity.root_detection_source
        ));
        diagnostics.push(format!(
            "package_identity_status=root={}|status={}|package_id={}|canonical_package_id={}|origin={}|provenance_roles={}",
            identity.normalized_package_root,
            identity.package_identity_status,
            render_text(&identity.package_id),
            render_text(&identity.package_id_canonical),
            identity.origin_text(),
            identity.provenance_roles_text()
        ));
    }
}

fn append_canonical_identity_summary(identities: &[ModIdentity], diagnostics: &mut Vec<String>) {
    for identity in identities {
        if identity.package_identity_status == "canonical" {
            diagnostics.push(format!(
                "canonical_mod_identity=root={}|package_id={}|display_name_basis={}|origin={}",
                identity.normalized_package_root,
                render_text(&identity.package_id_canonical),
                render_text(&identity.display_name_basis),
                identity.origin_text()
            ));
        }
    }
}

fn emit_discovered_package_root(
    diagnostics: &mut Vec<String>,
    basis: &SearchRootBasis,
    identity: &ModIdentity,
) {
    diagnostics.push(format!(
        "discovered_package_root=role={}|root={}|normalized={}|source={}",
        basis.role.as_str(),
        identity.package_root,
        identity.normalized_package_root,
        basis.source
    ));
}

fn extract_tag_text(content: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}>", tag);
    let close_tag = format!("</{}>", tag);
    let start = content.find(&open_tag)? + open_tag.len();
    let end = content[start..].find(&close_tag)? + start;
    let value = content[start..end].trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn render_text(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "none".to_string())
}
