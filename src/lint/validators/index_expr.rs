use tree_sitter::Node;

use crate::lint::core::{ValidationError, Validator};
use crate::lint::grammar::INDEX_EXPRESSION;
use crate::lint::rule::RULE_INDEX_EXPRESSION;

pub struct IndexExpressionValidator;

impl Validator for IndexExpressionValidator {
    fn validate(&self, node: &Node, _source: &str) -> Result<(), ValidationError> {
        if node.kind() != INDEX_EXPRESSION {
            return Ok(());
        }
        Err(ValidationError::from_node(node, RULE_INDEX_EXPRESSION))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lint::filters::filter_nothing::NothingFilter;
    use crate::lint::utils::{assert_source_ok, validate};
    use crate::RuleCode;

    #[test]
    fn test_no_index_expression() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { let a = [0]; 233 }";
        assert_source_ok(source_code, Box::new(IndexExpressionValidator), &filter);
    }

    #[test]
    fn test_index_expression() {
        let filter = NothingFilter;
        let source_code = "fn test() -> usize { let a = [0]; a[0] }";
        let res = validate(source_code, Box::new(IndexExpressionValidator), &filter);
        let err = assert_err!(res);
        assert_eq!(err.rule.code, RuleCode::IndexExpression);
    }
}
