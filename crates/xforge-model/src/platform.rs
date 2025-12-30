//! Platform definitions

use serde::{Deserialize, Serialize};

// Re-export ProductType from xforge-core
pub use xforge_core::ProductType;

/// Target platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    /// iOS devices
    #[serde(rename = "iphoneos")]
    iOS,
    
    /// iOS Simulator
    #[serde(rename = "iphonesimulator")]
    iOSSimulator,
    
    /// macOS
    #[serde(rename = "macosx")]
    macOS,
    
    /// tvOS
    #[serde(rename = "appletvos")]
    tvOS,
    
    /// watchOS
    #[serde(rename = "watchos")]
    watchOS,
    
    /// visionOS
    #[serde(rename = "xros")]
    visionOS,
}

impl Platform {
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::iOS => "iphoneos",
            Platform::iOSSimulator => "iphonesimulator",
            Platform::macOS => "macosx",
            Platform::tvOS => "appletvos",
            Platform::watchOS => "watchos",
            Platform::visionOS => "xros",
        }
    }
}
