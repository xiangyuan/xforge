//! XCSwiftPackageProductDependency and XCRemoteSwiftPackageReference

use xforge_core::{ObjectId, PBXObject};

/// Swift Package Product Dependency
#[derive(Debug, Clone)]
pub struct XCSwiftPackageProductDependency {
    id: ObjectId,
    pub package: ObjectId,
    pub product_name: String,
}

impl XCSwiftPackageProductDependency {
    pub fn new(package: ObjectId, product_name: impl Into<String>) -> Self {
        Self {
            id: ObjectId::generate(),
            package,
            product_name: product_name.into(),
        }
    }
    
    pub fn product_name(&self) -> &str {
        &self.product_name
    }
}

impl PBXObject for XCSwiftPackageProductDependency {
    fn isa(&self) -> &'static str {
        "XCSwiftPackageProductDependency"
    }
}

/// Remote Swift Package Reference
#[derive(Debug, Clone)]
pub struct XCRemoteSwiftPackageReference {
    id: ObjectId,
    pub repository_url: String,
    pub requirement: PackageRequirement,
}

#[derive(Debug, Clone)]
pub enum PackageRequirement {
    UpToNextMajorVersion(String),
    UpToNextMinorVersion(String),
    Exact(String),
    Branch(String),
    Revision(String),
}

impl XCRemoteSwiftPackageReference {
    pub fn new(repository_url: impl Into<String>, requirement: PackageRequirement) -> Self {
        Self {
            id: ObjectId::generate(),
            repository_url: repository_url.into(),
            requirement,
        }
    }
    
    pub fn repository_url(&self) -> &str {
        &self.repository_url
    }
}

impl PBXObject for XCRemoteSwiftPackageReference {
    fn isa(&self) -> &'static str {
        "XCRemoteSwiftPackageReference"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swift_package_product_dependency() {
        let package_id = ObjectId::generate();
        let dep = XCSwiftPackageProductDependency::new(package_id, "Alamofire");
        assert_eq!(dep.isa(), "XCSwiftPackageProductDependency");
        assert_eq!(dep.product_name(), "Alamofire");
    }

    #[test]
    fn test_remote_swift_package_reference() {
        let package = XCRemoteSwiftPackageReference::new(
            "https://github.com/Alamofire/Alamofire.git",
            PackageRequirement::UpToNextMajorVersion("5.0.0".to_string()),
        );
        assert_eq!(package.isa(), "XCRemoteSwiftPackageReference");
        assert_eq!(package.repository_url(), "https://github.com/Alamofire/Alamofire.git");
    }
}
