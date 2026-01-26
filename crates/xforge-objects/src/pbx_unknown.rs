//! PBXUnknownObject - Preserves unknown ISA types.

use indexmap::IndexMap;
use xforge_core::{ObjectId, PBXObject};
use xforge_serialization::PlistValue;

/// Represents an unknown PBX object type. Fields are preserved as-is.
#[derive(Debug, Clone)]
pub struct PBXUnknownObject {
    id: ObjectId,
    isa: String,
    fields: IndexMap<String, PlistValue>,
}

impl PBXUnknownObject {
    pub fn new(id: ObjectId, isa: impl Into<String>, fields: IndexMap<String, PlistValue>) -> Self {
        Self {
            id,
            isa: isa.into(),
            fields,
        }
    }

    pub fn actual_isa(&self) -> &str {
        &self.isa
    }

    pub fn fields(&self) -> &IndexMap<String, PlistValue> {
        &self.fields
    }
}

impl PBXObject for PBXUnknownObject {
    fn isa(&self) -> &'static str {
        "PBXUnknownObject"
    }

    fn id(&self) -> &ObjectId {
        &self.id
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
