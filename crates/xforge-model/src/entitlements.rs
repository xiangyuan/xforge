//! Entitlements management for iOS/macOS apps
//! 
//! Handles .entitlements files which define app capabilities and permissions

use std::path::{Path, PathBuf};
use std::fs;

/// Entitlements Manager - High-level API for managing app entitlements
#[derive(Debug, Clone)]
pub struct EntitlementsManager {
    file_path: PathBuf,
    data: plist::Dictionary,
}

impl EntitlementsManager {
    /// Create a new empty entitlements file
    pub fn new() -> Self {
        Self {
            file_path: PathBuf::new(),
            data: plist::Dictionary::new(),
        }
    }

    /// Load an entitlements file from disk
    pub fn load(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        let file = fs::File::open(path)
            .map_err(|e| format!("Failed to open entitlements file: {}", e))?;
        
        let plist_value: plist::Value = plist::from_reader(file)
            .map_err(|e| format!("Failed to parse entitlements: {}", e))?;
        
        let data = match plist_value {
            plist::Value::Dictionary(dict) => dict,
            _ => return Err("Entitlements root must be a dictionary".to_string()),
        };
        
        Ok(Self {
            file_path: path.to_path_buf(),
            data,
        })
    }

    /// Save the entitlements to disk in XML format
    pub fn save(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }
        
        let plist_value = plist::Value::Dictionary(self.data.clone());
        let file = fs::File::create(path)
            .map_err(|e| format!("Failed to create entitlements file: {}", e))?;
        
        plist::to_writer_xml(file, &plist_value)
            .map_err(|e| format!("Failed to write entitlements: {}", e))?;
        
