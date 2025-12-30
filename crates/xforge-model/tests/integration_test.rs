use xforge_model::Project;

#[test]
fn test_load_real_xcode_project() {
    let pbxproj_path = "/Users/boss/Desktop/AAB/test/test.xcodeproj/project.pbxproj";
    
    // Load the project
    let result = Project::load(pbxproj_path);
    
    assert!(result.is_ok(), "Failed to load project: {:?}", result.err());
    
    let project = result.unwrap();
    
    // Verify basic metadata
    println!("Archive Version: {}", project.metadata().archive_version);
    println!("Object Version: {}", project.metadata().object_version);
    println!("Number of objects: {}", project.registry().len());
    
    assert_eq!(project.metadata().archive_version, "1");
    assert_eq!(project.metadata().object_version, "56");  // This test project is v56, not v77
    assert!(project.registry().len() > 0, "Registry should not be empty");
}

#[test]
fn test_load_and_save_roundtrip() {
    let original_path = "/Users/boss/Desktop/AAB/test/test.xcodeproj/project.pbxproj";
    let temp_path = "/tmp/test_project_roundtrip.pbxproj";
    
    // Load original project
    let project = Project::load(original_path)
        .expect("Failed to load original project");
    
    let original_object_count = project.registry().len();
    println!("Original project has {} objects", original_object_count);
    
    // Save to temporary file
    project.save(temp_path)
        .expect("Failed to save project");
    
    // Load the saved file
    let reloaded_project = Project::load(temp_path)
        .expect("Failed to reload saved project");
    
    // Verify data integrity
    assert_eq!(
        reloaded_project.metadata().archive_version,
        project.metadata().archive_version,
        "Archive version mismatch"
    );
    
    assert_eq!(
        reloaded_project.metadata().object_version,
        project.metadata().object_version,
        "Object version mismatch"
    );
    
    assert_eq!(
        reloaded_project.registry().len(),
        original_object_count,
        "Object count mismatch after roundtrip"
    );
    
    println!("Roundtrip test passed! {} objects preserved", reloaded_project.registry().len());
    
    // Cleanup
    std::fs::remove_file(temp_path).ok();
}

#[test]
fn test_project_metadata() {
    let pbxproj_path = "/Users/boss/Desktop/AAB/test/test.xcodeproj/project.pbxproj";
    
    let project = Project::load(pbxproj_path)
        .expect("Failed to load project");
    
    // Check metadata fields
    println!("Project Metadata:");
    println!("  Archive Version: {}", project.metadata().archive_version);
    println!("  Object Version: {}", project.metadata().object_version);
    println!("  Name: {}", project.metadata().name);
    
    // Verify expected values for this test project
    assert_eq!(project.metadata().archive_version, "1");
    assert_eq!(project.metadata().object_version, "56");
}

#[test]
fn test_save_preserves_format() {
    let original_path = "/Users/boss/Desktop/AAB/test/test.xcodeproj/project.pbxproj";
    let temp_path = "/tmp/test_project_format.pbxproj";
    
    // Load and save
    let project = Project::load(original_path)
        .expect("Failed to load project");
    
    project.save(temp_path)
        .expect("Failed to save project");
    
    // Read both files
    let original_content = std::fs::read_to_string(original_path)
        .expect("Failed to read original file");
    let saved_content = std::fs::read_to_string(temp_path)
        .expect("Failed to read saved file");
    
    // Check that both contain required metadata
    assert!(saved_content.contains("archiveVersion = 1"));
    assert!(saved_content.contains("objectVersion = 56"));
    
    println!("Original size: {} bytes", original_content.len());
    println!("Saved size: {} bytes", saved_content.len());
    
    // Cleanup
    std::fs::remove_file(temp_path).ok();
}
