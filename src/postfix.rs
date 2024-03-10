use std::fmt::{Display, Formatter};
use crate::node::Node;
use crate::operator::Operator;

#[derive(Debug, PartialEq, Clone)]
pub enum ShuntedStackItem {
    Operator(Operator),
    Operand(Node),
}

impl Display for ShuntedStackItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShuntedStackItem::Operator(op) => write!(f, "{}", op),
            ShuntedStackItem::Operand(node) => write!(f, "{}", node),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShuntedStack {
    items: Vec<ShuntedStackItem>,
    current_iter: usize,
}

impl ShuntedStack {
    pub(crate) fn new() -> Self {
        Self {
            items: Vec::new(),
            current_iter: 0
        }
    }

    pub(crate) fn push(&mut self, item: ShuntedStackItem) {
        self.items.push(item);
    }
}

impl Iterator for ShuntedStack {
    type Item = ShuntedStackItem;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.items.get(self.current_iter).cloned();
        self.current_iter += 1;
        i
    }
}