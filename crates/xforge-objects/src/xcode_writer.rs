//! Xcode-specific serialization with proper formatting
//!
//! This module generates Xcode project files with:
//! - Section comments (/* Begin PBXBuildFile section */)
//! - Object name comments (uuid /* name */)  
//! - Proper field ordering (isa always first)
//! - UUID reference comments

use crate::*;
use xforge_core::{Registry, PBXObject};
use xforge_serialization::PlistValue;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Write a complete Xcode project file with proper formatting
pub fn write_xcode_project(registry: &Registry, root_id: &str) -> Result<String, String> {
    let mut output = String::new();
    
    // Build UUID comment cache
    let uuid_comments = build_uuid_comment_cache(registry);
    
    // 1. UTF-8 magic marker (REQUIRED by Xcode)
    output.push_str("// !$*UTF8*$!\n");
    
    // 2. Root dictionary opening
    output.push_str("{\n");
    
    // 3. Archive version
    output.push_str("\tarchiveVersion = 1;\n");
    
    // 4. Empty classes
    output.push_str("\tclasses = {\n");
    output.push_str("\t};\n");
    
    // 5. Object version  
    output.push_str("\tobjectVersion = 56;\n");
    
    // 6. Objects section with grouping
    output.push_str("\tobjects = {\n");
    output.push_str("\n");
    
    write_objects_with_sections(&mut output, registry, &uuid_comments)?;
    
    output.push_str("\t};\n");
    
    // 7. Root object with comment
    let root_id_obj = xforge_core::ObjectId::from_uuid_string(root_id)
        .map_err(|e| format!("Invalid root ID: {}", e))?;
    if let Some(obj) = registry.get::<PBXProject>(&root_id_obj) {
        if let Some(comment) = get_object_comment(obj, registry) {
            output.push_str(&format!("\trootObject = {} /* {} */;\n", root_id, comment));
        } else {
            output.push_str(&format!("\trootObject = {};\n", root_id));
        }
    } else {
        output.push_str(&format!("\trootObject = {};\n", root_id));
    }
    
    // 8. Close root dictionary
    output.push_str("}\n");
    
    Ok(output)
}

/// Build a cache of UUID to comment mappings
fn build_uuid_comment_cache(registry: &Registry) -> HashMap<String, String> {
    let mut cache = HashMap::new();
    
    for (id, obj) in registry.iter() {
        // Special handling for PBXBuildFile to add build phase suffix
        if obj.isa() == "PBXBuildFile" {
            if let Some(comment) = get_build_file_comment_with_phase(id, obj.as_ref(), registry) {
                cache.insert(id.clone(), comment);
            }
        } else if obj.isa() == "XCConfigurationList" {
            // Special handling for XCConfigurationList to add target/project name
            if let Some(comment) = get_config_list_comment(id, registry) {
                cache.insert(id.clone(), comment);
            }
        } else if let Some(comment) = get_object_comment(obj.as_ref(), registry) {
            cache.insert(id.clone(), comment);
        }
    }
    
    cache
}

/// Get comment for PBXBuildFile with build phase suffix
fn get_build_file_comment_with_phase(build_file_id: &str, obj: &dyn PBXObject, registry: &Registry) -> Option<String> {
    use std::any::Any;
    let any_obj = obj as &dyn Any;
    
    if let Some(build_file) = any_obj.downcast_ref::<PBXBuildFile>() {
        // Get the file name from fileRef (could be PBXFileReference or PBXVariantGroup)
        if let Ok(file_ref_id) = xforge_core::ObjectId::from_uuid_string(&build_file.file_ref.to_string()) {
            // Try PBXFileReference first
            let filename = if let Some(file_ref) = registry.get::<PBXFileReference>(&file_ref_id) {
                file_ref.name.clone().or_else(|| file_ref.path.clone())
            } else if let Some(variant_group) = registry.get::<PBXVariantGroup>(&file_ref_id) {
                // Handle PBXVariantGroup (like InfoPlist.strings)
                variant_group.name.clone()
            } else {
                None
            };
            
            if let Some(filename) = filename {
                // Find which build phase this build file belongs to
                if let Some(phase_name) = find_build_phase_for_file(build_file_id, registry) {
                    return Some(format!("{} in {}", filename, phase_name));
                }
                
                return Some(filename);
            }
        }
    }
    None
}

