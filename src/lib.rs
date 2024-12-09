use swc_core::ecma::{
    ast::{ModuleItem, Program},
    visit::{VisitMut, VisitMutWith, visit_mut_pass},
    transforms::testing::test_inline,
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use regex::Regex;

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

/// The main transformation function used by SWC.
#[plugin_transform]
pub fn process_transform(
    mut program: Program,
    _metadata: TransformPluginProgramMetadata,
) -> Program {
    // Extract pattern from metadata (replace this with real extraction logic if needed)
    let pattern = Regex::new("some-pattern").unwrap();
    program.visit_mut_with(&mut TransformVisitor::new(pattern));
    program
}

// Unit tests for the plugin transformation behavior.
test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("some-pattern").unwrap()
    )),
    remove_matching_imports,
    // Input code
    r#"import "some-pattern"; import "keep-this";"#,
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
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new(".*-pattern").unwrap()
    )),
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
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("lodash").unwrap()
    )),
    remove_lodash_imports,
    // Input code
    r#"import _ from "lodash"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("react").unwrap()
    )),
    remove_react_imports,
    // Input code
    r#"import React from "react"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("@eslint/js").unwrap()
    )),
    remove_express_imports,
    // Input code
    r#"import express from "express"; import "@eslint/js";"#,
    // Expected output
    r#"import express from "express";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("moment").unwrap()
    )),
    remove_moment_imports,
    // Input code
    r#"import moment from "moment"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("ignore-me").unwrap()
    )),
    remove_ignore_me_imports,
    // Input code
    r#"import "ignore-me"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("ignore-.*").unwrap()
    )),
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
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("^ignore$").unwrap()
    )),
    remove_exact_ignore_imports,
    // Input code
    r#"import "ignore"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);

test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        Regex::new("ignore").unwrap()
    )),
    remove_substring_ignore_imports,
    // Input code
    r#"import "ignore"; import "ignore-this"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);
