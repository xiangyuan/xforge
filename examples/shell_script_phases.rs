//! Example: Adding Shell Script Build Phases
//! 
//! This demonstrates how to add custom shell scripts to Xcode targets,
//! similar to Ruby xcodeproj's shell script phase management.

use xforge_model::Project;
use xforge_core::ProductType;

fn main() {
    println!("=== xforge Shell Script Phases Demo ===\n");

    // Create a new project
    let mut project = Project::new("MyApp");
    println!("1. Creating new project...");
    
    // Create iOS application target
    let target = project.create_target("MyApp".to_string(), ProductType::Application)
        .expect("Failed to create target");
    println!("   ✓ Target created: MyApp\n");

    // Example 1: Simple shell script (e.g., SwiftLint)
    println!("2. Adding SwiftLint script phase:");
    let swiftlint_script = r#"
if which swiftlint >/dev/null; then
  swiftlint
else
  echo "warning: SwiftLint not installed, download from https://github.com/realm/SwiftLint"
fi
"#;
    project.add_shell_script_phase("Run SwiftLint", swiftlint_script, &target)
        .expect("Failed to add SwiftLint phase");
    println!("   ✓ SwiftLint phase added\n");

    // Example 2: Code generation with input/output files
    println!("3. Adding code generation phase:");
    let codegen_script = r#"
echo "Generating Swift code from templates..."
python3 "${SRCROOT}/scripts/generate_code.py" \
  --input "${SRCROOT}/Resources/config.json" \
  --output "${DERIVED_FILE_DIR}/Generated.swift"
"#;
    let input_files = vec![
        "$(SRCROOT)/Resources/config.json",
        "$(SRCROOT)/scripts/generate_code.py",
    ];
    let output_files = vec![
        "$(DERIVED_FILE_DIR)/Generated.swift",
    ];
    
    project.add_shell_script_phase_with_files(
        "Generate Code",
        codegen_script,
        &target,
        input_files,
        output_files
    ).expect("Failed to add code generation phase");
    println!("   ✓ Code generation phase added\n");

    // Example 3: Resource processing
    println!("4. Adding asset processing phase:");
    let asset_script = r#"
echo "Processing custom assets..."
for file in "${SRCROOT}/RawAssets"/*.png; do
  echo "Processing: $file"
  # Custom processing here
done
"#;
    project.add_shell_script_phase("Process Custom Assets", asset_script, &target)
        .expect("Failed to add asset processing phase");
    println!("   ✓ Asset processing phase added\n");

    // Example 4: Build information script
    println!("5. Adding build info script:");
    let build_info_script = r#"
echo "Generating build info..."
BUILD_DATE=$(date '+%Y-%m-%d %H:%M:%S')
GIT_COMMIT=$(git rev-parse --short HEAD)
echo "let buildDate = \"${BUILD_DATE}\"" > "${SRCROOT}/BuildInfo.swift"
echo "let gitCommit = \"${GIT_COMMIT}\"" >> "${SRCROOT}/BuildInfo.swift"
"#;
    let build_info_input = vec!["$(SRCROOT)/.git/HEAD"];
    let build_info_output = vec!["$(SRCROOT)/BuildInfo.swift"];
    
    project.add_shell_script_phase_with_files(
        "Generate Build Info",
        build_info_script,
        &target,
        build_info_input,
        build_info_output
    ).expect("Failed to add build info phase");
    println!("   ✓ Build info phase added\n");

    // Summary
    println!("6. Project summary:");
    println!("   - Total objects: {}", project.registry().len());
    println!("   ✓ Project configured with 4 shell script phases\n");

    // Verify all phases were added
    let target_obj = project.registry()
        .get::<xforge_objects::PBXNativeTarget>(target.id())
        .expect("Target not found");
    println!("   Build phases in target:");
    println!("   - Sources phase");
    println!("   - Frameworks phase");
    println!("   - Resources phase");
    println!("   - Run SwiftLint (shell script)");
    println!("   - Generate Code (shell script with dependencies)");
    println!("   - Process Custom Assets (shell script)");
    println!("   - Generate Build Info (shell script with dependencies)");
    println!("   Total: {} phases\n", target_obj.build_phases.len());

    println!("=== Demo Complete ===\n");
    println!("Key features demonstrated:");
    println!("✓ add_shell_script_phase() - Simple scripts");
    println!("✓ add_shell_script_phase_with_files() - Scripts with input/output tracking");
    println!("✓ SwiftLint integration");
    println!("✓ Code generation");
    println!("✓ Asset processing");
    println!("✓ Build info generation");
}
