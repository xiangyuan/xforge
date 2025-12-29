//! Helpful macros for implementing PBXObject

/// Macro to implement as_any for PBXObject types
#[macro_export]
macro_rules! impl_pbx_object_basics {
    ($type:ty, $isa:expr) => {
        impl $crate::PBXObject for $type {
            fn isa(&self) -> &'static str {
                $isa
            }
            
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}
