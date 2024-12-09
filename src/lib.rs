use swc_core::ecma::{
    ast::{ModuleItem, Program},
    visit::{VisitMut, VisitMutWith},
};
use swc_core::ecma::{
    transforms::testing::test_inline,
    visit::{visit_mut_pass},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

/// A visitor that removes imports matching a specific pattern.
pub struct TransformVisitor {
    pattern: String,
}

impl TransformVisitor {
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }
}

impl VisitMut for TransformVisitor {
    fn visit_mut_module_items(&mut self, items: &mut Vec<ModuleItem>) {
        // Mutate the items in place
        items.retain(|item| {
            // Example: Filter items based on some logic
            // Replace this with your logic to ignore certain imports
            match item {
                ModuleItem::ModuleDecl(decl) => {
                    // Example: Remove import declarations matching the pattern
                    !matches!(decl, swc_core::ecma::ast::ModuleDecl::Import(import) if import.src.value.contains(&self.pattern))
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
    metadata: TransformPluginProgramMetadata,
) -> Program {
    // Example: Extract pattern from metadata (replace this with real extraction logic if needed)
    let pattern = "some-pattern".to_string();
    program.visit_mut_with(&mut TransformVisitor::new(pattern));
    program
}

// Unit tests for the plugin transformation behavior.
test_inline!(
    Default::default(),
    |_| visit_mut_pass(TransformVisitor::new(
        "some-pattern".to_string()
    )),
    remove_matching_imports,
    // Input code
    r#"import "some-pattern"; import "keep-this";"#,
    // Expected output
    r#"import "keep-this";"#
);
