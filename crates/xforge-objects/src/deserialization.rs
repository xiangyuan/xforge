//! Deserialization support for PBX objects from ASCII Plist format

use xforge_core::{ObjectId, Handle, Registry};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;

use crate::{
    pbx_build_rule::PBXBuildRule,
    pbx_project::PBXProject,
    pbx_target::PBXNativeTarget,
    pbx_file_reference::PBXFileReference,
    pbx_file_system_synchronized::{
        PBXFileSystemSynchronizedBuildFileExceptionSet,
        PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet,
        PBXFileSystemSynchronizedRootGroup,
    },
    pbx_group::PBXGroup,
    pbx_variant_group::PBXVariantGroup,
    pbx_build_configuration::{XCBuildConfiguration, XCConfigurationList},
    pbx_build_phase::{
        PBXSourcesBuildPhase, PBXFrameworksBuildPhase, PBXResourcesBuildPhase, PBXRezBuildPhase,
        PBXShellScriptBuildPhase, PBXCopyFilesBuildPhase, PBXHeadersBuildPhase, PBXBuildFile,
    },
    pbx_aggregate_target::PBXAggregateTarget,
    pbx_target_dependency::PBXTargetDependency,
    pbx_container_item_proxy::PBXContainerItemProxy,
    pbx_legacy_target::PBXLegacyTarget,
    pbx_reference_proxy::PBXReferenceProxy,
    pbx_unknown::PBXUnknownObject,
    xc_swift_package::{XCLocalSwiftPackageReference, XCSwiftPackageProductDependency, XCRemoteSwiftPackageReference, PackageRequirement},
    xc_version_group::XCVersionGroup,
};

