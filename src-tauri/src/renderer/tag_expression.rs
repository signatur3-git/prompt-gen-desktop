// M5 Phase 2: Complex Tag Expression Parser
// Supports AND (&&), OR (||), NOT (!), and comparisons (==, !=)

use serde_json::Value as JsonValue;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected end of expression")]
    UnexpectedEnd,

    #[error("Unexpected token: {0}")]
    UnexpectedToken(String),

    #[allow(dead_code)] // Reserved for future operator validation
    #[error("Invalid operator: {0}")]
    InvalidOperator(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

/// A parsed tag filter expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Logical AND: left && right
    And(Box<Expression>, Box<Expression>),

    /// Logical OR: left || right
    Or(Box<Expression>, Box<Expression>),

    /// Logical NOT: !expr
    Not(Box<Expression>),

    /// Comparison: tag == value or tag != value
    Comparison {
        tag: String,
        operator: ComparisonOp,
        value: ComparisonValue,
    },

    /// Simple tag check: tags.can_fly (checks if true)
    TagCheck(String),

    /// Reference access: ref:other.text or ref:other.tags.field
    /// Used for cross-reference filtering
    RefAccess {
        ref_name: String,
        field_path: Vec<String>,
    },

    /// List membership check: ref:other.text in tags.applies_to
    /// Checks if the value from ref is in the tag list
    InList {
        value: Box<Expression>,
        list_tag: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonOp {
    Equal,    // ==
    NotEqual, // !=
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonValue {
    String(String),
    Number(f64),
    Bool(bool),
}

/// Parser for tag filter expressions
pub struct ExpressionParser {
    tokens: Vec<String>,
    position: usize,
}

impl ExpressionParser {
    /// Parse a tag filter expression
    pub fn parse(expr: &str) -> Result<Expression> {
        let tokens = Self::tokenize(expr);
        let mut parser = ExpressionParser {
            tokens,
            position: 0,
        };
        parser.parse_or()
    }

    /// Tokenize the expression into individual tokens
    fn tokenize(expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = expr.chars().peekable();

        while let Some(ch) = chars.next() {
            match ch {
                // Whitespace - separator
                ' ' | '\t' | '\n' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }

                // Operators
                '&' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    if chars.peek() == Some(&'&') {
                        chars.next();
                        tokens.push("&&".to_string());
                    }
                }

                '|' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    if chars.peek() == Some(&'|') {
                        chars.next();
                        tokens.push("||".to_string());
                    }
                }

                '!' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push("!=".to_string());
                    } else {
                        tokens.push("!".to_string());
                    }
                }

                '=' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    if chars.peek() == Some(&'=') {
                        chars.next();
                        tokens.push("==".to_string());
                    }
                }

                // String literals
                '"' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    let mut string_val = String::new();
                    for ch in chars.by_ref() {
                        if ch == '"' {
                            break;
                        }
                        string_val.push(ch);
                    }
                    tokens.push(format!("\"{}\"", string_val));
                }

                // Parentheses
                '(' | ')' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                    tokens.push(ch.to_string());
                }

                // Everything else
                _ => {
                    current.push(ch);
                }
            }
        }

        if !current.is_empty() {
            tokens.push(current);
        }

        tokens
    }

    /// Parse OR expression (lowest precedence)
    fn parse_or(&mut self) -> Result<Expression> {
        let mut left = self.parse_and()?;

        while self.peek() == Some("||") {
            self.consume();
            let right = self.parse_and()?;
            left = Expression::Or(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Parse AND expression
    fn parse_and(&mut self) -> Result<Expression> {
        let mut left = self.parse_not()?;

        while self.peek() == Some("&&") {
            self.consume();
            let right = self.parse_not()?;
            left = Expression::And(Box::new(left), Box::new(right));
        }

        Ok(left)
    }

    /// Parse NOT expression
    fn parse_not(&mut self) -> Result<Expression> {
        if self.peek() == Some("!") {
            self.consume();
            let expr = self.parse_not()?;
            Ok(Expression::Not(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    /// Parse primary expression (comparisons, tag checks, ref access, parentheses)
    fn parse_primary(&mut self) -> Result<Expression> {
        // Parentheses
        if self.peek() == Some("(") {
            self.consume();
            let expr = self.parse_or()?;
            if self.peek() != Some(")") {
                return Err(ParseError::UnexpectedToken("Expected ')'".to_string()));
            }
            self.consume();
            return Ok(expr);
        }

        // Get the next token
        let token = self.consume().ok_or(ParseError::UnexpectedEnd)?;

        // Check if this is a ref: expression
        if token.starts_with("ref:") {
            // Parse ref:name.field.path
            let ref_part = token.strip_prefix("ref:").unwrap();
            let parts: Vec<String> = ref_part.split('.').map(|s| s.to_string()).collect();

            if parts.is_empty() {
                return Err(ParseError::UnexpectedToken("ref: requires a reference name".to_string()));
            }

            let ref_name = parts[0].clone();
            let field_path = parts[1..].to_vec();

            let ref_expr = Expression::RefAccess { ref_name, field_path };

            // Check for "in" operator
            if self.peek() == Some("in") {
                self.consume();
                // Next should be tags.something
                let tag_token = self.consume().ok_or(ParseError::UnexpectedEnd)?;
                if !tag_token.starts_with("tags.") {
                    return Err(ParseError::UnexpectedToken(
                        format!("Expected 'tags.' after 'in', got '{}'", tag_token)
                    ));
                }
                let list_tag = tag_token.strip_prefix("tags.").unwrap().to_string();

                return Ok(Expression::InList {
                    value: Box::new(ref_expr),
                    list_tag,
                });
            }

            return Ok(ref_expr);
        }

        // Tag path (tags.something)
        if !token.starts_with("tags.") {
            return Err(ParseError::UnexpectedToken(format!("Expected 'tags.' or 'ref:', got '{}'", token)));
        }

        let tag_name = token.strip_prefix("tags.").unwrap().to_string();

        // Check for comparison operator
        if let Some(op_str) = self.peek() {
            let op_str_clone = op_str.to_string(); // Clone before consuming
            if op_str_clone == "==" || op_str_clone == "!=" {
                self.consume();
                let op = match op_str_clone.as_str() {
                    "==" => ComparisonOp::Equal,
                    "!=" => ComparisonOp::NotEqual,
                    _ => unreachable!(),
                };

                // Parse value
                let value_token = self.consume().ok_or(ParseError::UnexpectedEnd)?;
                let value = Self::parse_value(&value_token)?;

                return Ok(Expression::Comparison {
                    tag: tag_name,
                    operator: op,
                    value,
                });
            }
        }

        // Simple tag check
        Ok(Expression::TagCheck(tag_name))
    }

    /// Parse a comparison value
    fn parse_value(token: &str) -> Result<ComparisonValue> {
        // String literal
        if token.starts_with('"') && token.ends_with('"') {
            let s = token[1..token.len()-1].to_string();
            return Ok(ComparisonValue::String(s));
        }

        // Boolean
        if token == "true" {
            return Ok(ComparisonValue::Bool(true));
        }
        if token == "false" {
            return Ok(ComparisonValue::Bool(false));
        }

        // Number
        if let Ok(n) = token.parse::<f64>() {
            return Ok(ComparisonValue::Number(n));
        }

        Err(ParseError::UnexpectedToken(format!("Invalid value: {}", token)))
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.position).map(|s| s.as_str())
    }

    fn consume(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

/// Evaluate an expression against a set of tags (without cross-reference support)
/// For backward compatibility - use evaluate_with_context for cross-reference filtering
#[allow(dead_code)]
pub fn evaluate(expr: &Expression, tags: &HashMap<String, JsonValue>) -> bool {
    evaluate_with_context(expr, tags, &HashMap::new())
}

/// Evaluate an expression with support for cross-reference filtering
///
/// # Arguments
/// * `expr` - The parsed expression to evaluate
/// * `tags` - The tags of the current value being filtered
/// * `selected` - Previously selected values for ref: expressions
pub fn evaluate_with_context(
    expr: &Expression,
    tags: &HashMap<String, JsonValue>,
    selected: &HashMap<String, (String, HashMap<String, JsonValue>)>
) -> bool {
    match expr {
        Expression::And(left, right) => {
            evaluate_with_context(left, tags, selected) && evaluate_with_context(right, tags, selected)
        }

        Expression::Or(left, right) => {
            evaluate_with_context(left, tags, selected) || evaluate_with_context(right, tags, selected)
        }

        Expression::Not(inner) => {
            !evaluate_with_context(inner, tags, selected)
        }

        Expression::Comparison { tag, operator, value } => {
            if let Some(tag_value) = tags.get(tag) {
                match (operator, value) {
                    (ComparisonOp::Equal, ComparisonValue::String(s)) => {
                        tag_value.as_str() == Some(s)
                    }
                    (ComparisonOp::NotEqual, ComparisonValue::String(s)) => {
                        tag_value.as_str() != Some(s)
                    }
                    (ComparisonOp::Equal, ComparisonValue::Number(n)) => {
                        tag_value.as_f64() == Some(*n)
                    }
                    (ComparisonOp::NotEqual, ComparisonValue::Number(n)) => {
                        tag_value.as_f64() != Some(*n)
                    }
                    (ComparisonOp::Equal, ComparisonValue::Bool(b)) => {
                        tag_value.as_bool() == Some(*b)
                    }
                    (ComparisonOp::NotEqual, ComparisonValue::Bool(b)) => {
                        tag_value.as_bool() != Some(*b)
                    }
                }
            } else {
                false
            }
        }

        Expression::TagCheck(tag) => {
            if let Some(tag_value) = tags.get(tag) {
                // Check if tag is truthy
                match tag_value {
                    JsonValue::Bool(b) => *b,
                    JsonValue::String(s) => !s.is_empty(),
                    JsonValue::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                    JsonValue::Null => false,
                    _ => true, // Arrays and objects are truthy
                }
            } else {
                false
            }
        }

        Expression::RefAccess { ref_name, field_path } => {
            // Look up the selected reference
            if let Some((text, ref_tags)) = selected.get(ref_name) {
                if field_path.is_empty() {
                    // Just ref:name - return the text as truthy check
                    !text.is_empty()
                } else if field_path.len() == 1 && field_path[0] == "text" {
                    // ref:name.text
                    !text.is_empty()
                } else if field_path.len() >= 2 && field_path[0] == "tags" {
                    // ref:name.tags.field
                    let tag_name = &field_path[1];
                    if let Some(tag_value) = ref_tags.get(tag_name) {
                        // Check if truthy
                        match tag_value {
                            JsonValue::Bool(b) => *b,
                            JsonValue::String(s) => !s.is_empty(),
                            JsonValue::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
                            JsonValue::Null => false,
                            _ => true,
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                // Reference not found - fail gracefully
                false
            }
        }

        Expression::InList { value, list_tag } => {
            // Evaluate the value expression to get a string
            let ref_value = match value.as_ref() {
                Expression::RefAccess { ref_name, field_path } => {
                    if let Some((text, ref_tags)) = selected.get(ref_name) {
                        if field_path.is_empty() || (field_path.len() == 1 && field_path[0] == "text") {
                            Some(text.clone())
                        } else if field_path.len() >= 2 && field_path[0] == "tags" {
                            // Get tag value as string
                            let tag_name = &field_path[1];
                            ref_tags.get(tag_name).and_then(|v| v.as_str().map(|s| s.to_string()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                _ => None
            };

            if let Some(ref_val) = ref_value {
                // Check if the tag contains a list with this value
                if let Some(tag_value) = tags.get(list_tag) {
                    match tag_value {
                        JsonValue::Array(arr) => {
                            // Check if any array element matches
                            arr.iter().any(|item| {
                                item.as_str() == Some(&ref_val)
                            })
                        }
                        JsonValue::String(s) => {
                            // Single string match
                            s == &ref_val
                        }
                        _ => false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

/// Extract all reference names used in a tag expression
///
/// Used for dependency ordering - determines which references must be selected
/// before this filter can be evaluated
///
/// # Example
/// ```
/// use rpg_lib::renderer::tag_expression::{extract_ref_dependencies, Expression};
///
/// // Example: "ref:body_part.text in tags.applies_to"
/// let expr = Expression::InList {
///     value: Box::new(Expression::RefAccess {
///         ref_name: "body_part".to_string(),
///         field_path: vec!["text".to_string()],
///     }),
///     list_tag: "applies_to".to_string(),
/// };
///
/// let deps = extract_ref_dependencies(&expr);
/// assert_eq!(deps, vec!["body_part"]);
/// ```
pub fn extract_ref_dependencies(expr: &Expression) -> Vec<String> {
    let mut deps = Vec::new();
    collect_ref_dependencies(expr, &mut deps);
    deps
}

fn collect_ref_dependencies(expr: &Expression, deps: &mut Vec<String>) {
    match expr {
        Expression::And(left, right) | Expression::Or(left, right) => {
            collect_ref_dependencies(left, deps);
            collect_ref_dependencies(right, deps);
        }
        Expression::Not(inner) => {
            collect_ref_dependencies(inner, deps);
        }
        Expression::RefAccess { ref_name, .. } => {
            if !deps.contains(ref_name) {
                deps.push(ref_name.clone());
            }
        }
        Expression::InList { value, .. } => {
            collect_ref_dependencies(value, deps);
        }
        _ => {} // Other expressions don't contain ref dependencies
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = ExpressionParser::tokenize("tags.can_fly && tags.nocturnal");
        assert_eq!(tokens, vec!["tags.can_fly", "&&", "tags.nocturnal"]);
    }

    #[test]
    fn test_parse_simple() {
        let expr = ExpressionParser::parse("tags.can_fly").unwrap();
        assert_eq!(expr, Expression::TagCheck("can_fly".to_string()));
    }

    #[test]
    fn test_parse_and() {
        let expr = ExpressionParser::parse("tags.can_fly && tags.nocturnal").unwrap();
        assert!(matches!(expr, Expression::And(_, _)));
    }

    #[test]
    fn test_parse_or() {
        let expr = ExpressionParser::parse("tags.can_fly || tags.can_swim").unwrap();
        assert!(matches!(expr, Expression::Or(_, _)));
    }

    #[test]
    fn test_parse_not() {
        let expr = ExpressionParser::parse("!tags.domesticated").unwrap();
        assert!(matches!(expr, Expression::Not(_)));
    }

    #[test]
    fn test_parse_comparison() {
        let expr = ExpressionParser::parse("tags.type == \"melee\"").unwrap();
        assert!(matches!(expr, Expression::Comparison { .. }));
    }

    #[test]
    fn test_parse_ref_access() {
        let expr = ExpressionParser::parse("ref:body_part.text").unwrap();
        assert!(matches!(expr, Expression::RefAccess { .. }));

        if let Expression::RefAccess { ref_name, field_path } = expr {
            assert_eq!(ref_name, "body_part");
            assert_eq!(field_path, vec!["text"]);
        }
    }

    #[test]
    fn test_parse_ref_in_list() {
        let expr = ExpressionParser::parse("ref:body_part.text in tags.applies_to").unwrap();
        assert!(matches!(expr, Expression::InList { .. }));

        if let Expression::InList { value, list_tag } = expr {
            assert!(matches!(value.as_ref(), Expression::RefAccess { .. }));
            assert_eq!(list_tag, "applies_to");
        }
    }

    #[test]
    fn test_evaluate_with_context() {
        use serde_json::json;

        // Set up tags for a "scarred" quality
        let mut tags = HashMap::new();
        tags.insert("applies_to".to_string(), json!(["skin", "face"]));

        // Set up selected values context - body_part is "skin"
        let mut selected = HashMap::new();
        let mut body_part_tags = HashMap::new();
        body_part_tags.insert("type".to_string(), json!("body_part"));
        selected.insert("body_part".to_string(), ("skin".to_string(), body_part_tags));

        // Parse and evaluate: ref:body_part.text in tags.applies_to
        let expr = ExpressionParser::parse("ref:body_part.text in tags.applies_to").unwrap();
        let result = evaluate_with_context(&expr, &tags, &selected);

        // Should be true because "skin" is in ["skin", "face"]
        assert!(result);

        // Now test with body_part = "beard" (not in applies_to)
        selected.insert("body_part".to_string(), ("beard".to_string(), HashMap::new()));
        let result = evaluate_with_context(&expr, &tags, &selected);

        // Should be false because "beard" is not in ["skin", "face"]
        assert!(!result);
    }

    #[test]
    fn test_evaluate_simple() {
        let mut tags = HashMap::new();
        tags.insert("can_fly".to_string(), JsonValue::Bool(true));

        let expr = Expression::TagCheck("can_fly".to_string());
        assert!(evaluate(&expr, &tags));
    }

    #[test]
    fn test_evaluate_and() {
        let mut tags = HashMap::new();
        tags.insert("can_fly".to_string(), JsonValue::Bool(true));
        tags.insert("nocturnal".to_string(), JsonValue::Bool(true));

        let expr = Expression::And(
            Box::new(Expression::TagCheck("can_fly".to_string())),
            Box::new(Expression::TagCheck("nocturnal".to_string())),
        );
        assert!(evaluate(&expr, &tags));
    }

    #[test]
    fn test_evaluate_comparison() {
        let mut tags = HashMap::new();
        tags.insert("type".to_string(), JsonValue::String("melee".to_string()));

        let expr = Expression::Comparison {
            tag: "type".to_string(),
            operator: ComparisonOp::Equal,
            value: ComparisonValue::String("melee".to_string()),
        };
        assert!(evaluate(&expr, &tags));
    }
}

