//! Framework management example - demonstrating new P1 features
//! 
//! cargo run --example framework_management

use xforge_model::Project;
use xforge_core::ProductType;

fn main() -> Result<(), String> {
    println!("=== xforge Framework Management Demo ===\n");
    
    // 1. Create a new project
    println!("1. Creating new project...");
    let mut project = Project::new("FrameworkDemo");
    
    // 2. Create a target
    println!("2. Creating iOS application target...");
    let target = project.create_target("MyApp".to_string(), ProductType::Application)?;
    println!("   ✓ Target created: MyApp\n");
    
    // 3. Add system frameworks
    println!("3. Adding system frameworks:");
    
    println!("   - Adding CoreGraphics.framework (normal)");
    project.add_system_framework("CoreGraphics.framework", target.clone())?;
    
    println!("   - Adding WebKit.framework (weak)");
    project.add_weak_framework("WebKit.framework", target.clone())?;
    
    println!("   - Adding UIKit.framework with custom attributes");
    project.add_framework("UIKit.framework", target.clone(), vec!["Optional".to_string()])?;
    
    println!("   ✓ Frameworks added\n");
    
    // 4. Modify build settings using array operations
    println!("4. Modifying build settings:");
    
    println!("   - Appending to OTHER_LDFLAGS");
    project.append_to_target_setting("OTHER_LDFLAGS", "-ObjC", target.clone())?;
    project.append_to_target_setting("OTHER_LDFLAGS", "-lc++", target.clone())?;
    
    println!("   - Adding framework search paths");
    project.append_to_target_setting(
        "FRAMEWORK_SEARCH_PATHS",
        "$(PROJECT_DIR)/Frameworks",
        target.clone()
    )?;
    
    println!("   ✓ Build settings updated\n");
    
    // 5. Embed a framework
    println!("5. Embedding a user framework:");
    let user_framework = project.add_file("MySDK.framework", None)?;
    project.embed_framework(user_framework, target.clone())?;
    println!("   ✓ MySDK.framework embedded with code signing\n");
    
    // 6. Verify the configuration
    println!("6. Project summary:");
    println!("   - Total objects: {}", project.registry().len());
    println!("   ✓ Project configured successfully\n");
    
    // 7. Show build settings
    if let Some(target_obj) = project.registry().get::<xforge_objects::PBXNativeTarget>(target.id()) {
        if let Some(config_list_id) = target_obj.build_configuration_list {
            if let Some(config_list) = project.registry().get::<xforge_objects::XCConfigurationList>(&config_list_id) {
                for config_handle in &config_list.build_configurations {
                    if let Some(config) = project.registry().get::<xforge_objects::XCBuildConfiguration>(config_handle.id()) {
                        println!("\n   Configuration: {}", config.name());
                        
                        if let Some(ldflags) = config.build_settings().get("OTHER_LDFLAGS") {
                            println!("   - OTHER_LDFLAGS: {}", ldflags);
                        }
                        
                        if let Some(fwk_paths) = config.build_settings().get("FRAMEWORK_SEARCH_PATHS") {
                            println!("   - FRAMEWORK_SEARCH_PATHS: {}", fwk_paths);
                        }
                    }
                }
            }
        }
    }
    
    println!("\n=== Demo Complete ===");
    println!("\nKey features demonstrated:");
    println!("✓ add_system_framework() - Add regular frameworks");
    println!("✓ add_weak_framework() - Add weak frameworks");
    println!("✓ add_framework() - Add frameworks with custom attributes");
    println!("✓ append_to_target_setting() - Smart array append with $(inherited)");
    println!("✓ embed_framework() - Embed with CodeSignOnCopy");
    
    Ok(())
}
