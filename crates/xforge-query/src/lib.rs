//! xforge-query - Query API for Xcode projects
//!
//! This crate provides convenient query methods for navigating
//! and inspecting Xcode project structures.

mod errors;

pub use errors::{QueryError, QueryResult};

// Re-export core types for convenience
pub use xforge_core::{Registry, ObjectId, Handle};
pub use xforge_objects::*;

/// Extension trait for Registry to add query capabilities
pub trait RegistryExt {
    // Target queries
    fn find_target_by_name(&self, project_id: &ObjectId, name: &str) -> QueryResult<ObjectId>;
    fn get_targets(&self, project_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    fn get_native_target_build_phases(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    
    // File and group queries  
    fn find_file_in_group(&self, group_id: &ObjectId, path: &str) -> QueryResult<ObjectId>;
    fn find_group_by_path(&self, root_group_id: &ObjectId, path: &str) -> QueryResult<ObjectId>;
    fn get_group_children(&self, group_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    
    // Configuration queries
    fn find_configuration_by_name(&self, config_list_id: &ObjectId, name: &str) -> QueryResult<ObjectId>;
    fn get_configurations(&self, config_list_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    fn get_build_setting(&self, config_id: &ObjectId, key: &str) -> QueryResult<Option<String>>;
    
    // Dependency queries
    
    /// Returns all dependency IDs for a target
    ///
    /// # Arguments
    /// * `target_id` - The ObjectId of the target (PBXNativeTarget or PBXAggregateTarget)
    ///
    /// # Returns
    /// A vector of ObjectIds representing PBXTargetDependency objects
    ///
    /// # Example
    /// ```no_run
    /// # use xforge_query::*;
    /// # let registry = Registry::new();
    /// # let target_id = ObjectId::generate();
    /// let deps = registry.get_target_dependencies(&target_id)?;
    /// println!("Target has {} dependencies", deps.len());
    /// # Ok::<(), QueryError>(())
    /// ```
    fn get_target_dependencies(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    
    /// Finds the target that a dependency points to
    ///
    /// # Arguments
    /// * `dependency_id` - The ObjectId of a PBXTargetDependency
    ///
    /// # Returns
    /// Some(ObjectId) if the dependency has a target, None otherwise
    /// Finds the target that a dependency points to
    ///
    /// Given a PBXTargetDependency ID, returns the ObjectId of the target it depends on.
    ///
    /// # Arguments
    /// * `dependency_id` - The ObjectId of the PBXTargetDependency
    ///
    /// # Returns
    /// `Some(ObjectId)` if the dependency has a target, `None` otherwise
    fn find_dependency_target(&self, dependency_id: &ObjectId) -> QueryResult<Option<ObjectId>>;
    
    /// Returns Swift Package product dependency IDs for a native target
    ///
    /// # Arguments
    /// * `target_id` - The ObjectId of a PBXNativeTarget
    ///
    /// # Returns
    /// A vector of ObjectIds representing XCSwiftPackageProductDependency objects
    /// Returns all Swift Package product dependencies for a target
    ///
    /// Gets the list of XCSwiftPackageProductDependency IDs for a native target.
    /// These represent dependencies on Swift Package Manager packages.
    ///
    /// # Arguments
    /// * `target_id` - The ObjectId of the PBXNativeTarget
    ///
    /// # Returns
    /// A vector of ObjectIds representing XCSwiftPackageProductDependency objects
    fn get_swift_package_dependencies(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    
    // Advanced file queries
    
    /// Recursively finds all files with a specific extension in a group
    ///
    /// # Arguments
    /// * `group_id` - The ObjectId of the starting PBXGroup
    /// * `extension` - File extension to search for (without the dot)
    ///
    /// # Returns
    /// A vector of ObjectIds representing matching PBXFileReference objects
    ///
    /// # Example
    /// ```no_run
    /// # use xforge_query::*;
    /// # let registry = Registry::new();
    /// # let group_id = ObjectId::generate();
    /// // Find all Swift files
    /// let swift_files = registry.find_files_by_extension(&group_id, "swift")?;
    /// # Ok::<(), QueryError>(())
    /// ```
    fn find_files_by_extension(&self, group_id: &ObjectId, extension: &str) -> QueryResult<Vec<ObjectId>>;
    
    /// Finds all source code files recursively in a group
    ///
    /// Searches for common source file extensions: swift, m, mm, c, cpp, cc, cxx, h, hpp
    ///
    /// # Arguments
    /// * `group_id` - The ObjectId of the starting PBXGroup
    ///
    /// # Returns
    /// A vector of ObjectIds representing source file PBXFileReference objects
    /// Finds all source code files in a group
    ///
    /// Searches recursively for common source file types including:
    /// swift, m, mm, c, cpp, cc, cxx, h, hpp
    ///
    /// # Arguments
    /// * `group_id` - The ObjectId of the PBXGroup to search in
    ///
    /// # Returns
    /// A vector of ObjectIds representing PBXFileReference objects
    fn find_all_source_files(&self, group_id: &ObjectId) -> QueryResult<Vec<ObjectId>>;
    
    // Build setting queries
    
    /// Returns all build settings for a configuration as a HashMap
    ///
    /// # Arguments
    /// * `config_id` - The ObjectId of an XCBuildConfiguration
    ///
    /// # Returns
    /// A HashMap containing all key-value pairs of build settings
    ///
    /// # Example
    /// ```no_run
    /// # use xforge_query::*;
    /// # let registry = Registry::new();
    /// # let config_id = ObjectId::generate();
    /// let settings = registry.get_all_build_settings(&config_id)?;
    /// if let Some(sdk) = settings.get("SDKROOT") {
    ///     println!("SDK: {}", sdk);
    /// }
    /// # Ok::<(), QueryError>(())
    /// ```
    fn get_all_build_settings(&self, config_id: &ObjectId) -> QueryResult<std::collections::HashMap<String, String>>;
    
    /// Compares build settings between two configurations
    ///
    /// # Arguments
    /// * `config1_id` - The ObjectId of the first XCBuildConfiguration
    /// * `config2_id` - The ObjectId of the second XCBuildConfiguration
    ///
    /// # Returns
    /// A vector of tuples (key, value1, value2) for settings that differ.
    /// value1/value2 are None if the setting doesn't exist in that configuration.
    ///
    /// # Example
    /// ```no_run
    /// # use xforge_query::*;
    /// # let registry = Registry::new();
    /// # let debug_id = ObjectId::generate();
    /// # let release_id = ObjectId::generate();
    /// let diffs = registry.compare_build_settings(&debug_id, &release_id)?;
    /// for (key, debug_val, release_val) in diffs {
    ///     println!("{}: {:?} vs {:?}", key, debug_val, release_val);
    /// }
    /// # Ok::<(), QueryError>(())
    /// ```
    /// Compares build settings between two configurations
    ///
    /// Identifies differences in build settings between two XCBuildConfiguration objects.
    /// Returns only settings that differ between the two configurations.
    ///
    /// # Arguments
    /// * `config1_id` - The ObjectId of the first XCBuildConfiguration
    /// * `config2_id` - The ObjectId of the second XCBuildConfiguration
    ///
    /// # Returns
    /// A vector of tuples containing:
    /// - Setting key name
    /// - Value in first configuration (None if not present)
    /// - Value in second configuration (None if not present)
    ///
    /// # Example
    /// ```no_run
    /// # use xforge_query::*;
    /// # let registry = Registry::new();
    /// # let debug_id = ObjectId::generate();
    /// # let release_id = ObjectId::generate();
    /// let diffs = registry.compare_build_settings(&debug_id, &release_id)?;
    /// for (key, val1, val2) in diffs {
    ///     println!("{}: {:?} vs {:?}", key, val1, val2);
    /// }
    /// # Ok::<(), QueryError>(())
    /// ```
    fn compare_build_settings(&self, config1_id: &ObjectId, config2_id: &ObjectId) -> QueryResult<Vec<(String, Option<String>, Option<String>)>>;
}

impl RegistryExt for Registry {
    fn find_target_by_name(&self, project_id: &ObjectId, name: &str) -> QueryResult<ObjectId> {
        if let Some(project) = self.get::<PBXProject>(project_id) {
            for target_id in &project.targets {
                // Try native target
                if let Some(target) = self.get::<PBXNativeTarget>(target_id) {
                    if target.name() == name {
                        return Ok(target_id.clone());
                    }
                }
                // Try aggregate target
                if let Some(target) = self.get::<PBXAggregateTarget>(target_id) {
                    if target.name() == name {
                        return Ok(target_id.clone());
                    }
                }
                // Try legacy target
                if let Some(target) = self.get::<PBXLegacyTarget>(target_id) {
                    if target.name() == name {
                        return Ok(target_id.clone());
                    }
                }
            }
        }
        Err(QueryError::TargetNotFound(name.to_string()))
    }

    fn get_targets(&self, project_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(project) = self.get::<PBXProject>(project_id) {
            Ok(project.targets.clone())
        } else {
            Err(QueryError::RegistryError("Project not found".to_string()))
        }
    }

    fn get_native_target_build_phases(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(target) = self.get::<PBXNativeTarget>(target_id) {
            return Ok(target.build_phases().to_vec());
        }
        Err(QueryError::RegistryError("Not a native target".to_string()))
    }

    fn find_file_in_group(&self, group_id: &ObjectId, path: &str) -> QueryResult<ObjectId> {
        if let Some(group) = self.get::<PBXGroup>(group_id) {
            for child in group.children() {
                if let Some(file_ref) = self.get::<PBXFileReference>(child) {
                    if file_ref.path() == Some(path) {
                        return Ok(child.clone());
                    }
                }
            }
        }
        Err(QueryError::FileNotFound(path.to_string()))
    }

    fn find_group_by_path(&self, root_group_id: &ObjectId, path: &str) -> QueryResult<ObjectId> {
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        // If empty path, return root group
        if components.is_empty() {
            if self.get::<PBXGroup>(root_group_id).is_some() {
                return Ok(root_group_id.clone());
            }
            return Err(QueryError::GroupNotFound(path.to_string()));
        }

        let mut current_group_id = root_group_id.clone();
        
        for component in components {
            if let Some(group) = self.get::<PBXGroup>(&current_group_id) {
                let mut found = false;
                for child in group.children() {
                    if let Some(child_group) = self.get::<PBXGroup>(child) {
                        if child_group.path() == Some(component) {
                            current_group_id = child.clone();
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    return Err(QueryError::GroupNotFound(path.to_string()));
                }
            } else {
                return Err(QueryError::GroupNotFound(path.to_string()));
            }
        }
        
        Ok(current_group_id)
    }

    fn get_group_children(&self, group_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(group) = self.get::<PBXGroup>(group_id) {
            Ok(group.children().to_vec())
        } else {
            Err(QueryError::GroupNotFound(group_id.to_string()))
        }
    }

    fn find_configuration_by_name(&self, config_list_id: &ObjectId, name: &str) -> QueryResult<ObjectId> {
        if let Some(config_list) = self.get::<XCConfigurationList>(config_list_id) {
            for config in config_list.build_configurations() {
                if let Some(cfg) = self.get::<XCBuildConfiguration>(config.id()) {
                    if cfg.name() == name {
                        return Ok(config.id().clone());
                    }
                }
            }
        }
        Err(QueryError::ConfigurationNotFound(name.to_string()))
    }

    fn get_configurations(&self, config_list_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(config_list) = self.get::<XCConfigurationList>(config_list_id) {
            Ok(config_list.build_configurations().iter().map(|h| h.id().clone()).collect())
        } else {
            Err(QueryError::RegistryError("Configuration list not found".to_string()))
        }
    }

    fn get_build_setting(&self, config_id: &ObjectId, key: &str) -> QueryResult<Option<String>> {
        if let Some(config) = self.get::<XCBuildConfiguration>(config_id) {
            Ok(config.build_settings().get(key).cloned())
        } else {
            Err(QueryError::ConfigurationNotFound(config_id.to_string()))
        }
    }

    fn get_target_dependencies(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(target) = self.get::<PBXNativeTarget>(target_id) {
            return Ok(target.dependencies().to_vec());
        }
        if let Some(target) = self.get::<PBXAggregateTarget>(target_id) {
            // PBXAggregateTarget uses Handle<PBXTargetDependency>
            return Ok(target.dependencies.iter().map(|h| h.id().clone()).collect());
        }
        Err(QueryError::RegistryError("Not a valid target".to_string()))
    }

    fn find_dependency_target(&self, dependency_id: &ObjectId) -> QueryResult<Option<ObjectId>> {
        if let Some(dep) = self.get::<PBXTargetDependency>(dependency_id) {
            return Ok(dep.target.clone());
        }
        Ok(None)
    }

    fn get_swift_package_dependencies(&self, target_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        if let Some(target) = self.get::<PBXNativeTarget>(target_id) {
            return Ok(target.package_product_dependencies().to_vec());
        }
        Ok(Vec::new())
    }

    fn find_files_by_extension(&self, group_id: &ObjectId, extension: &str) -> QueryResult<Vec<ObjectId>> {
        let mut result = Vec::new();
        
        if let Some(group) = self.get::<PBXGroup>(group_id) {
            for child in group.children() {
                if let Some(file_ref) = self.get::<PBXFileReference>(child) {
                    if let Some(path) = file_ref.path() {
                        if path.ends_with(&format!(".{}", extension)) {
                            result.push(child.clone());
                        }
                    }
                }
                if self.get::<PBXGroup>(child).is_some() {
                    if let Ok(mut subfiles) = self.find_files_by_extension(child, extension) {
                        result.append(&mut subfiles);
                    }
                }
            }
        }
        
        Ok(result)
    }

    fn find_all_source_files(&self, group_id: &ObjectId) -> QueryResult<Vec<ObjectId>> {
        let source_extensions = vec!["swift", "m", "mm", "c", "cpp", "cc", "cxx", "h", "hpp"];
        let mut result = Vec::new();
        
        for ext in source_extensions {
            if let Ok(mut files) = self.find_files_by_extension(group_id, ext) {
                result.append(&mut files);
            }
        }
        
        Ok(result)
    }

    fn get_all_build_settings(&self, config_id: &ObjectId) -> QueryResult<std::collections::HashMap<String, String>> {
        if let Some(config) = self.get::<XCBuildConfiguration>(config_id) {
            Ok(config.build_settings().clone().into_iter().collect())
        } else {
            Err(QueryError::ConfigurationNotFound(config_id.to_string()))
        }
    }

    fn compare_build_settings(&self, config1_id: &ObjectId, config2_id: &ObjectId) -> QueryResult<Vec<(String, Option<String>, Option<String>)>> {
        let settings1 = self.get_all_build_settings(config1_id)?;
        let settings2 = self.get_all_build_settings(config2_id)?;
        
        let mut result = Vec::new();
        let mut all_keys = std::collections::HashSet::<String>::new();
        
        for key in settings1.keys() {
            all_keys.insert(key.clone());
        }
        for key in settings2.keys() {
            all_keys.insert(key.clone());
        }
        
        for key in all_keys {
            let val1 = settings1.get(&key).cloned();
            let val2 = settings2.get(&key).cloned();
            
            if val1 != val2 {
                result.push((key, val1, val2));
            }
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xforge_objects::*;

    #[test]
    fn test_registry_ext() {
        let registry = Registry::new();
        // Basic smoke test - just ensure trait is available
    }
    
    #[test]
    fn test_get_target_dependencies() {
        let mut registry = Registry::new();
        
        // Create a native target with dependencies
        let dep1 = PBXTargetDependency::new();
        let dep1_handle = registry.register(dep1);
        let dep1_id = dep1_handle.id().clone();
        
        let dep2 = PBXTargetDependency::new();
        let dep2_handle = registry.register(dep2);
        let dep2_id = dep2_handle.id().clone();
        
        let mut target = PBXNativeTarget::new("TestTarget");
        target.dependencies.push(dep1_id.clone());
        target.dependencies.push(dep2_id.clone());
        let target_id = registry.register(target).id().clone();
        
        // Test getting dependencies
        let deps = registry.get_target_dependencies(&target_id).unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&dep1_id));
        assert!(deps.contains(&dep2_id));
    }
    
    #[test]
    fn test_find_dependency_target() {
        let mut registry = Registry::new();
        
        // Create a target
        let target = PBXNativeTarget::new("MainTarget");
        let target_id = registry.register(target).id().clone();
        
        // Create a dependency pointing to that target
        let dep = PBXTargetDependency::new().with_target(target_id.clone());
        let dep_id = registry.register(dep);
        
        // Test finding the target
        let found = registry.find_dependency_target(&dep_id.id()).unwrap();
        assert_eq!(found, Some(target_id));
    }
    
    #[test]
    fn test_find_dependency_target_none() {
        let mut registry = Registry::new();
        
        // Create a dependency without a target
        let dep = PBXTargetDependency::new();
        let dep_id = registry.register(dep);
        
        let found = registry.find_dependency_target(&dep_id.id()).unwrap();
        assert_eq!(found, None);
    }
    
    #[test]
    fn test_get_swift_package_dependencies() {
        let mut registry = Registry::new();
        
        // Create a native target with package dependencies
        let mut target = PBXNativeTarget::new("TestTarget");
        let pkg1 = ObjectId::generate();
        let pkg2 = ObjectId::generate();
        target.package_product_dependencies.push(pkg1.clone());
        target.package_product_dependencies.push(pkg2.clone());
        let target_id = registry.register(target).id().clone();
        
        // Test getting package dependencies
        let pkgs = registry.get_swift_package_dependencies(&target_id).unwrap();
        assert_eq!(pkgs.len(), 2);
        assert!(pkgs.contains(&pkg1));
        assert!(pkgs.contains(&pkg2));
    }
    
    #[test]
    fn test_find_files_by_extension() {
        let mut registry = Registry::new();
        
        // Create a group with various files
        let mut group = PBXGroup::new("");
        
        let swift_file1 = PBXFileReference::new("File1.swift");
        let swift_file1_handle = registry.register(swift_file1);
        let swift_file1_id = swift_file1_handle.id().clone();
        group.add_child(swift_file1_handle);
        
        let swift_file2 = PBXFileReference::new("File2.swift");
        let swift_file2_handle = registry.register(swift_file2);
        let swift_file2_id = swift_file2_handle.id().clone();
        group.add_child(swift_file2_handle);
        
        let h_file = PBXFileReference::new("Header.h");
        let h_file_handle = registry.register(h_file);
        let h_file_id = h_file_handle.id().clone();
        group.add_child(h_file_handle);
        
        let group_id = registry.register(group).id().clone();
        
        // Test finding Swift files
        let swift_files = registry.find_files_by_extension(&group_id, "swift").unwrap();
        assert_eq!(swift_files.len(), 2);
        assert!(swift_files.contains(&swift_file1_id));
        assert!(swift_files.contains(&swift_file2_id));
        
        // Test finding header files
        let h_files = registry.find_files_by_extension(&group_id, "h").unwrap();
        assert_eq!(h_files.len(), 1);
    }
    
    #[test]
    fn test_find_files_by_extension_multiple() {
        let mut registry = Registry::new();
        
        // Create a group with multiple file types
        let mut group = PBXGroup::new("TestGroup");
        
        let swift1 = PBXFileReference::new("File1.swift");
        let handle1 = registry.register(swift1);
        let id1 = handle1.id().clone();
        group.add_child(handle1);
        
        let swift2 = PBXFileReference::new("File2.swift");
        let handle2 = registry.register(swift2);
        let id2 = handle2.id().clone();
        group.add_child(handle2);
        
        let objc = PBXFileReference::new("File.m");
        let handle3 = registry.register(objc);
        group.add_child(handle3);
        
        let group_id = registry.register(group).id().clone();
        
        // Test finding Swift files
        let swift_files = registry.find_files_by_extension(&group_id, "swift").unwrap();
        assert_eq!(swift_files.len(), 2);
        assert!(swift_files.contains(&id1));
        assert!(swift_files.contains(&id2));
        
        // Test finding Objective-C files
        let m_files = registry.find_files_by_extension(&group_id, "m").unwrap();
        assert_eq!(m_files.len(), 1);
    }
    
    #[test]
    fn test_find_all_source_files() {
        let mut registry = Registry::new();
        
        let mut group = PBXGroup::new("TestGroup");
        
        // Add various source files
        let swift_file = PBXFileReference::new("File.swift");
        let swift_handle = registry.register(swift_file);
        let swift_id = swift_handle.id().clone();
        
        let m_file = PBXFileReference::new("File.m");
        let m_handle = registry.register(m_file);
        let m_id = m_handle.id().clone();
        
        let h_file = PBXFileReference::new("File.h");
        let h_handle = registry.register(h_file);
        let h_id = h_handle.id().clone();
        
        // Add a non-source file
        let json_file = PBXFileReference::new("data.json");
        let json_handle = registry.register(json_file);
        
        group.add_child(swift_handle);
        group.add_child(m_handle);
        group.add_child(h_handle);
        group.add_child(json_handle);
        
        let group_id = registry.register(group).id().clone();
        
        // Test finding all source files
        let sources = registry.find_all_source_files(&group_id).unwrap();
        assert_eq!(sources.len(), 3);
        assert!(sources.contains(&swift_id));
        assert!(sources.contains(&m_id));
        assert!(sources.contains(&h_id));
    }
    
    #[test]
    fn test_get_all_build_settings() {
        let mut registry = Registry::new();
        
        let mut config = XCBuildConfiguration::new("Debug");
        config.set_build_setting("PRODUCT_NAME", "MyApp");
        config.set_build_setting("SDKROOT", "iphoneos");
        let config_id = registry.register(config).id().clone();
        
        // Test getting all settings
        let settings = registry.get_all_build_settings(&config_id).unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings.get("PRODUCT_NAME"), Some(&"MyApp".to_string()));
        assert_eq!(settings.get("SDKROOT"), Some(&"iphoneos".to_string()));
    }
    
    #[test]
    fn test_compare_build_settings() {
        let mut registry = Registry::new();
        
        // Create Debug configuration
        let mut debug_config = XCBuildConfiguration::new("Debug");
        debug_config.set_build_setting("OPTIMIZATION_LEVEL", "0");
        debug_config.set_build_setting("ENABLE_TESTABILITY", "YES");
        debug_config.set_build_setting("PRODUCT_NAME", "MyApp");
        let debug_id = registry.register(debug_config);
        
        // Create Release configuration
        let mut release_config = XCBuildConfiguration::new("Release");
        release_config.set_build_setting("OPTIMIZATION_LEVEL", "2");
        release_config.set_build_setting("ENABLE_TESTABILITY", "NO");
        release_config.set_build_setting("PRODUCT_NAME", "MyApp");
        let release_id = registry.register(release_config);
        
        // Test comparison
        let diffs = registry.compare_build_settings(&debug_id.id(), &release_id.id()).unwrap();
        
        // Should have 2 differences (OPTIMIZATION_LEVEL and ENABLE_TESTABILITY)
        // PRODUCT_NAME is the same so it shouldn't appear
        assert_eq!(diffs.len(), 2);
        
        // Check the differences
        let opt_diff = diffs.iter().find(|(k, _, _)| k == "OPTIMIZATION_LEVEL");
        assert!(opt_diff.is_some());
        let (_, debug_val, release_val) = opt_diff.unwrap();
        assert_eq!(debug_val, &Some("0".to_string()));
        assert_eq!(release_val, &Some("2".to_string()));
    }
    
    #[test]
    fn test_compare_build_settings_missing_key() {
        let mut registry = Registry::new();
        
        // Create configurations with different keys
        let mut config1 = XCBuildConfiguration::new("Config1");
        config1.set_build_setting("KEY1", "value1");
        let config1_id = registry.register(config1);
        
        let mut config2 = XCBuildConfiguration::new("Config2");
        config2.set_build_setting("KEY2", "value2");
        let config2_id = registry.register(config2);
        
        // Test comparison
        let diffs = registry.compare_build_settings(&config1_id.id(), &config2_id.id()).unwrap();
        
        // Should have 2 differences (each key missing from one config)
        assert_eq!(diffs.len(), 2);
    }
}

#[cfg(test)]
mod new_query_tests {
    use super::*;
    use xforge_objects::*;

    #[test]
    fn test_get_target_dependencies() {
        let mut registry = Registry::new();
        
        // Create a target with dependencies
        let mut target = PBXNativeTarget::new("TestTarget");
        let dep1 = registry.register(PBXTargetDependency::new()).id().clone();
        let dep2 = registry.register(PBXTargetDependency::new()).id().clone();
        target.dependencies.push(dep1.clone());
        target.dependencies.push(dep2.clone());
        
        let target_id = registry.register(target).id().clone();
        
        // Test the query
        let deps = registry.get_target_dependencies(&target_id).unwrap();
        assert_eq!(deps.len(), 2);
        assert!(deps.contains(&dep1));
        assert!(deps.contains(&dep2));
    }

    #[test]
    fn test_find_dependency_target() {
        let mut registry = Registry::new();
        
        // Create a dependency with a target
        let target = registry.register(PBXNativeTarget::new("TargetA")).id().clone();
        let mut dep = PBXTargetDependency::new();
        dep.target = Some(target.clone());
        
        let dep_id = registry.register(dep).id().clone();
        
        // Test the query
        let found = registry.find_dependency_target(&dep_id).unwrap();
        assert_eq!(found, Some(target));
    }

    #[test]
    fn test_get_swift_package_dependencies() {
        let mut registry = Registry::new();
        
        // Create a target with package dependencies
        let mut target = PBXNativeTarget::new("TestTarget");
        let pkg_dep1 = ObjectId::generate();
        let pkg_dep2 = ObjectId::generate();
        target.package_product_dependencies.push(pkg_dep1.clone());
        target.package_product_dependencies.push(pkg_dep2.clone());
        
        let target_id = registry.register(target).id().clone();
        
        // Test the query
        let pkg_deps = registry.get_swift_package_dependencies(&target_id).unwrap();
        assert_eq!(pkg_deps.len(), 2);
        assert!(pkg_deps.contains(&pkg_dep1));
        assert!(pkg_deps.contains(&pkg_dep2));
    }

    #[test]
    fn test_find_files_by_extension() {
        let mut registry = Registry::new();
        
        // Create a group with files
        let mut group = PBXGroup::new("TestGroup");
        
        let swift_file = PBXFileReference::new("main.swift");
        let swift_handle = registry.register(swift_file);
        let swift_id = swift_handle.id().clone();
        group.add_child(swift_handle);
        
        let objc_file = PBXFileReference::new("Helper.m");
        let objc_handle = registry.register(objc_file);
        group.add_child(objc_handle);
        
        let group_id = registry.register(group).id().clone();
        
        // Test finding Swift files
        let swift_files = registry.find_files_by_extension(&group_id, "swift").unwrap();
        assert_eq!(swift_files.len(), 1);
        assert_eq!(swift_files[0], swift_id);
    }

    #[test]
    fn test_find_all_source_files() {
        let mut registry = Registry::new();
        
        // Create a group with various file types
        let mut group = PBXGroup::new("TestGroup");
        
        let swift_file = PBXFileReference::new("main.swift");
        let swift_handle = registry.register(swift_file);
        group.add_child(swift_handle);
        
        let h_file = PBXFileReference::new("Header.h");
        let h_handle = registry.register(h_file);
        group.add_child(h_handle);
        
        let txt_file = PBXFileReference::new("README.txt");
        let txt_handle = registry.register(txt_file);
        group.add_child(txt_handle);
        
        let group_id = registry.register(group).id().clone();
        
        // Test finding all source files (should exclude txt)
        let source_files = registry.find_all_source_files(&group_id).unwrap();
        assert_eq!(source_files.len(), 2);
    }

    #[test]
    fn test_get_all_build_settings() {
        let mut registry = Registry::new();
        
        // Create a configuration with settings
        let mut config = XCBuildConfiguration::new("Debug");
        config.set_build_setting("PRODUCT_NAME", "MyApp");
        config.set_build_setting("SWIFT_VERSION", "5.0");
        
        let config_id = registry.register(config).id().clone();
        
        // Test the query
        let settings = registry.get_all_build_settings(&config_id).unwrap();
        assert_eq!(settings.len(), 2);
        assert_eq!(settings.get("PRODUCT_NAME"), Some(&"MyApp".to_string()));
        assert_eq!(settings.get("SWIFT_VERSION"), Some(&"5.0".to_string()));
    }

    #[test]
    fn test_compare_build_settings() {
        let mut registry = Registry::new();
        
        // Create Debug configuration
        let mut debug_config = XCBuildConfiguration::new("Debug");
        debug_config.set_build_setting("GCC_OPTIMIZATION_LEVEL", "0");
        debug_config.set_build_setting("SWIFT_OPTIMIZATION_LEVEL", "-Onone");
        debug_config.set_build_setting("PRODUCT_NAME", "MyApp");
        
        let debug_id = registry.register(debug_config).id().clone();
        
        // Create Release configuration
        let mut release_config = XCBuildConfiguration::new("Release");
        release_config.set_build_setting("GCC_OPTIMIZATION_LEVEL", "s");
        release_config.set_build_setting("SWIFT_OPTIMIZATION_LEVEL", "-O");
        release_config.set_build_setting("PRODUCT_NAME", "MyApp");
        
        let release_id = registry.register(release_config).id().clone();
        
        // Test comparison
        let diffs = registry.compare_build_settings(&debug_id, &release_id).unwrap();
        
        // Should have 2 differences (optimization levels)
        assert_eq!(diffs.len(), 2);
        
        // Check that PRODUCT_NAME is not in diffs (same in both)
        assert!(!diffs.iter().any(|(key, _, _)| key == "PRODUCT_NAME"));
    }
}
