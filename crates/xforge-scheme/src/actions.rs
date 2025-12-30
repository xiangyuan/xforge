use serde::{Deserialize, Serialize};
use crate::BuildableReference;

/// Build action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildAction {
    /// Parallelize build
    #[serde(rename = "@parallelizeBuildables")]
    pub parallelize_buildables: String,
    
    /// Build implicit dependencies
    #[serde(rename = "@buildImplicitDependencies")]
    pub build_implicit_dependencies: String,
    
    /// Buildable references
    #[serde(rename = "BuildActionEntries")]
    pub build_action_entries: BuildActionEntries,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildActionEntries {
    #[serde(rename = "BuildActionEntry", default)]
    pub entries: Vec<BuildActionEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildActionEntry {
    #[serde(rename = "@buildForTesting")]
    pub build_for_testing: String,
    
    #[serde(rename = "@buildForRunning")]
    pub build_for_running: String,
    
    #[serde(rename = "@buildForProfiling")]
    pub build_for_profiling: String,
    
    #[serde(rename = "@buildForArchiving")]
    pub build_for_archiving: String,
    
    #[serde(rename = "@buildForAnalyzing")]
    pub build_for_analyzing: String,
    
    #[serde(rename = "BuildableReference")]
    pub buildable_reference: BuildableReference,
}

impl BuildAction {
    pub fn new() -> Self {
        Self {
            parallelize_buildables: "YES".to_string(),
            build_implicit_dependencies: "YES".to_string(),
            build_action_entries: BuildActionEntries { entries: Vec::new() },
        }
    }
    
    pub fn add_entry(&mut self, reference: BuildableReference) {
        self.build_action_entries.entries.push(BuildActionEntry {
            build_for_testing: "YES".to_string(),
            build_for_running: "YES".to_string(),
            build_for_profiling: "YES".to_string(),
            build_for_archiving: "YES".to_string(),
            build_for_analyzing: "YES".to_string(),
            buildable_reference: reference,
        });
    }
}

impl Default for BuildAction {
    fn default() -> Self {
        Self::new()
    }
}

/// Test action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestAction {
    #[serde(rename = "@buildConfiguration")]
    pub build_configuration: String,
    
    #[serde(rename = "@selectedDebuggerIdentifier")]
    pub selected_debugger_identifier: String,
    
    #[serde(rename = "@selectedLauncherIdentifier")]
    pub selected_launcher_identifier: String,
    
    #[serde(rename = "@shouldUseLaunchSchemeArgsEnv")]
    pub should_use_launch_scheme_args_env: String,
    
    #[serde(rename = "Testables", skip_serializing_if = "Option::is_none")]
    pub testables: Option<Testables>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Testables {
    #[serde(rename = "TestableReference", default)]
    pub references: Vec<TestableReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestableReference {
    #[serde(rename = "@skipped")]
    pub skipped: String,
    
    #[serde(rename = "BuildableReference")]
    pub buildable_reference: BuildableReference,
}

impl TestAction {
    pub fn new(build_configuration: impl Into<String>) -> Self {
        Self {
            build_configuration: build_configuration.into(),
            selected_debugger_identifier: "Xcode.DebuggerFoundation.Debugger.LLDB".to_string(),
            selected_launcher_identifier: "Xcode.DebuggerFoundation.Launcher.LLDB".to_string(),
            should_use_launch_scheme_args_env: "YES".to_string(),
            testables: None,
        }
    }
}

/// Launch action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchAction {
    #[serde(rename = "@buildConfiguration")]
    pub build_configuration: String,
    
    #[serde(rename = "@selectedDebuggerIdentifier")]
    pub selected_debugger_identifier: String,
    
    #[serde(rename = "@selectedLauncherIdentifier")]
    pub selected_launcher_identifier: String,
    
    #[serde(rename = "@launchStyle")]
    pub launch_style: String,
    
    #[serde(rename = "@useCustomWorkingDirectory")]
    pub use_custom_working_directory: String,
    
    #[serde(rename = "@ignoresPersistentStateOnLaunch")]
    pub ignores_persistent_state_on_launch: String,
    
    #[serde(rename = "@debugDocumentVersioning")]
    pub debug_document_versioning: String,
    
    #[serde(rename = "@debugServiceExtension")]
    pub debug_service_extension: String,
    
    #[serde(rename = "@allowLocationSimulation")]
    pub allow_location_simulation: String,
    
    #[serde(rename = "BuildableProductRunnable", skip_serializing_if = "Option::is_none")]
    pub buildable_product_runnable: Option<BuildableProductRunnable>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildableProductRunnable {
    #[serde(rename = "@runnableDebuggingMode")]
    pub runnable_debugging_mode: String,
    
    #[serde(rename = "BuildableReference")]
    pub buildable_reference: BuildableReference,
}

impl LaunchAction {
    pub fn new(build_configuration: impl Into<String>) -> Self {
        Self {
            build_configuration: build_configuration.into(),
            selected_debugger_identifier: "Xcode.DebuggerFoundation.Debugger.LLDB".to_string(),
            selected_launcher_identifier: "Xcode.DebuggerFoundation.Launcher.LLDB".to_string(),
            launch_style: "0".to_string(),
            use_custom_working_directory: "NO".to_string(),
            ignores_persistent_state_on_launch: "NO".to_string(),
            debug_document_versioning: "YES".to_string(),
            debug_service_extension: "internal".to_string(),
            allow_location_simulation: "YES".to_string(),
            buildable_product_runnable: None,
        }
    }
    
    pub fn set_buildable_reference(&mut self, reference: BuildableReference) {
        self.buildable_product_runnable = Some(BuildableProductRunnable {
            runnable_debugging_mode: "0".to_string(),
            buildable_reference: reference,
        });
    }
}

/// Profile action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileAction {
    #[serde(rename = "@buildConfiguration")]
    pub build_configuration: String,
    
    #[serde(rename = "@shouldUseLaunchSchemeArgsEnv")]
    pub should_use_launch_scheme_args_env: String,
    
    #[serde(rename = "@savedToolIdentifier")]
    pub saved_tool_identifier: String,
    
    #[serde(rename = "@useCustomWorkingDirectory")]
    pub use_custom_working_directory: String,
    
    #[serde(rename = "@debugDocumentVersioning")]
    pub debug_document_versioning: String,
    
    #[serde(rename = "BuildableProductRunnable", skip_serializing_if = "Option::is_none")]
    pub buildable_product_runnable: Option<BuildableProductRunnable>,
}

impl ProfileAction {
    pub fn new(build_configuration: impl Into<String>) -> Self {
        Self {
            build_configuration: build_configuration.into(),
            should_use_launch_scheme_args_env: "YES".to_string(),
            saved_tool_identifier: String::new(),
            use_custom_working_directory: "NO".to_string(),
            debug_document_versioning: "YES".to_string(),
            buildable_product_runnable: None,
        }
    }
}

/// Analyze action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalyzeAction {
    #[serde(rename = "@buildConfiguration")]
    pub build_configuration: String,
}

