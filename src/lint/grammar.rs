// The Rust grammar sits in
// https://github.com/tree-sitter/tree-sitter-rust/blob/master/grammar.js
// We can use the playground to view the node kine:
// https://tree-sitter.github.io/tree-sitter/playground

pub type NodeKind = &'static str;

pub const UNSAFE: NodeKind = "unsafe";
pub const INDEX_EXPRESSION: NodeKind = "index_expression";
pub const IDENTIFIER: NodeKind = "identifier";
pub const FIELD_IDENTIFIER: NodeKind = "field_identifier";
pub const MOD_ITEM: NodeKind = "mod_item";
pub const ATTRIBUTE_ITEM: NodeKind = "attribute_item";
pub const META_ITEM: NodeKind = "meta_item";
pub const ARGUMENTS: NodeKind = "arguments";
