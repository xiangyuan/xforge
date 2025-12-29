//! Project builder for fluent API

use xforge_model::{Project, ProjectMetadata};
use std::path::PathBuf;

/// Builder for creating projects with fluent API
pub struct ProjectBuilder {
    name: String,
    organization: Option<String>,
    development_region: String,
    path: Option<PathBuf>,
}

impl ProjectBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            organization: None,
            development_region: "en".to_string(),
            path: None,
        }
    }
    
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    
    pub fn organization(mut self, org: impl Into<String>) -> Self {
        self.organization = Some(org.into());
        self
    }
    
    pub fn development_region(mut self, region: impl Into<String>) -> Self {
        self.development_region = region.into();
        self
    }
    
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = Some(path.into());
        self
    }
    
    pub fn build(self) -> Project {
        let mut project = Project::new(self.name);
        
        if let Some(org) = self.organization {
            project.metadata_mut().organization = Some(org);
        }
        
        project.metadata_mut().development_region = self.development_region;
        
        project
    }
}

impl Default for ProjectBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let project = ProjectBuilder::new()
            .name("MyApp")
            .build();
        
        assert_eq!(project.name(), "MyApp");
    }
    
    #[test]
    fn test_builder_with_org() {
        let project = ProjectBuilder::new()
            .name("MyApp")
            .organization("MyCompany")
            .build();
        
        assert_eq!(project.metadata().organization.as_deref(), Some("MyCompany"));
    }
}