impl AnalyzeAction {
    pub fn new(build_configuration: impl Into<String>) -> Self {
        Self {
            build_configuration: build_configuration.into(),
        }
    }
}

/// Archive action configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveAction {
    #[serde(rename = "@buildConfiguration")]
    pub build_configuration: String,
    
    #[serde(rename = "@revealArchiveInOrganizer")]
    pub reveal_archive_in_organizer: String,
}

impl ArchiveAction {
    pub fn new(build_configuration: impl Into<String>) -> Self {
        Self {
            build_configuration: build_configuration.into(),
            reveal_archive_in_organizer: "YES".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_action() {
        let mut action = BuildAction::new();
        assert!(action.build_action_entries.entries.is_empty());
        
        let reference = BuildableReference::new("MyApp", "ABC", "container:MyApp.xcodeproj");
        action.add_entry(reference);
        
        assert_eq!(action.build_action_entries.entries.len(), 1);
    }

    #[test]
    fn test_test_action() {
        let action = TestAction::new("Debug");
        assert_eq!(action.build_configuration, "Debug");
    }

    #[test]
    fn test_launch_action() {
        let mut action = LaunchAction::new("Debug");
        assert_eq!(action.build_configuration, "Debug");
        
        let reference = BuildableReference::new("MyApp", "ABC", "container:MyApp.xcodeproj");
        action.set_buildable_reference(reference);
        
        assert!(action.buildable_product_runnable.is_some());
    }
}
