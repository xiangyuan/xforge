use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::{BuildAction, TestAction, LaunchAction, ProfileAction, AnalyzeAction, ArchiveAction, Result};

/// Represents an Xcode scheme
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "Scheme")]
pub struct Scheme {
    /// Last upgrade version
    #[serde(rename = "@LastUpgradeVersion")]
    pub last_upgrade_version: String,
    
    /// Scheme version
    #[serde(rename = "@version")]
    pub version: String,
    
    /// Build action
    #[serde(rename = "BuildAction")]
    pub build_action: BuildAction,
    
    /// Test action
    #[serde(rename = "TestAction")]
    pub test_action: TestAction,
    
    /// Launch action
    #[serde(rename = "LaunchAction")]
    pub launch_action: LaunchAction,
    
    /// Profile action
    #[serde(rename = "ProfileAction")]
    pub profile_action: ProfileAction,
    
    /// Analyze action
    #[serde(rename = "AnalyzeAction")]
    pub analyze_action: AnalyzeAction,
    
    /// Archive action
    #[serde(rename = "ArchiveAction")]
    pub archive_action: ArchiveAction,
}

impl Scheme {
    /// Creates a new scheme with default configuration
    pub fn new(
        _name: impl Into<String>,
        debug_config: impl Into<String>,
        release_config: impl Into<String>,
    ) -> Self {
        let debug = debug_config.into();
        let release = release_config.into();
        
        Self {
            last_upgrade_version: "1600".to_string(),
            version: "1.7".to_string(),
            build_action: BuildAction::new(),
            test_action: TestAction::new(&debug),
            launch_action: LaunchAction::new(&debug),
            profile_action: ProfileAction::new(&release),
            analyze_action: AnalyzeAction::new(&debug),
            archive_action: ArchiveAction::new(&release),
        }
    }
    
    /// Loads a scheme from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to read scheme file: {}", e))?;
        
        Self::from_xml(&content)
    }
    
    /// Saves the scheme to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let xml = self.to_xml()?;
        fs::write(path.as_ref(), xml)
            .map_err(|e| anyhow::anyhow!("Failed to write scheme file: {}", e))?;
        
        Ok(())
    }
    
    /// Parses scheme from XML string
    pub fn from_xml(xml: &str) -> Result<Self> {
        quick_xml::de::from_str(xml)
            .map_err(|e| anyhow::anyhow!("Failed to parse scheme XML: {}", e))
    }
    
    /// Converts scheme to XML string
    pub fn to_xml(&self) -> Result<String> {
        let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        let serialized = quick_xml::se::to_string(self)
            .map_err(|e| anyhow::anyhow!("Failed to serialize scheme: {}", e))?;
        xml.push_str(&serialized);
        Ok(xml)
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Self::new("Default", "Debug", "Release")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BuildableReference;

    #[test]
    fn test_scheme_creation() {
        let scheme = Scheme::new("MyScheme", "Debug", "Release");
        assert_eq!(scheme.version, "1.7");
        assert_eq!(scheme.test_action.build_configuration, "Debug");
        assert_eq!(scheme.archive_action.build_configuration, "Release");
    }

    #[test]
    fn test_scheme_xml_serialization() {
        let mut scheme = Scheme::new("MyScheme", "Debug", "Release");
        
        let reference = BuildableReference::new(
            "MyApp",
            "ABC123",
            "container:MyApp.xcodeproj"
        );
        scheme.build_action.add_entry(reference.clone());
        scheme.launch_action.set_buildable_reference(reference);
        
        let xml = scheme.to_xml().unwrap();
        assert!(xml.contains("<?xml version"));
        assert!(xml.contains("Scheme"));
        assert!(xml.contains("BuildAction"));
        assert!(xml.contains("MyApp"));
    }

    #[test]
    fn test_scheme_roundtrip() {
        let mut scheme = Scheme::new("TestScheme", "Debug", "Release");
        
        let reference = BuildableReference::new(
            "TestApp",
            "XYZ789",
            "container:TestApp.xcodeproj"
        );
        scheme.build_action.add_entry(reference);
        
        let xml = scheme.to_xml().unwrap();
        let parsed = Scheme::from_xml(&xml).unwrap();
        
        assert_eq!(scheme.version, parsed.version);
        assert_eq!(scheme.build_action.build_action_entries.entries.len(), 
                   parsed.build_action.build_action_entries.entries.len());
    }

    #[test]
    fn test_default_scheme() {
        let scheme = Scheme::default();
        assert_eq!(scheme.test_action.build_configuration, "Debug");
        assert_eq!(scheme.launch_action.build_configuration, "Debug");
        assert_eq!(scheme.profile_action.build_configuration, "Release");
        assert_eq!(scheme.archive_action.build_configuration, "Release");
    }
}
