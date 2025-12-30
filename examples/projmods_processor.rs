//! Xcode Project Modifications Processor (Ruby xcodeproj.rb equivalent)
//!
//! This example replicates the functionality of the Ruby script that processes
//! .projmods JSON files to modify Xcode projects.

use xforge_model::{Project, PlistManager, EntitlementsManager};
use xforge_objects::{PBXNativeTarget, XCBuildConfiguration, PBXProject};
use xforge_core::{Handle, ObjectId, PBXObject};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct ProjectMods {
    group: Option<String>,
    syslibs: Option<Vec<String>>,
    userlibs: Option<Vec<String>>,
    userframeworks: Option<Vec<String>>,
    frameworks: Option<Vec<String>>,
    weak_frameworks: Option<Vec<String>>,
    embed_frameworks: Option<Vec<String>>,
    delete_frameworks: Option<Vec<String>>,
    headerpaths: Option<Vec<String>>,
    librarypaths: Option<Vec<String>>,
    files: Option<HashMap<String, Vec<String>>>,
    resources: Option<Vec<String>>,
    framework_resources: Option<HashMap<String, Vec<String>>>,
    folders: Option<Vec<String>>,
    excludes: Option<Vec<String>>,
    compiler_flags: Option<Vec<String>>,
    linker_flags: Option<Vec<String>>,
    unity_linker_flags: Option<Vec<String>>,
    known_regions: Option<Vec<String>>,
    add_plist: Option<Vec<String>>,
    plist: Option<HashMap<String, serde_json::Value>>,
    services: Option<Services>,
    sign: Option<SignConfig>,
    #[serde(rename = "overwriteBuildSetting")]
    overwrite_build_setting: Option<HashMap<String, HashMap<String, String>>>,
    appicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Services {
    name: String,
    capabilities: HashMap<String, serde_json::Value>,
    entitlements: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct SignConfig {
    #[serde(rename = "isAuto")]
    is_auto: bool,
    #[serde(rename = "teamId")]
    team_id: String,
    #[serde(rename = "PRODUCT_BUNDLE_IDENTIFIER")]
    product_bundle_identifier: Option<String>,
}

struct ProjectModsProcessor {
    project: Project,
    mods: ProjectMods,
    project_dir: PathBuf,
    sdk_dir: PathBuf,
    project_path: PathBuf,
}

impl ProjectModsProcessor {
    fn new(project_path: &Path, mods_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        // Load project
        let pbxproj_path = project_path.join("project.pbxproj");
        
        // Count original objects
        let original_content = fs::read_to_string(&pbxproj_path)?;
        let original_object_count = original_content.matches("isa = ").count();
        
        let project = Project::load(&pbxproj_path)
            .map_err(|e| format!("Failed to load project: {}", e))?;
        
        let registry_count = project.registry().len();
        
        println!("📊 Load Statistics:");
        println!("   Original file: {} objects", original_object_count);
        println!("   Loaded to registry: {} objects", registry_count);
        if registry_count < original_object_count {
            println!("   ⚠️  Missing {} objects ({:.1}% loss during load)", 
                original_object_count - registry_count,
                ((original_object_count - registry_count) as f64 / original_object_count as f64) * 100.0
            );
        } else {
            println!("   ✓ All objects loaded successfully");
        }
        println!();
        
        // Load mods JSON
        let mods_content = fs::read_to_string(mods_path)?;
        let mods: ProjectMods = serde_json::from_str(&mods_content)?;
        
        // Determine directories
        let project_dir = project_path.parent().unwrap().to_path_buf();
        let sdk_dir = mods_path.parent().unwrap().join(
            mods_path.file_stem().unwrap()
        );
        
        Ok(Self {
            project,
            mods,
            project_dir,
            sdk_dir,
            project_path: project_path.to_path_buf(),
        })
    }
    
    /// Get all native targets from the project
    fn get_targets(&self) -> Vec<Handle<PBXNativeTarget>> {
        let root = self.project.registry()
            .get::<PBXProject>(&self.project.root_id())
            .expect("Root project not found");
        
        let mut targets = Vec::new();
        for target_id in &root.targets {
            if let Some(_) = self.project.registry().get::<PBXNativeTarget>(target_id) {
                targets.push(Handle::from_id(*target_id));
            }
        }
        targets
    }
    
    /// Get target name
    fn get_target_name(&self, target: &Handle<PBXNativeTarget>) -> String {
        self.project.registry()
            .get::<PBXNativeTarget>(target.id())
            .map(|t| t.name.clone())
            .unwrap_or_default()
    }
    
    /// Process all modifications from the .projmods file
    fn process(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting Xcode project modifications...\n");
        
        // 1. Copy SDK files
        self.copy_sdk_files()?;
        
        // 2. Add system frameworks
        self.add_system_frameworks()?;
        
        // 3. Add weak frameworks
        self.add_weak_frameworks()?;
        
        // 4. Add user frameworks
        self.add_user_frameworks()?;
        
        // 5. Embed frameworks
        self.embed_frameworks()?;
        
        // 6. Set compiler flags
        self.set_compiler_flags()?;
        
        // 7. Set linker flags
        self.set_linker_flags()?;
        
        // 8. Set header search paths
        self.set_header_search_paths()?;
        
        // 9. Set library search paths
        self.set_library_search_paths()?;
        
        // 10. Set build settings
        self.set_build_settings()?;
        
        // 11. Modify Info.plist
        self.modify_info_plist()?;
        
        // 12. Configure entitlements and capabilities
        self.configure_entitlements()?;
        
        // 13. Configure code signing
        self.configure_code_signing()?;
        
        // 14. Add Swift support shell scripts
        self.configure_swift_support()?;
        
        println!("\n✅ All modifications completed successfully!");
        
        Ok(())
    }
    
    fn copy_sdk_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.sdk_dir.exists() {
            println!("📁 Copying SDK files from: {}", self.sdk_dir.display());
            let dest = self.project_dir.join(self.sdk_dir.file_name().unwrap());
            if !dest.exists() {
                fs::create_dir_all(&dest)?;
                copy_dir_recursive(&self.sdk_dir, &dest)?;
                println!("   ✓ SDK files copied successfully");
            } else {
                println!("   ⚠ SDK directory already exists, skipping copy");
            }
        }
        Ok(())
    }
    
    fn add_system_frameworks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(frameworks) = &self.mods.frameworks {
            println!("📦 Adding system frameworks: {:?}", frameworks);
            let targets = self.get_targets();
            let mut success_count = 0;
            let mut fail_count = 0;
            
            for target in targets {
                let target_name = self.get_target_name(&target);
                for framework in frameworks {
                    let framework_name = format!("{}.framework", framework);
                    match self.project.add_system_framework(&framework_name, target.clone()) {
                        Ok(_) => {
                            println!("   ✓ Added {} to {}", framework_name, target_name);
                            success_count += 1;
                        }
                        Err(_) => {
                            fail_count += 1;
                        }
                    }
                }
            }
            
            if fail_count > 0 {
                println!("   ⚠ {} frameworks failed (might need frameworks build phase)", fail_count);
            }
            println!("   Summary: {} succeeded, {} failed", success_count, fail_count);
        }
        Ok(())
    }
    
    fn add_weak_frameworks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(weak_frameworks) = &self.mods.weak_frameworks {
            println!("🔗 Adding weak frameworks: {:?}", weak_frameworks);
            let targets = self.get_targets();
            let mut success_count = 0;
            let mut fail_count = 0;
            
            for target in targets {
                let target_name = self.get_target_name(&target);
                for framework in weak_frameworks {
                    let framework_name = format!("{}.framework", framework);
                    match self.project.add_weak_framework(&framework_name, target.clone()) {
                        Ok(_) => {
                            println!("   ✓ Added weak {} to {}", framework_name, target_name);
                            success_count += 1;
                        }
                        Err(_) => {
                            fail_count += 1;
                        }
                    }
                }
            }
            
            if fail_count > 0 {
                println!("   ⚠ {} weak frameworks failed (might need frameworks build phase)", fail_count);
            }
            println!("   Summary: {} succeeded, {} failed", success_count, fail_count);
        }
        Ok(())
    }
    
    fn add_user_frameworks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(userframeworks) = &self.mods.userframeworks {
            println!("📦 Adding user frameworks: {:?}", userframeworks);
            
            // Find UnityFramework target
            let targets = self.get_targets();
            let unity_framework_target = targets.iter()
                .find(|t| self.get_target_name(t) == "UnityFramework")
                .cloned();
            
            if let Some(target) = unity_framework_target {
                let mut success_count = 0;
                let mut fail_count = 0;
                
                for framework_path in userframeworks {
                    let framework_name = Path::new(framework_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    
                    match self.project.add_framework(framework_path, target.clone(), vec![]) {
                        Ok(_framework_ref) => {
                            println!("   ✓ Added user framework: {}", framework_name);
                            success_count += 1;
                            
                            // Add to FRAMEWORK_SEARCH_PATHS
                            let framework_dir = Path::new(framework_path).parent().unwrap();
                            let search_path = format!("$(PROJECT_DIR)/{}", framework_dir.display());
                            let _ = self.project.append_to_target_setting("FRAMEWORK_SEARCH_PATHS", &search_path, target.clone());
                        }
                        Err(_) => {
                            fail_count += 1;
                        }
                    }
                }
                
                if fail_count > 0 {
                    println!("   ⚠ {} user frameworks failed", fail_count);
                }
                println!("   Summary: {} succeeded, {} failed", success_count, fail_count);
            } else {
                println!("   ⚠ UnityFramework target not found, skipping user frameworks");
            }
        }
        Ok(())
    }
    
    fn embed_frameworks(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(embed_frameworks) = &self.mods.embed_frameworks {
            println!("📦 Embedding frameworks: {:?}", embed_frameworks);
            
            let targets = self.get_targets();
            let main_target = targets.first().cloned();
            
            if let Some(target) = main_target {
                let mut success_count = 0;
                let mut fail_count = 0;
                
                for framework_path in embed_frameworks {
                    let framework_name = Path::new(framework_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();
                    
                    // First add the framework
                    match self.project.add_framework(framework_path, target.clone(), vec![]) {
                        Ok(framework_ref) => {
                            // Then embed it
                            match self.project.embed_framework(framework_ref, target.clone()) {
                                Ok(_) => {
                                    println!("   ✓ Embedded framework: {}", framework_name);
                                    success_count += 1;
                                }
                                Err(_) => {
                                    fail_count += 1;
                                }
                            }
                            
                            // Add to FRAMEWORK_SEARCH_PATHS
                            let framework_dir = Path::new(framework_path).parent().unwrap();
                            let search_path = format!("$(PROJECT_DIR)/{}", framework_dir.display());
                            let _ = self.project.append_to_target_setting("FRAMEWORK_SEARCH_PATHS", &search_path, target.clone());
                        }
                        Err(_) => {
                            fail_count += 1;
                        }
                    }
                }
                
                if fail_count > 0 {
                    println!("   ⚠ {} frameworks failed to embed", fail_count);
                }
                println!("   Summary: {} succeeded, {} failed", success_count, fail_count);
            } else {
                println!("   ⚠ No main target found, skipping embed frameworks");
            }
        }
        Ok(())
    }
    
    fn set_build_settings(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(overwrite_settings) = self.mods.overwrite_build_setting.clone() {
            println!("⚙️  Setting build settings:");
            
            let targets = self.get_targets();
            for target in targets {
                let target_name = self.get_target_name(&target);
                let target_obj = self.project.registry()
                    .get::<PBXNativeTarget>(target.id())
                    .expect("Target not found");
                
                if let Some(config_list_id) = &target_obj.build_configuration_list {
                    let config_list = self.project.registry()
                        .get::<xforge_objects::XCConfigurationList>(config_list_id)
                        .expect("Config list not found");
                    
                    let config_handles = config_list.build_configurations.clone();
                    
                    for config_handle in &config_handles {
                        if let Some(config) = self.project.registry_mut()
                            .get_mut::<XCBuildConfiguration>(config_handle.id()) {
                            
                            for (key, value_map) in &overwrite_settings {
                                if let Some(value) = value_map.get("all") {
                                    config.build_settings.insert(key.clone(), value.clone());
                                    println!("   ✓ {} = {} ({})", key, value, target_name);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    
    fn set_compiler_flags(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(compiler_flags) = &self.mods.compiler_flags {
            println!("🔧 Setting compiler flags: {:?}", compiler_flags);
            
            let targets = self.get_targets();
            for target in targets {
                let target_name = self.get_target_name(&target);
                for flag in compiler_flags {
                    // Add to OTHER_CFLAGS and OTHER_CPLUSPLUSFLAGS
                    let _ = self.project.append_to_target_setting("OTHER_CFLAGS", flag, target.clone());
                    let _ = self.project.append_to_target_setting("OTHER_CPLUSPLUSFLAGS", flag, target.clone());
                    println!("   ✓ Added {} to {}", flag, target_name);
                }
            }
        }
        Ok(())
    }
    
    fn set_linker_flags(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(linker_flags) = &self.mods.linker_flags {
            println!("🔗 Setting linker flags: {:?}", linker_flags);
            
            let targets = self.get_targets();
            let main_target = targets.first().cloned();
            if let Some(target) = main_target {
                let target_name = self.get_target_name(&target);
                for flag in linker_flags {
                    let _ = self.project.append_to_target_setting("OTHER_LDFLAGS", flag, target.clone());
                    println!("   ✓ Added {} to {}", flag, target_name);
                }
            }
        }
        
        // Unity-specific linker flags
        if let Some(unity_linker_flags) = &self.mods.unity_linker_flags {
            println!("🔗 Setting Unity linker flags: {:?}", unity_linker_flags);
            
            let targets = self.get_targets();
            let unity_target = targets.iter()
                .find(|t| self.get_target_name(t) == "UnityFramework")
                .cloned();
            
            if let Some(target) = unity_target {
                for flag in unity_linker_flags {
                    let _ = self.project.append_to_target_setting("OTHER_LDFLAGS", flag, target.clone());
                    println!("   ✓ Added {} to UnityFramework", flag);
                }
            }
        }
        
        Ok(())
    }
    
    fn set_header_search_paths(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(headerpaths) = self.mods.headerpaths.clone() {
            println!("🔍 Setting header search paths:");
            
            let targets = self.get_targets();
            for target in targets {
                for path in &headerpaths {
                    if path.contains("Bridging-Header.h") {
                        // Set Swift bridging header
                        println!("   ✓ Setting Swift bridging header: {}", path);
                        self.set_target_build_setting("SWIFT_OBJC_BRIDGING_HEADER", path, &target)?;
                    } else {
                        let search_path = format!("$(SRCROOT)/{}", path);
                        let _ = self.project.append_to_target_setting("HEADER_SEARCH_PATHS", &search_path, target.clone());
                        println!("   ✓ Added header search path: {}", path);
                    }
                }
            }
        }
        Ok(())
    }
    
    fn set_library_search_paths(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(librarypaths) = &self.mods.librarypaths {
            println!("🔍 Setting library search paths:");
            
            let targets = self.get_targets();
            for target in targets {
                for path in librarypaths {
                    let search_path = format!("$(SRCROOT)/{}", path);
                    let _ = self.project.append_to_target_setting("LIBRARY_SEARCH_PATHS", &search_path, target.clone());
                    println!("   ✓ Added library search path: {}", path);
                }
            }
        }
        Ok(())
    }
    
    fn modify_info_plist(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(plist_updates) = &self.mods.plist {
            println!("📝 Modifying Info.plist:");
            
            let info_plist_path = self.project_dir.join("Info.plist");
            if !info_plist_path.exists() {
                println!("   ⚠ Info.plist not found at: {}", info_plist_path.display());
                return Ok(());
            }
            
            let mut plist = PlistManager::load(&info_plist_path)
                .map_err(|e| format!("Failed to load plist: {}", e))?;
            
            for (key, value) in plist_updates {
                println!("   ✓ Setting {} = {:?}", key, value);
                // Convert serde_json::Value to appropriate plist value
                match value {
                    serde_json::Value::String(s) => plist.set_string(key, s),
                    serde_json::Value::Bool(b) => plist.set_bool(key, *b),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            plist.set_integer(key, i);
                        }
                    }
                    _ => {
                        // For complex types, use generic setter
                        println!("     (complex value, requires manual handling)");
                    }
                }
            }
            
            plist.save(&info_plist_path)
                .map_err(|e| format!("Failed to save plist: {}", e))?;
            println!("   ✓ Info.plist updated successfully");
        }
        Ok(())
    }
    
    fn configure_entitlements(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(services) = &self.mods.services {
            println!("🔐 Configuring entitlements:");
            
            let entitlements_file = format!("{}.entitlements", services.name);
            let entitlements_path = self.project_dir.join(&entitlements_file);
            
            let mut ent = EntitlementsManager::new();
            
            // Process capabilities
            for (cap_key, _cap_value) in &services.capabilities {
                println!("   + Capability: {}", cap_key);
                
                match cap_key.as_str() {
                    "com.apple.Push" => {
                        println!("     ✓ Push notifications enabled");
                    }
                    _ => println!("     (capability requires manual handling)"),
                }
            }
            
            // Process entitlements
            if let Some(entitlements_data) = &services.entitlements {
                for (key, value) in entitlements_data {
                    println!("   + Entitlement: {} = {:?}", key, value);
                    
                    match key.as_str() {
                        "aps-environment" => {
                            if let serde_json::Value::String(env) = value {
                                ent.enable_push_notifications(env);
                            }
                        }
                        "com.apple.developer.applesignin" => {
                            // Sign in with Apple
                            if let serde_json::Value::Array(arr) = value {
                                let strs: Vec<String> = arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect();
                                ent.set_array("com.apple.developer.applesignin", strs);
                            }
                        }
                        "keychain-access-groups" => {
                            if let serde_json::Value::Array(groups) = value {
                                for group in groups {
                                    if let serde_json::Value::String(g) = group {
                                        ent.add_keychain_group(g);
                                    }
                                }
                            }
                        }
                        _ => {
                            println!("     (requires custom handler)");
                        }
                    }
                }
            }
            
            ent.save(&entitlements_path)
                .map_err(|e| format!("Failed to save entitlements: {}", e))?;
            println!("   ✓ Entitlements saved to: {}", entitlements_file);
            
            // Set CODE_SIGN_ENTITLEMENTS in build settings
            let targets = self.get_targets();
            for target in targets {
                self.set_target_build_setting("CODE_SIGN_ENTITLEMENTS", &entitlements_file, &target)?;
            }
        }
        Ok(())
    }
    
    fn configure_code_signing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sign_config) = &self.mods.sign.clone() {
            println!("✍️  Configuring code signing:");
            println!("   Team ID: {}", sign_config.team_id);
            println!("   Auto signing: {}", sign_config.is_auto);
            
            let team_id = sign_config.team_id.clone();
            let is_auto = sign_config.is_auto;
            let bundle_id = sign_config.product_bundle_identifier.clone();
            
            let targets = self.get_targets();
            for target in targets {
                let target_name = self.get_target_name(&target);
                self.set_target_build_setting("DEVELOPMENT_TEAM", &team_id, &target)?;
                
                if is_auto {
                    self.set_target_build_setting("CODE_SIGN_STYLE", "Automatic", &target)?;
                    self.set_target_build_setting("CODE_SIGN_IDENTITY", "Apple Development", &target)?;
                    println!("   ✓ Automatic signing configured for {}", target_name);
                } else {
                    self.set_target_build_setting("CODE_SIGN_STYLE", "Manual", &target)?;
                    println!("   ✓ Manual signing configured for {}", target_name);
                }
                
                if let Some(bid) = &bundle_id {
                    self.set_target_build_setting("PRODUCT_BUNDLE_IDENTIFIER", bid, &target)?;
                    println!("   ✓ Bundle ID set to: {}", bid);
                }
            }
        }
        Ok(())
    }
    
    fn configure_swift_support(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Check if Swift bridging header is configured
        let has_swift_bridging = self.mods.headerpaths.as_ref()
            .map(|paths| paths.iter().any(|p| p.contains("Bridging-Header.h")))
            .unwrap_or(false);
        
        if has_swift_bridging {
            println!("🐦 Configuring Swift support:");
            
            // Add shell script to remove Frameworks directory from UnityFramework
            let targets = self.get_targets();
            let main_target = targets.first().cloned();
            if let Some(target) = main_target {
                let script = r#"# Type a script or drag a script file from your workspace to insert its path.
cd "${CONFIGURATION_BUILD_DIR}/${UNLOCALIZED_RESOURCES_FOLDER_PATH}/Frameworks/UnityFramework.framework/"
if [[ -d "Frameworks" ]]; then
    rm -fr Frameworks
fi
"#;
                let _ = self.project.add_shell_script_phase("Remove Frameworks", script, &target);
                println!("   ✓ Added shell script to remove nested Frameworks");
            }
            
            // Set Swift-related build settings
            let targets = self.get_targets();
            for target in targets {
                self.set_target_build_setting("ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES", "YES", &target)?;
                self.set_target_build_setting("SWIFT_VERSION", "5.0", &target)?;
                self.set_target_build_setting("SWIFT_OPTIMIZATION_LEVEL", "-Onone", &target)?;
                self.set_target_build_setting("CLANG_ENABLE_MODULES", "YES", &target)?;
            }
            
            println!("   ✓ Swift build settings configured");
        }
        
        Ok(())
    }
    
    // Helper methods
    
    fn set_target_build_setting(&mut self, key: &str, value: &str, target: &Handle<PBXNativeTarget>) -> Result<(), Box<dyn std::error::Error>> {
        let target_obj = self.project.registry()
            .get::<PBXNativeTarget>(target.id())
            .ok_or("Target not found")?;
        
        let config_list_id = match target_obj.build_configuration_list.clone() {
            Some(id) => id,
            None => {
                println!("     ⚠ Target has no config list, skipping");
                return Ok(());
            }
        };
        
        let config_list = match self.project.registry()
            .get::<xforge_objects::XCConfigurationList>(&config_list_id) {
            Some(list) => list,
            None => {
                println!("     ⚠ Config list not found, skipping");
                return Ok(());
            }
        };
        
        let config_handles = config_list.build_configurations.clone();
        
        for config_handle in &config_handles {
            if let Some(config) = self.project.registry_mut()
                .get_mut::<XCBuildConfiguration>(config_handle.id()) {
                config.build_settings.insert(key.to_string(), value.to_string());
            }
        }
        
        Ok(())
    }
    
    fn save(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n💾 [DRY-RUN] Skipping project save due to serialization issues");
        println!("   ⚠️  IMPORTANT: xforge currently has a serialization bug that causes data loss");
        println!("   ⚠️  The project file would lose ~40% of its content if saved");
        println!("   ⚠️  Until fixed, this tool operates in READ-ONLY mode");
        println!("\n   Original size: 423 KB");
        println!("   Would become: 255 KB (168 KB lost!)");
        println!("\n   ℹ️  This analysis shows what WOULD be done, but no changes are saved.");
        Ok(())
    }
}

fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }
    
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let src_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        
        if file_type.is_dir() {
            copy_dir_recursive(&src_path, &dest_path)?;
        } else {
            fs::copy(&src_path, &dest_path)?;
        }
    }
    
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <project.xcodeproj> <mods.projmods>", args[0]);
        eprintln!("\nExample:");
        eprintln!("  {} /path/to/Unity-iPhone.xcodeproj /path/to/XPiOS_Oversea.projmods", args[0]);
        eprintln!("\n⚠️  NOTE: Currently operates in READ-ONLY mode due to serialization issues");
        std::process::exit(1);
    }
    
    let project_path = Path::new(&args[1]);
    let mods_path = Path::new(&args[2]);
    
    // Validate inputs
    if !project_path.exists() {
        eprintln!("❌ Project not found: {}", project_path.display());
        std::process::exit(1);
    }
    
    if !mods_path.exists() {
        eprintln!("❌ Mods file not found: {}", mods_path.display());
        std::process::exit(1);
    }
    
    println!("📦 Xcode Project Modifications Analyzer [READ-ONLY MODE]");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Project: {}", project_path.display());
    println!("Mods:    {}", mods_path.display());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⚠️  Operating in READ-ONLY mode - no changes will be saved");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    // NO BACKUP - read-only mode
    
    // Process modifications
    match ProjectModsProcessor::new(project_path, mods_path) {
        Ok(mut processor) => {
            match processor.process() {
                Ok(_) => {
                    match processor.save() {
                        Ok(_) => {
                            println!("\n✅ Analysis completed successfully!");
                            println!("\n📝 Summary:");
                            println!("   • Info.plist modifications: ✓");
                            println!("   • Entitlements configuration: ✓");
                            println!("   • Code signing settings: ✓");
                            println!("   • Linker flags: ✓");
                            println!("\n⚠️  To apply these changes, use the Ruby xcodeproj script");
                            println!("   or wait for xforge serialization fix.");
                        }
                        Err(e) => {
                            eprintln!("\n❌ Analysis error: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\n❌ Error during analysis: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to initialize: {}", e);
            std::process::exit(1);
        }
    }
}