/// Group objects by type and write with section comments
fn write_objects_with_sections(output: &mut String, registry: &Registry, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    // Group objects by ISA type
    let mut grouped: HashMap<String, Vec<(String, &dyn PBXObject)>> = HashMap::new();
    
    for (id, obj) in registry.iter() {
        let isa = obj.isa();
        grouped.entry(isa.to_string())
            .or_insert_with(Vec::new)
            .push((id.clone(), obj.as_ref()));
    }
    
    // Sort ISA types alphabetically
    let mut isa_types: Vec<String> = grouped.keys().cloned().collect();
    isa_types.sort();
    
    // Write each section
    for isa in isa_types {
        let mut objects_in_section = grouped.remove(&isa).unwrap();
        
        // Sort objects by ID within section
        objects_in_section.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Section header
        output.push_str(&format!("/* Begin {} section */\n", isa));
        
        // Write each object
        for (id, obj) in objects_in_section {
            write_object(output, &id, obj, registry, uuid_comments)?;
        }
        
        // Section footer
        output.push_str(&format!("/* End {} section */\n", isa));
        output.push_str("\n");
    }
    
    Ok(())
}

/// Write a single object with proper formatting
fn write_object(output: &mut String, id: &str, obj: &dyn PBXObject, registry: &Registry, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    let isa = obj.isa();
    // Use the comment from the cache (built by build_uuid_comment_cache)
    let comment = uuid_comments.get(id);
    
    // Determine if this object should be flat (single line)
    let is_flat = matches!(isa, "PBXBuildFile" | "PBXFileReference");
    
    // Object opening with optional comment
    if let Some(comment_text) = comment {
        output.push_str(&format!("\t\t{} /* {} */ = {{", id, comment_text));
    } else {
        output.push_str(&format!("\t\t{} = {{", id));
    }
    
    // Serialize object to dictionary
    let obj_dict = serialize_object_to_dict(obj, registry)?;
    
    if is_flat {
        write_dict_flat(output, &obj_dict, uuid_comments)?;
        output.push_str(" };\n");
    } else {
        output.push_str("\n");
        write_dict_multiline(output, &obj_dict, 3, uuid_comments)?;
        output.push_str("\t\t};\n");
    }
    
    Ok(())
}