/// Deserialize the entire registry from PlistValue
pub fn deserialize_registry(plist: &PlistValue) -> Result<(Registry, ObjectId), String> {
    let root_dict = plist.as_dictionary()
        .ok_or("Root value must be a dictionary")?;
    
    // Get root object ID
    let root_id_str = root_dict.get("rootObject")
        .and_then(|v| v.as_string())
        .ok_or("Missing rootObject")?;
    
    let root_id = ObjectId::from_uuid_string(root_id_str)
        .map_err(|e| format!("Invalid rootObject ID: {}", e))?;
    
    // Get objects dictionary
    let objects = root_dict.get("objects")
        .and_then(|v| v.as_dictionary())
        .ok_or("Missing objects dictionary")?;
    
    let mut registry = Registry::new();
    
    // First pass: create all objects with basic data
    for (id_str, obj_plist) in objects {
        if let Some(obj_dict) = obj_plist.as_dictionary() {
            if let Some(isa) = obj_dict.get("isa").and_then(|v| v.as_string()) {
                let obj_id = ObjectId::from_uuid_string(id_str)
                    .map_err(|e| format!("Invalid object ID {}: {}", id_str, e))?;
                
                // Create object based on isa type
                match isa {
                    "PBXProject" => {
                        if let Ok(project) = deserialize_project(obj_dict) {
                            registry.register_with_id(obj_id, project);
                        }
                    }
                    "PBXAggregateTarget" => {
                        if let Ok(target) = deserialize_aggregate_target(obj_dict) {
                            registry.register_with_id(obj_id, target);
                        }
                    }
                    "PBXNativeTarget" => {
                        if let Ok(target) = deserialize_native_target(obj_dict) {
                            registry.register_with_id(obj_id, target);
                        }
                    }
                    "PBXLegacyTarget" => {
                        if let Ok(target) = deserialize_legacy_target(obj_dict) {
                            registry.register_with_id(obj_id, target);
                        }
                    }
                    "PBXFileReference" => {
                        if let Ok(file_ref) = deserialize_file_reference(obj_dict) {
                            registry.register_with_id(obj_id, file_ref);
                        }
                    }
                    "PBXGroup" => {
                        if let Ok(group) = deserialize_group(obj_dict) {
                            registry.register_with_id(obj_id, group);
                        }
                    }
                    "PBXVariantGroup" => {
                        if let Ok(variant_group) = deserialize_variant_group(obj_dict) {
                            registry.register_with_id(obj_id, variant_group);
                        }
                    }
                    "XCVersionGroup" => {
                        if let Ok(version_group) = deserialize_version_group(obj_dict) {
                            registry.register_with_id(obj_id, version_group);
                        }
                    }
                    "XCBuildConfiguration" => {
                        if let Ok(config) = deserialize_build_configuration(obj_dict) {
                            registry.register_with_id(obj_id, config);
                        }
                    }
                    "XCConfigurationList" => {
                        if let Ok(config_list) = deserialize_configuration_list(obj_dict) {
                            registry.register_with_id(obj_id, config_list);
                        }
                    }
                    "PBXSourcesBuildPhase" => {
                        if let Ok(phase) = deserialize_sources_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXFrameworksBuildPhase" => {
                        if let Ok(phase) = deserialize_frameworks_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXResourcesBuildPhase" => {
                        if let Ok(phase) = deserialize_resources_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXRezBuildPhase" => {
                        if let Ok(phase) = deserialize_rez_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXShellScriptBuildPhase" => {
                        if let Ok(phase) = deserialize_shell_script_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXCopyFilesBuildPhase" => {
                        if let Ok(phase) = deserialize_copy_files_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXHeadersBuildPhase" => {
                        if let Ok(phase) = deserialize_headers_build_phase(obj_dict) {
                            registry.register_with_id(obj_id, phase);
                        }
                    }
                    "PBXBuildRule" => {
                        if let Ok(rule) = deserialize_build_rule(obj_dict) {
                            registry.register_with_id(obj_id, rule);
                        }
                    }
                    "PBXBuildFile" => {
                        if let Ok(build_file) = deserialize_build_file(obj_dict) {
                            registry.register_with_id(obj_id, build_file);
                        }
                    }
                    "PBXTargetDependency" => {
                        if let Ok(dependency) = deserialize_target_dependency(obj_dict) {
                            registry.register_with_id(obj_id, dependency);
                        }
                    }
                    "PBXContainerItemProxy" => {
                        if let Ok(proxy) = deserialize_container_item_proxy(obj_dict) {
                            registry.register_with_id(obj_id, proxy);
                        }
                    }
                    "PBXReferenceProxy" => {
                        if let Ok(proxy) = deserialize_reference_proxy(obj_dict) {
                            registry.register_with_id(obj_id, proxy);
                        }
                    }
                    "PBXFileSystemSynchronizedBuildFileExceptionSet" => {
                        if let Ok(exception_set) = deserialize_file_system_exception_set(obj_dict) {
                            registry.register_with_id(obj_id, exception_set);
                        }
                    }
                    "PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet" => {
                        if let Ok(exception_set) = deserialize_file_system_group_exception_set(obj_dict) {
                            registry.register_with_id(obj_id, exception_set);
                        }
                    }
                    "PBXFileSystemSynchronizedRootGroup" => {
                        if let Ok(sync_group) = deserialize_file_system_synchronized_group(obj_dict) {
                            registry.register_with_id(obj_id, sync_group);
                        }
                    }
                    "XCLocalSwiftPackageReference" => {
                        if let Ok(local_ref) = deserialize_local_swift_package_reference(obj_dict) {
                            registry.register_with_id(obj_id, local_ref);
                        }
                    }
                    "XCRemoteSwiftPackageReference" => {
                        if let Ok(remote_ref) = deserialize_remote_swift_package_reference(obj_dict) {
                            registry.register_with_id(obj_id, remote_ref);
                        }
                    }
                    "XCSwiftPackageProductDependency" => {
                        if let Ok(product_dep) = deserialize_swift_package_product_dependency(obj_dict) {
                            registry.register_with_id(obj_id, product_dep);
                        }
                    }
                    _ => {
                        let mut fields = obj_dict.clone();
                        fields.remove("isa");
                        let unknown = PBXUnknownObject::new(obj_id.clone(), isa, fields);
                        registry.register_with_id(obj_id, unknown);
                        eprintln!("Warning: Preserved unknown object type: {}", isa);
                    }
                }
            }
        }
    }
    
    Ok((registry, root_id))
}

fn deserialize_aggregate_target(dict: &IndexMap<String, PlistValue>) -> Result<PBXAggregateTarget, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("PBXAggregateTarget missing name")?
        .to_string();
    let mut target = PBXAggregateTarget::new(name);

    if let Some(product_name) = dict.get("productName").and_then(|v| v.as_string()) {
        target.product_name = Some(product_name.to_string());
    }

    if let Some(config_list_str) = dict.get("buildConfigurationList").and_then(|v| v.as_string()) {
        if let Ok(config_list_id) = ObjectId::from_uuid_string(config_list_str) {
            target.build_configuration_list = Some(Handle::from_id(config_list_id));
        }
    }

    if let Some(phases_array) = dict.get("buildPhases").and_then(|v| v.as_array()) {
        for phase_val in phases_array {
            if let Some(phase_str) = phase_val.as_string() {
                if let Ok(phase_id) = ObjectId::from_uuid_string(phase_str) {
                    target.build_phases.push(Handle::from_id(phase_id));
                }
            }
        }
    }

    if let Some(deps_array) = dict.get("dependencies").and_then(|v| v.as_array()) {
        for dep_val in deps_array {
            if let Some(dep_str) = dep_val.as_string() {
                if let Ok(dep_id) = ObjectId::from_uuid_string(dep_str) {
                    target.dependencies.push(Handle::from_id(dep_id));
                }
            }
        }
    }

    Ok(target)
}

