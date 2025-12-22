// M5 Phase 3+4: Separator Set Formatter
// Formats lists with proper separators (comma_and, semicolon_or, etc.)

use crate::core::SeparatorSet;

impl SeparatorSet {
    /// Format a list of items with this separator set
    ///
    /// Examples:
    /// - 0 items: ""
    /// - 1 item: "red"
    /// - 2 items: "red and blue" (using secondary)
    /// - 3+ items: "red, blue and green" (using primary + secondary/tertiary)
    /// - 3+ items with tertiary: "red, blue, and green" (Oxford comma)
    pub fn format(&self, items: &[String]) -> String {
        match items.len() {
            0 => String::new(),
            1 => items[0].clone(),
            2 => {
                // Use secondary separator for exactly two items
                // Example: "red and blue"
                format!("{}{}{}", items[0], self.secondary, items[1])
            }
            _ => {
                // Use primary for all but last, then tertiary (if defined) or secondary before last
                // Example: "red, blue and green" or "red, blue, and green" (Oxford comma)
                let all_but_last = &items[0..items.len() - 1];
                let last = &items[items.len() - 1];

                let mut result = all_but_last.join(&self.primary);
                // Use tertiary if defined, otherwise fall back to secondary
                let final_sep = self.tertiary.as_ref().unwrap_or(&self.secondary);
                result.push_str(final_sep);
                result.push_str(last);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn comma_and() -> SeparatorSet {
        SeparatorSet {
            name: "comma_and".to_string(),
            primary: ", ".to_string(),
            secondary: " and ".to_string(),
            tertiary: None,
        }
    }

    fn semicolon_or() -> SeparatorSet {
        SeparatorSet {
            name: "semicolon_or".to_string(),
            primary: "; ".to_string(),
            secondary: " or ".to_string(),
            tertiary: None,
        }
    }

    fn oxford_comma() -> SeparatorSet {
        SeparatorSet {
            name: "oxford_comma".to_string(),
            primary: ", ".to_string(),
            secondary: " and ".to_string(),
            tertiary: Some(", and ".to_string()),
        }
    }

    #[test]
    fn test_format_empty() {
        let sep = comma_and();
        assert_eq!(sep.format(&[]), "");
    }

    #[test]
    fn test_format_one_item() {
        let sep = comma_and();
        assert_eq!(sep.format(&["red".to_string()]), "red");
    }

    #[test]
    fn test_format_two_items() {
        let sep = comma_and();
        assert_eq!(
            sep.format(&["red".to_string(), "blue".to_string()]),
            "red and blue"
        );
    }

    #[test]
    fn test_format_three_items() {
        let sep = comma_and();
        assert_eq!(
            sep.format(&["red".to_string(), "blue".to_string(), "green".to_string()]),
            "red, blue and green"
        );
    }

    #[test]
    fn test_format_four_items() {
        let sep = comma_and();
        assert_eq!(
            sep.format(&[
                "red".to_string(),
                "blue".to_string(),
                "green".to_string(),
                "yellow".to_string()
            ]),
            "red, blue, green and yellow"
        );
    }

    #[test]
    fn test_format_or_separator() {
        let sep = semicolon_or();
        assert_eq!(
            sep.format(&["a".to_string(), "b".to_string(), "c".to_string()]),
            "a; b or c"
        );
    }

    #[test]
    fn test_oxford_comma_two_items() {
        let sep = oxford_comma();
        // Two items should use secondary, not tertiary
        assert_eq!(
            sep.format(&["red".to_string(), "blue".to_string()]),
            "red and blue"
        );
    }

    #[test]
    fn test_oxford_comma_three_items() {
        let sep = oxford_comma();
        // Three items should use tertiary (Oxford comma)
        assert_eq!(
            sep.format(&["red".to_string(), "blue".to_string(), "green".to_string()]),
            "red, blue, and green"
        );
    }

    #[test]
    fn test_oxford_comma_four_items() {
        let sep = oxford_comma();
        // Four items should use tertiary (Oxford comma)
        assert_eq!(
            sep.format(&[
                "red".to_string(),
                "blue".to_string(),
                "green".to_string(),
                "yellow".to_string()
            ]),
            "red, blue, green, and yellow"
        );
    }
}

