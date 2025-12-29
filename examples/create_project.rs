//! Example: Create a new Xcode project from scratch

use xforge::prelude::*;

fn main() -> Result<()> {
    println!("Creating a new Xcode project...");
    
    // Create a new project
    let mut project = Project::builder()
        .name("MyApp")
        .organization("MyCompany")
        .build();
    
    println!("Project '{}' created successfully!", project.name());
    
    // Add an iOS application target
    let app_target = project.add_target()
        .name("MyApp")
        .product_type(ProductType::Application)
        .platform(Platform::iOS)
        .build()?;
    
    println!("Added target: MyApp");
    
    // Add source files
    project.add_file("Sources/AppDelegate.swift")
        .to_target(&app_target)
        .build()?;
    
    project.add_file("Sources/SceneDelegate.swift")
        .to_target(&app_target)
        .build()?;
    
    project.add_file("Sources/ViewController.swift")
        .to_target(&app_target)
        .build()?;
    
    println!("Added source files");
    
    // Save the project
    project.save()?;
    
    println!("Project saved to: {:?}", project.path());
    println!("✅ Done!");
    
    Ok(())
}