fn deserialize_project(dict: &IndexMap<String, PlistValue>) -> Result<PBXProject, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .unwrap_or("Unnamed")
        .to_string();
    
    let mut project = PBXProject::new(name);
    
    // Deserialize buildConfigurationList - CRITICAL!
    if let Some(config_list_str) = dict.get("buildConfigurationList").and_then(|v| v.as_string()) {
        if let Ok(config_list_id) = ObjectId::from_uuid_string(config_list_str) {
            project.build_configuration_list = Some(config_list_id);
        }
    }
    
    // Deserialize compatibility_version
    if let Some(compat_ver) = dict.get("compatibilityVersion").and_then(|v| v.as_string()) {
        project.compatibility_version = compat_ver.to_string();
    }
    
    // Deserialize developmentRegion
    if let Some(dev_region) = dict.get("developmentRegion").and_then(|v| v.as_string()) {
        project.development_region = dev_region.to_string();
    }
    
    // Deserialize hasScannedForEncodings
    if let Some(has_scanned) = dict.get("hasScannedForEncodings").and_then(|v| v.as_integer()) {
        project.has_scanned_for_encodings = has_scanned != 0;
    }
    
    // Deserialize knownRegions
    if let Some(regions_array) = dict.get("knownRegions").and_then(|v| v.as_array()) {
        project.known_regions.clear();
        for region_val in regions_array {
            if let Some(region) = region_val.as_string() {
                project.known_regions.push(region.to_string());
            }
        }
    }
    
    // Deserialize mainGroup
    if let Some(main_group_str) = dict.get("mainGroup").and_then(|v| v.as_string()) {
        if let Ok(main_group_id) = ObjectId::from_uuid_string(main_group_str) {
            project.main_group = Some(main_group_id);
        }
    }
    
    // Deserialize productRefGroup
    if let Some(product_ref_str) = dict.get("productRefGroup").and_then(|v| v.as_string()) {
        if let Ok(product_ref_id) = ObjectId::from_uuid_string(product_ref_str) {
            project.product_ref_group = Some(product_ref_id);
        }
    }
    
    // Deserialize projectDirPath
    if let Some(dir_path) = dict.get("projectDirPath").and_then(|v| v.as_string()) {
        project.project_dir_path = dir_path.to_string();
    }
    
    // Deserialize projectRoot
    if let Some(proj_root) = dict.get("projectRoot").and_then(|v| v.as_string()) {
        project.project_root = proj_root.to_string();
    }
    
    // Deserialize targets
    if let Some(targets_array) = dict.get("targets").and_then(|v| v.as_array()) {
        for target_id_val in targets_array {
            if let Some(id_str) = target_id_val.as_string() {
                if let Ok(target_id) = ObjectId::from_uuid_string(id_str) {
                    project.targets.push(target_id);
                }
            }
        }
    }
    
    // Deserialize packageReferences
    if let Some(pkg_refs_array) = dict.get("packageReferences").and_then(|v| v.as_array()) {
        for pkg_ref_val in pkg_refs_array {
            if let Some(id_str) = pkg_ref_val.as_string() {
                if let Ok(pkg_ref_id) = ObjectId::from_uuid_string(id_str) {
                    project.package_references.push(pkg_ref_id);
                }
            }
        }
    }
    
    // Deserialize attributes (supports nested dictionaries)
    if let Some(attrs_value) = dict.get("attributes") {
        if let Some(attrs_dict) = attrs_value.as_dictionary() {
            for (key, value) in attrs_dict {
                project.attributes.insert(key.clone(), value.clone());
            }
        }
    }
    
    Ok(project)
}

