

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(i32),
    Binary(Box<Expr>, Op, Box<Expr>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::Literal(n) => *n,
            Expr::Binary(lhs, op, rhs) => {
                let lhs = lhs.eval();
                let rhs = rhs.eval();
                match op {
                    Op::Add => lhs + rhs,
                    Op::Sub => lhs - rhs,
                    Op::Mul => lhs * rhs,
                    Op::Div => lhs / rhs,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}