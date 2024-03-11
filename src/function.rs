use crate::node::Node;
use crate::variable::Variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    // native rust functions
    Native(fn(Vec<Variable>) -> Variable),

    // local matador functions
    Local(Vec<String>, Node),
}