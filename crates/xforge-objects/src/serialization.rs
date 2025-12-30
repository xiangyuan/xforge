//! Serialization support for PBX objects to ASCII Plist format

use xforge_core::{Handle, PBXObject, Registry};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;
use std::any::Any;

use crate::{
    pbx_project::PBXProject,
    pbx_target::PBXNativeTarget,
    pbx_file_reference::PBXFileReference,
    pbx_group::PBXGroup,
    pbx_build_configuration::{XCBuildConfiguration, XCConfigurationList},
    pbx_build_phase::{
        PBXSourcesBuildPhase, PBXFrameworksBuildPhase, PBXResourcesBuildPhase,
        PBXShellScriptBuildPhase, PBXCopyFilesBuildPhase, PBXHeadersBuildPhase, PBXBuildFile,
    },
    pbx_container_item_proxy::PBXContainerItemProxy,
    pbx_file_system_synchronized::{
        PBXFileSystemSynchronizedBuildFileExceptionSet,
        PBXFileSystemSynchronizedRootGroup,
    },
    pbx_target_dependency::PBXTargetDependency,
    pbx_variant_group::PBXVariantGroup,
    pbx_reference_proxy::PBXReferenceProxy,
    pbx_aggregate_target::PBXAggregateTarget,
    pbx_legacy_target::PBXLegacyTarget,
    xc_swift_package::{XCSwiftPackageProductDependency, XCRemoteSwiftPackageReference},
    xc_version_group::XCVersionGroup,
};

/// Serialize the entire registry to PlistValue
pub fn serialize_registry(registry: &Registry, root_project_id: &str) -> PlistValue {
    let mut root_dict = IndexMap::new();
    
    // Archive version
    root_dict.insert("archiveVersion".to_string(), PlistValue::String("1".to_string()));
    
    // Classes (empty)
    root_dict.insert("classes".to_string(), PlistValue::Dictionary(IndexMap::new()));
    
    // Object version
    root_dict.insert("objectVersion".to_string(), PlistValue::Integer(56));
    
    // Objects dictionary
    let mut objects = IndexMap::new();
    
    // Serialize all objects in registry
    for (id, obj) in registry.iter() {
        if let Some(plist_obj) = serialize_object(obj.as_ref(), registry) {
            objects.insert(id.clone(), plist_obj);
        }
    }
    
    root_dict.insert("objects".to_string(), PlistValue::Dictionary(objects));
    
    // Root object
    root_dict.insert("rootObject".to_string(), PlistValue::String(root_project_id.to_string()));
    
    PlistValue::Dictionary(root_dict)
}

