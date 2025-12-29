//! Example: Save an Xcode project to disk
//!
//! Demonstrates how to create a project and serialize it to project.pbxproj format

use xforge_model::Project;
use xforge_objects::serialize_registry;
use xforge_serialization::PlistWriter;
use std::fs;

fn main() -> anyhow::Result<()> {
    // Create a new project
    let project = Project::new("MyApp");
    
    println!("Creating project: {}", project.name());
    println!("Project path: {}", project.path().display());
    
    // Serialize the registry to PlistValue
    let root_id = project.root_id().to_uuid_string();
    let plist_value = serialize_registry(project.registry(), &root_id);
    
    // Write to project.pbxproj format
    let mut writer = PlistWriter::new();
    let content = writer.write_plist(&plist_value)
        .map_err(|e| anyhow::anyhow!(e))?;
    
    // Create output directory and save
    let project_dir = project.path();
    fs::create_dir_all(project_dir)?;
    
    let pbxproj_path = project_dir.join("project.pbxproj");
    fs::write(&pbxproj_path, content)?;
    
    println!("✓ Project saved to: {}", pbxproj_path.display());
    println!("\nFile preview (first 500 bytes):");
    
    let saved_content = fs::read_to_string(&pbxproj_path)?;
    let preview = &saved_content[..saved_content.len().min(500)];
    println!("{}", preview);
    
    Ok(())
}
