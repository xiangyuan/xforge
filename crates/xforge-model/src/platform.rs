//! Platform and product type definitions

use serde::{Deserialize, Serialize};

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

/// Product type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    /// Application
    #[serde(rename = "com.apple.product-type.application")]
    Application,
    
    /// Framework
    #[serde(rename = "com.apple.product-type.framework")]
    Framework,
    
    /// Static Library
    #[serde(rename = "com.apple.product-type.library.static")]
    StaticLibrary,
    
    /// Dynamic Library
    #[serde(rename = "com.apple.product-type.library.dynamic")]
    DynamicLibrary,
    
    /// Bundle
    #[serde(rename = "com.apple.product-type.bundle")]
    Bundle,
    
    /// Unit Test Bundle
    #[serde(rename = "com.apple.product-type.bundle.unit-test")]
    UnitTest,
    
    /// UI Test Bundle
    #[serde(rename = "com.apple.product-type.bundle.ui-testing")]
    UITest,
    
    /// App Extension
    #[serde(rename = "com.apple.product-type.app-extension")]
    AppExtension,
    
    /// XCTest
    #[serde(rename = "com.apple.product-type.bundle.xctest")]
    XCTest,
}

impl ProductType {
    pub fn as_str(&self) -> &str {
        match self {
            ProductType::Application => "com.apple.product-type.application",
            ProductType::Framework => "com.apple.product-type.framework",
            ProductType::StaticLibrary => "com.apple.product-type.library.static",
            ProductType::DynamicLibrary => "com.apple.product-type.library.dynamic",
            ProductType::Bundle => "com.apple.product-type.bundle",
            ProductType::UnitTest => "com.apple.product-type.bundle.unit-test",
            ProductType::UITest => "com.apple.product-type.bundle.ui-testing",
            ProductType::AppExtension => "com.apple.product-type.app-extension",
            ProductType::XCTest => "com.apple.product-type.bundle.xctest",
        }
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "com.apple.product-type.application" => Some(ProductType::Application),
            "com.apple.product-type.framework" => Some(ProductType::Framework),
            "com.apple.product-type.library.static" => Some(ProductType::StaticLibrary),
            "com.apple.product-type.library.dynamic" => Some(ProductType::DynamicLibrary),
            "com.apple.product-type.bundle" => Some(ProductType::Bundle),
            "com.apple.product-type.bundle.unit-test" => Some(ProductType::UnitTest),
            "com.apple.product-type.bundle.ui-testing" => Some(ProductType::UITest),
            "com.apple.product-type.app-extension" => Some(ProductType::AppExtension),
            "com.apple.product-type.bundle.xctest" => Some(ProductType::XCTest),
            _ => None,
        }
    }
}
