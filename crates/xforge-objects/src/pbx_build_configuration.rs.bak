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
    
    /// Append a value to an array build setting (e.g., FRAMEWORK_SEARCH_PATHS, OTHER_LDFLAGS)
    /// Automatically adds $(inherited) if not present and deduplicates values
    pub fn append_to_array_setting(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value = value.into();
        let current = self.build_settings.entry(key)
            .or_insert_with(|| "$(inherited)".to_string());
        
        // Parse current value
        let mut items = parse_array_setting(current);
        
        // Ensure $(inherited) is present
        if !items.contains(&"$(inherited)".to_string()) {
            items.insert(0, "$(inherited)".to_string());
        }
        
        // Add new value if not already present
        if !items.contains(&value) {
            items.push(value);
        }
        
        // Format back
        *current = format_array_setting(&items);
    }
    
    /// Remove a value from an array build setting
    pub fn remove_from_array_setting(&mut self, key: &str, value: &str) {
        if let Some(current) = self.build_settings.get_mut(key) {
            let mut items = parse_array_setting(current);
            items.retain(|item| item != value);
            *current = format_array_setting(&items);
        }
    }
    
    /// Get an array build setting as Vec<String>
    pub fn get_array_setting(&self, key: &str) -> Vec<String> {
        self.build_settings.get(key)
            .map(|v| parse_array_setting(v))
            .unwrap_or_default()
    }
}

/// Parse an array-format build setting value
/// Handles both single values and array format: (item1, item2, item3)
fn parse_array_setting(value: &str) -> Vec<String> {
    let trimmed = value.trim();
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        // Array format: (item1, item2, item3)
        trimmed[1..trimmed.len()-1]
            .split(',')
            .map(|s| s.trim().trim_matches('"').to_string())
            .filter(|s| !s.is_empty())
            .collect()
    } else {
        // Single value
        vec![trimmed.to_string()]
    }
}

/// Format items as array-format build setting value
fn format_array_setting(items: &[String]) -> String {
    if items.len() == 1 {
        items[0].clone()
    } else {
        let quoted: Vec<String> = items.iter()
            .map(|s| if s.contains(' ') || s.starts_with('$') {
                format!("\"{}\"", s)
            } else {
                s.clone()
            })
            .collect();
        format!("({})", quoted.join(", "))
    }
}

impl PBXObject for XCBuildConfiguration {
    fn isa(&self) -> &'static str {
        "XCBuildConfiguration"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
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
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
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
    
    #[test]
    fn test_parse_array_setting() {
        assert_eq!(parse_array_setting("single"), vec!["single"]);
        assert_eq!(
            parse_array_setting("(a, b, c)"),
            vec!["a", "b", "c"]
        );
        assert_eq!(
            parse_array_setting("(\"$(inherited)\", \"-ObjC\")"),
            vec!["$(inherited)", "-ObjC"]
        );
    }
    
    #[test]
    fn test_format_array_setting() {
        assert_eq!(format_array_setting(&["single".to_string()]), "single");
        assert_eq!(
            format_array_setting(&["a".to_string(), "b".to_string()]),
            "(a, b)"
        );
        assert_eq!(
            format_array_setting(&["$(inherited)".to_string(), "-ObjC".to_string()]),
            "(\"$(inherited)\", -ObjC)"
        );
    }
    
    #[test]
    fn test_append_to_array_setting() {
        let mut config = XCBuildConfiguration::new("Debug");
        
        // First append
        config.append_to_array_setting("OTHER_LDFLAGS", "-ObjC");
        let value = config.build_settings.get("OTHER_LDFLAGS").unwrap();
        assert!(value.contains("$(inherited)"));
        assert!(value.contains("-ObjC"));
        
        // Second append (should not duplicate)
        config.append_to_array_setting("OTHER_LDFLAGS", "-ObjC");
        let items = config.get_array_setting("OTHER_LDFLAGS");
        assert_eq!(items.iter().filter(|&s| s == "-ObjC").count(), 1);
        
        // Third append (different value)
        config.append_to_array_setting("OTHER_LDFLAGS", "-lc++");
        let items = config.get_array_setting("OTHER_LDFLAGS");
        assert_eq!(items.len(), 3); // $(inherited), -ObjC, -lc++
    }
    
    #[test]
    fn test_remove_from_array_setting() {
        let mut config = XCBuildConfiguration::new("Debug");
        config.build_settings.insert(
            "OTHER_LDFLAGS".to_string(),
            "(\"$(inherited)\", -ObjC, -lc++)".to_string()
        );
        
        config.remove_from_array_setting("OTHER_LDFLAGS", "-ObjC");
        let items = config.get_array_setting("OTHER_LDFLAGS");
        assert!(!items.contains(&"-ObjC".to_string()));
        assert!(items.contains(&"$(inherited)".to_string()));
        assert!(items.contains(&"-lc++".to_string()));
    }
    
    #[test]
    fn test_get_array_setting() {
        let mut config = XCBuildConfiguration::new("Debug");
        config.build_settings.insert(
            "FRAMEWORK_SEARCH_PATHS".to_string(),
            "(\"$(inherited)\", \"$(PROJECT_DIR)/Frameworks\")".to_string()
        );
        
        let items = config.get_array_setting("FRAMEWORK_SEARCH_PATHS");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0], "$(inherited)");
        assert_eq!(items[1], "$(PROJECT_DIR)/Frameworks");
    }
}
