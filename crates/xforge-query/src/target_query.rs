//! Target-level query API

use xforge_core::{Handle, Registry};
use xforge_objects::{
    PBXNativeTarget, PBXSourcesBuildPhase, PBXFrameworksBuildPhase,
    PBXResourcesBuildPhase, XCBuildConfiguration, XCConfigurationList,
};
use crate::errors::{QueryError, QueryResult};

/// Query API for Target operations
pub struct TargetQuery<'a> {
    target: &'a PBXNativeTarget,
    registry: &'a Registry,
}

impl<'a> TargetQuery<'a> {
    pub fn new(target: &'a PBXNativeTarget, registry: &'a Registry) -> Self {
        Self { target, registry }
    }

    pub fn name(&self) -> &str {
        self.target.name()
    }

    pub fn product_name(&self) -> Option<&str> {
        self.target.product_name()
    }

    pub fn find_configuration(&self, name: &str) -> QueryResult<Handle<XCBuildConfiguration>> {
        if let Some(config_list_handle) = self.target.build_configuration_list() {
            if let Some(config_list) = self.registry.get::<XCConfigurationList>(config_list_handle.id()) {
                for config_handle in config_list.build_configurations() {
                    if let Some(config) = self.registry.get::<XCBuildConfiguration>(config_handle.id()) {
                        if config.name() == name {
                            return Ok(*config_handle);
                        }
                    }
                }
            }
        }
        Err(QueryError::ConfigurationNotFound(name.to_string()))
    }

    pub fn configurations(&self) -> Vec<Handle<XCBuildConfiguration>> {
        if let Some(config_list_handle) = self.target.build_configuration_list() {
            if let Some(config_list) = self.registry.get::<XCConfigurationList>(config_list_handle.id()) {
                return config_list.build_configurations().to_vec();
            }
        }
        Vec::new()
    }

    pub fn sources_phase(&self) -> QueryResult<Handle<PBXSourcesBuildPhase>> {
        for phase_handle in self.target.build_phases() {
            if self.registry.get::<PBXSourcesBuildPhase>(phase_handle.id()).is_some() {
                return Ok(Handle::from_id(*phase_handle.id()));
            }
        }
        Err(QueryError::BuildPhaseNotFound("Sources".to_string()))
    }

    pub fn frameworks_phase(&self) -> QueryResult<Handle<PBXFrameworksBuildPhase>> {
        for phase_handle in self.target.build_phases() {
            if self.registry.get::<PBXFrameworksBuildPhase>(phase_handle.id()).is_some() {
                return Ok(Handle::from_id(*phase_handle.id()));
            }
        }
        Err(QueryError::BuildPhaseNotFound("Frameworks".to_string()))
    }

    pub fn resources_phase(&self) -> QueryResult<Handle<PBXResourcesBuildPhase>> {
        for phase_handle in self.target.build_phases() {
            if self.registry.get::<PBXResourcesBuildPhase>(phase_handle.id()).is_some() {
                return Ok(Handle::from_id(*phase_handle.id()));
            }
        }
        Err(QueryError::BuildPhaseNotFound("Resources".to_string()))
    }

    pub fn build_setting(&self, config_name: &str, key: &str) -> QueryResult<Option<String>> {
        let config = self.find_configuration(config_name)?;
        if let Some(config_obj) = self.registry.get::<XCBuildConfiguration>(config.id()) {
            Ok(config_obj.build_settings().get(key).cloned())
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_query_name() {
        let registry = Registry::new();
        let target = PBXNativeTarget::new("TestTarget");
        let query = TargetQuery::new(&target, &registry);
        assert_eq!(query.name(), "TestTarget");
    }
}
