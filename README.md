# XForge

> A modern, type-safe Rust library for creating and manipulating Xcode project files

[![Crates.io](https://img.shields.io/crates/v/xforge.svg)](https://crates.io/crates/xforge)
[![Documentation](https://docs.rs/xforge/badge.svg)](https://docs.rs/xforge)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

✨ **Type-Safe**: Strong typing with `Handle<T>` prevents UUID errors  
🚀 **High Performance**: Zero-allocation UUID generation with collision avoidance  
🎯 **Fluent API**: Builder pattern for intuitive project construction  
🔍 **Powerful Queries**: Filter and search objects with type-safe queries  
🧩 **Modular Design**: Clean architecture with separated concerns  
📦 **No Unwrap**: Comprehensive error handling with `anyhow` and `thiserror`

## Quick Start

\`\`\`rust
use xforge::prelude::*;

fn main() -> Result<()> {
    // Create a new project
    let mut project = Project::builder()
        .name("MyApp")
        .organization("MyCompany")
        .build();
    
    // Add an iOS target
    let target = project.add_target()
        .name("MyApp")
        .product_type(ProductType::Application)
        .platform(Platform::iOS)
        .build()?;
    
    // Add source files
    project.add_file("Sources/AppDelegate.swift")
        .to_target(&target)
        .build()?;
    
    // Save the project
    project.save()?;
    
    Ok(())
}
\`\`\`

## Architecture

XForge is built with a modular architecture:

- **xforge-core**: Core types (Handle, ObjectId, Registry, traits)
- **xforge-model**: Domain models (Project, Target, BuildPhase)
- **xforge-objects**: PBX object definitions
- **xforge-builder**: Fluent builder API
- **xforge-query**: Type-safe query system
- **xforge-serialization**: Plist serialization/deserialization
- **xforge-cli**: Command-line tool (optional)

## UUID Generation Strategy

To avoid UUID collisions across multiple runs, XForge uses a hybrid approach:

- **Timestamp** (48 bits): Ensures uniqueness across different runs
- **Counter** (24 bits): Ensures sequential ordering within a single run
- **Process ID** (24 bits): Ensures uniqueness in concurrent scenarios

This design guarantees:
- ✅ No collision risk across runs
- ✅ High performance (no system random calls)
- ✅ Sortable UUIDs with time ordering
- ✅ Test-friendly with deterministic IDs

## Examples

### Loading and Modifying an Existing Project

\`\`\`rust
use xforge::prelude::*;

// Load existing project
let mut project = Project::load("MyApp.xcodeproj")?;

// Find a target
let target = project.target_by_name("MyApp")
    .ok_or_else(|| anyhow!("Target not found"))?;

// Add a new file
project.add_file("NewFeature/ViewController.swift")
    .to_target(&target)
    .to_group("NewFeature")
    .build()?;

// Save changes
project.save()?;
\`\`\`

### Adding Swift Packages

\`\`\`rust
// Add a Swift package dependency
let alamofire = project.add_swift_package(SwiftPackage {
    url: "https://github.com/Alamofire/Alamofire.git".to_string(),
    version: PackageVersion::Range {
        from: "5.0.0".to_string(),
        to: "6.0.0".to_string(),
    },
})?;

// Add package product to target
project.add_package_product("Alamofire", alamofire, target)?;
\`\`\`

### Query API

\`\`\`rust
// Find all iOS targets
let ios_targets = project.query::<PBXNativeTarget>()
    .with_platform(Platform::iOS)
    .all();

// Find specific files
let swift_files = project.query::<PBXFileReference>()
    .filter(|f| f.path().ends_with(".swift"))
    .all();
\`\`\`

## Design Philosophy

XForge follows these core principles:

1. **Type Safety First**: Leverage Rust's type system to prevent errors at compile time
2. **Zero Cost Abstractions**: High-level APIs without runtime overhead
3. **Backward Compatible**: Fully compatible with Xcode-generated files
4. **Easy to Use**: Fluent builder API for intuitive usage
5. **Extensible**: Plugin architecture for custom extensions

## Installation

Add to your `Cargo.toml`:

\`\`\`toml
[dependencies]
xforge = "0.1"
\`\`\`

## Documentation

- [API Documentation](https://docs.rs/xforge)
- [Design Document](./REDESIGN_PROPOSAL.md)
- [Examples](./examples)

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) first.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Acknowledgments

Inspired by:
- [tuist/XcodeProj](https://github.com/tuist/XcodeProj) - Swift implementation
- [CocoaPods/Xcodeproj](https://github.com/CocoaPods/Xcodeproj) - Ruby implementation
