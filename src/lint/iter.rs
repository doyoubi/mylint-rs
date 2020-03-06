use super::core::Filter;
use tree_sitter::{Node, TreeCursor};

enum State {
    Curr,
    Child,
    Sibling,
    Parent,
}

pub struct NodeIterator<'a> {
    state: State,
    cursor: TreeCursor<'a>,
    source: &'a str,
    sub_tree_filter: &'a dyn Filter,
}

impl<'a> NodeIterator<'a> {
    pub fn new(cursor: TreeCursor<'a>, source: &'a str, sub_tree_filter: &'a dyn Filter) -> Self {
        Self {
            state: State::Curr,
            cursor,
            source,
            sub_tree_filter,
        }
    }

    fn filter_sub_tree(&self, node: Node<'a>) -> bool {
        self.sub_tree_filter.filter(&node, self.source)
    }
}

impl<'a> Iterator for NodeIterator<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state {
                State::Curr => {
                    if self.filter_sub_tree(self.cursor.node()) {
                        self.state = State::Child;
                        return Some(self.cursor.node());
                    } else {
                        self.state = State::Sibling;
                    }
                }
                State::Child => {
                    if self.cursor.goto_first_child() {
                        self.state = State::Curr;
                    } else {
                        self.state = State::Sibling;
                    }
                }
                State::Sibling => {
                    if self.cursor.goto_next_sibling() {
                        self.state = State::Curr;
                    } else {
                        self.state = State::Parent;
                    }
                }
                State::Parent => {
                    if self.cursor.goto_parent() {
                        self.state = State::Sibling;
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