        Ok(())
    }

    // ===== Push Notifications =====

    /// Enable Push Notifications (APS Environment)
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::EntitlementsManager;
    /// # let mut entitlements = EntitlementsManager::new();
    /// entitlements.enable_push_notifications("development");
    /// // or "production" for production builds
    /// ```
    pub fn enable_push_notifications(&mut self, environment: &str) {
        self.data.insert(
            "aps-environment".to_string(),
            plist::Value::String(environment.to_string()),
        );
    }

    // ===== App Groups =====

    /// Add an app group (for sharing data between apps/extensions)
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::EntitlementsManager;
    /// # let mut entitlements = EntitlementsManager::new();
    /// entitlements.add_app_group("group.com.example.myapp");
    /// ```
    pub fn add_app_group(&mut self, group_id: impl Into<String>) {
        let key = "com.apple.security.application-groups".to_string();
        let group_id = group_id.into();
        
        match self.data.get_mut(&key) {
            Some(plist::Value::Array(arr)) => {
                let group_value = plist::Value::String(group_id.clone());
                if !arr.contains(&group_value) {
                    arr.push(group_value);
                }
            }
            _ => {
                self.data.insert(
                    key,
                    plist::Value::Array(vec![plist::Value::String(group_id)]),
                );
            }
        }
    }

    /// Get all app groups
    pub fn get_app_groups(&self) -> Vec<String> {
        let key = "com.apple.security.application-groups";
        self.data
            .get(key)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_string())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default()
    }

    // ===== iCloud =====

    /// Enable iCloud with key-value storage
    pub fn enable_icloud_key_value(&mut self) {
        self.data.insert(
            "com.apple.developer.ubiquity-kvstore-identifier".to_string(),
            plist::Value::String("$(TeamIdentifierPrefix)$(CFBundleIdentifier)".to_string()),
        );
    }

    /// Enable iCloud Documents (CloudKit container)
    pub fn enable_icloud_documents(&mut self, container_id: impl Into<String>) {
        let key = "com.apple.developer.ubiquity-container-identifiers".to_string();
        let container = plist::Value::String(container_id.into());
        
        self.data.insert(key, plist::Value::Array(vec![container]));
    }

    /// Add CloudKit container
    pub fn add_cloudkit_container(&mut self, container_id: impl Into<String>) {
        let key = "com.apple.developer.icloud-container-identifiers".to_string();
        let container_id = container_id.into();
        
        match self.data.get_mut(&key) {
            Some(plist::Value::Array(arr)) => {
                let container_value = plist::Value::String(container_id.clone());
                if !arr.contains(&container_value) {
                    arr.push(container_value);
                }
            }
            _ => {
                self.data.insert(
                    key,
                    plist::Value::Array(vec![plist::Value::String(container_id)]),
                );
            }
        }
    }

    // ===== Associated Domains =====

    /// Add an associated domain (for Universal Links, Handoff, etc.)
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::EntitlementsManager;
    /// # let mut entitlements = EntitlementsManager::new();
    /// entitlements.add_associated_domain("applinks:example.com");
    /// entitlements.add_associated_domain("webcredentials:example.com");
    /// ```
    pub fn add_associated_domain(&mut self, domain: impl Into<String>) {
        let key = "com.apple.developer.associated-domains".to_string();
        let domain = domain.into();
        
        match self.data.get_mut(&key) {
            Some(plist::Value::Array(arr)) => {
                let domain_value = plist::Value::String(domain.clone());
                if !arr.contains(&domain_value) {
                    arr.push(domain_value);
                }
            }
            _ => {
                self.data.insert(
                    key,
                    plist::Value::Array(vec![plist::Value::String(domain)]),
                );
            }
        }
    }

    // ===== In-App Purchase =====

    /// Enable In-App Purchase capability
    pub fn enable_in_app_purchase(&mut self) {
        // In-App Purchase doesn't require specific entitlements in most cases
        // but we add it for documentation purposes
        self.data.insert(
            "com.apple.developer.in-app-payments".to_string(),
            plist::Value::Array(vec![]),
        );
    }

    // ===== HealthKit =====

    /// Enable HealthKit
    pub fn enable_healthkit(&mut self) {
        self.data.insert(
            "com.apple.developer.healthkit".to_string(),
            plist::Value::Boolean(true),
        );
        self.data.insert(
            "com.apple.developer.healthkit.access".to_string(),
            plist::Value::Array(vec![]),
        );
    }

    // ===== HomeKit =====

    /// Enable HomeKit
    pub fn enable_homekit(&mut self) {
        self.data.insert(
            "com.apple.developer.homekit".to_string(),
            plist::Value::Boolean(true),
        );
    }

    // ===== Siri =====

    /// Enable Siri capability
    pub fn enable_siri(&mut self) {
        self.data.insert(
            "com.apple.developer.siri".to_string(),
            plist::Value::Boolean(true),
        );
    }

    // ===== Wallet (Apple Pay) =====

    /// Enable Apple Pay with merchant IDs
    pub fn enable_apple_pay(&mut self, merchant_ids: Vec<String>) {
        let merchants: Vec<plist::Value> = merchant_ids
            .into_iter()
            .map(plist::Value::String)
            .collect();
        
        self.data.insert(
            "com.apple.developer.in-app-payments".to_string(),
            plist::Value::Array(merchants),
        );
    }

    // ===== Keychain Sharing =====

    /// Add keychain access group
    pub fn add_keychain_group(&mut self, group_id: impl Into<String>) {
        let key = "keychain-access-groups".to_string();
        let group_id = group_id.into();
        
        match self.data.get_mut(&key) {
            Some(plist::Value::Array(arr)) => {
                let group_value = plist::Value::String(group_id.clone());
                if !arr.contains(&group_value) {
                    arr.push(group_value);
                }
            }
            _ => {
                self.data.insert(
                    key,
                    plist::Value::Array(vec![plist::Value::String(group_id)]),
                );
            }
        }
    }

    // ===== Network Extensions =====

    /// Enable Network Extensions
    pub fn enable_network_extensions(&mut self, extension_types: Vec<&str>) {
        let extensions: Vec<plist::Value> = extension_types
            .into_iter()
            .map(|s| plist::Value::String(s.to_string()))
            .collect();
        
        self.data.insert(
            "com.apple.developer.networking.networkextension".to_string(),
            plist::Value::Array(extensions),
        );
    }

    // ===== Personal VPN =====

    /// Enable Personal VPN
    pub fn enable_personal_vpn(&mut self) {
        self.data.insert(
            "com.apple.developer.networking.vpn.api".to_string(),
            plist::Value::Array(vec![plist::Value::String("allow-vpn".to_string())]),
        );
    }

    // ===== Wireless Accessory Configuration =====

    /// Enable Wireless Accessory Configuration
    pub fn enable_wireless_accessory(&mut self) {
        self.data.insert(
            "com.apple.external-accessory.wireless-configuration".to_string(),
            plist::Value::Boolean(true),
        );
    }

    // ===== Data Protection =====

    /// Set data protection level
    /// 
    /// Options: "NSFileProtectionComplete", "NSFileProtectionCompleteUnlessOpen", etc.
    pub fn set_data_protection(&mut self, protection_class: impl Into<String>) {
        self.data.insert(
            "com.apple.developer.default-data-protection".to_string(),
            plist::Value::String(protection_class.into()),
        );
    }

    // ===== Game Center =====

    /// Enable Game Center
    pub fn enable_game_center(&mut self) {
        self.data.insert(
            "com.apple.developer.game-center".to_string(),
            plist::Value::Boolean(true),
        );
    }

    // ===== Generic setters/getters =====

    /// Set a boolean entitlement
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.data.insert(key.into(), plist::Value::Boolean(value));
    }

    /// Set a string entitlement
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.data.insert(key.into(), plist::Value::String(value.into()));
    }

    /// Set an array entitlement
    pub fn set_array(&mut self, key: impl Into<String>, values: Vec<String>) {
        let array: Vec<plist::Value> = values
            .into_iter()
            .map(plist::Value::String)
            .collect();
        self.data.insert(key.into(), plist::Value::Array(array));
    }

    /// Get a boolean entitlement
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.data.get(key).and_then(|v| v.as_boolean())
    }

    /// Get a string entitlement
    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.data.get(key).and_then(|v| v.as_string())
    }

    /// Get an array entitlement
    pub fn get_array(&self, key: &str) -> Option<Vec<String>> {
        self.data.get(key).and_then(|v| v.as_array()).map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_string())
                .map(|s| s.to_string())
                .collect()
        })
    }

    /// Check if an entitlement key exists
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Remove an entitlement
    pub fn remove(&mut self, key: &str) -> Option<plist::Value> {
        self.data.remove(key)
    }

    /// Get all entitlement keys
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.data.keys()
    }

    /// Get the file path
    pub fn path(&self) -> &Path {
        &self.file_path
    }
}