/// Serialize an object to a dictionary (reusing existing serialization)
fn serialize_object_to_dict(obj: &dyn PBXObject, registry: &Registry) -> Result<IndexMap<String, PlistValue>, String> {
    use std::any::Any;
    
    let mut dict = IndexMap::new();
    dict.insert("isa".to_string(), PlistValue::String(obj.isa().to_string()));
    
    let any_obj = obj as &dyn Any;
    
    // Use existing serialization functions
    if let Some(project) = any_obj.downcast_ref::<PBXProject>() {
        crate::serialization::serialize_project(project, &mut dict);
    } else if let Some(target) = any_obj.downcast_ref::<PBXNativeTarget>() {
        crate::serialization::serialize_target(target, &mut dict);
    } else if let Some(file_ref) = any_obj.downcast_ref::<PBXFileReference>() {
        crate::serialization::serialize_file_reference(file_ref, &mut dict);
    } else if let Some(group) = any_obj.downcast_ref::<PBXGroup>() {
        crate::serialization::serialize_group(group, &mut dict);
    } else if let Some(config) = any_obj.downcast_ref::<XCBuildConfiguration>() {
        crate::serialization::serialize_build_configuration(config, &mut dict);
    } else if let Some(config_list) = any_obj.downcast_ref::<XCConfigurationList>() {
        crate::serialization::serialize_configuration_list(config_list, &mut dict);
    } else if let Some(sources) = any_obj.downcast_ref::<PBXSourcesBuildPhase>() {
        crate::serialization::serialize_build_phase_common(&sources.files, sources.build_action_mask, sources.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(frameworks) = any_obj.downcast_ref::<PBXFrameworksBuildPhase>() {
        crate::serialization::serialize_build_phase_common(&frameworks.files, frameworks.build_action_mask, frameworks.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(resources) = any_obj.downcast_ref::<PBXResourcesBuildPhase>() {
        crate::serialization::serialize_build_phase_common(&resources.files, resources.build_action_mask, resources.run_only_for_deployment_postprocessing, &mut dict);
    } else if let Some(shell) = any_obj.downcast_ref::<PBXShellScriptBuildPhase>() {
        crate::serialization::serialize_shell_script_phase(shell, &mut dict);
    } else if let Some(copy) = any_obj.downcast_ref::<PBXCopyFilesBuildPhase>() {
        crate::serialization::serialize_copy_files_phase(copy, &mut dict);
    } else if let Some(headers) = any_obj.downcast_ref::<PBXHeadersBuildPhase>() {
        crate::serialization::serialize_headers_phase(headers, &mut dict);
    } else if let Some(build_file) = any_obj.downcast_ref::<PBXBuildFile>() {
        crate::serialization::serialize_build_file(build_file, &mut dict);
    } else if let Some(proxy) = any_obj.downcast_ref::<PBXContainerItemProxy>() {
        crate::serialization::serialize_container_item_proxy(proxy, &mut dict);
    } else if let Some(dependency) = any_obj.downcast_ref::<PBXTargetDependency>() {
        crate::serialization::serialize_target_dependency(dependency, &mut dict);
    } else if let Some(variant_group) = any_obj.downcast_ref::<PBXVariantGroup>() {
        crate::serialization::serialize_variant_group(variant_group, &mut dict);
    } else if let Some(ref_proxy) = any_obj.downcast_ref::<PBXReferenceProxy>() {
        crate::serialization::serialize_reference_proxy(ref_proxy, &mut dict);
    }
    
    Ok(dict)
}

/// Write dictionary in flat format (single line)
fn write_dict_flat(output: &mut String, dict: &IndexMap<String, PlistValue>, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    // Sort keys alphabetically
    let mut keys: Vec<&String> = dict.keys().collect();
    keys.sort();
    
    for (i, key) in keys.iter().enumerate() {
        let value = dict.get(*key).unwrap();
        
        if needs_quotes(key) {
            output.push_str(&format!("\"{}\" = ", key));
        } else {
            output.push_str(&format!("{} = ", key));
        }
        
        write_value_flat(output, value, uuid_comments)?;
        output.push_str(";");
        
        if i < keys.len() - 1 {
            output.push_str(" ");
        }
    }
    
    Ok(())
}

/// Write dictionary in multiline format
fn write_dict_multiline(output: &mut String, dict: &IndexMap<String, PlistValue>, indent: usize, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    // Sort keys alphabetically, but ALWAYS put 'isa' first (Xcode requirement)
    let mut keys: Vec<&String> = dict.keys().collect();
    keys.sort_by(|a, b| {
        if a.as_str() == "isa" {
            std::cmp::Ordering::Less
        } else if b.as_str() == "isa" {
            std::cmp::Ordering::Greater
        } else {
            a.cmp(b)
        }
    });
    
    for key in keys {
        let value = dict.get(key).unwrap();
        
        // Indent
        for _ in 0..indent {
            output.push('\t');
        }
        
        if needs_quotes(key) {
            output.push_str(&format!("\"{}\" = ", key));
        } else {
            output.push_str(&format!("{} = ", key));
        }
        
        write_value_multiline(output, value, indent, false, uuid_comments)?;
        output.push_str(";\n");
    }
    
    Ok(())
}

/// Write value in flat format
fn write_value_flat(output: &mut String, value: &PlistValue, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    match value {
        PlistValue::String(s) => {
            // Add UUID comments in flat format too
            if is_uuid(s) {
                if let Some(comment) = uuid_comments.get(s) {
                    output.push_str(&format!("{} /* {} */", s, comment));
                    return Ok(());
                }
            }
            // Regular string (not UUID or no comment found)
            if needs_quotes(s) {
                output.push_str(&format!("\"{}\"", escape_string(s)));
            } else {
                output.push_str(s);
            }
        }
        PlistValue::Integer(i) => output.push_str(&i.to_string()),
        PlistValue::Real(f) => output.push_str(&f.to_string()),
        PlistValue::Boolean(b) => output.push_str(if *b { "YES" } else { "NO" }),
        PlistValue::Array(arr) => {
            output.push_str("(");
            for (i, item) in arr.iter().enumerate() {
                write_value_flat(output, item, uuid_comments)?;
                output.push_str(",");
                if i < arr.len() - 1 {
                    output.push_str(" ");
                }
            }
            output.push_str(")");
        }
        PlistValue::Dictionary(dict) => {
            output.push_str("{");
            let mut first = true;
            for (k, v) in dict {
                if !first {
                    output.push_str(" ");
                }
                first = false;
                output.push_str(&format!("{} = ", k));
                write_value_flat(output, v, uuid_comments)?;
                output.push_str(";");
            }
            output.push_str("}");
        }
        _ => {}
    }
    Ok(())
}

/// Write value in multiline format
fn write_value_multiline(output: &mut String, value: &PlistValue, indent: usize, _in_array: bool, uuid_comments: &HashMap<String, String>) -> Result<(), String> {
    match value {
        PlistValue::String(s) => {
            // Add UUID comments everywhere (field values AND arrays)
            if is_uuid(s) {
                if let Some(comment) = uuid_comments.get(s) {
                    output.push_str(&format!("{} /* {} */", s, comment));
                    return Ok(());
                }
            }
            if needs_quotes(s) {
                output.push_str(&format!("\"{}\"", escape_string(s)));
            } else {
                output.push_str(s);
            }
        }
        PlistValue::Integer(i) => output.push_str(&i.to_string()),
        PlistValue::Real(f) => output.push_str(&f.to_string()),
        PlistValue::Boolean(b) => output.push_str(if *b { "YES" } else { "NO" }),
        PlistValue::Array(arr) => {
            output.push_str("(\n");
            for item in arr {
                for _ in 0..=indent {
                    output.push('\t');
                }
                write_value_multiline(output, item, indent + 1, true, uuid_comments)?;
                output.push_str(",\n");
            }
            for _ in 0..indent {
                output.push('\t');
            }
            output.push_str(")");
        }
        PlistValue::Dictionary(dict) => {
            output.push_str("{\n");
            write_dict_multiline(output, dict, indent + 1, uuid_comments)?;
            for _ in 0..indent {
                output.push('\t');
            }
            output.push_str("}");
        }
        _ => {}
    }
    Ok(())
}

/// Get a comment for an object
/// Helper to find which build phase contains a given PBXBuildFile
fn find_build_phase_for_file(build_file_id: &str, registry: &Registry) -> Option<String> {
    // Check each type of build phase
    for (_id, obj) in registry.iter() {
        let any_obj = obj.as_ref() as &dyn std::any::Any;
        
        // Check if this is a build phase that contains our build file
        if let Some(phase) = any_obj.downcast_ref::<PBXSourcesBuildPhase>() {
            if phase.files.iter().any(|f| f.to_string() == build_file_id) {
                return Some("Sources".to_string());
            }
        } else if let Some(phase) = any_obj.downcast_ref::<PBXFrameworksBuildPhase>() {
            if phase.files.iter().any(|f| f.to_string() == build_file_id) {
                return Some("Frameworks".to_string());
            }
        } else if let Some(phase) = any_obj.downcast_ref::<PBXResourcesBuildPhase>() {
            if phase.files.iter().any(|f| f.to_string() == build_file_id) {
                return Some("Resources".to_string());
            }
        } else if let Some(phase) = any_obj.downcast_ref::<PBXHeadersBuildPhase>() {
            if phase.files.iter().any(|f| f.to_string() == build_file_id) {
                return Some("Headers".to_string());
            }
        } else if let Some(phase) = any_obj.downcast_ref::<PBXCopyFilesBuildPhase>() {
            if phase.files.iter().any(|f| f.to_string() == build_file_id) {
                return phase.name.clone().or_else(|| Some("Embed Frameworks".to_string()));
            }
        }
    }
    None
}

fn get_object_comment(obj: &dyn PBXObject, registry: &Registry) -> Option<String> {
    use std::any::Any;
    let any_obj = obj as &dyn Any;
    
    // PBXBuildFile: Follow fileRef to get the filename, and add build phase suffix
    if let Some(build_file) = any_obj.downcast_ref::<PBXBuildFile>() {
        if let Ok(file_ref_id) = xforge_core::ObjectId::from_uuid_string(&build_file.file_ref.to_string()) {
            if let Some(file_ref) = registry.get::<PBXFileReference>(&file_ref_id) {
                let filename = file_ref.name.clone().or_else(|| file_ref.path.clone())?;
                // Add build phase suffix (e.g., " in Sources", " in Frameworks")
                // Note: We need the build file's ID to find its phase, but we don't have it here
                // So we'll need to pass it from the caller or use a different approach
                return Some(filename);
            }
        }
        return None;
    } else if let Some(fr) = any_obj.downcast_ref::<PBXFileReference>() {
        // Filter out empty strings - use path if name is empty
        return fr.name.clone().filter(|s| !s.is_empty()).or_else(|| fr.path.clone());
    } else if let Some(target) = any_obj.downcast_ref::<PBXNativeTarget>() {
        return Some(target.name.clone());
    } else if let Some(group) = any_obj.downcast_ref::<PBXGroup>() {
        // Filter out empty strings - use path if name is empty
        return group.name.clone().filter(|s| !s.is_empty()).or_else(|| group.path.clone());
    } else if let Some(config) = any_obj.downcast_ref::<XCBuildConfiguration>() {
        return Some(config.name.clone());
    } else if let Some(_) = any_obj.downcast_ref::<PBXSourcesBuildPhase>() {
        return Some("Sources".to_string());
    } else if let Some(_) = any_obj.downcast_ref::<PBXFrameworksBuildPhase>() {
        return Some("Frameworks".to_string());
    } else if let Some(_) = any_obj.downcast_ref::<PBXResourcesBuildPhase>() {
        return Some("Resources".to_string());
    } else if let Some(_) = any_obj.downcast_ref::<PBXHeadersBuildPhase>() {
        return Some("Headers".to_string());
    } else if let Some(copy) = any_obj.downcast_ref::<PBXCopyFilesBuildPhase>() {
        // Return name if present, otherwise determine by dstSubfolderSpec
        return copy.name.clone().or_else(|| {
            // dstSubfolderSpec = 10 means "Frameworks" subfolder (Embed Frameworks)
            // dstSubfolderSpec = 0 or other values default to "CopyFiles"
            if copy.dst_subfolder_spec == 10 {
                Some("Embed Frameworks".to_string())
            } else {
                Some("CopyFiles".to_string())
            }
        });
    } else if let Some(shell) = any_obj.downcast_ref::<PBXShellScriptBuildPhase>() {
        return shell.name.clone().or_else(|| Some("ShellScript".to_string()));
    } else if let Some(variant) = any_obj.downcast_ref::<PBXVariantGroup>() {
        return variant.name.clone();
    } else if let Some(proxy) = any_obj.downcast_ref::<PBXContainerItemProxy>() {
        return proxy.remote_info.clone();
    } else if let Some(_) = any_obj.downcast_ref::<PBXProject>() {
        return Some("Project object".to_string());
    }
    
    None
}

/// Get detailed comment for XCConfigurationList
/// Format: "Build configuration list for PBXNativeTarget \"Unity-iPhone\""
fn get_config_list_comment(config_list_uuid: &str, registry: &Registry) -> Option<String> {
    use xforge_core::PBXObject;
    
    // Search for targets or project that reference this configuration list
    for (_id, obj) in registry.iter() {
        let any_obj = obj.as_any();
        
        // Check PBXNativeTarget
        if let Some(target) = any_obj.downcast_ref::<PBXNativeTarget>() {
            if let Some(ref config_list_id) = target.build_configuration_list {
                if config_list_id.to_string() == config_list_uuid {
                    return Some(format!("Build configuration list for PBXNativeTarget \"{}\"", target.name));
                }
            }
        }
        
        // Check PBXProject
        if let Some(project) = any_obj.downcast_ref::<PBXProject>() {
            if let Some(ref config_list_id) = project.build_configuration_list {
                if config_list_id.to_string() == config_list_uuid {
                    return Some(format!("Build configuration list for PBXProject \"{}\"", project.name));
                }
            }
        }
    }
    
    // Fallback to generic comment
    Some("Build configuration list".to_string())
}

/// Get comment for a UUID reference
fn get_uuid_comment(uuid: &str, registry: &Registry) -> Option<String> {
    let uuid_obj = xforge_core::ObjectId::from_uuid_string(uuid).ok()?;
    // Try all possible types
    if let Some(obj) = registry.get::<PBXBuildFile>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXFileReference>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXGroup>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXVariantGroup>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXNativeTarget>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<XCBuildConfiguration>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<XCConfigurationList>(&uuid_obj) {
        return get_config_list_comment(uuid, registry);
    }
    if let Some(obj) = registry.get::<PBXSourcesBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXFrameworksBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXResourcesBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXHeadersBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXCopyFilesBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXShellScriptBuildPhase>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXContainerItemProxy>(&uuid_obj) {
        return get_object_comment(obj, registry);
    }
    if let Some(obj) = registry.get::<PBXTargetDependency>(&uuid_obj) {
        return Some("PBXTargetDependency".to_string());
    }
    None
}

/// Check if string is a UUID (24 hex characters)
fn is_uuid(s: &str) -> bool {
    s.len() == 24 && s.chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if a string needs quotes
fn needs_quotes(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    
    if s.starts_with("___") {
        return true;
    }
    
    s.chars().any(|c| {
        !c.is_ascii_alphanumeric() && c != '_' && c != '.' && c != '$' && c != '/'
    })
}

/// Escape special characters
fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
