//! Object ID generation for Xcode objects
//!
//! Generates 24-character uppercase hexadecimal UUIDs for Xcode project objects.
//! Format: TIMESTAMP(48bit) + COUNTER(24bit) + PROCESS_ID(24bit)

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

static COUNTER: Lazy<AtomicU32> = Lazy::new(|| AtomicU32::new(0));
static PROCESS_ID: Lazy<u32> = Lazy::new(|| {
    let pid = std::process::id();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    ((pid as u64).wrapping_mul(nanos as u64) % 0xFFFFFF) as u32
});

/// Unique identifier for Xcode project objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId {
    timestamp: u64,
    counter: u32,
    process_id: u32,
}

impl ObjectId {
    /// Generate a new unique ObjectId
    ///
    /// # Examples
    ///
    /// \`\`\`
    /// use xforge_core::ObjectId;
    ///
    /// let id1 = ObjectId::generate();
    /// let id2 = ObjectId::generate();
    /// assert_ne!(id1, id2);
    /// \`\`\`
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        let counter = COUNTER.fetch_add(1, Ordering::SeqCst);
        
        Self {
            timestamp: timestamp & 0xFFFFFFFFFFFF,
            counter: counter & 0xFFFFFF,
            process_id: *PROCESS_ID & 0xFFFFFF,
        }
    }

    /// Convert to Xcode UUID string format (24 uppercase hex characters)
    ///
    /// # Examples
    ///
    /// \`\`\`
    /// use xforge_core::ObjectId;
    ///
    /// let id = ObjectId::generate();
    /// let uuid = id.to_uuid_string();
    /// assert_eq!(uuid.len(), 24);
    /// assert!(uuid.chars().all(|c| c.is_ascii_hexdigit() && !c.is_lowercase()));
    /// \`\`\`
    pub fn to_uuid_string(&self) -> String {
        format!(
            "{:012X}{:06X}{:06X}",
            self.timestamp, self.counter, self.process_id
        )
    }

    /// Create ObjectId from Xcode UUID string
    ///
    /// # Examples
    ///
    /// \`\`\`
    /// use xforge_core::ObjectId;
    ///
    /// let id = ObjectId::generate();
    /// let uuid = id.to_uuid_string();
    /// let parsed = ObjectId::from_uuid_string(&uuid).unwrap();
    /// assert_eq!(id, parsed);
    /// \`\`\`
    pub fn from_uuid_string(s: &str) -> Result<Self, String> {
        if s.len() != 24 {
            return Err(format!("UUID string must be 24 characters, got {}", s.len()));
        }

        let timestamp = u64::from_str_radix(&s[0..12], 16)
            .map_err(|e| format!("Invalid timestamp: {}", e))?;
        let counter = u32::from_str_radix(&s[12..18], 16)
            .map_err(|e| format!("Invalid counter: {}", e))?;
        let process_id = u32::from_str_radix(&s[18..24], 16)
            .map_err(|e| format!("Invalid process_id: {}", e))?;

        Ok(Self {
            timestamp,
            counter,
            process_id,
        })
    }
}

impl std::fmt::Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_uuid_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_unique() {
        let id1 = ObjectId::generate();
        let id2 = ObjectId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_uuid_string_format() {
        let id = ObjectId::generate();
        let uuid = id.to_uuid_string();
        assert_eq!(uuid.len(), 24);
        assert!(uuid.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_round_trip() {
        let id = ObjectId::generate();
        let uuid = id.to_uuid_string();
        let parsed = ObjectId::from_uuid_string(&uuid).unwrap();
        assert_eq!(id, parsed);
    }
}
