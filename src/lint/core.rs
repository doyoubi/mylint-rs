use super::filters::get_all_filters;
use super::rule::Rule;
use super::validators::get_all_validators;
use crate::lint::iter::NodeIterator;
use std::collections::HashSet;
use std::iter::FromIterator;
use tree_sitter::{Language, Node, Parser, Point, Tree};

#[derive(Debug)]
pub struct CodePosition {
    pub row: usize,
    pub column: usize,
}

impl CodePosition {
    pub fn from_point(point: &Point) -> Self {
        Self {
            row: point.row,
            column: point.column,
        }
    }
}

#[derive(Debug)]
pub struct CodeRange {
    pub start: CodePosition,
    pub end: CodePosition,
}

impl CodeRange {
    pub fn from_node(node: &Node) -> Self {
        CodeRange {
            start: CodePosition::from_point(&node.start_position()),
            end: CodePosition::from_point(&node.end_position()),
        }
    }
}

#[derive(Debug)]
pub struct ValidationError {
    pub code_range: CodeRange,
    pub rule: Rule,
}

impl ValidationError {
    pub fn new(code_range: CodeRange, rule: Rule) -> Self {
        Self { code_range, rule }
    }

    pub fn from_node(node: &Node, rule: Rule) -> Self {
        let range = CodeRange::from_node(node);
        Self::new(range, rule)
    }
}

pub trait Validator {
    fn validate(&self, node: &Node, source: &str) -> Result<(), ValidationError>;
}

pub struct AllRulesValidator {
    validators: Vec<Box<dyn Validator>>,
    suppressed_rules: HashSet<String>,
    filter: Box<dyn Filter>,
}

impl Default for AllRulesValidator {
    fn default() -> Self {
        Self {
            validators: get_all_validators(),
            suppressed_rules: HashSet::new(),
            filter: default_filter(),
        }
    }
}

impl AllRulesValidator {
    pub fn new(suppressed_rules: Vec<String>, filter: Box<dyn Filter>) -> Self {
        Self {
            validators: get_all_validators(),
            suppressed_rules: HashSet::from_iter(
                suppressed_rules.into_iter().map(|s| s.to_lowercase()),
            ),
            filter,
        }
    }
}

impl Validator for AllRulesValidator {
    fn validate(&self, node: &Node, source: &str) -> Result<(), ValidationError> {
        for n in NodeIterator::new(node.walk(), source, &(*self.filter)) {
            for validator in &self.validators {
                if let Err(err) = validator.validate(&n, source) {
                    if !self
                        .suppressed_rules
                        .contains(err.rule.code.to_string().to_lowercase().as_str())
                    {
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }
}

extern "C" {
    fn tree_sitter_rust() -> Language;
}

pub struct SourceCode {
    tree: Tree,
}

impl SourceCode {
    pub fn parse(source_code: &str) -> Option<Self> {
        let mut parser = Parser::new();
        let language = unsafe { tree_sitter_rust() };
        if let Err(err) = parser.set_language(language) {
            error!("failed to set language: {:?}", err);
            return None;
        }
        let tree = parser.parse(source_code, None)?;
        Some(Self { tree })
    }

    pub fn get_root_node(&self) -> Node {
        self.tree.root_node()
    }
}

pub trait Filter {
    // The syntax sub-tree will not be checked
    // if this function returns false.
    fn filter(&self, node: &Node, source: &str) -> bool;
}

pub fn default_filter() -> Box<dyn Filter> {
    Box::new(AndFilter::new(get_all_filters()))
}

pub struct AndFilter {
    filters: Vec<Box<dyn Filter>>,
}

impl AndFilter {
    pub fn new(filters: Vec<Box<dyn Filter>>) -> Self {
        Self { filters }
    }
}

impl Filter for AndFilter {
    fn filter(&self, node: &Node, source: &str) -> bool {
        for f in &self.filters {
            if !f.filter(node, source) {
                return false;
            }
        }
        true
    }
}
