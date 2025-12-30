use serde::{Deserialize, Serialize};

/// Represents a buildable reference to a target
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildableReference {
    /// Type of blueprint (usually "container")
    #[serde(rename = "@BuildableIdentifier")]
    pub buildable_identifier: String,
    
    /// Blueprint name (usually target name)
    #[serde(rename = "@BlueprintName")]
    pub blueprint_name: String,
    
    /// Building for (usually empty string)
    #[serde(rename = "@BuildingFor", skip_serializing_if = "Option::is_none")]
    pub building_for: Option<String>,
    
    /// Blueprint identifier (target UUID)
    #[serde(rename = "@BlueprintIdentifier")]
    pub blueprint_identifier: String,
    
    /// Path to the project file
    #[serde(rename = "@ReferencedContainer")]
    pub referenced_container: String,
}

impl BuildableReference {
    /// Creates a new buildable reference
    pub fn new(
        blueprint_name: impl Into<String>,
        blueprint_identifier: impl Into<String>,
        referenced_container: impl Into<String>,
    ) -> Self {
        Self {
            buildable_identifier: "primary".to_string(),
            blueprint_name: blueprint_name.into(),
            building_for: None,
            blueprint_identifier: blueprint_identifier.into(),
            referenced_container: referenced_container.into(),
        }
    }
}

/// Blueprint identifier helper
pub struct BlueprintIdentifier;

impl BlueprintIdentifier {
    /// Creates a primary identifier
    pub fn primary() -> String {
        "primary".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buildable_reference() {
        let ref_ = BuildableReference::new(
            "MyApp",
            "ABC123",
            "container:MyApp.xcodeproj"
        );
        
        assert_eq!(ref_.blueprint_name, "MyApp");
        assert_eq!(ref_.blueprint_identifier, "ABC123");
        assert_eq!(ref_.buildable_identifier, "primary");
    }
}
