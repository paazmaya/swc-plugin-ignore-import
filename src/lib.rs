use regex::Regex;
use serde_json::Value;
use swc_core::ecma::{
    ast::{ModuleItem, Program},
    transforms::testing::test_inline,
    visit::{visit_mut_pass, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

/// A visitor that removes imports matching a specific pattern.
pub struct TransformVisitor {
    pattern: Regex,
}

impl TransformVisitor {
    // Constructor for TransformVisitor, initializes with a regex pattern
    pub fn new(pattern: Regex) -> Self {
        Self { pattern }
    }
}

impl VisitMut for TransformVisitor {
    // Implementation of the visit_mut_module_items method to filter module items
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        // Mutate the items in place
        items.retain(|item| {
            match item {
                ModuleItem::ModuleDecl(decl) => {
                    // Remove import declarations matching the pattern
                    !matches!(decl, swc_core::ecma::ast::ModuleDecl::Import(import) if self.pattern.is_match(&import.src.value))
                }
                _ => true,
            }
        });

        // Visit children nodes if necessary
        items.visit_mut_children_with(self);
    }
}

fn parse_options(options_str: &str) -> Regex {
    let options: Value = serde_json::from_str(options_str).expect("Failed to parse options JSON");

    let pattern_str = options
        .get("pattern")
        .expect("Pattern not found in options")
        .as_str()
        .expect("Pattern is not a string");

    Regex::new(pattern_str).expect("Invalid regex pattern")
}

/// The main transformation function used by SWC.
#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    metadata: TransformPluginProgramMetadata,
) -> Program {
    // Get the options passed for the plugin as a JSON string
    let pattern = parse_options(
        &metadata
            .get_transform_plugin_config()
            .expect("Failed to get plugin config"),
    );

    program.visit_mut_with(&mut TransformVisitor::new(pattern));
    program
}

// Unit tests for the plugin transformation behavior.
test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("some.+name").unwrap())),
    remove_matching_imports,
    // Input code
    r#"import "some-module-name"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("another-pattern").unwrap()
    )),
    remove_another_matching_imports,
    // Input code
    r#"import "another-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new(".*-pattern").unwrap())),
    remove_wildcard_matching_imports,
    // Input code
    r#"import "some-pattern"; import "another-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("^exact-pattern$").unwrap()
    )),
    remove_exact_matching_imports,
    // Input code
    r#"import "exact-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("lodash").unwrap())),
    remove_lodash_imports,
    // Input code
    r#"import _ from "lodash"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("react").unwrap())),
    remove_react_imports,
    // Input code
    r#"import React from "react"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("@eslint/js").unwrap())),
    remove_express_imports,
    // Input code
    r#"import express from "express"; import "@eslint/js";"#,
    // Expected output
    r#"import express from "express";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("moment").unwrap())),
    remove_moment_imports,
    // Input code
    r#"import moment from "moment"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("ignore-me").unwrap())),
    remove_ignore_me_imports,
    // Input code
    r#"import "ignore-me"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("ignore-.*").unwrap())),
    remove_ignore_wildcard_imports,
    // Input code
    r#"import "ignore-this"; import "ignore-that"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("ignore-this|ignore-that").unwrap()
    )),
    remove_multiple_patterns_imports,
    // Input code
    r#"import "ignore-this"; import "ignore-that"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("^ignore$").unwrap())),
    remove_exact_ignore_imports,
    // Input code
    r#"import "ignore"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("ignore").unwrap())),
    remove_substring_ignore_imports,
    // Input code
    r#"import "ignore"; import "ignore-this"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("some.+name").unwrap())),
    test_process_transform,
    // Input code
    r#"import "some-module-name"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("another-pattern").unwrap()
    )),
    test_process_transform_another_pattern,
    // Input code
    r#"import "another-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new(".*-pattern").unwrap())),
    test_process_transform_wildcard_pattern,
    // Input code
    r#"import "some-pattern"; import "another-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("^exact-pattern$").unwrap()
    )),
    test_process_transform_exact_pattern,
    // Input code
    r#"import "exact-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(Regex::new("ignore").unwrap())),
    test_process_transform_substring_pattern,
    // Input code
    r#"import "ignore"; import "ignore-this"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_options_valid_pattern() {
        let options_str = r#"{"pattern": "some.+name"}"#;
        let regex = parse_options(options_str);
        assert!(regex.is_match("some-module-name"));
        assert!(!regex.is_match("another-module-name"));
    }

    #[test]
    #[should_panic(expected = "Failed to parse options JSON")]
    fn test_parse_options_invalid_json() {
        let options_str = r#"{"pattern": "some.+name""#; // Missing closing brace
        parse_options(options_str);
    }

    #[test]
    #[should_panic(expected = "Pattern not found in options")]
    fn test_parse_options_missing_pattern() {
        let options_str = r#"{}"#;
        parse_options(options_str);
    }

    #[test]
    #[should_panic(expected = "Pattern is not a string")]
    fn test_parse_options_pattern_not_string() {
        let options_str = r#"{"pattern": 123}"#;
        parse_options(options_str);
    }

    #[test]
    #[should_panic(expected = "Invalid regex pattern")]
    fn test_parse_options_invalid_regex() {
        let options_str = r#"{"pattern": "["}"#; // Invalid regex
        parse_options(options_str);
    }
}
