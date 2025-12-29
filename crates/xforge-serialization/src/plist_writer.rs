//! ASCII Plist writer for Xcode project files
//!
//! Xcode project files (project.pbxproj) use ASCII Plist format with specific requirements:
//! - First line must be: // !$*UTF8*$!
//! - Dictionaries use { key = value; } syntax
//! - Arrays use ( item, ) syntax
//! - Keys should be sorted alphabetically
//! - Strings with special characters must be quoted

use std::fmt::Write as FmtWrite;
use indexmap::IndexMap;

/// Value types in ASCII Plist format
#[derive(Debug, Clone, PartialEq)]
pub enum PlistValue {
    String(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Data(Vec<u8>),
    Date(String),  // ISO 8601 format
    Array(Vec<PlistValue>),
    Dictionary(IndexMap<String, PlistValue>),
}

/// Writer for ASCII Plist format
pub struct PlistWriter {
    indent_level: usize,
    indent_string: String,
}

impl PlistWriter {
    /// Create a new plist writer
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            indent_string: "\t".to_string(),
        }
    }
    
    /// Write a complete plist file (includes UTF-8 magic marker)
    pub fn write_plist(&mut self, value: &PlistValue) -> Result<String, String> {
        let mut output = String::new();
        
        // Write UTF-8 magic marker (REQUIRED by Xcode)
        writeln!(output, "// !$*UTF8*$!").map_err(|e| format!("Failed to write marker: {}", e))?;
        
        // Write root value (should be a dictionary for Xcode projects)
        self.write_value(&mut output, value)?;
        
        Ok(output)
    }
    
    /// Write a value to string (without magic marker)
    pub fn write(&mut self, value: &PlistValue) -> Result<String, String> {
        let mut output = String::new();
        self.write_value(&mut output, value)?;
        Ok(output)
    }
    
    fn write_value(&mut self, output: &mut String, value: &PlistValue) -> Result<(), String> {
        match value {
            PlistValue::String(s) => self.write_string(output, s)?,
            PlistValue::Integer(i) => write!(output, "{}", i)
                .map_err(|e| format!("Failed to write integer: {}", e))?,
            PlistValue::Real(f) => write!(output, "{}", f)
                .map_err(|e| format!("Failed to write real: {}", e))?,
            PlistValue::Boolean(b) => write!(output, "{}", if *b { "YES" } else { "NO" })
                .map_err(|e| format!("Failed to write boolean: {}", e))?,
            PlistValue::Data(d) => {
                // Write data as hex string enclosed in < >
                write!(output, "<").map_err(|e| format!("Failed to write data: {}", e))?;
                for byte in d.iter() {
                    write!(output, "{:02X}", byte).map_err(|e| format!("Failed to write data byte: {}", e))?;
                }
                write!(output, ">").map_err(|e| format!("Failed to write data end: {}", e))?;
            }
            PlistValue::Date(_) => {
                // Dates are rare in project files, write as empty string
                write!(output, "\"\"").map_err(|e| format!("Failed to write date: {}", e))?;
            }
            PlistValue::Array(arr) => self.write_array(output, arr)?,
            PlistValue::Dictionary(dict) => self.write_dictionary(output, dict)?,
        }
        Ok(())
    }
    
    fn write_string(&self, output: &mut String, s: &str) -> Result<(), String> {
        // Check if string needs quoting
        if needs_quoting(s) {
            write!(output, "\"{}\"", escape_string(s))
                .map_err(|e| format!("Failed to write string: {}", e))?;
        } else {
            write!(output, "{}", s)
                .map_err(|e| format!("Failed to write string: {}", e))?;
        }
        Ok(())
    }
    
    fn write_array(&mut self, output: &mut String, arr: &[PlistValue]) -> Result<(), String> {
        write!(output, "(").map_err(|e| format!("Failed to write array: {}", e))?;
        
        if !arr.is_empty() {
            writeln!(output).map_err(|e| format!("Failed to write array: {}", e))?;
            self.indent_level += 1;
            
            for value in arr {
                self.write_indent(output)?;
                self.write_value(output, value)?;
                writeln!(output, ",").map_err(|e| format!("Failed to write array item: {}", e))?;
            }
            
            self.indent_level -= 1;
            self.write_indent(output)?;
        }
        
        write!(output, ")").map_err(|e| format!("Failed to write array end: {}", e))?;
        Ok(())
    }
    
    fn write_dictionary(&mut self, output: &mut String, dict: &IndexMap<String, PlistValue>) -> Result<(), String> {
        writeln!(output, "{{").map_err(|e| format!("Failed to write dict: {}", e))?;
        self.indent_level += 1;
        
        // Sort keys alphabetically (Xcode requirement)
        let mut keys: Vec<&String> = dict.keys().collect();
        keys.sort();
        
        for key in keys {
            let value = dict.get(key).unwrap();
            
            self.write_indent(output)?;
            self.write_string(output, key)?;
            write!(output, " = ").map_err(|e| format!("Failed to write separator: {}", e))?;
            self.write_value(output, value)?;
            writeln!(output, ";").map_err(|e| format!("Failed to write dict item: {}", e))?;
        }
        
        self.indent_level -= 1;
        self.write_indent(output)?;
        write!(output, "}}").map_err(|e| format!("Failed to write dict end: {}", e))?;
        Ok(())
    }
    
    fn write_indent(&self, output: &mut String) -> Result<(), String> {
        for _ in 0..self.indent_level {
            write!(output, "{}", self.indent_string)
                .map_err(|e| format!("Failed to write indent: {}", e))?;
        }
        Ok(())
    }
}

