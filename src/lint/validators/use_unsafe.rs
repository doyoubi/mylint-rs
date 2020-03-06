use tree_sitter::Node;

use crate::lint::core::{ValidationError, Validator};
use crate::lint::grammar::IDENTIFIER;
use crate::lint::rule::RULE_USE_UNSAFE;
use crate::lint::utils::node_lowercase_contains;

pub struct UseUnsafeValidator;

impl Validator for UseUnsafeValidator {
    fn validate(&self, node: &Node, source: &str) -> Result<(), ValidationError> {
        if node_lowercase_contains(IDENTIFIER, node, source, "unsafe") {
            Err(ValidationError::from_node(node, RULE_USE_UNSAFE))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::filters::filter_nothing::NothingFilter;
    use crate::lint::utils::{assert_source_ok, validate};
    use crate::RuleCode;

    #[test]
    fn test_no_unsafe_ident() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { let a = [0]; 233 }";
        assert_source_ok(source_code, Box::new(UseUnsafeValidator), &filter);
    }

    #[test]
    fn test_unsafe_call() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { unsafe_func() }";
        let res = validate(source_code, Box::new(UseUnsafeValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::UseUnsafe);
    }

    #[test]
    fn test_unsafe_macro() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { unsafe_macro!() }";
        let res = validate(source_code, Box::new(UseUnsafeValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::UseUnsafe);
    }
}
