//! Build file settings support

use std::collections::HashMap;

/// Build file setting value - can be string or array of strings
#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue {
    String(String),
    Array(Vec<String>),
}

impl SettingValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            SettingValue::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_array(&self) -> Option<&[String]> {
        match self {
            SettingValue::Array(arr) => Some(arr),
            _ => None,
        }
    }
    
    pub fn to_array(&self) -> Vec<String> {
        match self {
            SettingValue::String(s) => vec![s.clone()],
            SettingValue::Array(arr) => arr.clone(),
        }
    }
}

impl From<String> for SettingValue {
    fn from(s: String) -> Self {
        SettingValue::String(s)
    }
}

impl From<&str> for SettingValue {
    fn from(s: &str) -> Self {
        SettingValue::String(s.to_string())
    }
}

impl From<Vec<String>> for SettingValue {
    fn from(arr: Vec<String>) -> Self {
        SettingValue::Array(arr)
    }
}

/// Build file settings
pub type BuildFileSettings = HashMap<String, SettingValue>;

/// Common build file attributes
pub mod attributes {
    pub const WEAK: &str = "Weak";
    pub const OPTIONAL: &str = "Optional";
    pub const CODE_SIGN_ON_COPY: &str = "CodeSignOnCopy";
    pub const REMOVE_HEADERS_ON_COPY: &str = "RemoveHeadersOnCopy";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_value_string() {
        let val = SettingValue::from("test");
        assert_eq!(val.as_string(), Some("test"));
        assert!(val.as_array().is_none());
    }

    #[test]
    fn test_setting_value_array() {
        let val = SettingValue::from(vec!["a".to_string(), "b".to_string()]);
        assert!(val.as_string().is_none());
        assert_eq!(val.as_array(), Some(&["a".to_string(), "b".to_string()][..]));
    }

    #[test]
    fn test_to_array() {
        let str_val = SettingValue::from("test");
        assert_eq!(str_val.to_array(), vec!["test"]);
        
        let arr_val = SettingValue::from(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(arr_val.to_array(), vec!["a", "b"]);
    }
}
