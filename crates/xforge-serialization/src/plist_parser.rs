//! ASCII Plist parser for Xcode project files

use indexmap::IndexMap;
use crate::plist_writer::PlistValue;

/// Parser for ASCII Plist format
pub struct PlistParser {
    input: Vec<char>,
    pos: usize,
}

impl PlistParser {
    /// Create a new parser
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }
    
    /// Parse the input
    pub fn parse(&mut self) -> Result<PlistValue, String> {
        self.skip_whitespace();
        self.parse_value()
    }
    
    fn parse_value(&mut self) -> Result<PlistValue, String> {
        self.skip_whitespace();
        
        if self.pos >= self.input.len() {
            return Err("Unexpected end of input".to_string());
        }
        
        match self.input[self.pos] {
            '{' => self.parse_dictionary(),
            '(' => self.parse_array(),
            '"' => self.parse_quoted_string(),
            '<' => self.parse_data(),
            _ => self.parse_unquoted_value(),
        }
    }
    
    fn parse_dictionary(&mut self) -> Result<PlistValue, String> {
        self.expect_char('{')?;
        self.skip_whitespace();
        
        let mut dict = IndexMap::new();
        
        while self.pos < self.input.len() && self.input[self.pos] != '}' {
            self.skip_whitespace();
            if self.input[self.pos] == '}' {
                break;
            }
            
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.expect_char('=')?;
            self.skip_whitespace();
            let value = self.parse_value()?;
            self.skip_whitespace();
            self.expect_char(';')?;
            self.skip_whitespace();
            
            dict.insert(key, value);
        }
        
        self.expect_char('}')?;
        Ok(PlistValue::Dictionary(dict))
    }
    
    fn parse_array(&mut self) -> Result<PlistValue, String> {
        self.expect_char('(')?;
        self.skip_whitespace();
        
        let mut arr = Vec::new();
        
        while self.pos < self.input.len() && self.input[self.pos] != ')' {
            self.skip_whitespace();
            if self.input[self.pos] == ')' {
                break;
            }
            
            let value = self.parse_value()?;
            arr.push(value);
            
            self.skip_whitespace();
            if self.pos < self.input.len() && self.input[self.pos] == ',' {
                self.pos += 1;
            }
            self.skip_whitespace();
        }
        
        self.expect_char(')')?;
        Ok(PlistValue::Array(arr))
    }
    
    fn parse_data(&mut self) -> Result<PlistValue, String> {
        self.expect_char('<')?;
        let mut bytes = Vec::new();
        
        while self.pos < self.input.len() {
            self.skip_whitespace();
            if self.input[self.pos] == '>' {
                self.pos += 1;
                return Ok(PlistValue::Data(bytes));
            }
            
            // Parse two hex digits
            if self.pos + 1 >= self.input.len() {
                return Err("Incomplete hex byte in data".to_string());
            }
            
            let hex_str: String = self.input[self.pos..self.pos+2].iter().collect();
            match u8::from_str_radix(&hex_str, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => return Err(format!("Invalid hex byte: {}", hex_str)),
            }
            self.pos += 2;
        }
        
        Err("Unterminated data".to_string())
    }
    
    fn parse_string(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        if self.pos < self.input.len() && self.input[self.pos] == '"' {
            self.parse_quoted_string().map(|v| {
                if let PlistValue::String(s) = v {
                    s
                } else {
                    unreachable!()
                }
            })
        } else {
            self.parse_unquoted_string()
        }
    }
    
    fn parse_quoted_string(&mut self) -> Result<PlistValue, String> {
        self.expect_char('"')?;
        let mut result = String::new();
        
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            
            if c == '\\' {
                self.pos += 1;
                if self.pos >= self.input.len() {
                    return Err("Unexpected end in string escape".to_string());
                }
                let escaped = match self.input[self.pos] {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    c => c,
                };
                result.push(escaped);
                self.pos += 1;
            } else if c == '"' {
                self.pos += 1;
                return Ok(PlistValue::String(result));
            } else {
                result.push(c);
                self.pos += 1;
            }
        }
        
        Err("Unterminated string".to_string())
    }
    
    fn parse_unquoted_string(&mut self) -> Result<String, String> {
        let mut result = String::new();
        
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            
            if c.is_whitespace() || c == '=' || c == ';' || c == ',' || c == '}' || c == ')' {
                break;
            }
            
            result.push(c);
            self.pos += 1;
        }
        
        if result.is_empty() {
            Err("Empty unquoted string".to_string())
        } else {
            Ok(result)
        }
    }
    
    fn parse_unquoted_value(&mut self) -> Result<PlistValue, String> {
        let s = self.parse_unquoted_string()?;
        
        // Try to parse as boolean
        if s == "YES" {
            return Ok(PlistValue::Boolean(true));
        } else if s == "NO" {
            return Ok(PlistValue::Boolean(false));
        }
        
        // Try to parse as integer
        if let Ok(i) = s.parse::<i64>() {
            return Ok(PlistValue::Integer(i));
        }
        
        // Try to parse as real (float)
        if let Ok(f) = s.parse::<f64>() {
            return Ok(PlistValue::Real(f));
        }
        
        // Otherwise, treat as string
        Ok(PlistValue::String(s))
    }
    
    fn expect_char(&mut self, expected: char) -> Result<(), String> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Err(format!("Expected '{}' but reached end of input", expected));
        }
        if self.input[self.pos] != expected {
            return Err(format!("Expected '{}' but found '{}'", expected, self.input[self.pos]));
        }
        self.pos += 1;
        Ok(())
    }
    
    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            if c.is_whitespace() {
                self.pos += 1;
            } else if self.pos + 1 < self.input.len() && c == '/' {
                if self.input[self.pos + 1] == '/' {
                    // Single-line comment
                    self.pos += 2;
                    while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                        self.pos += 1;
                    }
                } else if self.input[self.pos + 1] == '*' {
                    // Multi-line comment
                    self.pos += 2;
                    while self.pos + 1 < self.input.len() {
                        if self.input[self.pos] == '*' && self.input[self.pos + 1] == '/' {
                            self.pos += 2;
                            break;
                        }
                        self.pos += 1;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_string() {
        let mut parser = PlistParser::new("test");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::String("test".to_string()));
    }

    #[test]
    fn test_parse_quoted_string() {
        let mut parser = PlistParser::new("\"test file.txt\"");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::String("test file.txt".to_string()));
    }
    
    #[test]
    fn test_parse_integer() {
        let mut parser = PlistParser::new("42");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::Integer(42));
    }
    
    #[test]
    fn test_parse_real() {
        let mut parser = PlistParser::new("3.14");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::Real(3.14));
    }
    
    #[test]
    fn test_parse_boolean_yes() {
        let mut parser = PlistParser::new("YES");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::Boolean(true));
    }
    
    #[test]
    fn test_parse_boolean_no() {
        let mut parser = PlistParser::new("NO");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::Boolean(false));
    }
    
    #[test]
    fn test_parse_data() {
        let mut parser = PlistParser::new("<DEADBEEF>");
        let result = parser.parse().unwrap();
        assert_eq!(result, PlistValue::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    }

    #[test]
    fn test_parse_array() {
        let mut parser = PlistParser::new("(a, b, c)");
        let result = parser.parse().unwrap();
        if let PlistValue::Array(arr) = result {
            assert_eq!(arr.len(), 3);
        } else {
            panic!("Expected array");
        }
    }
    
    #[test]
    fn test_parse_array_with_mixed_types() {
        let mut parser = PlistParser::new("(42, 3.14, YES, \"string\")");
        let result = parser.parse().unwrap();
        if let PlistValue::Array(arr) = result {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], PlistValue::Integer(42));
            assert_eq!(arr[1], PlistValue::Real(3.14));
            assert_eq!(arr[2], PlistValue::Boolean(true));
            assert_eq!(arr[3], PlistValue::String("string".to_string()));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_parse_dictionary() {
        let mut parser = PlistParser::new("{ key1 = value1; key2 = value2; }");
        let result = parser.parse().unwrap();
        if let PlistValue::Dictionary(dict) = result {
            assert_eq!(dict.len(), 2);
            assert_eq!(dict.get("key1").unwrap(), &PlistValue::String("value1".to_string()));
        } else {
            panic!("Expected dictionary");
        }
    }
    
    #[test]
    fn test_parse_dictionary_with_mixed_values() {
        let mut parser = PlistParser::new("{ count = 42; ratio = 3.14; enabled = YES; }");
        let result = parser.parse().unwrap();
        if let PlistValue::Dictionary(dict) = result {
            assert_eq!(dict.len(), 3);
            assert_eq!(dict.get("count").unwrap(), &PlistValue::Integer(42));
            assert_eq!(dict.get("ratio").unwrap(), &PlistValue::Real(3.14));
            assert_eq!(dict.get("enabled").unwrap(), &PlistValue::Boolean(true));
        } else {
            panic!("Expected dictionary");
        }
    }
}
