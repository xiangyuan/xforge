//! PBXReferenceProxy - Proxy for referencing products from other projects

use xforge_core::{ObjectId, PBXObject};

/// Reference proxy for products from other projects
#[derive(Debug, Clone)]
pub struct PBXReferenceProxy {
    id: ObjectId,
    pub file_type: String,
    pub path: String,
    pub remote_ref: ObjectId,
    pub source_tree: String,
}

impl PBXReferenceProxy {
    pub fn new(path: impl Into<String>, file_type: impl Into<String>, remote_ref: ObjectId) -> Self {
        Self {
            id: ObjectId::generate(),
            file_type: file_type.into(),
            path: path.into(),
            remote_ref,
            source_tree: "BUILT_PRODUCTS_DIR".to_string(),
        }
    }
}

impl PBXObject for PBXReferenceProxy {
    fn isa(&self) -> &'static str {
        "PBXReferenceProxy"
    }
}
