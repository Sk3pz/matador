use std::fmt::Display;
use better_term::{Color, flush_styles};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum VariableType {
    Int,
    Float,
    String,
    Bool,
    Range,
    Array,
    Map,
}

impl Display for VariableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let typ = match self {
            VariableType::Int => "int",
            VariableType::Float => "float",
            VariableType::String => "string",
            VariableType::Bool => "bool",
            VariableType::Range => "range",
            VariableType::Array => "array",
            VariableType::Map => "map",
        };
        write!(f, "{}", typ)
    }

}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Variable {
    // static types
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),

    // dynamic types
    Range(i64, i64),
    Array(Vec<Box<Variable>>),
    Map(Vec<(Box<Variable>, Box<Variable>)>),
}

impl Variable {

    pub(crate) fn add(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a + b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a + b)),
            (Variable::String(a), b) => Some(Variable::String(format!("{}{}", a, b))), // can append anything to a string
            (a, Variable::String(b)) => Some(Variable::String(format!("{}{}", a, b))),

            // adding arrays
            (Variable::Array(a), Variable::Array(b)) => {
                let mut arr = a.clone();
                arr.extend(b.clone());
                Some(Variable::Array(arr))
            }

            // adding an element to an array
            (Variable::Array(a), b) => {
                let mut arr = a.clone();
                arr.push(Box::new(b.clone()));
                Some(Variable::Array(arr))
            }

            // adding maps
            (Variable::Map(a), Variable::Map(b)) => {
                let mut map = a.clone();
                for (k, v) in b.iter() {
                    map.push((k.clone(), v.clone()));
                }
                Some(Variable::Map(map))
            }

            _ => None,
        }
    }

    pub(crate) fn sub(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a - b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a - b)),
            _ => None,
        }
    }

    pub(crate) fn mul(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a * b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a * b)),
            _ => None,
        }
    }

    pub(crate) fn div(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a / b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a / b)),
            _ => None,
        }
    }

    pub(crate) fn rem(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a % b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a % b)),
            _ => None,
        }
    }

    pub(crate) fn pow(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a.pow(*b as u32))),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Float(a.powf(*b))),
            _ => None,
        }
    }

    pub(crate) fn inc(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Int(a + 1)),
            Variable::Float(a) => Some(Variable::Float(a + 1.0)),
            _ => None,
        }
    }

    pub(crate) fn dec(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Int(a - 1)),
            Variable::Float(a) => Some(Variable::Float(a - 1.0)),
            _ => None,
        }
    }

    pub(crate) fn eq(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a == b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a == b)),
            (Variable::String(a), Variable::String(b)) => Some(Variable::Bool(a == b)),
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(a == b)),
            _ => None,
        }
    }

    pub(crate) fn neq(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a != b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a != b)),
            (Variable::String(a), Variable::String(b)) => Some(Variable::Bool(a != b)),
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(a != b)),
            _ => None,
        }
    }

    pub(crate) fn gt(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a > b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a > b)),
            _ => None,
        }
    }

    pub(crate) fn lt(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a < b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a < b)),
            _ => None,
        }
    }

    pub(crate) fn gte(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a >= b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a >= b)),
            _ => None,
        }
    }

    pub(crate) fn lte(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Bool(a <= b)),
            (Variable::Float(a), Variable::Float(b)) => Some(Variable::Bool(a <= b)),
            _ => None,
        }
    }

    pub(crate) fn bitand(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(*a && *b)),
            _ => None,
        }
    }

    pub(crate) fn bitor(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(*a || *b)),
            _ => None,
        }
    }

    pub(crate) fn xor(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(*a ^ *b)),
            _ => None,
        }
    }

    pub(crate) fn shl(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a << b)),
            _ => None,
        }
    }

    pub(crate) fn shr(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Int(a), Variable::Int(b)) => Some(Variable::Int(a >> b)),
            _ => None,
        }
    }

    pub(crate) fn not(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Int(!*a)),
            Variable::Bool(a) => Some(Variable::Bool(!*a)),
            _ => None,
        }
    }

    pub(crate) fn and(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(*a && *b)),
            _ => None,
        }
    }

    pub(crate) fn or(&self, other: &Variable) -> Option<Variable> {
        match (self, other) {
            (Variable::Bool(a), Variable::Bool(b)) => Some(Variable::Bool(*a || *b)),
            _ => None,
        }
    }

    pub(crate) fn neg(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Int(-*a)),
            Variable::Float(a) => Some(Variable::Float(-*a)),
            _ => None,
        }
    }

    pub(crate) fn to_int(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Int(*a)),
            Variable::Float(a) => Some(Variable::Int(*a as i64)),
            Variable::String(a) => Some(Variable::Int(a.parse().unwrap_or_else(|_| {
                println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, a);
                flush_styles();
                std::process::exit(0);
            }))),
            Variable::Bool(a) => Some(Variable::Int(*a as i64)),

            _ => None,
        }
    }

    pub(crate) fn to_float(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Float(*a as f64)),
            Variable::Float(a) => Some(Variable::Float(*a)),
            Variable::String(a) => Some(Variable::Float(a.parse().unwrap_or_else(|_| {
                println!("{}Invalid input: {}{}", Color::BrightRed, Color::Red, a);
                flush_styles();
                std::process::exit(0);
            }))),
            Variable::Bool(a) => Some(Variable::Float(*a as i64 as f64)),
            _ => None,
        }
    }

    pub(crate) fn to_string(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::String(a.to_string())),
            Variable::Float(a) => Some(Variable::String(a.to_string())),
            Variable::String(a) => Some(Variable::String(a.clone())),
            Variable::Bool(a) => Some(Variable::String(a.to_string())),
            _ => None,
        }
    }

    pub(crate) fn to_bool(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Bool(*a != 0)),
            Variable::Float(a) => Some(Variable::Bool(*a != 0.0)),
            Variable::String(a) => Some(Variable::Bool(!a.is_empty())),
            Variable::Bool(a) => Some(Variable::Bool(*a)),
            _ => None,
        }
    }

    pub(crate) fn to_array(&self) -> Option<Variable> {
        match self {
            Variable::Int(a) => Some(Variable::Array(vec![Box::new(Variable::Int(*a))])),
            Variable::Float(a) => Some(Variable::Array(vec![Box::new(Variable::Float(*a))])),
            Variable::String(a) => Some(Variable::Array(a.chars().map(|c| Box::new(Variable::String(c.to_string()))).collect())),
            Variable::Bool(a) => Some(Variable::Array(vec![Box::new(Variable::Bool(*a))])),
            Variable::Array(a) => Some(Variable::Array(a.clone())),
            _ => None,
        }
    }

    pub(crate) fn to_map(&self) -> Option<Variable> {
        match self {
            Variable::Map(a) => Some(Variable::Map(a.clone())),
            _ => None,
        }
    }

    pub(crate) fn to_range(&self) -> Option<Variable> {
        match self {
            Variable::Range(a, b) => Some(Variable::Range(*a, *b)),
            _ => None,
        }
    }

    pub(crate) fn access(&self, i: Variable) -> Option<Variable> {
        match (self, i) {
            (Variable::String(s), Variable::Int(i)) => {
                if i < 0 || i as usize >= s.len() {
                    println!("{}Index out of range: {}{}", Color::BrightRed, Color::Red, i);
                    flush_styles();
                    std::process::exit(0);
                }
                Some(Variable::String(s.chars().nth(i as usize).unwrap().to_string()))
            }
            (Variable::Array(a), Variable::Int(i)) => {
                if i < 0 || i as usize >= a.len() {
                    println!("{}Index out of range: {}{}", Color::BrightRed, Color::Red, i);
                    flush_styles();
                    std::process::exit(0);
                }
                Some(*a[i as usize].clone())
            }
            (Variable::Map(a), i) => {
                for (k, v) in a.iter() {
                    if k.eq(&Box::new(i.clone())) {
                        return Some(*v.clone());
                    }
                }
                println!("{}Key not found: {}{}", Color::BrightRed, Color::Red, i);
                flush_styles();
                std::process::exit(0);
            }
            _ => None,
        }
    }

    pub(crate) fn assign(&mut self, i: Variable, v: Variable) -> Option<Variable> {
        match self {
            Variable::String(s) => {
                // insert a string at a specific index
                if let Variable::Int(i) = i {
                    if i < 0 || i as usize >= s.len() {
                        println!("{}Index out of range: {}{}", Color::BrightRed, Color::Red, i);
                        flush_styles();
                        std::process::exit(0);
                    }
                    let mut s = s.clone();
                    let Variable::String(v) = v else {
                        println!("{}Invalid assignment: {}{}", Color::BrightRed, Color::Red, v);
                        flush_styles();
                        std::process::exit(0);
                    };
                    s.insert_str(i as usize, &v);
                    Some(Variable::String(s))
                } else {
                    None
                }
            }
            Variable::Array(a) => {
                if let Variable::Int(i) = i {
                    if i < 0 || i as usize >= a.len() {
                        println!("{}Index out of range: {}{}", Color::BrightRed, Color::Red, i);
                        flush_styles();
                        std::process::exit(0);
                    }
                    a[i as usize] = Box::new(v);
                    Some(Variable::Array(a.clone()))
                } else {
                    None
                }
            }
            Variable::Map(a) => {
                for (k, v) in a.iter_mut() {
                    if (*k).eq(&Box::new(i.clone())) {
                        *v = v.clone();
                        return Some(Variable::Map(a.clone()));
                    }
                }
                a.push((Box::new(i), Box::new(v)));
                Some(Variable::Map(a.clone()))
            }
            _ => None,
        }
    }

    pub(crate) fn sizeof(&self) -> Option<Variable> {
        match self {
            Variable::String(s) => Some(Variable::Int(s.len() as i64)),
            Variable::Array(a) => Some(Variable::Int(a.len() as i64)),
            Variable::Map(a) => Some(Variable::Int(a.len() as i64)),
            _ => None,
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variable::Int(n) => write!(f, "{}", n),
            Variable::Float(n) => write!(f, "{}", n),
            Variable::String(s) => write!(f, "{}", s),
            Variable::Bool(b) => write!(f, "{}", b),
            Variable::Range(a, b) => write!(f, "{}..{}", a, b),
            Variable::Array(arr) => {
                write!(f, "[")?;
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }
            Variable::Map(map) => {
                write!(f, "{{")?;
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}