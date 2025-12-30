//! Example: Demonstrate serialization/deserialization

use xforge_objects::{serialize_registry, deserialize_registry, PBXProject};
use xforge_core::{Registry, ObjectId};
use xforge_serialization::PlistWriter;

fn main() -> Result<(), String> {
    println!("Xcode Project Serialization Demo");
    println!("=================================\n");
    
    // Example 1: Create objects and serialize
    println!("1. Creating objects in registry...");
    let mut registry = Registry::new();
    
    let project = PBXProject::new("DemoApp");
    let project_id = ObjectId::generate();
    registry.register_with_id(project_id.clone(), project);
    
    println!("   ✓ Created PBXProject 'DemoApp'\n");
    
    // Example 2: Serialize to plist
    println!("2. Serializing to plist format...");
    let plist = serialize_registry(&registry, &project_id.to_string());
    
    let mut writer = PlistWriter::new();
    let plist_string = writer.write_plist(&plist)?;
    
    println!("   ✓ Serialized {} bytes", plist_string.len());
    println!("   First 200 chars:\n   {}\n", &plist_string[..200.min(plist_string.len())]);
    
    // Example 3: Deserialize back
    println!("3. Deserializing from plist...");
    let (loaded_registry, loaded_root_id) = deserialize_registry(&plist)?;
    
    println!("   ✓ Loaded {} objects", loaded_registry.len());
    println!("   Root ID: {}\n", loaded_root_id.to_string());
    
    println!("✓ Serialization round-trip successful!");
    println!("\nNote: Use xforge_model::Project for full load/save API");
    
    Ok(())
}
