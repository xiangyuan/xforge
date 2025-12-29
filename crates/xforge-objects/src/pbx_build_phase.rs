//! PBX build phase objects

use xforge_core::{ObjectId, Handle, PBXObject};

/// Source files build phase
#[derive(Debug, Clone)]
pub struct PBXSourcesBuildPhase {
    id: ObjectId,
    files: Vec<Handle<PBXBuildFile>>,
    build_action_mask: u32,
    run_only_for_deployment_postprocessing: bool,
}

impl PBXSourcesBuildPhase {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
        }
    }
    
    pub fn files(&self) -> &[Handle<PBXBuildFile>] {
        &self.files
    }
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXSourcesBuildPhase {
    
    fn isa(&self) -> &'static str {
        "PBXSourcesBuildPhase"
    }
}

impl Default for PBXSourcesBuildPhase {
    fn default() -> Self {
        Self::new()
    }
}

/// Frameworks and libraries build phase
#[derive(Debug, Clone)]
pub struct PBXFrameworksBuildPhase {
    id: ObjectId,
    files: Vec<Handle<PBXBuildFile>>,
    build_action_mask: u32,
    run_only_for_deployment_postprocessing: bool,
}

impl PBXFrameworksBuildPhase {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
        }
    }
    
    pub fn files(&self) -> &[Handle<PBXBuildFile>] {
        &self.files
    }
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXFrameworksBuildPhase {
    
    fn isa(&self) -> &'static str {
        "PBXFrameworksBuildPhase"
    }
}

impl Default for PBXFrameworksBuildPhase {
    fn default() -> Self {
        Self::new()
    }
}

/// Resources build phase
#[derive(Debug, Clone)]
pub struct PBXResourcesBuildPhase {
    id: ObjectId,
    files: Vec<Handle<PBXBuildFile>>,
    build_action_mask: u32,
    run_only_for_deployment_postprocessing: bool,
}

impl PBXResourcesBuildPhase {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
        }
    }
    
    pub fn files(&self) -> &[Handle<PBXBuildFile>] {
        &self.files
    }
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXResourcesBuildPhase {
    
    fn isa(&self) -> &'static str {
        "PBXResourcesBuildPhase"
    }
}

impl Default for PBXResourcesBuildPhase {
    fn default() -> Self {
        Self::new()
    }
}

/// Shell script build phase
#[derive(Debug, Clone)]
pub struct PBXShellScriptBuildPhase {
    id: ObjectId,
    files: Vec<Handle<PBXBuildFile>>,
    build_action_mask: u32,
    run_only_for_deployment_postprocessing: bool,
    shell_path: String,
    shell_script: String,
    name: Option<String>,
    input_paths: Vec<String>,
    output_paths: Vec<String>,
    show_env_vars_in_log: bool,
}

impl PBXShellScriptBuildPhase {
    pub fn new(script: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
            shell_path: "/bin/sh".to_string(),
            shell_script: script.into(),
            name: None,
            input_paths: Vec::new(),
            output_paths: Vec::new(),
            show_env_vars_in_log: true,
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn add_input_path(&mut self, path: impl Into<String>) {
        self.input_paths.push(path.into());
    }
    
    pub fn add_output_path(&mut self, path: impl Into<String>) {
        self.output_paths.push(path.into());
    }
    
    pub fn shell_script(&self) -> &str {
        &self.shell_script
    }
    
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}

impl PBXObject for PBXShellScriptBuildPhase {
    
    fn isa(&self) -> &'static str {
        "PBXShellScriptBuildPhase"
    }
}

/// Copy files build phase
#[derive(Debug, Clone)]
pub struct PBXCopyFilesBuildPhase {
    id: ObjectId,
    files: Vec<Handle<PBXBuildFile>>,
    build_action_mask: u32,
    run_only_for_deployment_postprocessing: bool,
    dst_path: String,
    dst_subfolder_spec: u32,
    name: Option<String>,
}

impl PBXCopyFilesBuildPhase {
    pub fn new(dst_path: impl Into<String>, dst_subfolder_spec: u32) -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
            dst_path: dst_path.into(),
            dst_subfolder_spec,
            name: None,
        }
    }
    
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXCopyFilesBuildPhase {
    
    fn isa(&self) -> &'static str {
        "PBXCopyFilesBuildPhase"
    }
}

/// Build file reference
#[derive(Debug, Clone)]
pub struct PBXBuildFile {
    id: ObjectId,
    file_ref: Handle<crate::PBXFileReference>,
    settings: Option<std::collections::HashMap<String, String>>,
}

impl PBXBuildFile {
    pub fn new(file_ref: Handle<crate::PBXFileReference>) -> Self {
        Self {
            id: ObjectId::generate(),
            file_ref,
            settings: None,
        }
    }
    
    pub fn with_settings(mut self, settings: std::collections::HashMap<String, String>) -> Self {
        self.settings = Some(settings);
        self
    }
    
    pub fn file_ref(&self) -> &Handle<crate::PBXFileReference> {
        &self.file_ref
    }
    
    pub fn settings(&self) -> Option<&std::collections::HashMap<String, String>> {
        self.settings.as_ref()
    }
}

impl PBXObject for PBXBuildFile {
    
    fn isa(&self) -> &'static str {
        "PBXBuildFile"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sources_build_phase() {
        let phase = PBXSourcesBuildPhase::new();
        assert_eq!(phase.isa(), "PBXSourcesBuildPhase");
        assert_eq!(phase.files().len(), 0);
    }

    #[test]
    fn test_frameworks_build_phase() {
        let phase = PBXFrameworksBuildPhase::new();
        assert_eq!(phase.isa(), "PBXFrameworksBuildPhase");
    }

    #[test]
    fn test_resources_build_phase() {
        let phase = PBXResourcesBuildPhase::new();
        assert_eq!(phase.isa(), "PBXResourcesBuildPhase");
    }

    #[test]
    fn test_shell_script_build_phase() {
        let phase = PBXShellScriptBuildPhase::new("echo 'Hello'")
            .with_name("Run Script");
        
        assert_eq!(phase.isa(), "PBXShellScriptBuildPhase");
        assert_eq!(phase.shell_script(), "echo 'Hello'");
        assert_eq!(phase.name(), Some("Run Script"));
    }

    #[test]
    fn test_copy_files_build_phase() {
        let phase = PBXCopyFilesBuildPhase::new("", 16)
            .with_name("Embed Frameworks");
        
        assert_eq!(phase.isa(), "PBXCopyFilesBuildPhase");
    }
}