fn deserialize_native_target(dict: &IndexMap<String, PlistValue>) -> Result<PBXNativeTarget, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("PBXNativeTarget missing name")?
        .to_string();
    
    let mut target = PBXNativeTarget::new(name);
    
    // Deserialize buildConfigurationList
    if let Some(config_list_str) = dict.get("buildConfigurationList").and_then(|v| v.as_string()) {
        if let Ok(config_list_id) = ObjectId::from_uuid_string(config_list_str) {
            target.build_configuration_list = Some(config_list_id);
        }
    }
    
    // Deserialize buildPhases
    if let Some(phases_array) = dict.get("buildPhases").and_then(|v| v.as_array()) {
        for phase_val in phases_array {
            if let Some(phase_str) = phase_val.as_string() {
                if let Ok(phase_id) = ObjectId::from_uuid_string(phase_str) {
                    target.build_phases.push(phase_id);
                }
            }
        }
    }
    
    // Deserialize buildRules
    if let Some(rules_array) = dict.get("buildRules").and_then(|v| v.as_array()) {
        for rule_val in rules_array {
            if let Some(rule_str) = rule_val.as_string() {
                if let Ok(rule_id) = ObjectId::from_uuid_string(rule_str) {
                    target.build_rules.push(rule_id);
                }
            }
        }
    }
    
    // Deserialize dependencies
    if let Some(deps_array) = dict.get("dependencies").and_then(|v| v.as_array()) {
        for dep_val in deps_array {
            if let Some(dep_str) = dep_val.as_string() {
                if let Ok(dep_id) = ObjectId::from_uuid_string(dep_str) {
                    target.dependencies.push(dep_id);
                }
            }
        }
    }
    
    // Deserialize productName
    if let Some(product_name) = dict.get("productName").and_then(|v| v.as_string()) {
        target.product_name = Some(product_name.to_string());
    }
    
    // Deserialize productReference
    if let Some(product_ref_str) = dict.get("productReference").and_then(|v| v.as_string()) {
        if let Ok(product_ref_id) = ObjectId::from_uuid_string(product_ref_str) {
            target.product_reference = Some(product_ref_id);
        }
    }
    
    // Deserialize productType
    if let Some(product_type_str) = dict.get("productType").and_then(|v| v.as_string()) {
        if let Some(product_type) = xforge_core::ProductType::from_string(product_type_str) {
            target.product_type = Some(product_type);
        }
    }
    
    // Deserialize packageProductDependencies
    if let Some(pkg_deps_array) = dict.get("packageProductDependencies").and_then(|v| v.as_array()) {
        for pkg_dep_val in pkg_deps_array {
            if let Some(pkg_dep_str) = pkg_dep_val.as_string() {
                if let Ok(pkg_dep_id) = ObjectId::from_uuid_string(pkg_dep_str) {
                    target.package_product_dependencies.push(pkg_dep_id);
                }
            }
        }
    }
    
    Ok(target)
}

fn deserialize_legacy_target(dict: &IndexMap<String, PlistValue>) -> Result<PBXLegacyTarget, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("PBXLegacyTarget missing name")?
        .to_string();
    let build_tool_path = dict.get("buildToolPath")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();

    let mut target = PBXLegacyTarget::new(name, build_tool_path);

    if let Some(product_name) = dict.get("productName").and_then(|v| v.as_string()) {
        target.product_name = Some(product_name.to_string());
    }
    if let Some(config_list_str) = dict.get("buildConfigurationList").and_then(|v| v.as_string()) {
        if let Ok(config_list_id) = ObjectId::from_uuid_string(config_list_str) {
            target.build_configuration_list = Some(Handle::from_id(config_list_id));
        }
    }
    if let Some(args) = dict.get("buildArgumentsString").and_then(|v| v.as_string()) {
        target.build_arguments_string = Some(args.to_string());
    }
    if let Some(working_dir) = dict.get("buildWorkingDirectory").and_then(|v| v.as_string()) {
        target.build_working_directory = Some(working_dir.to_string());
    }
    if let Some(pass) = dict.get("passBuildSettingsInEnvironment").and_then(plist_bool) {
        target.pass_build_settings_in_environment = pass;
    }
    if let Some(deps_array) = dict.get("dependencies").and_then(|v| v.as_array()) {
        for dep_val in deps_array {
            if let Some(dep_str) = dep_val.as_string() {
                if let Ok(dep_id) = ObjectId::from_uuid_string(dep_str) {
                    target.dependencies.push(Handle::from_id(dep_id));
                }
            }
        }
    }

    Ok(target)
}

fn deserialize_file_reference(dict: &IndexMap<String, PlistValue>) -> Result<PBXFileReference, String> {
    let path = dict.get("path")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let source_tree = dict.get("sourceTree")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let last_known_file_type = dict.get("lastKnownFileType")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let explicit_file_type = dict.get("explicitFileType")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let file_encoding = dict.get("fileEncoding")
        .and_then(|v| v.as_integer())
        .map(|i| i as u32);
    
    let mut file_ref = PBXFileReference::new(path.unwrap_or_default());
    file_ref.name = name;
    file_ref.source_tree = source_tree.unwrap_or("<group>".to_string());
    file_ref.last_known_file_type = last_known_file_type;
    file_ref.explicit_file_type = explicit_file_type;
    file_ref.file_encoding = file_encoding;
    
    Ok(file_ref)
}

