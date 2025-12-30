//! Example: Managing Info.plist files
//! 
//! This demonstrates how to read, modify, and merge Info.plist files,
//! similar to Ruby xcodeproj's plist management.

use xforge_model::PlistManager;
use std::fs;
use tempfile::TempDir;

fn main() {
    println!("=== xforge Plist Management Demo ===\n");

    // Create temporary directory for demo
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let info_plist_path = temp_dir.path().join("Info.plist");

    // Example 1: Create and populate Info.plist
    println!("1. Creating new Info.plist:");
    let mut info_plist = PlistManager::new();
    
    // Basic bundle information
    info_plist.set_string("CFBundleName", "MyApp");
    info_plist.set_string("CFBundleDisplayName", "My Application");
    info_plist.set_string("CFBundleIdentifier", "com.example.myapp");
    info_plist.set_string("CFBundleVersion", "1.0.0");
    info_plist.set_string("CFBundleShortVersionString", "1.0");
    info_plist.set_string("CFBundlePackageType", "APPL");
    println!("   ✓ Basic bundle info set\n");

    // Device capabilities
    info_plist.add_to_array("UIRequiredDeviceCapabilities", "armv7");
    info_plist.add_to_array("UIRequiredDeviceCapabilities", "arm64");
    println!("   ✓ Device capabilities added\n");

    // Supported interface orientations
    info_plist.add_to_array("UISupportedInterfaceOrientations", "UIInterfaceOrientationPortrait");
    info_plist.add_to_array("UISupportedInterfaceOrientations", "UIInterfaceOrientationLandscapeLeft");
    info_plist.add_to_array("UISupportedInterfaceOrientations", "UIInterfaceOrientationLandscapeRight");
    println!("   ✓ Interface orientations configured\n");

    // iOS specific settings
    info_plist.set_bool("UIRequiresFullScreen", false);
    info_plist.set_bool("UILaunchStoryboardName", true);
    info_plist.set_integer("UIStatusBarHidden", 0);
    println!("   ✓ iOS settings configured\n");

    // Save Info.plist
    info_plist.save(&info_plist_path)
        .expect("Failed to save Info.plist");
    println!("2. Saved Info.plist to: {}\n", info_plist_path.display());

    // Example 2: Load and modify
    println!("3. Loading and modifying Info.plist:");
    let mut loaded_plist = PlistManager::load(&info_plist_path)
        .expect("Failed to load Info.plist");
    
    println!("   - Current version: {}", loaded_plist.get_string("CFBundleVersion").unwrap());
    
    // Update version
    loaded_plist.set_string("CFBundleVersion", "1.0.1");
    loaded_plist.set_string("CFBundleShortVersionString", "1.0.1");
    println!("   ✓ Version updated to 1.0.1\n");

    // Example 3: Merge plists
    println!("4. Merging configuration overrides:");
    let mut override_plist = PlistManager::new();
    
    // Production configuration
    override_plist.set_string("CFBundleDisplayName", "My App Pro");
    override_plist.set_string("API_BASE_URL", "https://api.production.example.com");
    override_plist.add_to_array("UIRequiredDeviceCapabilities", "metal"); // Add new capability
    
    loaded_plist.merge_recursive(&override_plist);
    println!("   ✓ Production configuration merged\n");

    // Example 4: Query plist values
    println!("5. Final Info.plist configuration:");
    println!("   Bundle Info:");
    println!("     - Name: {}", loaded_plist.get_string("CFBundleName").unwrap_or("N/A"));
    println!("     - Display Name: {}", loaded_plist.get_string("CFBundleDisplayName").unwrap_or("N/A"));
    println!("     - Bundle ID: {}", loaded_plist.get_string("CFBundleIdentifier").unwrap_or("N/A"));
    println!("     - Version: {}", loaded_plist.get_string("CFBundleVersion").unwrap_or("N/A"));
    
    if let Some(capabilities) = loaded_plist.get_array("UIRequiredDeviceCapabilities") {
        println!("\n   Device Capabilities:");
        for cap in capabilities {
            if let Some(cap_str) = cap.as_string() {
                println!("     - {}", cap_str);
            }
        }
    }
    
    if let Some(orientations) = loaded_plist.get_array("UISupportedInterfaceOrientations") {
        println!("\n   Supported Orientations:");
        for orientation in orientations {
            if let Some(orient_str) = orientation.as_string() {
                println!("     - {}", orient_str);
            }
        }
    }
    
    println!("\n   iOS Settings:");
    println!("     - Full Screen: {}", loaded_plist.get_bool("UIRequiresFullScreen").unwrap_or(false));
    println!("     - API URL: {}", loaded_plist.get_string("API_BASE_URL").unwrap_or("N/A"));
    
    // Save final version
    loaded_plist.save(&info_plist_path)
        .expect("Failed to save final Info.plist");
    println!("\n   ✓ Saved final Info.plist\n");

    // Example 5: Demonstrate file operations
    println!("6. Plist file operations:");
    println!("   - File path: {}", loaded_plist.path().display());
    println!("   - Total keys: {}", loaded_plist.keys().count());
    println!("   - Contains 'CFBundleName': {}", loaded_plist.contains_key("CFBundleName"));
    println!("   - Contains 'UnknownKey': {}", loaded_plist.contains_key("UnknownKey"));
    
    // Show file content
    let content = fs::read_to_string(&info_plist_path)
        .expect("Failed to read plist file");
    println!("\n7. Generated Info.plist content (first 20 lines):");
    println!("   {}", "=".repeat(50));
    for (i, line) in content.lines().take(20).enumerate() {
        println!("   {:3} | {}", i + 1, line);
    }
    println!("   ...");
    println!("   {}\n", "=".repeat(50));

    println!("=== Demo Complete ===\n");
    println!("Key features demonstrated:");
    println!("✓ PlistManager::new() - Create empty plist");
    println!("✓ set_string(), set_bool(), set_integer() - Set values");
    println!("✓ add_to_array() - Add array elements with deduplication");
    println!("✓ save() - Write plist to disk");
    println!("✓ load() - Read plist from disk");
    println!("✓ merge_recursive() - Merge plists with array deduplication");
    println!("✓ get_string(), get_bool(), get_array() - Query values");
    println!("✓ keys(), contains_key() - Introspection");
}
