mod rpn;
mod searcher;

pub use rpn::Rpn;
pub use searcher::*;
use std::fmt::{Display, Formatter, Result, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    const ALL: [Self; 4] = [Self::Add, Self::Sub, Self::Mul, Self::Div];
    fn apply(&self, lhs: &(i32, i32), rhs: &(i32, i32)) -> Option<(i32, i32)> {
        match self {
            Self::Add => Some((lhs.0 * rhs.1 + rhs.0 * lhs.1, lhs.1 * rhs.1)),
            Self::Sub => Some((lhs.0 * rhs.1 - rhs.0 * lhs.1, lhs.1 * rhs.1)),
            Self::Mul => Some((lhs.0 * rhs.0, lhs.1 * rhs.1)),
            Self::Div if rhs.0 != 0 => Some((lhs.0 * rhs.1, lhs.1 * rhs.0)),
            _ => None,
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_char(match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
        })
    }
}

#[derive(Debug, Clone)]
pub enum ExpressionElement {
    Operand(i32),
    Operator(Op),
}
