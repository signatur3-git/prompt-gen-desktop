// M4: Context Store
// Scoped key-value storage for coordination between template elements

use crate::context::value::ContextValue;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Key not found in scope '{scope}': {key}")]
    KeyNotFound { scope: String, key: String },

    #[error("Invalid key format: {0}")]
    InvalidKey(String),

    #[error("Type conversion failed for key '{key}': expected {expected}, got {actual}")]
    TypeConversion {
        key: String,
        expected: String,
        actual: String,
    },
}

pub type Result<T> = std::result::Result<T, ContextError>;

/// Context store with scoped key-value storage
///
/// Scopes provide isolation for different rendering contexts:
/// - "prompt" - Values for the current prompt (most common)
/// - "global" - Values shared across multiple renders
/// - Custom scopes can be created as needed
#[derive(Debug, Clone)]
pub struct Context {
    scopes: HashMap<String, HashMap<String, ContextValue>>,
}

impl Context {
    /// Create a new empty context
    pub fn new() -> Self {
        let mut scopes = HashMap::new();

        // Initialize common scopes
        scopes.insert("prompt".to_string(), HashMap::new());
        scopes.insert("global".to_string(), HashMap::new());

        Context { scopes }
    }

    /// Set a value in the specified scope
    ///
    /// Key format: "scope:key" or just "key" (defaults to "prompt" scope)
    pub fn set<K, V>(&mut self, key: K, value: V) -> Result<()>
    where
        K: AsRef<str>,
        V: Into<ContextValue>,
    {
        let (scope, key_name) = Self::parse_key(key.as_ref())?;

        // Ensure scope exists
        if !self.scopes.contains_key(&scope) {
            self.scopes.insert(scope.clone(), HashMap::new());
        }

        // Set value
        self.scopes
            .get_mut(&scope)
            .unwrap()
            .insert(key_name, value.into());

        Ok(())
    }

    /// Get a value from the specified scope
    ///
    /// Key format: "scope:key" or just "key" (defaults to "prompt" scope)
    pub fn get<K>(&self, key: K) -> Result<&ContextValue>
    where
        K: AsRef<str>,
    {
        let (scope, key_name) = Self::parse_key(key.as_ref())?;

        self.scopes
            .get(&scope)
            .and_then(|scope_map| scope_map.get(&key_name))
            .ok_or_else(|| ContextError::KeyNotFound {
                scope: scope.clone(),
                key: key_name,
            })
    }

    /// Get a value as text, with conversion if needed
    pub fn get_text<K>(&self, key: K) -> Result<String>
    where
        K: AsRef<str>,
    {
        let value = self.get(key.as_ref())?;
        value.as_text().ok_or_else(|| ContextError::TypeConversion {
            key: key.as_ref().to_string(),
            expected: "text".to_string(),
            actual: format!("{:?}", value),
        })
    }

    /// Get a value as number, with conversion if needed
    #[allow(dead_code)] // Public API method
    pub fn get_number<K>(&self, key: K) -> Result<i32>
    where
        K: AsRef<str>,
    {
        let value = self.get(key.as_ref())?;
        value
            .as_number()
            .ok_or_else(|| ContextError::TypeConversion {
                key: key.as_ref().to_string(),
                expected: "number".to_string(),
                actual: format!("{:?}", value),
            })
    }

    /// Get a value as boolean, with conversion if needed
    #[allow(dead_code)] // Public API method
    pub fn get_boolean<K>(&self, key: K) -> Result<bool>
    where
        K: AsRef<str>,
    {
        let value = self.get(key.as_ref())?;
        value
            .as_boolean()
            .ok_or_else(|| ContextError::TypeConversion {
                key: key.as_ref().to_string(),
                expected: "boolean".to_string(),
                actual: format!("{:?}", value),
            })
    }

    /// Check if a key exists
    pub fn has<K>(&self, key: K) -> bool
    where
        K: AsRef<str>,
    {
        if let Ok((scope, key_name)) = Self::parse_key(key.as_ref()) {
            self.scopes
                .get(&scope)
                .map(|scope_map| scope_map.contains_key(&key_name))
                .unwrap_or(false)
        } else {
            false
        }
    }

    /// Remove a value from context
    #[allow(dead_code)] // Public API method
    pub fn remove<K>(&mut self, key: K) -> Option<ContextValue>
    where
        K: AsRef<str>,
    {
        if let Ok((scope, key_name)) = Self::parse_key(key.as_ref()) {
            self.scopes
                .get_mut(&scope)
                .and_then(|scope_map| scope_map.remove(&key_name))
        } else {
            None
        }
    }

