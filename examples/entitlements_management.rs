//! Example: Managing iOS Entitlements
//! 
//! This demonstrates how to create and configure app entitlements for various
//! iOS capabilities like Push Notifications, App Groups, iCloud, etc.

use xforge_model::EntitlementsManager;
use std::fs;
use tempfile::TempDir;

fn main() {
    println!("=== xforge Entitlements Management Demo ===\n");

    // Create temporary directory for demo
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let entitlements_path = temp_dir.path().join("MyApp.entitlements");

    // Example 1: Create basic entitlements with Push Notifications
    println!("1. Creating entitlements with Push Notifications:");
    let mut entitlements = EntitlementsManager::new();
    
    entitlements.enable_push_notifications("development");
    println!("   ✓ Push Notifications enabled (development)\n");

    // Example 2: Add App Groups for data sharing
    println!("2. Adding App Groups:");
    entitlements.add_app_group("group.com.example.myapp");
    entitlements.add_app_group("group.com.example.myapp.shared");
    println!("   ✓ App Groups added for data sharing\n");

    // Example 3: Configure iCloud
    println!("3. Enabling iCloud capabilities:");
    entitlements.enable_icloud_key_value();
    entitlements.add_cloudkit_container("iCloud.com.example.myapp");
    println!("   ✓ iCloud Key-Value Storage enabled");
    println!("   ✓ CloudKit container added\n");

    // Example 4: Add Associated Domains for Universal Links
    println!("4. Configuring Associated Domains:");
    entitlements.add_associated_domain("applinks:example.com");
    entitlements.add_associated_domain("applinks:www.example.com");
    entitlements.add_associated_domain("webcredentials:example.com");
    println!("   ✓ Universal Links configured");
    println!("   ✓ Web Credentials configured\n");

    // Example 5: Enable various capabilities
    println!("5. Enabling additional capabilities:");
    entitlements.enable_healthkit();
    println!("   ✓ HealthKit enabled");
    
    entitlements.enable_siri();
    println!("   ✓ Siri enabled");
    
    entitlements.enable_game_center();
    println!("   ✓ Game Center enabled");
    
    entitlements.enable_homekit();
    println!("   ✓ HomeKit enabled\n");

    // Example 6: Configure Keychain Sharing
    println!("6. Setting up Keychain Sharing:");
    entitlements.add_keychain_group("$(AppIdentifierPrefix)com.example.myapp");
    entitlements.add_keychain_group("$(AppIdentifierPrefix)com.example.myapp.extension");
    println!("   ✓ Keychain access groups configured\n");

    // Example 7: Enable Apple Pay
    println!("7. Configuring Apple Pay:");
    entitlements.enable_apple_pay(vec![
        "merchant.com.example.myapp".to_string(),
        "merchant.com.example.myapp.store".to_string(),
    ]);
    println!("   ✓ Apple Pay merchant IDs added\n");

    // Example 8: Set Data Protection
    println!("8. Setting Data Protection:");
    entitlements.set_data_protection("NSFileProtectionComplete");
    println!("   ✓ Complete data protection enabled\n");

    // Save entitlements
    entitlements.save(&entitlements_path)
        .expect("Failed to save entitlements");
    println!("9. Saved entitlements to: {}\n", entitlements_path.display());

    // Example 9: Load and query entitlements
    println!("10. Loading and querying entitlements:");
    let loaded = EntitlementsManager::load(&entitlements_path)
        .expect("Failed to load entitlements");
    
    println!("    Push Notifications: {}", 
        loaded.get_string("aps-environment").unwrap_or("not set"));
    
    println!("    App Groups: {:?}", loaded.get_app_groups());
    
    println!("    HealthKit enabled: {}", 
        loaded.get_bool("com.apple.developer.healthkit").unwrap_or(false));
    
    println!("    Total entitlements: {}\n", loaded.keys().count());

    // Display the generated file
    let content = fs::read_to_string(&entitlements_path)
        .expect("Failed to read entitlements file");
    
    println!("11. Generated entitlements file (first 40 lines):");
    println!("    {}", "=".repeat(60));
    for (i, line) in content.lines().take(40).enumerate() {
        println!("    {:3} | {}", i + 1, line);
    }
    println!("    ...");
    println!("    {}\n", "=".repeat(60));

    // Example 10: Create production entitlements
    println!("12. Creating production entitlements variant:");
    let mut prod_entitlements = EntitlementsManager::new();
    
    // Production uses different APS environment
    prod_entitlements.enable_push_notifications("production");
    
    // Copy other settings from development
    for group in loaded.get_app_groups() {
        prod_entitlements.add_app_group(&group);
    }
    
    prod_entitlements.enable_icloud_key_value();
    prod_entitlements.add_cloudkit_container("iCloud.com.example.myapp");
    
    let prod_path = temp_dir.path().join("MyApp-Production.entitlements");
    prod_entitlements.save(&prod_path)
        .expect("Failed to save production entitlements");
    
    println!("    ✓ Production entitlements created");
    println!("    - APS Environment: production");
    println!("    - Saved to: {}\n", prod_path.display());

    // Example 11: Demonstrate custom entitlements
    println!("13. Adding custom entitlements:");
    let mut custom = EntitlementsManager::new();
    
    custom.set_bool("com.custom.feature.enabled", true);
    custom.set_string("com.custom.api.key", "your-api-key-here");
    custom.set_array("com.custom.allowed.domains", vec![
        "api.example.com".to_string(),
        "cdn.example.com".to_string(),
    ]);
    
    println!("    ✓ Custom boolean entitlement");
    println!("    ✓ Custom string entitlement");
    println!("    ✓ Custom array entitlement\n");

    println!("=== Demo Complete ===\n");
    println!("Key features demonstrated:");
    println!("✓ EntitlementsManager::new() - Create empty entitlements");
    println!("✓ enable_push_notifications() - APS Environment");
    println!("✓ add_app_group() - App Groups for data sharing");
    println!("✓ enable_icloud_*() - iCloud and CloudKit");
    println!("✓ add_associated_domain() - Universal Links");
    println!("✓ enable_healthkit/siri/game_center/homekit() - iOS capabilities");
    println!("✓ add_keychain_group() - Keychain Sharing");
    println!("✓ enable_apple_pay() - Apple Pay merchants");
    println!("✓ set_data_protection() - Data Protection");
    println!("✓ set_bool/string/array() - Custom entitlements");
    println!("✓ save() - Write XML entitlements file");
    println!("✓ load() - Read existing entitlements");
}
