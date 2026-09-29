use regex::Regex;
use serde_json::Value;
use swc_core::ecma::{
    ast::{ModuleItem, Program},
    visit::{VisitMut, VisitMutWith},
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
                    !matches!(
                        decl,
                        swc_core::ecma::ast::ModuleDecl::Import(import)
                            if import.src.value.as_str().is_some_and(|src| self.pattern.is_match(src))
                    )
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

/// Transform a program with a given regex pattern.
/// This function is extracted for testability.
pub fn transform_program_with_pattern(mut program: Program, pattern: Regex) -> Program {
    program.visit_mut_with(&mut TransformVisitor::new(pattern));
    program
}

/// The main transformation function used by SWC.
#[plugin_transform]
pub fn process_transform(program: Program, metadata: TransformPluginProgramMetadata) -> Program {
    // Get the options passed for the plugin as a JSON string
    let pattern = parse_options(
        &metadata
            .get_transform_plugin_config()
            .expect("Failed to get plugin config"),
    );

    transform_program_with_pattern(program, pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use swc_core::common::DUMMY_SP;
    use swc_core::ecma::{
        ast::{ImportDecl, ModuleDecl, ModuleItem, Str},
        transforms::testing::test_inline,
        visit::visit_mut_pass,
    };

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

    #[test]
    fn test_transform_visitor_new() {
        let pattern = Regex::new("test-pattern").unwrap();
        let visitor = TransformVisitor::new(pattern);
        assert!(visitor.pattern.is_match("test-pattern"));
        assert!(!visitor.pattern.is_match("other-pattern"));
    }

    #[test]
    fn test_visit_mut_module_items_removes_matching_imports() {
        let pattern = Regex::new("remove-me").unwrap();
        let mut visitor = TransformVisitor::new(pattern);

        // Create a module with two imports - one that should be removed and one that should be kept
        let mut items = vec![
            create_import_module_item("remove-me"),
            create_import_module_item("keep-this"),
        ];

        // Apply the visitor
        visitor.visit_mut_module_items(&mut items);

        // We should only have one item left
        assert_eq!(items.len(), 1);

        // Check that the remaining import is the one we wanted to keep
        if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = &items[0] {
            assert_eq!(import_decl.src.value, "keep-this");
        } else {
            panic!("Expected an import declaration");
        }
    }

    #[test]
    fn test_visit_mut_module_items_keeps_non_matching_imports() {
        let pattern = Regex::new("remove-me").unwrap();
        let mut visitor = TransformVisitor::new(pattern);

        // Create a module with two non-matching imports
        let mut items = vec![
            create_import_module_item("keep-this-1"),
            create_import_module_item("keep-this-2"),
        ];

        // Original length
        let original_len = items.len();

        // Apply the visitor
        visitor.visit_mut_module_items(&mut items);

        // We should still have the same number of items
        assert_eq!(items.len(), original_len);
    }
    #[test]
    fn test_visit_mut_module_items_non_import_items() {
        let pattern = Regex::new("remove-me").unwrap();
        let mut visitor = TransformVisitor::new(pattern);

        // Create a non-import module item (we'll use an empty export declaration as a placeholder)        // Create a different non-import item to avoid complicated AST structures
        let non_import_item = ModuleItem::Stmt(swc_core::ecma::ast::Stmt::Empty(
            swc_core::ecma::ast::EmptyStmt { span: DUMMY_SP },
        ));

        let mut items = vec![non_import_item];

        // Original length
        let original_len = items.len();

        // Apply the visitor
        visitor.visit_mut_module_items(&mut items);

        // Non-import items should not be removed
        assert_eq!(items.len(), original_len);
    } // Helper function to create an import module item
    fn create_import_module_item(source: &str) -> ModuleItem {
        ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
            span: DUMMY_SP,
            specifiers: vec![],
            src: Box::new(Str {
                span: DUMMY_SP,
                value: source.into(),
                raw: None,
            }),
            type_only: false,
            with: None,
            phase: swc_core::ecma::ast::ImportPhase::Evaluation,
        }))
    }
    #[test]
    fn test_direct_transformation_with_visitor() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a simple program with an import that should be removed
        let import_stmt = create_import_module_item("test-pattern");
        let module = Module {
            span: DUMMY_SP,
            body: vec![import_stmt],
            shebang: None,
        };

        let mut program = Program::Module(module);

        // Apply transformation directly using the visitor
        let pattern = Regex::new("test-pattern").unwrap();
        program.visit_mut_with(&mut TransformVisitor::new(pattern));

        // Verify the import was removed
        if let Program::Module(result_module) = program {
            assert_eq!(
                result_module.body.len(),
                0,
                "Import should have been removed"
            );
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_direct_transformation_with_multiple_imports() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program with multiple imports, some matching and some not
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                create_import_module_item("react"),
                create_import_module_item("lodash"),
                create_import_module_item("react-dom"),
                create_import_module_item("helpers"),
            ],
            shebang: None,
        };

        let mut program = Program::Module(module);

        // Apply transformation to remove react and react-dom
        let pattern = Regex::new("^react").unwrap();
        program.visit_mut_with(&mut TransformVisitor::new(pattern));

        // Verify only the non-matching imports remain
        if let Program::Module(result_module) = program {
            assert_eq!(result_module.body.len(), 2, "Should have 2 imports left");
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = &result_module.body[0]
            {
                assert_eq!(import_decl.src.value, "lodash");
            } else {
                panic!("Expected an import declaration");
            }
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = &result_module.body[1]
            {
                assert_eq!(import_decl.src.value, "helpers");
            } else {
                panic!("Expected an import declaration");
            }
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_parse_options_complex_patterns() {
        // Test with complex regex patterns
        let options_str = r#"{"pattern": "^(react|vue|angular)"}"#;
        let regex = parse_options(options_str);
        assert!(regex.is_match("react"));
        assert!(regex.is_match("vue"));
        assert!(regex.is_match("angular"));
        assert!(!regex.is_match("jquery"));
    }

    #[test]
    fn test_empty_imports_list() {
        use swc_core::ecma::ast::{Module, Program};

        // Create an empty program
        let module = Module {
            span: DUMMY_SP,
            body: vec![],
            shebang: None,
        };

        let mut program = Program::Module(module);

        // Apply transformation
        let pattern = Regex::new("test-pattern").unwrap();
        program.visit_mut_with(&mut TransformVisitor::new(pattern));

        // Should still be empty
        if let Program::Module(result_module) = program {
            assert_eq!(result_module.body.len(), 0);
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_transformation_preserves_non_import_items() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program with mixed items (imports and other statements)
        let items = vec![
            create_import_module_item("remove-me"),
            ModuleItem::Stmt(swc_core::ecma::ast::Stmt::Empty(
                swc_core::ecma::ast::EmptyStmt { span: DUMMY_SP },
            )),
            create_import_module_item("keep-this"),
        ];

        let module = Module {
            span: DUMMY_SP,
            body: items,
            shebang: None,
        };

        let mut program = Program::Module(module);

        // Apply transformation
        let pattern = Regex::new("remove-me").unwrap();
        program.visit_mut_with(&mut TransformVisitor::new(pattern));

        // Should preserve non-import items and the kept import
        if let Program::Module(result_module) = program {
            assert_eq!(
                result_module.body.len(),
                2,
                "Should have 2 items: 1 non-import and 1 import"
            );
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_parse_options_special_regex_characters() {
        // Test with regex special characters
        let options_str = r#"{"pattern": "\\.(css|scss)$"}"#;
        let regex = parse_options(options_str);
        assert!(regex.is_match("styles.css"));
        assert!(regex.is_match("styles.scss"));
        assert!(!regex.is_match("styles.js"));
    }

    #[test]
    fn test_transform_program_with_pattern_removes_imports() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program with imports
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                create_import_module_item("test-pattern"),
                create_import_module_item("keep-this"),
            ],
            shebang: None,
        };

        let program = Program::Module(module);
        let pattern = Regex::new("test-pattern").unwrap();

        // Call the testable transformation function
        let result = transform_program_with_pattern(program, pattern);

        // Verify the import was removed
        if let Program::Module(result_module) = result {
            assert_eq!(result_module.body.len(), 1);
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = &result_module.body[0]
            {
                assert_eq!(import_decl.src.value, "keep-this");
            } else {
                panic!("Expected an import declaration");
            }
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_transform_program_with_pattern_complex_regex() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program with imports
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                create_import_module_item("react"),
                create_import_module_item("react-dom"),
                create_import_module_item("lodash"),
            ],
            shebang: None,
        };

        let program = Program::Module(module);
        let pattern = Regex::new("^react").unwrap();

        // Call the testable transformation function
        let result = transform_program_with_pattern(program, pattern);

        // Verify only lodash remains
        if let Program::Module(result_module) = result {
            assert_eq!(result_module.body.len(), 1);
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import_decl)) = &result_module.body[0]
            {
                assert_eq!(import_decl.src.value, "lodash");
            } else {
                panic!("Expected an import declaration");
            }
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_transform_program_with_pattern_all_removed() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program where all imports match
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                create_import_module_item("styles.css"),
                create_import_module_item("theme.scss"),
            ],
            shebang: None,
        };

        let program = Program::Module(module);
        let pattern = Regex::new("\\.(css|scss)$").unwrap();

        // Call the testable transformation function
        let result = transform_program_with_pattern(program, pattern);

        // Verify all imports were removed
        if let Program::Module(result_module) = result {
            assert_eq!(result_module.body.len(), 0);
        } else {
            panic!("Expected a module program");
        }
    }

    #[test]
    fn test_transform_program_with_pattern_none_removed() {
        use swc_core::ecma::ast::{Module, Program};

        // Create a program where no imports match
        let module = Module {
            span: DUMMY_SP,
            body: vec![
                create_import_module_item("react"),
                create_import_module_item("lodash"),
            ],
            shebang: None,
        };

        let program = Program::Module(module);
        let pattern = Regex::new("vue|angular").unwrap();

        // Call the testable transformation function
        let result = transform_program_with_pattern(program, pattern);

        // Verify all imports remain
        if let Program::Module(result_module) = result {
            assert_eq!(result_module.body.len(), 2);
        } else {
            panic!("Expected a module program");
        }
    }
}
