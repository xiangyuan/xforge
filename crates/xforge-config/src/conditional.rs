use std::collections::HashMap;

/// Build context for evaluating conditional settings
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildContext {
    /// SDK name (e.g., "iphoneos", "iphonesimulator", "macosx")
    pub sdk: Option<String>,
    /// Configuration name (e.g., "Debug", "Release")
    pub configuration: Option<String>,
    /// Architecture (e.g., "arm64", "x86_64")
    pub arch: Option<String>,
    /// Custom variables
    pub variables: HashMap<String, String>,
}

impl BuildContext {
    /// Creates a new empty build context
    pub fn new() -> Self {
        Self {
            sdk: None,
            configuration: None,
            arch: None,
            variables: HashMap::new(),
        }
    }
    
    /// Creates a build context with SDK
    pub fn with_sdk(sdk: impl Into<String>) -> Self {
        Self {
            sdk: Some(sdk.into()),
            configuration: None,
            arch: None,
            variables: HashMap::new(),
        }
    }
    
    /// Creates a build context with configuration
    pub fn with_configuration(config: impl Into<String>) -> Self {
        Self {
            sdk: None,
            configuration: Some(config.into()),
            arch: None,
            variables: HashMap::new(),
        }
    }
    
    /// Sets a custom variable
    pub fn set_variable(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }
    
    /// Evaluates a condition (e.g., "sdk=iphoneos*", "config=Debug")
    pub fn matches_condition(&self, condition: &str) -> bool {
        if let Some((key, pattern)) = condition.split_once('=') {
            let key = key.trim();
            let pattern = pattern.trim();
            
            match key {
                "sdk" => self.matches_pattern(&self.sdk, pattern),
                "config" | "configuration" => self.matches_pattern(&self.configuration, pattern),
                "arch" => self.matches_pattern(&self.arch, pattern),
                _ => {
                    // Check custom variables
                    if let Some(value) = self.variables.get(key) {
                        self.matches_pattern(&Some(value.clone()), pattern)
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
    
    fn matches_pattern(&self, value: &Option<String>, pattern: &str) -> bool {
        if let Some(val) = value {
            if pattern.ends_with('*') {
                let prefix = &pattern[..pattern.len() - 1];
                val.starts_with(prefix)
            } else {
                val == pattern
            }
        } else {
            false
        }
    }
}

impl Default for BuildContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a conditional build setting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalSetting {
    /// Setting key
    pub key: String,
    /// Setting value
    pub value: String,
    /// Optional condition (e.g., "sdk=iphoneos*")
    pub condition: Option<String>,
}

impl ConditionalSetting {
    /// Creates a new setting without condition
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            condition: None,
        }
    }
    
    /// Creates a new setting with condition
    pub fn with_condition(
        key: impl Into<String>,
        value: impl Into<String>,
        condition: impl Into<String>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            condition: Some(condition.into()),
        }
    }
    
    /// Checks if this setting applies in the given context
    pub fn applies_to(&self, context: &BuildContext) -> bool {
        match &self.condition {
            Some(cond) => context.matches_condition(cond),
            None => true, // Unconditional settings always apply
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_context_sdk_matching() {
        let context = BuildContext::with_sdk("iphoneos");
        assert!(context.matches_condition("sdk=iphoneos"));
        assert!(!context.matches_condition("sdk=iphonesimulator"));
    }

    #[test]
    fn test_build_context_wildcard_matching() {
        let context = BuildContext::with_sdk("iphoneos16.0");
        assert!(context.matches_condition("sdk=iphoneos*"));
        assert!(!context.matches_condition("sdk=iphonesimulator*"));
    }

    #[test]
    fn test_build_context_configuration_matching() {
        let context = BuildContext::with_configuration("Debug");
        assert!(context.matches_condition("config=Debug"));
        assert!(!context.matches_condition("config=Release"));
    }

    #[test]
    fn test_conditional_setting() {
        let setting = ConditionalSetting::with_condition(
            "FRAMEWORK_SEARCH_PATHS",
            "/path/to/frameworks",
            "sdk=iphoneos*"
        );
        
        let ios_context = BuildContext::with_sdk("iphoneos16.0");
        let sim_context = BuildContext::with_sdk("iphonesimulator16.0");
        
        assert!(setting.applies_to(&ios_context));
        assert!(!setting.applies_to(&sim_context));
    }

    #[test]
    fn test_unconditional_setting() {
        let setting = ConditionalSetting::new("ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES", "YES");
        
        let context = BuildContext::new();
        assert!(setting.applies_to(&context));
    }

    #[test]
    fn test_custom_variables() {
        let mut context = BuildContext::new();
        context.set_variable("PLATFORM_NAME", "iphoneos");
        
        assert!(context.matches_condition("PLATFORM_NAME=iphoneos"));
    }
}
