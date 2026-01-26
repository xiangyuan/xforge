//! PBXBuildRule - custom build rule definition

use xforge_core::{ObjectId, PBXObject};

/// Represents a custom build rule for transforming files.
#[derive(Debug, Clone)]
pub struct PBXBuildRule {
    id: ObjectId,
    pub compiler_spec: String,
    pub file_patterns: Option<String>,
    pub file_type: String,
    pub is_editable: bool,
    pub name: Option<String>,
    pub dependency_file: Option<String>,
    pub output_files: Vec<String>,
    pub input_files: Option<Vec<String>>,
    pub output_files_compiler_flags: Option<Vec<String>>,
    pub script: Option<String>,
    pub run_once_per_architecture: Option<bool>,
}

impl PBXBuildRule {
    pub fn new(compiler_spec: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            compiler_spec: compiler_spec.into(),
            file_patterns: None,
            file_type: file_type.into(),
            is_editable: true,
            name: None,
            dependency_file: None,
            output_files: Vec::new(),
            input_files: None,
            output_files_compiler_flags: None,
            script: None,
            run_once_per_architecture: None,
        }
    }
}

impl PBXObject for PBXBuildRule {
    fn isa(&self) -> &'static str {
        "PBXBuildRule"
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
