use tree_sitter::Node;

use crate::lint::core::Filter;

pub struct NothingFilter;

impl Filter for NothingFilter {
    fn filter(&self, _node: &Node, _source: &str) -> bool {
        true
    }
}
