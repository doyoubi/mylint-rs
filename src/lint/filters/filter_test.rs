use tree_sitter::Node;

use crate::lint::core::Filter;
use crate::lint::grammar::{ARGUMENTS, ATTRIBUTE_ITEM, IDENTIFIER, META_ITEM, MOD_ITEM};
use crate::lint::utils::node_lowercase_eq;

pub struct TestModuleFilter;

impl Filter for TestModuleFilter {
    fn filter(&self, node: &Node, source: &str) -> bool {
        if node.kind() != MOD_ITEM {
            return true;
        }

        // Find the #[cfg(test)] node.
        let attr_node = match node.prev_named_sibling() {
            Some(prev) if prev.kind() == ATTRIBUTE_ITEM => prev,
            _ => return true,
        };
        let meta_item = match attr_node.named_child(0) {
            Some(child) if child.kind() == META_ITEM => child,
            _ => return true,
        };

        if let Some(child) = meta_item.named_child(0) {
            if !node_lowercase_eq(IDENTIFIER, &child, source, "cfg") {
                return true;
            }
        }

        let meta_arguments = match meta_item.child_by_field_name(ARGUMENTS) {
            Some(meta_arguments) => meta_arguments,
            None => return true,
        };

        for i in 0.. {
            let meta_item = match meta_arguments.named_child(i) {
                Some(child) if child.kind() == META_ITEM => child,
                _ => break,
            };
            if let Some(child) = meta_item.named_child(0) {
                if node_lowercase_eq(IDENTIFIER, &child, source, "test") {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::super::filter_nothing::NothingFilter;
    use super::*;
    use crate::lint::rule::RuleCode;
    use crate::lint::utils::{assert_source_ok, validate};
    use crate::lint::validators::unwrap_call::UnwrapCallValidator;

    #[test]
    fn test_tests_module() {
        let source_code = r#"
            #[cfg(test)]
            mod tests {
                #[test]
                fn test_key_exists() {
                    None.unwrap();
                }
            }
        "#;
        let filter = TestModuleFilter;
        assert_source_ok(source_code, Box::new(UnwrapCallValidator), &filter);

        let res = validate(source_code, Box::new(UnwrapCallValidator), &NothingFilter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unwrap);
    }

    #[test]
    fn test_contains_tests_module() {
        let source_code = r#"
            #[cfg(some_attr, test)]
            mod tests {
                #[test]
                fn test_key_exists() {
                    None.unwrap();
                }
            }
        "#;
        let filter = TestModuleFilter;
        assert_source_ok(source_code, Box::new(UnwrapCallValidator), &filter);

        let res = validate(source_code, Box::new(UnwrapCallValidator), &NothingFilter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unwrap);
    }

    #[test]
    fn test_normal_module() {
        let source_code = r#"
            #[cfg(some_attr)]
            mod tests {
                fn test_key_exists() {
                    None.unwrap();
                }
            }
        "#;
        let res = validate(
            source_code,
            Box::new(UnwrapCallValidator),
            &TestModuleFilter,
        );
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unwrap);
    }
}
