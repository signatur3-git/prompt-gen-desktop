// M4: Rules Processor
// Executes coordination rules during enrichment phase

use crate::context::{Context, ContextValue};
use crate::core::models::Rule;
use crate::renderer::selector::SelectedValue;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuleError {
    #[allow(dead_code)] // Reserved for future rule evaluation
    #[error("Failed to evaluate expression '{expr}': {reason}")]
    EvaluationError { expr: String, reason: String },

    #[error("Reference not found: {0}")]
    ReferenceNotFound(String),

    #[error("Tag not found: {path}")]
    TagNotFound { path: String },

    #[error("Invalid expression format: {0}")]
    InvalidExpression(String),

    #[error("Context error: {0}")]
    ContextError(#[from] crate::context::ContextError),
}

pub type Result<T> = std::result::Result<T, RuleError>;

/// Rules processor - executes rules during Phase 2 (Enrichment)
pub struct RulesProcessor<'a> {
    context: &'a mut Context,
    selected: &'a HashMap<String, SelectedValue>,
}

impl<'a> RulesProcessor<'a> {
    /// Create a new rules processor
    pub fn new(context: &'a mut Context, selected: &'a HashMap<String, SelectedValue>) -> Self {
        RulesProcessor { context, selected }
    }

    /// Execute all rules (from HashMap)
    pub fn execute_rules(&mut self, rules: &HashMap<String, Rule>) -> Result<()> {
        // Execute rules (order not guaranteed with HashMap, but that's acceptable for now)
        for rule in rules.values() {
            self.execute_rule(rule)?;
        }
        Ok(())
    }

    /// Execute a single rule
    ///
    /// New simplified structure:
    /// - when: Field to check (e.g., "ref:creature.tags.article")
    /// - logic: Optional condition (empty = exists check)
    /// - set: Context field to write to (e.g., "context.prompt.article")
    /// - value: Value to write (e.g., "ref:creature.tags.article")
    fn execute_rule(&mut self, rule: &Rule) -> Result<()> {
        // Check if the "when" condition is met
        let _when_value = match self.evaluate_expression(&rule.when) {
            Ok(val) => val,
            Err(RuleError::ReferenceNotFound(_)) => {
                // Reference doesn't exist, skip this rule
                return Ok(());
            }
            Err(e) => return Err(e),
        };

        // If logic is specified, evaluate it
        // For now, logic is simplified - just existence check means when_value exists
        // Future: could parse expressions like "== value", "> 5", etc.
        // If when_value was successfully retrieved, the condition is met

        // Extract the key from "context.prompt.article" -> "article"
        // or "context.global.time" -> "global:time"
        let key = if rule.set.starts_with("context.prompt.") {
            rule.set
                .strip_prefix("context.prompt.")
                .unwrap()
                .to_string()
        } else if rule.set.starts_with("context.global.") {
            format!(
                "global:{}",
                rule.set.strip_prefix("context.global.").unwrap()
            )
        } else if rule.set.starts_with("context.") {
            // Generic context.section.key format
            rule.set.strip_prefix("context.").unwrap().replace(".", ":")
        } else {
            // Assume it's already in the right format
            rule.set.clone()
        };

        // Skip if value already exists (first contribution wins)
        if self.context.has(&key) {
            return Ok(());
        }

        // Evaluate the value expression
        let value = self.evaluate_expression(&rule.value)?;

        // Set in context
        self.context.set(&key, value)?;

        Ok(())
    }

    /// Evaluate an expression
    ///
    /// Supported formats:
    /// - "ref:color.tags.article" - Get tag from selected value
    /// - "ref:color.text" - Get text from selected value
    /// - Literal values: "a", "an", "true", "42"
    /// - "literal_value" - Just a string literal
    fn evaluate_expression(&self, expr: &str) -> Result<ContextValue> {
        let expr = expr.trim();

        // Check if it's a reference expression
        if let Some(ref_path) = expr.strip_prefix("ref:") {
            self.evaluate_reference(ref_path)
        } else {
            // Literal string value
            Ok(ContextValue::Text(expr.to_string()))
        }
    }

