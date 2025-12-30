//! Plist management utilities for Info.plist and other property list files

use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

/// Plist Manager - High-level API for managing property list files
#[derive(Debug, Clone)]
pub struct PlistManager {
    file_path: PathBuf,
    data: plist::Dictionary,
}

impl PlistManager {
    /// Load a plist file from disk
    /// 
    /// # Example
    /// ```no_run
    /// use xforge_model::PlistManager;
    /// 
    /// let plist = PlistManager::load("MyApp/Info.plist")
    ///     .expect("Failed to load Info.plist");
    /// ```
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let file = fs::File::open(path)
            .map_err(|e| format!("Failed to open plist file: {}", e))?;
        
        let plist_value: plist::Value = plist::from_reader(file)
            .map_err(|e| format!("Failed to parse plist: {}", e))?;
        
        let data = match plist_value {
            plist::Value::Dictionary(dict) => dict,
            _ => return Err("Plist root must be a dictionary".to_string()),
        };
        
        Ok(Self {
            file_path: path.to_path_buf(),
            data,
        })
    }

    /// Create a new empty plist
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::new(),
            data: plist::Dictionary::new(),
        }
    }

    /// Get a string value from the plist
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(|v| v.as_string())
    }

    /// Get a boolean value from the plist
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.data.get(key).and_then(|v| v.as_boolean())
    }

    /// Get an integer value from the plist
    pub fn get_integer(&self, key: &str) -> Option<i64> {
        self.data.get(key).and_then(|v| match v {
            plist::Value::Integer(i) => Some(i.as_signed()?),
            _ => None,
        })
    }

    /// Get an array value from the plist
    pub fn get_array(&self, key: &str) -> Option<&Vec<plist::Value>> {
        self.data.get(key).and_then(|v| v.as_array())
    }

    /// Get a dictionary value from the plist
    pub fn get_dict(&self, key: &str) -> Option<&plist::Dictionary> {
        self.data.get(key).and_then(|v| v.as_dictionary())
    }

    /// Set a string value in the plist
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::PlistManager;
    /// # let mut plist = PlistManager::new();
    /// plist.set_string("CFBundleDisplayName", "My App");
    /// plist.set_string("CFBundleVersion", "1.0.0");
    /// ```
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), plist::Value::String(value.into()));
    }

    /// Set a boolean value in the plist
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.data.insert(key.into(), plist::Value::Boolean(value));
    }

    /// Set an integer value in the plist
    pub fn set_integer(&mut self, key: impl Into<String>, value: i64) {
        self.data.insert(key.into(), plist::Value::Integer(value.into()));
    }

    /// Set an array value in the plist
    pub fn set_array(&mut self, key: impl Into<String>, value: Vec<plist::Value>) {
        self.data.insert(key.into(), plist::Value::Array(value));
    }

    /// Add a string to an array in the plist (creates array if it doesn't exist)
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::PlistManager;
    /// # let mut plist = PlistManager::new();
    /// plist.add_to_array("UIRequiredDeviceCapabilities", "armv7");
    /// plist.add_to_array("UIRequiredDeviceCapabilities", "arm64");
    /// ```
    pub fn add_to_array(&mut self, key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value_str = value.into();
        
        match self.data.get_mut(&key) {
            Some(plist::Value::Array(arr)) => {
                // Check if value already exists
                let value_plist = plist::Value::String(value_str.clone());
                if !arr.iter().any(|v| v == &value_plist) {
                    arr.push(value_plist);
                }
            }
            _ => {
                // Create new array
                self.data.insert(key, plist::Value::Array(vec![plist::Value::String(value_str)]));
            }
        }
    }

    /// Remove a value from the plist
    pub fn remove(&mut self, key: &str) -> Option<plist::Value> {
        self.data.remove(key)
    }

    /// Merge another plist into this one (overwrites existing keys)
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::PlistManager;
    /// # let mut base = PlistManager::new();
    /// let override_plist = PlistManager::load("Override.plist")
    ///     .expect("Failed to load override");
    /// base.merge(&override_plist);
    /// ```
    pub fn merge(&mut self, other: &PlistManager) {
        for (key, value) in &other.data {
            self.data.insert(key.clone(), value.clone());
        }
    }

    /// Merge another plist recursively (deep merge for dictionaries and arrays)
    pub fn merge_recursive(&mut self, other: &PlistManager) {
        for (key, other_value) in &other.data {
            match (self.data.get_mut(key), other_value) {
                (Some(plist::Value::Dictionary(base_dict)), plist::Value::Dictionary(other_dict)) => {
                    // Merge dictionaries recursively
                    for (k, v) in other_dict {
                        base_dict.insert(k.clone(), v.clone());
                    }
                }
                (Some(plist::Value::Array(base_arr)), plist::Value::Array(other_arr)) => {
                    // Append unique array elements
                    for item in other_arr {
                        if !base_arr.contains(item) {
                            base_arr.push(item.clone());
                        }
                    }
                }
                _ => {
                    // Overwrite for other types
                    self.data.insert(key.clone(), other_value.clone());
                }
            }
        }
    }

    /// Save the plist to disk in XML format
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::PlistManager;
    /// # let mut plist = PlistManager::new();
    /// plist.set_string("CFBundleVersion", "1.0.1");
    /// plist.save("MyApp/Info.plist")
    ///     .expect("Failed to save Info.plist");
    /// ```
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        let plist_value = plist::Value::Dictionary(self.data.clone());
        let file = fs::File::create(path)
            .map_err(|e| format!("Failed to create plist file: {}", e))?;
        
        plist::to_writer_xml(file, &plist_value)
            .map_err(|e| format!("Failed to write plist: {}", e))?;
        
        Ok(())
    }

    /// Get all keys in the plist
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.file_path
    }

    /// Convert to a HashMap for easier manipulation
    pub fn to_hashmap(&self) -> HashMap<String, plist::Value> {
        self.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

impl Default for PlistManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_plist() {
        let plist = PlistManager::new();
        assert_eq!(plist.data.len(), 0);
    }

    #[test]
    fn test_set_and_get_string() {
        let mut plist = PlistManager::new();
        plist.set_string("CFBundleName", "MyApp");
        
        assert_eq!(plist.get_string("CFBundleName"), Some("MyApp"));
    }

    #[test]
    fn test_set_and_get_bool() {
        let mut plist = PlistManager::new();
        plist.set_bool("UIRequiresFullScreen", true);
        
        assert_eq!(plist.get_bool("UIRequiresFullScreen"), Some(true));
    }

    #[test]
    fn test_set_and_get_integer() {
        let mut plist = PlistManager::new();
        plist.set_integer("UIBackgroundModes", 42);
        
        assert_eq!(plist.get_integer("UIBackgroundModes"), Some(42));
    }

    #[test]
    fn test_add_to_array() {
        let mut plist = PlistManager::new();
        
        plist.add_to_array("UIRequiredDeviceCapabilities", "armv7");
        plist.add_to_array("UIRequiredDeviceCapabilities", "arm64");
        
        let arr = plist.get_array("UIRequiredDeviceCapabilities").unwrap();
        assert_eq!(arr.len(), 2);
        assert_eq!(arr[0].as_string(), Some("armv7"));
        assert_eq!(arr[1].as_string(), Some("arm64"));
    }

    #[test]
    fn test_add_to_array_no_duplicates() {
        let mut plist = PlistManager::new();
        
        plist.add_to_array("UIDeviceFamily", "1");
        plist.add_to_array("UIDeviceFamily", "1"); // duplicate
        plist.add_to_array("UIDeviceFamily", "2");
        
        let arr = plist.get_array("UIDeviceFamily").unwrap();
        assert_eq!(arr.len(), 2); // Should only have 2 unique values
    }

    #[test]
    fn test_remove_key() {
        let mut plist = PlistManager::new();
        plist.set_string("TestKey", "TestValue");
        
        assert!(plist.contains_key("TestKey"));
        plist.remove("TestKey");
        assert!(!plist.contains_key("TestKey"));
    }

    #[test]
    fn test_merge() {
        let mut base = PlistManager::new();
        base.set_string("CFBundleName", "BaseApp");
        base.set_string("CFBundleVersion", "1.0.0");
        
        let mut override_plist = PlistManager::new();
        override_plist.set_string("CFBundleName", "OverrideApp");
        override_plist.set_string("CFBundleDisplayName", "Override Display");
        
        base.merge(&override_plist);
        
        assert_eq!(base.get_string("CFBundleName"), Some("OverrideApp"));
        assert_eq!(base.get_string("CFBundleVersion"), Some("1.0.0"));
        assert_eq!(base.get_string("CFBundleDisplayName"), Some("Override Display"));
    }

    #[test]
    fn test_merge_recursive_arrays() {
        let mut base = PlistManager::new();
        base.add_to_array("Capabilities", "push");
        base.add_to_array("Capabilities", "location");
        
        let mut override_plist = PlistManager::new();
        override_plist.add_to_array("Capabilities", "location"); // duplicate
        override_plist.add_to_array("Capabilities", "camera");
        
        base.merge_recursive(&override_plist);
        
        let arr = base.get_array("Capabilities").unwrap();
        assert_eq!(arr.len(), 3); // push, location, camera (no duplicate location)
    }
}