fn deserialize_group(dict: &IndexMap<String, PlistValue>) -> Result<PBXGroup, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let path = dict.get("path")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let source_tree = dict.get("sourceTree")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut children = Vec::new();
    
    // Deserialize children (ObjectIds, not Handles - can be any type)
    if let Some(children_array) = dict.get("children").and_then(|v| v.as_array()) {
        for child_id_val in children_array {
            if let Some(id_str) = child_id_val.as_string() {
                if let Ok(child_id) = ObjectId::from_uuid_string(id_str) {
                    children.push(child_id);
                }
            }
        }
    }
    
    let mut group = PBXGroup::new(name.unwrap_or_default());
    group.path = path;
    group.source_tree = source_tree.unwrap_or("<group>".to_string());
    group.children = children;
    Ok(group)
}

fn deserialize_variant_group(dict: &IndexMap<String, PlistValue>) -> Result<PBXVariantGroup, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let source_tree = dict.get("sourceTree")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut children = Vec::new();
    
    // Deserialize children (file references for localized variants)
    if let Some(children_array) = dict.get("children").and_then(|v| v.as_array()) {
        for child_id_val in children_array {
            if let Some(id_str) = child_id_val.as_string() {
                if let Ok(child_id) = ObjectId::from_uuid_string(id_str) {
                    children.push(Handle::from_id(child_id));
                }
            }
        }
    }
    
    let mut variant_group = PBXVariantGroup::new(name.unwrap_or_default());
    variant_group.source_tree = source_tree.unwrap_or("<group>".to_string());
    variant_group.children = children;
    Ok(variant_group)
}

fn deserialize_build_configuration(dict: &IndexMap<String, PlistValue>) -> Result<XCBuildConfiguration, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("XCBuildConfiguration missing name")?
        .to_string();
    
    let mut build_settings = IndexMap::new();
    
    if let Some(settings) = dict.get("buildSettings").and_then(|v| v.as_dictionary()) {
        for (key, value) in settings {
            match value {
                PlistValue::String(s) => {
                    build_settings.insert(key.clone(), s.clone());
                }
                PlistValue::Boolean(b) => {
                    // Convert boolean to YES/NO string (Xcode format)
                    build_settings.insert(key.clone(), if *b { "YES".to_string() } else { "NO".to_string() });
                }
                PlistValue::Integer(i) => {
                    build_settings.insert(key.clone(), i.to_string());
                }
                PlistValue::Array(arr) => {
                    // Handle array values by joining them
                    let items: Vec<String> = arr.iter()
                        .filter_map(|v| v.as_string())
                        .map(|s| s.to_string())
                        .collect();
                    build_settings.insert(key.clone(), items.join(" "));
                }
                _ => {
                    // Skip other types (dictionaries, etc.)
                }
            }
        }
    }
    
    let mut config = XCBuildConfiguration::new(name);
    config.build_settings = build_settings;
    Ok(config)
}

fn deserialize_configuration_list(dict: &IndexMap<String, PlistValue>) -> Result<XCConfigurationList, String> {
    let mut build_configurations = Vec::new();
    
    if let Some(configs_array) = dict.get("buildConfigurations").and_then(|v| v.as_array()) {
        for config_id_val in configs_array {
            if let Some(id_str) = config_id_val.as_string() {
                if let Ok(config_id) = ObjectId::from_uuid_string(id_str) {
                    build_configurations.push(Handle::from_id(config_id));
                }
            }
        }
    }
    
    let default_config_name = dict.get("defaultConfigurationName")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut config_list = XCConfigurationList::new();
    config_list.build_configurations = build_configurations;
    config_list.default_configuration_name = default_config_name;
    Ok(config_list)
}

fn deserialize_sources_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXSourcesBuildPhase, String> {
    let mut files = Vec::new();
    
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    
    let mut phase = PBXSourcesBuildPhase::new();
    phase.files = files;
    Ok(phase)
}

fn deserialize_frameworks_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXFrameworksBuildPhase, String> {
    let mut files = Vec::new();
    
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    
    let mut phase = PBXFrameworksBuildPhase::new();
    phase.files = files;
    Ok(phase)
}

fn deserialize_resources_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXResourcesBuildPhase, String> {
    let mut files = Vec::new();
    
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    
    let mut phase = PBXResourcesBuildPhase::new();
    phase.files = files;
    Ok(phase)
}

fn deserialize_shell_script_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXShellScriptBuildPhase, String> {
    let shell_script = dict.get("shellScript")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut phase = PBXShellScriptBuildPhase::new(shell_script);
    phase.name = name;
    
    // Parse input paths
    if let Some(inputs_array) = dict.get("inputPaths").and_then(|v| v.as_array()) {
        phase.input_paths = inputs_array.iter()
            .filter_map(|v| v.as_string())
            .map(|s| s.to_string())
            .collect();
    }
    
    // Parse output paths
    if let Some(outputs_array) = dict.get("outputPaths").and_then(|v| v.as_array()) {
        phase.output_paths = outputs_array.iter()
            .filter_map(|v| v.as_string())
            .map(|s| s.to_string())
            .collect();
    }
    
    Ok(phase)
}

