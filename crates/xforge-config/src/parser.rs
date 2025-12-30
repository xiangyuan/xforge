use regex::Regex;
use crate::{ConditionalSetting, Result};

/// Parses xcconfig file content
pub struct Parser;

impl Parser {
    /// Parses a line into a setting or include directive
    pub fn parse_line(line: &str) -> Option<ParsedLine> {
        let trimmed = line.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with("//") {
            return None;
        }
        
        // Check for include directive
        let include_regex = Regex::new(r#"^#include\s+"([^"]+)"\s*$"#).unwrap();
        if let Some(caps) = include_regex.captures(trimmed) {
            return Some(ParsedLine::Include(caps[1].to_string()));
        }
        
        // Check for setting
        let setting_regex = Regex::new(r"^([A-Z_][A-Z0-9_]*)\s*(?:\[([^\]]+)\])?\s*=\s*(.*)$").unwrap();
        if let Some(caps) = setting_regex.captures(trimmed) {
            let key = caps[1].to_string();
            let condition = caps.get(2).map(|m| m.as_str().to_string());
            let value = caps[3].trim().to_string();
            
            let setting = if let Some(cond) = condition {
                ConditionalSetting::with_condition(key, value, cond)
            } else {
                ConditionalSetting::new(key, value)
            };
            
            return Some(ParsedLine::Setting(setting));
        }
        
        None
    }
    
    /// Parses complete xcconfig content
    pub fn parse(content: &str) -> Result<ParseResult> {
        let mut settings = Vec::new();
        let mut includes = Vec::new();
        
        for line in content.lines() {
            if let Some(parsed) = Self::parse_line(line) {
                match parsed {
                    ParsedLine::Setting(setting) => settings.push(setting),
                    ParsedLine::Include(path) => includes.push(path),
                }
            }
        }
        
        Ok(ParseResult { settings, includes })
    }
}

/// Parsed line type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedLine {
    Setting(ConditionalSetting),
    Include(String),
}

/// Parse result
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub settings: Vec<ConditionalSetting>,
    pub includes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_setting() {
        let line = "FRAMEWORK_SEARCH_PATHS = /path/to/frameworks";
        let parsed = Parser::parse_line(line).unwrap();
        
        match parsed {
            ParsedLine::Setting(setting) => {
                assert_eq!(setting.key, "FRAMEWORK_SEARCH_PATHS");
                assert_eq!(setting.value, "/path/to/frameworks");
                assert!(setting.condition.is_none());
            }
            _ => panic!("Expected setting"),
        }
    }

    #[test]
    fn test_parse_conditional_setting() {
        let line = "FRAMEWORK_SEARCH_PATHS[sdk=iphoneos*] = /ios/frameworks";
        let parsed = Parser::parse_line(line).unwrap();
        
        match parsed {
            ParsedLine::Setting(setting) => {
                assert_eq!(setting.key, "FRAMEWORK_SEARCH_PATHS");
                assert_eq!(setting.value, "/ios/frameworks");
                assert_eq!(setting.condition, Some("sdk=iphoneos*".to_string()));
            }
            _ => panic!("Expected setting"),
        }
    }

    #[test]
    fn test_parse_include() {
        let line = r#"#include "Base.xcconfig""#;
        let parsed = Parser::parse_line(line).unwrap();
        
        match parsed {
            ParsedLine::Include(path) => {
                assert_eq!(path, "Base.xcconfig");
            }
            _ => panic!("Expected include"),
        }
    }

    #[test]
    fn test_parse_comment() {
        let line = "// This is a comment";
        assert!(Parser::parse_line(line).is_none());
    }

    #[test]
    fn test_parse_empty_line() {
        let line = "   ";
        assert!(Parser::parse_line(line).is_none());
    }

    #[test]
    fn test_parse_complete_config() {
        let content = r#"
// Configuration file
#include "Base.xcconfig"

FRAMEWORK_SEARCH_PATHS = /common/frameworks
FRAMEWORK_SEARCH_PATHS[sdk=iphoneos*] = /ios/frameworks
ALWAYS_EMBED_SWIFT_STANDARD_LIBRARIES = YES
        "#;
        
        let result = Parser::parse(content).unwrap();
        assert_eq!(result.includes.len(), 1);
        assert_eq!(result.settings.len(), 3);
        assert_eq!(result.includes[0], "Base.xcconfig");
    }
}