impl Default for EntitlementsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_entitlements() {
        let entitlements = EntitlementsManager::new();
        assert_eq!(entitlements.data.len(), 0);
    }

    #[test]
    fn test_push_notifications() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.enable_push_notifications("development");
        
        assert_eq!(entitlements.get_string("aps-environment"), Some("development"));
    }

    #[test]
    fn test_app_groups() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.add_app_group("group.com.example.app1");
        entitlements.add_app_group("group.com.example.app2");
        entitlements.add_app_group("group.com.example.app1"); // duplicate
        
        let groups = entitlements.get_app_groups();
        assert_eq!(groups.len(), 2);
        assert!(groups.contains(&"group.com.example.app1".to_string()));
        assert!(groups.contains(&"group.com.example.app2".to_string()));
    }

    #[test]
    fn test_associated_domains() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.add_associated_domain("applinks:example.com");
        entitlements.add_associated_domain("webcredentials:example.com");
        
        let domains = entitlements.get_array("com.apple.developer.associated-domains").unwrap();
        assert_eq!(domains.len(), 2);
    }

    #[test]
    fn test_healthkit() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.enable_healthkit();
        
        assert_eq!(entitlements.get_bool("com.apple.developer.healthkit"), Some(true));
    }

    #[test]
    fn test_keychain_groups() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.add_keychain_group("$(AppIdentifierPrefix)com.example.app");
        
        let groups = entitlements.get_array("keychain-access-groups").unwrap();
        assert_eq!(groups.len(), 1);
    }

    #[test]
    fn test_generic_setters() {
        let mut entitlements = EntitlementsManager::new();
        entitlements.set_bool("custom-bool", true);
        entitlements.set_string("custom-string", "value");
        entitlements.set_array("custom-array", vec!["item1".to_string(), "item2".to_string()]);
        
        assert_eq!(entitlements.get_bool("custom-bool"), Some(true));
        assert_eq!(entitlements.get_string("custom-string"), Some("value"));
        assert_eq!(entitlements.get_array("custom-array").unwrap().len(), 2);
    }
}