/// Serialize a single PBX object
fn serialize_object(obj: &dyn PBXObject, _registry: &Registry) -> Option<PlistValue> {
    let mut dict = IndexMap::new();
    
    // Add isa field
    dict.insert("isa".to_string(), PlistValue::String(obj.isa().to_string()));
    
    // Try to downcast to specific types and serialize their fields
    let any_obj = obj as &dyn Any;
    
    if let Some(project) = any_obj.downcast_ref::<PBXProject>() {
        serialize_project(project, &mut dict);
    } else if let Some(target) = any_obj.downcast_ref::<PBXNativeTarget>() {
        serialize_target(target, &mut dict);
    } else if let Some(file_ref) = any_obj.downcast_ref::<PBXFileReference>() {
        serialize_file_reference(file_ref, &mut dict);
    } else if let Some(group) = any_obj.downcast_ref::<PBXGroup>() {
        serialize_group(group, &mut dict);
    } else if let Some(config) = any_obj.downcast_ref::<XCBuildConfiguration>() {
        serialize_build_configuration(config, &mut dict);
    } else if let Some(config_list) = any_obj.downcast_ref::<XCConfigurationList>() {
        serialize_configuration_list(config_list, &mut dict);
    } else if let Some(sources) = any_obj.downcast_ref::<PBXSourcesBuildPhase>() {
        serialize_build_phase_common(&sources.files, sources.build_action_mask, sources.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(frameworks) = any_obj.downcast_ref::<PBXFrameworksBuildPhase>() {
        serialize_build_phase_common(&frameworks.files, frameworks.build_action_mask, frameworks.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(resources) = any_obj.downcast_ref::<PBXResourcesBuildPhase>() {
        serialize_build_phase_common(&resources.files, resources.build_action_mask, resources.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(shell) = any_obj.downcast_ref::<PBXShellScriptBuildPhase>() {
        serialize_shell_script_phase(shell, &mut dict);
    } else if let Some(copy) = any_obj.downcast_ref::<PBXCopyFilesBuildPhase>() {
        serialize_copy_files_phase(copy, &mut dict);
    } else if let Some(headers) = any_obj.downcast_ref::<PBXHeadersBuildPhase>() {
        serialize_headers_phase(headers, &mut dict);
    } else if let Some(build_file) = any_obj.downcast_ref::<PBXBuildFile>() {
        serialize_build_file(build_file, &mut dict);
    } else if let Some(proxy) = any_obj.downcast_ref::<PBXContainerItemProxy>() {
        serialize_container_item_proxy(proxy, &mut dict);
    } else if let Some(exception_set) = any_obj.downcast_ref::<PBXFileSystemSynchronizedBuildFileExceptionSet>() {
        serialize_file_system_exception_set(exception_set, &mut dict);
    } else if let Some(sync_group) = any_obj.downcast_ref::<PBXFileSystemSynchronizedRootGroup>() {
        serialize_file_system_synchronized_group(sync_group, &mut dict);
    } else if let Some(dependency) = any_obj.downcast_ref::<PBXTargetDependency>() {
        serialize_target_dependency(dependency, &mut dict);
    } else if let Some(variant_group) = any_obj.downcast_ref::<PBXVariantGroup>() {
        serialize_variant_group(variant_group, &mut dict);
    } else if let Some(ref_proxy) = any_obj.downcast_ref::<PBXReferenceProxy>() {
        serialize_reference_proxy(ref_proxy, &mut dict);
    } else if let Some(swift_product) = any_obj.downcast_ref::<XCSwiftPackageProductDependency>() {
        serialize_swift_package_product_dependency(swift_product, &mut dict);
    } else if let Some(swift_ref) = any_obj.downcast_ref::<XCRemoteSwiftPackageReference>() {
        serialize_remote_swift_package_reference(swift_ref, &mut dict);
    } else if let Some(aggregate) = any_obj.downcast_ref::<PBXAggregateTarget>() {
        serialize_aggregate_target(aggregate, &mut dict);
    } else if let Some(legacy) = any_obj.downcast_ref::<PBXLegacyTarget>() {
        serialize_legacy_target(legacy, &mut dict);
    } else if let Some(version_group) = any_obj.downcast_ref::<XCVersionGroup>() {
        serialize_version_group(version_group, &mut dict);
    }
    
    Some(PlistValue::Dictionary(dict))
}

fn serialize_project(project: &PBXProject, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("compatibilityVersion".to_string(), PlistValue::String(project.compatibility_version().to_string()));
    dict.insert("developmentRegion".to_string(), PlistValue::String(project.development_region().to_string()));
    
    if let Some(main_group) = project.main_group() {
        dict.insert("mainGroup".to_string(), PlistValue::String(main_group.to_string()));
    }
    
    let targets: Vec<PlistValue> = project.targets()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("targets".to_string(), PlistValue::Array(targets));
    
    // Add missing critical fields
    if let Some(config_list) = project.build_configuration_list {
        dict.insert("buildConfigurationList".to_string(), PlistValue::String(config_list.to_string()));
    }
    
    if project.has_scanned_for_encodings {
        dict.insert("hasScannedForEncodings".to_string(), PlistValue::Integer(1));
    }
    
    if !project.known_regions.is_empty() {
        let regions: Vec<PlistValue> = project.known_regions
            .iter()
            .map(|r| PlistValue::String(r.clone()))
            .collect();
        dict.insert("knownRegions".to_string(), PlistValue::Array(regions));
    }
    
    if let Some(product_ref_group) = project.product_ref_group {
        dict.insert("productRefGroup".to_string(), PlistValue::String(product_ref_group.to_string()));
    }
    
    if !project.project_dir_path.is_empty() {
        dict.insert("projectDirPath".to_string(), PlistValue::String(project.project_dir_path.clone()));
    }
    
    if !project.project_root.is_empty() {
        dict.insert("projectRoot".to_string(), PlistValue::String(project.project_root.clone()));
    }
    
    if !project.package_references.is_empty() {
        let pkg_refs: Vec<PlistValue> = project.package_references
            .iter()
            .map(|h| PlistValue::String(h.to_string()))
            .collect();
        dict.insert("packageReferences".to_string(), PlistValue::Array(pkg_refs));
    }
    
    if !project.attributes.is_empty() {
        let mut attrs = IndexMap::new();
        for (key, value) in &project.attributes {
            attrs.insert(key.clone(), PlistValue::String(value.clone()));
        }
        dict.insert("attributes".to_string(), PlistValue::Dictionary(attrs));
    }
}

fn serialize_target(target: &PBXNativeTarget, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("name".to_string(), PlistValue::String(target.name().to_string()));
    
    if let Some(product_name) = target.product_name() {
        dict.insert("productName".to_string(), PlistValue::String(product_name.to_string()));
    }
    
    if let Some(product_type) = target.product_type() {
        dict.insert("productType".to_string(), PlistValue::String(product_type.as_str().to_string()));
    }
    
    let phases: Vec<PlistValue> = target.build_phases()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("buildPhases".to_string(), PlistValue::Array(phases));
    
    // Add missing critical fields
    if let Some(config_list) = target.build_configuration_list {
        dict.insert("buildConfigurationList".to_string(), PlistValue::String(config_list.to_string()));
    }
    
    if !target.build_rules.is_empty() {
        let rules: Vec<PlistValue> = target.build_rules
            .iter()
            .map(|h| PlistValue::String(h.to_string()))
            .collect();
        dict.insert("buildRules".to_string(), PlistValue::Array(rules));
    }
    
    if !target.dependencies.is_empty() {
        let deps: Vec<PlistValue> = target.dependencies
            .iter()
            .map(|h| PlistValue::String(h.to_string()))
            .collect();
        dict.insert("dependencies".to_string(), PlistValue::Array(deps));
    }
    
    if let Some(product_ref) = target.product_reference {
        dict.insert("productReference".to_string(), PlistValue::String(product_ref.to_string()));
    }
    
    if !target.package_product_dependencies.is_empty() {
        let pkg_deps: Vec<PlistValue> = target.package_product_dependencies
            .iter()
            .map(|h| PlistValue::String(h.to_string()))
            .collect();
        dict.insert("packageProductDependencies".to_string(), PlistValue::Array(pkg_deps));
    }
}

fn serialize_file_reference(file_ref: &PBXFileReference, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(path) = file_ref.path() {
        dict.insert("path".to_string(), PlistValue::String(path.to_string()));
    }
    
    if let Some(source_tree) = file_ref.source_tree() {
        dict.insert("sourceTree".to_string(), PlistValue::String(source_tree.to_string()));
    }
    
    // Add missing fields that were causing 40% data loss
    if let Some(ref name) = file_ref.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
    
    if let Some(ref last_known_file_type) = file_ref.last_known_file_type {
        dict.insert("lastKnownFileType".to_string(), PlistValue::String(last_known_file_type.clone()));
    }
    
    if let Some(file_encoding) = file_ref.file_encoding {
        dict.insert("fileEncoding".to_string(), PlistValue::Integer(file_encoding as i64));
    }
    
    if let Some(ref explicit_file_type) = file_ref.explicit_file_type {
        dict.insert("explicitFileType".to_string(), PlistValue::String(explicit_file_type.clone()));
    }
}

fn serialize_group(group: &PBXGroup, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(path) = group.path() {
        dict.insert("path".to_string(), PlistValue::String(path.to_string()));
    }
    
    if let Some(source_tree) = group.source_tree() {
        dict.insert("sourceTree".to_string(), PlistValue::String(source_tree.to_string()));
    }
    
    // Add missing name field
    if let Some(ref name) = group.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
    
    let children: Vec<PlistValue> = group.children()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("children".to_string(), PlistValue::Array(children));
}

fn serialize_build_configuration(config: &XCBuildConfiguration, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("name".to_string(), PlistValue::String(config.name().to_string()));
    
    let mut settings = IndexMap::new();
    for (key, value) in config.build_settings() {
        settings.insert(key.clone(), PlistValue::String(value.clone()));
    }
    dict.insert("buildSettings".to_string(), PlistValue::Dictionary(settings));
}

fn serialize_configuration_list(config_list: &XCConfigurationList, dict: &mut IndexMap<String, PlistValue>) {
    let configs: Vec<PlistValue> = config_list.build_configurations()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("buildConfigurations".to_string(), PlistValue::Array(configs));
    
    if let Some(default_name) = config_list.default_configuration_name() {
        dict.insert("defaultConfigurationName".to_string(), PlistValue::String(default_name.to_string()));
    }
    
    dict.insert("defaultConfigurationIsVisible".to_string(), 
        PlistValue::Integer(if config_list.default_configuration_is_visible() { 1 } else { 0 }));
}

fn serialize_build_phase_common(
    files: &[Handle<PBXBuildFile>],
    build_action_mask: u32,
    run_only: bool,
    dict: &mut IndexMap<String, PlistValue>
) {
    dict.insert("buildActionMask".to_string(), PlistValue::Integer(build_action_mask as i64));
    
    let files_array: Vec<PlistValue> = files
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("files".to_string(), PlistValue::Array(files_array));
    
    dict.insert("runOnlyForDeploymentPostprocessing".to_string(), 
        PlistValue::Integer(if run_only { 1 } else { 0 }));
}

fn serialize_shell_script_phase(shell: &PBXShellScriptBuildPhase, dict: &mut IndexMap<String, PlistValue>) {
    serialize_build_phase_common(
        &shell.files,
        shell.build_action_mask,
        shell.run_only_for_deployment_postprocessing,
        dict
    );
    
    if let Some(ref name) = shell.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
    
    dict.insert("shellPath".to_string(), PlistValue::String(shell.shell_path.clone()));
    dict.insert("shellScript".to_string(), PlistValue::String(shell.shell_script.clone()));
    
    if !shell.input_paths.is_empty() {
        let paths: Vec<PlistValue> = shell.input_paths.iter()
            .map(|p| PlistValue::String(p.clone()))
            .collect();
        dict.insert("inputPaths".to_string(), PlistValue::Array(paths));
    }
    
    if !shell.output_paths.is_empty() {
        let paths: Vec<PlistValue> = shell.output_paths.iter()
            .map(|p| PlistValue::String(p.clone()))
            .collect();
        dict.insert("outputPaths".to_string(), PlistValue::Array(paths));
    }
}

fn serialize_copy_files_phase(copy: &PBXCopyFilesBuildPhase, dict: &mut IndexMap<String, PlistValue>) {
    serialize_build_phase_common(
        &copy.files,
        copy.build_action_mask,
        copy.run_only_for_deployment_postprocessing,
        dict
    );
    
    if let Some(ref name) = copy.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
    
    dict.insert("dstSubfolderSpec".to_string(), PlistValue::Integer(copy.dst_subfolder_spec as i64));
    dict.insert("dstPath".to_string(), PlistValue::String(copy.dst_path.clone()));
}

fn serialize_headers_phase(headers: &PBXHeadersBuildPhase, dict: &mut IndexMap<String, PlistValue>) {
    serialize_build_phase_common(
        &headers.files,
        headers.build_action_mask,
        headers.run_only_for_deployment_postprocessing,
        dict
    );
}

fn serialize_build_file(build_file: &PBXBuildFile, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("fileRef".to_string(), PlistValue::String(build_file.file_ref.to_string()));
    
    if let Some(ref settings) = build_file.settings {
        let mut settings_dict = IndexMap::new();
        for (key, value) in settings {
            settings_dict.insert(key.clone(), PlistValue::String(value.clone()));
        }
        dict.insert("settings".to_string(), PlistValue::Dictionary(settings_dict));
    }
}

fn serialize_container_item_proxy(proxy: &PBXContainerItemProxy, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("containerPortal".to_string(), PlistValue::String(proxy.container_portal.to_string()));
    
    dict.insert("proxyType".to_string(), PlistValue::Integer(proxy.proxy_type as i64));
    
    if let Some(ref remote_id) = proxy.remote_global_id_string {
        dict.insert("remoteGlobalIDString".to_string(), PlistValue::String(remote_id.clone()));
    }
    
    if let Some(ref remote_info) = proxy.remote_info {
        dict.insert("remoteInfo".to_string(), PlistValue::String(remote_info.clone()));
    }
}

fn serialize_target_dependency(dependency: &PBXTargetDependency, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(ref target) = dependency.target {
        dict.insert("target".to_string(), PlistValue::String(target.to_string()));
    }
    
    if let Some(ref target_proxy) = dependency.target_proxy {
        dict.insert("targetProxy".to_string(), PlistValue::String(target_proxy.to_string()));
    }
    
    if let Some(ref name) = dependency.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
}

fn serialize_variant_group(variant_group: &PBXVariantGroup, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(ref name) = variant_group.name {
        dict.insert("name".to_string(), PlistValue::String(name.clone()));
    }
    
    let children: Vec<PlistValue> = variant_group.children
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("children".to_string(), PlistValue::Array(children));
    
    dict.insert("sourceTree".to_string(), PlistValue::String(variant_group.source_tree.clone()));
}

fn serialize_reference_proxy(ref_proxy: &PBXReferenceProxy, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("path".to_string(), PlistValue::String(ref_proxy.path.clone()));
    dict.insert("fileType".to_string(), PlistValue::String(ref_proxy.file_type.clone()));
    dict.insert("remoteRef".to_string(), PlistValue::String(ref_proxy.remote_ref.to_string()));
    
    dict.insert("sourceTree".to_string(), PlistValue::String(ref_proxy.source_tree.clone()));
}

fn serialize_swift_package_product_dependency(swift_product: &XCSwiftPackageProductDependency, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("productName".to_string(), PlistValue::String(swift_product.product_name.clone()));
    dict.insert("package".to_string(), PlistValue::String(swift_product.package.to_string()));
}

fn serialize_remote_swift_package_reference(swift_ref: &XCRemoteSwiftPackageReference, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("repositoryURL".to_string(), PlistValue::String(swift_ref.repository_url.clone()));
    
    let mut req_dict = IndexMap::new();
    match &swift_ref.requirement {
        crate::xc_swift_package::PackageRequirement::UpToNextMajorVersion(version) => {
            req_dict.insert("kind".to_string(), PlistValue::String("upToNextMajorVersion".to_string()));
            req_dict.insert("minimumVersion".to_string(), PlistValue::String(version.clone()));
        }
        crate::xc_swift_package::PackageRequirement::UpToNextMinorVersion(version) => {
            req_dict.insert("kind".to_string(), PlistValue::String("upToNextMinorVersion".to_string()));
            req_dict.insert("minimumVersion".to_string(), PlistValue::String(version.clone()));
        }
        crate::xc_swift_package::PackageRequirement::Exact(version) => {
            req_dict.insert("kind".to_string(), PlistValue::String("exactVersion".to_string()));
            req_dict.insert("version".to_string(), PlistValue::String(version.clone()));
        }
        crate::xc_swift_package::PackageRequirement::Branch(branch) => {
            req_dict.insert("kind".to_string(), PlistValue::String("branch".to_string()));
            req_dict.insert("branch".to_string(), PlistValue::String(branch.clone()));
        }
        crate::xc_swift_package::PackageRequirement::Revision(revision) => {
            req_dict.insert("kind".to_string(), PlistValue::String("revision".to_string()));
            req_dict.insert("revision".to_string(), PlistValue::String(revision.clone()));
        }
    }
    dict.insert("requirement".to_string(), PlistValue::Dictionary(req_dict));
}

#[cfg(test)]
mod tests {
    use super::*;
    use xforge_core::ObjectId;
    
    #[test]
    fn test_serialize_empty_registry() {
        let registry = Registry::new();
        let root_id = ObjectId::generate().to_uuid_string();
        let result = serialize_registry(&registry, &root_id);
        
        if let PlistValue::Dictionary(dict) = result {
            assert!(dict.contains_key("archiveVersion"));
            assert!(dict.contains_key("objects"));
            assert!(dict.contains_key("rootObject"));
        } else {
            panic!("Expected dictionary");
        }
    }
}

fn serialize_aggregate_target(target: &PBXAggregateTarget, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("name".to_string(), PlistValue::String(target.name().to_string()));
    
    if let Some(product_name) = target.product_name() {
        dict.insert("productName".to_string(), PlistValue::String(product_name.to_string()));
    }
    
    if let Some(config_list) = target.build_configuration_list() {
        dict.insert("buildConfigurationList".to_string(), PlistValue::String(config_list.to_string()));
    }
    
    let phases: Vec<PlistValue> = target.build_phases()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("buildPhases".to_string(), PlistValue::Array(phases));
    
    let deps: Vec<PlistValue> = target.dependencies()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("dependencies".to_string(), PlistValue::Array(deps));
}

fn serialize_legacy_target(target: &PBXLegacyTarget, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("name".to_string(), PlistValue::String(target.name().to_string()));
    
    if let Some(product_name) = target.product_name() {
        dict.insert("productName".to_string(), PlistValue::String(product_name.to_string()));
    }
    
    if let Some(config_list) = target.build_configuration_list() {
        dict.insert("buildConfigurationList".to_string(), PlistValue::String(config_list.to_string()));
    }
    
    dict.insert("buildToolPath".to_string(), PlistValue::String(target.build_tool_path().to_string()));
    
    if let Some(args) = target.build_arguments_string() {
        dict.insert("buildArgumentsString".to_string(), PlistValue::String(args.to_string()));
    }
    
    if let Some(working_dir) = target.build_working_directory() {
        dict.insert("buildWorkingDirectory".to_string(), PlistValue::String(working_dir.to_string()));
    }
    
    dict.insert("passBuildSettingsInEnvironment".to_string(), 
        PlistValue::Integer(if target.pass_build_settings_in_environment() { 1 } else { 0 }));
    
    let deps: Vec<PlistValue> = target.dependencies()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("dependencies".to_string(), PlistValue::Array(deps));
}

fn serialize_version_group(group: &XCVersionGroup, dict: &mut IndexMap<String, PlistValue>) {
    dict.insert("path".to_string(), PlistValue::String(group.path().to_string()));
    dict.insert("sourceTree".to_string(), PlistValue::String(group.source_tree().to_string()));
    
    let children: Vec<PlistValue> = group.children()
        .iter()
        .map(|h| PlistValue::String(h.to_string()))
        .collect();
    dict.insert("children".to_string(), PlistValue::Array(children));
    
    if let Some(current) = group.current_version() {
        dict.insert("currentVersion".to_string(), PlistValue::String(current.to_string()));
    }
    
    dict.insert("versionGroupType".to_string(), PlistValue::String(group.version_group_type().to_string()));
}

fn serialize_file_system_exception_set(exception_set: &PBXFileSystemSynchronizedBuildFileExceptionSet, dict: &mut IndexMap<String, PlistValue>) {
    // Serialize membershipExceptions array
    if !exception_set.membership_exceptions.is_empty() {
        let exceptions: Vec<PlistValue> = exception_set.membership_exceptions
            .iter()
            .map(|s| PlistValue::String(s.clone()))
            .collect();
        dict.insert("membershipExceptions".to_string(), PlistValue::Array(exceptions));
    }
    
    // Serialize target reference
    if let Some(target) = exception_set.target {
        dict.insert("target".to_string(), PlistValue::String(target.to_uuid_string()));
    }
}

fn serialize_file_system_synchronized_group(sync_group: &PBXFileSystemSynchronizedRootGroup, dict: &mut IndexMap<String, PlistValue>) {
    // Serialize path
    if let Some(path) = &sync_group.path {
        dict.insert("path".to_string(), PlistValue::String(path.clone()));
    }
    
    // Serialize sourceTree
    dict.insert("sourceTree".to_string(), PlistValue::String(sync_group.source_tree.clone()));
    
    // Serialize exceptions array
    if !sync_group.exceptions.is_empty() {
        let exceptions: Vec<PlistValue> = sync_group.exceptions
            .iter()
            .map(|id| PlistValue::String(id.to_uuid_string()))
            .collect();
        dict.insert("exceptions".to_string(), PlistValue::Array(exceptions));
    }
}