impl Default for PlistWriter {
    fn default() -> Self {
        Self::new()
    }
}

fn needs_quoting(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    
    // Check if string contains special characters
    s.chars().any(|c| {
        !c.is_alphanumeric() && c != '_' && c != '.' && c != '/' && c != '-'
    })
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_plist_with_magic_marker() {
        let mut writer = PlistWriter::new();
        let mut dict = IndexMap::new();
        dict.insert("test".to_string(), PlistValue::String("value".to_string()));
        let value = PlistValue::Dictionary(dict);
        let result = writer.write_plist(&value).unwrap();
        
        // Must start with UTF-8 magic marker
        assert!(result.starts_with("// !$*UTF8*$!"));
        assert!(result.contains("test = value;"));
    }

    #[test]
    fn test_write_simple_string() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::String("test".to_string());
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_write_quoted_string() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::String("test file.txt".to_string());
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "\"test file.txt\"");
    }
    
    #[test]
    fn test_write_integer() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Integer(42);
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "42");
    }
    
    #[test]
    fn test_write_real() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Real(3.14);
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "3.14");
    }
    
    #[test]
    fn test_write_boolean() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Boolean(true);
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "YES");
        
        let value = PlistValue::Boolean(false);
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "NO");
    }
    
    #[test]
    fn test_write_data() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "<DEADBEEF>");
    }
    
    #[test]
    fn test_write_date() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Date("2025-12-29".to_string());
        let result = writer.write(&value).unwrap();
        assert_eq!(result, "\"\"");
    }

    #[test]
    fn test_write_array() {
        let mut writer = PlistWriter::new();
        let value = PlistValue::Array(vec![
            PlistValue::String("a".to_string()),
            PlistValue::String("b".to_string()),
        ]);
        let result = writer.write(&value).unwrap();
        assert!(result.contains("(\n"));
        assert!(result.contains("a,"));
        assert!(result.contains("b,"));
        assert!(result.contains(")"));
    }

    #[test]
    fn test_write_dictionary() {
        let mut writer = PlistWriter::new();
        let mut dict = IndexMap::new();
        dict.insert("key1".to_string(), PlistValue::String("value1".to_string()));
        dict.insert("key2".to_string(), PlistValue::String("value2".to_string()));
        let value = PlistValue::Dictionary(dict);
        let result = writer.write(&value).unwrap();
        assert!(result.contains("{\n"));
        assert!(result.contains("key1 = value1;"));
        assert!(result.contains("key2 = value2;"));
        assert!(result.contains("}"));
    }
    
    #[test]
    fn test_dictionary_keys_sorted() {
        let mut writer = PlistWriter::new();
        let mut dict = IndexMap::new();
        dict.insert("zebra".to_string(), PlistValue::String("z".to_string()));
        dict.insert("apple".to_string(), PlistValue::String("a".to_string()));
        dict.insert("banana".to_string(), PlistValue::String("b".to_string()));
        let value = PlistValue::Dictionary(dict);
        let result = writer.write(&value).unwrap();
        
        // Keys should appear in alphabetical order
        let apple_pos = result.find("apple").unwrap();
        let banana_pos = result.find("banana").unwrap();
        let zebra_pos = result.find("zebra").unwrap();
        assert!(apple_pos < banana_pos);
        assert!(banana_pos < zebra_pos);
    }
}