fn deserialize_copy_files_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXCopyFilesBuildPhase, String> {
    let mut files = Vec::new();
    
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    
    let dst_path = dict.get("dstPath")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    
    let dst_subfolder_spec = dict.get("dstSubfolderSpec")
        .and_then(|v| v.as_string())
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(0);
    
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut phase = PBXCopyFilesBuildPhase::new(dst_path, dst_subfolder_spec);
    phase.files = files;
    phase.name = name;
    Ok(phase)
}

fn deserialize_headers_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXHeadersBuildPhase, String> {
    let mut files = Vec::new();
    
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    
    let build_action_mask = dict.get("buildActionMask")
        .and_then(|v| v.as_integer())
        .unwrap_or(2147483647) as u32;
    let run_only_for_deployment_postprocessing = dict.get("runOnlyForDeploymentPostprocessing")
        .and_then(|v| v.as_integer())
        .map(|i| i != 0)
        .unwrap_or(false);
    
    let mut phase = PBXHeadersBuildPhase::new();
    phase.files = files;
    phase.build_action_mask = build_action_mask;
    phase.run_only_for_deployment_postprocessing = run_only_for_deployment_postprocessing;
    Ok(phase)
}

fn deserialize_rez_build_phase(dict: &IndexMap<String, PlistValue>) -> Result<PBXRezBuildPhase, String> {
    let mut files = Vec::new();
    if let Some(files_array) = dict.get("files").and_then(|v| v.as_array()) {
        for file_id_val in files_array {
            if let Some(id_str) = file_id_val.as_string() {
                if let Ok(file_id) = ObjectId::from_uuid_string(id_str) {
                    files.push(Handle::from_id(file_id));
                }
            }
        }
    }
    let build_action_mask = dict.get("buildActionMask")
        .and_then(|v| v.as_integer())
        .unwrap_or(2147483647) as u32;
    let run_only_for_deployment_postprocessing = dict.get("runOnlyForDeploymentPostprocessing")
        .and_then(|v| v.as_integer())
        .map(|i| i != 0)
        .unwrap_or(false);

    let mut phase = PBXRezBuildPhase::new();
    phase.files = files;
    phase.build_action_mask = build_action_mask;
    phase.run_only_for_deployment_postprocessing = run_only_for_deployment_postprocessing;
    Ok(phase)
}

fn deserialize_build_rule(dict: &IndexMap<String, PlistValue>) -> Result<PBXBuildRule, String> {
    let compiler_spec = dict.get("compilerSpec")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    let file_type = dict.get("fileType")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    let mut rule = PBXBuildRule::new(compiler_spec, file_type);

    rule.file_patterns = dict.get("filePatterns").and_then(|v| v.as_string()).map(|s| s.to_string());
    rule.name = dict.get("name").and_then(|v| v.as_string()).map(|s| s.to_string());
    rule.dependency_file = dict.get("dependencyFile").and_then(|v| v.as_string()).map(|s| s.to_string());
    rule.is_editable = dict.get("isEditable").and_then(plist_bool).unwrap_or(true);
    rule.script = dict.get("script").and_then(|v| v.as_string()).map(|s| s.to_string());
    rule.run_once_per_architecture = dict.get("runOncePerArchitecture").and_then(plist_bool);

    if let Some(outputs) = dict.get("outputFiles").and_then(|v| v.as_array()) {
        rule.output_files = outputs.iter().filter_map(|v| v.as_string()).map(|s| s.to_string()).collect();
    }
    if let Some(inputs) = dict.get("inputFiles").and_then(|v| v.as_array()) {
        rule.input_files = Some(inputs.iter().filter_map(|v| v.as_string()).map(|s| s.to_string()).collect());
    }
    if let Some(flags) = dict.get("outputFilesCompilerFlags").and_then(|v| v.as_array()) {
        rule.output_files_compiler_flags = Some(flags.iter().filter_map(|v| v.as_string()).map(|s| s.to_string()).collect());
    }

    Ok(rule)
}

fn deserialize_build_file(dict: &IndexMap<String, PlistValue>) -> Result<PBXBuildFile, String> {
    let file_ref_id = dict.get("fileRef")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok())
        .ok_or("PBXBuildFile missing fileRef")?;
    
    let file_ref = Handle::from_id(file_ref_id);
    let mut build_file = PBXBuildFile::new(file_ref);
    
    // Parse settings if present
    if let Some(settings_dict) = dict.get("settings").and_then(|v| v.as_dictionary()) {
        let mut settings = std::collections::HashMap::new();
        for (key, value) in settings_dict {
            if let Some(val_str) = value.as_string() {
                settings.insert(key.clone(), val_str.to_string());
            }
        }
        build_file.settings = Some(settings);
    }
    
    Ok(build_file)
}

