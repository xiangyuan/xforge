//! PBX build phase objects

use xforge_core::{ObjectId, Handle, PBXObject};

/// Source files build phase
#[derive(Debug, Clone)]
pub struct PBXSourcesBuildPhase {
    id: ObjectId,
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
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
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXSourcesBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXSourcesBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
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
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXFrameworksBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXFrameworksBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
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
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
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
    
    pub fn add_file(&mut self, file: Handle<PBXBuildFile>) {
        self.files.push(file);
    }
}

impl PBXObject for PBXResourcesBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXResourcesBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}

impl Default for PBXResourcesBuildPhase {
    fn default() -> Self {
        Self::new()
    }
}

/// Rez build phase (legacy Carbon resources)
#[derive(Debug, Clone)]
pub struct PBXRezBuildPhase {
    id: ObjectId,
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
}

impl PBXRezBuildPhase {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
        }
    }
}

impl PBXObject for PBXRezBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXRezBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Shell script build phase
#[derive(Debug, Clone)]
pub struct PBXShellScriptBuildPhase {
    id: ObjectId,
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
    pub shell_path: String,
    pub shell_script: String,
    pub name: Option<String>,
    pub input_paths: Vec<String>,
    pub output_paths: Vec<String>,
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
}

impl PBXObject for PBXShellScriptBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXShellScriptBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}

/// Copy files build phase
#[derive(Debug, Clone)]
pub struct PBXCopyFilesBuildPhase {
    id: ObjectId,
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
    pub dst_path: String,
    pub dst_subfolder_spec: u32,
    pub name: Option<String>,
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

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}

/// Build file
#[derive(Debug, Clone)]
pub struct PBXBuildFile {
    id: ObjectId,
    pub file_ref: Handle<crate::PBXFileReference>,
    pub settings: Option<std::collections::HashMap<String, String>>,
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
}

impl PBXObject for PBXBuildFile {
    fn isa(&self) -> &'static str {
        "PBXBuildFile"
    }

    fn id(&self) -> &ObjectId {
        &self.id
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
    fn test_sources_build_phase() {
        let phase = PBXSourcesBuildPhase::new();
        assert_eq!(phase.isa(), "PBXSourcesBuildPhase");
        assert_eq!(phase.files.len(), 0);
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
    fn test_shell_script_phase() {
        let phase = PBXShellScriptBuildPhase::new("echo hello")
            .with_name("Run Script");
        assert_eq!(phase.isa(), "PBXShellScriptBuildPhase");
        assert_eq!(phase.shell_script, "echo hello");
        assert_eq!(phase.name, Some("Run Script".to_string()));
    }

    #[test]
    fn test_copy_files_phase() {
        let phase = PBXCopyFilesBuildPhase::new("", 16)
            .with_name("Embed Frameworks");
        assert_eq!(phase.isa(), "PBXCopyFilesBuildPhase");
        assert_eq!(phase.dst_subfolder_spec, 16);
    }
    
    #[test]
    fn test_headers_phase() {
        let phase = PBXHeadersBuildPhase::new();
        assert_eq!(phase.isa(), "PBXHeadersBuildPhase");
        assert_eq!(phase.files.len(), 0);
    }
}

/// Headers build phase
#[derive(Debug, Clone)]
pub struct PBXHeadersBuildPhase {
    id: ObjectId,
    pub files: Vec<Handle<PBXBuildFile>>,
    pub build_action_mask: u32,
    pub run_only_for_deployment_postprocessing: bool,
}

impl PBXHeadersBuildPhase {
    pub fn new() -> Self {
        Self {
            id: ObjectId::generate(),
            files: Vec::new(),
            build_action_mask: 2147483647,
            run_only_for_deployment_postprocessing: false,
        }
    }
}

impl Default for PBXHeadersBuildPhase {
    fn default() -> Self {
        Self::new()
    }
}

impl PBXObject for PBXHeadersBuildPhase {
    fn isa(&self) -> &'static str {
        "PBXHeadersBuildPhase"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
