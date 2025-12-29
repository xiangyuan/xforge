//! Example: Load and save Xcode projects

use xforge_objects::Project;

fn main() -> Result<(), String> {
    println!("Xcode Project Load/Save Demo");
    println!("============================\n");
    
    // Example 1: Create a new project
    println!("1. Creating new project...");
    let mut project = Project::new("MyTestApp");
    println!("   ✓ Created project 'MyTestApp'\n");
    
    // Example 2: Access root project
    if let Some(root) = project.root_project() {
        println!("2. Project info:");
        println!("   Name: {}", root.name);
        println!("   Targets: {}", root.targets.len());
        println!();
    }
    
    // Example 3: Save project
    println!("3. Saving project to /tmp/test.pbxproj...");
    match project.save("/tmp/test.pbxproj") {
        Ok(_) => println!("   ✓ Project saved successfully\n"),
        Err(e) => {
            println!("   ✗ Failed to save: {}\n", e);
            return Err(e);
        }
    }
    
    // Example 4: Load project back
    println!("4. Loading project from /tmp/test.pbxproj...");
    match Project::load("/tmp/test.pbxproj") {
        Ok(loaded_project) => {
            println!("   ✓ Project loaded successfully");
            if let Some(root) = loaded_project.root_project() {
                println!("   Loaded project name: {}", root.name);
            }
            println!();
        }
        Err(e) => {
            println!("   ✗ Failed to load: {}\n", e);
            return Err(e);
        }
    }
    
    println!("✓ All operations completed successfully!");
    println!("\nUsage in your code:");
    println!("  let project = Project::new(\"MyApp\");");
    println!("  project.save(\"MyApp.xcodeproj/project.pbxproj\")?;");
    println!("  let loaded = Project::load(\"MyApp.xcodeproj/project.pbxproj\")?;");
    
    Ok(())
}
