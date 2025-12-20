// M4: Context Value Types
// Supports different data types for context storage

use serde::{Deserialize, Serialize};
use std::fmt;

/// Value types that can be stored in context
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum ContextValue {
    /// Text value (most common)
    Text(String),

    /// Numeric value
    Number(i32),

    /// Boolean value
    Boolean(bool),

    /// List of text values
    List(Vec<String>),
}

impl ContextValue {
    /// Get as text, converting if possible
    pub fn as_text(&self) -> Option<String> {
        match self {
            ContextValue::Text(s) => Some(s.clone()),
            ContextValue::Number(n) => Some(n.to_string()),
            ContextValue::Boolean(b) => Some(b.to_string()),
            ContextValue::List(list) => Some(list.join(", ")),
        }
    }

    /// Get as number, converting if possible
    #[allow(dead_code)] // Public API method
    pub fn as_number(&self) -> Option<i32> {
        match self {
            ContextValue::Number(n) => Some(*n),
            ContextValue::Text(s) => s.parse().ok(),
            ContextValue::Boolean(b) => Some(if *b { 1 } else { 0 }),
            ContextValue::List(_) => None,
        }
    }

    /// Get as boolean, converting if possible
    #[allow(dead_code)] // Public API method
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ContextValue::Boolean(b) => Some(*b),
            ContextValue::Number(n) => Some(*n != 0),
            ContextValue::Text(s) => match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Some(true),
                "false" | "no" | "0" => Some(false),
                _ => None,
            },
            ContextValue::List(_) => None,
        }
    }

    /// Get as list, converting if possible
    #[allow(dead_code)] // Public API method
    pub fn as_list(&self) -> Option<Vec<String>> {
        match self {
            ContextValue::List(list) => Some(list.clone()),
            ContextValue::Text(s) => Some(vec![s.clone()]),
            ContextValue::Number(n) => Some(vec![n.to_string()]),
            ContextValue::Boolean(b) => Some(vec![b.to_string()]),
        }
    }
}

impl fmt::Display for ContextValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContextValue::Text(s) => write!(f, "{}", s),
            ContextValue::Number(n) => write!(f, "{}", n),
            ContextValue::Boolean(b) => write!(f, "{}", b),
            ContextValue::List(list) => write!(f, "[{}]", list.join(", ")),
        }
    }
}

impl From<String> for ContextValue {
    fn from(s: String) -> Self {
        ContextValue::Text(s)
    }
}

impl From<&str> for ContextValue {
    fn from(s: &str) -> Self {
        ContextValue::Text(s.to_string())
    }
}

impl From<i32> for ContextValue {
    fn from(n: i32) -> Self {
        ContextValue::Number(n)
    }
}

impl From<bool> for ContextValue {
    fn from(b: bool) -> Self {
        ContextValue::Boolean(b)
    }
}

impl From<Vec<String>> for ContextValue {
    fn from(list: Vec<String>) -> Self {
        ContextValue::List(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_value() {
        let val = ContextValue::Text("hello".to_string());
        assert_eq!(val.as_text(), Some("hello".to_string()));
        assert_eq!(val.to_string(), "hello");
    }

    #[test]
    fn test_number_value() {
        let val = ContextValue::Number(42);
        assert_eq!(val.as_number(), Some(42));
        assert_eq!(val.as_text(), Some("42".to_string()));
        assert_eq!(val.to_string(), "42");
    }

    #[test]
    fn test_boolean_value() {
        let val = ContextValue::Boolean(true);
        assert_eq!(val.as_boolean(), Some(true));
        assert_eq!(val.as_text(), Some("true".to_string()));
        assert_eq!(val.as_number(), Some(1));
    }

    #[test]
    fn test_list_value() {
        let val = ContextValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(val.as_list(), Some(vec!["a".to_string(), "b".to_string()]));
        assert_eq!(val.as_text(), Some("a, b".to_string()));
        assert_eq!(val.to_string(), "[a, b]");
    }

    #[test]
    fn test_text_to_number() {
        let val = ContextValue::Text("123".to_string());
        assert_eq!(val.as_number(), Some(123));
    }

    #[test]
    fn test_text_to_boolean() {
        assert_eq!(ContextValue::Text("true".to_string()).as_boolean(), Some(true));
        assert_eq!(ContextValue::Text("false".to_string()).as_boolean(), Some(false));
        assert_eq!(ContextValue::Text("yes".to_string()).as_boolean(), Some(true));
        assert_eq!(ContextValue::Text("no".to_string()).as_boolean(), Some(false));
    }

    #[test]
    fn test_from_string() {
        let val: ContextValue = "test".into();
        assert_eq!(val, ContextValue::Text("test".to_string()));
    }

    #[test]
    fn test_from_i32() {
        let val: ContextValue = 42.into();
        assert_eq!(val, ContextValue::Number(42));
    }

    #[test]
    fn test_from_bool() {
        let val: ContextValue = true.into();
        assert_eq!(val, ContextValue::Boolean(true));
    }
}

