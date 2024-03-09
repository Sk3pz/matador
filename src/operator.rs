use std::fmt::Display;
use crate::literal::Literal;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operator {
    // arithmetic
    Plus,   // +
    Minus,  // -
    Mul,    // *
    Div,    // /
    Mod,    // %
    Pow,    // **
    Inc,    // ++
    Dec,    // --

    // misc
    Decimal, // .
    Range,   // ..

    // control flow
    LParen,   // (
    RParen,   // )
    LBracket, // [
    RBracket, // ]

    // bitwise
    And,    // &
    Or,     // |
    Xor,    // ^
    Not,    // !
    LShift, // <<
    RShift, // >>

    // conditionals
    Eq,     // ==
    Neq,    // !=
    Gt,     // >
    Lt,     // <
    Gte,    // >=
    Lte,    // <=
}

impl Operator {
    pub fn precedence(&self) -> Option<u8> {
        match self {
            Operator::Plus | Operator::Minus => Some(1),
            Operator::Mul | Operator::Div | Operator::Mod => Some(2),
            Operator::Inc | Operator::Dec => Some(3),
            Operator::Pow => Some(4),
            Operator::And | Operator::Or | Operator::Xor |
            Operator::Not | Operator::LShift | Operator::RShift => Some(5),
            Operator::Eq | Operator::Neq | Operator::Gt | Operator::Lt | Operator::Gte | Operator::Lte => Some(6),
            _ => None,
        }
    }

    pub fn can_apply(&self) -> bool {
        match self {
            Operator::Decimal | Operator::Range |
            Operator::LParen | Operator::RParen |
            Operator::LBracket | Operator::RBracket => false,
            _ => true,
        }
    }

    pub fn apply_binary(&self, left: Literal, right: Literal) -> Option<Literal> {
        match self {
            // standard
            Operator::Plus => left.add(&right),
            Operator::Minus => left.sub(&right),
            Operator::Mul => left.mul(&right),
            Operator::Div => left.div(&right),
            Operator::Mod => left.rem(&right),
            Operator::Pow => left.pow(&right),

            // bitwise
            Operator::And => left.and(&right),
            Operator::Or => left.or(&right),
            Operator::Xor => left.xor(&right),
            Operator::Not => left.not(),
            Operator::LShift => left.shl(&right),
            Operator::RShift => left.shr(&right),

            // comparison
            Operator::Eq => left.eq(&right),
            Operator::Neq => left.neq(&right),
            Operator::Gt => left.gt(&right),
            Operator::Lt => left.lt(&right),
            Operator::Gte => left.gte(&right),
            Operator::Lte => left.lte(&right),

            _ => None,
        }
    }

    pub fn apply_unary(&self, left: Literal) -> Option<Literal> {
        match self {
            Operator::Inc => left.inc(),
            Operator::Dec => left.dec(),
            Operator::Minus => left.neg(),
            _ => None,
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operator::Plus => "+",
            Operator::Minus => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Mod => "%",
            Operator::Pow => "**",
            Operator::Inc => "++",
            Operator::Dec => "--",
            Operator::Decimal => ".",
            Operator::Range => "..",
            Operator::LParen => "(",
            Operator::RParen => ")",
            Operator::LBracket => "[",
            Operator::RBracket => "]",
            Operator::And => "&",
            Operator::Or => "|",
            Operator::Xor => "^",
            Operator::Not => "~",
            Operator::LShift => "<<",
            Operator::RShift => ">>",
            Operator::Eq => "==",
            Operator::Neq => "!=",
            Operator::Gt => ">",
            Operator::Lt => "<",
            Operator::Gte => ">=",
            Operator::Lte => "<=",
        };
        write!(f, "{}", op)
    }
}