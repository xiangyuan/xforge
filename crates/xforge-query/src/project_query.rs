//! Project-level query API

use xforge_core::{Handle, ObjectId, Registry};
use xforge_model::Project;
use xforge_objects::{PBXProject, PBXNativeTarget, PBXGroup, PBXFileReference};
use crate::errors::{QueryError, QueryResult};

/// Query API for Project operations
pub struct ProjectQuery<'a> {
    project: &'a Project,
}

impl<'a> ProjectQuery<'a> {
    /// Create a new ProjectQuery
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    /// Get the registry
    pub fn registry(&self) -> &Registry {
        self.project.registry()
    }

    /// Get the root PBXProject object
    pub fn root_project(&self) -> QueryResult<&PBXProject> {
        let root_id = self.project.root_id();
        self.registry()
            .get::<PBXProject>(&root_id)
            .ok_or_else(|| QueryError::RegistryError("Root project not found".to_string()))
    }

    /// Find a target by name
    pub fn find_target(&self, name: &str) -> QueryResult<Handle<PBXNativeTarget>> {
        let root_proj = self.root_project()?;
        for target_handle in root_proj.targets() {
            if let Some(target) = self.registry().get::<PBXNativeTarget>(target_handle.id()) {
                if target.name() == name {
                    return Ok(*target_handle);
                }
            }
        }
        Err(QueryError::TargetNotFound(name.to_string()))
    }

    /// Find all targets matching a predicate
    pub fn find_targets<F>(&self, predicate: F) -> QueryResult<Vec<Handle<PBXNativeTarget>>>
    where
        F: Fn(&PBXNativeTarget) -> bool,
    {
        let root_proj = self.root_project()?;
        let mut results = Vec::new();
        for target_handle in root_proj.targets() {
            if let Some(target) = self.registry().get::<PBXNativeTarget>(target_handle.id()) {
                if predicate(target) {
                    results.push(*target_handle);
                }
            }
        }
        Ok(results)
    }

    /// Get the main group
    pub fn main_group(&self) -> QueryResult<Handle<PBXGroup>> {
        let root_proj = self.root_project()?;
        root_proj
            .main_group()
            .cloned()
            .ok_or_else(|| QueryError::GroupNotFound("main".to_string()))
    }

    /// Find a file by path (searches recursively)
    pub fn find_file(&self, path: &str) -> QueryResult<Handle<PBXFileReference>> {
        let main_group_handle = self.main_group()?;
        self.find_file_in_group(&main_group_handle, path)
    }

    /// Find a group by path
    pub fn find_group(&self, path: &str) -> QueryResult<Handle<PBXGroup>> {
        let main_group_handle = self.main_group()?;
        if path.is_empty() || path == "/" {
            return Ok(main_group_handle);
        }
        self.find_group_in_group(&main_group_handle, path)
    }

    /// List all targets
    pub fn targets(&self) -> QueryResult<Vec<Handle<PBXNativeTarget>>> {
        let root_proj = self.root_project()?;
        Ok(root_proj.targets().to_vec())
    }

    /// Count total targets
    pub fn target_count(&self) -> QueryResult<usize> {
        let root_proj = self.root_project()?;
        Ok(root_proj.targets().len())
    }

    // Private helper methods

    fn find_file_in_group(
        &self,
        group_handle: &Handle<PBXGroup>,
        path: &str,
    ) -> QueryResult<Handle<PBXFileReference>> {
        if let Some(group) = self.registry().get::<PBXGroup>(group_handle.id()) {
            for child_handle in group.children() {
                if let Some(file) = self.registry().get::<PBXFileReference>(child_handle.id()) {
                    if let Some(file_path) = file.path() {
                        if file_path == path {
                            return Ok(Handle::from_id(*child_handle.id()));
                        }
                    }
                }
                if self.registry().get::<PBXGroup>(child_handle.id()).is_some() {
                    if let Ok(result) = self.find_file_in_group(
                        &Handle::from_id(*child_handle.id()),
                        path,
                    ) {
                        return Ok(result);
                    }
                }
            }
        }
        Err(QueryError::FileNotFound(path.to_string()))
    }

    fn find_group_in_group(
        &self,
        group_handle: &Handle<PBXGroup>,
        path: &str,
    ) -> QueryResult<Handle<PBXGroup>> {
        if let Some(group) = self.registry().get::<PBXGroup>(group_handle.id()) {
            for child_handle in group.children() {
                if let Some(subgroup) = self.registry().get::<PBXGroup>(child_handle.id()) {
                    if let Some(group_path) = subgroup.path() {
                        if group_path == path {
                            return Ok(Handle::from_id(*child_handle.id()));
                        }
                    }
                    if let Ok(result) = self.find_group_in_group(
                        &Handle::from_id(*child_handle.id()),
                        path,
                    ) {
                        return Ok(result);
                    }
                }
            }
        }
        Err(QueryError::GroupNotFound(path.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_query_creation() {
        let project = Project::new("TestProject");
        let query = ProjectQuery::new(&project);
        // Basic creation test
    }
}
