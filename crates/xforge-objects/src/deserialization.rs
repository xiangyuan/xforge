//! Deserialization support for PBX objects from ASCII Plist format

use xforge_core::{ObjectId, Handle, Registry};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;

use crate::{
    pbx_project::PBXProject,
    pbx_target::PBXNativeTarget,
    pbx_file_reference::PBXFileReference,
    pbx_group::PBXGroup,
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
