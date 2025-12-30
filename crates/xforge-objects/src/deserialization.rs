//! Deserialization support for PBX objects from ASCII Plist format

use xforge_core::{ObjectId, Handle, Registry};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;

use crate::{
    pbx_project::PBXProject,
    pbx_target::PBXNativeTarget,
    pbx_file_reference::PBXFileReference,
    pbx_file_system_synchronized::{
        PBXFileSystemSynchronizedBuildFileExceptionSet,
        PBXFileSystemSynchronizedRootGroup,
    },
    pbx_group::PBXGroup,
    pbx_build_configuration::{XCBuildConfiguration, XCConfigurationList},
    pbx_build_phase::{
        PBXSourcesBuildPhase, PBXFrameworksBuildPhase, PBXResourcesBuildPhase,
        PBXShellScriptBuildPhase, PBXCopyFilesBuildPhase, PBXHeadersBuildPhase, PBXBuildFile,
    },
    pbx_target_dependency::PBXTargetDependency,
    pbx_container_item_proxy::PBXContainerItemProxy,
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
                    "PBXNativeTarget" => {
                        if let Ok(target) = deserialize_native_target(obj_dict) {
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
                    "PBXFileSystemSynchronizedBuildFileExceptionSet" => {
                        if let Ok(exception_set) = deserialize_file_system_exception_set(obj_dict) {
                            registry.register_with_id(obj_id, exception_set);
                        }
                    }
                    "PBXFileSystemSynchronizedRootGroup" => {
                        if let Ok(sync_group) = deserialize_file_system_synchronized_group(obj_dict) {
                            registry.register_with_id(obj_id, sync_group);
                        }
                    }
                    _ => {
                        // Skip unknown types for now
                        eprintln!("Warning: Skipping unknown object type: {}", isa);
                    }
                }
            }
        }
    }
    
    Ok((registry, root_id))
}

fn deserialize_project(dict: &IndexMap<String, PlistValue>) -> Result<PBXProject, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .unwrap_or("Unnamed")
        .to_string();
    
    let mut project = PBXProject::new(name);
    
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
    
    Ok(project)
}

fn deserialize_native_target(dict: &IndexMap<String, PlistValue>) -> Result<PBXNativeTarget, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("PBXNativeTarget missing name")?
        .to_string();
    
    let target = PBXNativeTarget::new(name);
    
    // TODO: Deserialize build phases, dependencies, etc.
    
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
    
    let mut file_ref = PBXFileReference::new(path.unwrap_or_default());
    file_ref.name = name;
    file_ref.source_tree = source_tree.unwrap_or("<group>".to_string());
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
    
    // Deserialize children
    if let Some(children_array) = dict.get("children").and_then(|v| v.as_array()) {
        for child_id_val in children_array {
            if let Some(id_str) = child_id_val.as_string() {
                if let Ok(child_id) = ObjectId::from_uuid_string(id_str) {
                    children.push(Handle::from_id(child_id));
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

fn deserialize_build_configuration(dict: &IndexMap<String, PlistValue>) -> Result<XCBuildConfiguration, String> {
    let name = dict.get("name")
        .and_then(|v| v.as_string())
        .ok_or("XCBuildConfiguration missing name")?
        .to_string();
    
    let mut build_settings = IndexMap::new();
    
    if let Some(settings) = dict.get("buildSettings").and_then(|v| v.as_dictionary()) {
        for (key, value) in settings {
            if let Some(val_str) = value.as_string() {
                build_settings.insert(key.clone(), val_str.to_string());
            } else if let Some(arr) = value.as_array() {
                // Handle array values by joining them
                let items: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_string())
                    .map(|s| s.to_string())
                    .collect();
                build_settings.insert(key.clone(), items.join(" "));
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