fn deserialize_target_dependency(dict: &IndexMap<String, PlistValue>) -> Result<PBXTargetDependency, String> {
    let target = dict.get("target")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok());
    
    let target_proxy = dict.get("targetProxy")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok());
    
    let mut dependency = PBXTargetDependency::new();
    dependency.target = target;
    dependency.target_proxy = target_proxy;
    Ok(dependency)
}

fn deserialize_reference_proxy(dict: &IndexMap<String, PlistValue>) -> Result<PBXReferenceProxy, String> {
    let path = dict.get("path").and_then(|v| v.as_string()).unwrap_or("").to_string();
    let file_type = dict.get("fileType").and_then(|v| v.as_string()).unwrap_or("").to_string();
    let remote_ref = dict.get("remoteRef")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok())
        .ok_or("PBXReferenceProxy missing remoteRef")?;
    let mut proxy = PBXReferenceProxy::new(path, file_type, remote_ref);
    if let Some(source_tree) = dict.get("sourceTree").and_then(|v| v.as_string()) {
        proxy.source_tree = source_tree.to_string();
    }
    Ok(proxy)
}

fn deserialize_container_item_proxy(dict: &IndexMap<String, PlistValue>) -> Result<PBXContainerItemProxy, String> {
    let container_portal = dict.get("containerPortal")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok())
        .ok_or("PBXContainerItemProxy missing containerPortal")?;
    
    let proxy_type = dict.get("proxyType")
        .and_then(|v| v.as_string())
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);
    
    let remote_global_id_string = dict.get("remoteGlobalIDString")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let remote_info = dict.get("remoteInfo")
        .and_then(|v| v.as_string())
        .map(|s| s.to_string());
    
    let mut proxy = PBXContainerItemProxy::new(container_portal, proxy_type);
    proxy.remote_global_id_string = remote_global_id_string;
    proxy.remote_info = remote_info;
    Ok(proxy)
}

fn deserialize_file_system_exception_set(dict: &IndexMap<String, PlistValue>) -> Result<PBXFileSystemSynchronizedBuildFileExceptionSet, String> {
    let mut exception_set = PBXFileSystemSynchronizedBuildFileExceptionSet::new();
    
    // Parse membershipExceptions array
    if let Some(exceptions_array) = dict.get("membershipExceptions").and_then(|v| v.as_array()) {
        exception_set.membership_exceptions = exceptions_array.iter()
            .filter_map(|v| v.as_string())
            .map(|s| s.to_string())
            .collect();
    }
    
    // Parse target reference
    exception_set.target = dict.get("target")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok());
    
    Ok(exception_set)
}

fn deserialize_file_system_group_exception_set(dict: &IndexMap<String, PlistValue>) -> Result<PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet, String> {
    let build_phase = dict.get("buildPhase")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok())
        .ok_or("PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet missing buildPhase")?;
    let mut exception_set = PBXFileSystemSynchronizedGroupBuildPhaseMembershipExceptionSet::new(build_phase);

    if let Some(exceptions) = dict.get("membershipExceptions").and_then(|v| v.as_array()) {
        exception_set.membership_exceptions = exceptions.iter()
            .filter_map(|v| v.as_string())
            .map(|s| s.to_string())
            .collect();
    }

    if let Some(attrs_dict) = dict.get("attributesByRelativePath").and_then(|v| v.as_dictionary()) {
        for (key, value) in attrs_dict {
            if let Some(values) = value.as_array() {
                let list = values.iter().filter_map(|v| v.as_string()).map(|s| s.to_string()).collect();
                exception_set.attributes_by_relative_path.insert(key.clone(), list);
            }
        }
    }

    Ok(exception_set)
}

fn deserialize_file_system_synchronized_group(dict: &IndexMap<String, PlistValue>) -> Result<PBXFileSystemSynchronizedRootGroup, String> {
    let path = dict.get("path")
        .and_then(|v| v.as_string())
        .ok_or("PBXFileSystemSynchronizedRootGroup missing path")?
        .to_string();
    
    let mut sync_group = PBXFileSystemSynchronizedRootGroup::new(path);
    
    // Parse sourceTree
    if let Some(source_tree) = dict.get("sourceTree").and_then(|v| v.as_string()) {
        sync_group.source_tree = source_tree.to_string();
    }
    
    // Parse exceptions array
    if let Some(exceptions_array) = dict.get("exceptions").and_then(|v| v.as_array()) {
        sync_group.exceptions = exceptions_array.iter()
            .filter_map(|v| v.as_string())
            .filter_map(|s| ObjectId::from_uuid_string(s).ok())
            .collect();
    }
    
    Ok(sync_group)
}

