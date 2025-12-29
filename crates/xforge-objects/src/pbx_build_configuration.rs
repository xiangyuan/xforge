//! Build configurations

use xforge_core::{ObjectId, Handle, PBXObject};
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct XCBuildConfiguration {
    id: ObjectId,
    pub name: String,
    pub build_settings: IndexMap<String, String>,
}

impl XCBuildConfiguration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            name: name.into(),
            build_settings: IndexMap::new(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn build_settings(&self) -> &IndexMap<String, String> {
        &self.build_settings
    }
    
    pub fn set_build_setting(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.build_settings.insert(key.into(), value.into());
    }
}

impl PBXObject for XCBuildConfiguration {
    fn isa(&self) -> &'static str {
        "XCBuildConfiguration"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

#[derive(Debug, Clone)]
pub struct XCConfigurationList {
    id: ObjectId,
    pub build_configurations: Vec<Handle<XCBuildConfiguration>>,
    pub default_configuration_name: Option<String>,
    pub default_configuration_is_visible: bool,
}

impl XCConfigurationList {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            build_configurations: Vec::new(),
            default_configuration_name: None,
            default_configuration_is_visible: false,
        }
    }
    
    pub fn build_configurations(&self) -> &[Handle<XCBuildConfiguration>] {
        &self.build_configurations
    }
    
    pub fn default_configuration_name(&self) -> Option<&str> {
        self.default_configuration_name.as_deref()
    }
    
    pub fn default_configuration_is_visible(&self) -> bool {
        self.default_configuration_is_visible
    }
    
    pub fn add_configuration(&mut self, config: Handle<XCBuildConfiguration>) {
        self.build_configurations.push(config);
    }
    
    pub fn set_default_configuration_name(&mut self, name: impl Into<String>) {
        self.default_configuration_name = Some(name.into());
    }
}

impl Default for XCConfigurationList {
    fn default() -> Self {
        Self::new()
    }
}

impl PBXObject for XCConfigurationList {
    fn isa(&self) -> &'static str {
        "XCConfigurationList"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_configuration() {
        let mut config = XCBuildConfiguration::new("Debug");
        config.set_build_setting("SWIFT_VERSION", "5.0");
        assert_eq!(config.name(), "Debug");
        assert_eq!(config.build_settings().get("SWIFT_VERSION").unwrap(), "5.0");
    }

    #[test]
    fn test_configuration_list() {
        let list = XCConfigurationList::new();
        assert_eq!(list.build_configurations().len(), 0);
        assert_eq!(list.default_configuration_is_visible(), false);
    }
}
