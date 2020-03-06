use tree_sitter::Node;

use crate::lint::core::{ValidationError, Validator};
use crate::lint::grammar::FIELD_IDENTIFIER;
use crate::lint::rule::RULE_EXPECT_CALL;
use crate::lint::utils::node_lowercase_eq;

pub struct ExpectCallValidator;

impl Validator for ExpectCallValidator {
    fn validate(&self, node: &Node, source: &str) -> Result<(), ValidationError> {
        if node_lowercase_eq(FIELD_IDENTIFIER, node, source, "expect") {
            Err(ValidationError::from_node(node, RULE_EXPECT_CALL))
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
    fn test_no_expect_call() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { a.no_expect_call() }";
        assert_source_ok(source_code, Box::new(ExpectCallValidator), &filter);
    }

    #[test]
    fn test_expect_call() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { a.expect() }";
        let res = validate(source_code, Box::new(ExpectCallValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::Expect);
    }
}