fn deserialize_local_swift_package_reference(dict: &IndexMap<String, PlistValue>) -> Result<XCLocalSwiftPackageReference, String> {
    let relative_path = dict.get("relativePath")
        .and_then(|v| v.as_string())
        .ok_or("XCLocalSwiftPackageReference missing relativePath")?
        .to_string();
    Ok(XCLocalSwiftPackageReference::new(relative_path))
}

fn deserialize_remote_swift_package_reference(dict: &IndexMap<String, PlistValue>) -> Result<XCRemoteSwiftPackageReference, String> {
    let repository_url = dict.get("repositoryURL")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();

    let requirement = dict.get("requirement")
        .and_then(|v| v.as_dictionary())
        .and_then(|req| parse_package_requirement(req));

    Ok(XCRemoteSwiftPackageReference::new(repository_url, requirement))
}

fn deserialize_swift_package_product_dependency(dict: &IndexMap<String, PlistValue>) -> Result<XCSwiftPackageProductDependency, String> {
    let raw_name = dict.get("productName")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    let (product_name, is_plugin) = if let Some(stripped) = raw_name.strip_prefix("plugin:") {
        (stripped.to_string(), true)
    } else {
        (raw_name, false)
    };
    let package = dict.get("package")
        .and_then(|v| v.as_string())
        .and_then(|s| ObjectId::from_uuid_string(s).ok());
    let mut dep = XCSwiftPackageProductDependency::new(package, product_name);
    dep.is_plugin = is_plugin;
    Ok(dep)
}

fn deserialize_version_group(dict: &IndexMap<String, PlistValue>) -> Result<XCVersionGroup, String> {
    let path = dict.get("path")
        .and_then(|v| v.as_string())
        .unwrap_or("")
        .to_string();
    let mut group = XCVersionGroup::new(path);
    if let Some(source_tree) = dict.get("sourceTree").and_then(|v| v.as_string()) {
        group.source_tree = source_tree.to_string();
    }
    if let Some(children_array) = dict.get("children").and_then(|v| v.as_array()) {
        for child_val in children_array {
            if let Some(child_str) = child_val.as_string() {
                if let Ok(child_id) = ObjectId::from_uuid_string(child_str) {
                    group.children.push(Handle::from_id(child_id));
                }
            }
        }
    }
    if let Some(current_str) = dict.get("currentVersion").and_then(|v| v.as_string()) {
        if let Ok(current_id) = ObjectId::from_uuid_string(current_str) {
            group.current_version = Some(Handle::from_id(current_id));
        }
    }
    if let Some(group_type) = dict.get("versionGroupType").and_then(|v| v.as_string()) {
        group.version_group_type = group_type.to_string();
    }
    Ok(group)
}

fn parse_package_requirement(dict: &IndexMap<String, PlistValue>) -> Option<PackageRequirement> {
    let kind = dict.get("kind").and_then(|v| v.as_string())?;
    match kind {
        "upToNextMajorVersion" => dict.get("minimumVersion")
            .and_then(|v| v.as_string())
            .map(|s| PackageRequirement::UpToNextMajorVersion(s.to_string())),
        "upToNextMinorVersion" => dict.get("minimumVersion")
            .and_then(|v| v.as_string())
            .map(|s| PackageRequirement::UpToNextMinorVersion(s.to_string())),
        "versionRange" => {
            let min = dict.get("minimumVersion").and_then(|v| v.as_string())?;
            let max = dict.get("maximumVersion").and_then(|v| v.as_string())?;
            Some(PackageRequirement::Range { from: min.to_string(), to: max.to_string() })
        }
        "exactVersion" => dict.get("version").and_then(|v| v.as_string())
            .map(|s| PackageRequirement::Exact(s.to_string())),
        "branch" => dict.get("branch").and_then(|v| v.as_string())
            .map(|s| PackageRequirement::Branch(s.to_string())),
        "revision" => dict.get("revision").and_then(|v| v.as_string())
            .map(|s| PackageRequirement::Revision(s.to_string())),
        _ => None,
    }
}

fn plist_bool(value: &PlistValue) -> Option<bool> {
    match value {
        PlistValue::Boolean(b) => Some(*b),
        PlistValue::Integer(i) => Some(*i != 0),
        PlistValue::String(s) => match s.as_str() {
            "YES" | "yes" | "true" | "1" => Some(true),
            "NO" | "no" | "false" | "0" => Some(false),
            _ => None,
        },
        _ => None,
    }
}
