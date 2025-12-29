//! ObjectId - Collision-resistant UUID generation
//!
//! Uses a hybrid approach: timestamp (48 bits) + counter (24 bits) + process_id (24 bits)
//! to ensure uniqueness across runs, processes, and concurrent operations.

use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;


static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Process identifier (initialized once at startup)
static PROCESS_ID: Lazy<u64> = Lazy::new(|| {
   let mut hasher = DefaultHasher::new();
    std::process::id().hash(&mut hasher);
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);
    (hasher.finish() & 0xFFFFFF) as u64 // Take 24 bits
});

// Object identifier with collision resistance
///
/// Composed of three parts:
/// - **Timestamp** (48 bits): Milliseconds since UNIX epoch, ensures cross-run uniqueness
/// - **Counter** (24 bits): Monotonically increasing, ensures in-run uniqueness
/// - **Process ID** (24 bits): Hash of process ID + startup time, ensures cross-process uniqueness
///
/// Total: 96 bits = 24 hexadecimal characters (Xcode UUID format)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId {
    timestamp: u64,
    counter: u32,
    process_id: u32,
}

impl ObjectId {
    /// Generate a new ObjectId
    ///
    /// # Examples
    ///
    /// ```
    /// use xforge_core::ObjectId;
    ///
    /// let id1 = ObjectId::generate();
    /// let id2 = ObjectId::generate();
    /// assert_ne!(id1, id2);
    /// ```
    pub fn generate() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        let counter = COUNTER.fetch_add(1, Ordering::Relaxed) as u64;

        Self {
            timestamp: timestamp & 0xFFFF_FFFF_FFFF, // 48 bits
            counter: (counter & 0xFFFFFF) as u32,              // 24 bits
            process_id: (*PROCESS_ID) as u32,
        }
    }

    /// Convert to Xcode UUID string format (24 uppercase hex characters)
    ///
    /// # Examples
    ///
    /// ```
    /// use xforge_core::ObjectId;
    ///
    /// let id = ObjectId::generate();
    /// let uuid = id.to_uuid_string();
    /// assert_eq!(uuid.len(), 24);
    /// assert!(uuid.chars().all(|c| c.is_ascii_hexdigit() && c.is_uppercase()));
    /// ```
    pub fn to_uuid_string(&self) -> String {
        format!(
            "{:012X}{:06X}{:06X}",
            self.timestamp, self.counter, self.process_id
        )
    }

    /// Parse from Xcode UUID string format
    ///
    /// # Examples
    ///
    /// ```
    /// use xforge_core::ObjectId;
    ///
    /// let original = ObjectId::generate();
    /// let uuid_str = original.to_uuid_string();
    /// let parsed = ObjectId::from_uuid_string(&uuid_str).unwrap();
    /// assert_eq!(original, parsed);
    /// ```
    pub fn from_uuid_string(s: &str) -> Result<Self, String> {
        if s.len() != 24 {
            return Err(format!("UUID must be 24 characters, got {}", s.len()));
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

    /// Create a test ObjectId with a specific value (for testing only)
    #[cfg(test)]
    pub fn test_id(value: u64) -> Self {
        Self {
            timestamp: value,
            counter: 0,
            process_id: 0,
        }
    }

    /// Get the timestamp component
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Get the counter component
    pub fn counter(&self) -> u32 {
        self.counter
    }

    /// Get the process_id component
    pub fn process_id(&self) -> u32 {
        self.process_id
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
    use std::collections::HashSet;

    #[test]
    fn test_generate_unique() {
        let id1 = ObjectId::generate();
        let id2 = ObjectId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_uuid_format() {
        let id = ObjectId::generate();
        let uuid = id.to_uuid_string();

        assert_eq!(uuid.len(), 24);
        assert!(uuid.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(uuid.chars().all(|c| c.is_uppercase() || c.is_ascii_digit()));
    }

    #[test]
    fn test_roundtrip() {
        let original = ObjectId::generate();
        let uuid_str = original.to_uuid_string();
        let parsed = ObjectId::from_uuid_string(&uuid_str).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_monotonic_increase() {
        let ids: Vec<_> = (0..100).map(|_| ObjectId::generate()).collect();

        // All should be unique
        let unique: HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);

        // Counters should increase (same timestamp)
        for i in 1..ids.len() {
            assert!(ids[i].counter() > ids[i - 1].counter() || ids[i].timestamp() > ids[i - 1].timestamp());
        }
    }

    #[test]
    fn test_collision_resistance() {
        let mut set = HashSet::new();
        for _ in 0..10000 {
            let id = ObjectId::generate();
            assert!(set.insert(id), "Duplicate ObjectId generated!");
        }
    }
}