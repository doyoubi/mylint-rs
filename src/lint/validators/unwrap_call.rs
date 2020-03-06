use tree_sitter::Node;

use crate::lint::core::{ValidationError, Validator};
use crate::lint::grammar::FIELD_IDENTIFIER;
use crate::lint::rule::RULE_UNWRAP_CALL;
use crate::lint::utils::node_lowercase_eq;

pub struct UnwrapCallValidator;

impl Validator for UnwrapCallValidator {
    fn validate(&self, node: &Node, source: &str) -> Result<(), ValidationError> {
        if node_lowercase_eq(FIELD_IDENTIFIER, node, source, "unwrap") {
            Err(ValidationError::from_node(node, RULE_UNWRAP_CALL))
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
    fn test_no_unwrap_call() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { a.no_wrap_call() }";
        assert_source_ok(source_code, Box::new(UnwrapCallValidator), &filter);
    }

    #[test]
    fn test_unwrap_call() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { a.unwrap() }";
        let res = validate(source_code, Box::new(UnwrapCallValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Unwrap);
    }
}
