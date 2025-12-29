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
            _ => self.parse_unquoted_string(),
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
            self.parse_unquoted_string().map(|v| {
                if let PlistValue::String(s) = v {
                    s
                } else {
                    unreachable!()
                }
            })
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
    
    fn parse_unquoted_string(&mut self) -> Result<PlistValue, String> {
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
            Ok(PlistValue::String(result))
        }
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
                    self.pos += 2;
                    while self.pos < self.input.len() && self.input[self.pos] != '\n' {
                        self.pos += 1;
                    }
                } else if self.input[self.pos + 1] == '*' {
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
}
