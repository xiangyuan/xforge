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
        PBXShellScriptBuildPhase, PBXCopyFilesBuildPhase, PBXBuildFile,
    },
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
    } else if let Some(build_file) = any_obj.downcast_ref::<PBXBuildFile>() {
        serialize_build_file(build_file, &mut dict);
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
}

fn serialize_file_reference(file_ref: &PBXFileReference, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(path) = file_ref.path() {
        dict.insert("path".to_string(), PlistValue::String(path.to_string()));
    }
    
    if let Some(source_tree) = file_ref.source_tree() {
        dict.insert("sourceTree".to_string(), PlistValue::String(source_tree.to_string()));
    }
}

fn serialize_group(group: &PBXGroup, dict: &mut IndexMap<String, PlistValue>) {
    if let Some(path) = group.path() {
        dict.insert("path".to_string(), PlistValue::String(path.to_string()));
    }
    
    if let Some(source_tree) = group.source_tree() {
        dict.insert("sourceTree".to_string(), PlistValue::String(source_tree.to_string()));
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
