//! xforge-objects - PBX object definitions

pub mod pbx_project;
pub mod pbx_target;
pub mod pbx_file_reference;
pub mod pbx_file_system_synchronized;
pub mod pbx_group;
pub mod pbx_build_configuration;
pub mod pbx_build_phase;
pub mod pbx_container_item_proxy;
pub mod pbx_target_dependency;
pub mod pbx_variant_group;
pub mod pbx_reference_proxy;
pub mod pbx_aggregate_target;
pub mod pbx_legacy_target;
pub mod xc_swift_package;
pub mod xc_version_group;
pub mod serialization;
pub mod deserialization;

pub use pbx_project::{PBXProject, ProjectReference};
pub use pbx_target::PBXNativeTarget;
pub use pbx_file_reference::PBXFileReference;
pub use pbx_file_system_synchronized::{
    PBXFileSystemSynchronizedBuildFileExceptionSet,
    PBXFileSystemSynchronizedRootGroup,
};
pub use pbx_group::PBXGroup;
pub use pbx_build_configuration::{
    XCBuildConfiguration, 
    XCConfigurationList,
};
pub use pbx_build_phase::{
    PBXSourcesBuildPhase,
    PBXFrameworksBuildPhase,
    PBXResourcesBuildPhase,
    PBXShellScriptBuildPhase,
    PBXCopyFilesBuildPhase,
    PBXHeadersBuildPhase,
    PBXBuildFile,
};
pub use pbx_container_item_proxy::PBXContainerItemProxy;
pub use pbx_target_dependency::PBXTargetDependency;
pub use pbx_variant_group::PBXVariantGroup;
pub use pbx_reference_proxy::PBXReferenceProxy;
pub use pbx_aggregate_target::PBXAggregateTarget;
pub use pbx_legacy_target::PBXLegacyTarget;
pub use xc_swift_package::{
    XCSwiftPackageProductDependency,
    XCRemoteSwiftPackageReference,
    PackageRequirement,
};
pub use xc_version_group::XCVersionGroup;
pub use serialization::serialize_registry;
pub use deserialization::deserialize_registry;
#[cfg(test)]
mod tests {
    use super::*;
    use xforge_core::PBXObject;

    #[test]
    fn test_all_objects_have_isa() {
        let project = PBXProject::new("Test");
        assert_eq!(project.isa(), "PBXProject");
        
        let target = PBXNativeTarget::new("Test");
        assert_eq!(target.isa(), "PBXNativeTarget");
        
        let file = PBXFileReference::new("test.swift");
        assert_eq!(file.isa(), "PBXFileReference");
        
        let group = PBXGroup::new("Sources");
        assert_eq!(group.isa(), "PBXGroup");
        
        let config = XCBuildConfiguration::new("Debug");
        assert_eq!(config.isa(), "XCBuildConfiguration");
        
        let sources = PBXSourcesBuildPhase::new();
        assert_eq!(sources.isa(), "PBXSourcesBuildPhase");
        
        let frameworks = PBXFrameworksBuildPhase::new();
        assert_eq!(frameworks.isa(), "PBXFrameworksBuildPhase");
        
        let resources = PBXResourcesBuildPhase::new();
        assert_eq!(resources.isa(), "PBXResourcesBuildPhase");
        
        use xforge_core::ObjectId;
        use crate::xc_swift_package::PackageRequirement;
        
        let container_id = ObjectId::generate();
        let proxy = PBXContainerItemProxy::new(container_id, 1);
        assert_eq!(proxy.isa(), "PBXContainerItemProxy");
        
        let dependency = PBXTargetDependency::new();
        assert_eq!(dependency.isa(), "PBXTargetDependency");
        
        let variant_group = PBXVariantGroup::new("Main.storyboard");
        assert_eq!(variant_group.isa(), "PBXVariantGroup");
        
        let remote_ref_id = ObjectId::generate();
        let ref_proxy = PBXReferenceProxy::new("libFoo.a", "compiled.mach-o.dylib", remote_ref_id);
        assert_eq!(ref_proxy.isa(), "PBXReferenceProxy");
        
        let package_id = ObjectId::generate();
        let swift_product = XCSwiftPackageProductDependency::new(package_id, "Alamofire");
        assert_eq!(swift_product.isa(), "XCSwiftPackageProductDependency");
        
        let swift_ref = XCRemoteSwiftPackageReference::new(
            "https://github.com/Alamofire/Alamofire",
            PackageRequirement::UpToNextMajorVersion("5.0.0".to_string())
        );
        assert_eq!(swift_ref.isa(), "XCRemoteSwiftPackageReference");
        
        let aggregate_target = PBXAggregateTarget::new("BuildAll");
        assert_eq!(aggregate_target.isa(), "PBXAggregateTarget");
        
        let legacy_target = PBXLegacyTarget::new("ExternalBuild", "/usr/bin/make");
        assert_eq!(legacy_target.isa(), "PBXLegacyTarget");
        
        let version_group = XCVersionGroup::new("Model.xcdatamodeld");
        assert_eq!(version_group.isa(), "XCVersionGroup");
    }
}