    /// Evaluate a reference expression
    ///
    /// Format: "reference_name.property" or "reference_name.tags.tag_name"
    fn evaluate_reference(&self, path: &str) -> Result<ContextValue> {
        // Split into parts: e.g., "color.tags.article" -> ["color", "tags", "article"]
        let parts: Vec<&str> = path.split('.').collect();

        if parts.is_empty() {
            return Err(RuleError::InvalidExpression(format!(
                "Empty reference path: ref:{}",
                path
            )));
        }

        // First part is the reference name
        let ref_name = parts[0];

        // Get the selected value
        let selected_value = self
            .selected
            .get(ref_name)
            .ok_or_else(|| RuleError::ReferenceNotFound(ref_name.to_string()))?;

        // If only one part, return the text
        if parts.len() == 1 {
            return Ok(ContextValue::Text(selected_value.text.clone()));
        }

        // Second part should be "text" or "tags"
        match parts[1] {
            "text" => {
                if parts.len() != 2 {
                    return Err(RuleError::InvalidExpression(format!(
                        "Invalid path after .text: ref:{}",
                        path
                    )));
                }
                Ok(ContextValue::Text(selected_value.text.clone()))
            }
            "tags" => {
                // Third part should be the tag name
                if parts.len() != 3 {
                    return Err(RuleError::InvalidExpression(format!(
                        "Invalid tags path (expected ref:name.tags.tag_name): ref:{}",
                        path
                    )));
                }

                let tag_name = parts[2];
                let tag_value =
                    selected_value
                        .tags
                        .get(tag_name)
                        .ok_or_else(|| RuleError::TagNotFound {
                            path: format!("{}.tags.{}", ref_name, tag_name),
                        })?;

                // Convert serde_json::Value to ContextValue
                self.json_to_context_value(tag_value)
            }
            _ => Err(RuleError::InvalidExpression(format!(
                "Unknown property '{}' (expected 'text' or 'tags'): ref:{}",
                parts[1], path
            ))),
        }
    }

    /// Convert serde_json::Value to ContextValue
    fn json_to_context_value(&self, value: &serde_json::Value) -> Result<ContextValue> {
        match value {
            serde_json::Value::String(s) => Ok(ContextValue::Text(s.clone())),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(ContextValue::Number(i as i32))
                } else {
                    Ok(ContextValue::Text(n.to_string()))
                }
            }
            serde_json::Value::Bool(b) => Ok(ContextValue::Boolean(*b)),
            serde_json::Value::Array(arr) => {
                let strings: Vec<String> = arr
                    .iter()
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    })
                    .collect();
                Ok(ContextValue::List(strings))
            }
            serde_json::Value::Null => Ok(ContextValue::Text("".to_string())),
            serde_json::Value::Object(_) => {
                // Convert object to JSON string
                Ok(ContextValue::Text(value.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_selected() -> HashMap<String, SelectedValue> {
        let mut selected = HashMap::new();

        let mut tags = HashMap::new();
        tags.insert("article".to_string(), json!("an"));
        tags.insert("phonetic".to_string(), json!("vowel"));
        tags.insert("count".to_string(), json!(5));

        selected.insert(
            "color".to_string(),
            SelectedValue {
                text: "orange".to_string(),
                tags,
            },
        );

        selected
    }

    #[test]
    fn test_evaluate_literal() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor.evaluate_expression("hello").unwrap();
        assert_eq!(result.as_text().unwrap(), "hello");
    }

    #[test]
    fn test_evaluate_ref_text() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor.evaluate_expression("ref:color.text").unwrap();
        assert_eq!(result.as_text().unwrap(), "orange");
    }

    #[test]
    fn test_evaluate_ref_tag() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor
            .evaluate_expression("ref:color.tags.article")
            .unwrap();
        assert_eq!(result.as_text().unwrap(), "an");
    }

    #[test]
    fn test_evaluate_ref_tag_number() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor
            .evaluate_expression("ref:color.tags.count")
            .unwrap();
        assert_eq!(result.as_number().unwrap(), 5);
    }

    #[test]
    fn test_execute_rule() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let mut processor = RulesProcessor::new(&mut ctx, &selected);

        let rule = Rule {
            when: "ref:color.tags.article".to_string(),
            logic: String::new(), // Empty = existence check
            set: "context.prompt.article".to_string(),
            value: "ref:color.tags.article".to_string(),
        };

        processor.execute_rule(&rule).unwrap();

        assert_eq!(ctx.get_text("article").unwrap(), "an");
    }

    #[test]
    fn test_execute_rules() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let mut processor = RulesProcessor::new(&mut ctx, &selected);

        let mut rules = HashMap::new();
        rules.insert(
            "rule1".to_string(),
            Rule {
                when: "ref:color.tags.article".to_string(),
                logic: String::new(),
                set: "context.prompt.article".to_string(),
                value: "ref:color.tags.article".to_string(),
            },
        );
        rules.insert(
            "rule2".to_string(),
            Rule {
                when: "ref:color.text".to_string(),
                logic: String::new(),
                set: "context.prompt.test_color".to_string(),
                value: "ref:color.text".to_string(),
            },
        );

        processor.execute_rules(&rules).unwrap();

        assert_eq!(ctx.get_text("article").unwrap(), "an");
        assert_eq!(ctx.get_text("test_color").unwrap(), "orange");
    }

    #[test]
    fn test_reference_not_found() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor.evaluate_expression("ref:nonexistent.tags.article");
        assert!(result.is_err());
    }

    #[test]
    fn test_tag_not_found() {
        let selected = create_test_selected();
        let mut ctx = Context::new();
        let processor = RulesProcessor::new(&mut ctx, &selected);

        let result = processor.evaluate_expression("ref:color.tags.nonexistent");
        assert!(result.is_err());
    }
}
