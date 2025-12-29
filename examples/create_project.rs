//! Example: Create a new Xcode project from scratch

use xforge::prelude::*;

fn main() -> anyhow::Result<()> {
    println!("🔨 Creating a new Xcode project...\n");
    
    // Create a new project with builder pattern
    let project = ProjectBuilder::new()
        .name("MyAwesomeApp")
        .organization("MyCompany")
        .development_region("en")
        .build();
    
    println!("✅ Project created successfully!");
    println!("   Name: {}", project.name());
    println!("   Path: {}", project.path().display());
    println!("   Organization: {:?}", project.metadata().organization);
    println!("   Development Region: {}", project.metadata().development_region);
    println!("   Archive Version: {}", project.metadata().archive_version);
    println!("   Object Version: {}", project.metadata().object_version);
    
    println!("\n🎯 Platforms available:");
    println!("   - iOS: {}", Platform::iOS.as_str());
    println!("   - macOS: {}", Platform::macOS.as_str());
    println!("   - tvOS: {}", Platform::tvOS.as_str());
    println!("   - watchOS: {}", Platform::watchOS.as_str());
    println!("   - visionOS: {}", Platform::visionOS.as_str());
    
    println!("\n📦 Product types available:");
    println!("   - Application: {}", ProductType::Application.as_str());
    println!("   - Framework: {}", ProductType::Framework.as_str());
    println!("   - Static Library: {}", ProductType::StaticLibrary.as_str());
    
    println!("\n💾 Saving project...");
    project.save()?;
    
    println!("\n🎉 Done! Project structure:");
    println!("   {}/", project.name());
    println!("   └── project.pbxproj (to be implemented)");
    
    Ok(())
}
