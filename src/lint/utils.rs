use crate::{Filter, NodeIterator, SourceCode, ValidationError, Validator};
use tree_sitter::Node;

#[allow(unused_macros)]
macro_rules! assert_some {
    ($expression:expr) => {
        match $expression {
            Some(item) => item,
            None => panic!("assertion failed: Option instance is not some"),
        }
    };
}

#[allow(unused_macros)]
macro_rules! assert_err {
    ($expression:expr) => {
        match $expression {
            Err(err) => err,
            ok => panic!("assertion failed: {:?} does not match Err()", ok),
        }
    };
}

fn recur_validate(
    node: &Node,
    source: &str,
    validator: Box<dyn Validator>,
    filter: &dyn Filter,
) -> Result<(), ValidationError> {
    for n in NodeIterator::new(node.walk(), source, filter) {
        validator.validate(&n, source)?;
    }
    Ok(())
}

pub fn node_lowercase_contains(node_kind: &str, node: &Node, source: &str, pat: &str) -> bool {
    match get_text(node_kind, node, source) {
        Some(ident) => ident.to_lowercase().contains(pat),
        None => false,
    }
}

pub fn node_lowercase_eq(node_kind: &str, node: &Node, source: &str, s: &str) -> bool {
    match get_text(node_kind, node, source) {
        Some(ident) => ident.to_lowercase().eq(s),
        None => false,
    }
}

fn get_text<'a>(node_kind: &str, node: &Node, source: &'a str) -> Option<&'a str> {
    if node.kind() == node_kind {
        let ident = match node.utf8_text(source.as_bytes()) {
            Ok(ident) => ident,
            Err(err) => {
                error!("failed to get identifier: {:?}", err);
                return None;
            }
        };
        return Some(ident);
    }
    None
}

#[allow(dead_code)]
pub fn assert_source_ok(source_code: &str, validator: Box<dyn Validator>, filter: &dyn Filter) {
    let res = validate(source_code, validator, filter);
    assert!(res.is_ok());
}

pub fn validate(
    source_code: &str,
    validator: Box<dyn Validator>,
    filter: &dyn Filter,
) -> Result<(), ValidationError> {
    let parse_result = SourceCode::parse(source_code);
    let source = assert_some!(parse_result);
    let root = source.get_root_node();
    recur_validate(&root, source_code, validator, filter)
}
