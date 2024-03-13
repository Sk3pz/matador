use crate::node::Node;
use crate::variable::Variable;

pub type NativeFunction = fn(Vec<Variable>) -> Variable;

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    // native rust functions
    Native(NativeFunction),

    // local matador functions
    Local(Vec<String>, Node),
}