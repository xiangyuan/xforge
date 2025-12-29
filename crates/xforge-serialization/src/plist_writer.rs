//! ASCII Plist writer for Xcode project files

use std::fmt::Write;
use indexmap::IndexMap;

/// Value types in ASCII Plist format
#[derive(Debug, Clone, PartialEq)]
pub enum PlistValue {
    String(String),
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
    
    /// Write a value to string
    pub fn write(&mut self, value: &PlistValue) -> Result<String, String> {
        let mut output = String::new();
        self.write_value(&mut output, value)?;
        Ok(output)
    }
    
    fn write_value(&mut self, output: &mut String, value: &PlistValue) -> Result<(), String> {
        match value {
            PlistValue::String(s) => self.write_string(output, s)?,
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
        writeln!(output, "(").map_err(|e| format!("Failed to write array: {}", e))?;
        self.indent_level += 1;
        
        for value in arr {
            self.write_indent(output)?;
            self.write_value(output, value)?;
            writeln!(output, ",").map_err(|e| format!("Failed to write array item: {}", e))?;
        }
        
        self.indent_level -= 1;
        self.write_indent(output)?;
        write!(output, ")").map_err(|e| format!("Failed to write array end: {}", e))?;
        Ok(())
    }
    
    fn write_dictionary(&mut self, output: &mut String, dict: &IndexMap<String, PlistValue>) -> Result<(), String> {
        writeln!(output, "{{").map_err(|e| format!("Failed to write dict: {}", e))?;
        self.indent_level += 1;
        
        for (key, value) in dict {
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
        !c.is_alphanumeric() && c != '_' && c != '.' && c != '/'
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
}
