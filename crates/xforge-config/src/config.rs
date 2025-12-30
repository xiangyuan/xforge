use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use indexmap::IndexMap;
use crate::{conditional::{BuildContext, ConditionalSetting}, parser::Parser, Result};

/// Represents an Xcode build configuration file (.xcconfig)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XCConfig {
    /// The file path this config was loaded from (if any)
    pub path: Option<PathBuf>,
    
    /// Settings in this config (preserves order)
    settings: IndexMap<String, Vec<ConditionalSetting>>,
    
    /// Include file paths
    includes: Vec<PathBuf>,
}

impl XCConfig {
    /// Creates a new empty config
    pub fn new() -> Self {
        Self {
            path: None,
            settings: IndexMap::new(),
            includes: Vec::new(),
        }
    }
    
    /// Loads a config from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;
        
        let mut config = Self::parse(&content)?;
        config.path = Some(path.to_path_buf());
        
        Ok(config)
    }
    
    /// Saves the config to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = self.to_string();
        fs::write(path.as_ref(), content)
            .map_err(|e| anyhow::anyhow!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    /// Parses config from string
    pub fn parse(content: &str) -> Result<Self> {
        let parse_result = Parser::parse(content)?;
        
        let mut config = Self::new();
        
        // Add includes
        for include in parse_result.includes {
            config.includes.push(PathBuf::from(include));
        }
        
        // Add settings
        for setting in parse_result.settings {
            config.add_setting(setting);
        }
        
        Ok(config)
    }
    
    /// Adds a setting
    pub fn add_setting(&mut self, setting: ConditionalSetting) {
        self.settings
            .entry(setting.key.clone())
            .or_insert_with(Vec::new)
            .push(setting);
    }
    
    /// Sets a simple (unconditional) setting
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        self.settings.insert(key.clone(), vec![ConditionalSetting::new(key, value)]);
    }
    
    /// Gets a setting value for a given context
    pub fn get(&self, key: &str, context: &BuildContext) -> Option<String> {
        let settings = self.settings.get(key)?;
        
        // Find the first matching setting (last one wins)
        for setting in settings.iter().rev() {
            if setting.applies_to(context) {
                return Some(self.expand_variables(&setting.value, context));
            }
        }
        
        None
    }
    
    /// Gets all settings as a flat map (without conditions)
    pub fn all_settings(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        let context = BuildContext::new();
        
        for (key, settings) in &self.settings {
            if let Some(setting) = settings.last() {
                result.insert(key.clone(), self.expand_variables(&setting.value, &context));
            }
        }
        
        result
    }
    
    /// Adds an include directive
    pub fn add_include<P: Into<PathBuf>>(&mut self, path: P) {
        self.includes.push(path.into());
    }
    
    /// Gets all include paths
    pub fn includes(&self) -> &[PathBuf] {
        &self.includes
    }
    
    /// Expands variable references like $(inherited) and $(VAR_NAME)
    fn expand_variables(&self, value: &str, context: &BuildContext) -> String {
        let mut result = value.to_string();
        
        // Handle $(inherited) - just return as-is for now
        // In a real implementation, this would merge with parent configs
        
        // Handle other variable references
        while let Some(start) = result.find("$(") {
            if let Some(end) = result[start..].find(')') {
                let var_name = &result[start + 2..start + end];
                let replacement = context.variables.get(var_name)
                    .map(|s| s.as_str())
                    .unwrap_or("");
                
                result.replace_range(start..start + end + 1, replacement);
            } else {
                break;
            }
        }
        
        result
    }
    
    /// Converts the config to string format
    pub fn to_string(&self) -> String {
        let mut lines = Vec::new();
        
        // Add includes
        for include in &self.includes {
            lines.push(format!(r#"#include "{}""#, include.display()));
        }
        
        if !self.includes.is_empty() && !self.settings.is_empty() {
            lines.push(String::new()); // Empty line after includes
        }
        
        // Add settings
        for (_, settings) in &self.settings {
            for setting in settings {
                let line = if let Some(cond) = &setting.condition {
                    format!("{}[{}] = {}", setting.key, cond, setting.value)
                } else {
                    format!("{} = {}", setting.key, setting.value)
                };
                lines.push(line);
            }
        }
        
        lines.join("\n") + "\n"
    }
}

impl Default for XCConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = XCConfig::new();
        assert!(config.settings.is_empty());
        assert!(config.includes.is_empty());
    }

    #[test]
    fn test_set_and_get() {
        let mut config = XCConfig::new();
        config.set("FRAMEWORK_SEARCH_PATHS", "/path/to/frameworks");
        
        let context = BuildContext::new();
        let value = config.get("FRAMEWORK_SEARCH_PATHS", &context).unwrap();
        assert_eq!(value, "/path/to/frameworks");
    }

    #[test]
    fn test_conditional_setting() {
        let mut config = XCConfig::new();
        config.add_setting(ConditionalSetting::new("FRAMEWORK_SEARCH_PATHS", "/default"));
        config.add_setting(ConditionalSetting::with_condition(
            "FRAMEWORK_SEARCH_PATHS",
            "/ios",
            "sdk=iphoneos*"
        ));
        
        let default_context = BuildContext::new();
        let ios_context = BuildContext::with_sdk("iphoneos16.0");
        
        assert_eq!(config.get("FRAMEWORK_SEARCH_PATHS", &default_context).unwrap(), "/default");
        assert_eq!(config.get("FRAMEWORK_SEARCH_PATHS", &ios_context).unwrap(), "/ios");
    }

    #[test]
    fn test_includes() {
        let mut config = XCConfig::new();
        config.add_include("Base.xcconfig");
        config.add_include("Debug.xcconfig");
        
        assert_eq!(config.includes().len(), 2);
    }

    #[test]
    fn test_parse_and_serialize() {
        let content = r#"#include "Base.xcconfig"

FRAMEWORK_SEARCH_PATHS = /common
FRAMEWORK_SEARCH_PATHS[sdk=iphoneos*] = /ios
"#;
        
        let config = XCConfig::parse(content).unwrap();
        assert_eq!(config.includes.len(), 1);
        assert_eq!(config.settings.len(), 1);
        
        let serialized = config.to_string();
        assert!(serialized.contains(r#"#include "Base.xcconfig""#));
        assert!(serialized.contains("FRAMEWORK_SEARCH_PATHS = /common"));
    }

    #[test]
    fn test_all_settings() {
        let mut config = XCConfig::new();
        config.set("KEY1", "value1");
        config.set("KEY2", "value2");
        
        let all = config.all_settings();
        assert_eq!(all.len(), 2);
        assert_eq!(all.get("KEY1").unwrap(), "value1");
        assert_eq!(all.get("KEY2").unwrap(), "value2");
    }
}
