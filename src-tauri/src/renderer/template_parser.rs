// M3: Template Parser
// Parses template strings containing {reference} syntax

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unclosed reference at position {0}")]
    UnclosedReference(usize),

    #[error("Empty reference name at position {0}")]
    EmptyReference(usize),

    #[error("Invalid reference format: {0}")]
    InvalidFormat(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

/// Token in a parsed template
#[derive(Debug, Clone, PartialEq)]
pub enum TemplateToken {
    /// Static text
    Text(String),

    /// Reference to a datatype or promptsection
    /// M5 Phase 3+4: Added min/max/separator/unique parameters
    Reference {
        name: String,
        filter: Option<String>,
        min: usize,              // M5: Default 1
        max: usize,              // M5: Default 1
        separator: Option<String>, // M5: Optional separator set reference
        unique: bool,            // M5: Default false
    },
}

/// Parsed template
#[derive(Debug, Clone)]
pub struct Template {
    pub tokens: Vec<TemplateToken>,
    #[allow(dead_code)] // Kept for debugging and potential future use
    pub original: String,
}

impl Template {
    /// Parse a template string
    ///
    /// Supports:
    /// - Static text: "Hello world"
    /// - References: "{color}", "{namespace:datatype}"
    /// - Escaped braces: "{{" becomes "{", "}}" becomes "}"
    ///
    /// M3 Scope: Simple {reference} only, no parameters
    /// M5 will add: {ref?min=1,max=3&sep=comma}
    pub fn parse(template: &str) -> Result<Self> {
        let mut tokens = Vec::new();
        let mut current_text = String::new();
        let mut chars = template.char_indices().peekable();

        while let Some((pos, ch)) = chars.next() {
            match ch {
                '{' => {
                    // Check for escaped brace {{
                    if let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch == '{' {
                            chars.next(); // consume second {
                            current_text.push('{');
                            continue;
                        }
                    }

                    // Save accumulated text
                    if !current_text.is_empty() {
                        tokens.push(TemplateToken::Text(current_text.clone()));
                        current_text.clear();
                    }

                    // Parse reference name and parameters
                    let mut ref_text = String::new();
                    let mut filter_expr = None;
                    let mut found_close = false;
                    let mut in_filter = false;
                    let mut filter_text = String::new();
                    let mut brace_depth = 0;

                    while let Some((_, ch)) = chars.next() {
                        if ch == '}' {
                            if in_filter && brace_depth > 0 {
                                // Closing brace inside filter expression
                                brace_depth -= 1;
                                filter_text.push(ch);
                            } else if in_filter && brace_depth == 0 {
                                // End of filter expression
                                filter_expr = Some(filter_text.trim().to_string());
                                filter_text.clear();
                                in_filter = false;
                            } else {
                                // End of reference
                                found_close = true;
                                break;
                            }
                        } else if ch == '#' && !in_filter {
                            // Start of filter expression
                            // Check if next char is '{'
                            if let Some(&(_, next_ch)) = chars.peek() {
                                if next_ch == '{' {
                                    chars.next(); // consume '{'
                                    in_filter = true;
                                    brace_depth = 0;
                                } else {
                                    // Not a filter, treat # as part of ref_text
                                    ref_text.push(ch);
                                }
                            } else {
                                ref_text.push(ch);
                            }
                        } else if in_filter {
                            if ch == '{' {
                                brace_depth += 1;
                            }
                            filter_text.push(ch);
                        } else {
                            ref_text.push(ch);
                        }
                    }

                    if !found_close {
                        return Err(ParseError::UnclosedReference(pos));
                    }

                    let ref_text = ref_text.trim();
                    if ref_text.is_empty() {
                        return Err(ParseError::EmptyReference(pos));
                    }

                    // M5 Phase 3+4: Parse name and parameters
                    // Format: name?min=2,max=3&sep=comma_and&unique=true
                    let (name, min, max, separator, unique) = Self::parse_reference_params(ref_text)?;

                    tokens.push(TemplateToken::Reference {
                        name,
                        filter: filter_expr,
                        min,
                        max,
                        separator,
                        unique,
                    });
                }

                '}' => {
                    // Check for escaped brace }}
                    if let Some(&(_, next_ch)) = chars.peek() {
                        if next_ch == '}' {
                            chars.next(); // consume second }
                            current_text.push('}');
                            continue;
                        }
                    }

                    // Unmatched closing brace - treat as literal for now
                    // (could be an error, but being lenient)
                    current_text.push(ch);
                }

                _ => {
                    current_text.push(ch);
                }
            }
        }

        // Add remaining text
        if !current_text.is_empty() {
            tokens.push(TemplateToken::Text(current_text));
        }

        Ok(Template {
            tokens,
            original: template.to_string(),
        })
    }

    /// Get list of all reference names in this template
    /// Extract all reference names from the template
    #[allow(dead_code)] // Public API method for future use
    pub fn get_references(&self) -> Vec<String> {
        self.tokens
            .iter()
            .filter_map(|token| {
                if let TemplateToken::Reference { name, .. } = token {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// M5 Phase 3+4: Parse reference parameters
    /// Format: name?min=2,max=3&sep=comma_and&unique=true
    /// Returns: (name, min, max, separator, unique)
    fn parse_reference_params(ref_text: &str) -> Result<(String, usize, usize, Option<String>, bool)> {
        // Split on '?' to separate name from parameters
        let parts: Vec<&str> = ref_text.splitn(2, '?').collect();
        let name = parts[0].to_string();

        // Default values
        let mut min = 1;
        let mut max = 1;
        let mut separator = None;
        let mut unique = false;

        // Parse parameters if present
        if parts.len() > 1 {
            let params_text = parts[1];

            // Split on '&' for multiple parameter groups
            // Example: "min=2,max=3&sep=comma_and&unique=true"
            for param_group in params_text.split('&') {
                // Check if it's a key=value pair or a boolean flag
                if param_group.contains('=') {
                    let kv: Vec<&str> = param_group.splitn(2, '=').collect();
                    if kv.len() == 2 {
                        let key = kv[0].trim();
                        let value = kv[1].trim();

                        match key {
                            "min" => {
                                min = value.parse::<usize>()
                                    .map_err(|_| ParseError::InvalidFormat(
                                        format!("Invalid min value: {}", value)
                                    ))?;
                            }
                            "max" => {
                                max = value.parse::<usize>()
                                    .map_err(|_| ParseError::InvalidFormat(
                                        format!("Invalid max value: {}", value)
                                    ))?;
                            }
                            "sep" => {
                                separator = Some(value.to_string());
                            }
                            "unique" => {
                                unique = value.parse::<bool>()
                                    .map_err(|_| ParseError::InvalidFormat(
                                        format!("Invalid unique value: {}", value)
                                    ))?;
                            }
                            _ => {
                                // Unknown parameter - ignore for forward compatibility
                            }
                        }
                    }
                } else {
                    // Boolean flag without value (e.g., "unique")
                    let flag = param_group.trim();
                    if flag == "unique" {
                        unique = true;
                    }
                }
            }

            // Validate min <= max
            if min > max {
                return Err(ParseError::InvalidFormat(
                    format!("min ({}) must be <= max ({})", min, max)
                ));
            }
        }

        Ok((name, min, max, separator, unique))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_text() {
        let template = Template::parse("Hello world").unwrap();
        assert_eq!(template.tokens.len(), 1);
        assert_eq!(template.tokens[0], TemplateToken::Text("Hello world".to_string()));
    }

    #[test]
    fn test_parse_simple_reference() {
        let template = Template::parse("{color}").unwrap();
        assert_eq!(template.tokens.len(), 1);
        assert_eq!(template.tokens[0], TemplateToken::Reference {
            name: "color".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
    }

    #[test]
    fn test_parse_mixed() {
        let template = Template::parse("A {color} {object}").unwrap();
        assert_eq!(template.tokens.len(), 4);
        assert_eq!(template.tokens[0], TemplateToken::Text("A ".to_string()));
        assert_eq!(template.tokens[1], TemplateToken::Reference {
            name: "color".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
        assert_eq!(template.tokens[2], TemplateToken::Text(" ".to_string()));
        assert_eq!(template.tokens[3], TemplateToken::Reference {
            name: "object".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
    }

    #[test]
    fn test_parse_with_namespace() {
        let template = Template::parse("{test:color}").unwrap();
        assert_eq!(template.tokens.len(), 1);
        assert_eq!(template.tokens[0], TemplateToken::Reference {
            name: "test:color".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
    }

    #[test]
    fn test_parse_escaped_braces() {
        let template = Template::parse("Use {{braces}} like this").unwrap();
        assert_eq!(template.tokens.len(), 1);
        assert_eq!(template.tokens[0], TemplateToken::Text("Use {braces} like this".to_string()));
    }

    #[test]
    fn test_parse_unclosed_reference() {
        let result = Template::parse("Hello {color");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::UnclosedReference(_)));
    }

    #[test]
    fn test_parse_empty_reference() {
        let result = Template::parse("Hello {}");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::EmptyReference(_)));
    }

    #[test]
    fn test_parse_whitespace_in_reference() {
        let template = Template::parse("{ color }").unwrap();
        // Should trim whitespace
        assert_eq!(template.tokens[0], TemplateToken::Reference {
            name: "color".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
    }

    #[test]
    fn test_get_references() {
        let template = Template::parse("A {color} {object} is here").unwrap();
        let refs = template.get_references();
        assert_eq!(refs.len(), 2);
        assert_eq!(refs[0], "color");
        assert_eq!(refs[1], "object");
    }

    #[test]
    fn test_get_references_empty() {
        let template = Template::parse("No references here").unwrap();
        let refs = template.get_references();
        assert_eq!(refs.len(), 0);
    }

    #[test]
    fn test_consecutive_references() {
        let template = Template::parse("{a}{b}{c}").unwrap();
        assert_eq!(template.tokens.len(), 3);
        let refs = template.get_references();
        assert_eq!(refs, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_tag_filter() {
        let template = Template::parse("{animal#{tags.can_fly}}").unwrap();
        assert_eq!(template.tokens.len(), 1);
        assert_eq!(template.tokens[0], TemplateToken::Reference {
            name: "animal".to_string(),
            filter: Some("tags.can_fly".to_string()),
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
    }

    #[test]
    fn test_parse_tag_filter_complex() {
        let template = Template::parse("{activity#{tags.requires_can_swim implies ref:animal.tags.can_swim}}").unwrap();
        assert_eq!(template.tokens.len(), 1);
        if let TemplateToken::Reference { name, filter, .. } = &template.tokens[0] {
            assert_eq!(name, "activity");
            assert!(filter.is_some());
            assert_eq!(filter.as_ref().unwrap(), "tags.requires_can_swim implies ref:animal.tags.can_swim");
        } else {
            panic!("Expected Reference token");
        }
    }

    #[test]
    fn test_parse_mixed_with_filter() {
        let template = Template::parse("A {color} {animal#{tags.can_fly}} flies").unwrap();
        assert_eq!(template.tokens.len(), 5);
        assert_eq!(template.tokens[0], TemplateToken::Text("A ".to_string()));
        assert_eq!(template.tokens[1], TemplateToken::Reference {
            name: "color".to_string(),
            filter: None,
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
        assert_eq!(template.tokens[2], TemplateToken::Text(" ".to_string()));
        assert_eq!(template.tokens[3], TemplateToken::Reference {
            name: "animal".to_string(),
            filter: Some("tags.can_fly".to_string()),
            min: 1,
            max: 1,
            separator: None,
            unique: false,
        });
        assert_eq!(template.tokens[4], TemplateToken::Text(" flies".to_string()));
    }

    #[test]
    fn test_parse_tag_filter_nested_braces() {
        let template = Template::parse("{animal#{tags.mood in {peaceful, calm}}}").unwrap();
        assert_eq!(template.tokens.len(), 1);
        if let TemplateToken::Reference { name, filter, .. } = &template.tokens[0] {
            assert_eq!(name, "animal");
            assert_eq!(filter.as_ref().unwrap(), "tags.mood in {peaceful, calm}");
        } else {
            panic!("Expected Reference token");
        }
    }
}