    /// Clear all values in a scope
    #[allow(dead_code)] // Public API method
    pub fn clear_scope<S>(&mut self, scope: S)
    where
        S: AsRef<str>,
    {
        if let Some(scope_map) = self.scopes.get_mut(scope.as_ref()) {
            scope_map.clear();
        }
    }

    /// Get all values in a scope
    pub fn get_scope<S>(&self, scope: S) -> Option<&HashMap<String, ContextValue>>
    where
        S: AsRef<str>,
    {
        self.scopes.get(scope.as_ref())
    }

    /// Parse key into (scope, key_name)
    /// Format: "scope:key" or "key" (defaults to "prompt")
    fn parse_key(key: &str) -> Result<(String, String)> {
        if key.is_empty() {
            return Err(ContextError::InvalidKey("Key cannot be empty".to_string()));
        }

        if let Some((scope, key_name)) = key.split_once(':') {
            if scope.is_empty() || key_name.is_empty() {
                return Err(ContextError::InvalidKey(format!(
                    "Invalid key format: '{}'",
                    key
                )));
            }
            Ok((scope.to_string(), key_name.to_string()))
        } else {
            // Default to "prompt" scope
            Ok(("prompt".to_string(), key.to_string()))
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get() {
        let mut ctx = Context::new();
        ctx.set("test", "value").unwrap();

        let val = ctx.get("test").unwrap();
        assert_eq!(val.as_text(), Some("value".to_string()));
    }

    #[test]
    fn test_scoped_keys() {
        let mut ctx = Context::new();
        ctx.set("prompt:key1", "value1").unwrap();
        ctx.set("global:key1", "value2").unwrap();

        assert_eq!(ctx.get_text("prompt:key1").unwrap(), "value1");
        assert_eq!(ctx.get_text("global:key1").unwrap(), "value2");
    }

    #[test]
    fn test_default_scope() {
        let mut ctx = Context::new();
        ctx.set("key", "value").unwrap();

        // Should be in prompt scope
        assert_eq!(ctx.get_text("prompt:key").unwrap(), "value");
        assert_eq!(ctx.get_text("key").unwrap(), "value");
    }

    #[test]
    fn test_get_nonexistent() {
        let ctx = Context::new();
        let result = ctx.get("nonexistent");
        assert!(result.is_err());
    }

    #[test]
    fn test_has() {
        let mut ctx = Context::new();
        ctx.set("exists", "value").unwrap();

        assert!(ctx.has("exists"));
        assert!(!ctx.has("nonexistent"));
    }

    #[test]
    fn test_remove() {
        let mut ctx = Context::new();
        ctx.set("key", "value").unwrap();

        assert!(ctx.has("key"));

        let removed = ctx.remove("key");
        assert!(removed.is_some());
        assert!(!ctx.has("key"));
    }

    #[test]
    fn test_clear_scope() {
        let mut ctx = Context::new();
        ctx.set("prompt:key1", "value1").unwrap();
        ctx.set("prompt:key2", "value2").unwrap();
        ctx.set("global:key3", "value3").unwrap();

        ctx.clear_scope("prompt");

        assert!(!ctx.has("prompt:key1"));
        assert!(!ctx.has("prompt:key2"));
        assert!(ctx.has("global:key3"));
    }

    #[test]
    fn test_type_conversions() {
        let mut ctx = Context::new();
        ctx.set("number", 42).unwrap();
        ctx.set("bool", true).unwrap();
        ctx.set("text", "123").unwrap();

        assert_eq!(ctx.get_number("number").unwrap(), 42);
        assert_eq!(ctx.get_text("number").unwrap(), "42");

        assert_eq!(ctx.get_boolean("bool").unwrap(), true);
        assert_eq!(ctx.get_number("bool").unwrap(), 1);

        assert_eq!(ctx.get_number("text").unwrap(), 123);
    }

    #[test]
    fn test_invalid_key() {
        let mut ctx = Context::new();

        assert!(ctx.set("", "value").is_err());
        assert!(ctx.set(":key", "value").is_err());
        assert!(ctx.set("scope:", "value").is_err());
    }

    #[test]
    fn test_custom_scope() {
        let mut ctx = Context::new();
        ctx.set("custom:key", "value").unwrap();

        assert_eq!(ctx.get_text("custom:key").unwrap(), "value");
    }

    #[test]
    fn test_get_scope() {
        let mut ctx = Context::new();
        ctx.set("prompt:key1", "value1").unwrap();
        ctx.set("prompt:key2", "value2").unwrap();

        let prompt_scope = ctx.get_scope("prompt").unwrap();
        assert_eq!(prompt_scope.len(), 2);
        assert!(prompt_scope.contains_key("key1"));
        assert!(prompt_scope.contains_key("key2"));
    }
}
