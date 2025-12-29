//! Query API usage example
//!
//! This example demonstrates how to use the query API to navigate
//! and inspect Xcode project structures.

use xforge_core::Registry;
use xforge_query::QueryResult;

fn main() -> QueryResult<()> {
    // Create a registry
    let _registry = Registry::new();
    
    // In a real scenario, you would load a project from a .pbxproj file
    // For now, this demonstrates the query API structure
    
    println!("xforge-query API Demo");
    println!("====================\n");
    
    // Example 1: Find a target by name
    println!("Example 1: Finding targets");
    println!("---------------------------");
    
    // In real usage:
    // let target_id = registry.find_target_by_name(&project_id, "MyApp")?;
    // println!("Found target: {:?}", target_id);
    println!("Usage: registry.find_target_by_name(&project_id, \"MyApp\")");
    
    // Example 2: Get all targets
    println!("\nExample 2: Getting all targets");
    println!("-------------------------------");
    // let targets = registry.get_targets(&project_id)?;
    // for target_id in targets {
    //     println!("Target: {:?}", target_id);
    // }
    println!("Usage: registry.get_targets(&project_id)");
    
    // Example 3: Get build phases
    println!("\nExample 3: Getting build phases");
    println!("--------------------------------");
    // let target_id = ObjectId::new();
    // let build_phases = registry.get_native_target_build_phases(&target_id)?;
    // for phase_id in build_phases {
    //     println!("Build phase: {:?}", phase_id);
    // }
    println!("Usage: registry.get_native_target_build_phases(&target_id)");
    
    // Example 4: Find files in groups
    println!("\nExample 4: Finding files");
    println!("------------------------");
    // let group_id = ObjectId::new();
    // let file_id = registry.find_file_in_group(&group_id, "main.swift")?;
    // println!("Found file: {:?}", file_id);
    println!("Usage: registry.find_file_in_group(&group_id, \"main.swift\")");
    
    // Example 5: Navigate group hierarchy
    println!("\nExample 5: Navigating groups");
    println!("----------------------------");
    // let root_group_id = ObjectId::new();
    // let subgroup_id = registry.find_group_by_path(&root_group_id, "Sources/Models")?;
    // println!("Found group: {:?}", subgroup_id);
    println!("Usage: registry.find_group_by_path(&root_group_id, \"Sources/Models\")");
    
    // Example 6: Query build settings
    println!("\nExample 6: Querying build settings");
    println!("-----------------------------------");
    // let config_list_id = ObjectId::new();
    // let debug_config = registry.find_configuration_by_name(&config_list_id, "Debug")?;
    // if let Some(bundle_id) = registry.get_build_setting(&debug_config, "PRODUCT_BUNDLE_IDENTIFIER")? {
    //     println!("Bundle ID: {}", bundle_id);
    // }
    println!("Usage: registry.find_configuration_by_name(&config_list_id, \"Debug\")");
    println!("       registry.get_build_setting(&config_id, \"PRODUCT_BUNDLE_IDENTIFIER\")");
    
    println!("\n✓ Query API is ready to use!");
    println!("  Load a real project with xforge-parser to test these queries.");
    
    Ok(())
}
