use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

impl Literal {

    pub fn add(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a + b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a + b)),
            (Literal::String(a), b) => Some(Literal::String(format!("{}{}", a, b))), // can append anything to a string
            (a, Literal::String(b)) => Some(Literal::String(format!("{}{}", a, b))),
            _ => None,
        }
    }

    pub fn sub(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a - b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a - b)),
            _ => None,
        }
    }

    pub fn mul(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a * b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a * b)),
            _ => None,
        }
    }

    pub fn div(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a / b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a / b)),
            _ => None,
        }
    }

    pub fn rem(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a % b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a % b)),
            _ => None,
        }
    }

    pub fn pow(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a.pow(*b as u32))),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Float(a.powf(*b))),
            _ => None,
        }
    }

    pub fn inc(&self) -> Option<Literal> {
        match self {
            Literal::Int(a) => Some(Literal::Int(a + 1)),
            Literal::Float(a) => Some(Literal::Float(a + 1.0)),
            _ => None,
        }
    }

    pub fn dec(&self) -> Option<Literal> {
        match self {
            Literal::Int(a) => Some(Literal::Int(a - 1)),
            Literal::Float(a) => Some(Literal::Float(a - 1.0)),
            _ => None,
        }
    }

    pub fn eq(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a == b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a == b)),
            (Literal::String(a), Literal::String(b)) => Some(Literal::Bool(a == b)),
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(a == b)),
            _ => None,
        }
    }

    pub fn neq(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a != b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a != b)),
            (Literal::String(a), Literal::String(b)) => Some(Literal::Bool(a != b)),
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(a != b)),
            _ => None,
        }
    }

    pub fn gt(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a > b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a > b)),
            _ => None,
        }
    }

    pub fn lt(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a < b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a < b)),
            _ => None,
        }
    }

    pub fn gte(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a >= b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a >= b)),
            _ => None,
        }
    }

    pub fn lte(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Bool(a <= b)),
            (Literal::Float(a), Literal::Float(b)) => Some(Literal::Bool(a <= b)),
            _ => None,
        }
    }

    pub fn and(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(*a && *b)),
            _ => None,
        }
    }

    pub fn or(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(*a || *b)),
            _ => None,
        }
    }

    pub fn xor(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(*a ^ *b)),
            _ => None,
        }
    }

    pub fn shl(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a << b)),
            _ => None,
        }
    }

    pub fn shr(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Int(a), Literal::Int(b)) => Some(Literal::Int(a >> b)),
            _ => None,
        }
    }

    pub fn not(&self) -> Option<Literal> {
        match self {
            Literal::Bool(a) => Some(Literal::Bool(!*a)),
            _ => None,
        }
    }

    pub fn neg(&self) -> Option<Literal> {
        match self {
            Literal::Int(a) => Some(Literal::Int(-*a)),
            Literal::Float(a) => Some(Literal::Float(-*a)),
            _ => None,
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "{}", s),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}