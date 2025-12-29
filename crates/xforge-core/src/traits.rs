//! Common traits for PBX objects

/// Trait for all PBX objects
pub trait PBXObject: std::any::Any + std::fmt::Debug {
    /// Get the ISA type name
    fn isa(&self) -> &'static str;
    
    /// Get the object name (if any)
    fn name(&self) -> Option<&str> {
        None
    }
    
    /// Downcast to Any for type checking
    fn as_any(&self) -> &dyn std::any::Any;
}
