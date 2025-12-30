use clap::{Parser, Subcommand};
use std::path::PathBuf;
use xforge_model::Project;
use xforge_core::ProductType;

#[derive(Parser)]
#[command(name = "xforge")]
#[command(about = "A command-line tool for managing Xcode projects", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display project information
    Info {
        /// Path to .xcodeproj file
        #[arg(value_name = "PROJECT")]
        project: PathBuf,
    },
    
    /// Add a file to the project
    AddFile {
        /// Path to .xcodeproj file
        #[arg(value_name = "PROJECT")]
        project: PathBuf,
        
        /// File to add
        #[arg(value_name = "FILE")]
        file: PathBuf,
        
        /// Target name to add the file to (optional)
        #[arg(short, long)]
        target: Option<String>,
    },
    
    /// Create a new target
    CreateTarget {
        /// Path to .xcodeproj file
        #[arg(value_name = "PROJECT")]
        project: PathBuf,
        
        /// Target name
        #[arg(value_name = "NAME")]
        name: String,
        
        /// Product type (app, framework, staticlib, dylib)
        #[arg(short, long, default_value = "app")]
        product_type: String,
    },
    
    /// List all targets in the project
    ListTargets {
        /// Path to .xcodeproj file
        #[arg(value_name = "PROJECT")]
        project: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Info { project } => {
            cmd_info(&project)?;
        }
        Commands::AddFile { project, file, target } => {
            cmd_add_file(&project, &file, target.as_deref())?;
        }
        Commands::CreateTarget { project, name, product_type } => {
            cmd_create_target(&project, &name, &product_type)?;
        }
        Commands::ListTargets { project } => {
            cmd_list_targets(&project)?;
        }
    }
    
    Ok(())
}

fn cmd_info(project_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let pbxproj_path = project_path.join("project.pbxproj");
    
    println!("Loading project: {}", project_path.display());
    let project = Project::load(&pbxproj_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;
    
    println!("\nProject Information:");
    println!("  Name: {}", project.metadata().name);
    println!("  Archive Version: {}", project.metadata().archive_version);
    println!("  Object Version: {}", project.metadata().object_version);
    println!("  Development Region: {}", project.metadata().development_region);
    println!("  Total Objects: {}", project.registry().len());
    
    Ok(())
}

fn cmd_add_file(
    project_path: &PathBuf,
    file_path: &PathBuf,
    target_name: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pbxproj_path = project_path.join("project.pbxproj");
    
    println!("Loading project...");
    let mut project = Project::load(&pbxproj_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;
    
    println!("Adding file: {}", file_path.display());
    let file_handle = project.add_file(file_path, None)
        .map_err(|e| format!("Failed to add file: {}", e))?;
    
    if let Some(target_name) = target_name {
        println!("Adding to target: {}", target_name);
        // TODO: Find target by name and add file to it
        println!("Warning: Target-specific file addition not yet implemented");
    }
    
    println!("Saving project...");
    project.save(&pbxproj_path)
        .map_err(|e| format!("Failed to save project: {}", e))?;
    
    println!("✓ File added successfully");
    Ok(())
}

fn cmd_create_target(
    project_path: &PathBuf,
    name: &str,
    product_type_str: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pbxproj_path = project_path.join("project.pbxproj");
    
    let product_type = match product_type_str.to_lowercase().as_str() {
        "app" | "application" => ProductType::Application,
        "framework" => ProductType::Framework,
        "staticlib" | "static-library" => ProductType::StaticLibrary,
        "dylib" | "dynamic-library" => ProductType::DynamicLibrary,
        _ => return Err(format!("Unknown product type: {}", product_type_str).into()),
    };
    
    println!("Loading project...");
    let mut project = Project::load(&pbxproj_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;
    
    println!("Creating target: {} ({:?})", name, product_type);
    project.create_target(name.to_string(), product_type)
        .map_err(|e| format!("Failed to create target: {}", e))?;
    
    println!("Saving project...");
    project.save(&pbxproj_path)
        .map_err(|e| format!("Failed to save project: {}", e))?;
    
    println!("✓ Target created successfully");
    Ok(())
}

fn cmd_list_targets(project_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let pbxproj_path = project_path.join("project.pbxproj");
    
    println!("Loading project...");
    let project = Project::load(&pbxproj_path)
        .map_err(|e| format!("Failed to load project: {}", e))?;
    
    // Use the query API to get all targets
    use xforge_query::RegistryExt;
    use xforge_objects::PBXNativeTarget;
    
    let target_ids = project.registry().get_targets(&project.root_id())
        .map_err(|e| format!("Failed to get targets: {}", e))?;
    
    println!("\nTargets ({}):", target_ids.len());
    for target_id in target_ids {
        if let Some(target) = project.registry().get::<PBXNativeTarget>(&target_id) {
            println!("  • {}", target.name);
            if let Some(product_type) = &target.product_type {
                println!("    Type: {:?}", product_type);
            }
        }
    }
    
    Ok(())
}
