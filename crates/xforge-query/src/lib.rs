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
                if let Some(file_ref) = self.get::<PBXFileReference>(child.id()) {
                    if file_ref.path() == Some(path) {
                        return Ok(child.id().clone());
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
                    if let Some(child_group) = self.get::<PBXGroup>(child.id()) {
                        if child_group.path() == Some(component) {
                            current_group_id = child.id().clone();
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
            Ok(group.children().iter().map(|h| h.id().clone()).collect())
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_ext() {
        let registry = Registry::new();
        // Basic smoke test - just ensure trait is available
    }
}
