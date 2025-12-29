//! PBXContainerItemProxy - Proxy for items in other projects

use xforge_core::{ObjectId, PBXObject};

/// Proxy for container items (used for project references and dependencies)
#[derive(Debug, Clone)]
pub struct PBXContainerItemProxy {
    id: ObjectId,
    pub container_portal: ObjectId,
    pub proxy_type: u32,
    pub remote_global_id_string: Option<String>,
    pub remote_info: Option<String>,
}

impl PBXContainerItemProxy {
    pub fn new(container_portal: ObjectId, proxy_type: u32) -> Self {
        Self {
            id: ObjectId::generate(),
            container_portal,
            proxy_type,
            remote_global_id_string: None,
            remote_info: None,
        }
    }
    
    pub fn with_remote_info(mut self, info: impl Into<String>) -> Self {
        self.remote_info = Some(info.into());
        self
    }
    
    pub fn with_remote_global_id(mut self, id: impl Into<String>) -> Self {
        self.remote_global_id_string = Some(id.into());
        self
    }
}

impl PBXObject for PBXContainerItemProxy {
    fn isa(&self) -> &'static str {
        "PBXContainerItemProxy"
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

}
