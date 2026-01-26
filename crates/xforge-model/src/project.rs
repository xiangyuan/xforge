//! Project - Core domain model for Xcode projects

use xforge_core::{ObjectId, Registry, Handle};
use xforge_objects::versioning::{ProjectFileFormat, DEFAULT_OBJECT_VERSION, LAST_KNOWN_ARCHIVE_VERSION};
use std::path::{Path, PathBuf};
use std::fs;

/// Xcode project
pub struct Project {
    path: PathBuf,
    registry: Registry,
    root_id: ObjectId,
    metadata: ProjectMetadata,
    file_format: ProjectFileFormat,
}

/// Project metadata
#[derive(Debug, Clone)]
pub struct ProjectMetadata {
    pub archive_version: String,
    pub object_version: String,
    pub name: String,
    pub organization: Option<String>,
    pub development_region: String,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            archive_version: LAST_KNOWN_ARCHIVE_VERSION.to_string(),
            object_version: DEFAULT_OBJECT_VERSION.to_string(),
            name: "Project".to_string(),
            organization: None,
            development_region: "en".to_string(),
        }
    }
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let path = PathBuf::from(format!("{}.xcodeproj", name));
        let registry = Registry::new();
        let root_id = ObjectId::generate();
        
        let mut metadata = ProjectMetadata::default();
        metadata.name = name;
        
        Self { path, registry, root_id, metadata, file_format: ProjectFileFormat::default() }
    }
    
    pub fn name(&self) -> &str {
        &self.metadata.name
    }
    
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    pub fn set_path(&mut self, path: impl Into<PathBuf>) {
        self.path = path.into();
    }
    
    pub fn registry_mut(&mut self) -> &mut Registry {
        &mut self.registry
    }
    
    pub fn registry(&self) -> &Registry {
        &self.registry
    }
    
    pub fn metadata(&self) -> &ProjectMetadata {
        &self.metadata
    }
    
    pub fn metadata_mut(&mut self) -> &mut ProjectMetadata {
        &mut self.metadata
    }
    
    pub fn root_id(&self) -> ObjectId {
        self.root_id
    }
    
    /// Load a project from a .pbxproj file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        
        // Read file content
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        // Parse the plist
        let mut parser = xforge_serialization::PlistParser::new(&content);
        let plist = parser.parse()
            .map_err(|e| format!("Failed to parse plist: {}", e))?;
        
        // Deserialize into registry
        let (registry, root_id) = xforge_objects::deserialize_registry(&plist)
            .map_err(|e| format!("Failed to deserialize project: {}", e))?;
        
        // Extract metadata from plist
        let root_dict = plist.as_dictionary()
            .ok_or("Root value must be a dictionary")?;
        
        let archive_version_num = parse_root_version(root_dict.get("archiveVersion"), LAST_KNOWN_ARCHIVE_VERSION);
        let object_version_num = parse_root_version(root_dict.get("objectVersion"), DEFAULT_OBJECT_VERSION);
        let archive_version = archive_version_num.to_string();
        let object_version = object_version_num.to_string();
        
        // Get project name from .xcodeproj directory name
        // Path is typically: /path/to/ProjectName.xcodeproj/project.pbxproj
        let project_name = path.parent()
            .and_then(|parent| parent.file_stem())
            .and_then(|stem| stem.to_str())
            .unwrap_or("Unnamed")
            .to_string();
        
        let metadata = ProjectMetadata {
            archive_version,
            object_version,
            name: project_name.clone(),
            organization: None,
            development_region: "en".to_string(),
        };

        let classes = root_dict.get("classes")
            .and_then(|v| v.as_dictionary())
            .cloned()
            .unwrap_or_default();
        let mut root_unknown_fields = indexmap::IndexMap::new();
        for (key, value) in root_dict {
            if matches!(key.as_str(), "archiveVersion" | "classes" | "objectVersion" | "objects" | "rootObject") {
                continue;
            }
            root_unknown_fields.insert(key.clone(), value.clone());
        }
        
        // Update PBXProject object's name field
        let mut registry = registry;
        if let Some(project) = registry.get_mut::<xforge_objects::PBXProject>(&root_id) {
            project.name = project_name;
        }
        
        Ok(Self {
            path: path.to_path_buf(),
            registry,
            root_id,
            metadata,
            file_format: ProjectFileFormat {
                archive_version: archive_version_num,
                object_version: object_version_num,
                classes,
                root_unknown_fields,
            },
        })
    }
    
    /// Save the project to a .pbxproj file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();
        
        // Use Xcode-specific writer with proper formatting
        let root_uuid = self.root_id.to_string();
        let content = xforge_objects::xcode_writer::write_xcode_project(&self.registry, &root_uuid, &self.file_format)?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(())
    }

    // === Project Modification APIs ===

    /// Add a file reference to the project
    /// Returns the Handle to the created PBXFileReference
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P, source_tree: Option<String>) -> Result<xforge_core::Handle<xforge_objects::PBXFileReference>, String> {
        use xforge_objects::PBXFileReference;
        
        let path_str = path.as_ref().to_string_lossy().to_string();
        let mut file_ref = PBXFileReference::new(path_str.clone());
        
        // Set source tree (default to "<group>")
        file_ref.source_tree = source_tree.unwrap_or_else(|| "<group>".to_string());
        
        // Extract file name for display
        if let Some(file_name) = path.as_ref().file_name() {
            file_ref.name = Some(file_name.to_string_lossy().to_string());
        }
        
        // Detect file type based on extension
        // Use last_known_file_type (not explicit_file_type) to match Xcode's format
        if let Some(ext) = path.as_ref().extension() {
            let ext_str = ext.to_string_lossy();
            file_ref.last_known_file_type = match ext_str.as_ref() {
                "swift" => Some("sourcecode.swift".to_string()),
                "m" => Some("sourcecode.c.objc".to_string()),
                "h" => Some("sourcecode.c.h".to_string()),
                "cpp" | "cc" => Some("sourcecode.cpp.cpp".to_string()),
                "framework" => Some("wrapper.framework".to_string()),
                _ => None,
            };
        }
        
        let handle = self.registry.register(file_ref);
        Ok(handle)
    }

    /// Create a new native target in the project
    /// Returns the Handle to the created PBXNativeTarget
    pub fn create_target(&mut self, name: String, product_type: xforge_core::ProductType) -> Result<xforge_core::Handle<xforge_objects::PBXNativeTarget>, String> {
        use xforge_objects::PBXNativeTarget;
        
        let mut target = PBXNativeTarget::new(name);
        target.product_type = Some(product_type);
        
        // Create build configuration list
        let config_list = self.create_configuration_list()?;
        target.build_configuration_list = Some(*config_list.id());
        
        // Create default build phases
        let sources_phase = self.create_sources_build_phase()?;
        let frameworks_phase = self.create_frameworks_build_phase()?;
        let resources_phase = self.create_resources_build_phase()?;
        
        target.build_phases.push(*sources_phase.id());
        target.build_phases.push(*frameworks_phase.id());
        target.build_phases.push(*resources_phase.id());
        
        let handle = self.registry.register(target);
        
        // Add target to the root PBXProject
        if let Some(project_obj) = self.registry.get_mut::<xforge_objects::PBXProject>(&self.root_id) {
            project_obj.targets.push(*handle.id());
        }
        
        Ok(handle)
    }

    /// Create a configuration list with Debug and Release configurations
    fn create_configuration_list(&mut self) -> Result<xforge_core::Handle<xforge_objects::XCConfigurationList>, String> {
        use xforge_objects::{XCConfigurationList, XCBuildConfiguration};
        
        // Create Debug configuration
        let debug_config = XCBuildConfiguration::new("Debug".to_string());
        let debug_handle = self.registry.register(debug_config);
        
        // Create Release configuration
        let release_config = XCBuildConfiguration::new("Release".to_string());
        let release_handle = self.registry.register(release_config);
        
        // Create configuration list
        let mut config_list = XCConfigurationList::new();
        config_list.build_configurations.push(debug_handle);
        config_list.build_configurations.push(release_handle);
        config_list.default_configuration_name = Some("Release".to_string());
        
        Ok(self.registry.register(config_list))
    }

    /// Create an empty sources build phase
    fn create_sources_build_phase(&mut self) -> Result<xforge_core::Handle<xforge_objects::PBXSourcesBuildPhase>, String> {
        use xforge_objects::PBXSourcesBuildPhase;
        Ok(self.registry.register(PBXSourcesBuildPhase::new()))
    }

    /// Create an empty frameworks build phase
    fn create_frameworks_build_phase(&mut self) -> Result<xforge_core::Handle<xforge_objects::PBXFrameworksBuildPhase>, String> {
        use xforge_objects::PBXFrameworksBuildPhase;
        Ok(self.registry.register(PBXFrameworksBuildPhase::new()))
    }

    /// Create an empty resources build phase
    fn create_resources_build_phase(&mut self) -> Result<xforge_core::Handle<xforge_objects::PBXResourcesBuildPhase>, String> {
        use xforge_objects::PBXResourcesBuildPhase;
        Ok(self.registry.register(PBXResourcesBuildPhase::new()))
    }

    /// Add a file to a target's sources build phase
    pub fn add_file_to_target(
        &mut self,
        file_ref: xforge_core::Handle<xforge_objects::PBXFileReference>,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<(), String> {
        use xforge_objects::{PBXBuildFile, PBXSourcesBuildPhase};
        
        // Get the sources build phase ID from target (separate scope to release borrow)
        let sources_phase_id = {
            let target_obj = self.registry.get::<xforge_objects::PBXNativeTarget>(target.id())
                .ok_or("Target not found")?;
            
            // Find the sources build phase
            target_obj.build_phases.iter()
                .find(|&phase_id| {
                    self.registry.get::<PBXSourcesBuildPhase>(phase_id).is_some()
                })
                .copied()
                .ok_or("No sources build phase found in target")?
        };
        
        // Create a build file
        let build_file = xforge_objects::PBXBuildFile::new(file_ref);
        let build_file_handle = self.registry.register(build_file);
        
        // Add build file to sources phase
        if let Some(sources_phase) = self.registry.get_mut::<PBXSourcesBuildPhase>(&sources_phase_id) {
            sources_phase.files.push(build_file_handle);
        }
        
        Ok(())
    }

    /// Add a group to organize files
    pub fn add_group(&mut self, name: String, path: Option<String>) -> Result<xforge_core::Handle<xforge_objects::PBXGroup>, String> {
        use xforge_objects::PBXGroup;
        
        let mut group = PBXGroup::new(name);
        group.path = path;
        group.source_tree = "<group>".to_string();
        
        Ok(self.registry.register(group))
    }

    /// Add a file reference to a group
    pub fn add_file_to_group(
        &mut self,
        file_ref: xforge_core::Handle<xforge_objects::PBXFileReference>,
        group: xforge_core::Handle<xforge_objects::PBXGroup>,
    ) -> Result<(), String> {
        if let Some(group_obj) = self.registry.get_mut::<xforge_objects::PBXGroup>(group.id()) {
            group_obj.children.push(*file_ref.id());
            Ok(())
        } else {
            Err("Group not found".to_string())
        }
    }

    /// Update build settings for a configuration
    pub fn update_build_settings(
        &mut self,
        config: xforge_core::Handle<xforge_objects::XCBuildConfiguration>,
        key: String,
        value: String,
    ) -> Result<(), String> {
        if let Some(config_obj) = self.registry.get_mut::<xforge_objects::XCBuildConfiguration>(config.id()) {
            config_obj.build_settings.insert(key, value);
            Ok(())
        } else {
            Err("Configuration not found".to_string())
        }
    }
    
    /// Add a framework with optional weak/optional attributes
    /// Example: add_framework("CoreGraphics.framework", target, vec!["Weak"])
    /// Find existing file reference by path
    fn find_file_reference(&self, path: &str) -> Option<xforge_core::Handle<xforge_objects::PBXFileReference>> {
        use xforge_objects::PBXFileReference;
        
        // Search all file references in the registry
        // IMPORTANT: Use the registry key (String UUID), convert to ObjectId
        for (registry_key, obj) in self.registry.iter() {
            if let Some(file_ref) = obj.as_any().downcast_ref::<PBXFileReference>() {
                if file_ref.path.as_deref() == Some(path) {
                    // Convert registry key (String) to ObjectId
                    if let Ok(object_id) = xforge_core::ObjectId::from_uuid_string(registry_key) {
                        return Some(xforge_core::Handle::from_id(object_id));
                    }
                }
            }
        }
        None
    }


    pub fn add_framework(
        &mut self,
        framework_name: &str,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
        attributes: Vec<String>,
    ) -> Result<xforge_core::Handle<xforge_objects::PBXFileReference>, String> {
        use xforge_objects::{PBXBuildFile, PBXFrameworksBuildPhase, PBXGroup};
        
        // Find or create framework file reference (avoid duplicates)
        let framework_path = format!("System/Library/Frameworks/{}", framework_name);
        let file_ref = if let Some(existing_ref) = self.find_file_reference(&framework_path) {
            existing_ref
        } else {
            self.add_file(&framework_path, Some("SDKROOT".to_string()))?
        };
        
        // Find or create Frameworks group and add the framework to it
        let frameworks_group_id = self.find_or_create_frameworks_group()?;
        if let Some(group) = self.registry.get_mut::<PBXGroup>(&frameworks_group_id) {
            // Check if not already in group (children are now ObjectIds, not Handles)
            if !group.children.iter().any(|id| id == file_ref.id()) {
                group.children.push(*file_ref.id());
            }
        }
        
        // Get target's frameworks build phase
        let target_obj = self.registry.get::<xforge_objects::PBXNativeTarget>(target.id())
            .ok_or("Target not found")?;
        
        let frameworks_phase_id = target_obj.build_phases.iter()
            .find(|phase_id| {
                self.registry.get::<PBXFrameworksBuildPhase>(phase_id).is_some()
            })
            .ok_or("Frameworks build phase not found")?
            .clone(); // Clone to avoid borrow conflict
        
        // Check if this framework is already added to this target's frameworks build phase
        let phase = self.registry.get::<PBXFrameworksBuildPhase>(&frameworks_phase_id)
            .ok_or("Frameworks build phase not found")?;
        
        let already_in_phase = phase.files.iter().any(|build_file_handle| {
            if let Some(build_file) = self.registry.get::<PBXBuildFile>(build_file_handle.id()) {
                build_file.file_ref.id() == file_ref.id()
            } else {
                false
            }
        });
        
        if !already_in_phase {
            // Create build file with attributes
            let mut build_file = PBXBuildFile::new(file_ref.clone());
            if !attributes.is_empty() {
                let mut settings = std::collections::HashMap::new();
                settings.insert("ATTRIBUTES".to_string(), attributes.join(","));
                build_file.settings = Some(settings);
            }
            
            let build_file_handle = self.registry.register(build_file);
            
            // Add to frameworks build phase
            if let Some(phase) = self.registry.get_mut::<PBXFrameworksBuildPhase>(&frameworks_phase_id) {
                phase.files.push(build_file_handle);
            }
        }
        
        Ok(file_ref)
    }
    
    /// Find or create the Frameworks group
    fn find_or_create_frameworks_group(&mut self) -> Result<xforge_core::ObjectId, String> {
        use xforge_objects::PBXGroup;
        
        // Get the root project to find main group
        let project_obj = self.registry.get::<xforge_objects::PBXProject>(&self.root_id)
            .ok_or("Root project not found")?;
        
        let main_group_id = project_obj.main_group.ok_or("Main group not found")?;
        
        // Try to find existing Frameworks group in main group's children
        if let Some(main_group) = self.registry.get::<PBXGroup>(&main_group_id) {
            for child_id in &main_group.children {
                if let Some(child_group) = self.registry.get::<PBXGroup>(child_id) {
                    if child_group.name.as_deref() == Some("Frameworks") {
                        return Ok(child_id.clone());
                    }
                }
            }
        }
        
        // Frameworks group not found, create it
        let frameworks_group = PBXGroup::new("Frameworks".to_string());
        let frameworks_group_handle = self.registry.register(frameworks_group);
        let frameworks_group_id = frameworks_group_handle.id().clone();
        
        // Add to main group (children are now ObjectIds, supporting any child type)
        if let Some(main_group) = self.registry.get_mut::<PBXGroup>(&main_group_id) {
            main_group.children.push(frameworks_group_id.clone());
        }
        
        Ok(frameworks_group_id)
    }
    
    /// Add a system framework (convenience method)
    pub fn add_system_framework(
        &mut self,
        framework_name: &str,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<xforge_core::Handle<xforge_objects::PBXFileReference>, String> {
        self.add_framework(framework_name, target, vec![])
    }
    
    /// Add a weak framework
    pub fn add_weak_framework(
        &mut self,
        framework_name: &str,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<xforge_core::Handle<xforge_objects::PBXFileReference>, String> {
        self.add_framework(framework_name, target, vec!["Weak".to_string()])
    }
    
    /// Embed a framework with code signing
    /// Finds or creates a "Embed Frameworks" copy files build phase
    pub fn embed_framework(
        &mut self,
        framework_ref: xforge_core::Handle<xforge_objects::PBXFileReference>,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<(), String> {
        use xforge_objects::{PBXBuildFile, PBXCopyFilesBuildPhase};
        
        // Find or create Embed Frameworks phase
        let embed_phase = self.get_or_create_embed_frameworks_phase(target.clone())?;
        
        // Create build file with code signing attributes
        let mut build_file = PBXBuildFile::new(framework_ref);
        let mut settings = std::collections::HashMap::new();
        settings.insert("ATTRIBUTES".to_string(), "CodeSignOnCopy,RemoveHeadersOnCopy".to_string());
        build_file.settings = Some(settings);
        
        let build_file_handle = self.registry.register(build_file);
        
        // Add to embed phase
        if let Some(phase) = self.registry.get_mut::<PBXCopyFilesBuildPhase>(embed_phase.id()) {
            phase.files.push(build_file_handle);
        }
        
        Ok(())
    }
    
    /// Get or create an "Embed Frameworks" copy files build phase
    fn get_or_create_embed_frameworks_phase(
        &mut self,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<xforge_core::Handle<xforge_objects::PBXCopyFilesBuildPhase>, String> {
        use xforge_objects::PBXCopyFilesBuildPhase;
        
        let target_obj = self.registry.get::<xforge_objects::PBXNativeTarget>(target.id())
            .ok_or("Target not found")?;
        
        // Try to find existing Embed Frameworks phase
        for phase_id in &target_obj.build_phases {
            if let Some(copy_phase) = self.registry.get::<PBXCopyFilesBuildPhase>(phase_id) {
                if copy_phase.name.as_deref() == Some("Embed Frameworks") {
                    return Ok(xforge_core::Handle::from_id(phase_id.clone()));
                }
            }
        }
        
        // Create new Embed Frameworks phase
        let mut embed_phase = PBXCopyFilesBuildPhase::new("", 10); // Empty path, Frameworks destination
        embed_phase.name = Some("Embed Frameworks".to_string());
        
        let handle = self.registry.register(embed_phase);
        
        // Add to target
        if let Some(target_obj) = self.registry.get_mut::<xforge_objects::PBXNativeTarget>(target.id()) {
            target_obj.build_phases.push(*handle.id());
        }
        
        Ok(handle)
    }
    
    /// Append to array build setting for a target's configurations
    /// Example: append_to_target_setting("OTHER_LDFLAGS", "-ObjC", target)
    pub fn append_to_target_setting(
        &mut self,
        key: &str,
        value: &str,
        target: xforge_core::Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<(), String> {
        let target_obj = self.registry.get::<xforge_objects::PBXNativeTarget>(target.id())
            .ok_or("Target not found")?;
        
        let config_list_id = target_obj.build_configuration_list
            .ok_or("Target has no configuration list")?;
        
        let config_list = self.registry.get::<xforge_objects::XCConfigurationList>(&config_list_id)
            .ok_or("Configuration list not found")?;
        
        // Clone the handles to avoid borrow conflict
        let config_handles: Vec<_> = config_list.build_configurations.iter().cloned().collect();
        
        // Update all configurations
        for config_handle in config_handles {
            if let Some(config) = self.registry.get_mut::<xforge_objects::XCBuildConfiguration>(config_handle.id()) {
                config.append_to_array_setting(key, value);
            }
        }
        
        Ok(())
    }

    /// Add a shell script build phase to a target
    /// 
    /// # Arguments
    /// * `name` - Display name for the script phase (e.g., "Run SwiftLint")
    /// * `script` - Shell script content to execute
    /// * `target` - Target handle to add the script phase to
    /// 
    /// # Returns
    /// Handle to the created PBXShellScriptBuildPhase
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::Project;
    /// # use xforge_core::ProductType;
    /// # let mut project = Project::new("MyApp");
    /// # let target = project.create_target("MyApp", ProductType::Application).unwrap();
    /// let script = r#"
    /// if which swiftlint >/dev/null; then
    ///   swiftlint
    /// else
    ///   echo "warning: SwiftLint not installed"
    /// fi
    /// "#;
    /// let phase = project.add_shell_script_phase("Run SwiftLint", script, &target)
    ///     .expect("Failed to add script phase");
    /// ```
    pub fn add_shell_script_phase(
        &mut self,
        name: impl Into<String>,
        script: impl Into<String>,
        target: &Handle<xforge_objects::PBXNativeTarget>,
    ) -> Result<Handle<xforge_objects::PBXShellScriptBuildPhase>, String> {
        // Create the shell script phase
        let phase = xforge_objects::PBXShellScriptBuildPhase::new(script)
            .with_name(name);
        
        let phase_handle = self.registry.register(phase);
        
        // Add phase to target's build phases
        if let Some(target_obj) = self.registry.get_mut::<xforge_objects::PBXNativeTarget>(target.id()) {
            target_obj.build_phases.push(*phase_handle.id());
        } else {
            return Err("Target not found".to_string());
        }
        
        Ok(phase_handle)
    }

    /// Add a shell script phase with input and output files
    /// 
    /// This is useful for scripts that need to track dependencies for incremental builds
    /// 
    /// # Example
    /// ```no_run
    /// # use xforge_model::Project;
    /// # use xforge_core::ProductType;
    /// # let mut project = Project::new("MyApp");
    /// # let target = project.create_target("MyApp", ProductType::Application).unwrap();
    /// let input_files = vec!["$(SRCROOT)/Resources/Strings.json"];
    /// let output_files = vec!["$(DERIVED_FILE_DIR)/Localizable.strings"];
    /// 
    /// project.add_shell_script_phase_with_files(
    ///     "Generate Strings",
    ///     "generate_strings.sh",
    ///     &target,
    ///     input_files,
    ///     output_files
    /// ).expect("Failed to add script phase");
    /// ```
    pub fn add_shell_script_phase_with_files(
        &mut self,
        name: impl Into<String>,
        script: impl Into<String>,
        target: &Handle<xforge_objects::PBXNativeTarget>,
        input_paths: Vec<impl Into<String>>,
        output_paths: Vec<impl Into<String>>,
    ) -> Result<Handle<xforge_objects::PBXShellScriptBuildPhase>, String> {
        // Create the shell script phase
        let mut phase = xforge_objects::PBXShellScriptBuildPhase::new(script)
            .with_name(name);
        
        // Add input paths
        for path in input_paths {
            phase.add_input_path(path);
        }
        
        // Add output paths
        for path in output_paths {
            phase.add_output_path(path);
        }
        
        let phase_handle = self.registry.register(phase);
        
        // Add phase to target's build phases
        if let Some(target_obj) = self.registry.get_mut::<xforge_objects::PBXNativeTarget>(target.id()) {
            target_obj.build_phases.push(*phase_handle.id());
        } else {
            return Err("Target not found".to_string());
        }
        
        Ok(phase_handle)
    }
}

fn parse_root_version(value: Option<&xforge_serialization::PlistValue>, default: i64) -> i64 {
    match value {
        Some(v) => {
            if let Some(i) = v.as_integer() {
                i
            } else if let Some(s) = v.as_string() {
                s.parse::<i64>().unwrap_or(default)
            } else {
                default
            }
        }
        None => default,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use xforge_core::ProductType;

    #[test]
    fn test_project_creation() {
        let project = Project::new("TestProject");
        assert_eq!(project.name(), "TestProject");
        assert!(project.registry().is_empty());
    }

    #[test]
    fn test_add_file() {
        let mut project = Project::new("TestProject");
        
        // Add a Swift file
        let file_handle = project.add_file("Sources/main.swift", None)
            .expect("Failed to add file");
        
        // Verify file was added
        assert_eq!(project.registry().len(), 1);
        
        // Verify file reference properties
        let file_ref = project.registry().get::<xforge_objects::PBXFileReference>(file_handle.id())
            .expect("File reference not found");
        assert_eq!(file_ref.path, Some("Sources/main.swift".to_string()));
        assert_eq!(file_ref.name, Some("main.swift".to_string()));
        assert_eq!(file_ref.explicit_file_type, Some("sourcecode.swift".to_string()));
    }

    #[test]
    fn test_create_target() {
        let mut project = Project::new("TestProject");
        
        // Create a new target
        let target_handle = project.create_target("MyApp".to_string(), ProductType::Application)
            .expect("Failed to create target");
        
        // Verify target was created with build phases and configurations
        // Should have: target + 3 build phases + config list + 2 configs = 7 objects
        assert_eq!(project.registry().len(), 7);
        
        // Verify target properties
        let target = project.registry().get::<xforge_objects::PBXNativeTarget>(target_handle.id())
            .expect("Target not found");
        assert_eq!(target.name, "MyApp");
        assert_eq!(target.product_type, Some(ProductType::Application));
        assert_eq!(target.build_phases.len(), 3); // sources, frameworks, resources
    }

    #[test]
    fn test_add_file_to_group() {
        let mut project = Project::new("TestProject");
        
        // Create a group
        let group_handle = project.add_group("Sources".to_string(), Some("Sources".to_string()))
            .expect("Failed to create group");
        
        // Add a file
        let file_handle = project.add_file("main.swift", None)
            .expect("Failed to add file");
        
        // Add file to group
        project.add_file_to_group(file_handle.clone(), group_handle.clone())
            .expect("Failed to add file to group");
        
        // Verify group contains the file
        let group = project.registry().get::<xforge_objects::PBXGroup>(group_handle.id())
            .expect("Group not found");
        assert_eq!(group.children.len(), 1);
        assert_eq!(*group.children[0].id(), *file_handle.id());
    }

    #[test]
    fn test_add_file_to_target() {
        let mut project = Project::new("TestProject");
        
        // Create target
        let target_handle = project.create_target("MyApp".to_string(), ProductType::Application)
            .expect("Failed to create target");
        
        // Add a file
        let file_handle = project.add_file("main.swift", None)
            .expect("Failed to add file");
        
        // Add file to target's build phase
        project.add_file_to_target(file_handle.clone(), target_handle.clone())
            .expect("Failed to add file to target");
        
        // Verify build file was created and added to sources phase
        // Initial 7 objects + 1 file + 1 build file = 9
        assert_eq!(project.registry().len(), 9);
    }

    #[test]
    fn test_update_build_settings() {
        let mut project = Project::new("TestProject");
        
        // Create a standalone configuration for testing
        let config = xforge_objects::XCBuildConfiguration::new("Debug".to_string());
        let config_handle = project.registry_mut().register(config);
        
        // Update build settings
        project.update_build_settings(config_handle.clone(), "PRODUCT_NAME".to_string(), "MyApp".to_string())
            .expect("Failed to update build settings");
        
        // Verify setting was updated
        let config = project.registry().get::<xforge_objects::XCBuildConfiguration>(config_handle.id())
            .expect("Configuration not found");
        assert_eq!(config.build_settings.get("PRODUCT_NAME"), Some(&"MyApp".to_string()));
    }

    #[test]
    fn test_end_to_end_project_modification() {
        let mut project = Project::new("CompleteTest");
        
        // Create a target
        let target = project.create_target("TestApp".to_string(), ProductType::Application)
            .expect("Failed to create target");
        
        // Create a group
        let sources_group = project.add_group("Sources".to_string(), Some("Sources".to_string()))
            .expect("Failed to create group");
        
        // Add multiple files
        let main_file = project.add_file("main.swift", None)
            .expect("Failed to add main.swift");
        let helper_file = project.add_file("Helper.swift", None)
            .expect("Failed to add Helper.swift");
        
        // Add files to group
        project.add_file_to_group(main_file.clone(), sources_group.clone())
            .expect("Failed to add main to group");
        project.add_file_to_group(helper_file.clone(), sources_group.clone())
            .expect("Failed to add helper to group");
        
        // Add files to target
        project.add_file_to_target(main_file.clone(), target.clone())
            .expect("Failed to add main to target");
        project.add_file_to_target(helper_file.clone(), target.clone())
            .expect("Failed to add helper to target");
        
        // Verify complete structure
        // 1 target + 3 build phases + 1 config list + 2 configs + 1 group + 2 files + 2 build files = 12
        assert_eq!(project.registry().len(), 12);
        
        // Verify group structure
        let group = project.registry().get::<xforge_objects::PBXGroup>(sources_group.id())
            .expect("Group not found");
        assert_eq!(group.children.len(), 2);
    }

    #[test]
    fn test_add_shell_script_phase() {
        let mut project = Project::new("TestProject");
        
        // Create a target
        let target = project.create_target("MyApp".to_string(), ProductType::Application)
            .expect("Failed to create target");
        
        // Add a simple shell script phase
        let script = "echo 'Building...'";
        let phase_handle = project.add_shell_script_phase("Custom Build Step", script, &target)
            .expect("Failed to add script phase");
        
        // Verify phase was created
        let phase = project.registry().get::<xforge_objects::PBXShellScriptBuildPhase>(phase_handle.id())
            .expect("Phase not found");
        assert_eq!(phase.name, Some("Custom Build Step".to_string()));
        assert_eq!(phase.shell_script, "echo 'Building...'");
        assert_eq!(phase.input_paths.len(), 0);
        assert_eq!(phase.output_paths.len(), 0);
        
        // Verify phase was added to target
        let target_obj = project.registry().get::<xforge_objects::PBXNativeTarget>(target.id())
            .expect("Target not found");
        assert_eq!(target_obj.build_phases.len(), 4); // sources, frameworks, resources, shell script
    }

    #[test]
    fn test_add_shell_script_phase_with_files() {
        let mut project = Project::new("TestProject");
        
        // Create a target
        let target = project.create_target("MyApp".to_string(), ProductType::Application)
            .expect("Failed to create target");
        
        // Add shell script with input/output files
        let input_files = vec!["$(SRCROOT)/config.json", "$(SRCROOT)/template.txt"];
        let output_files = vec!["$(DERIVED_FILE_DIR)/Generated.swift"];
        
        let phase_handle = project.add_shell_script_phase_with_files(
            "Code Generation",
            "swift codegen.swift",
            &target,
            input_files,
            output_files
        ).expect("Failed to add script phase");
        
        // Verify phase configuration
        let phase = project.registry().get::<xforge_objects::PBXShellScriptBuildPhase>(phase_handle.id())
            .expect("Phase not found");
        assert_eq!(phase.name, Some("Code Generation".to_string()));
        assert_eq!(phase.shell_script, "swift codegen.swift");
        assert_eq!(phase.input_paths.len(), 2);
        assert_eq!(phase.output_paths.len(), 1);
        assert_eq!(phase.input_paths[0], "$(SRCROOT)/config.json");
        assert_eq!(phase.output_paths[0], "$(DERIVED_FILE_DIR)/Generated.swift");
    }
}
