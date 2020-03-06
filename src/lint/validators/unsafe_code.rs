use tree_sitter::Node;

use crate::lint::core::{ValidationError, Validator};
use crate::lint::grammar::UNSAFE;
use crate::lint::rule::RULE_UNSAFE_CODE;

pub struct UnsafeCodeValidator;

impl Validator for UnsafeCodeValidator {
    fn validate(&self, node: &Node, _source: &str) -> Result<(), ValidationError> {
        if node.kind() == UNSAFE {
            return Err(ValidationError::from_node(node, RULE_UNSAFE_CODE));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::filters::filter_nothing::NothingFilter;
    use crate::lint::utils::{assert_source_ok, validate};
    use crate::RuleCode;

    #[test]
    fn test_no_unsafe() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { let a = [0]; 233 }";
        assert_source_ok(source_code, Box::new(UnsafeCodeValidator), &filter);
    }

    #[test]
    fn test_unsafe_block() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { unsafe { 233 } }";
        let res = validate(source_code, Box::new(UnsafeCodeValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unsafe);
    }

    #[test]
    fn test_unsafe_func() {
        let filter = NothingFilter;
        let source_code = "unsafe async fn test() -> usize { 233 }";
        let res = validate(source_code, Box::new(UnsafeCodeValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unsafe);
    }
}
