use std::fmt::Display;
use better_term::{Color, flush_styles};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StaticType {
    Int,
    Float,
    String,
    Bool,
}

impl Display for StaticType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ = match self {
            StaticType::Int => "int",
            StaticType::Float => "float",
            StaticType::String => "string",
            StaticType::Bool => "bool",
        };
        write!(f, "{}", typ)
    }

}

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

    pub fn bitand(&self, other: &Literal) -> Option<Literal> {
        match (self, other) {
            (Literal::Bool(a), Literal::Bool(b)) => Some(Literal::Bool(*a && *b)),
            _ => None,
        }
    }

    pub fn bitor(&self, other: &Literal) -> Option<Literal> {
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

    pub fn neg(&self) -> Option<Literal> {
        match self {
            Literal::Int(a) => Some(Literal::Int(-*a)),
            Literal::Float(a) => Some(Literal::Float(-*a)),
            _ => None,
        }
    }

    pub fn to_int(&self) -> Literal {
        match self {
            Literal::Int(a) => Literal::Int(*a),
            Literal::Float(a) => Literal::Int(*a as i64),
            Literal::String(a) => Literal::Int(a.parse().unwrap_or_else(|_| {
                println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, a);
                flush_styles();
                std::process::exit(0);
            })),
            Literal::Bool(a) => Literal::Int(*a as i64),
        }
    }

    pub fn to_float(&self) -> Literal {
        match self {
            Literal::Int(a) => Literal::Float(*a as f64),
            Literal::Float(a) => Literal::Float(*a),
            Literal::String(a) => Literal::Float(a.parse().unwrap_or_else(|_| {
                println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, a);
                flush_styles();
                std::process::exit(0);
            })),
            Literal::Bool(a) => Literal::Float(*a as i64 as f64),
        }
    }

    pub fn to_string(&self) -> Literal {
        match self {
            Literal::Int(a) => Literal::String(a.to_string()),
            Literal::Float(a) => Literal::String(a.to_string()),
            Literal::String(a) => Literal::String(a.clone()),
            Literal::Bool(a) => Literal::String(a.to_string()),
        }
    }

    pub fn to_bool(&self) -> Literal {
        match self {
            Literal::Int(a) => Literal::Bool(*a != 0),
            Literal::Float(a) => Literal::Bool(*a != 0.0),
            Literal::String(a) => Literal::Bool(!a.is_empty()),
            Literal::Bool(a) => Literal::Bool(*a),
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